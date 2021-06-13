mod api;
mod library;

pub use api::global_env;
pub use library::*;

pub const JNI_VERSION: u32 = sys::JNI_VERSION_1_6;

#[allow(clippy::all, warnings)]
/// bindgen generated from jni.h
pub mod sys;
