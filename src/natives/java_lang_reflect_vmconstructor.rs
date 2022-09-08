use crate::alloc::VmRef;
use crate::class::{Class, FunctionArgs, Method, Object};
use crate::error::{Throwable, VmResult};
use crate::exec_helper::{ArrayType, ExecHelperStandalone};
use crate::thread;
use crate::types::{DataType, DataValue, PrimitiveDataType};
use cafebabe::mutf8::StrExt;
use std::borrow::Cow;
use std::iter::empty;

/// (constructor method, java/lang/Class object instance, class vmdata)
fn parse_args(this: &VmRef<Object>) -> VmResult<(VmRef<Method>, VmRef<Object>, VmRef<Class>)> {
    let clazz = ExecHelperStandalone
        .get_instance_field(
            this,
            "clazz",
            &DataType::Reference(Cow::Borrowed("java/lang/Class".as_mstr())),
        )?
        .into_reference()
        .unwrap();

    let slot = ExecHelperStandalone
        .get_instance_field(this, "slot", &DataType::Primitive(PrimitiveDataType::Int))?
        .as_int()
        .unwrap();

    let (cls, f) = clazz.vmdata();
    let cls = cls.expect("not a class"); // TODO better return value from vmdata()

    let method = cls
        .find_method_by_id(slot)
        .expect("no method at expected slot");

    Ok((method, clazz, cls))
}

/// ()I
pub fn get_modifiers_internal(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (this,) = args.destructure::<(VmRef<Object>,)>()?;
    let (method, _, _) = parse_args(&this)?;
    let modifiers = method.flags().bits();
    Ok(Some(DataValue::Int(modifiers as i32)))
}

/// ()[Ljava/lang/Class;
pub fn get_parameter_types(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (this,) = args.destructure::<(VmRef<Object>,)>()?;
    let (method, clazz, _) = parse_args(&this)?;

    assert_eq!(method.args().len(), 0, "todo: return parameter classes");
    let thread = thread::get();
    let helper = thread.exec_helper();
    let arr = helper.collect_array(
        ArrayType::Reference(clazz.class().unwrap()),
        empty(),
    )?;

    Ok(Some(arr.into()))
}

/// ()[Ljava/lang/Class;
pub fn get_exception_types(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmconstructor::get_exception_types")
}

/// ([Ljava/lang/Object;)Ljava/lang/Object;
pub fn construct(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (this, args) = args.destructure::<(VmRef<Object>, VmRef<Object>)>()?;
    let (method, _, cls) = parse_args(&this)?;

    let thread = thread::get();

    let array = args.array().expect("not an array");
    let new_obj = thread.exec_helper().instantiate_and_invoke_constructor(
        cls,
        method,
        array.iter().cloned(),
    )?;

    Ok(Some(new_obj.into()))
}

/// ()Ljava/lang/String;
pub fn get_signature(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmconstructor::get_signature")
}

/// ()[[Ljava/lang/annotation/Annotation;
pub fn get_parameter_annotations(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmconstructor::get_parameter_annotations")
}

/// (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
pub fn get_annotation(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmconstructor::get_annotation")
}

/// ()[Ljava/lang/annotation/Annotation;
pub fn get_declared_annotations(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmconstructor::get_declared_annotations")
}
