use crate::class::{null, Class};

use crate::alloc::{vmref_eq, VmRef};
use crate::class::Object;
use cafebabe::mutf8::{mstr, StrExt};

use crate::thread;
use num_enum::TryFromPrimitive;
use std::borrow::Cow;
use std::convert::TryInto;
use std::str::FromStr;

// TODO more efficient packing of data values
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataType<'a> {
    Primitive(PrimitiveDataType),
    ReturnAddress,
    /// Class name for class types, array types, and interface types.
    Reference(Cow<'a, mstr>),
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
    Reference(VmRef<Object>),

    /// java/lang/Class.vmdata
    VmDataClass(VmRef<Class>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArrayType<'a> {
    Primitive(PrimitiveDataType),
    Reference(&'a mstr),
}

pub struct MethodSignature<'a> {
    descriptor: &'a [u8],
    errored: bool,
    ret: ReturnType<'a>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ReturnType<'a> {
    Void,
    Returns(DataType<'a>),
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

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum NewarrayType {
    Boolean = 4,
    Char = 5,
    Float = 6,
    Double = 7,
    Byte = 8,
    Short = 9,
    Int = 10,
    Long = 11,
}

impl<'a> DataType<'a> {
    pub fn default_value(self) -> DataValue {
        match self {
            DataType::Primitive(prim) => prim.default_value(),
            DataType::ReturnAddress => DataValue::ReturnAddress(0),
            DataType::Reference(_) => DataValue::Reference(null()),
        }
    }
    pub fn from_descriptor(desc: &'a mstr) -> Option<Self> {
        Self::from_descriptor_stream(desc.as_bytes()).and_then(|(data, cursor)| {
            let len = desc.len();
            if cursor == len {
                Some(data)
            } else {
                None
            }
        })
    }

    fn from_descriptor_stream(desc: &'a [u8]) -> Option<(Self, usize)> {
        // collect array dimensions
        let array_dims = desc.iter().position(|b| *b != b'[')?;

        match array_dims {
            0 => {
                // not an array
            }
            1..=255 => {
                // valid array dimensions
                // parse element type but dont store
                let (_, idx) = Self::from_descriptor_stream(&desc[array_dims..])?;

                let elem_type = mstr::from_mutf8(&desc[..array_dims + idx]);
                return Some((Self::Reference(Cow::Borrowed(elem_type)), array_dims + idx));
            }
            _ => {
                // invalid array dims
                return None;
            }
        }
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
                Self::Reference(Cow::Borrowed(mstr::from_mutf8(name))),
                semicolon + 1,
            )
        } else {
            return None;
        };

        Some((datatype, cursor + array_dims))
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, DataType::Primitive(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, DataType::Reference(_))
    }

    pub fn to_owned(&self) -> DataType<'static> {
        match self {
            DataType::Reference(r) => {
                let str = r.clone();
                DataType::Reference(Cow::Owned(str.into_owned()))
            }
            DataType::Primitive(p) => DataType::Primitive(*p),
            DataType::ReturnAddress => DataType::ReturnAddress,
        }
    }
}

impl DataValue {
    pub fn is_wide(&self) -> bool {
        match self {
            DataValue::Long(_) | DataValue::Double(_) => true,
            _ => false,
        }
    }

    /// Panics if null
    pub fn data_type(&self) -> DataType<'static> {
        self.data_type_checked().expect("null")
    }

    /// None if null
    pub fn data_type_checked(&self) -> Option<DataType<'static>> {
        Some(match self {
            DataValue::Boolean(_) => DataType::Primitive(PrimitiveDataType::Boolean),
            DataValue::Byte(_) => DataType::Primitive(PrimitiveDataType::Byte),
            DataValue::Short(_) => DataType::Primitive(PrimitiveDataType::Short),
            DataValue::Int(_) => DataType::Primitive(PrimitiveDataType::Int),
            DataValue::Long(_) => DataType::Primitive(PrimitiveDataType::Long),
            DataValue::Char(_) => DataType::Primitive(PrimitiveDataType::Char),
            DataValue::Float(_) => DataType::Primitive(PrimitiveDataType::Float),
            DataValue::Double(_) => DataType::Primitive(PrimitiveDataType::Double),
            DataValue::ReturnAddress(_) => DataType::ReturnAddress,
            DataValue::Reference(o) => {
                let cls = o.class()?;
                DataType::Reference(Cow::Owned(cls.name().to_owned()))
            }
            DataValue::VmDataClass(_) => {
                DataType::Reference(Cow::Borrowed("java/lang/Class".as_mstr()))
            }
        })
    }

    pub fn as_int(&self) -> Option<i32> {
        self.assign_to(&DataType::Primitive(PrimitiveDataType::Int))
            .map(|val| match *val {
                DataValue::Int(i) => i,
                _ => unreachable!(),
            })
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            DataValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            DataValue::Double(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        match self {
            DataValue::Long(l) => Some(*l),
            _ => None,
        }
    }

    pub fn as_reference(&self) -> Option<&VmRef<Object>> {
        match self {
            DataValue::Reference(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn into_reference(self) -> Result<VmRef<Object>, DataType<'static>> {
        match self {
            DataValue::Reference(obj) => Ok(obj),
            v => Err(v.data_type()),
        }
    }

    pub fn is_reference_or_retaddr(&self) -> bool {
        matches!(self, DataValue::Reference(_) | DataValue::ReturnAddress(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, DataValue::Reference(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self, DataValue::Int(_) | DataValue::Boolean(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataValue::Float(_))
    }

    pub fn is_long(&self) -> bool {
        matches!(self, DataValue::Long(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, DataValue::Double(_))
    }

    pub fn is_short(&self) -> bool {
        matches!(self, DataValue::Short(_))
    }

    pub fn is_byte(&self) -> bool {
        matches!(self, DataValue::Byte(_))
    }

    pub fn is_char(&self) -> bool {
        matches!(self, DataValue::Char(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, DataValue::Boolean(_))
    }

    /// For putfield/putstatic. Self is type of value being assigned to field
    /// https://docs.oracle.com/javase/specs/jls/se11/html/jls-5.html#jls-5.2
    pub fn assign_to(&self, field_type: &DataType) -> Option<Cow<DataValue>> {
        let my_type = self.data_type();

        // identity conversion
        if field_type == &my_type {
            return Some(Cow::Borrowed(self));
        }

        let result = match (field_type, my_type) {
            // primitives
            (DataType::Primitive(field_prim), DataType::Primitive(_)) => self
                .widen_primitive_to(*field_prim)
                .or_else(|| self.narrow_primitive_to(*field_prim)),

            // reference types
            (DataType::Reference(cls_tgt), DataType::Reference(_)) => {
                let self_obj = self.as_reference().unwrap(); // just checked

                // allow cast to superclass
                return if let Some(self_cls) = self_obj.class() {
                    // resolve dst class object
                    thread::get()
                        .global()
                        .class_loader()
                        .load_class(&cls_tgt, self_cls.loader().clone())
                        .ok()
                        .and_then(|cls_tgt| {
                            if self_cls.is_instance_of(&cls_tgt) {
                                Some(Cow::Borrowed(self))
                            } else {
                                None
                            }
                        })
                } else {
                    // this is null, just return null back again?
                    Some(Cow::Borrowed(self))
                };
            }
            _ => return None,
        };

        result.map(Cow::Owned)
    }

    pub fn widen_primitive_to(&self, target: PrimitiveDataType) -> Option<DataValue> {
        Some(match (target, self) {
            (PrimitiveDataType::Short, DataValue::Byte(val)) => DataValue::from(*val as i16),

            (PrimitiveDataType::Int, DataValue::Byte(val)) => DataValue::from(*val as i32),
            (PrimitiveDataType::Int, DataValue::Short(val)) => DataValue::from(*val as i32),
            (PrimitiveDataType::Int, DataValue::Char(val)) => DataValue::from(*val as i32),

            (PrimitiveDataType::Long, DataValue::Byte(val)) => DataValue::from(*val as i64),
            (PrimitiveDataType::Long, DataValue::Short(val)) => DataValue::from(*val as i64),
            (PrimitiveDataType::Long, DataValue::Char(val)) => DataValue::from(*val as i64),
            (PrimitiveDataType::Long, DataValue::Int(val)) => DataValue::from(*val as i64),

            (PrimitiveDataType::Float, DataValue::Byte(val)) => DataValue::from(*val as f32),
            (PrimitiveDataType::Float, DataValue::Short(val)) => DataValue::from(*val as f32),
            (PrimitiveDataType::Float, DataValue::Char(val)) => DataValue::from(*val as f32),
            (PrimitiveDataType::Float, DataValue::Int(val)) => DataValue::from(*val as f32),
            (PrimitiveDataType::Float, DataValue::Long(val)) => DataValue::from(*val as f32),

            (PrimitiveDataType::Double, DataValue::Byte(val)) => DataValue::from(*val as f64),
            (PrimitiveDataType::Double, DataValue::Short(val)) => DataValue::from(*val as f64),
            (PrimitiveDataType::Double, DataValue::Char(val)) => DataValue::from(*val as f64),
            (PrimitiveDataType::Double, DataValue::Int(val)) => DataValue::from(*val as f64),
            (PrimitiveDataType::Double, DataValue::Long(val)) => DataValue::from(*val as f64),
            (PrimitiveDataType::Double, DataValue::Float(val)) => DataValue::from(*val as f64),

            // TODO does boolean conversions count as widening
            (PrimitiveDataType::Int, DataValue::Boolean(val)) => DataValue::Int(*val as i32),

            _ => return None,
        })
    }

    pub fn narrow_primitive_to(&self, target: PrimitiveDataType) -> Option<DataValue> {
        Some(match (target, self) {
            (PrimitiveDataType::Byte, DataValue::Short(val)) => DataValue::from(*val as i8),
            (PrimitiveDataType::Byte, DataValue::Char(val)) => DataValue::from(*val as i8),
            (PrimitiveDataType::Byte, DataValue::Int(val)) => DataValue::from(*val as i8),
            (PrimitiveDataType::Byte, DataValue::Long(val)) => DataValue::from(*val as i8),
            (PrimitiveDataType::Byte, DataValue::Float(val)) => DataValue::from(*val as i8),
            (PrimitiveDataType::Byte, DataValue::Double(val)) => DataValue::from(*val as i8),

            (PrimitiveDataType::Char, DataValue::Short(val)) => DataValue::from(*val as u16),
            (PrimitiveDataType::Char, DataValue::Int(val)) => DataValue::from(*val as u16),
            (PrimitiveDataType::Char, DataValue::Long(val)) => DataValue::from(*val as u16),
            (PrimitiveDataType::Char, DataValue::Float(val)) => DataValue::from(*val as u16),
            (PrimitiveDataType::Char, DataValue::Double(val)) => DataValue::from(*val as u16),

            (PrimitiveDataType::Short, DataValue::Char(val)) => DataValue::from(*val as i16),
            (PrimitiveDataType::Short, DataValue::Int(val)) => DataValue::from(*val as i16),
            (PrimitiveDataType::Short, DataValue::Long(val)) => DataValue::from(*val as i16),
            (PrimitiveDataType::Short, DataValue::Float(val)) => DataValue::from(*val as i16),
            (PrimitiveDataType::Short, DataValue::Double(val)) => DataValue::from(*val as i16),

            (PrimitiveDataType::Int, DataValue::Long(val)) => DataValue::from(*val as i32),
            (PrimitiveDataType::Int, DataValue::Float(val)) => DataValue::from(*val as i32),
            (PrimitiveDataType::Int, DataValue::Double(val)) => DataValue::from(*val as i32),

            (PrimitiveDataType::Long, DataValue::Float(val)) => DataValue::from(*val as i64),
            (PrimitiveDataType::Long, DataValue::Double(val)) => DataValue::from(*val as i64),

            (PrimitiveDataType::Float, DataValue::Double(val)) => DataValue::from(*val as f32),

            // TODO is int->bool technically narrowing and should it be included here?
            (PrimitiveDataType::Boolean, DataValue::Int(val)) => {
                DataValue::Boolean((*val & 0x1) != 0)
            }

            _ => return None,
        })
    }
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataValue::Boolean(a), DataValue::Boolean(b)) => a == b,
            (DataValue::ReturnAddress(a), DataValue::ReturnAddress(b)) => a == b,
            (DataValue::Byte(a), DataValue::Byte(b)) => a == b,
            (DataValue::Short(a), DataValue::Short(b)) => a == b,
            (DataValue::Int(a), DataValue::Int(b)) => a == b,
            (DataValue::Long(a), DataValue::Long(b)) => a == b,
            (DataValue::Char(a), DataValue::Char(b)) => a == b,
            (DataValue::Float(a), DataValue::Float(b)) => a == b,
            (DataValue::Double(a), DataValue::Double(b)) => a == b,
            (DataValue::Reference(a), DataValue::Reference(b)) => vmref_eq(a, b),
            _ => false,
        }
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

        let first_idx = bytes.iter().position(|b| *b == b'[')?;
        let last_idx = bytes.iter().rposition(|b| *b == b'[')?;
        if first_idx != last_idx {
            // multidim
            return Some(ArrayType::Reference(mstr::from_mutf8(
                &bytes[first_idx + 1..],
            )));
        }

        let first_char = *bytes.get(last_idx + 1)?;
        Some(if first_char == b'L' {
            if !matches!(bytes.last(), Some(b';')) {
                return None;
            }

            let ref_name = bytes.get(last_idx + 2..bytes.len() - 1).and_then(|b| {
                if b.is_empty() {
                    None
                } else {
                    Some(b)
                }
            })?;

            ArrayType::Reference(mstr::from_mutf8(ref_name))
        } else {
            let prim = PrimitiveDataType::from_descriptor(&bytes[last_idx + 1..])?; //.map(ArrayType::Primitive)
            ArrayType::Primitive(prim)
        })
    }
}

impl<'a> ReturnType<'a> {
    pub fn to_owned(&self) -> ReturnType<'static> {
        match self {
            ReturnType::Returns(ty) => ReturnType::Returns(ty.to_owned()),
            ReturnType::Void => ReturnType::Void,
        }
    }

    pub fn convert_value(
        &self,
        ret_val: Option<DataValue>,
    ) -> Result<Option<DataValue>, Option<DataValue>> {
        match (self, &ret_val) {
            (ReturnType::Void, None) => Ok(None),
            (ReturnType::Void, Some(_)) => Err(ret_val),
            (ReturnType::Returns(ret_type), Some(val)) => {
                // special case for null
                if let Some(obj) = val.as_reference() {
                    if obj.is_null() {
                        return if ret_type.is_reference() {
                            // null is any reference type
                            Ok(ret_val)
                        } else {
                            Err(ret_val)
                        };
                    }
                }

                match val.assign_to(ret_type) {
                    Some(val) => Ok(Some(val.into_owned())),
                    None => Err(ret_val),
                }
            }
            _ => Err(ret_val),
        }
    }
}

impl<'a> From<Option<&'a DataValue>> for ReturnType<'static> {
    fn from(val: Option<&'a DataValue>) -> Self {
        val.map(|val| ReturnType::Returns(val.data_type()))
            .unwrap_or(ReturnType::Void)
    }
}

macro_rules! impl_data_value_type {
    (@from $ty:ty, $variant:ident) => {
        impl From<$ty> for DataValue {
            fn from(v: $ty) -> Self {
                Self::$variant(v)
            }
        }
    };

    ($ty:ty, $variant:ident) => {
        impl_data_value_type!(@from $ty, $variant);

        impl TryFrom<DataValue> for $ty {
            type Error = DataValue;

            fn try_from(val: DataValue) -> Result<$ty, Self::Error> {
                if let DataValue::$variant(v) = val {
                    Ok(v)
                } else {
                    Err(val)
                }
            }
        }
    };
}

impl_data_value_type!(@from bool, Boolean); // custom
impl_data_value_type!(i8, Byte);
impl_data_value_type!(i16, Short);
impl_data_value_type!(i32, Int);
impl_data_value_type!(i64, Long);
impl_data_value_type!(@from u16, Char); // custom
impl_data_value_type!(f32, Float);
impl_data_value_type!(f64, Double);
impl_data_value_type!(VmRef<Object>, Reference);

impl TryFrom<DataValue> for bool {
    type Error = DataValue;

    fn try_from(val: DataValue) -> Result<bool, Self::Error> {
        match val {
            DataValue::Boolean(b) => Ok(b),
            DataValue::Int(i) => Ok(i == 1),
            val => Err(val),
        }
    }
}

impl TryFrom<DataValue> for u16 {
    type Error = DataValue;

    fn try_from(val: DataValue) -> Result<Self, Self::Error> {
        match val {
            DataValue::Char(c) => Ok(c),
            DataValue::Int(i) => u16::try_from(i).map_err(|_| val),
            val => Err(val),
        }
    }
}

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

    pub fn is_valid(descriptor: &mstr) -> bool {
        let mut desc = MethodSignature::from_descriptor(descriptor);
        for _ in desc.iter_args() {}
        !desc.errored()
    }
}

impl<'a, 'b> Iterator for MethodSignatureIter<'a, 'b> {
    type Item = DataType<'a>;

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

impl From<NewarrayType> for PrimitiveDataType {
    fn from(ty: NewarrayType) -> Self {
        match ty {
            NewarrayType::Boolean => PrimitiveDataType::Boolean,
            NewarrayType::Char => PrimitiveDataType::Char,
            NewarrayType::Float => PrimitiveDataType::Float,
            NewarrayType::Double => PrimitiveDataType::Double,
            NewarrayType::Byte => PrimitiveDataType::Byte,
            NewarrayType::Short => PrimitiveDataType::Short,
            NewarrayType::Int => PrimitiveDataType::Int,
            NewarrayType::Long => PrimitiveDataType::Long,
        }
    }
}

impl FromStr for PrimitiveDataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "boolean" => PrimitiveDataType::Boolean,
            "char" => PrimitiveDataType::Char,
            "float" => PrimitiveDataType::Float,
            "double" => PrimitiveDataType::Double,
            "byte" => PrimitiveDataType::Byte,
            "short" => PrimitiveDataType::Short,
            "int" => PrimitiveDataType::Int,
            "long" => PrimitiveDataType::Long,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        ArrayType, DataType, DataValue, MethodSignature, PrimitiveDataType, ReturnType,
    };
    use cafebabe::mutf8::StrExt;

    fn check(input: &'static str, expected: Option<DataType>) {
        assert_eq!(DataType::from_descriptor(input.as_mstr()), expected)
    }

    fn check_ref(input: &'static str, expected: &'static str) {
        assert_eq!(
            DataType::from_descriptor(input.as_mstr()),
            Some(DataType::Reference(expected.to_mstr()))
        )
    }

    fn check_array(input: &'static str, expected: Option<ArrayType>) {
        assert_eq!(ArrayType::from_descriptor(input.as_mstr()), expected)
    }

    fn check_method(input: &'static str, expected: Option<(Vec<DataType>, ReturnType)>) {
        let mut sig = MethodSignature::from_descriptor(input.as_mstr());
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
        check_ref("Ljava/lang/Woopdedoo;", "java/lang/Woopdedoo");
        check("Lwoop;nah", None);
    }

    #[test]
    fn array() {
        check("[", None);
        check_ref("[I", "[I");
        check_ref("[[D", "[[D");
        check_ref("[[Ljava/lang/Object;", "[[Ljava/lang/Object;");
    }

    #[test]
    fn array_type() {
        check_array("", None);
        check_array("I", None);

        check_array("[I", Some(ArrayType::Primitive(PrimitiveDataType::Int)));
        check_array("[[C", Some(ArrayType::Reference("[C".as_mstr())));
        check_array(
            "[[[Lcool;",
            Some(ArrayType::Reference("[[Lcool;".as_mstr())),
        );
        // check_array("[[[[I.", None);

        check_array("[nothing", None);
        check_array("[Lcool", None);
        check_array("[L;", None);
        check_array("[Lcool;", Some(ArrayType::Reference("cool".as_mstr())));
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
                ReturnType::Returns(DataType::Reference("nice".to_mstr())),
            )),
        );
        check_method("()asdf", None);

        check_method(
            "(I[[D)V",
            Some((
                vec![
                    DataType::Primitive(PrimitiveDataType::Int),
                    DataType::Reference("[[D".to_mstr()),
                ],
                ReturnType::Void,
            )),
        );
    }

    #[test]
    fn assignment_trivial() {
        let boolean = DataValue::Boolean(true);
        let byte = DataValue::Byte(50);
        let short = DataValue::Short(20_000);
        let int = DataValue::Int(100_000);
        let long = DataValue::Long(6_000_000_000);
        let float = DataValue::Float(6.2);
        let double = DataValue::Double(4.111111111111123);

        let all = &[
            boolean.clone(),
            byte.clone(),
            short.clone(),
            int.clone(),
            long.clone(),
            float.clone(),
            double.clone(),
        ];

        // identity
        for val in all {
            assert_eq!(
                val.assign_to(&val.data_type()).map(|v| v.data_type()),
                Some(val.data_type())
            );
        }

        // widening primitive
        assert_eq!(
            short.assign_to(&int.data_type()).map(|v| v.data_type()),
            Some(int.data_type())
        );

        // narrow primitive
        // TODO actually check values of converted primitives
        assert!(short.assign_to(&byte.data_type()).is_some());
        assert!(int.assign_to(&short.data_type()).is_some());
        assert!(long.assign_to(&int.data_type()).is_some());

        // int to bool
        assert_eq!(
            DataValue::Int(1)
                .assign_to(&boolean.data_type())
                .unwrap()
                .into_owned(),
            DataValue::Boolean(true)
        );
        assert_eq!(
            DataValue::Int(0)
                .assign_to(&boolean.data_type())
                .unwrap()
                .into_owned(),
            DataValue::Boolean(false)
        );
    }
}
