use crate::constant_pool::{Index, Tag};
use crate::types::ClassVersion;

use std::str::Utf8Error;
use thiserror::*;

pub type ClassResult<T> = Result<T, ClassError>;

#[derive(Error, Debug)]
pub enum ClassError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to read bytes: {0:?}")]
    Reading(byte::Error),

    #[error("Failed to read type {0:?} from bytes")]
    ReadingN(&'static str),

    #[error("Missing class magic")]
    Magic,

    #[error("Unsupported class version {0}")]
    Unsupported(ClassVersion),

    #[error("Unknown constant pool tag {0}")]
    CpTag(u8),

    #[error("Bad unicode: {0}")]
    Unicode(#[from] Utf8Error),

    #[error("Bad access flags: {0}")]
    AccessFlags(u16),

    #[error("No such index in the constant pool: {0}")]
    CpIndex(Index),

    #[error("Incorrect constant pool type at #{index}: expected {expected:?} but got {actual:?}")]
    CpEntry {
        index: Index,
        expected: Tag,
        actual: Tag,
    },

    /// Attribute name
    #[error("Attribute not found {0:?}")]
    Attribute(&'static str),

    /// Arbitrary reason
    #[error("Invalid attribute format: {0}")]
    AttributeFormat(&'static str),

    #[error("Attribute {0:?} is limited to 1 but found multiple")]
    MultipleAttributes(&'static str),

    #[error("No super class, must be java/lang/Object")]
    NoSuper,
}

impl From<byte::Error> for ClassError {
    fn from(e: byte::Error) -> Self {
        ClassError::Reading(e)
    }
}
