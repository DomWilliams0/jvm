use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()Ljava/lang/String;
pub fn get_local_hostname(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_local_hostname")
}

/// ()[B
pub fn lookup_inaddr_any(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::lookup_inaddr_any")
}

/// ([B)Ljava/lang/String;
pub fn get_host_by_addr(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_host_by_addr")
}

/// (Ljava/lang/String;)[[B
pub fn get_host_by_name(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::get_host_by_name")
}

/// (Ljava/lang/String;)[B
pub fn aton(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_net_vminetaddress::aton")
}
