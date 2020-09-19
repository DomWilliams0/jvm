use crate::class::null;

use crate::alloc::{NativeString, VmRef};
use crate::class::Object;
use cafebabe::mutf8::mstr;

use std::convert::{TryFrom, TryInto};

// TODO more efficient packing of data types, dont want huge enum discriminant taking up all the space
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataType {
    Primitive(PrimitiveDataType),
    ReturnAddress,
    /// class types, array types, and interface types
    Reference(ReferenceDataType),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PrimitiveDataType {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
}

// TODO interned strings for class names?
// TODO gross that we always need an allocation for reference type - Cow/vmref<class> for class and store array dim inline?
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ReferenceDataType {
    Class(NativeString),
    Array { dims: u8, elem_type: Box<DataType> },
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

#[derive(Debug, Eq, PartialEq)]
pub enum ArrayType<'a> {
    Primitive(PrimitiveDataType),
    Reference(&'a mstr),
}

pub struct MethodSignature<'a> {
    descriptor: &'a [u8],
    errored: bool,
    ret: ReturnType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ReturnType {
    Void,
    Returns(DataType),
}

enum SignatureState {
    Start,
    Args,
    Ret,
}

pub struct MethodSignatureIter<'a, 'b> {
    sig: &'b mut MethodSignature<'a>,
    state: SignatureState,
    cursor: usize,
}

impl DataType {
    pub fn default_value(self) -> DataValue {
        match self {
            DataType::Primitive(prim) => prim.default_value(),
            DataType::ReturnAddress => DataValue::ReturnAddress(0),
            DataType::Reference(reftype) => DataValue::Reference(reftype, null()),
        }
    }
    pub fn from_descriptor(desc: &mstr) -> Option<Self> {
        Self::from_descriptor_stream(desc.as_bytes()).and_then(|(data, cursor)| {
            if cursor == desc.len() {
                Some(data)
            } else {
                None
            }
        })
    }

    fn from_descriptor_stream(desc: &[u8]) -> Option<(Self, usize)> {
        // collect array dimensions
        let array_dims = desc.iter().position(|b| *b != b'[')?;

        let desc = &desc[array_dims..];

        let first = *desc.get(0)?;
        let (datatype, cursor) = if let Some(prim) = PrimitiveDataType::from_char(first) {
            (Self::Primitive(prim), 1)
        } else if first == b'L' {
            let semicolon = desc.iter().position(|b| *b == b';')?;
            let name = &desc[1..semicolon];
            if name.is_empty() {
                return None;
            }

            (
                Self::Reference(ReferenceDataType::Class(NativeString::from_mutf8(name))),
                semicolon + 1,
            )
        } else {
            return None;
        };

        let datatype = if array_dims == 0 {
            datatype
        } else {
            // limit to 255
            let array_dims = u8::try_from(array_dims).ok()?;

            Self::Reference(ReferenceDataType::Array {
                dims: array_dims as u8,
                elem_type: Box::new(datatype),
            })
        };

        Some((datatype, cursor + array_dims))
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, DataType::Primitive(_))
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

    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::Boolean(_) => DataType::Primitive(PrimitiveDataType::Boolean),
            DataValue::Byte(_) => DataType::Primitive(PrimitiveDataType::Byte),
            DataValue::Short(_) => DataType::Primitive(PrimitiveDataType::Short),
            DataValue::Int(_) => DataType::Primitive(PrimitiveDataType::Int),
            DataValue::Long(_) => DataType::Primitive(PrimitiveDataType::Long),
            DataValue::Char(_) => DataType::Primitive(PrimitiveDataType::Char),
            DataValue::Float(_) => DataType::Primitive(PrimitiveDataType::Float),
            DataValue::Double(_) => DataType::Primitive(PrimitiveDataType::Double),
            DataValue::ReturnAddress(_) => DataType::ReturnAddress,
            DataValue::Reference(ty, _) => DataType::Reference(ty.clone()),
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            DataValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_reference_array(&self) -> Option<VmRef<Object>> {
        match self {
            DataValue::Reference(ReferenceDataType::Array { .. }, obj) => Some(obj.clone()),
            _ => None,
        }
    }

    pub fn as_reference_nonarray(&self) -> Option<VmRef<Object>> {
        match self {
            DataValue::Reference(ReferenceDataType::Class(_), obj) => Some(obj.clone()),
            _ => None,
        }
    }

    pub fn as_reference(&self) -> Option<VmRef<Object>> {
        match self {
            DataValue::Reference(_, obj) => Some(obj.clone()),
            _ => None,
        }
    }

    pub fn is_reference_or_retaddr(&self) -> bool {
        matches!(self, DataValue::Reference(_, _) | DataValue::ReturnAddress(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, DataValue::Reference(_, _))
    }
}

impl PrimitiveDataType {
    pub const TYPES: [(PrimitiveDataType, &'static str); 8] = [
        (PrimitiveDataType::Boolean, "boolean"),
        (PrimitiveDataType::Byte, "byte"),
        (PrimitiveDataType::Short, "short"),
        (PrimitiveDataType::Int, "int"),
        (PrimitiveDataType::Long, "long"),
        (PrimitiveDataType::Char, "char"),
        (PrimitiveDataType::Float, "float"),
        (PrimitiveDataType::Double, "double"),
    ];

    pub fn from_char(b: u8) -> Option<Self> {
        Some(match b {
            b'B' => Self::Byte,
            b'C' => Self::Char,
            b'D' => Self::Double,
            b'F' => Self::Float,
            b'I' => Self::Int,
            b'J' => Self::Long,
            b'S' => Self::Short,
            b'Z' => Self::Boolean,
            _ => return None,
        })
    }

    pub fn from_descriptor(str: &[u8]) -> Option<Self> {
        if str.len() != 1 {
            None
        } else {
            let b = unsafe { *str.get_unchecked(0) };
            Self::from_char(b)
        }
    }

    pub fn default_value(&self) -> DataValue {
        match self {
            PrimitiveDataType::Boolean => DataValue::Boolean(false),
            PrimitiveDataType::Byte => DataValue::Byte(0),
            PrimitiveDataType::Short => DataValue::Short(0),
            PrimitiveDataType::Int => DataValue::Int(0),
            PrimitiveDataType::Long => DataValue::Long(0),
            PrimitiveDataType::Char => DataValue::Char(0),
            PrimitiveDataType::Float => DataValue::Float(0.0),
            PrimitiveDataType::Double => DataValue::Double(0.0),
        }
    }

    pub fn char(&self) -> char {
        match self {
            PrimitiveDataType::Boolean => 'Z',
            PrimitiveDataType::Byte => 'B',
            PrimitiveDataType::Short => 'S',
            PrimitiveDataType::Int => 'I',
            PrimitiveDataType::Long => 'J',
            PrimitiveDataType::Char => 'C',
            PrimitiveDataType::Float => 'F',
            PrimitiveDataType::Double => 'D',
        }
    }
}

impl<'a> ArrayType<'a> {
    pub fn from_descriptor(str: &'a mstr) -> Option<Self> {
        // let s = str.to_utf8();
        let bytes = str.as_bytes();

        // find where array element starts
        let idx = bytes.iter().position(|b| *b != b'[')?;
        if idx == 0 {
            return None;
        };

        let first_char = *bytes.get(idx)?;
        if first_char == b'L' {
            if !matches!(bytes.last(), Some(b';')) {
                return None;
            }

            let ref_name = bytes.get(idx + 1..bytes.len() - 1).and_then(|b| {
                if b.is_empty() {
                    None
                } else {
                    Some(b)
                }
            })?;

            return Some(ArrayType::Reference(mstr::from_mutf8(ref_name)));
        }

        PrimitiveDataType::from_descriptor(&bytes[idx..]).map(ArrayType::Primitive)
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

impl<'a> MethodSignature<'a> {
    pub fn from_descriptor(descriptor: &'a mstr) -> Self {
        MethodSignature {
            descriptor: descriptor.as_bytes(),
            errored: true,
            ret: ReturnType::Void,
        }
    }

    pub fn errored(&self) -> bool {
        self.errored
    }

    pub fn return_type(&mut self) -> ReturnType {
        std::mem::replace(&mut self.ret, ReturnType::Void)
    }

    pub fn iter_args(&mut self) -> MethodSignatureIter<'a, '_> {
        MethodSignatureIter {
            sig: self,
            cursor: 0,
            state: SignatureState::Start,
        }
    }
}

impl<'a, 'b> Iterator for MethodSignatureIter<'a, 'b> {
    type Item = DataType;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                SignatureState::Start => {
                    let b = self.pop_byte()?;
                    if b == b'(' {
                        self.state = SignatureState::Args;
                        continue;
                    } else {
                        return None;
                    }
                }
                SignatureState::Args => {
                    let b = self.peek_byte()?;
                    if b == b')' {
                        self.state = SignatureState::Ret;
                        self.advance();
                        continue;
                    }

                    return DataType::from_descriptor_stream(&self.sig.descriptor[self.cursor..])
                        .map(|(arg, new_cursor)| {
                            self.cursor += new_cursor;
                            arg
                        });
                }
                SignatureState::Ret => {
                    let ret = {
                        let b = self.peek_byte()?;
                        if b == b'V' {
                            self.advance();
                            ReturnType::Void
                        } else {
                            DataType::from_descriptor_stream(&self.sig.descriptor[self.cursor..])
                                .map(|(ret, new_cursor)| {
                                    self.cursor += new_cursor;
                                    ReturnType::Returns(ret)
                                })?
                        }
                    };

                    self.sig.ret = ret;
                    if self.cursor == self.sig.descriptor.len() {
                        // consumed whole string, success
                        self.sig.errored = false;
                    }

                    // clean finish
                    return None;
                }
            }
        }
    }
}

impl<'a, 'b> MethodSignatureIter<'a, 'b> {
    fn peek_byte(&mut self) -> Option<u8> {
        self.sig.descriptor.get(self.cursor).copied()
    }
    fn pop_byte(&mut self) -> Option<u8> {
        self.peek_byte().map(|b| {
            self.advance();
            b
        })
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        ArrayType, DataType, MethodSignature, PrimitiveDataType, ReferenceDataType, ReturnType,
    };
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

    fn check_array(input: &str, expected: Option<ArrayType>) {
        let mstr = mstr::from_utf8(input.as_bytes());
        assert_eq!(ArrayType::from_descriptor(mstr.as_ref()), expected)
    }

    fn check_method(input: &str, expected: Option<(Vec<DataType>, ReturnType)>) {
        let mstr = mstr::from_utf8(input.as_bytes());
        let mut sig = MethodSignature::from_descriptor(mstr.as_ref());
        let types: Vec<_> = sig.iter_args().collect();

        match expected {
            None => assert!(sig.errored()),
            Some((expected_args, expected_ret)) => {
                assert!(!sig.errored());
                assert_eq!(types, expected_args);
                assert_eq!(sig.return_type(), expected_ret);
            }
        }
    }

    #[test]
    fn primitive() {
        check("", None);
        check("B", Some(DataType::Primitive(PrimitiveDataType::Byte)));
        check("B!", None);
        check("S", Some(DataType::Primitive(PrimitiveDataType::Short)));
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
                elem_type: Box::new(DataType::Primitive(PrimitiveDataType::Int)),
            },
        );
        check_ref(
            "[[[I",
            ReferenceDataType::Array {
                dims: 3,
                elem_type: Box::new(DataType::Primitive(PrimitiveDataType::Int)),
            },
        );
        check("[[[I;", None);

        check_ref(
            "[[Ljava/lang/Object;",
            ReferenceDataType::Array {
                dims: 2,
                elem_type: Box::new(DataType::Reference(ReferenceDataType::Class(
                    MString::from_utf8(b"java/lang/Object"),
                ))),
            },
        );
    }

    #[test]
    fn array_type() {
        check_array("", None);
        check_array("I", None);

        check_array("[I", Some(ArrayType::Primitive(PrimitiveDataType::Int)));
        check_array("[[[[I", Some(ArrayType::Primitive(PrimitiveDataType::Int)));
        check_array("[[[[I.", None);

        check_array("[nothing", None);
        check_array("[Lcool", None);
        check_array("[L;", None);
        check_array(
            "[Lcool;",
            Some(ArrayType::Reference(mstr::from_utf8(b"cool").as_ref())),
        );
    }

    #[test]
    fn method_descriptor() {
        check_method("boo", None);
        check_method("()V", Some((vec![], ReturnType::Void)));
        check_method(
            "()I",
            Some((
                vec![],
                ReturnType::Returns(DataType::Primitive(PrimitiveDataType::Int)),
            )),
        );
        check_method(
            "()Lnice;",
            Some((
                vec![],
                ReturnType::Returns(DataType::Reference(ReferenceDataType::Class(
                    MString::from_utf8(b"nice"),
                ))),
            )),
        );
        check_method("()asdf", None);

        check_method(
            "(I[[D)V",
            Some((
                vec![
                    DataType::Primitive(PrimitiveDataType::Int),
                    DataType::Reference(ReferenceDataType::Array {
                        dims: 2,
                        elem_type: Box::new(DataType::Primitive(PrimitiveDataType::Double)),
                    }),
                ],
                ReturnType::Void,
            )),
        );
    }
}
