use std::sync::Arc;

use log::*;

use crate::alloc::{vmref_ptr, InternedString, VmRef};
use crate::class::{Class, ClassType, NativeInternalFn, Object};
use crate::classpath::ClassPath;
use crate::error::{Throwables, VmResult};

use crate::types::{ArrayType, PrimitiveDataType};
use cafebabe::mutf8::{mstr, MString};
use cafebabe::{ClassError, MethodAccessFlags};
use parking_lot::RwLock;

use crate::interpreter::{Frame, InterpreterResult};
use crate::properties::vm_systemproperties_preinit;
use crate::thread;
use std::cell::RefCell;
use std::thread::ThreadId;
use strum_macros::EnumDiscriminants;

pub struct ClassLoader {
    classes: RwLock<Vec<(InternedString, WhichLoader, LoadState)>>,
    bootclasspath: Arc<ClassPath>,
    /// Indexed by PrimitiveDataType, initialised during bootstrap
    primitives: RefCell<Option<Box<[VmRef<Class>]>>>,
}

#[derive(Clone, Debug, EnumDiscriminants)]
enum LoadState {
    Unloaded,
    Loading(ThreadId, WhichLoader),
    Loaded(ThreadId, VmRef<Class>),
    Failed,
}

#[derive(Clone, Debug)]
pub enum WhichLoader {
    Bootstrap,
    User(VmRef<Object>),
}

impl ClassLoader {
    pub fn new(bootclasspath: Arc<ClassPath>) -> Self {
        ClassLoader {
            bootclasspath,
            classes: Default::default(),
            primitives: RefCell::default(),
        }
    }

    fn load_state(&self, class_name: &mstr, loader: &WhichLoader) -> LoadState {
        let guard = self.classes.read();
        match guard
            .iter()
            .find(|(c, l, _)| l == loader && c.as_mstr() == class_name)
        {
            None => LoadState::Unloaded,
            Some((_, _, state)) => state.clone(),
        }
    }

    fn update_state(&self, class_name: &mstr, loader: &WhichLoader, state: LoadState) {
        trace!(
            "updating loading state of {:?} to {:?}",
            class_name,
            LoadStateDiscriminants::from(&state)
        );

        let mut guard = self.classes.write();
        match guard
            .iter_mut()
            .find(|(c, l, _)| l == loader && c.as_mstr() == class_name)
        {
            Some((_, _, s)) => {
                *s = state;
            }
            None => {
                guard.push((class_name.to_owned(), loader.clone(), state));
            }
        }
    }

    // TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;

    /// Loads class file and links it
    fn do_load(
        &self,
        class_name: &mstr,
        bytes: &[u8],
        loader: WhichLoader,
    ) -> VmResult<VmRef<Class>> {
        // TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)

        // load class
        let loaded = match cafebabe::load_from_buffer(&bytes) {
            Ok(cls) => cls,
            Err(err) => {
                // TODO actually instantiate exceptions
                warn!("class loading failed: {}", err);
                let exc = match err {
                    ClassError::Unsupported(_) => Throwables::UnsupportedClassVersionError,
                    ClassError::Io(_) => Throwables::Other("IOError"),
                    _ => Throwables::ClassFormatError,
                };
                return Err(exc);
            }
        };

        // link loaded .class
        Class::link(class_name, loaded, loader, self)
    }

