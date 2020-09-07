use thiserror::*;

#[derive(Error, Debug, Clone)]
pub enum InterpreterError {
    #[error("Incomplete instruction at byte {0}")]
    IncompleteInstruction(usize),

    #[error("Invalid instruction 0x{0:x}")]
    InvalidInstruction(u8),

    #[error("No code provided")]
    NoCode,
}
