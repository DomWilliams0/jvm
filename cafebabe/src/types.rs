use crate::buffer::Buffer;
use crate::constant_pool::ConstantPool;
use crate::{ClassError, ClassResult};
use bitflags::bitflags;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ClassVersion {
    major: u16,
    minor: u16,
}

bitflags! {
    pub struct ClassAccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        /// Declared final; no subclasses allowed.
        const FINAL = 0x0010;
        /// Treat superclass methods specially when invoked by the invokespecial instruction.
        const SUPER = 0x0020;
        /// Is an interface, not a class.
        const INTERFACE = 0x0200;
        /// Declared abstract; must not be instantiated.
        const ABSTRACT = 0x0400;
        /// Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
        /// Declared as an annotation type.
        const ANNOTATION = 0x2000;
        /// Declared as an enum type.
        const ENUM = 0x4000;
        /// Is a module, not a class or interface.
        const MODULE = 0x8000;
    }

}

bitflags! {
    pub struct FieldAccessFlags: u16 {
        ///  Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        ///  Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const PRIVATE = 0x0002;
        ///  Declared protected; may be accessed within subclasses.
        const PROTECTED = 0x0004;
        ///  Declared static.
        const STATIC = 0x0008;
        ///  Declared final; never directly assigned to after object construction (JLS §17.5).
        const FINAL = 0x0010;
        ///  Declared volatile; cannot be cached.
        const VOLATILE = 0x0040;
        ///  Declared transient; not written or read by a persistent object manager.
        const TRANSIENT = 0x0080;
        ///  Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
        ///  Declared as an element of an enum.
        const ENUM = 0x4000;
    }

}

bitflags! {
    pub struct MethodAccessFlags: u16 {
        ///  Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        ///  Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const PRIVATE = 0x0002;
        ///  Declared protected; may be accessed within subclasses.
        const PROTECTED = 0x0004;
        ///  Declared static.
        const STATIC = 0x0008;
        ///  Declared final; must not be overridden (§5.4.5).
        const FINAL = 0x0010;
        ///  Declared synchronized; invocation is wrapped by a monitor use.
        const SYNCHRONIZED = 0x0020;
        ///  A bridge method, generated by the compiler.
        const BRIDGE = 0x0040;
        ///  Declared with variable number of arguments.
        const VARARGS = 0x0080;
        ///  Declared native; implemented in a language other than the Java programming language.
        const NATIVE = 0x0100;
        ///  Declared abstract; no implementation is provided.
        const ABSTRACT = 0x0400;
        ///  Declared strictfp; floating-point mode is FP-strict.
        const STRICT = 0x0800;
        ///  Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
    }
}

bitflags! {
    pub struct CommonAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const STATIC = 0x0008;
        const SYNTHETIC = 0x1000;
    }
}

pub trait AccessFlags: Copy {
    fn common(self) -> CommonAccessFlags;

    fn is_public(&self) -> bool {
        self.common().contains(CommonAccessFlags::PUBLIC)
    }
    fn is_static(&self) -> bool {
        self.common().contains(CommonAccessFlags::STATIC)
    }
    fn is_final(&self) -> bool {
        self.common().contains(CommonAccessFlags::FINAL)
    }
}

// TODO resolve constant pool entries
// TODO reduce duplication

#[derive(Debug)]
pub struct FieldInfo<'c> {
    pub access_flags: FieldAccessFlags,
    pub name: &'c mutf8::mstr,
    pub descriptor: &'c mutf8::mstr,
    pub attributes: Vec<RawAttribute<'c>>,
}

#[derive(Debug)]
pub struct MethodInfo<'c> {
    pub access_flags: MethodAccessFlags,
    pub name: &'c mutf8::mstr,
    pub descriptor: &'c mutf8::mstr,
    pub attributes: Vec<RawAttribute<'c>>,
}

pub struct RawAttribute<'c> {
    pub name: &'c mutf8::mstr,
    pub info: &'c [u8],
}

impl ClassVersion {
    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn is_supported(&self) -> bool {
        // java SE 11
        let minor_range = 45..=55;
        self.major == 0 && minor_range.contains(&self.minor)
    }
}

