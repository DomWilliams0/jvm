use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn init_ids(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::init_ids")
}

pub fn get_v_m_interfaces(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::get_v_m_interfaces")
}

pub fn is_up(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_up")
}

pub fn is_loopback(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_loopback")
}

pub fn is_point_to_point(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_point_to_point")
}

pub fn supports_multicast(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::supports_multicast")
}
