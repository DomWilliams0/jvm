use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()Ljava/lang/String;
pub fn get_system_time_zone_id(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_util_vmtimezone::get_system_time_zone_id")
}
