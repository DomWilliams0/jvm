use crate::class::NULL;

use crate::alloc::{NativeString, VmRef};
use crate::class::Object;
use cafebabe::mutf8::mstr;
use itertools::Itertools;
use std::convert::{TryFrom, TryInto};

// TODO more efficient packing of data types, dont want huge enum discriminant taking up all the space
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataType {
    Boolean,
    ReturnAddress,
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    /// class types, array types, and interface types
    Reference(ReferenceDataType),
}
// TODO interned strings for class names?
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ReferenceDataType {
    Class(NativeString),
    Array { dims: u8, elements: Box<DataType> },
}
#[derive(Debug, Clone)]
pub enum DataValue {
    Boolean(bool),
    ReturnAddress(usize),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Char(u16),
    Float(f32),
    Double(f64),
    /// class types, array types, and interface types
    Reference(ReferenceDataType, VmRef<Object>),
}

impl DataType {
    pub fn default_value(self) -> DataValue {
        match self {
            DataType::Boolean => DataValue::Boolean(false),
            DataType::ReturnAddress => DataValue::ReturnAddress(0),
            DataType::Byte => DataValue::Byte(0),
            DataType::Short => DataValue::Short(0),
            DataType::Int => DataValue::Int(0),
            DataType::Long => DataValue::Long(0),
            DataType::Char => DataValue::Char(0),
            DataType::Float => DataValue::Float(0.0),
            DataType::Double => DataValue::Double(0.0),
            DataType::Reference(reftype) => DataValue::Reference(reftype, NULL.clone()),
        }
    }

    pub fn from_descriptor(desc: &mstr) -> Option<Self> {
        let desc = desc.to_utf8(); // not likely to require an allocation
        Some(match desc.as_ref() {
            "B" => Self::Byte,
            "C" => Self::Char,
            "D" => Self::Double,
            "F" => Self::Float,
            "I" => Self::Int,
            "J" => Self::Long,
            "S" => Self::Short,
            "Z" => Self::Boolean,
            s if s.starts_with('L') => {
                let mut chars = s.chars().skip(1);
                let cls = chars.take_while_ref(|c| *c != ';').collect::<String>();

                // cant be empty
                if cls.is_empty() {
                    return None;
                }

                // semicolon necessary at the end
                {
                    let semicolon = chars.next();
                    let end = chars.next();
                    if (semicolon, end) != (Some(';'), None) {
                        return None;
                    }
                }

                // TODO MString method from owned utf8 to avoid this copy
                let class_name = NativeString::from_utf8(cls.as_bytes());
                Self::Reference(ReferenceDataType::Class(class_name))
            }
            s if s.starts_with('[') => {
                let mut chars = s.chars();
                let dims = chars.take_while_ref(|c| *c == '[').count();

                // limit to 255
                let dims = u8::try_from(dims).ok()?;

                // recurse
                // TODO avoid extra allocation here too
                let element_type = chars.collect::<String>().into_bytes();
                let element_type = DataType::from_descriptor(mstr::from_mutf8(&element_type))?;
                Self::Reference(ReferenceDataType::Array {
                    dims,
                    elements: Box::new(element_type),
                })
            }
            _ => {
                return None;
            }
        })
    }
}

impl DataValue {
    pub fn is_wide(&self) -> bool {
        match self {
            DataValue::Long(_) | DataValue::Double(_) => true,
            _ => false,
        }
    }

    /// Must be non null
    pub fn reference(reference: VmRef<Object>) -> Self {
        let reference_data = ReferenceDataType::Class(
            reference
                .class()
                .expect("should be non null")
                .name()
                .to_owned(),
        );
        DataValue::Reference(reference_data, reference)
    }
}

macro_rules! impl_data_value_type {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for DataValue {
            fn from(v: $ty) -> Self {
                Self::$variant(v)
            }
        }

        impl TryInto<$ty> for DataValue {
            type Error = ();

            fn try_into(self) -> Result<$ty, Self::Error> {
                if let Self::$variant(v) = self {
                    Ok(v)
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_data_value_type!(bool, Boolean);
impl_data_value_type!(i8, Byte);
impl_data_value_type!(i16, Short);
impl_data_value_type!(i32, Int);
impl_data_value_type!(i64, Long);
impl_data_value_type!(u16, Char);
impl_data_value_type!(f32, Float);
impl_data_value_type!(f64, Double);

#[cfg(test)]
mod tests {
    use crate::types::{DataType, ReferenceDataType};
    use cafebabe::mutf8::{mstr, MString};

    fn check(input: &str, expected: Option<DataType>) {
        let mstr = mstr::from_utf8(input.as_bytes());
        assert_eq!(DataType::from_descriptor(mstr.as_ref()), expected)
    }

    fn check_ref(input: &str, expected: ReferenceDataType) {
        let mstr = mstr::from_utf8(input.as_bytes());
        assert_eq!(
            DataType::from_descriptor(mstr.as_ref()),
            Some(DataType::Reference(expected))
        )
    }

    #[test]
    fn primitive() {
        check("", None);
        check("B", Some(DataType::Byte));
        check("B!", None);
        check("S", Some(DataType::Short));
        check("S ", None);
    }

    #[test]
    fn class() {
        check("L", None);
        check("L;", None);
        check_ref(
            "Ljava/lang/Woopdedoo;",
            ReferenceDataType::Class(MString::from_utf8(b"java/lang/Woopdedoo")),
        );
        check("Lwoop;nah", None);
    }

    #[test]
    fn array() {
        check("[", None);
        check_ref(
            "[I",
            ReferenceDataType::Array {
                dims: 1,
                elements: Box::new(DataType::Int),
            },
        );
        check_ref(
            "[[[I",
            ReferenceDataType::Array {
                dims: 3,
                elements: Box::new(DataType::Int),
            },
        );
        check("[[[I;", None);

        check_ref(
            "[[Ljava/lang/Object;",
            ReferenceDataType::Array {
                dims: 2,
                elements: Box::new(DataType::Reference(ReferenceDataType::Class(
                    MString::from_utf8(b"java/lang/Object"),
                ))),
            },
        );
    }
}