    /// Loads and creates Class object with ClassState::Uninitialised
    /// TODO use a FnOnce() -> WhichLoader or &WhichLoader to avoid many useless clones
    pub fn load_class(&self, class_name: &mstr, mut loader: WhichLoader) -> VmResult<VmRef<Class>> {
        // TODO run user classloader first
        // TODO array classes are treated differently

        let array_type = ArrayType::from_descriptor(class_name);

        // use bootstrap loader for primitives
        if let Some(ArrayType::Primitive(_)) = array_type {
            loader = WhichLoader::Bootstrap;
        }

        if let WhichLoader::User(_classloader) = &loader {
            match array_type {
                None => {
                    // run user classloader instead of bootstrap
                    todo!("run user classloader")
                }
                Some(ArrayType::Reference(elem)) => {
                    // load element class first
                    let elem_cls = self.load_class(elem, loader.clone())?;

                    // use same loader for this array class
                    loader = elem_cls.loader().clone();
                }
                Some(ArrayType::Primitive(_)) => unreachable!(),
            }
        }

        // check if loading is needed
        match self.load_state(class_name, &loader) {
            LoadState::Loading(t, _) => {
                if t == current_thread() {
                    // this thread is already loading this class, keep going
                } else {
                    // TODO wait for other thread to finish loading
                    todo!("wait for other thread")
                }
            }
            LoadState::Loaded(_, cls) => {
                return Ok(cls);
            }
            LoadState::Unloaded | LoadState::Failed => {}
        }

        debug!("loading class {:?}", class_name);

        // loading is required, update shared state
        // TODO record that this loader is an initiating loader
        self.update_state(
            class_name,
            &loader,
            LoadState::Loading(current_thread(), loader.clone()),
        );

        // load and link
        let link_result = match array_type {
            None => {
                // non-array class
                let class_bytes = self.find_boot_class(class_name.to_utf8().as_ref())?;
                self.do_load(class_name, &class_bytes, loader.clone())
            }
            Some(array) => {
                // array class
                self.do_load_array_class(class_name, loader.clone(), array)
            }
        };

        let linked_class = match link_result {
            Err(e) => {
                self.update_state(class_name, &loader, LoadState::Failed);
                warn!("failed to load class {:?}: {:?}", class_name, e);
                return Err(e);
            }
            Ok(class) => {
                self.update_state(
                    class_name,
                    &loader,
                    LoadState::Loaded(current_thread(), class.clone()),
                );
                debug!(
                    "loaded class {:?} successfully with loader {:?}",
                    class_name, loader
                );
                class
            }
        };

        Ok(linked_class)
    }

    fn do_load_array_class(
        &self,
        name: &mstr,
        loader: WhichLoader,
        array: ArrayType,
    ) -> VmResult<VmRef<Class>> {
        // array class
        let elem_cls = match array {
            ArrayType::Primitive(prim) => self.get_primitive(prim),
            ArrayType::Reference(elem) => self.load_class(elem, loader)?,
        };

        // array classloader = element classloader
        let elem_loader = elem_cls.loader();

        let array_cls = Class::new_array_class(name, elem_loader.clone(), elem_cls, self)?;

        Ok(array_cls)
    }

    fn find_boot_class(&self, class_name: &str) -> VmResult<Vec<u8>> {
        trace!("looking for class {}", class_name);

        let path = self
            .bootclasspath
            .find(class_name)
            .ok_or(Throwables::NoClassDefFoundError)?;

        trace!("found class at {}", path.display());

        let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError

        Ok(bytes)
    }