impl Display for ClassVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl<'c> FieldInfo<'c> {
    pub fn load(buf: &mut Buffer<'c>, constant_pool: &ConstantPool<'c>) -> ClassResult<Self> {
        let access_flags = {
            let int = buf.read()?;
            let flags =
                FieldAccessFlags::from_bits(int).ok_or_else(|| ClassError::AccessFlags(int))?;
            // TODO validate combinations
            flags
        };

        let name = constant_pool.string_entry(buf.read()?)?;
        let descriptor = constant_pool.string_entry(buf.read()?)?;

        let attributes = {
            let count = buf.read::<u16>()? as usize;
            RawAttribute::load_n(buf, constant_pool, count)?
        };

        Ok(Self {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
}

impl<'c> MethodInfo<'c> {
    pub fn load(buf: &mut Buffer<'c>, constant_pool: &ConstantPool<'c>) -> ClassResult<Self> {
        let access_flags = {
            let int = buf.read()?;
            let flags =
                MethodAccessFlags::from_bits(int).ok_or_else(|| ClassError::AccessFlags(int))?;
            // TODO validate combinations
            flags
        };

        let name = constant_pool.string_entry(buf.read()?)?;
        let descriptor = constant_pool.string_entry(buf.read()?)?;

        let attributes = {
            let count = buf.read::<u16>()? as usize;
            RawAttribute::load_n(buf, constant_pool, count)?
        };

        Ok(Self {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }
}

impl<'c> RawAttribute<'c> {
    pub fn load(buf: &mut Buffer<'c>, constant_pool: &ConstantPool<'c>) -> ClassResult<Self> {
        let name = constant_pool.string_entry(buf.read()?)?;
        let length = buf.read::<u32>()? as usize;
        let info = buf.read_slice(length)?;
        Ok(Self { name, info })
    }

    pub fn load_n(
        buf: &mut Buffer<'c>,
        constant_pool: &ConstantPool<'c>,
        n: usize,
    ) -> ClassResult<Vec<Self>> {
        let mut attributes = Vec::with_capacity(n);
        for _ in 0..n {
            attributes.push(RawAttribute::load(buf, constant_pool)?);
        }

        Ok(attributes)
    }
}

impl<'c> Debug for RawAttribute<'c> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attribute({:?}, {} bytes)", self.name, self.info.len())
    }
}

impl AccessFlags for ClassAccessFlags {
    fn common(self) -> CommonAccessFlags {
        CommonAccessFlags::from_bits_truncate(self.bits)
    }
}
impl AccessFlags for FieldAccessFlags {
    fn common(self) -> CommonAccessFlags {
        CommonAccessFlags::from_bits_truncate(self.bits)
    }
}

impl AccessFlags for MethodAccessFlags {
    fn common(self) -> CommonAccessFlags {
        CommonAccessFlags::from_bits_truncate(self.bits)
    }
}

impl MethodAccessFlags {
    pub fn is_native(&self) -> bool {
        self.contains(MethodAccessFlags::NATIVE)
    }
}

#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    #[test]
    fn parse_CharData_UPPER_SPECIAL() {
        let bytes = [
            0xc3, 0x9f, 0xc0, 0x80, 0xc5, 0x89, 0x02, 0xc7, 0xb0, 0x04, 0xce, 0x90, 0x06, 0xce,
            0xb0, 0x09, 0xd6, 0x87, 0x0c, 0xe1, 0xba, 0x96, 0x0e, 0xe1, 0xba, 0x97, 0x10, 0xe1,
            0xba, 0x98, 0x12, 0xe1, 0xba, 0x99, 0x14, 0xe1, 0xba, 0x9a, 0x16, 0xe1, 0xbd, 0x90,
            0x18, 0xe1, 0xbd, 0x92, 0x1a, 0xe1, 0xbd, 0x94, 0x1d, 0xe1, 0xbd, 0x96, 0x20, 0xe1,
            0xbe, 0x80, 0x23, 0xe1, 0xbe, 0x81, 0x25, 0xe1, 0xbe, 0x82, 0x27, 0xe1, 0xbe, 0x83,
            0x29, 0xe1, 0xbe, 0x84, 0x2b, 0xe1, 0xbe, 0x85, 0x2d, 0xe1, 0xbe, 0x86, 0x2f, 0xe1,
            0xbe, 0x87, 0x31, 0xe1, 0xbe, 0x88, 0x33, 0xe1, 0xbe, 0x89, 0x35, 0xe1, 0xbe, 0x8a,
            0x37, 0xe1, 0xbe, 0x8b, 0x39, 0xe1, 0xbe, 0x8c, 0x3b, 0xe1, 0xbe, 0x8d, 0x3d, 0xe1,
            0xbe, 0x8e, 0x3f, 0xe1, 0xbe, 0x8f, 0x41, 0xe1, 0xbe, 0x90, 0x43, 0xe1, 0xbe, 0x91,
            0x45, 0xe1, 0xbe, 0x92, 0x47, 0xe1, 0xbe, 0x93, 0x49, 0xe1, 0xbe, 0x94, 0x4b, 0xe1,
            0xbe, 0x95, 0x4d, 0xe1, 0xbe, 0x96, 0x4f, 0xe1, 0xbe, 0x97, 0x51, 0xe1, 0xbe, 0x98,
            0x53, 0xe1, 0xbe, 0x99, 0x55, 0xe1, 0xbe, 0x9a, 0x57, 0xe1, 0xbe, 0x9b, 0x59, 0xe1,
            0xbe, 0x9c, 0x5b, 0xe1, 0xbe, 0x9d, 0x5d, 0xe1, 0xbe, 0x9e, 0x5f, 0xe1, 0xbe, 0x9f,
            0x61, 0xe1, 0xbe, 0xa0, 0x63, 0xe1, 0xbe, 0xa1, 0x65, 0xe1, 0xbe, 0xa2, 0x67, 0xe1,
            0xbe, 0xa3, 0x69, 0xe1, 0xbe, 0xa4, 0x6b, 0xe1, 0xbe, 0xa5, 0x6d, 0xe1, 0xbe, 0xa6,
            0x6f, 0xe1, 0xbe, 0xa7, 0x71, 0xe1, 0xbe, 0xa8, 0x73, 0xe1, 0xbe, 0xa9, 0x75, 0xe1,
            0xbe, 0xaa, 0x77, 0xe1, 0xbe, 0xab, 0x79, 0xe1, 0xbe, 0xac, 0x7b, 0xe1, 0xbe, 0xad,
            0x7d, 0xe1, 0xbe, 0xae, 0x7f, 0xe1, 0xbe, 0xaf, 0xc2, 0x81, 0xe1, 0xbe, 0xb2, 0xc2,
            0x83, 0xe1, 0xbe, 0xb3, 0xc2, 0x85, 0xe1, 0xbe, 0xb4, 0xc2, 0x87, 0xe1, 0xbe, 0xb6,
            0xc2, 0x89, 0xe1, 0xbe, 0xb7, 0xc2, 0x8b, 0xe1, 0xbe, 0xbc, 0xc2, 0x8e, 0xe1, 0xbf,
            0x82, 0xc2, 0x90, 0xe1, 0xbf, 0x83, 0xc2, 0x92, 0xe1, 0xbf, 0x84, 0xc2, 0x94, 0xe1,
            0xbf, 0x86, 0xc2, 0x96, 0xe1, 0xbf, 0x87, 0xc2, 0x98, 0xe1, 0xbf, 0x8c, 0xc2, 0x9b,
            0xe1, 0xbf, 0x92, 0xc2, 0x9d, 0xe1, 0xbf, 0x93, 0xc2, 0xa0, 0xe1, 0xbf, 0x96, 0xc2,
            0xa3, 0xe1, 0xbf, 0x97, 0xc2, 0xa5, 0xe1, 0xbf, 0xa2, 0xc2, 0xa8, 0xe1, 0xbf, 0xa3,
            0xc2, 0xab, 0xe1, 0xbf, 0xa4, 0xc2, 0xae, 0xe1, 0xbf, 0xa6, 0xc2, 0xb0, 0xe1, 0xbf,
            0xa7, 0xc2, 0xb2, 0xe1, 0xbf, 0xb2, 0xc2, 0xb5, 0xe1, 0xbf, 0xb3, 0xc2, 0xb7, 0xe1,
            0xbf, 0xb4, 0xc2, 0xb9, 0xe1, 0xbf, 0xb6, 0xc2, 0xbb, 0xe1, 0xbf, 0xb7, 0xc2, 0xbd,
            0xe1, 0xbf, 0xbc, 0xc3, 0x80, 0xef, 0xac, 0x80, 0xc3, 0x82, 0xef, 0xac, 0x81, 0xc3,
            0x84, 0xef, 0xac, 0x82, 0xc3, 0x86, 0xef, 0xac, 0x83, 0xc3, 0x88, 0xef, 0xac, 0x84,
            0xc3, 0x8b, 0xef, 0xac, 0x85, 0xc3, 0x8e, 0xef, 0xac, 0x86, 0xc3, 0x90, 0xef, 0xac,
            0x93, 0xc3, 0x92, 0xef, 0xac, 0x94, 0xc3, 0x94, 0xef, 0xac, 0x95, 0xc3, 0x96, 0xef,
            0xac, 0x96, 0xc3, 0x98, 0xef, 0xac, 0x97, 0xc3, 0x9a,
        ];

        // not utf8
        assert!(std::str::from_utf8(&bytes).is_err());

        // is mutf8
        let mstring = mutf8::mstr::from_utf8(&bytes);
        eprintln!("nice {:?}", mstring);
        assert_eq!(mstring.as_bytes(), &bytes[..]);

        // can convert back and forth losslessly
        let str = mstring.to_utf8();
        let mstring2 = mutf8::mstr::from_utf8(str.as_bytes());
        assert_eq!(mstring2.as_bytes(), &bytes[..]);
    }
}
