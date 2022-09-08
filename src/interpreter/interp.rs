use crate::alloc::{vmref_from_raw, VmRef};
use log::*;

use crate::error::{Throwable, Throwables};
use crate::interpreter::frame::{Frame, FrameStack, JavaFrame, NativeFrame, NativeFrameInner};
use crate::interpreter::insn::{get_insn, InstructionBlob, PostExecuteAction};
use crate::thread;

use crate::class::{Class, FunctionArgs, Method, NativeFunction};
use crate::interpreter::InterpreterError;

use crate::types::{DataType, DataValue, PrimitiveDataType, ReturnType};
use cafebabe::AccessFlags;

use std::cell::{RefCell, RefMut};

use std::fmt::{Debug, Formatter};

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

pub struct InterpFrameStackPrinter<'a>(&'a FrameStack);

impl InterpreterState {
    pub fn push_frame(&mut self, frame: Frame) {
        trace!(
            "pushed new frame, stack depth is now {}: {:?}",
            self.frames.depth() + 1,
            frame
        );
        self.frames.push(frame, 0);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        match self.frames.pop() {
            Some((f, _)) => {
                trace!(
                    "popped frame, stack depth is now {}: {:?}",
                    self.frames.depth(),
                    self.frames.top(),
                );
                Some(f)
            }
            None => {
                error!("no frames to pop");
                None
            }
        }
    }

    pub fn current_frame_mut(&mut self) -> &mut JavaFrame {
        self.frames.top_java_mut().expect("no java frame").0
    }

    /// Pc returned is for the next instruction
    pub fn current_frame_mut_with_next_pc(&mut self) -> (&mut JavaFrame, usize) {
        let (frame, pc) = self.frames.top_java_mut().expect("no java frame");
        (frame, *pc)
    }

    pub fn current_frame_mut_checked(&mut self) -> Option<&mut JavaFrame> {
        self.frames.top_java_mut().map(|(frame, _)| frame)
    }

    pub fn current_native_frame_mut(&mut self) -> &mut NativeFrame {
        self.frames.top_native_mut().expect("no native frame")
    }

    fn current_method(&self) -> Option<&Method> {
        match self.frames.top()? {
            Frame::Java(frame) => Some(&frame.method),
            Frame::Native(NativeFrame {
                inner: NativeFrameInner::Method { method, .. },
                ..
            }) => Some(method),
            _ => None,
        }
    }

    pub fn current_class(&self) -> Option<&VmRef<Class>> {
        match self.frames.top()? {
            Frame::Java(frame) => Some(&frame.class),
            Frame::Native(NativeFrame {
                inner: NativeFrameInner::Method { class, .. },
                ..
            }) => Some(class),
            _ => None,
        }
    }

    pub fn return_value_to_caller(
        &mut self,
        ret_val: Option<DataValue>,
    ) -> Result<(), InterpreterError> {
        // check return type matches sig
        // TODO catch this at verification time

        let ret_val = {
            let method_ret = self.current_method().unwrap().return_type();

            let ret_val_orig = ret_val.clone(); // for logging only
            let new_val = method_ret.convert_value(ret_val).map_err(|val| {
                InterpreterError::InvalidReturnValue {
                    expected: method_ret.to_owned(),
                    actual: val,
                }
            })?;

            if new_val != ret_val_orig {
                trace!(
                    "allowing return value {:?} for return type {:?}",
                    ret_val_orig,
                    method_ret
                );
            }

            ret_val_orig
        };

        // pop frame
        if self.pop_frame().is_none() {
            return Err(InterpreterError::NoFrame);
        }

        // push return value onto caller's stack or set in TLS for e.g. native method
        if let Some(val) = ret_val {
            if let Some(caller) = self.current_frame_mut_checked() {
                caller.operand_stack.push(val);
            } else {
                thread::get().set_return_value(val);
            }
        }

        Ok(())
    }

    pub fn print_frame_stack(&self) -> InterpFrameStackPrinter {
        InterpFrameStackPrinter(&self.frames)
    }
}

