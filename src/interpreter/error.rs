use crate::error::Throwables;
use crate::interpreter::insn::Opcode;
use crate::types::DataValue;
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

    #[error("Local var {0:?} is not a reference ({1:?})")]
    NotReference(usize, DataValue),

    #[error("Cannot load invalid local var {requested}, max is {max}")]
    InvalidLoad { requested: usize, max: usize },

    #[error("Cannot load uninitialised local var {0}")]
    UninitialisedLoad(usize),

    /// Not really an error
    #[error("Exception raised")]
    ExceptionRaised(Throwables),
}

impl From<Throwables> for InterpreterError {
    fn from(e: Throwables) -> Self {
        Self::ExceptionRaised(e)
    }
}
