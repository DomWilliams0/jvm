mod error;
mod frame;
mod insn;
mod interp;

pub use error::InterpreterError;
pub use frame::Frame;
pub use insn::InstructionLookupTable;
pub use interp::{Interpreter, InterpreterResult, InterpreterState};
