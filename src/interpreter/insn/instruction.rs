#![allow(unused_variables)]

use crate::alloc::vmref_alloc_object;
use crate::constant_pool::Entry;

use crate::interpreter::error::InterpreterError;
use crate::interpreter::frame::JavaFrame;
use crate::interpreter::insn::bytecode::Reader;
use crate::thread::JvmThreadState;

use crate::class::{Class, Object};
use crate::types::DataValue;

use crate::interpreter::interp::MethodArguments;
use cafebabe::MethodAccessFlags;
use std::fmt::Debug;

pub enum ExecuteResult {
    Continue,
    Return,
}

pub trait Instruction: Debug {
    fn name(&self) -> &'static str;

    fn execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError>;
}

macro_rules! insn_common {
    ($insn:ident, $str:expr) => {
        impl Instruction for $insn {
            fn name(&self) -> &'static str {
                $str
            }

            #[inline]
            fn execute(
                &self,
                frame: &mut JavaFrame,
                thread: &JvmThreadState,
            ) -> Result<ExecuteResult, InterpreterError> {
                self.do_execute(frame, thread)
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
            pub(crate) fn parse(_: &mut Reader) -> Result<Self, InterpreterError> {
                Ok(Self)
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
            pub(crate) fn parse(reader: &mut Reader) -> Result<Self, InterpreterError> {
                Ok(Self(reader.read_1()?))
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
            pub(crate) fn parse(reader: &mut Reader) -> Result<Self, InterpreterError> {
                let bytes = reader.read_n(2)?;
                let index = ((bytes[0] as u16) << 8) | bytes[1] as u16;
                Ok(Self(index))
            }
        }
    };
}

macro_rules! insn_delegate {
    ($delegate:expr) => {
        fn do_execute(
            &self,
            frame: &mut JavaFrame,
            thread: &JvmThreadState,
        ) -> Result<ExecuteResult, InterpreterError> {
            $delegate.do_execute(frame, thread)
        }
    };
}

// insn_n!(Aaload, "aaload");
// insn_n!(Aastore, "aastore");
// insn_n!(AconstNull, "aconst_null");
insn_1!(Aload, "aload");
insn_0!(Aload0, "aload_0");
insn_0!(Aload1, "aload_1");
insn_0!(Aload2, "aload_2");
insn_0!(Aload3, "aload_3");
// insn_n!(Anewarray, "anewarray");
// insn_n!(Areturn, "areturn");
insn_0!(Arraylength, "arraylength");
// insn_n!(Astore, "astore");
// insn_n!(Astore0, "astore_<n>");
// insn_n!(Astore1, "astore_<n>");
// insn_n!(Astore2, "astore_<n>");
// insn_n!(Astore3, "astore_<n>");
// insn_n!(Athrow, "athrow");
// insn_n!(Baload, "baload");
// insn_n!(Bastore, "bastore");
insn_1!(Bipush, "bipush");
// insn_n!(Caload, "caload");
// insn_n!(Castore, "castore");
// insn_n!(Checkcast, "checkcast");
// insn_n!(D2F, "d2f");
// insn_n!(D2I, "d2i");
// insn_n!(D2L, "d2l");
// insn_n!(Dadd, "dadd");
// insn_n!(Daload, "daload");
// insn_n!(Dastore, "dastore");
// insn_n!(Dcmpg, "dcmp<op>");
// insn_n!(Dcmpl, "dcmp<op>");
// insn_n!(Dconst0, "dconst_<d>");
// insn_n!(Dconst1, "dconst_<d>");
// insn_n!(Ddiv, "ddiv");
// insn_n!(Dload, "dload");
// insn_n!(Dload0, "dload_<n>");
// insn_n!(Dload1, "dload_<n>");
// insn_n!(Dload2, "dload_<n>");
// insn_n!(Dload3, "dload_<n>");
// insn_n!(Dmul, "dmul");
// insn_n!(Dneg, "dneg");
// insn_n!(Drem, "drem");
// insn_n!(Dreturn, "dreturn");
// insn_n!(Dstore, "dstore");
// insn_n!(Dstore0, "dstore_<n>");
// insn_n!(Dstore1, "dstore_<n>");
// insn_n!(Dstore2, "dstore_<n>");
// insn_n!(Dstore3, "dstore_<n>");
// insn_n!(Dsub, "dsub");
insn_0!(Dup, "dup");
// insn_n!(Dup2, "dup2");
// insn_n!(Dup2X1, "dup2_x1");
// insn_n!(Dup2X2, "dup2_x2");
// insn_n!(DupX1, "dup_x1");
// insn_n!(DupX2, "dup_x2");
// insn_n!(F2D, "f2d");
// insn_n!(F2I, "f2i");
// insn_n!(F2L, "f2l");
// insn_n!(Fadd, "fadd");
// insn_n!(Faload, "faload");
// insn_n!(Fastore, "fastore");
// insn_n!(Fcmpg, "fcmp<op>");
// insn_n!(Fcmpl, "fcmp<op>");
// insn_n!(Fconst0, "fconst_<f>");
// insn_n!(Fconst1, "fconst_<f>");
// insn_n!(Fconst2, "fconst_<f>");
// insn_n!(Fdiv, "fdiv");
// insn_n!(Fload, "fload");
// insn_n!(Fload0, "fload_<n>");
// insn_n!(Fload1, "fload_<n>");
// insn_n!(Fload2, "fload_<n>");
// insn_n!(Fload3, "fload_<n>");
// insn_n!(Fmul, "fmul");
// insn_n!(Fneg, "fneg");
// insn_n!(Frem, "frem");
// insn_n!(Freturn, "freturn");
// insn_n!(Fstore, "fstore");
// insn_n!(Fstore0, "fstore_<n>");
// insn_n!(Fstore1, "fstore_<n>");
// insn_n!(Fstore2, "fstore_<n>");
// insn_n!(Fstore3, "fstore_<n>");
// insn_n!(Fsub, "fsub");
// insn_n!(Getfield, "getfield");
// insn_n!(Getstatic, "getstatic");
// insn_n!(Goto, "goto");
// insn_n!(GotoW, "goto_w");
// insn_n!(I2B, "i2b");
// insn_n!(I2C, "i2c");
// insn_n!(I2D, "i2d");
// insn_n!(I2F, "i2f");
// insn_n!(I2L, "i2l");
// insn_n!(I2S, "i2s");
// insn_n!(Iadd, "iadd");
// insn_n!(Iaload, "iaload");
// insn_n!(Iand, "iand");
// insn_n!(Iastore, "iastore");
insn_0!(Iconst0, "iconst_0");
// insn_n!(Iconst1, "iconst_<i>");
// insn_n!(Iconst2, "iconst_<i>");
// insn_n!(Iconst3, "iconst_<i>");
// insn_n!(Iconst4, "iconst_<i>");
// insn_n!(Iconst5, "iconst_<i>");
// insn_n!(IconstM1, "iconst_<i>");
// insn_n!(Idiv, "idiv");
// insn_n!(IfAcmpeq, "if_acmp<cond>");
// insn_n!(IfAcmpne, "if_acmp<cond>");
// insn_n!(IfIcmpeq, "if_icmp<cond>");
// insn_n!(IfIcmpge, "if_icmp<cond>");
// insn_n!(IfIcmpgt, "if_icmp<cond>");
// insn_n!(IfIcmple, "if_icmp<cond>");
// insn_n!(IfIcmplt, "if_icmp<cond>");
// insn_n!(IfIcmpne, "if_icmp<cond>");
// insn_n!(Ifeq, "if<cond>");
// insn_n!(Ifge, "if<cond>");
// insn_n!(Ifgt, "if<cond>");
// insn_n!(Ifle, "if<cond>");
// insn_n!(Iflt, "if<cond>");
// insn_n!(Ifne, "if<cond>");
// insn_n!(Ifnonnull, "ifnonnull");
// insn_n!(Ifnull, "ifnull");
// insn_n!(Iinc, "iinc");
// insn_n!(Iload, "iload");
// insn_n!(Iload0, "iload_<n>");
// insn_n!(Iload1, "iload_<n>");
// insn_n!(Iload2, "iload_<n>");
// insn_n!(Iload3, "iload_<n>");
// insn_n!(Imul, "imul");
// insn_n!(Ineg, "ineg");
// insn_n!(Instanceof, "instanceof");
// insn_n!(Invokedynamic, "invokedynamic");
// insn_n!(Invokeinterface, "invokeinterface");
insn_2!(Invokespecial, "invokespecial");
insn_2!(Invokestatic, "invokestatic");
// insn_n!(Invokevirtual, "invokevirtual");
// insn_n!(Ior, "ior");
// insn_n!(Irem, "irem");
// insn_n!(Ireturn, "ireturn");
// insn_n!(Ishl, "ishl");
// insn_n!(Ishr, "ishr");
// insn_n!(Istore, "istore");
// insn_n!(Istore0, "istore_<n>");
// insn_n!(Istore1, "istore_<n>");
// insn_n!(Istore2, "istore_<n>");
// insn_n!(Istore3, "istore_<n>");
// insn_n!(Isub, "isub");
// insn_n!(Iushr, "iushr");
// insn_n!(Ixor, "ixor");
// insn_n!(Jsr, "jsr");
// insn_n!(JsrW, "jsr_w");
// insn_n!(L2D, "l2d");
// insn_n!(L2F, "l2f");
// insn_n!(L2I, "l2i");
// insn_n!(Ladd, "ladd");
// insn_n!(Laload, "laload");
// insn_n!(Land, "land");
// insn_n!(Lastore, "lastore");
// insn_n!(Lcmp, "lcmp");
// insn_n!(Lconst0, "lconst_<l>");
// insn_n!(Lconst1, "lconst_<l>");
insn_1!(Ldc, "ldc");
// insn_n!(Ldc2W, "ldc2_w");
// insn_n!(LdcW, "ldc_w");
// insn_n!(Ldiv, "ldiv");
// insn_n!(Lload, "lload");
// insn_n!(Lload0, "lload_<n>");
// insn_n!(Lload1, "lload_<n>");
// insn_n!(Lload2, "lload_<n>");
// insn_n!(Lload3, "lload_<n>");
// insn_n!(Lmul, "lmul");
// insn_n!(Lneg, "lneg");
// insn_n!(Lookupswitch, "lookupswitch");
// insn_n!(Lor, "lor");
// insn_n!(Lrem, "lrem");
// insn_n!(Lreturn, "lreturn");
// insn_n!(Lshl, "lshl");
// insn_n!(Lshr, "lshr");
// insn_n!(Lstore, "lstore");
// insn_n!(Lstore0, "lstore_<n>");
// insn_n!(Lstore1, "lstore_<n>");
// insn_n!(Lstore2, "lstore_<n>");
// insn_n!(Lstore3, "lstore_<n>");
// insn_n!(Lsub, "lsub");
// insn_n!(Lushr, "lushr");
// insn_n!(Lxor, "lxor");
// insn_n!(Monitorenter, "monitorenter");
// insn_n!(Monitorexit, "monitorexit");
// insn_n!(Multianewarray, "multianewarray");
insn_2!(New, "new");
// insn_n!(Newarray, "newarray");
// insn_n!(Nop, "nop");
// insn_n!(Pop, "pop");
// insn_n!(Pop2, "pop2");
// insn_n!(Putfield, "putfield");
insn_2!(Putstatic, "putstatic");
// insn_n!(Ret, "ret");
insn_0!(Return, "return");
// insn_n!(Saload, "saload");
// insn_n!(Sastore, "sastore");
// insn_n!(Sipush, "sipush");
// insn_n!(Swap, "swap");
// insn_n!(Tableswitch, "tableswitch");
// insn_n!(Wide, "wide");

// impl Aaload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Aaload")
//     }
// }

// impl Aastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Aastore")
//     }
// }

// impl AconstNull {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction AconstNull")
//     }
// }

impl Aload {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        let value = frame.local_vars.load_reference(self.0 as usize)?;
        frame.operand_stack.push(value);
        Ok(ExecuteResult::Continue)
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

// impl Anewarray {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Anewarray")
//     }
// }

// impl Areturn {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Areturn")
//     }
// }

impl Arraylength {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Arraylength")
    }
}

// impl Astore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Astore")
//     }
// }

// impl Astore0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Astore0")
//     }
// }

// impl Astore1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Astore1")
//     }
// }

// impl Astore2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Astore2")
//     }
// }

// impl Astore3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Astore3")
//     }
// }

// impl Athrow {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Athrow")
//     }
// }

// impl Baload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Baload")
//     }
// }

// impl Bastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Bastore")
//     }
// }

impl Bipush {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Bipush")
    }
}

// impl Caload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Caload")
//     }
// }

// impl Castore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Castore")
//     }
// }

// impl Checkcast {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Checkcast")
//     }
// }

// impl D2F {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction D2F")
//     }
// }

// impl D2I {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction D2I")
//     }
// }

// impl D2L {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction D2L")
//     }
// }

// impl Dadd {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dadd")
//     }
// }

// impl Daload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Daload")
//     }
// }

// impl Dastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dastore")
//     }
// }

// impl Dcmpg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dcmpg")
//     }
// }

// impl Dcmpl {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dcmpl")
//     }
// }

// impl Dconst0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dconst0")
//     }
// }

// impl Dconst1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dconst1")
//     }
// }

// impl Ddiv {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ddiv")
//     }
// }

// impl Dload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dload")
//     }
// }

// impl Dload0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dload0")
//     }
// }

// impl Dload1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dload1")
//     }
// }

// impl Dload2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dload2")
//     }
// }

// impl Dload3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dload3")
//     }
// }

// impl Dmul {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dmul")
//     }
// }

// impl Dneg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dneg")
//     }
// }

// impl Drem {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Drem")
//     }
// }

// impl Dreturn {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dreturn")
//     }
// }

// impl Dstore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dstore")
//     }
// }

// impl Dstore0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dstore0")
//     }
// }

// impl Dstore1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dstore1")
//     }
// }

// impl Dstore2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dstore2")
//     }
// }

// impl Dstore3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dstore3")
//     }
// }

// impl Dsub {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dsub")
//     }
// }

impl Dup {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Dup")
    }
}

