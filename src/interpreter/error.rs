use crate::error::Throwables;
use crate::interpreter::insn::Opcode;
use crate::types::{DataType, DataValue, ReturnType};
use cafebabe::mutf8::MString;
use thiserror::*;

use crate::alloc::VmRef;
use crate::class::{Class, ClassType};

// TODO combine repetetive errors for different data types

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

    #[error("Constant pool entry {0} is not present or an interface method ref")]
    NotInterfaceRef(u16),

    #[error("Constant pool entry {0} is not present or a field ref")]
    NotFieldRef(u16),

    #[error("Constant pool entry {0} is not present or a class ref")]
    NotClassRef(u16),

    #[error("The method {class:?}.{name:?}:{desc:?} could not be resolved")]
    MethodNotFound {
        class: MString,
        name: MString,
        desc: MString,
    },

    #[error("The field {name:?}:{desc:?} could not be resolved")]
    FieldNotFound {
        name: MString,
        desc: DataType<'static>,
    },

    #[error("Not enough operands on stack to pop, expected {expected} but only have {actual}")]
    NotEnoughArgs { expected: usize, actual: usize },

    #[error("Local var {0:?} is not a reference ({1:?})")]
    NotReference(usize, DataValue),

    #[error("Local var {local_var:?} is not {expected:?}, is actually {actual:?}")]
    NotExpectedType {
        local_var: usize,
        expected: DataType<'static>,
        actual: DataType<'static>,
    },

    #[error("Cannot load/store local var {requested}/{count}")]
    InvalidLocalVar { requested: usize, count: usize },

    #[error("Cannot load uninitialised local var {0}")]
    UninitialisedLoad(usize),

    #[error("Cannot pop from empty operand stack")]
    NoOperand,

    #[error("Expected non-array reference but got {0:?} instead")]
    UnexpectedArray(ClassType),

    #[error("Expected integer operand but got {0:?} instead")]
    InvalidOperandForIntOp(DataType<'static>),

    #[error("Expected float operand but got {0:?} instead")]
    InvalidOperandForFloatOp(DataType<'static>),

    #[error("Expected reference or returnAddress operand but got {0:?} instead")]
    InvalidOperandForAstore(DataType<'static>),

    #[error("Expected reference operand for object op but got {0:?} instead")]
    InvalidOperandForObjectOp(DataType<'static>),

    #[error("Class of type {0:?} is not an array")]
    NotAnArray(ClassType),

    #[error("Unexpected array type")]
    UnexpectedArrayType,

    #[error("Invalid array element type {0}")]
    InvalidArrayType(u8),

    #[error("Expected return type of {expected:?} but got {actual:?}")]
    InvalidReturnValue {
        expected: ReturnType<'static>,
        actual: Option<DataValue>,
    },

    #[error("Cannot pop from empty frame stack")]
    NoFrame,

    /// Not really an error
    #[error("Exception raised: {0:?}")]
    ExceptionRaised(Throwables),

    #[error("The native method {}.{name:?}:{desc:?} could not be resolved", .class.name())]
    NativeMethodNotFound {
        class: VmRef<Class>,
        name: MString,
        desc: MString,
    },
}

impl From<Throwables> for InterpreterError {
    fn from(e: Throwables) -> Self {
        Self::ExceptionRaised(e)
    }
}
