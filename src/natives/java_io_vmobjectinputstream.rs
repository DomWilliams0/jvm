use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/reflect/Constructor;)Ljava/lang/Object;
pub fn allocate_object(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmobjectinputstream::allocate_object")
}
