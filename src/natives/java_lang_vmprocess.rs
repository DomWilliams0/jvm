use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ([Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;Z)V
pub fn native_spawn(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmprocess::native_spawn")
}

/// ()Z
pub fn native_reap(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmprocess::native_reap")
}

/// (J)V
pub fn native_kill(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmprocess::native_kill")
}
