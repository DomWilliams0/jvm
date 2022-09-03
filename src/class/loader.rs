use std::cell::RefCell;
use std::sync::Arc;
use std::thread::ThreadId;

use log::*;
use parking_lot::RwLock;
use strum_macros::EnumDiscriminants;

use cafebabe::mutf8::{mstr, StrExt};
use cafebabe::{ClassError, MethodAccessFlags};

use crate::alloc::{vmref_ptr, InternedString, VmRef};
use crate::class::class::Class;
use crate::class::object::Object;
use crate::class::ClassType;
use crate::classpath::{ClassPath, FindClassError};
use crate::error::{Throwables, VmResult};
use crate::interpreter::Frame;
use crate::thread;
use crate::types::{ArrayType, PrimitiveDataType};

pub struct ClassLoader {
    classes: RwLock<Vec<(InternedString, WhichLoader, LoadState)>>,
    bootclasspath: Arc<ClassPath>,
    /// Indexed by PrimitiveDataType, initialised during bootstrap
    primitives: RefCell<Option<Box<[VmRef<Class>]>>>,

    #[cfg(feature = "log-class-loading")]
    logger: parking_lot::Mutex<crate::debug::ClassLoadGraph>,
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
            #[cfg(feature = "log-class-loading")]
            logger: parking_lot::Mutex::new(
                crate::debug::ClassLoadGraph::with_file("/tmp/jvm-classes.dot")
                    .expect("failed to make class logger"),
            ),
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
        let loaded = match cafebabe::load_from_buffer(bytes) {
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
    fn do_load_class(
        &self,
        class_name: &mstr,
        mut loader: WhichLoader,
        _cause: Option<&mstr>,
    ) -> VmResult<VmRef<Class>> {
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
                    let elem_cls = self.load_class_caused_by(elem, loader.clone(), class_name)?;

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
        #[cfg(feature = "log-class-loading")]
        self.logger.lock().register_class_load(class_name, _cause);

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

    pub fn load_class(&self, class_name: &mstr, loader: WhichLoader) -> VmResult<VmRef<Class>> {
        self.do_load_class(class_name, loader, None)
    }

    pub fn load_class_caused_by(
        &self,
        class_name: &mstr,
        loader: WhichLoader,
        cause: &mstr,
    ) -> VmResult<VmRef<Class>> {
        // TODO get thread interpreter and current class automatically
        self.do_load_class(class_name, loader, Some(cause))
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
            ArrayType::Reference(elem) => self.load_class_caused_by(elem, loader, name)?,
        };

        debug!(
            "do_load_array_class: array={:?}, elem_cls={:?}",
            array, elem_cls
        );

        // array classloader = element classloader
        let elem_loader = elem_cls.loader();

        let array_cls = Class::new_array_class(name, elem_loader.clone(), elem_cls, self)?;

        Ok(array_cls)
    }

    fn find_boot_class(&self, class_name: &str) -> VmResult<Vec<u8>> {
        trace!("looking for class {}", class_name);

        match self.bootclasspath.find_and_load(class_name) {
            Ok(bytes) => Ok(bytes),
            Err(FindClassError::NotFound) => Err(Throwables::NoClassDefFoundError),
            Err(FindClassError::Io(err)) => panic!("io error: {}", err), // TODO java.lang.IOError
        }
    }

    pub(crate) fn init_primitives(&self, classes: Box<[VmRef<Class>]>) {
        let mut prims = self.primitives.borrow_mut();
        debug_assert!(prims.is_none(), "primitives should initialised only once");

        *prims = Some(classes);
    }

    pub fn get_bootstrap_class(&self, name: &'static str) -> VmRef<Class> {
        // TODO add array lookup with enum constants for common symbols like Object, or perfect hashing
        let name = name.as_mstr();
        match self.load_state(name.as_ref(), &WhichLoader::Bootstrap) {
            LoadState::Loaded(_, cls) => cls,
            s => panic!("bootstrap class {:?} not loaded (in state {:?})", name, s),
        }
    }

    pub(in crate::class) fn populate_class_vmdata(&self, cls: &mut VmRef<Class>) {
        if let Some(class_cls) = self.get_bootstrap_class_checked("java/lang/Class") {
            cls.init_class_object(class_cls);
        }

        // otherwise leave as null pointer and fix up later
    }

    pub(crate) fn fix_up_class_objects(&self) {
        let class_cls = self.get_bootstrap_class("java/lang/Class");

        let mut guard = self.classes.write();
        for (_, loader, state) in guard.iter_mut() {
            debug_assert!(matches!(*loader, WhichLoader::Bootstrap));

            match state {
                LoadState::Loaded(_, cls) => {
                    cls.init_class_object(class_cls.clone());
                }
                _ => unreachable!(),
            }
        }
    }

    fn get_bootstrap_class_checked(&self, name: &'static str) -> Option<VmRef<Class>> {
        let name = name.as_mstr();
        match self.load_state(name.as_ref(), &WhichLoader::Bootstrap) {
            LoadState::Loaded(_, cls) => Some(cls),
            _ => None,
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
        let array_cls_name = match element_type.class_type() {
            ClassType::Primitive(p) => unreachable!(
                "reference array class expects non-primitive element type but got [{:?}",
                p
            ),
            ClassType::Array(_) => format!("[{}", element_type.name()),
            ClassType::Normal => format!("[L{};", element_type.name()),
        };

        self.load_class(&array_cls_name.to_mstr(), loader)
    }

    pub fn system_classloader(&self) -> VmResult<VmRef<Object>> {
        trace!("getting system classloader");

        // get classloader class
        let classloader_class =
            self.load_class("java/lang/ClassLoader".as_mstr(), WhichLoader::Bootstrap)?;
        classloader_class.ensure_init()?;

        // resolve method
        let method = classloader_class.find_callable_method(
            "getSystemClassLoader".as_mstr(),
            "()Ljava/lang/ClassLoader;".as_mstr(),
            MethodAccessFlags::STATIC,
        )?;

        let thread = thread::get();
        let interpreter = thread.interpreter();
        let frame = Frame::new_no_args(method).unwrap();
        interpreter
            .execute_frame(frame)
            .expect("system classloader");

        todo!("return returned instance")
    }

    /// Is java/lang/Class loaded
    pub fn is_class_class_available(&self) -> bool {
        // TODO cache this
        self.get_bootstrap_class_checked("java/lang/Class")
            .is_some()
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
