use crate::constant_pool::item::{ClassRefItem, MethodRefItem, NameAndTypeItem, Tag, Utf8Item};
use crate::{ClassError, ClassResult, ConstantPool, Item};

pub trait Entry<'c>: Sized {
    const TAG: Tag;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self>;
}

impl<'c> Entry<'c> for Utf8Item<'c> {
    const TAG: Tag = Tag::Utf8;

    fn from_item(item: &Item<'c>, _: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::Utf8(s) => Ok(Utf8Item { string: *s }),
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}

impl<'c> Entry<'c> for ClassRefItem<'c> {
    const TAG: Tag = Tag::Class;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::Class { name } => {
                let name = pool.string_entry(*name)?;
                Ok(ClassRefItem { name })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
impl<'c> Entry<'c> for NameAndTypeItem<'c> {
    const TAG: Tag = Tag::NameAndType;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::NameAndType { name, descriptor } => {
                let name = pool.string_entry(*name)?;
                let desc = pool.string_entry(*descriptor)?;
                Ok(NameAndTypeItem { name, desc })
            }
            _ => Err(ClassError::WrongTag {
                expected: Self::TAG,
                actual: item.tag(),
            }),
        }
    }
}
impl<'c> Entry<'c> for MethodRefItem<'c> {
    const TAG: Tag = Tag::MethodRef;

    fn from_item(item: &Item<'c>, pool: &ConstantPool<'c>) -> ClassResult<Self> {
        match item {
            Item::MethodRef {
                class,
                name_and_type,
            } => {
                let class: ClassRefItem = pool.entry(*class)?;
                let name_and_type: NameAndTypeItem = pool.entry(*name_and_type)?;
                Ok(MethodRefItem {
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
