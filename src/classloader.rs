use std::collections::HashMap;
use std::sync::Arc;
use std::thread::ThreadId;

use log::*;

use crate::alloc::{InternedString, VmRef};
use crate::class::{Class, MethodLookupResult, Object};
use crate::classpath::ClassPath;
use crate::error::{Throwables, VmResult};
use crate::thread;
use cafebabe::mutf8::mstr;
use cafebabe::ClassError;

pub struct ClassLoader {
    classes: HashMap<InternedString, (LoadState, VmRef<Class>)>,
    bootclasspath: Arc<ClassPath>,
}

enum LoadState {
    Unloaded,
    Loading(ThreadId),
    Loaded(ThreadId),
    // TODO linked?
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
        }
    }
    // TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;

    pub fn define_class(
        &mut self,
        class_name: &mstr,
        bytes: &[u8],
        loader: WhichLoader,
    ) -> VmResult<VmRef<Class>> {
        debug!("defining class {:?} with {:?} loader", class_name, loader);
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
        let class = Class::link(class_name, loaded, loader, self)?;

        self.classes.insert(
            class_name.to_owned(),
            (LoadState::Loaded(thread::get().thread()), class.clone()),
        );

        // initialisation - run static constructor
        match class.find_static_constructor() {
            MethodLookupResult::FoundMultiple => {
                warn!("class has multiple static constructors");
                return Err(Throwables::ClassFormatError);
            }
            MethodLookupResult::NotFound => { /* no problem */ }
            MethodLookupResult::Found(m) => {
                debug!("running static constructor for {:?}", class.name());

                let thread = thread::get();
                let mut interpreter = thread.interpreter_mut();
                if let Err(e) = interpreter.execute_method(class.clone(), m, None /* static */) {
                    warn!("static constructor failed: {}", e);
                    return Err(Throwables::ClassFormatError); // TODO different exception
                }
            }
        }

        debug!("class: {:#?}", class);
        Ok(class)
    }

    // TODO ClassLoaderRef that holds an upgradable rwlock guard, so no need to hold the lock for the whole method
    pub fn load_class(&mut self, class_name: &mstr, loader: WhichLoader) -> VmResult<VmRef<Class>> {
        // check if loading is needed
        if let Some((state, obj)) = self.classes.get(class_name) {
            match state {
                LoadState::Loaded(_) => {
                    // already loaded, nothing to do
                    return Ok(obj.clone());
                }

                LoadState::Unloaded => unreachable!(
                    "class {:?} shouldnt be loaded with unloaded state",
                    class_name
                ),
                LoadState::Loading(t) => {
                    unimplemented!("class {:?} is being loaded by thread {:?}", class_name, t)
                }
            }
        }

        // TODO actually update and use load state

        // TODO array classes are treated differently

        let class_bytes = self.find_boot_class(class_name.to_utf8().as_ref())?;
        self.define_class(class_name, &class_bytes, loader)
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

    pub fn init_bootstrap_classes(&mut self) -> VmResult<()> {
        let classes = [
            "java/lang/ClassLoader",
            "java/lang/String",
            "java/lang/Object",
            "java/util/HashMap",
        ];

        for class in classes.iter() {
            self.load_class(
                mstr::from_utf8(class.as_bytes()).as_ref(),
                WhichLoader::Bootstrap,
            )?;
        }

        Ok(())
    }
}
