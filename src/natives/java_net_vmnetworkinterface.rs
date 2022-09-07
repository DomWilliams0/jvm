use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()V
pub fn init_ids(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::init_ids")
}

/// ()[Ljava/net/VMNetworkInterface;
pub fn get_v_m_interfaces(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::get_v_m_interfaces")
}

/// (Ljava/lang/String;)Z
pub fn is_up(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_up")
}

/// (Ljava/lang/String;)Z
pub fn is_loopback(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_loopback")
}

/// (Ljava/lang/String;)Z
pub fn is_point_to_point(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::is_point_to_point")
}

/// (Ljava/lang/String;)Z
pub fn supports_multicast(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vmnetworkinterface::supports_multicast")
}
