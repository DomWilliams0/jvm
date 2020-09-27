use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::interpreter::Frame;
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;

pub fn vm_systemproperties_preinit(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let props = args.take(0).into_reference().unwrap();
    // TODO actually do preInit

    let thread = thread::get();
    let system_properties = thread.global().properties();
    let interpreter = thread.interpreter();

    // lookup method once
    let props_class = props.class().unwrap();
    let method = props_class
        .find_callable_method(
            "setProperty".as_mstr(),
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".as_mstr(),
            MethodAccessFlags::empty(),
        )
        .expect("cant find setProperty");

    for (key, val) in system_properties.iter() {
        log::debug!("setting property {:?} => {:?}", key, val);

        // alloc jvm string
        let key = vmref_alloc_object(|| Object::new_string(&key.to_mstr())).expect("bad key");
        let val = vmref_alloc_object(|| Object::new_string(&val.to_mstr())).expect("bad value");

        // make frame for method call
        //                           2    1      0 (this)
        let args = [val, key, props.clone()];
        let frame = Frame::new_with_args(
            method.clone(),
            args.iter().map(|o| DataValue::Reference(o.to_owned())),
        )
        .unwrap();

        if let Err(e) = interpreter.execute_frame(frame) {
            // exception occurred
            log::error!("failed to set system property: {:?}", e);
            return Err(e);
        }
    }

    Ok(None) // void
}
