use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn get_local_hostname(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_local_hostname")
}

pub fn lookup_inaddr_any(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::lookup_inaddr_any")
}

pub fn get_host_by_addr(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_host_by_addr")
}

pub fn get_host_by_name(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_host_by_name")
}

pub fn aton(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::aton")
}
