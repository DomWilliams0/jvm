//! Initialisation of bootstrap classes

use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;

use crate::alloc::VmRef;
use crate::class::{Class, ClassLoader, NativeInternalFn, WhichLoader};
use crate::error::{Throwable, VmResult};
use crate::interpreter::Frame;
use crate::natives::*;
use crate::thread;
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
    Preload::with_natives(
        "java/lang/Object",
        &[
            ("registerNatives", "()V", vm_nop_void),
            ("hashCode", "()I", java_lang_system::vm_identity_hashcode),
        ],
    )
    .load(classloader)?;
    Preload::new("java/lang/Class").load(classloader)?;

    // now that Class is loaded, fix up missing class_object ptrs in all loaded classes so far
    classloader.fix_up_class_objects();

    init_primitives(classloader)?;

    let preload = [
        Preload::new("java/lang/String"),
        Preload::with_natives(
            "java/lang/Class",
            &[
                (
                    "registerNatives",
                    "()V",
                    java_lang_class::vm_register_natives,
                ),
                (
                    "getPrimitiveClass",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    java_lang_class::vm_get_primitive_class,
                ),
                (
                    "desiredAssertionStatus0",
                    "(Ljava/lang/Class;)Z",
                    java_lang_class::vm_desired_assertion_status,
                ),
            ],
        ),
        Preload::with_natives(
            "java/lang/ClassLoader",
            &[("registerNatives", "()V", vm_nop_void)],
        ),
        Preload::with_natives(
            "java/lang/Float",
            &[(
                "floatToRawIntBits",
                "(F)I",
                java_lang_float::vm_float_to_raw_int_bits,
            )],
        ),
        Preload::with_natives(
            "java/lang/Double",
            &[
                (
                    "doubleToRawLongBits",
                    "(D)J",
                    java_lang_double::vm_double_to_raw_int_bits,
                ),
                (
                    "longBitsToDouble",
                    "(J)D",
                    java_lang_double::vm_long_bits_to_double,
                ),
            ],
        ),
        Preload::with_natives(
            "java/lang/System",
            &[
                ("registerNatives", "()V", vm_nop_void),
                (
                    "initProperties",
                    "(Ljava/util/Properties;)Ljava/util/Properties;",
                    java_lang_system::vm_init_properties,
                ),
            ],
        ),
        Preload::new("[I"),
        Preload::new("java/util/HashMap"),
        Preload::with_natives(
            "java/security/AccessController",
            &[
                (
                    "doPrivileged",
                    "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
                    java_security_accesscontroller::vm_do_privileged,
                ),
                (
                    "doPrivileged",
                    "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
                    java_security_accesscontroller::vm_do_privileged_exception,
                ),
            ],
        ),
        Preload::with_natives("sun/misc/VM", &[("initialize", "()V", vm_nop_void)]),
        Preload::with_natives(
            "java/io/FileInputStream",
            &[("initIDs", "()V", vm_nop_void)],
        ),
        Preload::with_natives(
            "java/io/FileOutputStream",
            &[("initIDs", "()V", vm_nop_void)],
        ),
        Preload::with_natives("java/io/FileDescriptor", &[("initIDs", "()V", vm_nop_void)]),
        Preload::with_natives(
            "sun/misc/Unsafe",
            &[
                ("registerNatives", "()V", vm_nop_void),
                (
                    "arrayBaseOffset",
                    "(Ljava/lang/Class;)I",
                    sun_misc_unsafe::vm_array_base_offset,
                ),
                (
                    "arrayIndexScale",
                    "(Ljava/lang/Class;)I",
                    sun_misc_unsafe::vm_array_index_scale,
                ),
                ("addressSize", "()I", sun_misc_unsafe::vm_address_size),
            ],
        ),
        Preload::with_natives(
            "sun/reflect/Reflection",
            &[(
                "getCallerClass",
                "()Ljava/lang/Class;",
                sun_reflect_reflection::vm_get_caller_class,
            )],
        ),
        Preload::with_natives(
            "java/lang/Thread",
            &[("registerNatives", "()V", vm_nop_void)],
        ),
    ];

    for preload in preload.iter() {
        preload.load(classloader)?;
    }

    Ok(())
}

pub fn init_jvm() -> Result<(), VmRef<Throwable>> {
    let thread = thread::get();

    // resolve and call java/lang/System.initializeSystemClass
    let init_system = thread
        .global()
        .class_loader()
        .get_bootstrap_class("java/lang/System")
        .find_method_in_this_only(
            "initializeSystemClass".as_mstr(),
            "()V".as_mstr(),
            MethodAccessFlags::STATIC,
            MethodAccessFlags::ABSTRACT,
        )
        .expect("cant find java/lang/System.initializeSystemClass");
    thread
        .interpreter()
        .execute_frame(Frame::new_no_args(init_system).unwrap())?;

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
        let cls = classloader
            .load_class(self.class.as_mstr(), WhichLoader::Bootstrap)
            .and_then(|class| {
                // class.ensure_init()?;
                Ok(class)
            })?;

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
