use crate::alloc::{vmref_eq, VmRef};
use crate::class::{Class, ClassType, FunctionArgs, Object};
use crate::error::{Throwable, Throwables, VmResult};
use crate::types::DataValue;
use cafebabe::mutf8::StrExt;
use log::trace;

/// (Ljava/lang/Object;)I
pub fn identity_hash_code(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj,) = args.destructure::<(VmRef<Object>,)>()?;
    let hash = obj.identity_hashcode();
    Ok(Some(DataValue::Int(hash)))
}

/// (Ljava/lang/Object;ILjava/lang/Object;II)V
pub fn arraycopy(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (src, src_start, dst, dst_start, len) =
        args.destructure::<(VmRef<Object>, i32, VmRef<Object>, i32, i32)>()?;

    let (src_end, src_overflowed) = src_start.overflowing_add(len);
    let (dst_end, dst_overflowed) = dst_start.overflowing_add(len);
    if src_overflowed
        || dst_overflowed
        || src_start.is_negative()
        || len.is_negative()
        || dst_start.is_negative()
    {
        return Err(Throwables::Other("java/lang/IndexOutOfBoundsException").into());
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
    let src_cls = src.class().ok_or(Throwables::NullPointerException)?;
    let dst_cls = dst.class().ok_or(Throwables::NullPointerException)?;

    // array type check
    if !dst_cls.can_array_be_copied_to(&src_cls) {
        return Err(Throwables::Other("java/lang/ArrayStoreException").into());
    }

    // get array contents
    let src_arr = src.array_unchecked();
    let mut dst_arr = dst.array_unchecked();

    // bounds check - already checked len is positive so end >= start
    let src_len = src_arr.len() as i32;
    let dst_len = dst_arr.len() as i32;
    if src_end > src_len || dst_end > dst_len {
        return Err(Throwables::Other("java/lang/IndexOutOfBoundsException").into());
    }

    // do the copy
    // TODO remove bounds check here, we just checked it explicitly
    let dst_slice = &mut dst_arr[dst_start as usize..dst_end as usize];
    let src_slice = &src_arr[src_start as usize..src_end as usize];
    dst_slice.clone_from_slice(src_slice);

    Ok(None)
}
