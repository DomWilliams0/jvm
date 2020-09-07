mod bytecode;
pub use bytecode::Bytecode;
pub use bytecode::Opcode;

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
