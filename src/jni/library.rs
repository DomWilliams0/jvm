use crate::interpreter::NativeFrame;
use crate::jni::api::current_javavm;
use crate::jni::{sys, JNI_VERSION};
use crate::thread;
use log::*;

use crate::alloc::{vmref_to_weak, VmRef, WeakVmRef};
use crate::class::Object;
use std::ffi::CStr;
use std::path::Path;
use std::ptr;

use thiserror::Error;

pub struct NativeLibrary(libloading::Library);

#[derive(Default)]
pub struct NativeLibraries {
    /// (classloader (None for bootstrap), name, lib)
    libs: Vec<(Option<WeakVmRef<Object>>, String, NativeLibrary)>,
}

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Failed to load library: {0}")]
    Loading(#[from] libloading::Error),

    #[error("Unsupported JNI version {0:x}")]
    UnsupportedJniVersion(sys::jint),
}

impl NativeLibrary {
    pub fn load(path: &str) -> Result<Self, LibraryError> {
        let lib;
        let on_load;

        unsafe {
            lib = libloading::Library::new(&path)?;
            on_load =
                lib.get::<extern "C" fn(*const sys::JavaVM, *mut ()) -> sys::jint>(b"JNI_OnLoad\0");
        }

        let name = Path::new(path).file_name().unwrap(); // is a file if we got this far

        if let Ok(func) = on_load {
            let func_addr = symbol_addr(func.clone());

            // safety: library is still loaded
            let func = unsafe { func.into_raw() };

            debug!("running JNI_OnLoad in library {:?}", name);
            let thread = thread::get();
            let interp = thread.interpreter();
            let frame = NativeFrame::jni_direct("JNI_OnLoad", func_addr);

            interp.execute_native_frame(frame, || {
                let ret = func(current_javavm(), ptr::null_mut());
                trace!("JNI_OnLoad returned {}", ret);

                if ret <= JNI_VERSION as i32 {
                    info!("loaded native library {:?} successfully", name);
                    Ok(())
                } else {
                    warn!("unsupported native library, ignoring");
                    Err(LibraryError::UnsupportedJniVersion(ret))
                }
            })?;
        }

        Ok(Self(lib))
    }

    // TODO call JNI_OnUnload in Drop
}

fn symbol_addr<T>(sym: libloading::Symbol<T>) -> usize {
    // safety: not calling it
    let sym = unsafe { sym.into_raw() };

    // TODO does this work on windows too?
    sym.into_raw() as usize
}

impl NativeLibraries {
    pub fn contains(&self, name: &str) -> bool {
        self.libs.iter().any(|(_, n, _)| name == n)
    }

    /// Should not be already loaded. [class_loader] is null for bootstrap
    pub fn register(&mut self, lib: NativeLibrary, name: String, class_loader: &VmRef<Object>) {
        debug_assert!(!self.contains(&name), "lib {:?} already loaded", name);

        let owner = if class_loader.is_null() {
            None
        } else {
            Some(vmref_to_weak(class_loader))
        };
        self.libs.push((owner, name, lib))
    }

    pub fn resolve_symbol(&self, name: &CStr) -> Option<*const ()> {
        self.libs.iter().find_map(|(_, _, lib)| unsafe {
            lib.0
                .get::<*const ()>(name.to_bytes_with_nul())
                .ok()
                .map(|s| symbol_addr(s) as *const ())
        })
    }
}
