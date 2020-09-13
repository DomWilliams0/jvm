use crate::alloc::{vmref_alloc_exception, VmRef};
use crate::class::{Class, Method, Object};
use log::*;

use crate::interpreter::error::InterpreterError;
use crate::interpreter::frame::{Frame, FrameStack};
use crate::interpreter::insn::{Bytecode, ExecuteResult};
use crate::thread;
use crate::types::DataValue;
use std::cell::RefCell;

pub enum ProgramCounter {
    Java(usize),
    Native,
}

pub struct Interpreter(RefCell<InterpreterState>);

struct InterpreterState {
    pc: ProgramCounter,
    frames: FrameStack,
}

impl Interpreter {
    // TODO get current class
    // TODO get current method
    // TODO get current frame

    pub fn execute_method(
        &self,
        class: VmRef<Class>,
        method: VmRef<Method>,
        this: Option<VmRef<Object>>,
        args: impl Iterator<Item = DataValue>,
    ) -> Result<(), InterpreterError> {
        // TODO push frame onto stack
        let mut frame = Frame::new_from_method(method, class, this, args)?;
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

                let mut err = Ok(());
                for insn in bytecode.instructions() {
                    match insn.execute(frame, &thread) {
                        Err(InterpreterError::ExceptionRaised(exc)) => {
                            // TODO abrupt exit with proper exception creation
                            thread.set_exception(vmref_alloc_exception(exc)?);
                            todo!("handle exception")
                        }
                        Err(e) => {
                            error!("interpreter error: {}", e);
                            err = Err(e);
                            // TODO dont just bubble same error up through whole call stack
                            break;
                        }
                        Ok(ExecuteResult::Continue) => {}
                        Ok(ExecuteResult::Return) => {
                            // TODO handle return
                            todo!("return")
                        }
                    }
                }

                match err {
                    Ok(_) => debug!("exiting method {:?} successfully", frame.method.name()),
                    Err(e) => {
                        debug!(
                            "exiting method {:?} with interpreter error: {}",
                            frame.method.name(),
                            e
                        );
                        return Err(e);
                    }
                }
            }
        }

        todo!()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let state = InterpreterState {
            pc: ProgramCounter::Native,
            frames: FrameStack::new(),
        };

        Interpreter(RefCell::new(state))
    }
}
