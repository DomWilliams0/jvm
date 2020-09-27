use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_identity_hashcode(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    // TODO dont unwrap
    let obj = args.take(0).into_reference().unwrap();
    let hash = obj.identity_hashcode();
    Ok(Some(DataValue::Int(hash)))
}
