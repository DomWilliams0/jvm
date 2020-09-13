use crate::interpreter::error::InterpreterError;
use crate::interpreter::insn::instruction::Instruction;
use crate::interpreter::insn::instruction::*;
use crate::interpreter::insn::opcode::Opcode;
use dynstack::{dyn_push, DynStack};
use num_enum::TryFromPrimitive;

pub struct Bytecode(DynStack<dyn Instruction>);

pub(crate) struct Reader<'b> {
    stream: &'b [u8],
    cursor: usize,
}

impl<'b> Reader<'b> {
    pub(crate) fn read_n(&mut self, n: usize) -> Result<&[u8], InterpreterError> {
        let end = self.cursor + n;
        if end > self.stream.len() {
            Err(InterpreterError::IncompleteInstruction(self.cursor))
        } else {
            let b = &self.stream[self.cursor..end];
            self.cursor = end;
            Ok(b)
        }
    }

    pub(crate) fn read_1(&mut self) -> Result<u8, InterpreterError> {
        self.read_n(1).map(|b| b[0])
    }
}

#[inline]
fn parse_insn(
    _reader: &mut Reader,
    _output: &mut DynStack<dyn Instruction>,
    insn: u8,
) -> Result<(), InterpreterError> {
    macro_rules! insn {
        ($insn:ident) => {{
            let insn = $insn::parse(_reader)?;
            // TODO temporary, dont log every single instruction
            log::trace!("parsed insn {:?}", insn);
            dyn_push!(_output, insn);
        }};
    }

    let opcode =
        Opcode::try_from_primitive(insn).map_err(|_| InterpreterError::InvalidInstruction(insn))?;

    match opcode {
        // Opcode::Aaload => insn!(Aaload),
        // Opcode::Aastore => insn!(Aastore),
        // Opcode::AconstNull => insn!(AconstNull),
        Opcode::Aload => insn!(Aload),
        Opcode::Aload0 => insn!(Aload0),
        Opcode::Aload1 => insn!(Aload1),
        Opcode::Aload2 => insn!(Aload2),
        Opcode::Aload3 => insn!(Aload3),
        // Opcode::Anewarray => insn!(Anewarray),
        // Opcode::Areturn => insn!(Areturn),
        Opcode::Arraylength => insn!(Arraylength),
        // Opcode::Astore => insn!(Astore),
        // Opcode::Astore0 => insn!(Astore0),
        // Opcode::Astore1 => insn!(Astore1),
        // Opcode::Astore2 => insn!(Astore2),
        // Opcode::Astore3 => insn!(Astore3),
        // Opcode::Athrow => insn!(Athrow),
        // Opcode::Baload => insn!(Baload),
        // Opcode::Bastore => insn!(Bastore),
        Opcode::Bipush => insn!(Bipush),
        // Opcode::Caload => insn!(Caload),
        // Opcode::Castore => insn!(Castore),
        // Opcode::Checkcast => insn!(Checkcast),
        // Opcode::D2F => insn!(D2F),
        // Opcode::D2I => insn!(D2I),
        // Opcode::D2L => insn!(D2L),
        // Opcode::Dadd => insn!(Dadd),
        // Opcode::Daload => insn!(Daload),
        // Opcode::Dastore => insn!(Dastore),
        // Opcode::Dcmpg => insn!(Dcmpg),
        // Opcode::Dcmpl => insn!(Dcmpl),
        // Opcode::Dconst0 => insn!(Dconst0),
        // Opcode::Dconst1 => insn!(Dconst1),
        // Opcode::Ddiv => insn!(Ddiv),
        // Opcode::Dload => insn!(Dload),
        // Opcode::Dload0 => insn!(Dload0),
        // Opcode::Dload1 => insn!(Dload1),
        // Opcode::Dload2 => insn!(Dload2),
        // Opcode::Dload3 => insn!(Dload3),
        // Opcode::Dmul => insn!(Dmul),
        // Opcode::Dneg => insn!(Dneg),
        // Opcode::Drem => insn!(Drem),
        // Opcode::Dreturn => insn!(Dreturn),
        // Opcode::Dstore => insn!(Dstore),
        // Opcode::Dstore0 => insn!(Dstore0),
        // Opcode::Dstore1 => insn!(Dstore1),
        // Opcode::Dstore2 => insn!(Dstore2),
        // Opcode::Dstore3 => insn!(Dstore3),
        // Opcode::Dsub => insn!(Dsub),
        Opcode::Dup => insn!(Dup),
        // Opcode::Dup2 => insn!(Dup2),
        // Opcode::Dup2X1 => insn!(Dup2X1),
        // Opcode::Dup2X2 => insn!(Dup2X2),
        // Opcode::DupX1 => insn!(DupX1),
        // Opcode::DupX2 => insn!(DupX2),
        // Opcode::F2D => insn!(F2D),
        // Opcode::F2I => insn!(F2I),
        // Opcode::F2L => insn!(F2L),
        // Opcode::Fadd => insn!(Fadd),
        // Opcode::Faload => insn!(Faload),
        // Opcode::Fastore => insn!(Fastore),
        // Opcode::Fcmpg => insn!(Fcmpg),
        // Opcode::Fcmpl => insn!(Fcmpl),
        // Opcode::Fconst0 => insn!(Fconst0),
        // Opcode::Fconst1 => insn!(Fconst1),
        // Opcode::Fconst2 => insn!(Fconst2),
        // Opcode::Fdiv => insn!(Fdiv),
        // Opcode::Fload => insn!(Fload),
        // Opcode::Fload0 => insn!(Fload0),
        // Opcode::Fload1 => insn!(Fload1),
        // Opcode::Fload2 => insn!(Fload2),
        // Opcode::Fload3 => insn!(Fload3),
        // Opcode::Fmul => insn!(Fmul),
        // Opcode::Fneg => insn!(Fneg),
        // Opcode::Frem => insn!(Frem),
        // Opcode::Freturn => insn!(Freturn),
        // Opcode::Fstore => insn!(Fstore),
        // Opcode::Fstore0 => insn!(Fstore0),
        // Opcode::Fstore1 => insn!(Fstore1),
        // Opcode::Fstore2 => insn!(Fstore2),
        // Opcode::Fstore3 => insn!(Fstore3),
        // Opcode::Fsub => insn!(Fsub),
        // Opcode::Getfield => insn!(Getfield),
        // Opcode::Getstatic => insn!(Getstatic),
        // Opcode::Goto => insn!(Goto),
        // Opcode::GotoW => insn!(GotoW),
        // Opcode::I2B => insn!(I2B),
        // Opcode::I2C => insn!(I2C),
        // Opcode::I2D => insn!(I2D),
        // Opcode::I2F => insn!(I2F),
        // Opcode::I2L => insn!(I2L),
        // Opcode::I2S => insn!(I2S),
        // Opcode::Iadd => insn!(Iadd),
        // Opcode::Iaload => insn!(Iaload),
        // Opcode::Iand => insn!(Iand),
        // Opcode::Iastore => insn!(Iastore),
        Opcode::Iconst0 => insn!(Iconst0),
        // Opcode::Iconst1 => insn!(Iconst1),
        // Opcode::Iconst2 => insn!(Iconst2),
        // Opcode::Iconst3 => insn!(Iconst3),
        // Opcode::Iconst4 => insn!(Iconst4),
        // Opcode::Iconst5 => insn!(Iconst5),
        // Opcode::IconstM1 => insn!(IconstM1),
        // Opcode::Idiv => insn!(Idiv),
        // Opcode::IfAcmpeq => insn!(IfAcmpeq),
        // Opcode::IfAcmpne => insn!(IfAcmpne),
        // Opcode::IfIcmpeq => insn!(IfIcmpeq),
        // Opcode::IfIcmpge => insn!(IfIcmpge),
        // Opcode::IfIcmpgt => insn!(IfIcmpgt),
        // Opcode::IfIcmple => insn!(IfIcmple),
        // Opcode::IfIcmplt => insn!(IfIcmplt),
        // Opcode::IfIcmpne => insn!(IfIcmpne),
        // Opcode::Ifeq => insn!(Ifeq),
        // Opcode::Ifge => insn!(Ifge),
        // Opcode::Ifgt => insn!(Ifgt),
        // Opcode::Ifle => insn!(Ifle),
        // Opcode::Iflt => insn!(Iflt),
        // Opcode::Ifne => insn!(Ifne),
        // Opcode::Ifnonnull => insn!(Ifnonnull),
        // Opcode::Ifnull => insn!(Ifnull),
        // Opcode::Iinc => insn!(Iinc),
        // Opcode::Iload => insn!(Iload),
        // Opcode::Iload0 => insn!(Iload0),
        // Opcode::Iload1 => insn!(Iload1),
        // Opcode::Iload2 => insn!(Iload2),
        // Opcode::Iload3 => insn!(Iload3),
        // Opcode::Imul => insn!(Imul),
        // Opcode::Ineg => insn!(Ineg),
        // Opcode::Instanceof => insn!(Instanceof),
        // Opcode::Invokedynamic => insn!(Invokedynamic),
        // Opcode::Invokeinterface => insn!(Invokeinterface),
        Opcode::Invokespecial => insn!(Invokespecial),
        Opcode::Invokestatic => insn!(Invokestatic),
        // Opcode::Invokevirtual => insn!(Invokevirtual),
        // Opcode::Ior => insn!(Ior),
        // Opcode::Irem => insn!(Irem),
        // Opcode::Ireturn => insn!(Ireturn),
        // Opcode::Ishl => insn!(Ishl),
        // Opcode::Ishr => insn!(Ishr),
        // Opcode::Istore => insn!(Istore),
        // Opcode::Istore0 => insn!(Istore0),
        // Opcode::Istore1 => insn!(Istore1),
        // Opcode::Istore2 => insn!(Istore2),
        // Opcode::Istore3 => insn!(Istore3),
        // Opcode::Isub => insn!(Isub),
        // Opcode::Iushr => insn!(Iushr),
        // Opcode::Ixor => insn!(Ixor),
        // Opcode::Jsr => insn!(Jsr),
        // Opcode::JsrW => insn!(JsrW),
        // Opcode::L2D => insn!(L2D),
        // Opcode::L2F => insn!(L2F),
        // Opcode::L2I => insn!(L2I),
        // Opcode::Ladd => insn!(Ladd),
        // Opcode::Laload => insn!(Laload),
        // Opcode::Land => insn!(Land),
        // Opcode::Lastore => insn!(Lastore),
        // Opcode::Lcmp => insn!(Lcmp),
        // Opcode::Lconst0 => insn!(Lconst0),
        Opcode::Ldc => insn!(Ldc),
        // Opcode::Ldc2W => insn!(Ldc2W),
        // Opcode::LdcW => insn!(LdcW),
        // Opcode::Ldiv => insn!(Ldiv),
        // Opcode::Lload => insn!(Lload),
        // Opcode::Lload0 => insn!(Lload0),
        // Opcode::Lload1 => insn!(Lload1),
        // Opcode::Lload2 => insn!(Lload2),
        // Opcode::Lload3 => insn!(Lload3),
        // Opcode::Lmul => insn!(Lmul),
        // Opcode::Lneg => insn!(Lneg),
        // Opcode::Lookupswitch => insn!(Lookupswitch),
        // Opcode::Lor => insn!(Lor),
        // Opcode::Lrem => insn!(Lrem),
        // Opcode::Lreturn => insn!(Lreturn),
        // Opcode::Lshl => insn!(Lshl),
        // Opcode::Lshr => insn!(Lshr),
        // Opcode::Lstore => insn!(Lstore),
        // Opcode::Lstore0 => insn!(Lstore0),
        // Opcode::Lstore1 => insn!(Lstore1),
        // Opcode::Lstore2 => insn!(Lstore2),
        // Opcode::Lstore3 => insn!(Lstore3),
        // Opcode::Lsub => insn!(Lsub),
        // Opcode::Lushr => insn!(Lushr),
        // Opcode::Lxor => insn!(Lxor),
        // Opcode::Monitorenter => insn!(Monitorenter),
        // Opcode::Monitorexit => insn!(Monitorexit),
        // Opcode::Multianewarray => insn!(Multianewarray),
        Opcode::New => insn!(New),
        // Opcode::Newarray => insn!(Newarray),
        // Opcode::Nop => insn!(Nop),
        // Opcode::Pop => insn!(Pop),
        // Opcode::Pop2 => insn!(Pop2),
        // Opcode::Putfield => insn!(Putfield),
        Opcode::Putstatic => insn!(Putstatic),
        // Opcode::Ret => insn!(Ret),
        Opcode::Return => insn!(Return),
        // Opcode::Saload => insn!(Saload),
        // Opcode::Sastore => insn!(Sastore),
        // Opcode::Sipush => insn!(Sipush),
        // Opcode::Swap => insn!(Swap),
        // Opcode::Tableswitch => insn!(Tableswitch),
        // Opcode::Wide => insn!(Wide),
        o => {
            return Err(InterpreterError::UnimplementedOpcode(o));
        }
    }

    Ok(())
}

