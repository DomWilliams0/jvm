use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::Throwable;
use crate::thread;
use crate::types::DataValue;

// TODO native impls for other VMStackWalker methods

pub fn vm_get_class_context(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let thread = thread::get();

    // collect classes
    let mut classes = Vec::new();
    let mut do_skip = true;
    thread.interpreter().with_frames(|frame| {
        let (cls, _) = frame.class_and_method();

        if do_skip {
            // skip first to begin with the caller of this method
            do_skip = false;
            return;
        }

        classes.push(cls.class_object().to_owned());
    });

    log::debug!("woo {:#?}", classes);

    // create array
    let array_class = thread
        .global()
        .class_loader()
        .get_bootstrap_class("[Ljava/lang/Class;");
    let array_contents = classes.into_iter().map(|cls| DataValue::Reference(cls));
    let array =
        vmref_alloc_object(|| Ok(Object::new_array_with_elements(array_class, array_contents)))?;

    Ok(Some(DataValue::Reference(array)))
}

pub fn vm_get_classloader(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let class_obj = args.take(0).into_reference().unwrap();

    // TODO get vmdata field
    log::debug!("GET VMDATA FROM {:?}", class_obj);

    todo!()
}