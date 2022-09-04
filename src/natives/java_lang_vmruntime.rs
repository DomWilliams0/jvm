use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Object};
use crate::classpath::ClassPath;
use crate::error::Throwable;
use crate::jni::NativeLibrary;
use crate::thread;

use crate::types::DataValue;

pub fn vm_map_library_name(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    #[cfg(not(any(unix, windows)))]
    compile_error!("unsupported target");

    let (dll_path_str,) = args.destructure::<(String,)>()?;

    let mut dll_path = libloading::library_filename(&dll_path_str)
        .into_string()
        .ok()
        .expect("java/lang/String expected");

    let thread = thread::get();
    if let Some(path_str) = thread.global().properties().get("java.library.path") {
        // TODO borrow version of classpath
        let cp = ClassPath::from_colon_separated(path_str.as_ref());
        if let Some(found) = cp.find(&dll_path) {
            // TODO non utf8 paths?
            dll_path = found
                .to_str()
                .expect("non utf8 path to native library")
                .to_owned();
        }
    }

    match Object::new_string_utf8(&dll_path) {
        Ok(o) => Ok(Some(DataValue::Reference(o))),
        Err(err) => Err(err.into()),
    }
}

pub fn vm_native_load(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (class_loader, lib_name) = args.destructure::<(VmRef<Object>, String)>()?;

    let thread = thread::get();
    let jvm = thread.global();
    let mut native_libs = jvm.native_libraries_mut();

    let result = if native_libs.contains(&lib_name) {
        // already loaded, nop
        log::debug!("native library {:?} is already loaded", lib_name);
        1
    } else {
        log::debug!("loading native library {:?}", lib_name);

        let lib = NativeLibrary::load(&lib_name);

        match lib {
            Ok(lib) => {
                native_libs.register(lib, lib_name, &class_loader);
                2
            }
            Err(err) => {
                log::warn!("failed to load library: {}", err);
                0
            }
        }
    };

    Ok(Some(DataValue::Int(result)))
}
