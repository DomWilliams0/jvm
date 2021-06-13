use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::Throwable;
use crate::jni::NativeLibrary;
use crate::thread;
use crate::types::DataValue;

pub fn vm_map_library_name(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    #[cfg(not(any(unix, windows)))]
    compile_error!("unsupported target");

    let arg = args.take(0);
    let dll = arg.as_reference().expect("string expected");

    let dll_path = dll
        .string_value()
        .map(|s| {
            #[cfg(unix)]
            return format!("lib{}.so", s);

            #[cfg(windows)]
            return format!("{}.dll", s);
        })
        .expect("java/lang/String expected");

    match vmref_alloc_object(|| Object::new_string_utf8(&dll_path)) {
        Ok(o) => Ok(Some(DataValue::Reference(o))),
        Err(err) => Err(err.into()),
    }
}

pub fn vm_native_load(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let lib_name = args.take(1);
    let lib_name = lib_name
        .as_reference()
        .and_then(|obj| obj.string_value())
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
