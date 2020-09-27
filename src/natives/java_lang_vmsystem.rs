use crate::class::FunctionArgs;
use crate::types::DataValue;

pub fn vm_identity_hashcode(mut args: FunctionArgs) -> Option<DataValue> {
    // TODO dont unwrap
    let obj = args.take(0).into_reference().unwrap();
    let hash = obj.identity_hashcode();
    Some(DataValue::Int(hash))
}
