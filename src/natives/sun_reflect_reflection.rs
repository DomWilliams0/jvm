use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::Throwable;
use crate::thread;
use crate::types::DataValue;

pub fn vm_get_caller_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let thread = thread::get();

    let mut class = None;
    let mut do_skip = true;
    thread.interpreter().with_frames(|frame| {
        let (cls, _) = frame.class_and_method();

        if do_skip {
            // skip first to begin with the caller of this method
            do_skip = false;
            return true;
        }

        class = Some(cls.class_object().to_owned());
        false
    });

    let class = class.expect("no caller");
    Ok(Some(DataValue::Reference(class)))
}
