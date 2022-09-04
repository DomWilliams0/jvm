use crate::alloc::VmRef;
use crate::class::{FunctionArgs, Object, WhichLoader};
use crate::error::{Throwable, Throwables};
use crate::thread;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;

use log::{error, trace};
use smallvec::SmallVec;

pub fn vm_for_name(args: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    let (loader, initialize, mut name) = args.destructure::<(VmRef<Object>, bool, String)>()?;

    trace!(
        "VMClass.forName({:?}, {:?}, {:?})",
        name,
        initialize,
        loader
    );

    // TODO put this into helper
    // convert from java.lang.xyz to java/lang/xyz
    let mut byte_indices = SmallVec::<[_; 8]>::new();
    for (i, c) in name.char_indices() {
        if c == '.' {
            byte_indices.push(i);
        }
    }

    // safety: hasn't changed
    unsafe {
        let bytes = name.as_bytes_mut();
        for i in byte_indices {
            *bytes.get_unchecked_mut(i) = b'/';
        }
    }

    let loader = if loader.is_null() {
        WhichLoader::Bootstrap
    } else {
        WhichLoader::User(loader)
    };

    let state = thread::get();
    // TODO pass in cause for loading
    let loaded = state
        .global()
        .class_loader()
        .load_class(&mstr::from_utf8(name.as_bytes()), loader)
        .map_err(|e| {
            error!("failed to load class: {:?}", e);
            Throwables::ClassNotFoundException
        })?;

    if initialize {
        loaded.ensure_init()?;
    }

    Ok(Some(DataValue::Reference(loaded.class_object().clone())))
}
