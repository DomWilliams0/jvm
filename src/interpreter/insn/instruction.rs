//! Hacky, repetitive and hopefully working in the success case only for now. Missing verification,
//! type compatibility checks (e.g. field ops), value set conversion, narrowing etc. FOR NOW!!

#![allow(unused_variables)]

use std::cmp::Ordering;
use std::fmt::Debug;

use log::*;
use num_enum::TryFromPrimitive;

use cafebabe::mutf8::StrExt;
use cafebabe::{AccessFlags, ClassAccessFlags, MethodAccessFlags};

use crate::alloc::{vmref_alloc_object, vmref_eq, VmRef};
use crate::class::{null, Class, ClassType, Object};
use crate::class::{FoundField, WhichLoader};
use crate::constant_pool::Entry;
use crate::error::{Throwable, Throwables};
use crate::interpreter::error::InterpreterError;
use crate::interpreter::insn::bytecode::InsnReader;
use crate::interpreter::insn::opcode::Opcode;
use crate::interpreter::insn::InstructionBlob;
use crate::interpreter::{Frame, InterpreterState};
use crate::thread;
use crate::types::{DataType, DataValue, NewarrayType, PrimitiveDataType};
use std::ops::{BitAnd, BitXor, Shr};

// TODO operand stack pop then verify might be wrong - only pop if its the right type?

pub enum PostExecuteAction {
    Continue,
    Return,
    ThrowException(Throwables),
    Exception(VmRef<Throwable>),
    MethodCall,
    /// Relative offset to the opcode of this instruction
    Jmp(i32),
    /// Absolute jump to pc
    JmpAbsolute(usize),
    /// Initialise the given class then rerun this instruction
    /// TODO might be possible to continue with resolved methods/fields state instead of replay
    ClassInit(VmRef<Class>),
}

pub type ExecuteResult = Result<PostExecuteAction, InterpreterError>;

macro_rules! insn_common {
    ($insn:ident, $str:expr) => {
        impl $insn {
            pub const OPCODE: u8 = Opcode::$insn as u8;
            pub const INSN: &'static str = $str;

            pub fn trampoline(
                insn: &InstructionBlob,
                interp: &mut InterpreterState,
            ) -> PostExecuteAction {
                let myself: &Self = unsafe { insn.transmute() };

                match myself.execute(interp) {
                    Ok(action) => action,
                    Err(err) => {
                        error!("interpreter error: {}", err);
                        // TODO better handling of interpreter error
                        PostExecuteAction::ThrowException(Throwables::Other(
                            "java/lang/InternalError",
                        ))
                    }
                }
            }
        }
    };
}

macro_rules! insn_0 {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn;

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(_: &mut InsnReader) -> Option<Self> {
                Some(Self)
            }
        }
    };
}

macro_rules! insn_1 {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub u8);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                reader.read_u8().map(Self)
            }
        }
    };
}

macro_rules! insn_2 {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub u16);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                reader.read_u16().map(Self)
            }
        }
    };
}

macro_rules! insn_2s {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub i16);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                reader.read_i16().map(Self)
            }
        }
    };
}

/// 2 separate u8s
macro_rules! insn_2x {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub u8, pub u8);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                reader.read_u8s().map(|(a, b)| Self(a, b))
            }
        }
    };
}

macro_rules! insn_4s {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub i32);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                reader.read_i32().map(Self)
            }
        }
    };
}

/// u16 and 2 separate u8s
macro_rules! insn_4x {
    ($insn:ident, $str:expr) => {
        #[derive(Debug)]
        pub struct $insn(pub u16, pub u8, pub u8);

        insn_common!($insn, $str);

        impl $insn {
            pub(crate) fn parse(reader: &mut InsnReader) -> Option<Self> {
                let idx = reader.read_u16()?;
                let (a, b) = reader.read_u8s()?;
                Some(Self(idx, a, b))
            }
        }
    };
}

macro_rules! insn_delegate {
    ($delegate:expr) => {
        fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
            $delegate.execute(interp)
        }
    };
}

// TODO some 2s are signed