// impl Dup2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dup2")
//     }
// }

// impl Dup2X1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dup2X1")
//     }
// }

// impl Dup2X2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Dup2X2")
//     }
// }

// impl DupX1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction DupX1")
//     }
// }

// impl DupX2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction DupX2")
//     }
// }

// impl F2D {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction F2D")
//     }
// }

// impl F2I {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction F2I")
//     }
// }

// impl F2L {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction F2L")
//     }
// }

// impl Fadd {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fadd")
//     }
// }

// impl Faload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Faload")
//     }
// }

// impl Fastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fastore")
//     }
// }

// impl Fcmpg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fcmpg")
//     }
// }

// impl Fcmpl {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fcmpl")
//     }
// }

// impl Fconst0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fconst0")
//     }
// }

// impl Fconst1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fconst1")
//     }
// }

// impl Fconst2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fconst2")
//     }
// }

// impl Fdiv {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fdiv")
//     }
// }

// impl Fload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fload")
//     }
// }

// impl Fload0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fload0")
//     }
// }

// impl Fload1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fload1")
//     }
// }

// impl Fload2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fload2")
//     }
// }

// impl Fload3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fload3")
//     }
// }

// impl Fmul {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fmul")
//     }
// }

// impl Fneg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fneg")
//     }
// }