    pub fn init_bootstrap_classes(&self) -> VmResult<()> {
        // TODO define hardcoded preload classes in a better way

        fn load_class(loader: &ClassLoader, name: impl AsRef<[u8]>) -> VmResult<VmRef<Class>> {
            loader
                .load_class(
                    mstr::from_utf8(name.as_ref()).as_ref(),
                    WhichLoader::Bootstrap,
                )
                .and_then(|class| {
                    class.ensure_init()?;
                    Ok(class)
                })
        }

        // our lord and saviour Object first
        load_class(self, b"java/lang/Object")?;

        // then primitives
        {
            let mut primitives = Vec::with_capacity(PrimitiveDataType::TYPES.len());

            for (prim, name) in &PrimitiveDataType::TYPES {
                let name = mstr::from_utf8(name.as_bytes());
                let cls = Class::new_primitive_class(name.as_ref(), *prim, self)?;
                cls.ensure_init()?;

                primitives.push(cls);
            }

            self.primitives.replace(Some(primitives.into_boxed_slice()));
        }

        struct Preload {
            class: &'static str,
            native_methods: &'static [(&'static str, &'static str, NativeInternalFn)],
        }

        impl Preload {
            fn new(name: &'static str) -> Self {
                Self::with_natives(name, &[])
            }

            fn with_natives(
                name: &'static str,
                natives: &'static [(&'static str, &'static str, NativeInternalFn)],
            ) -> Self {
                Preload {
                    class: name,
                    native_methods: natives,
                }
            }
        }

        // then the rest
        let classes = [
            Preload::new("java/lang/String"),
            Preload::new("java/lang/ClassLoader"),
            Preload::with_natives(
                "gnu/classpath/VMSystemProperties",
                &[(
                    "preInit",
                    "(Ljava/util/Properties;)V",
                    vm_systemproperties_preinit,
                )],
            ),
            Preload::new("[I"),
            Preload::new("java/util/HashMap"),
        ];

        for class in classes.iter() {
            let cls = load_class(self, class.class)?;
            for (method_name, method_desc, fn_ptr) in class.native_methods.iter() {
                let method = cls
                    .find_method_in_this_only(
                        mstr::from_utf8(method_name.as_bytes()).as_ref(),
                        mstr::from_utf8(method_desc.as_bytes()).as_ref(),
                        MethodAccessFlags::NATIVE,
                        MethodAccessFlags::ABSTRACT,
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "cant find native method {:?}.{:?} ({:?}) to bind",
                            class.class, method_name, method_desc
                        )
                    });

                // mark method as bound
                let bound = cls.bind_internal_method(&method, *fn_ptr);
                assert!(
                    bound,
                    "failed to bind native method {:?}.{:?}",
                    class.class, method_name
                );

                // queue trampoline compilation
                // jit.queue_trampoline(method, fn_ptr);
            }
        }

        Ok(())
    }

    pub fn get_bootstrap_class(&self, name: &str) -> VmRef<Class> {
        // TODO add array lookup with enum constants for common symbols like Object, or perfect hashing
        let name = mstr::from_utf8(name.as_bytes());
        match self.load_state(name.as_ref(), &WhichLoader::Bootstrap) {
            LoadState::Loaded(_, cls) => cls,
            s => panic!("bootstrap class {:?} not loaded (in state {:?})", name, s),
        }
    }

    pub fn get_primitive(&self, prim: PrimitiveDataType) -> VmRef<Class> {
        let prims = self.primitives.borrow();
        let prims = prims.as_ref().expect("primitives not initialised");
        let idx = prim as usize;
        unsafe { prims.get_unchecked(idx).clone() }
    }

    pub fn get_primitive_array(&self, prim: PrimitiveDataType) -> VmRef<Class> {
        let array_cls_name = [b'[', prim.char() as u8];
        self.load_class(mstr::from_mutf8(&array_cls_name), WhichLoader::Bootstrap)
            .expect("primitive array class not loaded")
    }

    pub fn load_reference_array_class(
        &self,
        element_type: VmRef<Class>,
        loader: WhichLoader,
    ) -> VmResult<VmRef<Class>> {
        debug_assert!(matches!(element_type.class_type(), ClassType::Normal));

        // TODO mstr display impl
        let array_cls_name = format!("[L{};", element_type.name().to_utf8());
        self.load_class(
            MString::from_utf8(array_cls_name.as_bytes()).as_mstr(),
            loader,
        )
    }

    pub fn system_classloader(&self) -> VmResult<VmRef<Object>> {
        trace!("getting system classloader");

        // get classloader class
        let classloader_class = self.load_class(
            mstr::from_mutf8(b"java/lang/ClassLoader"),
            WhichLoader::Bootstrap,
        )?;
        classloader_class.ensure_init()?;

        // resolve method
        let method = classloader_class.find_callable_method(
            mstr::from_utf8(b"getSystemClassLoader").as_ref(),
            mstr::from_mutf8(b"()Ljava/lang/ClassLoader;").as_ref(),
            MethodAccessFlags::STATIC,
        )?;

        let thread = thread::get();
        let interpreter = thread.interpreter();
        let frame = Frame::new_no_args(method).expect("cant make frame");

        // TODO calling a method from native needs to be more ergonomic
        // TODO interpreter error -> internal vm error throwable
        interpreter.state_mut().push_frame(frame);
        if let InterpreterResult::Exception = interpreter.execute_until_return() {
            let exc = thread.exception().unwrap();
            error!("error getting system classloader: {:?}", exc);
            panic!("system classloader");
        }

        todo!("return returned instance")
    }
}

pub fn current_thread() -> ThreadId {
    std::thread::current().id()
}

impl PartialEq for WhichLoader {
    fn eq(&self, other: &Self) -> bool {
        // TODO newtype VmRef should handle equality
        match (self, other) {
            (WhichLoader::Bootstrap, WhichLoader::Bootstrap) => true,
            (WhichLoader::User(a), WhichLoader::User(b)) => vmref_ptr(a) == vmref_ptr(b),
            _ => false,
        }
    }
}

impl Eq for WhichLoader {}