insn_0!(Aaload, "aaload");
insn_0!(Aastore, "aastore");
insn_0!(AconstNull, "aconst_null");
insn_1!(Aload, "aload");
insn_0!(Aload0, "aload_0");
insn_0!(Aload1, "aload_1");
insn_0!(Aload2, "aload_2");
insn_0!(Aload3, "aload_3");
insn_2!(Anewarray, "anewarray");
insn_0!(Areturn, "areturn");
insn_0!(Arraylength, "arraylength");
insn_1!(Astore, "astore");
insn_0!(Astore0, "astore_0");
insn_0!(Astore1, "astore_1");
insn_0!(Astore2, "astore_2");
insn_0!(Astore3, "astore_3");
insn_0!(Athrow, "athrow");
insn_0!(Baload, "baload");
insn_0!(Bastore, "bastore");
insn_1!(Bipush, "bipush");
insn_0!(Caload, "caload");
insn_0!(Castore, "castore");
insn_2!(Checkcast, "checkcast");
insn_0!(D2F, "d2f");
insn_0!(D2I, "d2i");
insn_0!(D2L, "d2l");
insn_0!(Dadd, "dadd");
insn_0!(Daload, "daload");
insn_0!(Dastore, "dastore");
insn_0!(Dcmpg, "dcmpg");
insn_0!(Dcmpl, "dcmpl");
insn_0!(Dconst0, "dconst_0");
insn_0!(Dconst1, "dconst_1");
insn_0!(Ddiv, "ddiv");
insn_1!(Dload, "dload");
insn_0!(Dload0, "dload_0");
insn_0!(Dload1, "dload_1");
insn_0!(Dload2, "dload_2");
insn_0!(Dload3, "dload_3");
insn_0!(Dmul, "dmul");
insn_0!(Dneg, "dneg");
insn_0!(Drem, "drem");
insn_0!(Dreturn, "dreturn");
insn_1!(Dstore, "dstore");
insn_0!(Dstore0, "dstore_0");
insn_0!(Dstore1, "dstore_1");
insn_0!(Dstore2, "dstore_2");
insn_0!(Dstore3, "dstore_3");
insn_0!(Dsub, "dsub");
insn_0!(Dup, "dup");
insn_0!(Dup2, "dup2");
insn_0!(Dup2X1, "dup2_x1");
insn_0!(Dup2X2, "dup2_x2");
insn_0!(DupX1, "dup_x1");
insn_0!(DupX2, "dup_x2");
insn_0!(F2D, "f2d");
insn_0!(F2I, "f2i");
insn_0!(F2L, "f2l");
insn_0!(Fadd, "fadd");
insn_0!(Faload, "faload");
insn_0!(Fastore, "fastore");
insn_0!(Fcmpg, "fcmpg");
insn_0!(Fcmpl, "fcmpl");
insn_0!(Fconst0, "fconst_0");
insn_0!(Fconst1, "fconst_1");
insn_0!(Fconst2, "fconst_2");
insn_0!(Fdiv, "fdiv");
insn_1!(Fload, "fload");
insn_0!(Fload0, "fload_0");
insn_0!(Fload1, "fload_1");
insn_0!(Fload2, "fload_2");
insn_0!(Fload3, "fload_3");
insn_0!(Fmul, "fmul");
insn_0!(Fneg, "fneg");
insn_0!(Frem, "frem");
insn_0!(Freturn, "freturn");
insn_1!(Fstore, "fstore");
insn_0!(Fstore0, "fstore_0");
insn_0!(Fstore1, "fstore_1");
insn_0!(Fstore2, "fstore_2");
insn_0!(Fstore3, "fstore_3");
insn_0!(Fsub, "fsub");
insn_2!(Getfield, "getfield");
insn_2!(Getstatic, "getstatic");
insn_2s!(Goto, "goto");
insn_4s!(GotoW, "goto_w");
insn_0!(I2B, "i2b");
insn_0!(I2C, "i2c");
insn_0!(I2D, "i2d");
insn_0!(I2F, "i2f");
insn_0!(I2L, "i2l");
insn_0!(I2S, "i2s");
insn_0!(Iadd, "iadd");
insn_0!(Iaload, "iaload");
insn_0!(Iand, "iand");
insn_0!(Iastore, "iastore");
insn_0!(Iconst0, "iconst_0");
insn_0!(Iconst1, "iconst_1");
insn_0!(Iconst2, "iconst_2");
insn_0!(Iconst3, "iconst_3");
insn_0!(Iconst4, "iconst_4");
insn_0!(Iconst5, "iconst_5");
insn_0!(IconstM1, "iconst_m1");
insn_0!(Idiv, "idiv");
insn_2s!(IfAcmpeq, "if_acmpeq");
insn_2s!(IfAcmpne, "if_acmpne");
insn_2s!(IfIcmpeq, "if_icmpeq");
insn_2s!(IfIcmpge, "if_icmpge");
insn_2s!(IfIcmpgt, "if_icmpgt");
insn_2s!(IfIcmple, "if_icmple");
insn_2s!(IfIcmplt, "if_icmplt");
insn_2s!(IfIcmpne, "if_icmpne");
insn_2s!(Ifeq, "ifeq");
insn_2s!(Ifge, "ifge");
insn_2s!(Ifgt, "ifgt");
insn_2s!(Ifle, "ifle");
insn_2s!(Iflt, "iflt");
insn_2s!(Ifne, "ifne");
insn_2s!(Ifnonnull, "ifnonnull");
insn_2s!(Ifnull, "ifnull");
insn_2x!(Iinc, "iinc");
insn_1!(Iload, "iload");
insn_0!(Iload0, "iload_0");
insn_0!(Iload1, "iload_1");
insn_0!(Iload2, "iload_2");
insn_0!(Iload3, "iload_3");
insn_0!(Imul, "imul");
insn_0!(Ineg, "ineg");
insn_2!(Instanceof, "instanceof");
insn_4x!(Invokedynamic, "invokedynamic");
insn_4x!(Invokeinterface, "invokeinterface");
insn_2!(Invokespecial, "invokespecial");
insn_2!(Invokestatic, "invokestatic");
insn_2!(Invokevirtual, "invokevirtual");
insn_0!(Ior, "ior");
insn_0!(Irem, "irem");
insn_0!(Ireturn, "ireturn");
insn_0!(Ishl, "ishl");
insn_0!(Ishr, "ishr");
insn_1!(Istore, "istore");
insn_0!(Istore0, "istore_0");
insn_0!(Istore1, "istore_1");
insn_0!(Istore2, "istore_2");
insn_0!(Istore3, "istore_3");
insn_0!(Isub, "isub");
insn_0!(Iushr, "iushr");
insn_0!(Ixor, "ixor");
insn_2s!(Jsr, "jsr");
insn_4s!(JsrW, "jsr_w");
insn_0!(L2D, "l2d");
insn_0!(L2F, "l2f");
insn_0!(L2I, "l2i");
insn_0!(Ladd, "ladd");
insn_0!(Laload, "laload");
insn_0!(Land, "land");
insn_0!(Lastore, "lastore");
insn_0!(Lcmp, "lcmp");
insn_0!(Lconst0, "lconst_0");
insn_0!(Lconst1, "lconst_1");
insn_1!(Ldc, "ldc");
insn_2!(Ldc2W, "ldc2_w");
insn_2!(LdcW, "ldc_w");
insn_0!(Ldiv, "ldiv");
insn_1!(Lload, "lload");
insn_0!(Lload0, "lload_0");
insn_0!(Lload1, "lload_1");
insn_0!(Lload2, "lload_2");
insn_0!(Lload3, "lload_3");
insn_0!(Lmul, "lmul");
insn_0!(Lneg, "lneg");
// insn_n!(Lookupswitch, "lookupswitch");
insn_0!(Lor, "lor");
insn_0!(Lrem, "lrem");
insn_0!(Lreturn, "lreturn");
insn_0!(Lshl, "lshl");
insn_0!(Lshr, "lshr");
insn_1!(Lstore, "lstore");
insn_0!(Lstore0, "lstore_0");
insn_0!(Lstore1, "lstore_1");
insn_0!(Lstore2, "lstore_2");
insn_0!(Lstore3, "lstore_3");
insn_0!(Lsub, "lsub");
insn_0!(Lushr, "lushr");
insn_0!(Lxor, "lxor");
insn_0!(Monitorenter, "monitorenter");
insn_0!(Monitorexit, "monitorexit");
// insn_n!(Multianewarray, "multianewarray");
insn_2!(New, "new");
insn_1!(Newarray, "newarray");
insn_0!(Nop, "nop");
insn_0!(Pop, "pop");
insn_0!(Pop2, "pop2");
insn_2!(Putfield, "putfield");
insn_2!(Putstatic, "putstatic");
insn_1!(Ret, "ret");
insn_0!(Return, "return");
insn_0!(Saload, "saload");
insn_0!(Sastore, "sastore");
insn_2!(Sipush, "sipush");
insn_0!(Swap, "swap");
// insn_n!(Tableswitch, "tableswitch");
// insn_n!(Wide, "wide");

fn do_load_primitive(
    interp: &mut InterpreterState,
    idx: u8,
    f: impl FnOnce(&DataValue) -> bool,
    prim: PrimitiveDataType,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();
    let value = frame.local_vars.load(idx as usize).and_then(|v| {
        if f(&v) {
            Ok(v)
        } else {
            Err(InterpreterError::NotExpectedType {
                local_var: idx as usize,
                expected: DataType::Primitive(prim),
                actual: v.data_type(),
            })
        }
    })?;

    frame.operand_stack.push(value);
    Ok(PostExecuteAction::Continue)
}

impl Aaload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop reference type array and idx
        let (array, idx) =
            frame.pop_arrayref_and_idx(|cls| matches!(cls.class_type(), ClassType::Normal))?;

        let value = array.array_get_unchecked(idx);
        frame.operand_stack.push(value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Aastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop reference value first
        let value = frame.pop_reference_value()?;

        // pop reference type array and idx
        let (array, idx) =
            frame.pop_arrayref_and_idx(|cls| matches!(cls.class_type(), ClassType::Normal))?;

        // TODO assignment compatibility check
        array.array_set_unchecked(idx, value);
        Ok(PostExecuteAction::Continue)
    }
}

impl AconstNull {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        frame.operand_stack.push(DataValue::Reference(null()));
        Ok(PostExecuteAction::Continue)
    }
}

