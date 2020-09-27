use crate::alloc::VmRef;
use log::*;

use crate::error::{Throwable, Throwables};
use crate::interpreter::frame::{Frame, FrameStack, JavaFrame};
use crate::interpreter::insn::{get_insn, InstructionBlob, PostExecuteAction};
use crate::thread;

use crate::class::{FunctionArgs, Method, NativeFunction};
use crate::interpreter::InterpreterError;
use crate::types::DataValue;
use std::cell::{RefCell, RefMut};

#[derive(Debug)]
pub enum InterpreterResult {
    Success,
    Exception,
}

pub struct InterpreterState {
    frames: FrameStack,
}

pub struct Interpreter {
    state: RefCell<InterpreterState>,
}

impl InterpreterState {
    pub fn push_frame(&mut self, frame: Frame) {
        trace!(
            "pushed new frame, stack depth is now {}: {:?}",
            self.frames.depth() + 1,
            frame
        );
        self.frames.push(frame, 0);
    }

    pub fn pop_frame(&mut self) -> bool {
        match self.frames.pop() {
            Some(_) => {
                trace!(
                    "popped frame, stack depth is now {}: {:?}",
                    self.frames.depth(),
                    self.frames.top(),
                );
                true
            }
            None => {
                error!("no frames to pop");
                false
            }
        }
    }

    pub fn current_frame_mut(&mut self) -> &mut JavaFrame {
        self.frames.top_java_mut().expect("no java frame").0
    }

    pub fn current_frame_mut_checked(&mut self) -> Option<&mut JavaFrame> {
        self.frames.top_java_mut().map(|(frame, _)| frame)
    }

    fn current_method(&self) -> Option<&Method> {
        match self.frames.top()? {
            Frame::Java(frame) => Some(&frame.method),
            Frame::Native(frame) => Some(&frame.method),
        }
    }

    pub fn return_value_to_caller(
        &mut self,
        val: Option<DataValue>,
    ) -> Result<(), InterpreterError> {
        // check return type matches sig
        // TODO catch this at verification time

        let method_ret = self.current_method().unwrap().return_type();
        if !method_ret.matches(val.as_ref()) {
            return Err(InterpreterError::InvalidReturnValue {
                expected: method_ret.to_owned(),
                actual: val,
            });
        }

        // pop frame
        if !self.pop_frame() {
            return Err(InterpreterError::NoFrame);
        }

        // push return value onto caller's stack or set in TLS for e.g. native method
        if let Some(val) = val {
            if let Some(caller) = self.current_frame_mut_checked() {
                caller.operand_stack.push(val);
            } else {
                thread::get().set_return_value(val);
            }
        }

        Ok(())
    }
}

impl InterpreterResult {
    pub fn is_success(&self) -> bool {
        matches!(self, InterpreterResult::Success)
    }

    pub fn into_result(self) -> Result<(), VmRef<Throwable>> {
        if self.is_success() {
            Ok(())
        } else {
            let exc = thread::get()
                .exception()
                .expect("interpreter error should have set an exception");
            Err(exc)
        }
    }
}

impl Interpreter {
    pub fn execute_frame(&self, frame: Frame) -> Result<Option<DataValue>, VmRef<Throwable>> {
        self.state_mut().push_frame(frame);
        self.execute_until_return()
            .into_result()
            .map(|_| thread::get().take_return_value())
    }

