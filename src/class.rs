use log::*;

use crate::alloc::VmRef;
use crate::classloader::ClassLoader;
use crate::error::{Throwables, VmResult};
use javaclass::attribute::SourceFile;
use javaclass::ClassError;
use std::mem::MaybeUninit;

pub struct Class {
    name: String,
    source_file: Option<String>,

    /// java/lang/Class instance
    /// TODO weak reference for cyclic?
    class_object: MaybeUninit<VmRef<Object>>,
}

pub struct Object {
    class: VmRef<Class>,
}

impl Class {
    pub fn link(
        expected_name: &str,
        loaded: javaclass::ClassFile,
        classloader: &mut ClassLoader,
    ) -> VmResult<VmRef<Self>> {
        debug!("linking class {:?}", expected_name);

        // check this is indeed the class we expected
        // TODO verify constant pool offsets so we can raise a single classformaterror then trust it
        let defined_class_name = loaded
            .this_class()
            .map_err(|_| Throwables::ClassFormatError)?;
        if defined_class_name != expected_name {
            debug!(
                "expected to load class {:?} but actually loaded {:?}",
                expected_name, defined_class_name
            );
            return Err(Throwables::NoClassDefFoundError);
        }

        let name = defined_class_name.to_owned();
        let source_file = match loaded.attribute::<SourceFile>() {
            Ok(src) => {
                trace!("source file: {:?}", src.0);
                Some(src.0)
            }
            Err(ClassError::Attribute(_)) => None,
            Err(e) => {
                debug!("failed to get sourcefile: {}", e);
                return Err(Throwables::ClassFormatError);
            }
        };

        // TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2

        // TODO resolve symbols (classes, ifaces, methods, fields, etc)

        // alloc self with uninitialised object ptr
        let vm_class = VmRef::new(Self {
            name,
            source_file,
            class_object: MaybeUninit::zeroed(),
        });

        // alloc java/lang/Class
        let obj = VmRef::new(Object {
            class: vm_class.clone(),
        });

        // update ptr - im sure at some point we will need to mutate the class so this *const to
        // *mut hacky is temporary until that's needed
        unsafe {
            let ptr = vm_class.class_object.as_ptr();
            let ptr = ptr as *mut VmRef<Object>;
            ptr.write(obj);
        }

        // TODO set obj->vmdata field to vm_class

        Ok(vm_class)
    }
}

impl Object {}
