use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::thread;
use crate::types::{DataValue, PrimitiveDataType};

pub fn natives(_args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO actually register natives
    Ok(None)
}

pub fn primitive_class(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {

}
pub fn assertion_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO get actual assertion status
    Ok(Some(DataValue::Boolean(false)))
}
