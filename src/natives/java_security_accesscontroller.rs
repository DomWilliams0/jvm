use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Method, Object, WhichLoader};
use crate::error::{Throwable, Throwables};
use crate::interpreter::Frame;
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;
use std::iter::once;

fn find_run(
    action: &VmRef<Object>,
    iface: &'static str,
) -> Result<VmRef<Method>, VmRef<Throwable>> {
    let cls = if let Some(cls) = action.class() {
        cls
    } else {
        return Err(Throwables::NullPointerException.into());
    };

    let thread = thread::get();

    // resolve PrivilegedAction iface and run method
    let priv_action_cls = thread
        .global()
        .class_loader()
        .load_class(iface.as_mstr(), WhichLoader::Bootstrap)?;
    let run_iface_method = priv_action_cls
        .find_method_in_this_only(
            "run".as_mstr(),
            "()Ljava/lang/Object;".as_mstr(),
            MethodAccessFlags::ABSTRACT,
            MethodAccessFlags::STATIC,
        )
        .expect("no run method");

    // resolve run impl
    Ok(cls
        .find_overriding_method(&run_iface_method)
        .expect("no impl of run()"))
}

pub fn vm_do_privileged(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let action = args.take(0).into_reference().unwrap();

    // resolve run()
    let run_method = find_run(&action, "java/security/PrivilegedAction")?;

    // lmao just run it
    // TODO privileged execution?
    // TODO propagate unchecked exception
    let frame = Frame::new_with_args(run_method, once(DataValue::Reference(action)))
        .expect("cant make frame");
    let ret = thread::get().interpreter().execute_frame(frame)?;
    Ok(ret)
}
pub fn vm_do_privileged_exception(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let action = args.take(0).into_reference().unwrap();

    // resolve run()
    let run_method = find_run(&action, "java/security/PrivilegedExceptionAction")?;

    // lmao just run it
    // TODO privileged execution?
    // TODO propagate unchecked exception, raise another exception if it raises a checked exception

    let frame = Frame::new_with_args(run_method, once(DataValue::Reference(action)))
        .expect("cant make frame");
    let ret = thread::get().interpreter().execute_frame(frame)?;
    Ok(ret)
}
