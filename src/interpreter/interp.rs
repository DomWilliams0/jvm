use crate::alloc::VmRef;

use crate::error::Throwable;
use crate::interpreter::frame::{Frame, FrameStack, JavaFrame};
use crate::interpreter::insn::{get_insn, InstructionBlob, Opcode, PostExecuteAction, Putstatic};
use crate::thread;
use std::cell::{RefCell, RefMut};

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
        log::trace!(
            "pushed new frame, stack depth is now {}: {:?}",
            self.frames.depth() + 1,
            frame
        );
        self.frames.push(frame, 0);
    }

    pub fn current_frame_mut(&mut self) -> &mut JavaFrame {
        self.frames.top_java_mut().expect("no java frame").0
    }
}

impl Interpreter {
    pub fn execute_until_return(&self) -> InterpreterResult {
        let mut depth = 1;

        while depth != 0 {
            match self.execute() {
                PostExecuteAction::MethodCall => depth += 1,
                PostExecuteAction::Return => depth -= 1,
                PostExecuteAction::Exception(exc) => {
                    thread::get().set_exception(VmRef::new(Throwable {
                        class_name: exc.symbol(),
                    }));
                    return InterpreterResult::Exception;
                }
                PostExecuteAction::Continue => unreachable!(),
            }
        }

        InterpreterResult::Success
    }

    fn execute(&self) -> PostExecuteAction {
        let mut insn_blob = InstructionBlob::default();
        let thread = thread::get();

        let mut state = self.state_mut();
        if let Some(_native) = state.frames.top_native_mut() {
            todo!("native frame")
        }

        loop {
            // get current java frame
            let (frame, pc) = state.frames.top_java_mut().expect("no frame");

            // get current instruction
            let code = frame.code.as_ref();
            let (new_pc, opcode) = get_insn(code, *pc, &mut insn_blob).expect("bad opcode");
            *pc = new_pc;

            // lookup execute function
            log::trace!("executing {:?}", opcode);
            let exec_fn = thread.global().insn_lookup().resolve(opcode);
            let result = exec_fn(&insn_blob, &mut *state);

            match result {
                PostExecuteAction::Continue => {
                    // keep executing this frame
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
