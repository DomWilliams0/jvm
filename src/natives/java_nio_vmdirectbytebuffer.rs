use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn allocate(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::allocate")
}

pub fn free(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::free")
}

pub fn get(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::get")
}

pub fn put(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::put")
}

pub fn adjust_address(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::adjust_address")
}

pub fn shift_down(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_nio_vmdirectbytebuffer::shift_down")
}