impl Aload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let value = frame.local_vars.load_reference(self.0 as usize)?;
        frame.operand_stack.push(value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Aload0 {
    insn_delegate!(Aload(0));
}

impl Aload1 {
    insn_delegate!(Aload(1));
}

impl Aload2 {
    insn_delegate!(Aload(2));
}

impl Aload3 {
    insn_delegate!(Aload(3));
}

impl Anewarray {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let thread = thread::get();
        let class_loader = thread.global().class_loader();

        // resolve element type
        let elem_type = frame
            .class
            .constant_pool()
            .class_entry(self.0)
            .ok_or_else(|| InterpreterError::NotClassRef(self.0))?;

        let elem_class = class_loader.load_class_caused_by(
            &elem_type.name,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        // pop length
        let length = frame.pop_int()?;
        if length < 0 {
            return Ok(PostExecuteAction::ThrowException(Throwables::Other(
                "java/lang/NegativeArraySizeException",
            )));
        }

        // resolve array class
        let array_cls =
            class_loader.load_reference_array_class(elem_class, frame.class.loader().clone())?;

        // allocate array
        let array_instance =
            vmref_alloc_object(|| Ok(Object::new_array(array_cls, length as usize)))?;

        // push to stack
        frame
            .operand_stack
            .push(DataValue::Reference(array_instance));

        Ok(PostExecuteAction::Continue)
    }
}

fn do_return_value(interp: &mut InterpreterState, val: DataValue) -> ExecuteResult {
    interp.return_value_to_caller(Some(val))?;
    Ok(PostExecuteAction::Return)
}

fn do_return_void(interp: &mut InterpreterState) -> ExecuteResult {
    interp.return_value_to_caller(None)?;
    Ok(PostExecuteAction::Return)
}

impl Areturn {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop reference operand
        let obj = frame.pop_reference_value()?;

        do_return_value(interp, obj)
    }
}

impl Arraylength {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop non-null array reference
        let obj = frame.pop_reference()?;

        if obj.is_null() {
            return Ok(PostExecuteAction::ThrowException(
                Throwables::NullPointerException,
            ));
        }

        // get length
        let length = obj.array_length().ok_or_else(|| {
            let class = obj.class().unwrap();
            InterpreterError::NotAnArray(class.class_type().to_owned())
        })?;

        trace!("array length is {}", length);

        // push onto stack
        frame.operand_stack.push(DataValue::Int(length));
        Ok(PostExecuteAction::Continue)
    }
}

impl Astore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let obj = frame.pop_value()?;

        if !obj.is_reference_or_retaddr() {
            return Err(InterpreterError::InvalidOperandForAstore(obj.data_type()));
        }

        frame.local_vars.store(self.0 as usize, obj)?;
        Ok(PostExecuteAction::Continue)
    }
}

impl Astore0 {
    insn_delegate!(Astore(0));
}

impl Astore1 {
    insn_delegate!(Astore(1));
}

impl Astore2 {
    insn_delegate!(Astore(2));
}

impl Astore3 {
    insn_delegate!(Astore(3));
}

impl Athrow {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let exc = frame.pop_reference()?;
        debug!("throw {:?}", exc.print_fields());

        todo!("instruction Athrow")
    }
}

impl Baload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Baload")
    }
}

impl Bastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Bastore")
    }
}

impl Bipush {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let val = DataValue::Int(self.0 as i8 as i32);
        // TODO sign extended?

        interp.current_frame_mut().operand_stack.push(val);
        Ok(PostExecuteAction::Continue)
    }
}

impl Caload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop reference type array and idx
        let (array, idx) = frame.pop_arrayref_and_idx(|cls| {
            matches!(
                cls.class_type(),
                ClassType::Primitive(PrimitiveDataType::Char)
            )
        })?;

        let value = array.array_get_unchecked(idx);
        frame.operand_stack.push(value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Castore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop value
        let value = frame.pop_value()?;
        debug_assert!(value.is_int());

        // pop reference type array and idx
        let (array, idx) = frame.pop_arrayref_and_idx(|cls| {
            matches!(
                cls.class_type(),
                ClassType::Primitive(PrimitiveDataType::Char)
            )
        })?;

        array.array_set_unchecked(idx, value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Checkcast {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let obj = frame.peek_value()?;
        match obj.into_reference() {
            Ok(obj) => {
                if let Some(cls_to_check) = obj.class() {
                    let class_ref = frame
                        .class
                        .constant_pool()
                        .class_entry(self.0)
                        .ok_or_else(|| InterpreterError::NotClassRef(self.0))?;

                    let cls = thread::get().global().class_loader().load_class_caused_by(
                        &class_ref.name,
                        frame.class.loader().clone(),
                        frame.class.name(),
                    )?;

                    trace!("checkcast {:?} is {:?}", cls_to_check.name(), cls.name());
                    if cls_to_check.is_instance_of(&cls) {
                        Ok(PostExecuteAction::Continue)
                    } else {
                        Ok(PostExecuteAction::ThrowException(Throwables::Other(
                            "java/lang/ClassCastException",
                        )))
                    }
                } else {
                    // nop if null
                    Ok(PostExecuteAction::Continue)
                }
            }
            Err(ty) => Err(InterpreterError::InvalidOperandForObjectOp(ty)),
        }
    }
}

impl D2F {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction D2F")
    }
}

impl D2I {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction D2I")
    }
}

impl D2L {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction D2L")
    }
}

impl Dadd {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dadd")
    }
}

impl Daload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Daload")
    }
}

impl Dastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dastore")
    }
}

impl Dcmpg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dcmpg")
    }
}

impl Dcmpl {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dcmpl")
    }
}

impl Dconst0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dconst0")
    }
}

impl Dconst1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dconst1")
    }
}

impl Ddiv {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Ddiv")
    }
}

impl Dload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        do_load_primitive(interp, self.0, |v| v.is_double(), PrimitiveDataType::Double)
    }
}

impl Dload0 {
    insn_delegate!(Dload(0));
}

impl Dload1 {
    insn_delegate!(Dload(1));
}

impl Dload2 {
    insn_delegate!(Dload(2));
}

impl Dload3 {
    insn_delegate!(Dload(3));
}

impl Dmul {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dmul")
    }
}

impl Dneg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dneg")
    }
}

impl Drem {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Drem")
    }
}

impl Dreturn {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let val = frame.pop_value()?.as_double().expect("not double");
        // TODO value set conversion
        do_return_value(interp, DataValue::Double(val))
    }
}

impl Dstore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dstore")
    }
}

impl Dstore0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dstore0")
    }
}

impl Dstore1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dstore1")
    }
}

impl Dstore2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dstore2")
    }
}

impl Dstore3 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dstore3")
    }
}

impl Dsub {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dsub")
    }
}

impl Dup {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // peek top operand
        let obj = frame
            .operand_stack
            .peek()
            .ok_or(InterpreterError::NoOperand)?;

        // push clone
        let obj_clone = obj.clone();
        frame.operand_stack.push(obj_clone);

        Ok(PostExecuteAction::Continue)
    }
}

impl Dup2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dup2")
    }
}

impl Dup2X1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dup2X1")
    }
}

impl Dup2X2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Dup2X2")
    }
}

impl DupX1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let val = frame.peek_value()?;
        frame.operand_stack.insert_at(val, 2);
        Ok(PostExecuteAction::Continue)
    }
}

impl DupX2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction DupX2")
    }
}

impl F2D {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction F2D")
    }
}

impl F2I {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop float
        let float = frame.pop_float()?;

        // TODO narrow float to int properly
        let int = float as i32;

        frame.operand_stack.push(DataValue::Int(int));
        Ok(PostExecuteAction::Continue)
    }
}

