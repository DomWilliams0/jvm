use crate::alloc::VmRef;
use crate::class::{Class, FunctionArgs, Method, MethodCode, NativeCode, NativeFunction, Object};
use crate::interpreter::error::InterpreterError;
use crate::types::DataValue;

use log::*;

use crate::error::Throwables;
use cafebabe::AccessFlags;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

enum StackValue {
    Uninitialised,
    Initialised(DataValue),
}

pub struct LocalVariables(Vec<StackValue>);
pub struct OperandStack(Vec<DataValue>);
pub struct FrameStack(Vec<(Frame, usize)>);

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
    pub function: NativeFunction,
    pub args: Box<[DataValue]>,
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
        let count = self.0.len();
        self.0
            .get_mut(idx)
            .ok_or_else(|| InterpreterError::InvalidLocalVar {
                requested: idx,
                count,
            })
            .map(|val| *val = StackValue::Initialised(value))
    }

    pub fn load(&mut self, idx: usize) -> Result<DataValue, InterpreterError> {
        self.0
            .get(idx)
            .ok_or_else(|| InterpreterError::InvalidLocalVar {
                requested: idx,
                count: self.0.len(),
            })
            .and_then(|val| match val {
                StackValue::Uninitialised => Err(InterpreterError::UninitialisedLoad(idx)),
                StackValue::Initialised(val) => Ok(val.clone()),
            })
    }

    pub fn load_reference(&mut self, idx: usize) -> Result<DataValue, InterpreterError> {
        self.load(idx).and_then(|val| match val {
            DataValue::Reference(_) => Ok(val),
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
        // TODO longs and doubles take 2 slots!
        debug!(
            "pushing {:?} onto operand stack, count is now {:?}",
            value,
            self.count() + 1
        );
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<DataValue> {
        let val = self.0.pop();
        if let Some(val) = val.as_ref() {
            debug!(
                "popped {:?} from operand stack, count is now {:?}",
                val,
                self.count()
            );
        } else {
            error!("can't pop from empty operand stack");
        }
        val
    }

    pub fn pop_n(&mut self, n: usize) -> Option<impl DoubleEndedIterator<Item = DataValue> + '_> {
        if self.count() < n {
            None
        } else {
            let idx = self.0.len() - n;
            Some(self.0.drain(idx..).rev())
        }
    }

    pub fn depth(&self) -> usize {
        self.0.iter().map(|v| if v.is_wide() { 2 } else { 1 }).sum()
    }

    /// Available to be popped
    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn peek(&self) -> Option<&DataValue> {
        self.0.last()
    }
}
impl FrameStack {
    pub fn new() -> Self {
        FrameStack(Vec::with_capacity(64))
    }

    pub fn push(&mut self, frame: Frame, pc: usize) {
        self.0.push((frame, pc));
    }

    pub fn pop(&mut self) -> Option<(Frame, usize)> {
        self.0.pop()
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn top_native_mut(&mut self) -> Option<&mut NativeFrame> {
        self.0.last_mut().and_then(|(frame, _)| match frame {
            Frame::Native(frame) => Some(frame),
            Frame::Java(_) => None,
        })
    }

    pub fn top_java_mut(&mut self) -> Option<(&mut JavaFrame, &mut usize)> {
        self.0.last_mut().and_then(|(frame, pc)| match frame {
            Frame::Java(frame) => Some((frame, pc)),
            Frame::Native(_) => None,
        })
    }

    pub fn top_java(&self) -> Option<&JavaFrame> {
        self.0.last().and_then(|(frame, _)| match frame {
            Frame::Java(frame) => Some(frame),
            Frame::Native(_) => None,
        })
    }
    pub fn top(&self) -> Option<&Frame> {
        self.0.last().map(|(frame, _)| frame)
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
    pub fn new_with_args(
        method: VmRef<Method>,
        class: VmRef<Class>,
        mut args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> Result<Self, InterpreterError> {
        // TODO impl display on mstr
        match method.code() {
            MethodCode::Abstract => {
                warn!("method {:?}:{:?} is abstract", class.name(), method.name());
                Err(InterpreterError::ExceptionRaised(Throwables::Other(
                    "java/lang/AbstractMethodError",
                )))
            }
            MethodCode::Java(code) => {
                let mut local_vars = LocalVariables::new_static(code.max_locals as usize);

                // ensure `this` is not null
                let offset = if !method.flags().is_static() {
                    // TODO expects()
                    let this = args.next_back().expect("no this arg");
                    let thisref = this.as_reference().expect("this is not reference");
                    assert!(!thisref.is_null(), "this is null");

                    local_vars.store(0, this)?;
                    1
                } else {
                    0
                };

                for (i, arg) in args.rev().enumerate() {
                    local_vars.store(i + offset, arg)?;
                }

                Ok(Frame::Java(JavaFrame {
                    class,
                    method: method.clone(),
                    local_vars,
                    operand_stack: OperandStack::new(code.max_stack as usize),
                    code: code.code.clone(),
                }))
            }

            MethodCode::Native(native) => {
                let state = &*native.lock();
                match state {
                    NativeCode::Unbound => {
                        unreachable!(
                            "native method {:?}.{:?} has not been bound",
                            class.name(),
                            method.name()
                        );
                    }
                    NativeCode::FailedToBind => {
                        warn!(
                            "native method {:?}.{:?} could not be bound",
                            class.name(),
                            method.name()
                        );
                        Err(InterpreterError::ExceptionRaised(Throwables::Other(
                            "java/lang/UnsatisfiedLinkError",
                        )))
                    }
                    NativeCode::Bound(function) => {
                        // code.ensure_compiled()
                        //     .expect("failed to compile trampoline");

                        let args = args.collect();
                        Ok(Frame::Native(NativeFrame {
                            class,
                            method: method.clone(),
                            function: *function,
                            args,
                        }))
                    }
                }
            }
        }
    }

    pub fn new_no_args(
        method: VmRef<Method>,
        class: VmRef<Class>,
    ) -> Result<Self, InterpreterError> {
        Self::new_with_args(method, class, std::iter::empty())
    }

    pub fn new_with_caller(
        class: VmRef<Class>,
        method: VmRef<Method>,
        caller: &mut JavaFrame,
        nargs: usize,
    ) -> Result<Self, InterpreterError> {
        let stack_len = caller.operand_stack.count();
        let args =
            caller
                .operand_stack
                .pop_n(nargs)
                .ok_or_else(|| InterpreterError::NotEnoughArgs {
                    expected: nargs,
                    actual: stack_len,
                })?;

        Self::new_with_args(method, class, args)
    }

    fn class_and_method(&self) -> (&VmRef<Class>, &VmRef<Method>) {
        match self {
            Frame::Java(frame) => (&frame.class, &frame.method),
            Frame::Native(frame) => (&frame.class, &frame.method),
        }
    }

    fn is_java(&self) -> bool {
        matches!(self, Frame::Java(_))
    }
}

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (cls, method) = self.class_and_method();
        let suffix = if self.is_java() { "" } else { " (native)" };

        // TODO impl Display for mstr
        write!(
            f,
            "{}.{}{}",
            cls.name().to_utf8(),
            method.name().to_utf8(),
            suffix
        )
    }
}

impl JavaFrame {
    // TODO move these to extension trait on operandstack
    pub fn pop_value(&mut self) -> Result<DataValue, InterpreterError> {
        self.operand_stack.pop().ok_or(InterpreterError::NoOperand)
    }

    pub fn pop_values(
        &mut self,
        n: usize,
    ) -> Result<impl DoubleEndedIterator<Item = DataValue> + '_, InterpreterError> {
        self.operand_stack
            .pop_n(n)
            .ok_or(InterpreterError::NoOperand)
    }

    pub fn pop_int(&mut self) -> Result<i32, InterpreterError> {
        self.pop_value().and_then(|v| {
            v.as_int()
                .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(v.data_type()))
        })
    }

    pub fn pop_float(&mut self) -> Result<f32, InterpreterError> {
        self.pop_value().and_then(|v| {
            v.as_float()
                .ok_or_else(|| InterpreterError::InvalidOperandForFloatOp(v.data_type()))
        })
    }

    pub fn pop_reference_value(&mut self) -> Result<DataValue, InterpreterError> {
        self.pop_value().and_then(|v| {
            if v.is_reference() {
                Ok(v)
            } else {
                Err(InterpreterError::InvalidOperandForObjectOp(v.data_type()))
            }
        })
    }

    pub fn pop_reference(&mut self) -> Result<VmRef<Object>, InterpreterError> {
        self.pop_value().and_then(|v| {
            v.into_reference()
                .map_err(InterpreterError::InvalidOperandForObjectOp)
        })
    }

    /// "..., value1, value2 →"
    /// Returns (value1, value2)
    pub fn pop_2_ints(&mut self) -> Result<(i32, i32), InterpreterError> {
        let (val1, val2) = {
            let mut objs = self.pop_values(2)?;

            // popped in reverse order
            let val2 = objs.next().unwrap();
            let val1 = objs.next().unwrap();

            (val1, val2)
        };

        let val1 = val1
            .as_int()
            .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(val1.data_type()))?;

        let val2 = val2
            .as_int()
            .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(val2.data_type()))?;

        Ok((val1, val2))
    }

    /// "..., value1, value2 →"
    /// Returns (value1, value2)
    pub fn pop_2_floats(&mut self) -> Result<(f32, f32), InterpreterError> {
        let (val1, val2) = {
            let mut objs = self.pop_values(2)?;

            // popped in reverse order
            let val2 = objs.next().unwrap();
            let val1 = objs.next().unwrap();

            (val1, val2)
        };

        let val1 = val1
            .as_float()
            .ok_or_else(|| InterpreterError::InvalidOperandForFloatOp(val1.data_type()))?;

        let val2 = val2
            .as_float()
            .ok_or_else(|| InterpreterError::InvalidOperandForFloatOp(val2.data_type()))?;

        Ok((val1, val2))
    }
}

