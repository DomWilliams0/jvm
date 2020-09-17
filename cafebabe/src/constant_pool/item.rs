use crate::buffer::Buffer;

use crate::{ClassError, ClassResult};
use log::*;
use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Tag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

#[derive(Debug)]
pub enum Item<'c> {
    MethodRef {
        class: Index,
        name_and_type: Index,
    },
    FieldRef {
        class: Index,
        name_and_type: Index,
    },
    InterfaceMethodRef {
        class: Index,
        name_and_type: Index,
    },
    Class {
        name: Index,
    },
    String {
        string: Index,
    },
    Integer {
        int: u32,
    },
    Float {
        float: f32,
    },
    Long {
        long: u64,
    },
    Double {
        double: f64,
    },
    NameAndType {
        name: Index,
        descriptor: Index,
    },
    Utf8(&'c mutf8::mstr),
    MethodHandle {
        reference_kind: u8,
        reference: Index,
    },
    MethodType {
        descriptor: Index,
    },
    Dynamic {
        bootstrap_method_attr: Index,
        name_and_type: Index,
    },
    InvokeDynamic {
        bootstrap_method_attr: Index,
        name_and_type: Index,
    },
    Module {
        name: Index,
    },
    Package {
        name: Index,
    },
}

/// Starts at 1
pub type Index = u16;

impl<'c> Item<'c> {
    // TODO handle specific versions tags were introduced
    pub fn load(buf: &mut Buffer<'c>) -> ClassResult<Self> {
        let tag =
            Tag::try_from_primitive(buf.read::<u8>()?).map_err(|e| ClassError::CpTag(e.number))?;
        trace!("parsing item {:?}", tag);

        Ok(match tag {
            Tag::MethodRef => {
                let class = buf.read()?;
                let name_and_type = buf.read()?;
                Item::MethodRef {
                    class,
                    name_and_type,
                }
            }

            Tag::FieldRef => {
                let class = buf.read()?;
                let name_and_type = buf.read()?;
                Item::FieldRef {
                    class,
                    name_and_type,
                }
            }

            Tag::InterfaceMethodRef => {
                let class = buf.read()?;
                let name_and_type = buf.read()?;
                Item::InterfaceMethodRef {
                    class,
                    name_and_type,
                }
            }

            Tag::Class => {
                let name = buf.read()?;
                Item::Class { name }
            }

            Tag::String => {
                let string = buf.read()?;
                Item::String { string }
            }

            Tag::Integer => {
                let int = buf.read()?;
                Item::Integer { int }
            }

            Tag::Float => {
                let float = buf.read()?;
                // TODO float might need extra parsing
                Item::Float { float }
            }

            Tag::Long => {
                let high = buf.read::<u32>()?;
                let low = buf.read::<u32>()?;
                Item::Long {
                    long: ((high as u64) << 32) + low as u64,
                }
            }

            Tag::Double => {
                let double = buf.read()?;
                // TODO double might need extra parsing
                Item::Double { double }
            }

            Tag::NameAndType => {
                let name = buf.read()?;
                let descriptor = buf.read()?;
                Item::NameAndType { name, descriptor }
            }

            Tag::Utf8 => {
                let length = buf.read::<u16>()?;
                let bytes = buf.read_slice(length as usize)?;
                Item::Utf8(mutf8::mstr::from_mutf8(bytes))
            }

            Tag::MethodHandle => {
                let reference_kind = buf.read()?;
                let reference = buf.read()?;
                Item::MethodHandle {
                    reference_kind,
                    reference,
                }
            }

            Tag::MethodType => {
                let descriptor = buf.read()?;
                Item::MethodType { descriptor }
            }

            Tag::Dynamic => {
                let bootstrap_method_attr = buf.read()?;
                let name_and_type = buf.read()?;
                Item::Dynamic {
                    bootstrap_method_attr,
                    name_and_type,
                }
            }

            Tag::InvokeDynamic => {
                let bootstrap_method_attr = buf.read()?;
                let name_and_type = buf.read()?;
                Item::InvokeDynamic {
                    bootstrap_method_attr,
                    name_and_type,
                }
            }

            Tag::Module => {
                let name = buf.read()?;
                Item::Module { name }
            }

            Tag::Package => {
                let name = buf.read()?;
                Item::Package { name }
            }
        })
    }

    pub fn is_wide(&self) -> bool {
        matches!(self,
            Item::Long { .. } |
            Item::Double { .. })
    }

    // TODO is_loadable()
    pub fn tag(&self) -> Tag {
        match self {
            Item::MethodRef { .. } => Tag::MethodRef,
            Item::FieldRef { .. } => Tag::FieldRef,
            Item::InterfaceMethodRef { .. } => Tag::InterfaceMethodRef,
            Item::Class { .. } => Tag::Class,
            Item::String { .. } => Tag::String,
            Item::Integer { .. } => Tag::Integer,
            Item::Float { .. } => Tag::Float,
            Item::Long { .. } => Tag::Long,
            Item::Double { .. } => Tag::Double,
            Item::NameAndType { .. } => Tag::NameAndType,
            Item::Utf8(_) => Tag::Utf8,
            Item::MethodHandle { .. } => Tag::MethodHandle,
            Item::MethodType { .. } => Tag::MethodType,
            Item::Dynamic { .. } => Tag::Dynamic,
            Item::InvokeDynamic { .. } => Tag::InvokeDynamic,
            Item::Module { .. } => Tag::Module,
            Item::Package { .. } => Tag::Package,
        }
    }
}