// impl Frem {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Frem")
//     }
// }

// impl Freturn {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Freturn")
//     }
// }

// impl Fstore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fstore")
//     }
// }

// impl Fstore0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fstore0")
//     }
// }

// impl Fstore1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fstore1")
//     }
// }

// impl Fstore2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fstore2")
//     }
// }

// impl Fstore3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fstore3")
//     }
// }

// impl Fsub {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Fsub")
//     }
// }

// impl Getfield {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Getfield")
//     }
// }

// impl Getstatic {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Getstatic")
//     }
// }

// impl Goto {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Goto")
//     }
// }

// impl GotoW {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction GotoW")
//     }
// }

// impl I2B {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2B")
//     }
// }

// impl I2C {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2C")
//     }
// }

// impl I2D {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2D")
//     }
// }

// impl I2F {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2F")
//     }
// }

// impl I2L {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2L")
//     }
// }

// impl I2S {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction I2S")
//     }
// }

// impl Iadd {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iadd")
//     }
// }

// impl Iaload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iaload")
//     }
// }

// impl Iand {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iand")
//     }
// }

// impl Iastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iastore")
//     }
// }

impl Iconst0 {
    insn_delegate!(Bipush(0));
}

// impl Iconst1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iconst1")
//     }
// }

// impl Iconst2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iconst2")
//     }
// }

