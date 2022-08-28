#![feature(c_variadic, get_mut_unchecked)]
#![allow(dead_code)]

pub use self::jvm::{Jvm, JvmArgs};
pub use error::{JvmError, JvmResult};

mod alloc;
mod bootstrap;
mod class;
mod classpath;
mod constant_pool;
mod debug;
mod error;
mod interpreter;
mod jit;
mod jni;
mod jvm;
mod monitor;
mod natives;
mod properties;
mod storage;
mod thread;
mod types;

pub const JAVA_VERSION: &str = "1.6.0";
pub const JAVA_VM_SPEC_VERSION: &str = "1.0";
pub const JAVA_SPEC_VERSION: &str = "1.6";
pub const CLASS_VERSION: &str = "50.0";
