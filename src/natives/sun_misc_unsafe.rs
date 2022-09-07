use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (Ljava/lang/reflect/Field;)J
pub fn object_field_offset(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::object_field_offset")
}

/// (Ljava/lang/Object;JII)Z
pub fn compare_and_swap_int(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::compare_and_swap_int")
}

/// (Ljava/lang/Object;JJJ)Z
pub fn compare_and_swap_long(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::compare_and_swap_long")
}

/// (Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z
pub fn compare_and_swap_object(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::compare_and_swap_object")
}

/// (Ljava/lang/Object;JI)V
pub fn put_ordered_int(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_ordered_int")
}

/// (Ljava/lang/Object;JJ)V
pub fn put_ordered_long(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_ordered_long")
}

/// (Ljava/lang/Object;JLjava/lang/Object;)V
pub fn put_ordered_object(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_ordered_object")
}

/// (Ljava/lang/Object;JI)V
pub fn put_int_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_int_volatile")
}

/// (Ljava/lang/Object;J)I
pub fn get_int_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::get_int_volatile")
}

/// (Ljava/lang/Object;JJ)V
pub fn put_long_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_long_volatile")
}

/// (Ljava/lang/Object;JJ)V
pub fn put_long(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_long")
}

/// (Ljava/lang/Object;J)J
pub fn get_long_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::get_long_volatile")
}

/// (Ljava/lang/Object;J)J
pub fn get_long(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::get_long")
}

/// (Ljava/lang/Object;JLjava/lang/Object;)V
pub fn put_object_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_object_volatile")
}

/// (Ljava/lang/Object;JLjava/lang/Object;)V
pub fn put_object(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::put_object")
}

/// (Ljava/lang/Object;J)Ljava/lang/Object;
pub fn get_object_volatile(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::get_object_volatile")
}

/// (Ljava/lang/Class;)I
pub fn array_base_offset(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::array_base_offset")
}

/// (Ljava/lang/Class;)I
pub fn array_index_scale(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::array_index_scale")
}

/// (Ljava/lang/Object;)V
pub fn unpark(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::unpark")
}

/// (ZJ)V
pub fn park(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method sun_misc_unsafe::park")
}