impl F2L {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction F2L")
    }
}

impl Fadd {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_two_op(interp, "+", |a, b| a + b)
    }
}

impl Faload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Faload")
    }
}

impl Fastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fastore")
    }
}

fn float_cmp(interp: &mut InterpreterState, op: &'static str, nan_fallback: i32) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    // pop values
    let (val1, val2) = frame.pop_2_floats()?;

    // do comparison
    let result = val1
        .partial_cmp(&val2)
        .map(|cmp| match cmp {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        })
        .unwrap_or(nan_fallback);

    trace!(
        "cmp {a} {op} {b} => {}",
        result,
        a = val1,
        op = op,
        b = val2
    );

    frame.operand_stack.push(DataValue::Int(result));

    Ok(PostExecuteAction::Continue)
}

impl Fcmpg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_cmp(interp, "fcmpg", 1)
    }
}

impl Fcmpl {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_cmp(interp, "fcmpg", -1)
    }
}

impl Fconst0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        interp
            .current_frame_mut()
            .operand_stack
            .push(DataValue::Float(0.0));
        Ok(PostExecuteAction::Continue)
    }
}

impl Fconst1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        interp
            .current_frame_mut()
            .operand_stack
            .push(DataValue::Float(1.0));
        Ok(PostExecuteAction::Continue)
    }
}

impl Fconst2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        interp
            .current_frame_mut()
            .operand_stack
            .push(DataValue::Float(2.0));
        Ok(PostExecuteAction::Continue)
    }
}

impl Fdiv {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_two_op(interp, "/", |a, b| a / b)
    }
}

impl Fload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        do_load_primitive(interp, self.0, |v| v.is_float(), PrimitiveDataType::Float)
    }
}

impl Fload0 {
    insn_delegate!(Fload(0));
}

impl Fload1 {
    insn_delegate!(Fload(1));
}

impl Fload2 {
    insn_delegate!(Fload(2));
}

impl Fload3 {
    insn_delegate!(Fload(3));
}

fn float_two_op(
    interp: &mut InterpreterState,
    wat: &'static str,
    op: impl FnOnce(f32, f32) -> f32,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    let (val1, val2) = frame.pop_2_floats()?;
    let result = op(val1, val2);

    trace!(
        "{a} {op} {b} = {result}",
        a = val1,
        op = wat,
        b = val2,
        result = result
    );

    frame.operand_stack.push(DataValue::Float(result));
    Ok(PostExecuteAction::Continue)
}

impl Fmul {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_two_op(interp, "*", |a, b| a * b)
    }
}

impl Fneg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fneg")
    }
}

impl Frem {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        // TODO is probably wrong
        float_two_op(interp, "%", |a, b| a.rem_euclid(b))
    }
}

impl Freturn {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let val = frame.pop_value()?.as_float().expect("not float");
        // TODO value set conversion
        do_return_value(interp, DataValue::Float(val))
    }
}

impl Fstore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fstore")
    }
}

impl Fstore0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fstore0")
    }
}

impl Fstore1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fstore1")
    }
}

impl Fstore2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fstore2")
    }
}

impl Fstore3 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Fstore3")
    }
}

impl Fsub {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        float_two_op(interp, "-", |a, b| a - b)
    }
}

impl Getfield {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // resolve field
        let field = frame
            .class
            .constant_pool()
            .field_entry(self.0)
            .ok_or_else(|| InterpreterError::NotFieldRef(self.0))?;

        trace!("getfield {:?}", field);

        // pop operand
        let obj = frame
            .operand_stack
            .pop()
            .ok_or(InterpreterError::NoOperand)?;

        // ensure non-null non-array reference
        let obj = obj
            .as_reference()
            .ok_or_else(|| InterpreterError::InvalidOperandForObjectOp(obj.data_type()))?;

        let obj_class = match obj.class() {
            Some(cls) => cls,
            None => {
                return Ok(PostExecuteAction::ThrowException(
                    Throwables::NullPointerException,
                ))
            }
        };

        if obj_class.class_type().is_array() {
            return Err(InterpreterError::UnexpectedArray(
                obj_class.class_type().to_owned(),
            ));
        }

        // get field value
        let value = obj
            .find_instance_field(field.name.as_mstr(), &field.desc)
            .ok_or_else(|| InterpreterError::FieldNotFound {
                name: field.name.to_owned(),
                desc: field.desc.clone(),
            })?;

        // push onto operand stack
        frame.operand_stack.push(value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Getstatic {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // resolve field
        let field = frame
            .class
            .constant_pool()
            .field_entry(self.0)
            .ok_or_else(|| InterpreterError::NotFieldRef(self.0))?;

        trace!("getstatic {:?}", field);

        // resolve class
        let class = thread::get().global().class_loader().load_class_caused_by(
            &field.class,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        // get field id and owning class
        let found = class
            .find_static_field_recursive(&field.name, &field.desc)
            .ok_or_else(|| InterpreterError::FieldNotFound {
                name: field.name.clone(),
                desc: field.desc.clone(),
            })?;

        // class holding static field data is not necessarily the same
        let (storage_class, field_id) = match found {
            FoundField::InThisClass(id) => (class.clone(), id),
            FoundField::InOtherClass(id, cls) => (cls, id),
        };

        // initialise class on successful resolution
        if class.needs_init() {
            return Ok(PostExecuteAction::ClassInit(class));
        }

        // get field value from storage class
        let value = storage_class.static_fields().ensure_get(field_id);

        // push onto stack
        frame.operand_stack.push(value);
        Ok(PostExecuteAction::Continue)
    }
}

impl Goto {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        Ok(PostExecuteAction::Jmp(self.0 as i32))
    }
}

impl GotoW {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction GotoW")
    }
}

impl I2B {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction I2B")
    }
}

impl I2C {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop int
        let int = frame.pop_int()?;

        // truncate to char
        let char = int as u16;

        // extend back to int
        frame.operand_stack.push(DataValue::Int(char as i32));
        Ok(PostExecuteAction::Continue)
    }
}

impl I2D {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction I2D")
    }
}

impl I2F {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop int
        let int = frame.pop_int()?;

        // convert to float
        // TODO "converted to the float result using IEEE 754 round to nearest mode"
        let float = int as f32;

        frame.operand_stack.push(DataValue::Float(float));
        Ok(PostExecuteAction::Continue)
    }
}

impl I2L {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // pop int
        let int = frame.pop_int()?;

        // extend to long
        frame.operand_stack.push(DataValue::Long(int as i64));
        Ok(PostExecuteAction::Continue)
    }
}

impl I2S {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction I2S")
    }
}

impl Iadd {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_two_op(interp, "+", |a, b| a.wrapping_add(b))
    }
}

impl Iaload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Iaload")
    }
}

impl Iand {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_two_op(interp, "&", |a, b| a.bitand(b))
    }
}

impl Iastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Iastore")
    }
}

impl Iconst0 {
    insn_delegate!(Bipush(0));
}

impl Iconst1 {
    insn_delegate!(Bipush(1));
}

impl Iconst2 {
    insn_delegate!(Bipush(2));
}

impl Iconst3 {
    insn_delegate!(Bipush(3));
}

impl Iconst4 {
    insn_delegate!(Bipush(4));
}

impl Iconst5 {
    insn_delegate!(Bipush(5));
}

