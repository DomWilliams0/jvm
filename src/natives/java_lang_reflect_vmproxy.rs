use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;
pub fn get_proxy_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmproxy::get_proxy_class")
}

/// (Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/reflect/Proxy$ProxyData;
pub fn get_proxy_data(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmproxy::get_proxy_data")
}

/// (Ljava/lang/ClassLoader;Ljava/lang/reflect/Proxy$ProxyData;)Ljava/lang/Class;
pub fn generate_proxy_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmproxy::generate_proxy_class")
}
