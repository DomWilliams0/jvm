use crate::class::Object;
use cafebabe::mutf8::MString;
use std::sync::Arc;

// TODO gc arena
pub type VmRef<T> = Arc<T>;

// TODO actually intern strings

pub type NativeString = MString;

pub type InternedString = MString;

// TODO methods on VmRef newtype
pub fn vmref_is_null(vmref: &VmRef<Object>) -> bool {
    vmref.is_null()
}

pub fn vmref_ptr<O>(vmref: &VmRef<O>) -> u64 {
    Arc::as_ptr(vmref) as u64
}

#[cfg(test)]
mod tests {
    use crate::alloc::vmref_is_null;
    use crate::class::NULL;

    #[test]
    fn null_is_null() {
        assert!(vmref_is_null(&NULL));
    }
}