impl IconstM1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let val = DataValue::Int(-1);
        interp.current_frame_mut().operand_stack.push(val);
        Ok(PostExecuteAction::Continue)
    }
}

impl Idiv {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Idiv")
    }
}

impl IfAcmpeq {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        obj_cmp_two(interp, self.0, "==", |a, b| vmref_eq(a, b))
    }
}

impl IfAcmpne {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        obj_cmp_two(interp, self.0, "!=", |a, b| !vmref_eq(a, b))
    }
}

/// wat: ">= 0"
fn int_cmp_one(
    interp: &mut InterpreterState,
    offset: i16,
    wat: &'static str,
    cmp: impl FnOnce(i32) -> bool,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    // pop int
    let int = frame.pop_int()?;

    // do comparison
    let success = cmp(int);
    trace!("cmp {} {} => {}", int, wat, success);

    let action = if success {
        PostExecuteAction::Jmp(offset as i32)
    } else {
        PostExecuteAction::Continue
    };

    Ok(action)
}

/// cmp(value1, value2)
/// op: ">="
fn int_cmp_two(
    interp: &mut InterpreterState,
    offset: i16,
    op: &'static str,
    cmp: impl FnOnce(i32, i32) -> bool,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    // pop values
    let (val1, val2) = frame.pop_2_ints()?;

    // do comparison
    let success = cmp(val1, val2);
    trace!(
        "cmp {a} {op} {b} => {}",
        success,
        a = val1,
        op = op,
        b = val2
    );

    let action = if success {
        PostExecuteAction::Jmp(offset as i32)
    } else {
        PostExecuteAction::Continue
    };

    Ok(action)
}

/// wat: "!="
fn obj_cmp_two(
    interp: &mut InterpreterState,
    offset: i16,
    wat: &'static str,
    cmp: impl FnOnce(&VmRef<Object>, &VmRef<Object>) -> bool,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    // pop references
    let (a, b) = frame.pop_2_references()?;

    // do comparison
    let success = cmp(&a, &b);
    trace!("cmp reference {} reference => {}", wat, success);

    let action = if success {
        PostExecuteAction::Jmp(offset as i32)
    } else {
        PostExecuteAction::Continue
    };

    Ok(action)
}

/// wat: "!= null"
fn obj_cmp_one(
    interp: &mut InterpreterState,
    offset: i16,
    wat: &'static str,
    cmp: impl FnOnce(&VmRef<Object>) -> bool,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    // pop reference
    let obj = frame.pop_reference()?;

    // do comparison
    let success = cmp(&obj);
    trace!("cmp reference {} => {}", wat, success);

    let action = if success {
        PostExecuteAction::Jmp(offset as i32)
    } else {
        PostExecuteAction::Continue
    };

    Ok(action)
}

fn int_one_op(
    interp: &mut InterpreterState,
    wat: &'static str,
    op: impl FnOnce(i32) -> i32,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    let val = frame.pop_int()?;
    let result = op(val);

    trace!(
        "{op} {val} = {result}",
        op = wat,
        val = val,
        result = result
    );

    frame.operand_stack.push(DataValue::Int(result));
    Ok(PostExecuteAction::Continue)
}

fn int_two_op(
    interp: &mut InterpreterState,
    wat: &'static str,
    op: impl FnOnce(i32, i32) -> i32,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    let (val1, val2) = frame.pop_2_ints()?;
    let result = op(val1, val2);

    trace!(
        "{a} {op} {b} = {result}",
        a = val1,
        op = wat,
        b = val2,
        result = result
    );

    frame.operand_stack.push(DataValue::Int(result));
    Ok(PostExecuteAction::Continue)
}

fn long_two_op(
    interp: &mut InterpreterState,
    wat: &'static str,
    op: impl FnOnce(i64, i64) -> i64,
) -> ExecuteResult {
    let frame = interp.current_frame_mut();

    let (val1, val2) = frame.pop_2_longs()?;
    let result = op(val1, val2);

    trace!(
        "{a} {op} {b} = {result}",
        a = val1,
        op = wat,
        b = val2,
        result = result
    );

    frame.operand_stack.push(DataValue::Long(result));
    Ok(PostExecuteAction::Continue)
}

impl IfIcmpeq {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, "==", |a, b| a == b)
    }
}

impl IfIcmpge {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, ">=", |a, b| a >= b)
    }
}

impl IfIcmpgt {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, ">", |a, b| a > b)
    }
}

impl IfIcmple {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, "<=", |a, b| a <= b)
    }
}

impl IfIcmplt {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, "<", |a, b| a < b)
    }
}

impl IfIcmpne {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_two(interp, self.0, "!=", |a, b| a != b)
    }
}

impl Ifeq {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, "== 0", |i| i == 0)
    }
}

impl Ifge {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, ">= 0", |i| i >= 0)
    }
}

impl Ifgt {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, "> 0", |i| i > 0)
    }
}

impl Ifle {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, "<= 0", |i| i <= 0)
    }
}

impl Iflt {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, "< 0", |i| i < 0)
    }
}

impl Ifne {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_cmp_one(interp, self.0, "!= 0", |i| i != 0)
    }
}

impl Ifnonnull {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        obj_cmp_one(interp, self.0, "!= null", |o| !o.is_null())
    }
}

impl Ifnull {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        obj_cmp_one(interp, self.0, "== null", |o| o.is_null())
    }
}

impl Iinc {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let Iinc(idx, constant) = self;

        let val = frame.local_vars.load(*idx as usize).and_then(|val| {
            val.as_int()
                .ok_or_else(|| InterpreterError::InvalidOperandForIntOp(val.data_type()))
        })?;

        let new_val = val + (*constant as i8) as i32;
        trace!("iinc {} to {}", val, new_val);

        frame
            .local_vars
            .store(*idx as usize, DataValue::Int(new_val))?;

        Ok(PostExecuteAction::Continue)
    }
}

impl Iload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        do_load_primitive(interp, self.0, |v| v.is_int(), PrimitiveDataType::Int)
    }
}

impl Iload0 {
    insn_delegate!(Iload(0));
}

impl Iload1 {
    insn_delegate!(Iload(1));
}

impl Iload2 {
    insn_delegate!(Iload(2));
}

impl Iload3 {
    insn_delegate!(Iload(3));
}

impl Imul {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_two_op(interp, "*", |a, b| a.wrapping_mul(b))
    }
}

impl Ineg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_one_op(interp, "-", |a| -a)
    }
}

impl Instanceof {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let obj = frame.pop_value()?;
        let result = match obj.into_reference().map(|obj| obj.class()) {
            Ok(None) => {
                // null
                0
            }
            Ok(Some(cls_to_check)) => {
                let class_ref = frame
                    .class
                    .constant_pool()
                    .class_entry(self.0)
                    .ok_or_else(|| InterpreterError::NotClassRef(self.0))?;

                let cls = thread::get().global().class_loader().load_class_caused_by(
                    &class_ref.name,
                    frame.class.loader().clone(),
                    frame.class.name(),
                )?;

                let result = cls_to_check.is_instance_of(&cls);
                trace!(
                    "{} instanceof {} => {}",
                    cls_to_check.name(),
                    cls.name(),
                    result
                );
                result as i32
            }
            Err(ty) => return Err(InterpreterError::InvalidOperandForObjectOp(ty)),
        };

        frame.operand_stack.push(DataValue::Int(result));
        Ok(PostExecuteAction::Continue)
    }
}

impl Invokedynamic {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Invokedynamic")
    }
}

