use crate::jni::api::current_javavm;
use crate::jni::{sys, JNI_VERSION};
use log::*;
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
            debug!("running JNI_OnLoad in library {:?}", name);
            // TODO setup interpreter for direct native call
            let ret = func(current_javavm(), ptr::null_mut());
            trace!("JNI_OnLoad returned {}", ret);

            if ret <= JNI_VERSION as i32 {
                info!("loaded native library {:?} successfully", name);
            } else {
                warn!("unsupported native library, ignoring");
                return Err(LibraryError::UnsupportedJniVersion(ret));
            }
        }

        Ok(Self(lib))
    }

    // TODO call JNI_OnUnload in Drop
}
