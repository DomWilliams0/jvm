use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()I
pub fn get_modifiers_internal(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_modifiers_internal")
}

/// ()Ljava/lang/Class;
pub fn get_return_type(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_return_type")
}

/// ()[Ljava/lang/Class;
pub fn get_parameter_types(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_parameter_types")
}

/// ()[Ljava/lang/Class;
pub fn get_exception_types(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_exception_types")
}

/// (Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;
pub fn invoke(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::invoke")
}

/// ()Ljava/lang/String;
pub fn get_signature(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_signature")
}

/// ()Ljava/lang/Object;
pub fn get_default_value(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_default_value")
}

/// ()[[Ljava/lang/annotation/Annotation;
pub fn get_parameter_annotations(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_parameter_annotations")
}

/// (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
pub fn get_annotation(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_annotation")
}

/// ()[Ljava/lang/annotation/Annotation;
pub fn get_declared_annotations(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_reflect_vmmethod::get_declared_annotations")
}
