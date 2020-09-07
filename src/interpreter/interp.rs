use crate::alloc::VmRef;
use crate::class::{Class, Method, Object};

use crate::interpreter::error::InterpreterError;
use crate::interpreter::frame::{Frame, FrameDeets, FrameStack};
use crate::interpreter::insn::Bytecode;

pub enum ProgramCounter {
    Java(usize),
    Native,
}

pub struct Interpreter {
    pc: ProgramCounter,
    frames: FrameStack,
}

impl Interpreter {
    // TODO get current class
    // TODO get current method
    // TODO get current frame

    pub fn execute_method(
        &mut self,
        class: VmRef<Class>,
        method: VmRef<Method>,
        this: Option<VmRef<Object>>,
    ) -> Result<(), InterpreterError> {
        // push new frame
        let frame = Frame::new_from_method(method, class, this)?;

        match frame.deets() {
            FrameDeets::Native => {
                // TODO native frames
                unimplemented!()
            }
            FrameDeets::Java { code, .. } => {
                // get bytecode
                // TODO verify, "compile" and cache instructions

                let bytecode = Bytecode::parse(code)?;

                for insn in bytecode.instructions() {
                    // TODO execute
                }
            }
        }

        todo!()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            pc: ProgramCounter::Native,
            frames: FrameStack::new(),
        }
    }
}
