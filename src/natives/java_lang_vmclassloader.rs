use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::thread;
use crate::types::{DataValue, PrimitiveDataType};

/// (Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;
pub fn define_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclassloader::define_class")
}

/// (Ljava/lang/Class;)V
pub fn resolve_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclassloader::resolve_class")
}

/// (Ljava/lang/String;Z)Ljava/lang/Class;
pub fn load_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclassloader::load_class")
}

/// (C)Ljava/lang/Class;
pub fn get_primitive_class(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (chr,) = args.destructure::<(u16,)>()?;

    Ok(PrimitiveDataType::from_char(chr as u8).map(|ty| {
        let cls = thread::get().global().class_loader().get_primitive(ty);
        DataValue::Reference(cls.class_object().to_owned())
    }))
}

/// (Ljava/lang/ClassLoader;Ljava/lang/String;)Ljava/lang/Class;
pub fn find_loaded_class(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_lang_vmclassloader::find_loaded_class")
}
