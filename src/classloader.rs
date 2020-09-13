use std::sync::Arc;

use log::*;

use crate::alloc::{vmref_ptr, InternedString, VmRef};
use crate::class::{Class, Object};
use crate::classpath::ClassPath;
use crate::error::{Throwables, VmResult};

use cafebabe::mutf8::mstr;
use cafebabe::ClassError;
use parking_lot::RwLock;
use std::thread::ThreadId;
use strum_macros::EnumDiscriminants;

pub struct ClassLoader {
    classes: RwLock<Vec<(InternedString, WhichLoader, LoadState)>>,
    bootclasspath: Arc<ClassPath>,
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
    pub fn load_class(&self, class_name: &mstr, loader: WhichLoader) -> VmResult<VmRef<Class>> {
        // TODO run user classloader first
        // TODO array classes are treated differently
        debug!("loading class {:?}", class_name);

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

        // loading is required, update shared state
        self.update_state(
            class_name,
            &loader,
            LoadState::Loading(current_thread(), loader.clone()),
        );

        // load and link
        let class_bytes = self.find_boot_class(class_name.to_utf8().as_ref())?;
        let linked_class = match self.do_load(class_name, &class_bytes, loader.clone()) {
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
                debug!("loaded class {:?} successfully", class_name);
                class
            }
        };

        Ok(linked_class)
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
            )
            .and_then(|class| class.ensure_init())?;
        }

        Ok(())
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
