use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub mod java_lang_class;
pub mod java_lang_double;
pub mod java_lang_float;
pub mod java_lang_vmobject;
pub mod java_lang_vmsystem;
pub mod java_lang_vmthrowable;

pub mod gnu_classpath_vmstackwalker;
pub mod gnu_classpath_vmsystemproperties;

pub fn vm_nop_void(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    log::warn!("calling nop native method!");
    return Ok(None);
}
