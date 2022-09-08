use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Object, WhichLoader};
use crate::error::{Throwable, Throwables};
use crate::exec_helper::ArrayType;
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;
use cafebabe::MethodAccessFlags;
use log::{error, trace};
use smallvec::SmallVec;
use std::iter::once;

/// (Ljava/lang/Class;Ljava/lang/Object;)Z
pub fn is_instance(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_instance")
}

/// (Ljava/lang/Class;Ljava/lang/Class;)Z
pub fn is_assignable_from(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_assignable_from")
}

/// (Ljava/lang/Class;)Z
pub fn is_interface(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_interface")
}

/// (Ljava/lang/Class;)Z
pub fn is_primitive(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_primitive")
}

/// (Ljava/lang/Class;)Ljava/lang/String;
pub fn get_name(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_name")
}

/// (Ljava/lang/Class;)Ljava/lang/Class;
pub fn get_superclass(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_superclass")
}

/// (Ljava/lang/Class;)[Ljava/lang/Class;
pub fn get_interfaces(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_interfaces")
}

/// (Ljava/lang/Class;)Ljava/lang/Class;
pub fn get_component_type(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_component_type")
}

/// (Ljava/lang/Class;Z)I
pub fn get_modifiers(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (class_obj, ignore_inner) = args.destructure::<(VmRef<Object>, bool)>()?;
    assert!(
        ignore_inner,
        "todo: parse inner classes attribute in cafebabe"
    );

    let class = class_obj.class().expect("not a class");
    Ok(Some(DataValue::Int(class.flags().bits() as i32)))
}

/// (Ljava/lang/Class;)Ljava/lang/Class;
pub fn get_declaring_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_declaring_class")
}

/// (Ljava/lang/Class;Z)[Ljava/lang/Class;
pub fn get_declared_classes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_declared_classes")
}

/// (Ljava/lang/Class;Z)[Ljava/lang/reflect/Field;
pub fn get_declared_fields(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_declared_fields")
}

/// (Ljava/lang/Class;Z)[Ljava/lang/reflect/Method;
pub fn get_declared_methods(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_declared_methods")
}

/// (Ljava/lang/Class;Z)[Ljava/lang/reflect/Constructor;
pub fn get_declared_constructors(
    args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj, public_only) = args.destructure::<(VmRef<Object>, bool)>()?;
    let (class, _) = obj.vmdata();
    let class = class.expect("not a class");

    trace!("getDeclaredConstructors({:?}, {:?})", class, public_only);

    let state = thread::get();
    let methods = class
        .find_constructors(
            if public_only {
                MethodAccessFlags::PUBLIC
            } else {
                MethodAccessFlags::empty()
            },
            MethodAccessFlags::ABSTRACT,
        )
        .collect::<SmallVec<[_; 4]>>();

    trace!("returning array of {} constructors", methods.len());

    let vmcons_cls = state
        .global()
        .class_loader()
        .get_bootstrap_class("java/lang/reflect/VMConstructor");
    let cons_cls = state
        .global()
        .class_loader()
        .get_bootstrap_class("java/lang/reflect/Constructor");
    let arr = state.exec_helper().collect_array(
        ArrayType::Reference(cons_cls.clone()),
        methods.into_iter().map(|(i, method)| {
            let vmcons = state.exec_helper().instantiate_and_invoke_constructor(
                vmcons_cls.clone(),
                "(Ljava/lang/Class;I)V",
                [obj.clone().into(), (i as i32).into()].into_iter(),
            )?;

            state
                .exec_helper()
                .instantiate_and_invoke_constructor(
                    cons_cls.clone(),
                    "(Ljava/lang/reflect/VMConstructor;)V",
                    once(vmcons.into()),
                )
                .map(DataValue::from)
        }),
    )?;

    Ok(Some(DataValue::Reference(arr)))
}

/// (Ljava/lang/Class;)Ljava/lang/ClassLoader;
pub fn get_class_loader(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_class_loader")
}

/// (Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
pub fn for_name(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (mut name, initialize, loader) = args.destructure::<(String, bool, VmRef<Object>)>()?;

    trace!(
        "VMClass.forName({:?}, {:?}, {:?})",
        name,
        initialize,
        loader
    );

    // TODO put this into helper
    // convert from java.lang.xyz to java/lang/xyz
    let mut byte_indices = SmallVec::<[_; 8]>::new();
    for (i, c) in name.char_indices() {
        if c == '.' {
            byte_indices.push(i);
        }
    }

    // safety: hasn't changed
    unsafe {
        let bytes = name.as_bytes_mut();
        for i in byte_indices {
            *bytes.get_unchecked_mut(i) = b'/';
        }
    }

    let loader = if loader.is_null() {
        WhichLoader::Bootstrap
    } else {
        WhichLoader::User(loader)
    };

    let state = thread::get();
    // TODO pass in cause for loading
    let loaded = state
        .global()
        .class_loader()
        .load_class(&mstr::from_utf8(name.as_bytes()), loader)
        .map_err(|e| {
            error!("failed to load class: {:?}", e);
            Throwables::ClassNotFoundException
        })?;

    if initialize {
        loaded.ensure_init()?;
    }

    Ok(Some(DataValue::Reference(loaded.class_object().clone())))
}

/// (Ljava/lang/Class;)Z
pub fn is_array(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_array")
}

/// (Ljava/lang/Throwable;)V
pub fn throw_exception(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::throw_exception")
}

/// (Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;
pub fn get_declared_annotations(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_declared_annotations")
}

/// (Ljava/lang/Class;)Ljava/lang/Class;
pub fn get_enclosing_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_enclosing_class")
}

/// (Ljava/lang/Class;)Ljava/lang/reflect/Constructor;
pub fn get_enclosing_constructor(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_enclosing_constructor")
}

/// (Ljava/lang/Class;)Ljava/lang/reflect/Method;
pub fn get_enclosing_method(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_enclosing_method")
}

/// (Ljava/lang/Class;)Ljava/lang/String;
pub fn get_class_signature(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::get_class_signature")
}

/// (Ljava/lang/Class;)Z
pub fn is_anonymous_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_anonymous_class")
}

/// (Ljava/lang/Class;)Z
pub fn is_local_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_local_class")
}

/// (Ljava/lang/Class;)Z
pub fn is_member_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclass::is_member_class")
}
