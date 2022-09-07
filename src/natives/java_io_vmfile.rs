use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Object};
use crate::error::{Throwable, Throwables};
use crate::exec_helper::ArrayType;
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;
use log::warn;
use std::path::{Path, PathBuf};

// TODO non utf8 paths?

/// (Ljava/lang/String;)J
pub fn last_modified(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::last_modified")
}

/// (Ljava/lang/String;)Z
pub fn set_read_only(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::set_read_only")
}

/// (Ljava/lang/String;)Z
pub fn create(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::create")
}

/// (Ljava/lang/String;)[Ljava/lang/String;
pub fn list(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO ensure synchronised
    let (path,) = args.destructure::<(String,)>()?;
    let path = Path::new(&path);

    let thread = thread::get();
    let helper = thread.exec_helper();

    let mut output = Vec::new();
    for e in path.read_dir().map_err(|_| Throwables::IoError)? {
        let e = e.map_err(|_| Throwables::IoError)?;
        output.push(Object::new_string_utf8(&e.path().to_string_lossy()).map(DataValue::Reference));
    }

    let arr = helper.collect_array(
        ArrayType::Reference(
            thread
                .global()
                .class_loader()
                .get_bootstrap_class("java/lang/String"),
        ),
        output.into_iter(),
        Some(&thread),
    )?;

    Ok(Some(arr.into()))
}

/// (Ljava/lang/String;Ljava/lang/String;)Z
pub fn rename_to(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::rename_to")
}

/// (Ljava/lang/String;)J
pub fn length(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::length")
}

/// (Ljava/lang/String;)Z
pub fn exists(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (path,) = args.destructure::<(String,)>()?;
    Ok(Some(Path::new(&path).exists().into()))
}

/// (Ljava/lang/String;)Z
pub fn delete(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::delete")
}

/// (Ljava/lang/String;J)Z
pub fn set_last_modified(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::set_last_modified")
}

/// (Ljava/lang/String;)Z
pub fn mkdir(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::mkdir")
}

/// (Ljava/lang/String;)J
pub fn get_total_space(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::get_total_space")
}

/// (Ljava/lang/String;)J
pub fn get_free_space(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::get_free_space")
}

/// (Ljava/lang/String;)J
pub fn get_usable_space(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::get_usable_space")
}

/// (Ljava/lang/String;ZZ)Z
pub fn set_readable(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::set_readable")
}

/// (Ljava/lang/String;ZZ)Z
pub fn set_writable(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::set_writable")
}

/// (Ljava/lang/String;ZZ)Z
pub fn set_executable(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::set_executable")
}

/// (Ljava/lang/String;)Z
pub fn is_file(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (path,) = args.destructure::<(String,)>()?;
    Ok(Some(Path::new(&path).is_file().into()))
}

/// (Ljava/lang/String;)Z
pub fn can_write(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::can_write")
}

/// (Ljava/lang/String;)Z
pub fn can_write_directory(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::can_write_directory")
}

/// (Ljava/lang/String;)Z
pub fn can_read(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::can_read")
}

/// (Ljava/lang/String;)Z
pub fn can_execute(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method java_io_vmfile::can_execute")
}

/// (Ljava/lang/String;)Z
pub fn is_directory(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (path,) = args.destructure::<(String,)>()?;
    Ok(Some(Path::new(&path).is_dir().into()))
}

/// (Ljava/lang/String;)Ljava/lang/String;
pub fn to_canonical_form(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (path,) = args.destructure::<(String,)>()?;
    // TODO throw the proper io error
    let path = Path::new(&path);
    let res = path.canonicalize().map_err(|err| {
        warn!("io error canonicalising '{}': {}", path.display(), err);
        Throwables::IoError
    })?;

    let string = Object::new_string_utf8(&res.to_string_lossy())?;
    Ok(Some(string.into()))
}
