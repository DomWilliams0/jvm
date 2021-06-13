use crate::alloc::{vmref_increment, vmref_ptr, VmRef};
use crate::class::{Class, ClassType, Method, MethodCode, NativeCode, NativeFunction, Object};
use crate::interpreter::error::InterpreterError;
use crate::types::DataValue;

use log::*;

use crate::error::Throwables;

use cafebabe::AccessFlags;

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

enum StackValue {
    Uninitialised,
    Initialised(DataValue),
}

pub struct LocalVariables(Vec<StackValue>);
#[derive(Debug)]
pub struct OperandStack(Vec<DataValue>);
pub struct FrameStack(Vec<(Frame, usize)>);

pub struct JavaFrame {
    // TODO is this always the same as method.class() ?
    pub class: VmRef<Class>,
    pub method: VmRef<Method>,
    pub local_vars: LocalVariables,
    pub operand_stack: OperandStack,
    pub code: Arc<[u8]>,
}

pub struct NativeFrame {
    pub inner: NativeFrameInner,
    pub function: NativeFunction,
    /// Some on init, None after exec
    pub args: Option<Box<[DataValue]>>,
}

pub struct JniFrame {
    pub function_name: Cow<'static, str>,
    local_refs: RefCell<Vec<VmRef<()>>>,
}

pub enum NativeFrameInner {
    Method {
        class: VmRef<Class>,
        method: VmRef<Method>,
    },
    Jni(JniFrame),
}

pub enum Frame {
    Java(JavaFrame),
    Native(NativeFrame),
}

pub enum FrameInfo<'a> {
    Method(&'a VmRef<Class>, &'a VmRef<Method>),
    Jni(&'a str),
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
            .ok_or(InterpreterError::InvalidLocalVar {
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

    /// idx is n from the last element e.g. 0 is the last
    pub fn peek_at(&self, idx: usize) -> Option<&DataValue> {
        let idx = self.0.len() - 1 - idx;
        self.0.get(idx)
    }

    /// idx is n from the end e.g. 1 is before the last element
    pub fn insert_at(&mut self, val: DataValue, idx: usize) {
        let idx = self.0.len() - idx;
        debug!(
            "inserting {:?} at index {:?}, stack length is now {:?}",
            val,
            idx,
            self.0.len() + 1
        );
        self.0.insert(idx, val);
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

    /// Top down, first is the current method
    pub fn iter(&self) -> impl Iterator<Item = &Frame> + '_ {
        self.0.iter().rev().map(|(frame, _)| frame)
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
        mut args: impl DoubleEndedIterator<Item = DataValue>,
    ) -> Result<Self, InterpreterError> {
        let class = method.class().to_owned();

        match method.code() {
            MethodCode::Abstract => {
                warn!("method {} is abstract", method);
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

                    if thisref.is_null() {
                        debug!("`this` is null");
                        return Err(InterpreterError::ExceptionRaised(
                            Throwables::NullPointerException,
                        ));
                    }

                    trace!("`this`: {:?}", thisref.print_fields());
                    local_vars.store(0, this)?;
                    1
                } else {
                    0
                };

                for (i, arg) in args.rev().enumerate() {
                    let var_offset = i + offset;
                    trace!("local var {} = {:?}", var_offset, arg);

                    // TODO long and double are wide
                    debug_assert!(!arg.is_wide(), "wide local var");
                    local_vars.store(var_offset, arg)?;
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
                        unreachable!("native method {} has not been bound", method,);
                    }
                    NativeCode::FailedToBind => {
                        warn!("native method {} could not be bound", method);
                        Err(InterpreterError::ExceptionRaised(Throwables::Other(
                            "java/lang/UnsatisfiedLinkError",
                        )))
                    }
                    NativeCode::Bound(function) => {
                        // code.ensure_compiled()
                        //     .expect("failed to compile trampoline");

                        let args = args.collect();
                        Ok(Frame::Native(NativeFrame {
                            inner: NativeFrameInner::Method {
                                class,
                                method: method.clone(),
                            },
                            function: *function,
                            args: Some(args),
                        }))
                    }
                }
            }
        }
    }

    pub fn new_no_args(method: VmRef<Method>) -> Result<Self, InterpreterError> {
        Self::new_with_args(method, std::iter::empty())
    }

    pub fn new_with_caller(
        method: VmRef<Method>,
        caller: &mut JavaFrame,
        nargs: usize,
    ) -> Result<Self, InterpreterError> {
        let stack_len = caller.operand_stack.count();
        let args = caller
            .operand_stack
            .pop_n(nargs)
            .ok_or(InterpreterError::NotEnoughArgs {
                expected: nargs,
                actual: stack_len,
            })?;

        Self::new_with_args(method, args)
    }

    /// None for direct JNI calls like JNI_OnLoad
    pub fn class_and_method(&self) -> FrameInfo {
        match self {
            Frame::Java(frame) => FrameInfo::Method(&frame.class, &frame.method),
            Frame::Native(NativeFrame {
                inner: NativeFrameInner::Method { class, method },
                ..
            }) => FrameInfo::Method(class, method),
            Frame::Native(NativeFrame {
                inner: NativeFrameInner::Jni(JniFrame { function_name, .. }),
                ..
            }) => FrameInfo::Jni(function_name),
        }
    }

    fn is_java(&self) -> bool {
        matches!(self, Frame::Java(_))
    }
}

