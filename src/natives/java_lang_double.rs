use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_double_to_raw_int_bits(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let double = args.take(0).as_double().unwrap();
    // TODO this is definitely wrong
    Ok(Some(DataValue::Long(double as i64)))
}
pub fn vm_long_bits_to_double(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let long = args.take(0).as_long().unwrap();
    // TODO this is definitely wrong
    Ok(Some(DataValue::Double(long as f64)))
}
