use std::cell::{Cell, Ref, RefCell};
use std::iter::once;
use std::mem::MaybeUninit;
use std::sync::Arc;

use log::*;

use crate::alloc::VmRef;

use crate::class::Object;
use crate::error::{Throwable, VmResult};
use crate::exec_helper::{ExecHelper, ExecHelperStandalone};
use crate::interpreter::Interpreter;
use crate::jni::sys::JNIEnv;
use crate::jvm::JvmGlobalState;
use crate::types::DataValue;

use cafebabe::mutf8::StrExt;

use std::thread::ThreadId;

/// Each thread has its own in TLS
pub struct JvmThreadState {
    jvm: Arc<JvmGlobalState>,
    thread_handle: ThreadId,
    exception: RefCell<Option<VmRef<Throwable /* TODO vmobject */>>>,
    interpreter: RefCell<Interpreter>,
    /// Return value of last call to interpreter.execute_until_return()
    return_value: RefCell<Option<DataValue>>, // TODO really needed?
    state: Cell<ThreadState>,
    /// java.lang.VMThread instance for this thread
    jvm_thread: RefCell<Option<VmRef<Object>>>,
}

/// Maps mostly to java.lang.Thread.State
pub enum ThreadState {
    /// Thread state for a thread blocked waiting for a monitor lock.
    Blocked,
    /// Thread state for a thread which has not yet started.
    New,
    /// Thread state for a runnable thread.
    Runnable,
    /// Thread state for a terminated thread.
    Terminated,
    /// Thread state for a waiting thread with a specified waiting time.
    TimedWaiting,
    /// Thread state for a waiting thread.
    Waiting,
}

thread_local! {
    static THREAD_STATE: RefCell<MaybeUninit<Arc<JvmThreadState>>> = RefCell::new(MaybeUninit::uninit());
    static STATE_ENABLED: Cell<bool> = Cell::new(false);
}

pub fn is_initialised() -> bool {
    STATE_ENABLED.with(|b| b.get())
}

pub fn initialise(state: Arc<JvmThreadState>) -> bool {
    STATE_ENABLED.with(|b| {
        if b.get() {
            // already initialised
            return false;
        }

        THREAD_STATE.with(|tls| {
            let mut tls = tls.borrow_mut();
            *tls = MaybeUninit::new(state.clone());
        });

        b.set(true);

        debug!(
            "initialised thread local state for {:?}",
            std::thread::current().id()
        );
        true
    })
}

pub fn init_main_vmthread() -> VmResult<()> {
    let state = get();
    let classloader = state.global().class_loader();
    let helper = state.exec_helper();

    // find root threadgroup
    let threadgroup_cls = classloader.get_bootstrap_class("java/lang/ThreadGroup");
    let root_threadgroup = threadgroup_cls.get_static_field("root", "Ljava/lang/ThreadGroup;");

    // create vmthread instance
    let (vmthread_instance, _) = helper.instantiate("java/lang/VMThread")?;

    // create thread instance
    let thread_instance = helper.instantiate_and_invoke_constructor(
        "java/lang/Thread",
        "(Ljava/lang/VMThread;Ljava/lang/String;IZ)V",
        [
            DataValue::Reference(vmthread_instance.clone()), // vmthread
            DataValue::Reference(Object::new_string_utf8("main")?), // name
            DataValue::Int(5),                               // priority
            DataValue::Boolean(false),                       // daemon
        ]
        .into_iter(),
    )?;

    // touch up vmthread fields
    ExecHelperStandalone.set_instance_field(
        &vmthread_instance,
        "thread",
        DataValue::Reference(thread_instance.clone()),
    )?;
    // set_field("vmdata", DataValue::Reference(todo!()))?; // TODO vmthread struct
    state.set_vmthread(vmthread_instance);

    // attach to thread group
    helper.invoke_instance_method(
        root_threadgroup.clone(),
        threadgroup_cls,
        "addThread",
        "(Ljava/lang/Thread;)V",
        once(DataValue::Reference(thread_instance.clone())),
    )?;

    ExecHelperStandalone.set_instance_field(&thread_instance, "group", root_threadgroup)?;

    // inheritable thread local is required apparently
    helper.invoke_static_method(
        "java/lang/InheritableThreadLocal",
        "newChildThread",
        "(Ljava/lang/Thread;)V",
        once(DataValue::Reference(thread_instance)),
    )?;

    Ok(())
}

pub fn uninitialise() -> bool {
    STATE_ENABLED.with(|b| {
        if !b.get() {
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

        b.set(false);

        debug!(
            "uninitialised thread local state for {:?}",
            std::thread::current().id()
        );
        true
    })
}

pub fn get_checked() -> Option<Arc<JvmThreadState>> {
    STATE_ENABLED.with(|b| {
        if !b.get() {
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
        assert!(b.get(), "thread not initialised");

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
            state: Cell::new(ThreadState::New),
            jvm_thread: RefCell::new(None),
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

    pub fn vm_thread(&self) -> Ref<VmRef<Object>> {
        Ref::map(self.jvm_thread.borrow(), |opt| {
            opt.as_ref().expect("vmthread not initialised")
        })
    }

    pub fn global(&self) -> &JvmGlobalState {
        &self.jvm
    }

    pub fn jni_env(&self) -> *const JNIEnv {
        crate::jni::global_env()
    }

    fn set_vmthread(&self, vm_thread: VmRef<Object>) {
        assert_eq!(
            vm_thread.class().expect("null vmthread").name(),
            "java/lang/VMThread".as_mstr()
        );
        self.jvm_thread.replace(Some(vm_thread));
    }

    pub fn exec_helper(&self) -> ExecHelper {
        ExecHelper::new(self)
    }
}
