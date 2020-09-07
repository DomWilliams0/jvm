use crate::interpreter::error::InterpreterError;
use crate::interpreter::insn::Instruction;
use crate::interpreter::insn::*;
use dynstack::{dyn_push, DynStack};
use num_enum::TryFromPrimitive;

pub struct Bytecode(DynStack<dyn Instruction>);

struct Reader<'b> {
    stream: &'b [u8],
    cursor: usize,
}

impl<'b> Reader<'b> {
    fn read_n(&mut self, n: usize) -> Result<&[u8], InterpreterError> {
        let end = self.cursor + n;
        if end > self.stream.len() {
            Err(InterpreterError::IncompleteInstruction(self.cursor))
        } else {
            let b = &self.stream[self.cursor..end];
            self.cursor = end;
            Ok(b)
        }
    }

    fn read_1(&mut self) -> Result<u8, InterpreterError> {
        self.read_n(1).map(|b| b[0])
    }
}

#[inline]
fn parse_insn(
    reader: &mut Reader,
    _output: &mut DynStack<dyn Instruction>,
    insn: u8,
) -> Result<(), InterpreterError> {
    macro_rules! insn {
        ($insn:expr) => {{
            // TODO temporary, dont log every single instruction
            log::trace!("parsed insn {}", $insn.name());
            dyn_push!(_output, $insn);
        }};
    }

    let opcode =
        Opcode::try_from_primitive(insn).map_err(|_| InterpreterError::InvalidInstruction(insn))?;

    match opcode {
        Opcode::Aload0 => insn!(Aload0),
        Opcode::Aload => insn!(Aload(reader.read_1()?)),
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

#[derive(TryFromPrimitive, Debug, Clone)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0x0,
    AconstNull = 0x1,
    IconstM1 = 0x2,
    Lconst0 = 0x9,
    Fconst0 = 0xb,
    Dconst0 = 0xe,
    Bipush = 0x10,
    Sipush = 0x11,
    Ldc = 0x12,
    LdcW = 0x13,
    Ldc2W = 0x14,
    Iload = 0x15,
    Lload = 0x16,
    Fload = 0x17,
    Dload = 0x18,
    Aload = 0x19,
    Iload0 = 0x1a,
    Lload0 = 0x1e,
    Fload0 = 0x22,
    Dload0 = 0x26,
    Aload0 = 0x2a,
    Iaload = 0x2e,
    Laload = 0x2f,
    Faload = 0x30,
    Daload = 0x31,
    Aaload = 0x32,
    Baload = 0x33,
    Caload = 0x34,
    Saload = 0x35,
    Istore = 0x36,
    Lstore = 0x37,
    Fstore = 0x38,
    Dstore = 0x39,
    Astore = 0x3a,
    Istore0 = 0x3b,
    Lstore0 = 0x3f,
    Fstore0 = 0x43,
    Dstore0 = 0x47,
    Astore0 = 0x4b,
    Iastore = 0x4f,
    Lastore = 0x50,
    Fastore = 0x51,
    Dastore = 0x52,
    Aastore = 0x53,
    Bastore = 0x54,
    Castore = 0x55,
    Sastore = 0x56,
    Pop = 0x57,
    Pop2 = 0x58,
    Dup = 0x59,
    DupX1 = 0x5a,
    DupX2 = 0x5b,
    Dup2 = 0x5c,
    Dup2X1 = 0x5d,
    Dup2X2 = 0x5e,
    Swap = 0x5f,
    Iadd = 0x60,
    Ladd = 0x61,
    Fadd = 0x62,
    Dadd = 0x63,
    Isub = 0x64,
    Lsub = 0x65,
    Fsub = 0x66,
    Dsub = 0x67,
    Imul = 0x68,
    Lmul = 0x69,
    Fmul = 0x6a,
    Dmul = 0x6b,
    Idiv = 0x6c,
    Ldiv = 0x6d,
    Fdiv = 0x6e,
    Ddiv = 0x6f,
    Irem = 0x70,
    Lrem = 0x71,
    Frem = 0x72,
    Drem = 0x73,
    Ineg = 0x74,
    Lneg = 0x75,
    Fneg = 0x76,
    Dneg = 0x77,
    Ishl = 0x78,
    Lshl = 0x79,
    Ishr = 0x7a,
    Lshr = 0x7b,
    Iushr = 0x7c,
    Lushr = 0x7d,
    Iand = 0x7e,
    Land = 0x7f,
    Ior = 0x80,
    Lor = 0x81,
    Ixor = 0x82,
    Lxor = 0x83,
    Iinc = 0x84,
    I2L = 0x85,
    I2F = 0x86,
    I2D = 0x87,
    L2I = 0x88,
    L2F = 0x89,
    L2D = 0x8a,
    F2I = 0x8b,
    F2L = 0x8c,
    F2D = 0x8d,
    D2I = 0x8e,
    D2L = 0x8f,
    D2F = 0x90,
    I2B = 0x91,
    I2C = 0x92,
    I2S = 0x93,
    Lcmp = 0x94,
    Fcmpg = 0x96,
    Dcmpg = 0x98,
    Ifeq = 0x99,
    IfIcmpeq = 0x9f,
    IfAcmpeq = 0xa5,
    Goto = 0xa7,
    Jsr = 0xa8,
    Ret = 0xa9,
    Tableswitch = 0xaa,
    Lookupswitch = 0xab,
    Ireturn = 0xac,
    Lreturn = 0xad,
    Freturn = 0xae,
    Dreturn = 0xaf,
    Areturn = 0xb0,
    Return = 0xb1,
    Getstatic = 0xb2,
    Putstatic = 0xb3,
    Getfield = 0xb4,
    Putfield = 0xb5,
    Invokevirtual = 0xb6,
    Invokespecial = 0xb7,
    Invokestatic = 0xb8,
    Invokeinterface = 0xb9,
    Invokedynamic = 0xba,
    New = 0xbb,
    Newarray = 0xbc,
    Anewarray = 0xbd,
    Arraylength = 0xbe,
    Athrow = 0xbf,
    Checkcast = 0xc0,
    Instanceof = 0xc1,
    Monitorenter = 0xc2,
    Monitorexit = 0xc3,
    Wide = 0xc4,
    Multianewarray = 0xc5,
    Ifnull = 0xc6,
    Ifnonnull = 0xc7,
    GotoW = 0xc8,
    JsrW = 0xc9,
    Breakpoint = 0xca,
    Impdep1 = 0xfe,
    Impdep2 = 0xff,
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

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
