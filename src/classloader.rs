use std::collections::HashMap;
use std::sync::Arc;
use std::thread::ThreadId;

use log::*;

use crate::alloc::VmRef;
use crate::classpath::ClassPath;
use crate::error::{Throwables, VmResult};
use crate::JvmResult;

pub struct Object {}

pub struct Class {}

pub struct ClassLoader {
    classes: HashMap<String, (LoadState, VmRef<Class>)>,
    bootclasspath: Arc<ClassPath>,
}

enum LoadState {
    Unloaded,
    Loading(ThreadId),
    Loaded(ThreadId),
    // TODO linked?
}

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
        class_name: &str,
        bytes: &[u8],
        loader: WhichLoader,
    ) -> JvmResult<VmRef<Class>> {
        todo!()
    }

    // TODO ClassLoaderRef that holds an upgradable rwlock guard, so no need to hold the lock for the whole method
    pub fn load_class(&mut self, class_name: &str, loader: WhichLoader) -> VmResult<VmRef<Class>> {
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

        let class_bytes = self.find_boot_class(class_name)?;

        // TODO register class "package" (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)

        todo!("{:?}", class_name)
    }

    fn find_boot_class(&self, class_name: &str) -> VmResult<Vec<u8>> {
        trace!("looking for class {:?}", class_name);

        let path = self
            .bootclasspath
            .find(class_name)
            .ok_or(Throwables::NoClassDefFoundError)?;

        trace!("found class at {}", path.display());

        let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError

        Ok(bytes)
    }

    pub fn init_bootstrap_classes(&mut self) -> VmResult<()> {
        self.load_class("java/lang/ClassLoader", WhichLoader::Bootstrap)?;
        Ok(())
    }
}
