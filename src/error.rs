use std::fmt::{Debug, Formatter};

use thiserror::*;

use crate::alloc::{vmref_alloc_exception, VmRef};
use crate::thread;

pub type JvmResult<T> = Result<T, JvmError>;

/// Internal error
#[derive(Error)]
pub enum JvmError {
    #[error("Exception thrown: {0:?}")]
    ExceptionThrown(VmRef<Throwable>),
}

pub type VmResult<T> = Result<T, Throwables>;

/// Well-known throwables
#[derive(Debug, Clone)]
pub enum Throwables {
    NoClassDefFoundError,
    LinkageError,
    ClassNotFoundException,
    ClassFormatError,
    UnsupportedClassVersionError,
    OutOfMemoryError,
    NullPointerException,
    NoSuchFieldError,
    IoError,
    Other(&'static str),
}

#[derive(Debug, Clone)]
pub struct Throwable {
    // TODO reference to class instead of name
    // TODO reference to cause
    // TODO backtrace
    pub class_name: &'static str,
}

pub trait ResultExt<T> {
    fn throw(self) -> JvmResult<T>;
}

impl<T> ResultExt<T> for VmResult<T> {
    fn throw(self) -> JvmResult<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => {
                let exc: VmRef<Throwable> = e.into();
                thread::get().set_exception(exc.clone());

                Err(JvmError::ExceptionThrown(exc))
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
            Throwables::ClassFormatError => "java/lang/ClassFormatError",
            Throwables::UnsupportedClassVersionError => "java/lang/UnsupportedClassVersionError",
            Throwables::OutOfMemoryError => "java/lang/OutOfMemoryError",
            Throwables::NullPointerException => "java/lang/NullPointerException",
            Throwables::NoSuchFieldError => "java/lang/NoSuchFieldError",
            Throwables::IoError => "java/io/IOError",
            Throwables::Other(s) => s,
        }
    }
}

impl Debug for JvmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Throwables> for VmRef<Throwable> {
    fn from(exc: Throwables) -> Self {
        vmref_alloc_exception(exc)
    }
}
