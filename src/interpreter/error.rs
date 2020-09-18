use crate::error::Throwables;
use crate::interpreter::insn::Opcode;
use crate::types::{DataType, DataValue};
use cafebabe::mutf8::MString;
use thiserror::*;

#[derive(Error, Debug, Clone)]
pub enum InterpreterError {
    #[error("Incomplete instruction at byte {0}")]
    IncompleteInstruction(usize),

    #[error("Invalid instruction 0x{0:x}")]
    InvalidInstruction(u8),

    #[error("Opcode {0:?} not implemented")]
    UnimplementedOpcode(Opcode),

    #[error("No code provided")]
    NoCode,

    #[error("Constant pool entry {0} is not present or loadable")]
    NotLoadable(u16),

    #[error("Constant pool entry {0} is not present or a method ref")]
    NotMethodRef(u16),

    #[error("Constant pool entry {0} is not present or a field ref")]
    NotFieldRef(u16),

    #[error("The method {class:?}.{name:?}:{desc:?} could not be resolved")]
    MethodNotFound {
        class: MString,
        name: MString,
        desc: MString,
    },

    #[error("The field {name:?}:{desc:?} could not be resolved")]
    FieldNotFound { name: MString, desc: DataType },

    #[error("Not enough operands on stack to pop, expected {expected} but only have {actual}")]
    NotEnoughArgs { expected: usize, actual: usize },

    #[error("Local var {0:?} is not a reference ({1:?})")]
    NotReference(usize, DataValue),

    #[error("Cannot load/store local var {requested}/{count}")]
    InvalidLocalVar { requested: usize, count: usize },

    #[error("Cannot load uninitialised local var {0}")]
    UninitialisedLoad(usize),

    #[error("Cannot pop from empty operand stack")]
    NoOperand,

    #[error("Expected non-array reference type for field op but got {0:?} instead")]
    InvalidOperandForFieldOp(DataType),

    #[error("Expected integer operand but got {0:?} instead")]
    InvalidOperandForIntOp(DataType),

    /// Not really an error
    #[error("Exception raised: {0:?}")]
    ExceptionRaised(Throwables),
}

impl From<Throwables> for InterpreterError {
    fn from(e: Throwables) -> Self {
        Self::ExceptionRaised(e)
    }
}
