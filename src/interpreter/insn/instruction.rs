use crate::interpreter::error::InterpreterError;
use crate::interpreter::frame::JavaFrame;
use crate::interpreter::insn::bytecode::Reader;
use crate::thread::JvmThreadState;
use std::fmt::Debug;

type ExecuteResult = (); // TODO

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

// insn_n!(Aaload, "aaload");
// insn_n!(Aastore, "aastore");
// insn_n!(AconstNull, "aconst_null");
// insn_n!(Aload, "aload");
// insn_n!(Aload0, "aload_<n>");
// TODO n variations
// insn_n!(Anewarray, "anewarray");
// insn_n!(Areturn, "areturn");
// insn_n!(Arraylength, "arraylength");
// insn_n!(Astore, "astore");
// insn_n!(Astore0, "astore_<n>");
// insn_n!(Athrow, "athrow");
// insn_n!(Baload, "baload");
// insn_n!(Bastore, "bastore");
// insn_n!(Bipush, "bipush");
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
// insn_n!(Dconst0, "dconst_<d>");
// insn_n!(Ddiv, "ddiv");
// insn_n!(Dload, "dload");
// insn_n!(Dload0, "dload_<n>");
// insn_n!(Dmul, "dmul");
// insn_n!(Dneg, "dneg");
// insn_n!(Drem, "drem");
// insn_n!(Dreturn, "dreturn");
// insn_n!(Dstore, "dstore");
// insn_n!(Dstore0, "dstore_<n>");
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
// insn_n!(Fconst0, "fconst_<f>");
// insn_n!(Fdiv, "fdiv");
// insn_n!(Fload, "fload");
// insn_n!(Fload0, "fload_<n>");
// insn_n!(Fmul, "fmul");
// insn_n!(Fneg, "fneg");
// insn_n!(Frem, "frem");
// insn_n!(Freturn, "freturn");
// insn_n!(Fstore, "fstore");
// insn_n!(Fstore0, "fstore_<n>");
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
// insn_n!(IconstM1, "iconst_<i>");
// insn_n!(Idiv, "idiv");
// insn_n!(IfAcmpeq, "if_acmp<cond>");
// insn_n!(IfIcmpeq, "if_icmp<cond>");
// insn_n!(Ifeq, "if<cond>");
// insn_n!(Ifnonnull, "ifnonnull");
// insn_n!(Ifnull, "ifnull");
// insn_n!(Iinc, "iinc");
// insn_n!(Iload, "iload");
// insn_n!(Iload0, "iload_<n>");
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
insn_1!(Ldc, "ldc");
// insn_n!(Ldc2W, "ldc2_w");
// insn_n!(LdcW, "ldc_w");
// insn_n!(Ldiv, "ldiv");
// insn_n!(Lload, "lload");
// insn_n!(Lload0, "lload_<n>");
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

        unimplemented!()
    }
}

impl Invokespecial {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}

impl Invokestatic {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}

impl Putstatic {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}

impl New {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}

impl Dup {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}

impl Return {
    fn do_execute(
        &self,
        frame: &mut JavaFrame,
        thread: &JvmThreadState,
    ) -> Result<ExecuteResult, InterpreterError> {
        unimplemented!()
    }
}
