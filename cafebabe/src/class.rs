use log::*;

use crate::buffer::Buffer;
use crate::constant_pool::attribute::Attribute;
use crate::types::{ClassAccessFlags, ClassVersion, FieldInfo, MethodInfo, RawAttribute};
use crate::{constant_pool, ClassError, ClassRefEntry, ClassResult, ConstantPool, Index};
use mutf8::StrExt;

#[derive(Debug)]
pub struct ClassFile<'c> {
    version: ClassVersion,
    constant_pool: ConstantPool<'c>,
    access_flags: ClassAccessFlags,

    this_class: constant_pool::Index,
    super_class: constant_pool::Index,

    interfaces: Vec<constant_pool::Index>,
    fields: Vec<FieldInfo<'c>>,
    methods: Vec<MethodInfo<'c>>,
    attributes: Vec<RawAttribute<'c>>,
}

impl<'c> ClassFile<'c> {
    pub(crate) fn load(buf: &'c [u8]) -> ClassResult<Self> {
        let mut buf = Buffer::new(buf);

        // magic check
        if buf.read::<u32>()? != 0xcafebabe {
            return Err(ClassError::Magic);
        }

        let version = {
            let major = buf.read::<u16>()?;
            let minor = buf.read::<u16>()?;
            ClassVersion::new(major, minor)
        };

        debug!("class version: {}", version);
        if !version.is_supported() {
            return Err(ClassError::Unsupported(version));
        }

        let constant_pool = ConstantPool::load(&mut buf)?;
        let access_flags = {
            let int = buf.read()?;
            let flags =
                ClassAccessFlags::from_bits(int).ok_or(ClassError::AccessFlags(int))?;
            // TODO validate combinations
            debug!("access flags: {:?}", flags);
            flags
        };

        let this_class = buf.read()?;
        let super_class = buf.read()?;

        let interfaces = {
            let count = buf.read::<u16>()? as usize;
            debug!("{} interfaces", count);
            buf.read_n_u16(count)?
        };

        let fields = {
            let count = buf.read::<u16>()? as usize;
            debug!("{} fields", count);
            let mut fields = Vec::with_capacity(count);
            for _ in 0..count {
                fields.push(FieldInfo::load(&mut buf, &constant_pool)?);
            }

            // TODO detect dups with same name & descriptor
            fields
        };

        let methods = {
            let count = buf.read::<u16>()? as usize;
            debug!("{} methods", count);
            let mut methods = Vec::with_capacity(count);
            for _ in 0..count {
                methods.push(MethodInfo::load(&mut buf, &constant_pool)?);
            }

            // TODO detect dups with same name & descriptor
            methods
        };

        let attributes = {
            let count = buf.read::<u16>()? as usize;
            debug!("{} attributes", count);
            RawAttribute::load_n(&mut buf, &constant_pool, count)?
        };

        Ok(Self {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }

    fn class_name(&'c self, index: Index) -> ClassResult<&'c mutf8::mstr> {
        self.constant_pool
            .entry::<ClassRefEntry<'c>>(index)
            .map(|class| class.name)
    }

    pub fn this_class(&self) -> ClassResult<&mutf8::mstr> {
        self.class_name(self.this_class)
    }

    pub fn super_class(&self) -> ClassResult<&mutf8::mstr> {
        if self.super_class == 0 {
            Err(ClassError::NoSuper)
        } else {
            self.class_name(self.super_class)
        }
    }

    pub fn interfaces(&self) -> impl Iterator<Item = ClassResult<&mutf8::mstr>> {
        self.interfaces.iter().map(move |idx| self.class_name(*idx))
    }

    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    pub fn fields(&self) -> impl ExactSizeIterator<Item = &FieldInfo> {
        self.fields.iter()
    }

    pub fn methods(&self) -> impl ExactSizeIterator<Item = &MethodInfo> {
        self.methods.iter()
    }
    pub fn attribute<A: Attribute>(&self) -> ClassResult<A> {
        let attr_name = A::NAME.to_mstr();
        let bytes = self
            .attributes
            .iter()
            .find(|a| a.name == attr_name.as_ref())
            .map(|attr| attr.info).ok_or(ClassError::Attribute(A::NAME))?;

        A::parse(bytes, &self.constant_pool)
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn access_flags(&self) -> ClassAccessFlags {
        self.access_flags
    }
}
