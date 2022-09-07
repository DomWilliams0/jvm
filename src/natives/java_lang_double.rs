use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn to_raw_int_bits(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let double = args.take(0).as_double().unwrap();
    // TODO this is definitely wrong
    Ok(Some(DataValue::Long(double as i64)))
}
pub fn bits_to_double(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (long,) = args.destructure::<(i64,)>()?;
    // TODO this is definitely wrong
    Ok(Some(DataValue::Double(long as f64)))
}
