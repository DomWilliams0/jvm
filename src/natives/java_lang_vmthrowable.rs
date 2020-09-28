use crate::alloc::VmRef;
use crate::class::{null, FunctionArgs};
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_fill_in_stack_trace(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO implement fillInStackTrace
    return Ok(Some(DataValue::Reference(null())));
}