impl Bytecode {
    // TODO verified version of Bytecode that doesn't do all the safety checks for speed e.g. fn parse_unverified(bytes) -> Self
    pub fn parse(bytes: &[u8]) -> Result<Self, InterpreterError> {
        let mut insns = DynStack::<dyn Instruction>::new();
        let mut reader = Reader {
            stream: bytes,
            cursor: 0,
        };

        loop {
            let insn = reader.read_1();
            match insn {
                Err(_) => break, // done
                Ok(insn) => {
                    parse_insn(&mut reader, &mut insns, insn)?;
                }
            };
        }

        Ok(Bytecode(insns))
    }

    pub fn instructions(&self) -> impl Iterator<Item = &dyn Instruction> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn reader() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut reader = Reader {
            stream: bytes.as_slice(),
            cursor: 0,
        };

        assert_eq!(reader.read_1().unwrap(), 1);
        assert_eq!(reader.read_1().unwrap(), 2);
        assert_eq!(reader.read_1().unwrap(), 3);
        assert_eq!(reader.read_n(2).unwrap(), [4, 5]);
    }

    #[test]
    fn parse_simple_aloads() {
        let bytes = vec![0x2a, 0x19, 0x08];
        let code = Bytecode::parse(&bytes).expect("should succeed");

        let insns = code.instructions().collect_vec();

        assert_eq!(insns.len(), 2);

        assert_eq!(insns[0].name(), "aload_0");
        assert_eq!(insns[1].name(), "aload");
    }

    #[test]
    fn incomplete() {
        let bytes = vec![0x19]; // aload without index
        let err = Bytecode::parse(&bytes);
        assert!(matches!(
            err,
            Err(InterpreterError::IncompleteInstruction(_))
        ));
    }
}
