use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::types::DataValue;

pub fn vm_clone(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj,) = args.destructure::<(VmRef<Object>,)>()?;
    let obj_cls = obj.class().ok_or(Throwables::NullPointerException)?;

    let storage = obj.storage().clone();
    let clone = vmref_alloc_object(|| Ok(Object::with_storage(obj_cls, storage)))?;
    log::debug!("cloned {:?} into {:?}", obj, clone);

    Ok(Some(DataValue::Reference(clone)))
}

pub fn vm_get_class(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj,) = args.destructure::<(VmRef<Object>,)>()?;
    let obj_cls = obj.class().ok_or(Throwables::NullPointerException)?;
    Ok(Some(DataValue::Reference(obj_cls.class_object().clone())))
}
