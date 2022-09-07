use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()[Ljava/lang/String;
pub fn get_memory_pool_names(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_management_vmmanagementfactory::get_memory_pool_names")
}

/// ()[Ljava/lang/String;
pub fn get_memory_manager_names(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_management_vmmanagementfactory::get_memory_manager_names")
}

/// ()[Ljava/lang/String;
pub fn get_garbage_collector_names(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_management_vmmanagementfactory::get_garbage_collector_names")
}
