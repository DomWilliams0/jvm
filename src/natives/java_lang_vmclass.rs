use crate::alloc::VmRef;
use crate::class::{Class, FunctionArgs, Object, WhichLoader};
use crate::error::{Throwable, Throwables};
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::{mstr, StrExt};
use std::iter::once;

use crate::exec_helper::ArrayType;
use cafebabe::MethodAccessFlags;
use log::{error, trace};
use smallvec::SmallVec;

pub fn vm_for_name(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
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

pub fn vm_get_declared_constructors(
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
        || {
            state
                .interpreter()
                .state_mut()
                .current_class()
                .unwrap()
                .loader()
                .clone()
        },
    )?;

    Ok(Some(DataValue::Reference(arr)))
}
