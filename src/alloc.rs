use std::sync::Arc;

// TODO gc arena
pub type VmRef<T> = Arc<T>;

// TODO intern strings
