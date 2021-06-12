use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_float_to_raw_int_bits(
    mut args: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let float = args.take(0).as_float().unwrap();
    // TODO this is definitely wrong
    Ok(Some(DataValue::Int(float as i32)))
}
