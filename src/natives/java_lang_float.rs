use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn to_raw_int_bits(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (float,) = args.destructure::<(f32,)>()?;
    // TODO this is definitely wrong
    Ok(Some(DataValue::Int(float as i32)))
}