// TODO invokeinterface throws a lot more exceptions
// TODO NoSuchMethod error

impl Invokeinterface {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let thread = thread::get();
        let class_loader = thread.global().class_loader();

        let entry = frame
            .class
            .constant_pool()
            .interface_entry(self.0)
            .ok_or_else(|| InterpreterError::NotInterfaceRef(self.0))?;

        // resolve class and method
        let class = class_loader.load_class_caused_by(
            &entry.class,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        let resolved_method = class
            .find_method_in_this_only(
                &entry.name,
                &entry.desc,
                MethodAccessFlags::empty(),
                MethodAccessFlags::empty(), // resolved method can be abstract
            )
            .ok_or_else(|| InterpreterError::MethodNotFound {
                class: entry.class.clone(),
                name: entry.name.clone(),
                desc: entry.desc.clone(),
            })?;

        // TODO ensure method is not static, IncompatibleClassChangeError
        assert!(!resolved_method.flags().is_static());
        // TODO verify this
        assert!(
            !resolved_method.is_instance_initializer() && !resolved_method.is_class_initializer()
        );

        // now select method (5.4.6)
        let selected_method = {
            if resolved_method.flags().contains(MethodAccessFlags::PRIVATE) {
                // chosen if private
                resolved_method
            } else {
                // get `this` object
                let this_obj = frame
                    .operand_stack
                    .peek_at(resolved_method.args().len())
                    .and_then(|val| val.as_reference())
                    .expect("invalid arg len?");

                if let Some(this_cls) = this_obj.class() {
                    let fml = resolved_method.name().to_utf8();
                    Class::find_overriding_method(this_cls, &resolved_method)
                        .unwrap_or(resolved_method)
                } else {
                    // if obj is null this will be caught when creating the frame
                    resolved_method
                }
            }
        };

        // TODO ensure not abstract
        assert!(!selected_method
            .flags()
            .contains(MethodAccessFlags::ABSTRACT));

        trace!("invokeinterface {}", selected_method);

        // pop args and call method
        let arg_count = selected_method.args().len() + 1; // +1 for this
        debug_assert_eq!(arg_count, self.1 as usize, "wrong redundant count");
        let callee_frame = Frame::new_with_caller(selected_method, frame, arg_count)?;
        interp.push_frame(callee_frame);

        Ok(PostExecuteAction::MethodCall)
    }
}

impl Invokespecial {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let thread = thread::get();
        let class_loader = thread.global().class_loader();

        let entry = frame
            .class
            .constant_pool()
            .method_or_interface_entry(self.0)
            .ok_or_else(|| InterpreterError::NotMethodRef(self.0))?;

        trace!("invokespecial entry {:?}", entry);

        let (class, method) = {
            // resolve specified class and method
            let resolved_class = class_loader.load_class_caused_by(
                &entry.class,
                frame.class.loader().clone(),
                frame.class.name(),
            )?;

            let resolved_method = Class::find_method_recursive_in_superclasses(
                &resolved_class,
                &entry.name,
                &entry.desc,
                MethodAccessFlags::empty(),
                MethodAccessFlags::ABSTRACT,
            )
            .ok_or_else(|| InterpreterError::MethodNotFound {
                class: entry.class.clone(),
                name: entry.name.clone(),
                desc: entry.desc.clone(),
            })?;

            // choose actual class
            let class = if
            // The resolved method is not an instance initialization method
            !resolved_method.is_instance_initializer() &&

                // If the symbolic reference names a class (not an interface), then that class is a superclass of the current class.
                (!resolved_class.is_interface() &&
                    frame.class.super_class().map(|sup| vmref_eq(sup, &resolved_class)).unwrap_or(false)) &&

                // The ACC_SUPER flag is set for the class file
                resolved_class.flags().contains(ClassAccessFlags::SUPER)
            {
                let super_class = frame.class.super_class().unwrap(); // checked to be Some
                super_class.clone()
            } else {
                resolved_class
            };

            // choose actual method
            let lookup_actual_method = || {
                if let Some(method) = class.find_method_in_this_only(
                    resolved_method.name(),
                    resolved_method.descriptor(),
                    MethodAccessFlags::empty(),
                    MethodAccessFlags::empty(),
                ) {
                    return method;
                }

                if !class.is_interface() {
                    if class.super_class().is_some() {
                        if let Some(method) = Class::find_method_recursive_in_superclasses(
                            &class,
                            resolved_method.name(),
                            resolved_method.descriptor(),
                            MethodAccessFlags::empty(),
                            MethodAccessFlags::empty(),
                        ) {
                            return method;
                        }
                    }
                } else {
                    let object_class = class_loader.get_bootstrap_class("java/lang/Object");
                    if let Some(method) = object_class.find_method_in_this_only(
                        resolved_method.name(),
                        resolved_method.descriptor(),
                        MethodAccessFlags::PUBLIC,
                        MethodAccessFlags::empty(),
                    ) {
                        return method;
                    }
                }

                if let Some(method) = class.find_maximally_specific_method(
                    resolved_method.name(),
                    resolved_method.descriptor(),
                    MethodAccessFlags::empty(),
                    MethodAccessFlags::ABSTRACT,
                ) {
                    return method;
                }

                // TODO return error here
                unreachable!("method not found")
            };
            let method = lookup_actual_method();
            trace!("invokespecial resolved method to {}", method);

            // TODO ensure method is not static, IncompatibleClassChangeError
            assert!(!method.flags().is_static());

            // TODO native method
            assert!(!method.flags().is_native(), "native not implemented");

            (class, method)
        };

        // pop args and call method
        let arg_count = method.args().len() + 1; // +1 for this
        let callee_frame = Frame::new_with_caller(method, frame, arg_count)?;
        interp.push_frame(callee_frame);

        Ok(PostExecuteAction::MethodCall)
    }
}

impl Invokestatic {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let entry = frame
            .class
            .constant_pool()
            .method_entry(self.0)
            .ok_or_else(|| InterpreterError::NotMethodRef(self.0))?;
        // TODO ensure class is not interface, method not abstract, not constructor

        // resolve class and method
        let class = thread::get().global().class_loader().load_class_caused_by(
            &entry.class,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        let method = Class::find_method_recursive_in_superclasses(
            &class,
            &entry.name,
            &entry.desc,
            MethodAccessFlags::STATIC,
            MethodAccessFlags::ABSTRACT,
        )
        .ok_or_else(|| InterpreterError::MethodNotFound {
            class: entry.class.clone(),
            name: entry.name.clone(),
            desc: entry.desc.clone(),
        })?;

        // On successful resolution of the method, the class or interface that declared the
        // resolved method is initialized if that class or interface has not already been
        // initialized (§5.5).
        if class.needs_init() {
            return Ok(PostExecuteAction::ClassInit(class));
        }

        // ensure native method is bound
        class.ensure_method_bound(&method)?;

        // TODO typecheck args at verification time
        let arg_count = method.args().len();
        let callee_frame = Frame::new_with_caller(method, frame, arg_count)?;
        interp.push_frame(callee_frame);

        Ok(PostExecuteAction::MethodCall)
    }
}

impl Invokevirtual {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let thread = thread::get();
        let class_loader = thread.global().class_loader();

        let entry = frame
            .class
            .constant_pool()
            .method_or_interface_entry(self.0)
            .ok_or_else(|| InterpreterError::NotMethodRef(self.0))?;

