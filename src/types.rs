use crate::class::NULL;
use strum_macros::EnumDiscriminants;

use crate::alloc::VmRef;
use crate::class::Object;

// TODO more efficient packing of data types, dont want huge enum discriminant taking up all the space
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(DataType))]
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
    Reference(VmRef<Object>),
}

impl DataType {
    fn default_value(&self) -> DataValue {
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
            DataType::Reference => DataValue::Reference(NULL.clone()),
        }
    }
}
