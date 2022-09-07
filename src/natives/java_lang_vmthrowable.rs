use crate::alloc::VmRef;
use crate::class::{null, FunctionArgs};
use crate::error::Throwable;
use crate::types::DataValue;

pub fn fill_in_stack_trace(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO implement fillInStackTrace
    Ok(Some(DataValue::Reference(null())))
}

pub fn get_stack_trace(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthrowable::get_stack_trace")
}