        // resolve class and method
        let class = class_loader.load_class_caused_by(
            &entry.class,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        let resolved_method = Class::find_method_recursive_in_superclasses(
            &class,
            &entry.name,
            &entry.desc,
            MethodAccessFlags::empty(),
            MethodAccessFlags::empty(), // resolved method can be abstract
        )
        .ok_or_else(|| InterpreterError::MethodNotFound {
            class: entry.class.clone(),
            name: entry.name.clone(),
            desc: entry.desc.clone(),
        })?;

        // should already be initialised if its been instantiated
        // debug_assert!(!class.needs_init());

        // TODO ensure method is not static, IncompatibleClassChangeError
        assert!(!resolved_method.flags().is_static());

        // now select method (5.4.6)
        let selected_method = {
            if resolved_method.flags().contains(MethodAccessFlags::PRIVATE) {
                // chosen if private
                resolved_method
            } else {
                // get `this` object
                let this_obj = frame
                    .operand_stack
                    .peek_at(resolved_method.args().len())
                    .and_then(|val| val.as_reference())
                    .expect("invalid arg len?");

                if let Some(this_cls) = this_obj.class() {
                    let fml = resolved_method.name().to_utf8();
                    Class::find_overriding_method(this_cls, &resolved_method)
                        .unwrap_or(resolved_method)
                } else {
                    // if obj is null this will be caught when creating the frame
                    resolved_method
                }
            }
        };

        trace!("invokevirtual {}", selected_method);

        // pop args and call method
        let arg_count = selected_method.args().len() + 1; // +1 for this
        let callee_frame = Frame::new_with_caller(selected_method, frame, arg_count)?;
        interp.push_frame(callee_frame);

        Ok(PostExecuteAction::MethodCall)
    }
}

impl Ior {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Ior")
    }
}

impl Irem {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let (val1, val2) = frame.pop_2_ints()?;

        let result = match val1.checked_div(val2) {
            Some(div) => val1 - div * val2,
            None => {
                return Ok(PostExecuteAction::ThrowException(Throwables::Other(
                    "java/lang/ArithmeticException",
                )))
            }
        };

        trace!("irem {} % {} => {}", val1, val2, result);

        frame.operand_stack.push(DataValue::Int(result));

        Ok(PostExecuteAction::Continue)
    }
}

impl Ireturn {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let val = frame.pop_int()?;
        // TODO may need to convert int to byte/short etc first
        do_return_value(interp, DataValue::Int(val))
    }
}

impl Ishl {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let (value, mut shift_by) = frame.pop_2_ints()?;
        shift_by &= 0x1f; // low 5 bits only

        let result = value.wrapping_shl(shift_by as u32);
        trace!("{} << {} => {}", value, shift_by, result);

        frame.operand_stack.push(DataValue::Int(result));
        Ok(PostExecuteAction::Continue)
    }
}

impl Ishr {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Ishr")
    }
}

impl Istore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let val = frame.pop_int()?;
        frame
            .local_vars
            .store(self.0 as usize, DataValue::Int(val))?;
        Ok(PostExecuteAction::Continue)
    }
}

impl Istore0 {
    insn_delegate!(Istore(0));
}

impl Istore1 {
    insn_delegate!(Istore(1));
}

impl Istore2 {
    insn_delegate!(Istore(2));
}

impl Istore3 {
    insn_delegate!(Istore(3));
}

impl Isub {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_two_op(interp, "-", |a, b| a.wrapping_sub(b))
    }
}

impl Iushr {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let (value, mut shift_by) = frame.pop_2_ints()?;
        shift_by &= 0x1f; // low 5 bits only

        // unsigned for logical shift
        let result = (value as u32).shr(shift_by as u32) as i32;
        trace!("{} << {} => {}", value, shift_by, result);

        frame.operand_stack.push(DataValue::Int(result));
        Ok(PostExecuteAction::Continue)
    }
}

impl Ixor {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        int_two_op(interp, "^", |a, b| a.bitxor(b))
    }
}

impl Jsr {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Jsr")
    }
}

impl JsrW {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction JsrW")
    }
}

impl L2D {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction L2D")
    }
}

impl L2F {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction L2F")
    }
}

impl L2I {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction L2I")
    }
}

impl Ladd {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        long_two_op(interp, "+", |a, b| a.wrapping_add(b))
    }
}

impl Laload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Laload")
    }
}

impl Land {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        long_two_op(interp, "&", |a, b| a.bitand(b))
    }
}

impl Lastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lastore")
    }
}

impl Lcmp {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lcmp")
    }
}

impl Lconst0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lconst0")
    }
}

impl Lconst1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lconst1")
    }
}

impl Ldc {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let pool = frame.class.constant_pool();
        let entry = pool
            .entry_and(self.0 as u16, |e| e.is_loadable())
            .ok_or_else(|| InterpreterError::NotLoadable(self.0 as u16))?;

        let to_push = match entry {
            Entry::String(s) => {
                // TODO lookup natively interned string instance

                let string_class = thread::get()
                    .global()
                    .class_loader()
                    .load_class("java/lang/String".as_mstr(), WhichLoader::Bootstrap)?;

                // ensure initialised
                if string_class.needs_init() {
                    return Ok(PostExecuteAction::ClassInit(string_class));
                }

                // create string instance
                let string_instance = vmref_alloc_object(|| Object::new_string(s.as_mstr()))?;

                // TODO natively intern new string instance
                DataValue::Reference(string_instance)
            }
            Entry::Float(f) => DataValue::from(*f),
            Entry::Int(i) => DataValue::from(*i),
            // TODO deny long and double
            Entry::ClassRef(cls_ref) => {
                // resolve class
                let cls = thread::get()
                    .global()
                    .class_loader()
                    .load_class(&cls_ref.name, frame.class.loader().clone())?;

                // get class object reference
                DataValue::Reference(cls.class_object().clone())
            }
            e => unimplemented!("loadable entry {:?}", e),
        };

        frame.operand_stack.push(to_push);

        Ok(PostExecuteAction::Continue)
    }
}

impl Ldc2W {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let pool = frame.class.constant_pool();
        let entry = pool
            .entry_and(self.0 as u16, |e| e.is_loadable_wide())
            .ok_or_else(|| InterpreterError::NotLoadable(self.0 as u16))?;

        let to_push = match entry {
            Entry::Long(l) => DataValue::from(*l),
            Entry::Double(d) => DataValue::from(*d),
            e => unimplemented!("wide loadable entry {:?}", e),
        };

        frame.operand_stack.push(to_push);
        Ok(PostExecuteAction::Continue)
    }
}

impl LdcW {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction LdcW")
    }
}

impl Ldiv {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Ldiv")
    }
}

impl Lload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lload")
    }
}

impl Lload0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lload0")
    }
}

impl Lload1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lload1")
    }
}

impl Lload2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lload2")
    }
}

impl Lload3 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lload3")
    }
}

impl Lmul {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lmul")
    }
}

impl Lneg {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lneg")
    }
}

// impl Lookupswitch {
//     fn execute(
//         &self,
//         interp: &mut InterpreterState
//     ) -> ExecuteResult {
//         todo!("instruction Lookupswitch")
//     }
// }

impl Lor {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lor")
    }
}

impl Lrem {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lrem")
    }
}

impl Lreturn {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lreturn")
    }
}

