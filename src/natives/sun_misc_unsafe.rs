use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn vm_array_base_offset(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let obj = args.take(0).into_reference().unwrap();
    let _cls = obj.class().expect("null class").class_object().clone();

    // TODO Report the offset of the first element in the storage allocation of a given array class.
    let offset = 0;
    Ok(Some(DataValue::Int(offset)))
}

pub fn vm_array_index_scale(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let obj = args.take(0).into_reference().unwrap();
    let _cls = obj.class().expect("null class").class_object().clone();

    // TODO Report the scale factor for addressing elements in the storage
    //  allocation of a given array class.  However, arrays of "narrow" types
    //  will generally not work properly with accessors like {@link
    //  #getByte(Object, int)}, so the scale factor for such classes is reported
    //  as zero.
    let stride = 0;

    Ok(Some(DataValue::Int(stride)))
}

pub fn vm_address_size(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let sz = std::mem::size_of::<usize>();
    Ok(Some(DataValue::Int(sz as i32)))
}
