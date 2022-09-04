use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::thread;
use crate::types::{DataValue, PrimitiveDataType};

pub fn vm_register_natives(_args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO actually register natives
    Ok(None)
}

pub fn vm_get_primitive_class(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (str,) = args.destructure::<(String,)>()?;

    let prim_type = str
        .parse::<PrimitiveDataType>()
        .expect("invalid primitive type");

    let cls = thread::get()
        .global()
        .class_loader()
        .get_primitive(prim_type);
    Ok(Some(DataValue::Reference(cls.class_object().to_owned())))
}
pub fn vm_desired_assertion_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO get actual assertion status
    Ok(Some(DataValue::Boolean(false)))
}