impl Lshl {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        let shift_by = frame.pop_int()? & 0x3f; // low 6 bits only
        let value = frame.pop_value()?.as_long().expect("not long");

        let result = value.wrapping_shl(shift_by as u32);
        trace!("{} << {} => {}", value, shift_by, result);

        frame.operand_stack.push(DataValue::Long(result));
        Ok(PostExecuteAction::Continue)
    }
}

impl Lshr {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lshr")
    }
}

impl Lstore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lstore")
    }
}

impl Lstore0 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lstore0")
    }
}

impl Lstore1 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lstore1")
    }
}

impl Lstore2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lstore2")
    }
}

impl Lstore3 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lstore3")
    }
}

impl Lsub {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lsub")
    }
}

impl Lushr {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lushr")
    }
}

impl Lxor {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Lxor")
    }
}

impl Monitorenter {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        // TODO monitorenter
        let obj = interp.current_frame_mut().pop_reference()?;
        trace!("monitorenter for {:?}", obj);
        Ok(PostExecuteAction::Continue)
    }
}

impl Monitorexit {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        // TODO monitorexit
        let obj = interp.current_frame_mut().pop_reference()?;
        trace!("monitorexit for {:?}", obj);
        Ok(PostExecuteAction::Continue)
    }
}

// impl Multianewarray {
//     fn execute(
//         &self,
//         interp: &mut InterpreterState
//     ) -> ExecuteResult {
//         todo!("instruction Multianewarray")
//     }
// }

impl New {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // find class name
        let classref = frame
            .class
            .constant_pool()
            .class_entry(self.0)
            .ok_or_else(|| InterpreterError::NotClassRef(self.0))?;

        // resolve and init class
        let class = thread::get().global().class_loader().load_class_caused_by(
            &classref.name,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        // TODO ensure not abstract, throw InstantiationError

        // initialise class on successful resolution
        if class.needs_init() {
            return Ok(PostExecuteAction::ClassInit(class));
        }

        // instantiate
        let obj = vmref_alloc_object(|| Ok(Object::new(class)))?;
        trace!(
            "instantiated new instance of {:?}: {:?}",
            obj.class().unwrap().name(),
            obj
        );

        // push onto stack
        frame.operand_stack.push(DataValue::Reference(obj));

        Ok(PostExecuteAction::Continue)
    }
}

impl Newarray {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // parse array type
        let elem_ty = NewarrayType::try_from_primitive(self.0)
            .map_err(|_| InterpreterError::InvalidArrayType(self.0))?;

        // get length
        let length = frame.pop_int()?;
        if length < 0 {
            return Ok(PostExecuteAction::ThrowException(Throwables::Other(
                "java/lang/NegativeArraySizeException",
            )));
        }

        // resolve array class
        let array_cls = thread::get()
            .global()
            .class_loader()
            .get_primitive_array(elem_ty.into());

        // allocate array
        let array_instance =
            vmref_alloc_object(|| Ok(Object::new_array(array_cls, length as usize)))?;

        // push to stack
        frame
            .operand_stack
            .push(DataValue::Reference(array_instance));

        Ok(PostExecuteAction::Continue)
    }
}

impl Nop {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Nop")
    }
}

impl Pop {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let popped = interp.current_frame_mut().pop_value()?;
        assert!(!popped.is_wide());
        Ok(PostExecuteAction::Continue)
    }
}

impl Pop2 {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();
        let popped = frame.pop_value()?;
        if !popped.is_wide() {
            let second = frame.pop_value()?;
            assert!(!second.is_wide());
        }
        Ok(PostExecuteAction::Continue)
    }
}

impl Putfield {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // resolve field
        let field = frame
            .class
            .constant_pool()
            .field_entry(self.0)
            .ok_or_else(|| InterpreterError::NotFieldRef(self.0))?;

        trace!("putfield {:?}", field);

        // pop objects
        let (value, object, class) = {
            let mut popped = frame
                .operand_stack
                .pop_n(2)
                .ok_or(InterpreterError::NoOperand)?;

            let value = popped.next().unwrap();
            let object = popped.next().unwrap();

            // ensure object is non-null non-array reference
            let object = object
                .into_reference()
                .map_err(InterpreterError::InvalidOperandForObjectOp)?;

            let class = if let Some(cls) = object.class() {
                cls
            } else {
                return Ok(PostExecuteAction::ThrowException(
                    Throwables::NullPointerException,
                ));
            };

            (value, object, class)
        };

        // TODO verify not array class
        let fields = object.fields().expect("unexpected array");

        let class: VmRef<Class> = class;

        // get field id
        // TODO throw IncompatibleClassChangeError
        let field_id = class
            .find_instance_field_recursive(field.name.as_mstr(), &field.desc)
            .ok_or_else(|| InterpreterError::FieldNotFound {
                name: field.name.clone(),
                desc: field.desc.clone(),
            })?;

        // TODO check value is compatible with field desc
        // TODO if final can only be in constructor

        // set field
        trace!(
            "putfield {:?}.{} = {:?}",
            object,
            field.name.as_mstr(),
            value
        );
        fields.ensure_set(field_id, value);

        Ok(PostExecuteAction::Continue)
    }
}

impl Putstatic {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let frame = interp.current_frame_mut();

        // resolve field
        let field = frame
            .class
            .constant_pool()
            .field_entry(self.0)
            .ok_or_else(|| InterpreterError::NotFieldRef(self.0))?;

        trace!("putstatic {:?}", field);

        // resolve class
        let class = thread::get().global().class_loader().load_class_caused_by(
            &field.class,
            frame.class.loader().clone(),
            frame.class.name(),
        )?;

        // get field id
        // TODO throw IncompatibleClassChangeError
        let found = class
            .find_static_field_recursive(&field.name, &field.desc)
            .ok_or_else(|| InterpreterError::FieldNotFound {
                name: field.name.clone(),
                desc: field.desc.clone(),
            })?;

        // class holding static field data is not necessarily the same
        let (storage_class, field_id) = match found {
            FoundField::InThisClass(id) => (class.clone(), id),
            FoundField::InOtherClass(id, cls) => (cls, id),
        };

        // initialise class on successful resolution
        if class.needs_init() {
            return Ok(PostExecuteAction::ClassInit(class));
        }

        // pop value
        let val = frame.pop_value()?;

        // TODO check value is compatible with field desc
        // TODO if final can only be in constructor
        // TODO if class is interface then can only be in constructor

        // set field in storage class
        storage_class.static_fields().ensure_set(field_id, val);

        Ok(PostExecuteAction::Continue)
    }
}

impl Ret {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Ret")
    }
}

impl Return {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        do_return_void(interp)
    }
}

impl Saload {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Saload")
    }
}

impl Sastore {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Sastore")
    }
}

impl Sipush {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        let val = self.0 as i16 as i32;
        interp
            .current_frame_mut()
            .operand_stack
            .push(DataValue::Int(val));
        Ok(PostExecuteAction::Continue)
    }
}

impl Swap {
    fn execute(&self, interp: &mut InterpreterState) -> ExecuteResult {
        todo!("instruction Swap")
    }
}

// impl Tableswitch {
//     fn execute(
//         &self,
//         interp: &mut InterpreterState
//     ) -> ExecuteResult {
//         todo!("instruction Tableswitch")
//     }
// }

// impl Wide {
//     fn execute(
//         &self,
//         interp: &mut InterpreterState
//     ) -> ExecuteResult {
//         todo!("instruction Wide")
//     }
// }
