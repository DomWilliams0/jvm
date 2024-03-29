use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub mod gnu_classpath_jdwp_vmframe;
pub mod gnu_classpath_jdwp_vmmethod;
pub mod gnu_classpath_jdwp_vmvirtualmachine;
pub mod gnu_classpath_vmstackwalker;
pub mod gnu_classpath_vmsystemproperties;
pub mod java_lang_management_vmmanagementfactory;
pub mod java_lang_reflect_vmconstructor;
pub mod java_lang_reflect_vmfield;
pub mod java_lang_reflect_vmmethod;
pub mod java_lang_reflect_vmproxy;
pub mod java_lang_vmclass;
pub mod java_lang_vmclassloader;
pub mod java_lang_vmobject;
pub mod java_lang_vmruntime;
pub mod java_lang_vmsystem;
pub mod java_lang_vmthread;
pub mod java_lang_vmthrowable;
pub mod java_util_concurrent_atomic_atomiclong;
pub mod sun_misc_unsafe;

pub fn void(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    log::warn!("calling nop native method!");
    Ok(None)
}
