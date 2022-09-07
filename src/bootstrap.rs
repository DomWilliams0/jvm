//! Initialisation of bootstrap classes

use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;

use crate::class::{Class, ClassLoader, NativeInternalFn, WhichLoader};
use crate::error::VmResult;
use crate::natives::*;
use crate::types::PrimitiveDataType;

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

pub fn init_bootstrap_classes(classloader: &ClassLoader) -> VmResult<()> {
    // our lord and saviours first
    Preload::new("java/lang/Object").load(classloader)?;
    Preload::new("java/lang/Class").load(classloader)?;

    // now that Class is loaded, fix up missing class_object ptrs in all loaded classes so far
    classloader.fix_up_class_objects();

    init_primitives(classloader)?;

    // TODO remove class name from start of native methods
    let preload = include!("preload.txt");

    for preload in preload.iter() {
        preload.load(classloader)?;
    }

    Ok(())
}

fn init_primitives(classloader: &ClassLoader) -> VmResult<()> {
    let mut primitives = Vec::with_capacity(PrimitiveDataType::TYPES.len());

    for (prim, name) in &PrimitiveDataType::TYPES {
        let name = name.to_mstr();
        let cls = Class::new_primitive_class(name.as_ref(), *prim, classloader)?;
        // cls.ensure_init()?;

        primitives.push(cls);
    }

    classloader.init_primitives(primitives.into_boxed_slice());
    Ok(())
}

impl Preload {
    fn load(&self, classloader: &ClassLoader) -> VmResult<()> {
        let cls = classloader.load_class(self.class.as_mstr(), WhichLoader::Bootstrap)?;

        for (method_name, method_desc, fn_ptr) in self.native_methods.iter() {
            let method = cls
                .find_method_in_this_only(
                    &method_name.to_mstr(),
                    &method_desc.to_mstr(),
                    MethodAccessFlags::NATIVE,
                    MethodAccessFlags::ABSTRACT,
                )
                .unwrap_or_else(|| {
                    panic!(
                        "cant find native method {}.{} ({:?}) to bind",
                        self.class, method_name, method_desc
                    )
                });

            // mark method as bound
            let bound = cls.bind_internal_method(&method, *fn_ptr);
            assert!(bound, "failed to bind native method {}", method);
        }

        Ok(())
    }
}
