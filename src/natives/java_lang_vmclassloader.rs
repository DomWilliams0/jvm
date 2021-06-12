use crate::alloc::VmRef;
use crate::class::{null, FunctionArgs};
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_get_primitive_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO vm_get_primitive_class
    Ok(Some(DataValue::Reference(null())))
}
