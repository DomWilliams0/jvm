use crate::alloc::VmRef;
use crate::class::{Class, Method, Object};
use crate::interpreter::error::InterpreterError;
use crate::types::DataValue;

use log::*;

use crate::error::VmResult;
use crate::thread;
use cafebabe::mutf8::mstr;
use std::sync::Arc;

enum StackValue {
    Uninitialised,
    Initialised(DataValue),
}

pub struct LocalVariables(Vec<StackValue>);
pub struct OperandStack(Vec<DataValue>);
pub struct FrameStack(Vec<Frame>);

pub struct JavaFrame {
    pub class: VmRef<Class>,
    pub method: VmRef<Method>,
    pub local_vars: LocalVariables,
    pub operand_stack: OperandStack,
    pub code: Arc<[u8]>,
}

pub struct NativeFrame {
    pub class: VmRef<Class>,
    pub method: VmRef<Method>,
}

pub enum Frame {
    Java(JavaFrame),
    Native(NativeFrame),
}

impl LocalVariables {
    /// All uninitialised
    pub fn new_static(len: usize) -> Self {
        LocalVariables(StackValue::uninit(len))
    }

    /// Slot 0 initialised to `this`
    pub fn new_instance(len: usize, this: DataValue) -> Self {
        debug_assert!(len > 0);
        let mut vars = StackValue::uninit(len);
        vars[0] = StackValue::Initialised(this);
        LocalVariables(vars)
    }

    pub fn store(&mut self, idx: usize, value: DataValue) -> Result<(), InterpreterError> {
        todo!()
    }

    pub fn load(&mut self, idx: usize) -> Result<DataValue, InterpreterError> {
        self.0
            .get(idx)
            .ok_or_else(|| InterpreterError::InvalidLoad {
                requested: idx,
                max: self.0.len(),
            })
            .and_then(|val| match val {
                StackValue::Uninitialised => Err(InterpreterError::UninitialisedLoad(idx)),
                StackValue::Initialised(val) => Ok(val.clone()),
            })
    }

    pub fn load_reference(&mut self, idx: usize) -> Result<DataValue, InterpreterError> {
        self.load(idx).and_then(|val| match val {
            DataValue::Reference(_, _) => Ok(val),
            v => Err(InterpreterError::NotReference(idx, v)),
        })
    }

    // TODO validate local var slot in case of wide vars
}

impl OperandStack {
    pub fn new(len: usize) -> Self {
        OperandStack(Vec::with_capacity(len))
    }

    pub fn push(&mut self, value: DataValue) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<DataValue> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.iter().map(|v| if v.is_wide() { 2 } else { 1 }).sum()
    }
}
impl FrameStack {
    pub fn new() -> Self {
        FrameStack(Vec::with_capacity(64))
    }

    pub fn push(&mut self, frame: Frame) {
        self.0.push(frame);
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }
}

impl StackValue {
    fn uninit(n: usize) -> Vec<Self> {
        let mut vec = Vec::with_capacity(n);
        vec.resize_with(n, || StackValue::Uninitialised);
        vec
    }
}

// TODO tests for operand stack and local var array

impl Frame {
    // TODO instead of options, enum {Instance(obj), Static(class)}
    pub fn new_from_method(
        method: VmRef<Method>,
        class: VmRef<Class>,
        this: Option<VmRef<Object>>,
        args: impl Iterator<Item = DataValue>,
    ) -> Result<Self, InterpreterError> {
        if method.flags().is_native() {
            // TODO pass args to native function
            Ok(Frame::Native(NativeFrame { class, method }))
        } else {
            let code = method.code().ok_or_else(|| {
                warn!("method {:?}:{:?} has no code", class.name(), method.name());
                InterpreterError::NoCode
            })?;

            let mut local_vars = match this {
                Some(this) => LocalVariables::new_instance(
                    code.max_locals as usize,
                    DataValue::reference(this),
                ),
                None => LocalVariables::new_static(code.max_locals as usize),
            };

            for (i, arg) in args.enumerate() {
                local_vars.store(i + 1, arg)?;
            }

            Ok(Frame::Java(JavaFrame {
                class,
                method: method.clone(),
                local_vars,
                operand_stack: OperandStack::new(code.max_stack as usize),
                code: code.code.clone(),
            }))
        }
    }
}

impl JavaFrame {
    /// Ensure the given class is loaded, linked and initialised
    /// Name is utf8
    pub fn ensure_loaded(&self, class_name: impl AsRef<[u8]>) -> VmResult<VmRef<Class>> {
        let class_name = mstr::from_utf8(class_name.as_ref());
        let loader = self.class.loader().clone();
        thread::get()
            .global()
            .class_loader()
            .load_class(class_name.as_ref(), loader)
            .and_then(|c| {
                c.ensure_init()?;
                Ok(c)
            })
    }
}
