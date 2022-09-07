use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (I)Lgnu/classpath/Pointer;
pub fn allocate(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::allocate")
}

/// (Lgnu/classpath/Pointer;)V
pub fn free(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::free")
}

/// (Lgnu/classpath/Pointer;I)B
/// (Lgnu/classpath/Pointer;I[BII)V
pub fn get(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::get")
}

/// (Lgnu/classpath/Pointer;IB)V
/// (Lgnu/classpath/Pointer;I[BII)V
pub fn put(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::put")
}

/// (Lgnu/classpath/Pointer;I)Lgnu/classpath/Pointer;
pub fn adjust_address(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::adjust_address")
}

/// (Lgnu/classpath/Pointer;III)V
pub fn shift_down(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::shift_down")
}
