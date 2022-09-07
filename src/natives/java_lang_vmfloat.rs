use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (F)I
pub fn float_to_raw_int_bits(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmfloat::float_to_raw_int_bits")
}

/// (I)F
pub fn int_bits_to_float(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmfloat::int_bits_to_float")
}
