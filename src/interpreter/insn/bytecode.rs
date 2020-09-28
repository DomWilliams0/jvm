use crate::interpreter::insn::instruction::*;
use crate::interpreter::insn::opcode::Opcode;

use log::*;
use num_enum::TryFromPrimitive;

pub(crate) struct InsnReader<'a> {
    bytes: &'a [u8],
    pc: usize,
}

#[derive(Default)]
pub struct InstructionBlob([u8; Self::MAX_INSN_SIZE]);

impl InstructionBlob {
    const MAX_INSN_SIZE: usize = 4;

    fn fill<I>(&mut self, insn: &I) {
        let insn_size = std::mem::size_of::<I>();
        debug_assert!(
            insn_size <= Self::MAX_INSN_SIZE,
            "raise max instruction size to {} for {}",
            insn_size,
            std::any::type_name::<I>()
        );

        let src = insn as *const I as *const u8;
        let dst = self.0.as_mut_ptr();

        unsafe {
            src.copy_to_nonoverlapping(dst, insn_size);
        }
    }

    pub unsafe fn transmute<I>(&self) -> &I {
        let insn_size = std::mem::size_of::<I>();
        debug_assert!(
            insn_size <= Self::MAX_INSN_SIZE,
            "raise max instruction size to {} for {}",
            insn_size,
            std::any::type_name::<I>()
        );

        &*(self.0.as_ref() as *const [u8] as *const I)
    }
}