impl NativeFrame {
    pub fn invoke(&mut self) -> Option<DataValue> {
        let args = FunctionArgs::from(self.args.as_mut());
        match self.function {
            NativeFunction::Internal(func) => func(args),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::frame::OperandStack;
    use crate::types::DataValue;
    use itertools::Itertools;

    #[test]
    fn operand_pop() {
        let mut stack = OperandStack::new(6);

        stack.push(DataValue::Int(1));
        stack.push(DataValue::Int(2));
        stack.push(DataValue::Long(3));
        stack.push(DataValue::Int(4));

        assert_eq!(stack.count(), 4);
        assert_eq!(stack.depth(), 5);

        assert!(stack.pop_n(10).is_none());

        let popped = stack.pop_n(3).unwrap().collect_vec();
        assert_eq!(stack.count(), 1);
        assert_eq!(popped.len(), 3);

        let intvalue = |val: &DataValue| match val {
            DataValue::Int(i) => *i as i64,
            DataValue::Long(i) => *i,
            _ => unreachable!(),
        };
        let ints = popped.iter().map(intvalue).collect_vec();
        assert_eq!(ints, vec![4, 3, 2]);

        assert_eq!(intvalue(&stack.pop().unwrap()), 1);
        assert!(stack.pop().is_none());
    }
}
