use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::types::DataValue;

/// (Ljava/lang/Object;)Ljava/lang/Class;
pub fn get_class(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj,) = args.destructure::<(VmRef<Object>,)>()?;
    let obj_cls = obj.class().ok_or(Throwables::NullPointerException)?;
    Ok(Some(DataValue::Reference(obj_cls.class_object().clone())))
}

/// (Ljava/lang/Cloneable;)Ljava/lang/Object;
pub fn clone(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (obj,) = args.destructure::<(VmRef<Object>,)>()?;
    let obj_cls = obj.class().ok_or(Throwables::NullPointerException)?;

    let storage = obj.storage().clone();
    let clone = vmref_alloc_object(|| Ok(Object::with_storage(obj_cls, storage)))?;
    log::debug!("cloned {:?} into {:?}", obj, clone);

    Ok(Some(DataValue::Reference(clone)))
}

/// (Ljava/lang/Object;)V
pub fn notify(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmobject::notify")
}

/// (Ljava/lang/Object;)V
pub fn notify_all(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmobject::notify_all")
}

/// (Ljava/lang/Object;JI)V
pub fn wait(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmobject::wait")
}