//noinspection RsLiveness
pub fn get_insn(bytecode: &[u8], pc: usize, blob: &mut InstructionBlob) -> Option<(usize, Opcode)> {
    let insn = *bytecode.get(pc)?;
    let mut reader = InsnReader {
        bytes: bytecode,
        pc: pc + 1,
    };

    macro_rules! insn {
        ($insn:ident) => {{
            let insn = $insn::parse(&mut reader)?;
            blob.fill(&insn);
            Opcode::$insn
        }};
    }

    let opcode = match insn {
        Aaload::OPCODE => insn!(Aaload),
        Aastore::OPCODE => insn!(Aastore),
        AconstNull::OPCODE => insn!(AconstNull),
        Aload::OPCODE => insn!(Aload),
        Aload0::OPCODE => insn!(Aload0),
        Aload1::OPCODE => insn!(Aload1),
        Aload2::OPCODE => insn!(Aload2),
        Aload3::OPCODE => insn!(Aload3),
        Anewarray::OPCODE => insn!(Anewarray),
        Areturn::OPCODE => insn!(Areturn),
        Arraylength::OPCODE => insn!(Arraylength),
        Astore::OPCODE => insn!(Astore),
        Astore0::OPCODE => insn!(Astore0),
        Astore1::OPCODE => insn!(Astore1),
        Astore2::OPCODE => insn!(Astore2),
        Astore3::OPCODE => insn!(Astore3),
        Athrow::OPCODE => insn!(Athrow),
        Baload::OPCODE => insn!(Baload),
        Bastore::OPCODE => insn!(Bastore),
        Bipush::OPCODE => insn!(Bipush),
        Caload::OPCODE => insn!(Caload),
        Castore::OPCODE => insn!(Castore),
        Checkcast::OPCODE => insn!(Checkcast),
        D2F::OPCODE => insn!(D2F),
        D2I::OPCODE => insn!(D2I),
        D2L::OPCODE => insn!(D2L),
        Dadd::OPCODE => insn!(Dadd),
        Daload::OPCODE => insn!(Daload),
        Dastore::OPCODE => insn!(Dastore),
        Dcmpg::OPCODE => insn!(Dcmpg),
        Dcmpl::OPCODE => insn!(Dcmpl),
        Dconst0::OPCODE => insn!(Dconst0),
        Dconst1::OPCODE => insn!(Dconst1),
        Ddiv::OPCODE => insn!(Ddiv),
        Dload::OPCODE => insn!(Dload),
        Dload0::OPCODE => insn!(Dload0),
        Dload1::OPCODE => insn!(Dload1),
        Dload2::OPCODE => insn!(Dload2),
        Dload3::OPCODE => insn!(Dload3),
        Dmul::OPCODE => insn!(Dmul),
        Dneg::OPCODE => insn!(Dneg),
        Drem::OPCODE => insn!(Drem),
        Dreturn::OPCODE => insn!(Dreturn),
        Dstore::OPCODE => insn!(Dstore),
        Dstore0::OPCODE => insn!(Dstore0),
        Dstore1::OPCODE => insn!(Dstore1),
        Dstore2::OPCODE => insn!(Dstore2),
        Dstore3::OPCODE => insn!(Dstore3),
        Dsub::OPCODE => insn!(Dsub),
        Dup::OPCODE => insn!(Dup),
        Dup2::OPCODE => insn!(Dup2),
        Dup2X1::OPCODE => insn!(Dup2X1),
        Dup2X2::OPCODE => insn!(Dup2X2),
        DupX1::OPCODE => insn!(DupX1),
        DupX2::OPCODE => insn!(DupX2),
        F2D::OPCODE => insn!(F2D),
        F2I::OPCODE => insn!(F2I),
        F2L::OPCODE => insn!(F2L),
        Fadd::OPCODE => insn!(Fadd),
        Faload::OPCODE => insn!(Faload),
        Fastore::OPCODE => insn!(Fastore),
        Fcmpg::OPCODE => insn!(Fcmpg),
        Fcmpl::OPCODE => insn!(Fcmpl),
        Fconst0::OPCODE => insn!(Fconst0),
        Fconst1::OPCODE => insn!(Fconst1),
        Fconst2::OPCODE => insn!(Fconst2),
        Fdiv::OPCODE => insn!(Fdiv),
        Fload::OPCODE => insn!(Fload),
        Fload0::OPCODE => insn!(Fload0),
        Fload1::OPCODE => insn!(Fload1),
        Fload2::OPCODE => insn!(Fload2),
        Fload3::OPCODE => insn!(Fload3),
        Fmul::OPCODE => insn!(Fmul),
        Fneg::OPCODE => insn!(Fneg),
        Frem::OPCODE => insn!(Frem),
        Freturn::OPCODE => insn!(Freturn),
        Fstore::OPCODE => insn!(Fstore),
        Fstore0::OPCODE => insn!(Fstore0),
        Fstore1::OPCODE => insn!(Fstore1),
        Fstore2::OPCODE => insn!(Fstore2),
        Fstore3::OPCODE => insn!(Fstore3),
        Fsub::OPCODE => insn!(Fsub),
        Getfield::OPCODE => insn!(Getfield),
        Getstatic::OPCODE => insn!(Getstatic),
        Goto::OPCODE => insn!(Goto),
        GotoW::OPCODE => insn!(GotoW),
        I2B::OPCODE => insn!(I2B),
        I2C::OPCODE => insn!(I2C),
        I2D::OPCODE => insn!(I2D),
        I2F::OPCODE => insn!(I2F),
        I2L::OPCODE => insn!(I2L),
        I2S::OPCODE => insn!(I2S),
        Iadd::OPCODE => insn!(Iadd),
        Iaload::OPCODE => insn!(Iaload),
        Iand::OPCODE => insn!(Iand),
        Iastore::OPCODE => insn!(Iastore),
        Iconst0::OPCODE => insn!(Iconst0),
        Iconst1::OPCODE => insn!(Iconst1),
        Iconst2::OPCODE => insn!(Iconst2),
        Iconst3::OPCODE => insn!(Iconst3),
        Iconst4::OPCODE => insn!(Iconst4),
        Iconst5::OPCODE => insn!(Iconst5),
        IconstM1::OPCODE => insn!(IconstM1),
        Idiv::OPCODE => insn!(Idiv),
        IfAcmpeq::OPCODE => insn!(IfAcmpeq),
        IfAcmpne::OPCODE => insn!(IfAcmpne),
        IfIcmpeq::OPCODE => insn!(IfIcmpeq),
        IfIcmpge::OPCODE => insn!(IfIcmpge),
        IfIcmpgt::OPCODE => insn!(IfIcmpgt),
        IfIcmple::OPCODE => insn!(IfIcmple),
        IfIcmplt::OPCODE => insn!(IfIcmplt),
        IfIcmpne::OPCODE => insn!(IfIcmpne),
        Ifeq::OPCODE => insn!(Ifeq),
        Ifge::OPCODE => insn!(Ifge),
        Ifgt::OPCODE => insn!(Ifgt),
        Ifle::OPCODE => insn!(Ifle),
        Iflt::OPCODE => insn!(Iflt),
        Ifne::OPCODE => insn!(Ifne),
        Ifnonnull::OPCODE => insn!(Ifnonnull),
        Ifnull::OPCODE => insn!(Ifnull),
        Iinc::OPCODE => insn!(Iinc),
        Iload::OPCODE => insn!(Iload),
        Iload0::OPCODE => insn!(Iload0),
        Iload1::OPCODE => insn!(Iload1),
        Iload2::OPCODE => insn!(Iload2),
        Iload3::OPCODE => insn!(Iload3),
        Imul::OPCODE => insn!(Imul),
        Ineg::OPCODE => insn!(Ineg),
        Instanceof::OPCODE => insn!(Instanceof),
        Invokedynamic::OPCODE => insn!(Invokedynamic),
        Invokeinterface::OPCODE => insn!(Invokeinterface),
        Invokespecial::OPCODE => insn!(Invokespecial),
        Invokestatic::OPCODE => insn!(Invokestatic),
        Invokevirtual::OPCODE => insn!(Invokevirtual),
        Ior::OPCODE => insn!(Ior),
        Irem::OPCODE => insn!(Irem),
        Ireturn::OPCODE => insn!(Ireturn),
        Ishl::OPCODE => insn!(Ishl),
        Ishr::OPCODE => insn!(Ishr),
        Istore::OPCODE => insn!(Istore),
        Istore0::OPCODE => insn!(Istore0),
        Istore1::OPCODE => insn!(Istore1),
        Istore2::OPCODE => insn!(Istore2),
        Istore3::OPCODE => insn!(Istore3),
        Isub::OPCODE => insn!(Isub),
        Iushr::OPCODE => insn!(Iushr),
        Ixor::OPCODE => insn!(Ixor),
        Jsr::OPCODE => insn!(Jsr),
        JsrW::OPCODE => insn!(JsrW),
        L2D::OPCODE => insn!(L2D),
        L2F::OPCODE => insn!(L2F),
        L2I::OPCODE => insn!(L2I),
        Ladd::OPCODE => insn!(Ladd),
        Laload::OPCODE => insn!(Laload),
        Land::OPCODE => insn!(Land),
        Lastore::OPCODE => insn!(Lastore),
        Lcmp::OPCODE => insn!(Lcmp),
        Lconst0::OPCODE => insn!(Lconst0),
        Lconst1::OPCODE => insn!(Lconst1),
        Ldc::OPCODE => insn!(Ldc),
        Ldc2W::OPCODE => insn!(Ldc2W),
        LdcW::OPCODE => insn!(LdcW),
        Ldiv::OPCODE => insn!(Ldiv),
        Lload::OPCODE => insn!(Lload),
        Lload0::OPCODE => insn!(Lload0),
        Lload1::OPCODE => insn!(Lload1),
        Lload2::OPCODE => insn!(Lload2),
        Lload3::OPCODE => insn!(Lload3),
        Lmul::OPCODE => insn!(Lmul),
        Lneg::OPCODE => insn!(Lneg),
        // Lookupswitch::OPCODE => insn!(Lookupswitch),
        Lor::OPCODE => insn!(Lor),
        Lrem::OPCODE => insn!(Lrem),
        Lreturn::OPCODE => insn!(Lreturn),
        Lshl::OPCODE => insn!(Lshl),
        Lshr::OPCODE => insn!(Lshr),
        Lstore::OPCODE => insn!(Lstore),
        Lstore0::OPCODE => insn!(Lstore0),
        Lstore1::OPCODE => insn!(Lstore1),
        Lstore2::OPCODE => insn!(Lstore2),
        Lstore3::OPCODE => insn!(Lstore3),
        Lsub::OPCODE => insn!(Lsub),
        Lushr::OPCODE => insn!(Lushr),
        Lxor::OPCODE => insn!(Lxor),
        Monitorenter::OPCODE => insn!(Monitorenter),
        Monitorexit::OPCODE => insn!(Monitorexit),
        // Multianewarray::OPCODE => insn!(Multianewarray),
        New::OPCODE => insn!(New),
        Newarray::OPCODE => insn!(Newarray),
        Nop::OPCODE => insn!(Nop),
        Pop::OPCODE => insn!(Pop),
        Pop2::OPCODE => insn!(Pop2),
        Putfield::OPCODE => insn!(Putfield),
        Putstatic::OPCODE => insn!(Putstatic),
        Ret::OPCODE => insn!(Ret),
        Return::OPCODE => insn!(Return),
        Saload::OPCODE => insn!(Saload),
        Sastore::OPCODE => insn!(Sastore),
        Sipush::OPCODE => insn!(Sipush),
        Swap::OPCODE => insn!(Swap),
        // Tableswitch::OPCODE => insn!(Tableswitch),
        // Wide::OPCODE => insn!(Wide),
        o => {
            error!("unimplemented opcode {:?}", Opcode::try_from_primitive(o));
            return None;
        }
    };

    Some((reader.pc, opcode))
}

