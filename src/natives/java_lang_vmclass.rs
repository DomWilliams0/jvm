use crate::alloc::VmRef;
use crate::class::{null, FunctionArgs, WhichLoader};
use crate::error::{Throwable, Throwables};
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;
use itertools::Itertools;
use log::{error, trace};
use smallvec::SmallVec;

pub fn vm_for_name(mut args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (loader, initialize, name) = args.take_all().next_tuple().unwrap(); // verified
    trace!(
        "VMClass.forName({:?}, {:?}, {:?})",
        name,
        initialize,
        loader
    );

    let mut name_str: String = name
        .as_reference()
        .and_then(|r| r.string_value_utf8())
        .ok_or(Throwables::NullPointerException)?;

    // convert from java.lang.xyz to java/lang/xyz
    let mut byte_indices = SmallVec::<[_; 8]>::new();
    for (i, c) in name_str.char_indices() {
        if c == '.' {
            byte_indices.push(i);
        }
    }

    // safety: hasn't changed
    unsafe {
        let bytes = name_str.as_bytes_mut();
        for i in byte_indices {
            *bytes.get_unchecked_mut(i) = b'/';
        }
    }

    let initialize = match initialize {
        DataValue::Int(i) => i == 1,
        DataValue::Boolean(b) => b,
        _ => unreachable!(), // verified
    };

    let loader = match loader {
        DataValue::Reference(r) => {
            if r.is_null() {
                WhichLoader::Bootstrap
            } else {
                WhichLoader::User(r)
            }
        }
        _ => unreachable!(), // verified
    };

    let state = thread::get();
    // TODO pass in cause for loading
    let loaded = state
        .global()
        .class_loader()
        .load_class(&mstr::from_utf8(name_str.as_bytes()), loader)
        .map_err(|e| {
            error!("failed to load class: {:?}", e);
            Throwables::ClassNotFoundException
        })?;

    if initialize {
        loaded.ensure_init()?;
    }

    Ok(Some(DataValue::Reference(loaded.class_object().clone())))
}
