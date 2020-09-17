use crate::{ClassError, ClassResult, ConstantPool};
use crate::constant_pool::item::Item;
use crate::constant_pool::Tag;

pub trait Entry<'c>: Sized {
    const TAG: Tag;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self>;
}

#[derive(Debug)]
pub struct Utf8Entry<'c> {
    pub string: &'c mutf8::mstr,
}

#[derive(Debug)]
pub struct ClassRefEntry<'c> {
    pub name: &'c mutf8::mstr,
}

#[derive(Debug)]
struct NameAndTypeEntry<'c> {
    pub name: &'c mutf8::mstr,
    pub desc: &'c mutf8::mstr,
}

#[derive(Debug)]
pub struct MethodRefEntry<'c> {
    pub class: &'c mutf8::mstr,
    pub name: &'c mutf8::mstr,
    pub desc: &'c mutf8::mstr,
}

#[derive(Debug)]
pub struct FieldRefEntry<'c> {
    pub class: &'c mutf8::mstr,
    pub name: &'c mutf8::mstr,
    // TODO parse desc to DataType here in FieldRefEntry
    pub desc: &'c mutf8::mstr,
}

#[derive(Debug)]
pub struct InterfaceMethodRefEntry<'c> {
    pub class: &'c mutf8::mstr,
    pub name: &'c mutf8::mstr,
    pub desc: &'c mutf8::mstr,
}


impl<'c> Entry<'c> for Utf8Entry<'c> {
    const TAG: Tag = Tag::Utf8;

    fn from_item(item: &Item<'c>, _: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::Utf8(s) => Ok(Utf8Entry { string: *s }),
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}

impl<'c> Entry<'c> for ClassRefEntry<'c> {
    const TAG: Tag = Tag::Class;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::Class { name } => {
                let name = pool.string_entry(*name)?;
                Ok(ClassRefEntry { name })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
impl<'c> Entry<'c> for NameAndTypeEntry<'c> {
    const TAG: Tag = Tag::NameAndType;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::NameAndType { name, descriptor } => {
                let name = pool.string_entry(*name)?;
                let desc = pool.string_entry(*descriptor)?;
                Ok(NameAndTypeEntry { name, desc })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
impl<'c> Entry<'c> for MethodRefEntry<'c> {
    const TAG: Tag = Tag::MethodRef;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::MethodRef {
                class,
                name_and_type,
            } => {
                let class: ClassRefEntry = pool.entry(*class)?;
                let name_and_type: NameAndTypeEntry = pool.entry(*name_and_type)?;
                Ok(MethodRefEntry {
                    class: class.name,
                    name: name_and_type.name,
                    desc: name_and_type.desc,
                })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
impl<'c> Entry<'c> for FieldRefEntry<'c> {
    const TAG: Tag = Tag::FieldRef;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::FieldRef {
                class, name_and_type
            } => {
                let class: ClassRefEntry = pool.entry(*class)?;
                let name_and_type: NameAndTypeEntry = pool.entry(*name_and_type)?;
                Ok(FieldRefEntry {
                    class: class.name,
                    name: name_and_type.name,
                    desc: name_and_type.desc,
                })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
