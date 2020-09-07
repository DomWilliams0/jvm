use crate::alloc::{vmref_alloc_exception, VmRef};
use crate::class::{Class, Method, Object};
use log::*;

use crate::interpreter::error::InterpreterError;
use crate::interpreter::frame::{Frame, FrameStack};
use crate::interpreter::insn::{Bytecode, ExecuteResult};
use crate::thread;

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
        let mut frame = Frame::new_from_method(method, class, this)?;
        let thread = thread::get();

        match &mut frame {
            Frame::Native(_) => {
                // TODO native frames
                unimplemented!()
            }
            Frame::Java(frame) => {
                // get bytecode
                // TODO verify, "compile" and cache instructions
                let bytecode = Bytecode::parse(&frame.code)?;

                for insn in bytecode.instructions() {
                    match insn.execute(frame, &thread) {
                        Err(InterpreterError::ExceptionRaised(exc)) => {
                            // TODO abrupt exit with proper exception creation
                            thread.set_exception(vmref_alloc_exception(exc)?);
                            todo!("handle exception")
                        }
                        Err(e) => {
                            error!("interpreter error: {}", e);
                            return Err(e);
                        }
                        Ok(ExecuteResult::Continue) => {}
                        Ok(ExecuteResult::Return) => {
                            // TODO handle return
                            todo!("return")
                        }
                    }
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
