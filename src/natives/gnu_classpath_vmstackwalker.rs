use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{null, FunctionArgs, Object, WhichLoader};
use crate::error::Throwable;
use crate::interpreter::FrameInfo;
use crate::thread;
use crate::types::DataValue;

pub fn get_class_context(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let thread = thread::get();

    // collect classes
    let mut classes = Vec::new();
    let mut do_skip = true;
    thread.interpreter().with_frames(|frame| {
        let cls = match frame.class_and_method() {
            FrameInfo::Method(cls, _) => cls,
            _ => return,
        };

        if do_skip {
            // skip first to begin with the caller of this method
            do_skip = false;
            return;
        }

        classes.push(cls.class_object().to_owned());
    });

    // create array
    let array_class = thread
        .global()
        .class_loader()
        .get_bootstrap_class("[Ljava/lang/Class;");
    let array_contents = classes.into_iter().map(DataValue::Reference);
    let array =
        vmref_alloc_object(|| Ok(Object::new_array_with_elements(array_class, array_contents)))?;

    Ok(Some(DataValue::Reference(array)))
}

/// (Ljava/lang/Class;)Ljava/lang/ClassLoader;
pub fn get_class_loader(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (class_obj,) = args.destructure::<(VmRef<Object>,)>()?;

    let (vmdata, _) = class_obj.vmdata();
    let vmdata = vmdata.expect("vmdata not set");

    let obj = match vmdata.loader() {
        WhichLoader::Bootstrap => null(),
        WhichLoader::User(o) => o.clone(),
    };

    Ok(Some(DataValue::Reference(obj)))
}