    pub fn execute_until_return(&self) -> InterpreterResult {
        let mut depth = 1;

        while depth != 0 {
            match self.execute() {
                PostExecuteAction::MethodCall => depth += 1,
                PostExecuteAction::Return => depth -= 1,
                PostExecuteAction::ThrowException(exc) => {
                    thread::get().set_exception(exc.into());
                    return InterpreterResult::Exception;
                }
                PostExecuteAction::Exception(exc) => {
                    thread::get().set_exception(exc);
                    return InterpreterResult::Exception;
                }
                PostExecuteAction::JmpAbsolute(new_pc) => {
                    let mut state = self.state_mut();
                    let (_, pc) = state.frames.top_java_mut().unwrap(); // jmps only happen in java frames

                    debug!("jmping to insn {:?}", new_pc);
                    *pc = new_pc;
                }
                PostExecuteAction::ClassInit(cls) => {
                    debug!(
                        "initialising class {:?} before replaying last instruction",
                        cls.name()
                    );

                    if let Err(err) = cls.ensure_init() {
                        warn!("class initialisation failed: {:?}", err);
                        thread::get().set_exception(err.into());
                        return InterpreterResult::Exception;
                    }
                }

                PostExecuteAction::Jmp(_) => {
                    unreachable!("execute() should have filtered out relative jumps")
                }
                PostExecuteAction::Continue => {
                    unreachable!("execute() should have filtered out continues")
                }
            }
        }

        InterpreterResult::Success
    }

    fn execute(&self) -> PostExecuteAction {
        // TODO pass these into execute()
        let mut insn_blob = InstructionBlob::default();
        let thread = thread::get();
        let mut state = self.state_mut();

        if let Some(native) = state.frames.top_native_mut() {
            trace!("invoking native method {}", native.method,);

            // dismantle frame
            let func = native.function;
            let mut args = native.args.take().unwrap(); // this happens once only the upon call
            let args = FunctionArgs::from(args.as_mut());

            // drop mutable ref to interpreter to go native - this might recursively call this interpreter method
            drop(state);

            // go native!! best of luck
            let result = match func {
                NativeFunction::Internal(func) => func(args),
            };

            let return_value = match result {
                Err(e) => {
                    debug!("native method threw exception: {:?}", e);
                    return PostExecuteAction::Exception(e);
                }
                Ok(ret) => ret,
            };

            // we made it! go mutable again to push return value onto caller's stack
            return match self.state_mut().return_value_to_caller(return_value) {
                Err(err) => {
                    error!("interpreter error: {}", err);
                    // TODO better handling of interpreter error
                    PostExecuteAction::ThrowException(Throwables::Other("java/lang/InternalError"))
                }
                Ok(()) => PostExecuteAction::Return,
            };
        }

        loop {
            // get current java frame
            let (frame, pc) = state.frames.top_java_mut().expect("no frame");

            // get current instruction
            let code = frame.code.as_ref();
            let old_pc = *pc;
            let (new_pc, opcode) = get_insn(code, *pc, &mut insn_blob).expect("bad opcode");
            *pc = new_pc;

            // lookup execute function
            trace!(
                "{}: executing {:?} ({:?})",
                old_pc,
                opcode,
                state.frames.top().unwrap()
            );
            let exec_fn = thread.global().insn_lookup().resolve(opcode);
            let result = exec_fn(&insn_blob, &mut *state);

            match result {
                PostExecuteAction::Continue => {
                    // keep executing this frame
                }
                PostExecuteAction::Jmp(offset) => {
                    // jmp is relative to this opcode, make absolute
                    let new_offset = offset + (old_pc as i32);
                    trace!("adjusted jmp offset from {:?} to {:?}", offset, new_offset);
                    return PostExecuteAction::JmpAbsolute(new_offset as usize);
                }
                ret @ PostExecuteAction::ClassInit(_) => {
                    // after the class is initialised we want to replay the opcode that caused it,
                    // so rewind pc
                    trace!(
                        "rewinding pc from {} to {} to replay after class init",
                        new_pc,
                        old_pc
                    );

                    drop(state);
                    let mut state = self.state_mut();
                    let (_, pc) = state.frames.top_java_mut().unwrap();
                    *pc = old_pc;

                    return ret;
                }
                ret => return ret,
            }
        }
    }

    pub fn state_mut(&self) -> RefMut<InterpreterState> {
        self.state.borrow_mut()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            state: RefCell::new(InterpreterState {
                frames: FrameStack::new(),
            }),
        }
    }
}
