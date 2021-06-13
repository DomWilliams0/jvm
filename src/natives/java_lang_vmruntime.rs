use crate::alloc::{vmref_alloc_object, VmRef};
use crate::class::{FunctionArgs, Object};
use crate::error::Throwable;
use crate::types::DataValue;
use std::error::Error;

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

    // TODO use classloader arg - native lib can be loaded by 1 classloader only
    // let class_loader = args.take(0);

    log::debug!("loading native library {:?}", lib_name);

    let do_native_load = || -> Result<(), Box<dyn Error>> {
        let lib = unsafe { libloading::Library::new(&lib_name)? };

        // call JNI constructor
        unsafe {
            type JavaVM = (); // TODO JNI types
            let on_load =
                lib.get::<unsafe extern "C" fn(*mut JavaVM, *mut ()) -> i32>(b"JNI_OnLoad\0");

            if let Ok(_) = on_load {
                todo!("JNI_OnLoad")
            }
        }

        // TODO keep native library reference around and release when classloader is GC'd
        std::mem::forget(lib);
        Ok(())
    };

    let result = match do_native_load() {
        Ok(_) => 1,
        Err(err) => {
            log::warn!("failed to load library: {}", err);
            0
        }
    };

    Ok(Some(DataValue::Int(result)))
}
