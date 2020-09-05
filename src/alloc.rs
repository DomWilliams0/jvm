use crate::class::Object;
use javaclass::mutf8::MString;
use std::sync::Arc;

// TODO gc arena
pub type VmRef<T> = Arc<T>;

// TODO actually intern strings

pub type NativeString = MString;

pub type InternedString = MString;

// TODO method on VmRef
pub fn is_null(vmref: &VmRef<Object>) -> bool {
    vmref.is_null()
}

#[cfg(test)]
mod tests {
    use crate::alloc::is_null;
    use crate::class::NULL;

    #[test]
    fn null_is_null() {
        assert!(is_null(&NULL));
    }
}