// impl Iconst3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iconst3")
//     }
// }

// impl Iconst4 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iconst4")
//     }
// }

// impl Iconst5 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iconst5")
//     }
// }

// impl IconstM1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IconstM1")
//     }
// }

// impl Idiv {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Idiv")
//     }
// }

// impl IfAcmpeq {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfAcmpeq")
//     }
// }

// impl IfAcmpne {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfAcmpne")
//     }
// }

// impl IfIcmpeq {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmpeq")
//     }
// }

// impl IfIcmpge {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmpge")
//     }
// }

// impl IfIcmpgt {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmpgt")
//     }
// }

// impl IfIcmple {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmple")
//     }
// }

// impl IfIcmplt {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmplt")
//     }
// }

// impl IfIcmpne {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction IfIcmpne")
//     }
// }

// impl Ifeq {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifeq")
//     }
// }

// impl Ifge {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifge")
//     }
// }

// impl Ifgt {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifgt")
//     }
// }

// impl Ifle {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifle")
//     }
// }

// impl Iflt {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iflt")
//     }
// }

// impl Ifne {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifne")
//     }
// }

// impl Ifnonnull {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifnonnull")
//     }
// }

// impl Ifnull {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ifnull")
//     }
// }

// impl Iinc {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iinc")
//     }
// }

