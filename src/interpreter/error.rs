use crate::error::Throwables;
use crate::interpreter::insn::Opcode;
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

    #[error("Constant pool entry is not present or loadable")]
    NotLoadable(u16),

    /// Not really an error
    #[error("Exception raised")]
    ExceptionRaised(Throwables),
}

impl From<Throwables> for InterpreterError {
    fn from(e: Throwables) -> Self {
        Self::ExceptionRaised(e)
    }
}
