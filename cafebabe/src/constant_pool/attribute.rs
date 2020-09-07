use crate::buffer::Buffer;
use crate::constant_pool::ConstantPool;
use crate::{constant_pool, ClassError, ClassResult, RawAttribute};
use mutf8::MString;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub trait Attribute: Sized {
    const NAME: &'static str;

    fn parse(bytes: &[u8], constant_pool: &ConstantPool) -> ClassResult<Self>;
}

pub enum OwnedAttribute {
    SourceFile(SourceFile),
    Code(Code),
    Other { name: MString, info: Box<[u8]> },
}

#[derive(Debug)]
pub struct SourceFile(pub MString);

pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Arc<[u8]>,
    // TODO exception handlers
    // TODO attributes
}

impl Attribute for SourceFile {
    const NAME: &'static str = "SourceFile";

    fn parse(bytes: &[u8], constant_pool: &ConstantPool) -> ClassResult<Self> {
        if bytes.len() != 2 {
            return Err(ClassError::AttributeFormat("attr length should be 2"));
        }

        let index = constant_pool::Index::from_be_bytes([bytes[0], bytes[1]]);
        constant_pool
            .string_entry(index)
            .map(|s| SourceFile(s.to_owned()))
    }
}

impl Attribute for Code {
    const NAME: &'static str = "Code";

    fn parse(bytes: &[u8], _: &ConstantPool) -> ClassResult<Self> {
        let mut buf = Buffer::new(bytes);
        let max_stack = buf.read()?;
        let max_locals = buf.read()?;

        let code_len: u32 = buf.read()?;
        let code = Arc::from(
            buf.read_slice(code_len as usize)?
                .to_owned()
                .into_boxed_slice(),
        );

        Ok(Code {
            max_stack,
            max_locals,
            code,
        })
    }
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Code")
            .field("max_stack", &self.max_stack)
            .field("max_locals", &self.max_locals)
            .field("code length", &self.code.len())
            .finish()
    }
}

impl<'c> RawAttribute<'c> {
    pub fn to_owned(&self, constant_pool: &ConstantPool) -> ClassResult<OwnedAttribute> {
        Ok(match self.name.to_utf8().as_ref() {
            Code::NAME => OwnedAttribute::Code(Code::parse(self.info, constant_pool)?),
            SourceFile::NAME => {
                OwnedAttribute::SourceFile(SourceFile::parse(self.info, constant_pool)?)
            }
            _ => OwnedAttribute::Other {
                name: self.name.to_owned(),
                info: self.info.to_vec().into_boxed_slice(),
            },
        })
    }
}

impl Debug for OwnedAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedAttribute::SourceFile(a) => write!(f, "{:?}", a),
            OwnedAttribute::Code(a) => write!(f, "{:?}", a),
            OwnedAttribute::Other { name, .. } => write!(f, "{:?}", name),
        }
    }
}
