use std::fmt::{Debug, Formatter};

use thiserror::*;

use crate::alloc::VmRef;
use crate::thread;

pub type JvmResult<T> = Result<T, JvmError>;

/// Internal error
#[derive(Error)]
pub enum JvmError {
    #[error("Exception thrown: {0:?}")]
    ExceptionThrown(Throwables),
}

pub type VmResult<T> = Result<T, Throwables>;

/// Well-known throwables
#[derive(Debug)]
pub enum Throwables {
    NoClassDefFoundError,
    LinkageError,
    ClassNotFoundException,

    Other(&'static str),
}

#[derive(Debug, Clone)]
pub struct Throwable {
    // TODO reference to class instead of name
    // TODO reference to cause
    // TODO backtrace
    class_name: &'static str,
}

pub trait ResultExt<T> {
    fn throw(self) -> JvmResult<T>;
}

impl<T> ResultExt<T> for VmResult<T> {
    fn throw(self) -> JvmResult<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => {
                let exc = VmRef::new(Throwable {
                    class_name: e.symbol(),
                });
                thread::get().set_exception(exc);

                Err(JvmError::ExceptionThrown(e))
            }
        }
    }
}

impl Throwables {
    pub fn symbol(&self) -> &'static str {
        match self {
            Throwables::NoClassDefFoundError => "java/lang/NoClassDefFoundError",
            Throwables::LinkageError => "java/lang/LinkageError",
            Throwables::ClassNotFoundException => "java/lang/ClassNotFoundException",
            Throwables::Other(s) => s,
        }
    }
}

impl Debug for JvmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
