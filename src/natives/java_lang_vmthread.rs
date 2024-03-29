use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::{Throwable, Throwables};
use crate::thread;
use crate::types::{DataType, DataValue};
use cafebabe::mutf8::StrExt;
use std::borrow::Cow;

/// ()I
pub fn count_stack_frames(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::count_stack_frames")
}

/// (J)V
pub fn start(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::start")
}

/// ()V
pub fn interrupt(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::interrupt")
}

/// ()Z
pub fn is_interrupted(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::is_interrupted")
}

/// ()V
pub fn suspend(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::suspend")
}

/// ()V
pub fn resume(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::resume")
}

/// (I)V
pub fn native_set_priority(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::native_set_priority")
}

/// (Ljava/lang/Throwable;)V
pub fn native_stop(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::native_stop")
}

/// ()Ljava/lang/Thread;
pub fn current_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let state = thread::get();
    let vmthread = state.vm_thread();
    // TODO volatile field!
    let thread = vmthread
        .find_instance_field(
            "thread".as_mstr(),
            &DataType::Reference(Cow::Borrowed("java/lang/Thread".as_mstr())),
        )
        .ok_or(Throwables::NoSuchFieldError)?;

    let thread = thread.into_reference().expect("thread is not reference");

    Ok(Some(DataValue::Reference(thread)))
}

/// ()V
pub fn yield_(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::yield")
}

/// ()Z
pub fn interrupted(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::interrupted")
}

/// ()Ljava/lang/String;
pub fn get_state(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmthread::get_state")
}