impl Debug for InterpFrameStackPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frame stack (depth={}):", self.0.depth())?;
        for (i, frame) in self.0.iter().enumerate() {
            write!(f, "\n * {})\t{:?}", i, frame)?;
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
            trace!("{:?}", self.state.borrow().print_frame_stack());
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
            trace!("invoking native method {:?}", native.inner);

            // dismantle frame - unwraps move the values out here as the first and only call
            let func = native.function.take().unwrap();
            let mut args = native.args.take().unwrap();
            let args = FunctionArgs::from(args.as_mut());

            // for jni calls we need the method reference
            let method = match (&func, &native.inner) {
                (NativeFunction::Jni { .. }, NativeFrameInner::Method { method, .. }) => {
                    Some(method.clone())
                }
                _ => None,
            };

            // drop mutable ref to interpreter to go native - it might recursively call this interpreter method
            drop(state);

            // go native!! best of luck
            let result = match func {
                NativeFunction::Internal(func) => {
                    // internal native call, just call it
                    func(args)
                }
                NativeFunction::Jni { trampoline, .. } => {
                    let method = method.unwrap(); // always populated above
                    let trampoline = trampoline.expect("trampoline not initialized");

                    let cls_ref = if method.flags().is_static() {
                        method.class().clone()
                    } else {
                        todo!("instance method")
                    };
                    let jni_ref = thread.jni_env();

                    debug!("invoking jni function {}", method);
                    let raw_result = trampoline.invoke(jni_ref, cls_ref, args);
                    // check exception
                    let thread = thread::get();
                    if let Some(exc) = thread.exception() {
                        Err(exc)
                    } else {
                        match method.return_type() {
                            ReturnType::Returns(ty) => {
                                let val =
                                    unsafe { DataValue::from_raw_return_value(raw_result, ty) };
                                trace!("jni function returned {:#x} == {:?}", raw_result, val);
                                Ok(Some(val))
                            }
                            _ => {
                                // void
                                Ok(None)
                            }
                        }
                    }
                }
                NativeFunction::JniDirect(_) => {
                    // jni direct functions are called differently
                    unreachable!("direct jni functions are called differently")
                }
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

    /// Panics if not called from a method
    pub fn with_current_frame<R>(&self, f: impl FnOnce(&Frame) -> R) -> R {
        let state = self.state.borrow();
        let frame = state.frames.top().expect("must be called from a method");
        f(frame)
    }

    /// Panics if not in a native frame
    pub fn with_current_native_frame<R>(&self, f: impl FnOnce(&NativeFrame) -> R) -> R {
        let state = self.state.borrow();
        if let Some(Frame::Native(frame)) = state.frames.top() {
            f(frame)
        } else {
            unreachable!("not in a jni function ({:?})", state.frames.top())
        }
    }

    /// Called in top down order, first is current frame
    pub fn with_frames(&self, mut f: impl FnMut(&Frame)) {
        let state = self.state.borrow();
        for frame in state.frames.iter() {
            f(frame);
        }
    }

    /// 0 = current, 1 = calling, etc.
    ///
    /// Fn is not called if frame doesn't exist
    pub fn with_frame<R>(&self, n: usize, f: impl FnOnce(&Frame) -> R) -> Option<R> {
        let state = self.state.borrow();
        let ret = state.frames.iter().nth(n).map(|frame| f(frame));

        ret
    }

    pub fn execute_native_frame<R>(&self, frame: NativeFrame, f: impl FnOnce() -> R) -> R {
        self.state_mut().push_frame(Frame::Native(frame));
        let ret = f();
        self.state_mut().pop_frame();
        ret
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

impl DataValue {
    unsafe fn from_raw_return_value(ret: u64, ty: &DataType) -> Self {
        use DataType::*;
        use DataValue as V;
        use PrimitiveDataType::*;

        let ret_ptr = &ret as *const _;
        match ty {
            Primitive(Boolean) => V::Boolean(*(ret_ptr as *const _)),
            Primitive(Byte) => V::Byte(*(ret_ptr as *const _)),
            Primitive(Short) => V::Short(*(ret_ptr as *const _)),
            Primitive(Int) => V::Int(*(ret_ptr as *const _)),
            Primitive(Long) => V::Long(*(ret_ptr as *const _)),
            Primitive(Char) => V::Char(*(ret_ptr as *const _)),
            Primitive(Float) => V::Float(*(ret_ptr as *const _)),
            Primitive(Double) => V::Double(*(ret_ptr as *const _)),
            ReturnAddress => V::ReturnAddress(*(ret_ptr as *const _)),
            Reference(_) => V::Reference(vmref_from_raw(ret as *const _)),
        }
    }
}
