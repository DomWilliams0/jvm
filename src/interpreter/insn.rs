use crate::interpreter::error::InterpreterError;
use dynstack::{dyn_push, DynStack};

pub trait Instruction {
    fn name(&self) -> &'static str;

    // fn execute(&self, local_vars: &mut LocalV)
}

pub struct Aload0;
pub struct Aload(pub u8);

impl Instruction for Aload0 {
    fn name(&self) -> &'static str {
        "aload_0"
    }
}

impl Instruction for Aload {
    fn name(&self) -> &'static str {
        "aload"
    }
}

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
    match insn {
        0x2a => insn!(Aload0),
        0x19 => insn!(Aload(reader.read_1()?)),

        _ => {
            return Err(InterpreterError::InvalidInstruction(insn));
        }
    };

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
