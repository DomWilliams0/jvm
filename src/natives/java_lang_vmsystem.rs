use crate::alloc::{vmref_eq, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::types::DataValue;
use log::trace;

/// (Ljava/io/InputStream;)V
pub fn set_in(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::set_in")
}

/// (Ljava/io/PrintStream;)V
pub fn set_out(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::set_out")
}

/// (Ljava/io/PrintStream;)V
pub fn set_err(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::set_err")
}

/// ()J
pub fn current_time_millis(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::current_time_millis")
}

/// ()J
pub fn nano_time(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::nano_time")
}

/// ()Ljava/util/List;
pub fn environ(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::environ")
}

/// (Ljava/lang/String;)Ljava/lang/String;
pub fn getenv(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmsystem::getenv")
}

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
    let (src_cls, dst_cls) = src
        .class()
        .zip(dst.class())
        .ok_or(Throwables::NullPointerException)?;

    // array check
    let (src_ty, dst_ty) = src_cls
        .class_type()
        .array_class()
        .zip(dst_cls.class_type().array_class())
        .ok_or(Throwables::Other("java/lang/ArrayStoreException"))?;

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
        return Err(Throwables::Other("java/lang/IndexOutOfBoundsException").into());
    }

    // do the copy
    // TODO remove bounds check here, we just checked it explicitly
    let dst_slice = &mut dst_arr[dst_start as usize..dst_end as usize];
    let src_slice = &src_arr[src_start as usize..src_end as usize];
    dst_slice.clone_from_slice(src_slice);

    Ok(None)
}
