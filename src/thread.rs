use std::cell::{Cell, Ref, RefCell};
use std::iter::once;
use std::mem::MaybeUninit;
use std::sync::Arc;

use log::*;
use parking_lot::RwLock;

use crate::alloc::{vmref_alloc_object, VmRef};

use crate::class::{null, FieldSearchType, Object};
use crate::error::{Throwable, Throwables, VmResult};
use crate::interpreter::{Frame, Interpreter};
use crate::jni::sys::JNIEnv;
use crate::jvm::JvmGlobalState;
use crate::types::DataValue;
use crate::JvmError;
use cafebabe::mutf8::StrExt;
use cafebabe::MethodAccessFlags;
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

    // find root threadgroup
    let threadgroup_cls = classloader.get_bootstrap_class("java/lang/ThreadGroup");
    let root_threadgroup = threadgroup_cls.get_static_field("root", "Ljava/lang/ThreadGroup;");

    // create thread instance
    let thread_cls = classloader.get_bootstrap_class("java/lang/Thread");
    let thread_constructor = thread_cls
        .find_instance_constructor("(Ljava/lang/VMThread;Ljava/lang/String;IZ)V".as_mstr())
        .ok_or(Throwables::Other("java/lang/NoSuchMethodError"))?;
    let thread_instance = vmref_alloc_object(|| Ok(Object::new(thread_cls)))?;

    // create vmthread instance
    let vmthread_cls = classloader.get_bootstrap_class("java/lang/VMThread");
    let vmthread_instance = vmref_alloc_object(|| Ok(Object::new(vmthread_cls)))?;

    // invoke thread constructor
    {
        let interp = state.interpreter();
        let frame = Frame::new_with_args(
            thread_constructor,
            [
                DataValue::Reference(thread_instance.clone()),   // this
                DataValue::Reference(vmthread_instance.clone()), // vmthread
                DataValue::Reference(vmref_alloc_object(|| Object::new_string_utf8("main"))?), // name
                DataValue::Int(5),         // priority
                DataValue::Boolean(false), // daemon
            ]
            .into_iter()
            .rev(), // args are in reverse order
        )
        .unwrap();

        if let Err(exc) = interp.execute_frame(frame) {
            let exc_name = exc.class_name;
            state.set_exception(exc);
            error!("failed to create thread instance");
            return Err(Throwables::Other(exc_name)); // unsure this is fine
        }
    }

    // touch up vmthread fields
    let set_field = |obj: &Object, name: &'static str, value: DataValue| -> VmResult<()> {
        let name = name.to_mstr();
        let datatype = value.data_type();
        trace!("setting vmthread field {:?} to {:?}", name, value);
        let field_id = obj
            .find_field_in_this_only(name.as_ref(), &datatype, FieldSearchType::Instance)
            .ok_or(Throwables::Other("java/lang/NoSuchFieldError"))?;

        obj.fields().unwrap().ensure_set(field_id, value);
        Ok(())
    };
    set_field(
        &vmthread_instance,
        "thread",
        DataValue::Reference(thread_instance.clone()),
    )?;
    // set_field("vmdata", DataValue::Reference(todo!()))?; // TODO vmthread struct
    state.set_vmthread(vmthread_instance);

    // attach to thread group
    {
        let threadgroup_add = threadgroup_cls.find_callable_method(
            "addThread".as_mstr(),
            "(Ljava/lang/Thread;)V".as_mstr(),
            MethodAccessFlags::empty(),
        )?;
        let interp = state.interpreter();
        let frame = Frame::new_with_args(
            threadgroup_add,
            [
                root_threadgroup.clone(),                      // this
                DataValue::Reference(thread_instance.clone()), // thread
            ]
            .into_iter()
            .rev(), // args are in reverse order
        )
        .unwrap();

        if let Err(exc) = interp.execute_frame(frame) {
            let exc_name = exc.class_name;
            state.set_exception(exc);
            error!("failed to add thread to thread group");
            return Err(Throwables::Other(exc_name));
        }

        set_field(&thread_instance.clone(), "group", root_threadgroup)?;
    }

    // inheritable thread local is required apparently
    {
        let inheritablelocal_cls =
            classloader.get_bootstrap_class("java/lang/InheritableThreadLocal");
        let inheritable_local_newchildthread = inheritablelocal_cls.find_callable_method(
            "newChildThread".as_mstr(),
            "(Ljava/lang/Thread;)V".as_mstr(),
            MethodAccessFlags::STATIC,
        )?;
        let interp = state.interpreter();
        let frame = Frame::new_with_args(
            inheritable_local_newchildthread,
            once(DataValue::Reference(thread_instance)),
        )
        .unwrap();

        if let Err(exc) = interp.execute_frame(frame) {
            let exc_name = exc.class_name;
            state.set_exception(exc);
            error!("failed to add thread to InheritableThreadLocal");
            return Err(Throwables::Other(exc_name));
        }
    }

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
}