// impl Iload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iload")
//     }
// }

// impl Iload0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iload0")
//     }
// }

// impl Iload1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iload1")
//     }
// }

// impl Iload2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iload2")
//     }
// }

// impl Iload3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iload3")
//     }
// }

// impl Imul {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Imul")
//     }
// }

// impl Ineg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ineg")
//     }
// }

// impl Instanceof {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Instanceof")
//     }
// }

// impl Invokedynamic {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Invokedynamic")
//     }
// }

// impl Invokeinterface {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Invokeinterface")
//     }
// }

impl Invokespecial {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Invokespecial")
    }
}

impl Invokestatic {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        let entry = frame
            .class
            .constant_pool()
            .method_entry(self.0)
            .ok_or_else(|| InterpreterError::NotMethodRef(self.0))?;
        // TODO ensure class is not interface, method not abstract, not constructor

        // resolve class and method
        let class = thread
            .global()
            .class_loader()
            .load_class(&entry.class, frame.class.loader().clone())?;

        let method = Class::find_method_recursive(
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

        // TODO typecheck args at verification time
        let arg_count = method.args().len();
        thread.interpreter().execute_method_from_frame(
            class,
            method,
            MethodArguments::Frame(frame, arg_count),
        )?;

        todo!("invoek static returned")
    }
}

// impl Invokevirtual {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Invokevirtual")
//     }
// }

// impl Ior {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ior")
//     }
// }

// impl Irem {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Irem")
//     }
// }

// impl Ireturn {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ireturn")
//     }
// }

// impl Ishl {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ishl")
//     }
// }

// impl Ishr {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ishr")
//     }
// }

// impl Istore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Istore")
//     }
// }

// impl Istore0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Istore0")
//     }
// }

// impl Istore1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Istore1")
//     }
// }

// impl Istore2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Istore2")
//     }
// }

// impl Istore3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Istore3")
//     }
// }

// impl Isub {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Isub")
//     }
// }

// impl Iushr {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Iushr")
//     }
// }

