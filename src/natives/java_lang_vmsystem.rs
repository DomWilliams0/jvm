use crate::alloc::{vmref_eq, VmRef};
use crate::class::FunctionArgs;
use crate::error::{Throwable, Throwables};
use crate::types::DataValue;
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
