use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::thread;
use crate::types::{DataType, DataValue};
use cafebabe::mutf8::StrExt;
use std::borrow::Cow;

pub fn vm_current_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
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