// impl Ixor {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ixor")
//     }
// }

// impl Jsr {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Jsr")
//     }
// }

// impl JsrW {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction JsrW")
//     }
// }

// impl L2D {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction L2D")
//     }
// }

// impl L2F {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction L2F")
//     }
// }

// impl L2I {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction L2I")
//     }
// }

// impl Ladd {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ladd")
//     }
// }

// impl Laload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Laload")
//     }
// }

// impl Land {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Land")
//     }
// }

// impl Lastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lastore")
//     }
// }

// impl Lcmp {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lcmp")
//     }
// }

// impl Lconst0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lconst0")
//     }
// }

// impl Lconst1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lconst1")
//     }
// }

impl Ldc {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        let pool = frame.class.constant_pool();
        let entry = pool
            .loadable_entry(self.0 as u16)
            .and_then(|e| if e.is_long_or_double() { None } else { Some(e) })
            .ok_or_else(|| InterpreterError::NotLoadable(self.0 as u16))?;

        match entry {
            Entry::String(s) => {
                // TODO lookup natively interned string instance

                let string_class = frame.ensure_loaded("java/lang/String")?;

                // create string instance
                let string_instance = vmref_alloc_object(|| Object::new_string(s.as_mstr()))?;

                // TODO natively intern new string instance

                // push onto stack
                frame
                    .operand_stack
                    .push(DataValue::reference(string_instance));
            } // TODO int/float
            // TODO class symbolic reference
            e => unimplemented!("loadable entry {:?}", e),
        }

        Ok(ExecuteResult::Continue)
    }
}

// impl Ldc2W {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ldc2W")
//     }
// }

// impl LdcW {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction LdcW")
//     }
// }

// impl Ldiv {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ldiv")
//     }
// }

// impl Lload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lload")
//     }
// }

// impl Lload0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lload0")
//     }
// }

// impl Lload1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lload1")
//     }
// }

// impl Lload2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lload2")
//     }
// }

// impl Lload3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lload3")
//     }
// }

// impl Lmul {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lmul")
//     }
// }

// impl Lneg {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lneg")
//     }
// }

// impl Lookupswitch {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lookupswitch")
//     }
// }

// impl Lor {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lor")
//     }
// }

// impl Lrem {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lrem")
//     }
// }

// impl Lreturn {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lreturn")
//     }
// }

// impl Lshl {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lshl")
//     }
// }

// impl Lshr {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lshr")
//     }
// }

// impl Lstore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lstore")
//     }
// }

// impl Lstore0 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lstore0")
//     }
// }

// impl Lstore1 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lstore1")
//     }
// }

// impl Lstore2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lstore2")
//     }
// }

// impl Lstore3 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lstore3")
//     }
// }

// impl Lsub {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lsub")
//     }
// }

// impl Lushr {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lushr")
//     }
// }

// impl Lxor {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Lxor")
//     }
// }

// impl Monitorenter {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Monitorenter")
//     }
// }

// impl Monitorexit {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Monitorexit")
//     }
// }

// impl Multianewarray {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Multianewarray")
//     }
// }

impl New {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction New")
    }
}

// impl Newarray {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Newarray")
//     }
// }

// impl Nop {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Nop")
//     }
// }

// impl Pop {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Pop")
//     }
// }

// impl Pop2 {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Pop2")
//     }
// }

// impl Putfield {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Putfield")
//     }
// }

impl Putstatic {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Putstatic")
    }
}

// impl Ret {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Ret")
//     }
// }

impl Return {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        todo!("instruction Return")
    }
}

// impl Saload {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Saload")
//     }
// }

// impl Sastore {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Sastore")
//     }
// }

// impl Sipush {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Sipush")
//     }
// }

// impl Swap {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Swap")
//     }
// }

// impl Tableswitch {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Tableswitch")
//     }
// }

// impl Wide {
//     fn do_execute(
//         &self,
//         frame: &mut JavaFrame,
//         thread: &JvmThreadState,
//     ) -> Result<ExecuteResult, InterpreterError> {
//         todo!("instruction Wide")
//     }
// }
