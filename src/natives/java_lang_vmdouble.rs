use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (D)J
pub fn double_to_raw_long_bits(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmdouble::double_to_raw_long_bits")
}

/// (J)D
pub fn long_bits_to_double(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmdouble::long_bits_to_double")
}

/// (DZ)Ljava/lang/String;
pub fn to_string(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmdouble::to_string")
}

/// ()V
pub fn init_i_ds(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmdouble::init_i_ds")
}

/// (Ljava/lang/String;)D
pub fn parse_double(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmdouble::parse_double")
}