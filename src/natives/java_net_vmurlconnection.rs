use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()V
pub fn init(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmurlconnection::init")
}

/// ([BI)Ljava/lang/String;
pub fn guess_content_type_from_buffer(
    _: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmurlconnection::guess_content_type_from_buffer")
}