impl<'a> FrameInfo<'a> {
    pub fn class(self) -> Option<&'a VmRef<Class>> {
        match self {
            FrameInfo::Method(cls, _) => Some(cls),
            _ => None,
        }
    }
}

impl JniFrame {
    pub fn new(jni_func: impl Into<Cow<'static, str>>) -> Self {
        Self {
            function_name: jni_func.into(),
            local_refs: RefCell::new(vec![]),
        }
    }

    pub fn add_local_ref<T: Debug>(&self, obj: &VmRef<T>) {
        // store a strong copy in the frame
        let local_ref = unsafe { std::mem::transmute::<VmRef<T>, VmRef<()>>(obj.clone()) };
        self.local_refs.borrow_mut().push(local_ref);
        debug!("bumped local ref count for {:?}", obj);
    }
}

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.class_and_method() {
            FrameInfo::Method(_, method) => {
                let suffix = if self.is_java() { "" } else { " (native)" };
                write!(f, "{}{}", method, suffix)
            }
            FrameInfo::Jni(name) => write!(f, "{} (jni)", name),
        }
    }
}

// TODO generic helper methods for popping up to 3 types from stack
//  e.g. pop::<i32, f32, i32>()

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
    pub fn pop_2_references(&mut self) -> Result<(VmRef<Object>, VmRef<Object>), InterpreterError> {
        let (val1, val2) = {
            let mut objs = self.pop_values(2)?;

            // popped in reverse order
            let val2 = objs.next().unwrap();
            let val1 = objs.next().unwrap();

            (val1, val2)
        };

        let val1 = val1
            .as_reference()
            .ok_or_else(|| InterpreterError::InvalidOperandForObjectOp(val1.data_type()))?;

        let val2 = val2
            .as_reference()
            .ok_or_else(|| InterpreterError::InvalidOperandForObjectOp(val2.data_type()))?;

        Ok((val1.clone(), val2.clone()))
    }

    /// "..., value1, value2 →"
    /// Returns (value1, value2)
    pub fn pop_2_longs(&mut self) -> Result<(i64, i64), InterpreterError> {
        let (val1, val2) = {
            let mut objs = self.pop_values(2)?;

            // popped in reverse order
            let val2 = objs.next().unwrap();
            let val1 = objs.next().unwrap();

            (val1, val2)
        };

        let val1 = val1
            .as_long()
            .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(val1.data_type()))?;

        let val2 = val2
            .as_long()
            .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(val2.data_type()))?;

        Ok((val1, val2))
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

    pub fn pop_arrayref_and_idx(
        &mut self,
        elem_check: impl FnOnce(&VmRef<Class>) -> bool,
    ) -> Result<(VmRef<Object>, usize), InterpreterError> {
        let idx = self.pop_int()?;
        let obj = self.pop_reference()?;

        let obj_cls = obj.class();

        let cls_type = match obj_cls.as_ref() {
            None => {
                return Err(InterpreterError::ExceptionRaised(
                    Throwables::NullPointerException,
                ))
            }
            Some(cls) => cls.class_type(),
        };

        match cls_type {
            ClassType::Array(ty) => {
                if !elem_check(ty) {
                    error!("array has the wrong element type ({})", ty.name());
                    return Err(InterpreterError::UnexpectedArrayType);
                }
            }
            ty => return Err(InterpreterError::NotAnArray(ty.to_owned())),
        };

        // bounds check
        if idx < 0 || (idx as usize) >= obj.array_unchecked().len() {
            trace!(
                "array index {:?} out of bounds (len={:?})",
                idx,
                obj.array_unchecked().len()
            );
            Err(InterpreterError::ExceptionRaised(Throwables::Other(
                "java/lang/ArrayIndexOutOfBoundsException",
            )))
        } else {
            Ok((obj, idx as usize))
        }
    }

    pub fn peek_value(&mut self) -> Result<DataValue, InterpreterError> {
        self.operand_stack
            .peek()
            .cloned()
            .ok_or(InterpreterError::NoOperand)
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

    #[test]
    fn operand_insert() {
        let mut stack = OperandStack::new(3);
        stack.push(DataValue::Int(1));
        stack.push(DataValue::Int(2));
        stack.push(DataValue::Int(3));

        stack.insert_at(DataValue::Int(10), 1);
        let ints = stack
            .pop_n(4)
            .unwrap()
            .map(|v| v.as_int().unwrap())
            .collect_vec();
        assert_eq!(ints, vec![3, 10, 2, 1]);
    }
}