impl InsnReader<'_> {
    pub fn read_u8(&mut self) -> Option<u8> {
        let byte = self.bytes.get(self.pc).copied();
        self.pc += 1;
        byte
    }

    pub fn read_u8s(&mut self) -> Option<(u8, u8)> {
        let pc = self.pc;
        self.pc += 2;
        if self.pc > self.bytes.len() {
            None
        } else {
            let (a, b) = unsafe {
                let a = self.bytes.get_unchecked(pc);
                let b = self.bytes.get_unchecked(pc + 1);
                (*a, *b)
            };

            Some((a, b))
        }
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        let (a, b) = self.read_u8s()?;
        let index = ((a as i16) << 8) | b as i16;
        Some(index)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let (a, b) = self.read_u8s()?;
        let index = ((a as u16) << 8) | b as u16;
        Some(index)
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        let pc = self.pc;
        self.pc += 4;
        if self.pc > self.bytes.len() {
            None
        } else {
            let (a, b, c, d) = unsafe {
                let a = self.bytes.get_unchecked(pc);
                let b = self.bytes.get_unchecked(pc + 1);
                let c = self.bytes.get_unchecked(pc + 2);
                let d = self.bytes.get_unchecked(pc + 3);
                (*a, *b, *c, *d)
            };

            let index = ((a as i32) << 24) | ((b as i32) << 16) | ((c as i32) << 8) | (d as i32);
            Some(index)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::insn::bytecode::InsnReader;
    use crate::interpreter::insn::{get_insn, Aload, Getfield, InstructionBlob, Opcode};

    #[test]
    fn reader() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut reader = InsnReader {
            bytes: bytes.as_slice(),
            pc: 0,
        };

        assert_eq!(reader.read_u8().unwrap(), 1);
        assert_eq!(reader.read_u16().unwrap(), 0x0203);
        assert_eq!(reader.read_u8s().unwrap(), (4, 5));
    }

    #[test]
    fn parse_simple_insns() {
        let mut blob = InstructionBlob::default();
        let bytes = vec![0x2a, 0x19, 0x08, 0xb4, 0x56, 0x78];

        let pc = 0;
        let (pc, opcode) = get_insn(&bytes, pc, &mut blob).unwrap();
        assert_eq!(opcode, Opcode::Aload0);

        let (pc, opcode) = get_insn(&bytes, pc, &mut blob).unwrap();
        assert_eq!(opcode, Opcode::Aload);
        assert_eq!(unsafe { blob.transmute::<Aload>() }.0, 0x08);

        let (pc, opcode) = get_insn(&bytes, pc, &mut blob).unwrap();
        assert_eq!(opcode, Opcode::Getfield);
        assert_eq!(unsafe { blob.transmute::<Getfield>() }.0, 0x5678);

        assert!(get_insn(&bytes, pc, &mut blob).is_none());
    }
}
