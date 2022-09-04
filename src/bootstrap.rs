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

    let preload = [
        Preload::new("java/lang/Class"),
        Preload::new("java/lang/String"),
        Preload::new("java/lang/ClassLoader"),
        Preload::with_natives(
            "gnu/classpath/VMSystemProperties",
            &[(
                "preInit",
                "(Ljava/util/Properties;)V",
                gnu_classpath_vmsystemproperties::vm_systemproperties_preinit,
            )],
        ),
        Preload::with_natives(
            "java/lang/VMSystem",
            &[
                (
                    "identityHashCode",
                    "(Ljava/lang/Object;)I",
                    java_lang_vmsystem::vm_identity_hashcode,
                ),
                (
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    java_lang_vmsystem::vm_array_copy,
                ),
            ],
        ),
        Preload::with_natives(
            "java/lang/VMThrowable",
            &[(
                "fillInStackTrace",
                "(Ljava/lang/Throwable;)Ljava/lang/VMThrowable;",
                java_lang_vmthrowable::vm_fill_in_stack_trace,
            )],
        ),
        Preload::with_natives(
            "java/lang/VMObject",
            &[
                (
                    "clone",
                    "(Ljava/lang/Cloneable;)Ljava/lang/Object;",
                    java_lang_vmobject::vm_clone,
                ),
                (
                    "getClass",
                    "(Ljava/lang/Object;)Ljava/lang/Class;",
                    java_lang_vmobject::vm_get_class,
                ),
            ],
        ),
        Preload::new("[Ljava/lang/Class;"),
        Preload::with_natives(
            "gnu/classpath/VMStackWalker",
            &[
                (
                    "getClassContext",
                    "()[Ljava/lang/Class;",
                    gnu_classpath_vmstackwalker::vm_get_class_context,
                ),
                (
                    "getClassLoader",
                    "(Ljava/lang/Class;)Ljava/lang/ClassLoader;",
                    gnu_classpath_vmstackwalker::vm_get_classloader,
                ),
            ],
        ),
        Preload::with_natives(
            "java/lang/VMClassLoader",
            &[(
                "getPrimitiveClass",
                "(C)Ljava/lang/Class;",
                java_lang_vmclassloader::vm_get_primitive_class,
            )],
        ),
        Preload::with_natives(
            "java/lang/VMClass",
            &[
                (
                    "forName",
                    "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
                    java_lang_vmclass::vm_for_name,
                ),
                (
                    "getDeclaredConstructors",
                    "(Ljava/lang/Class;Z)[Ljava/lang/reflect/Constructor;",
                    java_lang_vmclass::vm_get_declared_constructors,
                ),
            ],
        ),
        Preload::with_natives("java/lang/reflect/VMConstructor", &[]),
        Preload::new("java/lang/System"),
        Preload::with_natives(
            "java/lang/VMRuntime",
            &[
                (
                    "mapLibraryName",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    java_lang_vmruntime::vm_map_library_name,
                ),
                (
                    "nativeLoad",
                    "(Ljava/lang/String;Ljava/lang/ClassLoader;)I",
                    java_lang_vmruntime::vm_native_load,
                ),
            ],
        ),
        Preload::with_natives(
            "java/lang/VMThread",
            &[(
                "currentThread",
                "()Ljava/lang/Thread;",
                java_lang_vmthread::vm_current_thread,
            )],
        ),
        Preload::new("[I"),
        Preload::new("java/util/HashMap"),
        Preload::new("java/lang/ThreadGroup"),
        Preload::new("java/lang/Thread"),
        Preload::new("java/lang/InheritableThreadLocal"),
        Preload::new("java/lang/reflect/Constructor"),
    ];

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
