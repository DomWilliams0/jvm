use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Object};
use crate::classpath::ClassPath;
use crate::error::Throwable;
use crate::jni::NativeLibrary;
use crate::thread;

use crate::types::DataValue;

pub fn vm_map_library_name(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    #[cfg(not(any(unix, windows)))]
    compile_error!("unsupported target");

    let arg = args.take(0);
    let dll = arg.as_reference().expect("string expected");

    let mut dll_path = dll
        .string_value_utf8()
        .and_then(|s| libloading::library_filename(s).into_string().ok())
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

pub fn vm_native_load(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let lib_name = args.take(1);
    let lib_name = lib_name
        .as_reference()
        .and_then(|obj| obj.string_value_utf8())
        .expect("not a string");

    let thread = thread::get();
    let jvm = thread.global();
    let mut native_libs = jvm.native_libraries_mut();

    let result = if native_libs.contains(&lib_name) {
        // already loaded, nop
        log::debug!("native library {:?} is already loaded", lib_name);
        1
    } else {
        let class_loader = args.take(0).into_reference().expect("not an object");

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
