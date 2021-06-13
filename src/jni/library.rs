use crate::interpreter::{JniFrame, NativeFrame, NativeFrameInner};
use crate::jni::api::current_javavm;
use crate::jni::{sys, JNI_VERSION};
use crate::thread;
use log::*;

use crate::class::NativeFunction;
use std::path::Path;
use std::ptr;
use thiserror::Error;

pub struct NativeLibrary(libloading::Library);

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
            let frame = NativeFrame {
                // TODO add constructor for native frame
                inner: NativeFrameInner::Jni(JniFrame::new("JNI_OnLoad")),
                function: NativeFunction::Jni(func_addr),
                args: None, // passed manually
            };

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
