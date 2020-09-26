use crate::alloc::VmRef;
use crate::class::Method;
use log::*;
use parking_lot::{Condvar, Mutex};
use std::fmt::{Debug, Formatter};
use std::sync::mpsc;
use std::thread::JoinHandle;

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
    Compiling,
    Compiled(()),
}

pub struct CompiledCode {
    mutex: Mutex<()>,
    cvar: Condvar,
    state: CompileState,
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
        todo!("{:?}", method.name());
    }

    info!("jit thread exiting");
}

impl JitClient {
    /// Trampoline should not already be compiled
    pub fn queue_trampoline(&self, method: VmRef<Method>, fn_ptr: usize) {
        // TODO debug assert not already compiled
        trace!(
            "queueing jit compilation of method trampoline {:?} -> {:#x}",
            method.name(),
            fn_ptr
        );
        let _ = self
            .tx
            .send(JitRequest::CompileTrampoline { method, fn_ptr });
    }
}

impl CompiledCode {
    pub fn new(ty: CodeType) -> Self {
        CompiledCode {
            mutex: Mutex::new(()),
            cvar: Condvar::new(),
            state: CompileState::NotCompiled,
            code_type: ty,
        }
    }
}

impl Debug for CompiledCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompiledCode({:?})", self.state)
    }
}
