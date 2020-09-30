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
mod jvm;
mod monitor;
mod natives;
mod properties;
mod storage;
mod thread;
mod types;

// TODO actually choose versions
pub const JAVA_VERSION: &str = "1.8.0";
pub const JAVA_VM_SPEC_VERSION: &str = "1.0";
pub const JAVA_SPEC_VERSION: &str = "1.8.0";
pub const CLASS_VERSION: &str = "50.0";
