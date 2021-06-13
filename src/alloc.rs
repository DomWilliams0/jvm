use crate::class::Object;
use crate::error::{Throwable, Throwables, VmResult};

use cafebabe::mutf8::MString;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Weak};

// TODO gc arena
pub type VmRef<T> = Arc<T>;
pub type WeakVmRef<T> = Weak<T>;

// TODO actually intern strings

pub type NativeString = MString;

pub type InternedString = MString;

// TODO methods on VmRef newtype
pub fn vmref_is_null(vmref: &VmRef<Object>) -> bool {
    vmref_ptr(vmref) == 0 || vmref.is_null()
}

pub fn vmref_ptr<O>(vmref: &VmRef<O>) -> usize {
    Arc::as_ptr(vmref) as usize
}

pub fn vmref_eq<A, B>(a: &VmRef<A>, b: &VmRef<B>) -> bool {
    vmref_ptr(a) == vmref_ptr(b)
}

pub fn vmref_into_raw<T>(vmref: VmRef<T>) -> *const T {
    Arc::into_raw(vmref)
}

pub fn vmref_as_raw<T>(vmref: &VmRef<T>) -> *const T {
    Arc::as_ptr(vmref)
}

/// Must have come from [vmref_into_raw]
pub unsafe fn vmref_from_raw<T>(ptr: *const T) -> VmRef<T> {
    Arc::from_raw(ptr)
}

/// Returns new count
pub fn vmref_increment<T>(vmref: &VmRef<T>) -> usize {
    let _clone = ManuallyDrop::new(vmref.clone());
    Arc::strong_count(vmref)
}

pub fn vmref_alloc_object(f: impl FnOnce() -> VmResult<Object>) -> VmResult<VmRef<Object>> {
    // TODO oom error
    Ok(VmRef::new(f()?))
}

pub fn vmref_alloc_exception(throwable: Throwables) -> VmRef<Throwable> {
    let class_name = throwable.symbol();
    VmRef::new(Throwable { class_name })
}

pub fn vmref_to_weak<T>(vmref: &VmRef<T>) -> WeakVmRef<T> {
    Arc::downgrade(vmref)
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
