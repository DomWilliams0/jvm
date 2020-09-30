use crate::alloc::{vmref_alloc_object, vmref_eq, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::interpreter::Frame;
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;
use log::*;

pub fn vm_identity_hashcode(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO dont unwrap
    let obj = args.take(0).into_reference().unwrap();
    let hash = obj.identity_hashcode();
    Ok(Some(DataValue::Int(hash)))
}
pub fn vm_array_copy(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let src = args.take(4).into_reference().unwrap();
    let src_start = args.take(3).as_int().unwrap();
    let dst = args.take(2).into_reference().unwrap();
    let dst_start = args.take(1).as_int().unwrap();
    let len = args.take(0).as_int().unwrap();

    let (src_end, src_overflowed) = src_start.overflowing_add(len);
    let (dst_end, dst_overflowed) = dst_start.overflowing_add(len);
    if src_overflowed
        || dst_overflowed
        || src_start.is_negative()
        || len.is_negative()
        || dst_start.is_negative()
    {
        Err(Throwables::Other("java/lang/IndexOutOfBoundsException"))?;
    }

    trace!(
        "arraycopy {:?}[{}..{}] => {:?}[{}..{}]",
        src,
        src_start,
        src_end,
        dst,
        dst_start,
        dst_end
    );

    // null check
    let (src_cls, dst_cls) = src
        .class()
        .zip(dst.class())
        .ok_or(Throwables::NullPointerException)?;

    // array check
    let (src_ty, dst_ty) = src_cls
        .class_type()
        .array_class()
        .zip(dst_cls.class_type().array_class())
        .ok_or_else(|| Throwables::Other("java/lang/ArrayStoreException"))?;

    // TODO check elements really are assignable
    assert!(
        vmref_eq(src_ty, dst_ty),
        "array element assignability check needed"
    );

    // get array contents
    let src_arr = src.array_unchecked();
    let mut dst_arr = dst.array_unchecked();

    // bounds check - already checked len is positive so end >= start
    let src_len = src_arr.len() as i32;
    let dst_len = dst_arr.len() as i32;
    if src_end > src_len || dst_end > dst_len {
        Err(Throwables::Other("java/lang/IndexOutOfBoundsException"))?;
    }

    // do the copy
    // TODO remove bounds check here, we just checked it explicitly
    let dst_slice = &mut dst_arr[dst_start as usize..dst_end as usize];
    let src_slice = &src_arr[src_start as usize..src_end as usize];
    dst_slice.clone_from_slice(src_slice);

    Ok(None)
}

pub fn vm_init_properties(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let props = args.take(0).into_reference().unwrap();
    // TODO actually do preInit

    let thread = thread::get();
    let system_properties = thread.global().properties();
    let interpreter = thread.interpreter();

    // lookup method once
    let props_class = props.class().unwrap();
    let method = props_class
        .find_callable_method(
            "setProperty".as_mstr(),
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".as_mstr(),
            MethodAccessFlags::empty(),
        )
        .expect("cant find setProperty");

    for (key, val) in system_properties.iter() {
        log::debug!("setting property {:?} => {:?}", key, val);

        // alloc jvm string
        let key = vmref_alloc_object(|| Object::new_string(&key.to_mstr())).expect("bad key");
        let val = vmref_alloc_object(|| Object::new_string(&val.to_mstr())).expect("bad value");

        // make frame for method call
        //                           2    1      0 (this)
        let args = [val, key, props.clone()];
        let frame = Frame::new_with_args(
            method.clone(),
            args.iter().map(|o| DataValue::Reference(o.to_owned())),
        )
        .unwrap();

        if let Err(e) = interpreter.execute_frame(frame) {
            // exception occurred
            log::error!("failed to set system property: {:?}", e);
            return Err(e);
        }
    }

    Ok(Some(DataValue::Reference(props)))
}
