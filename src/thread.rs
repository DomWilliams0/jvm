use std::cell::{Ref, RefCell};
use std::mem::MaybeUninit;
use std::sync::Arc;

use log::*;
use parking_lot::RwLock;

use crate::alloc::VmRef;

use crate::error::Throwable;
use crate::interpreter::Interpreter;
use crate::jvm::JvmGlobalState;
use crate::types::DataValue;
use std::thread::ThreadId;

/// Each thread has its own in TLS
pub struct JvmThreadState {
    jvm: Arc<JvmGlobalState>,
    thread_handle: ThreadId,
    exception: RefCell<Option<VmRef<Throwable /* TODO vmobject */>>>,
    interpreter: RefCell<Interpreter>,
    /// Return value of last call to interpreter.execute_until_return()
    return_value: RefCell<Option<DataValue>>,
}

thread_local! {
    static THREAD_STATE: RefCell<MaybeUninit<Arc<JvmThreadState>>> = RefCell::new(MaybeUninit::uninit());
    static STATE_ENABLED: RwLock<bool> = RwLock::new(false);
}

pub fn is_initialised() -> bool {
    STATE_ENABLED.with(|b| *b.read())
}

pub fn initialise(state: Arc<JvmThreadState>) -> bool {
    STATE_ENABLED.with(|b| {
        let mut guard = b.write();

        if *guard {
            // already initialised
            return false;
        }

        THREAD_STATE.with(|tls| {
            let mut tls = tls.borrow_mut();
            *tls = MaybeUninit::new(state);
        });

        *guard = true;

        debug!(
            "initialised thread local state for {:?}",
            std::thread::current().id()
        );
        true
    })
}

pub fn uninitialise() -> bool {
    STATE_ENABLED.with(|b| {
        let mut guard = b.write();

        if !*guard {
            // not initialised
            return false;
        }

        THREAD_STATE.with(|tls| {
            let mut tls = tls.borrow_mut();

            // safety: asserted initialised
            let state = unsafe { tls.as_ptr().read() };

            // state is now local, blat the copy in tls
            *tls = MaybeUninit::uninit();

            drop(state);
        });

        *guard = false;

        debug!(
            "uninitialised thread local state for {:?}",
            std::thread::current().id()
        );
        true
    })
}

pub fn get_checked() -> Option<Arc<JvmThreadState>> {
    STATE_ENABLED.with(|b| {
        let guard = b.read();
        if !*guard {
            None
        } else {
            Some(THREAD_STATE.with(|tls| {
                let tls = tls.borrow();
                // safety: asserted initialised
                unsafe {
                    let ptr = tls.as_ptr();
                    Arc::clone(&*ptr)
                }
            }))
        }
    })
}

pub fn get() -> Arc<JvmThreadState> {
    STATE_ENABLED.with(|b| {
        let guard = b.read();
        assert!(*guard, "thread not initialised");

        THREAD_STATE.with(|tls| {
            let tls = tls.borrow();
            // safety: asserted initialised
            unsafe {
                let ptr = tls.as_ptr();
                Arc::clone(&*ptr)
            }
        })
    })
}

impl JvmThreadState {
    pub fn new(jvm: Arc<JvmGlobalState>) -> Self {
        Self {
            interpreter: RefCell::new(Interpreter::default()),
            jvm,
            thread_handle: std::thread::current().id(),
            exception: RefCell::new(None),
            return_value: RefCell::new(None),
        }
    }

    pub fn set_exception(&self, exc: VmRef<Throwable>) {
        let mut current = self.exception.borrow_mut();
        if let Some(old) = current.replace(exc) {
            debug!("overwrote old exception with new exception: {:?}", old);
        }
        debug!("set exception: {:?}", current.as_ref().unwrap());
    }

    pub fn set_return_value(&self, val: DataValue) {
        debug!("set return value: {:?}", val);
        self.return_value.replace(Some(val));
    }

    pub fn take_return_value(&self) -> Option<DataValue> {
        self.return_value.borrow_mut().take()
    }

    pub fn exception(&self) -> Option<VmRef<Throwable>> {
        self.exception.borrow().clone()
    }

    pub fn thread(&self) -> ThreadId {
        self.thread_handle
    }

    pub fn interpreter(&self) -> Ref<Interpreter> {
        self.interpreter.borrow()
    }

    pub fn global(&self) -> &JvmGlobalState {
        &self.jvm
    }
}
