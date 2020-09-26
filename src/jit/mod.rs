use crate::alloc::VmRef;
use crate::class::{Method, MethodCode, NativeCode};
use log::*;
use parking_lot::{Condvar, Mutex};
use std::fmt::{Debug, Formatter};
use std::hint::unreachable_unchecked;
use std::sync::mpsc;
use std::thread::JoinHandle;

// TODO reorganise into modules

pub struct JitThread {
    thread: JoinHandle<()>,
}

/// Submits work to jit thread
pub struct JitClient {
    tx: mpsc::Sender<JitRequest>,
}

pub enum JitRequest {
    CompileTrampoline {
        method: VmRef<Method>,
        fn_ptr: usize,
    },
    Exit,
}

#[derive(Debug)]
enum CompileState {
    NotCompiled,
    Queued,
    Compiling,
    Compiled(()),
    Failed(CompileError),
}

pub type CompileError = ();

pub struct CompiledCode {
    mutex: Mutex<CompileState>,
    cvar: Condvar,
    code_type: CodeType,
}

pub enum CodeType {
    /// Trampoline to native fn at this address
    Trampoline(usize),

    /// JIT'd Java method
    Jit,
}

impl JitThread {
    pub fn start() -> (Self, JitClient) {
        let (tx, rx) = mpsc::channel();
        let thread = std::thread::Builder::new()
            .name("jit".to_owned())
            .spawn(|| jit_loop(rx))
            .expect("thread creation failed");
        debug!("started jit thread");

        let server = JitThread { thread };
        let client = JitClient { tx };

        (server, client)
    }
}

fn jit_loop(rx: mpsc::Receiver<JitRequest>) {
    while let Ok(JitRequest::CompileTrampoline { method, fn_ptr }) = rx.recv() {
        // safety: checked is_native_and_bound()
        unsafe {
            method.set_state(CompileState::Compiling);
        }

        // TODO actually compile
        let trampoline = ();
        todo!("compile");

        // update method code reference
        // safety: checked is_native_and_bound()
        unsafe {
            method.set_state(CompileState::Compiled(trampoline));
        }
    }

    info!("jit thread exiting");
}

impl Method {
    /// Must have checked is_native_and_bound() first
    unsafe fn set_state(&self, state: CompileState) {
        self.do_with_state(|s| *s = state);
    }

    /// Must have checked is_native_and_bound() first
    unsafe fn do_with_state<R>(&self, f: impl FnOnce(&mut CompileState) -> R) -> R {
        let native_code = match self.code() {
            MethodCode::Native(native) => native,
            _ => unreachable_unchecked(),
        };

        let native_guard = native_code.lock();
        let compiled_code = match &*native_guard {
            NativeCode::Bound(code) => code,
            _ => unreachable_unchecked(),
        };

        todo!()
        // let mut state_guard = compiled_code.mutex.lock();
        // f(&mut *state_guard)
    }

    fn is_native_and_bound(&self) -> bool {
        let native_code = match self.code() {
            MethodCode::Native(native) => native,
            _ => return false,
        };

        let native_guard = native_code.lock();
        match &*native_guard {
            NativeCode::Bound(_) => true,
            _ => false,
        }
    }
}

impl JitClient {
    /// Trampoline should not already be compiled
    // TODO return result
    pub fn queue_trampoline(&self, method: VmRef<Method>, fn_ptr: usize) -> bool {
        // ensure compilable
        if !method.is_native_and_bound() {
            warn!("method {:?} is either not native or unbound", method.name());
            return false;
        }

        // ensure not already queued or compiled, then update state to queued
        // safety: checked is_native_and_bound()
        unsafe {
            let success = method.do_with_state(|state| match *state {
                CompileState::NotCompiled => {
                    *state = CompileState::Queued;
                    true
                }
                _ => {
                    warn!("method {:?} is already queued or compiled", method.name());
                    false
                }
            });

            if !success {
                return false;
            }
        }

        trace!(
            "queueing jit compilation of method trampoline {:?} -> {:#x}",
            method.name(),
            fn_ptr
        );
        self.tx
            .send(JitRequest::CompileTrampoline { method, fn_ptr })
            .is_ok()
    }
}

impl CompiledCode {
    pub fn new(ty: CodeType) -> Self {
        CompiledCode {
            mutex: Mutex::new(CompileState::NotCompiled),
            cvar: Condvar::new(),
            code_type: ty,
        }
    }

    pub fn ensure_compiled(&self) -> Result<(), CompileError> {
        let mut state = self.mutex.lock();
        match &*state {
            CompileState::NotCompiled => unreachable!("not queued"), // TODO queue here?
            CompileState::Queued | CompileState::Compiling => {
                // wait for completion then try again
                debug!("waiting for compilation to finish");
                self.cvar.wait(&mut state);
                self.ensure_compiled()
            }
            CompileState::Compiled(_) => Ok(()),
            CompileState::Failed(err) => Err(err.clone()),
        }
    }
}

impl Debug for CompiledCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompiledCode({:?})", self.mutex.lock())
    }
}
