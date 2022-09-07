use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (IB)Lgnu/classpath/jdwp/value/Value;
pub fn get_value(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmframe::get_value")
}

/// (ILgnu/classpath/jdwp/value/Value;)V
pub fn set_value(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmframe::set_value")
}
