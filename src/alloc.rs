use crate::class::Object;
use crate::error::{Throwable, Throwables, VmResult};

use cafebabe::mutf8::MString;
use std::sync::Arc;

// TODO gc arena
pub type VmRef<T> = Arc<T>;

// TODO actually intern strings

pub type NativeString = MString;

pub type InternedString = MString;

// TODO methods on VmRef newtype
pub fn vmref_is_null(vmref: &VmRef<Object>) -> bool {
    vmref_ptr(vmref) == 0 || vmref.is_null()
}

pub fn vmref_ptr<O>(vmref: &VmRef<O>) -> u64 {
    Arc::as_ptr(vmref) as u64
}

pub fn vmref_eq<A, B>(a: &VmRef<A>, b: &VmRef<B>) -> bool {
    vmref_ptr(a) == vmref_ptr(b)
}

pub fn vmref_alloc_object(f: impl FnOnce() -> VmResult<Object>) -> VmResult<VmRef<Object>> {
    // TODO oom error
    Ok(VmRef::new(f()?))
}

pub fn vmref_alloc_exception(throwable: Throwables) -> VmRef<Throwable> {
    let class_name = throwable.symbol();
    VmRef::new(Throwable { class_name })
}

#[cfg(test)]
mod tests {
    use crate::alloc::{vmref_eq, vmref_is_null};
    use crate::class::null;

    #[test]
    fn null_is_null() {
        assert!(vmref_is_null(&null()));
    }

    #[test]
    fn null_singleton() {
        assert!(vmref_eq(&null(), &null()));
    }
}
