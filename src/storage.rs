use crate::alloc::NativeString;
use crate::types::DataValue;
use cafebabe::mutf8::mstr;
use parking_lot::RwLock;
use std::borrow::Borrow;
use std::collections::HashMap;

pub trait Storage {
    type KeyRef: ?Sized;
    type Key: Borrow<Self::KeyRef>;
    type Value: Clone;

    fn get(&self, key: &Self::KeyRef) -> Option<Self::Value>;
    fn put(&mut self, key: Self::Key, value: impl Into<Self::Value>);
    fn set(&self, key: &Self::KeyRef, value: impl Into<Self::Value>) -> bool;
}

#[derive(Debug)]
pub struct FieldMapStorage(RwLock<HashMap<NativeString, DataValue>>);

impl Storage for FieldMapStorage {
    type KeyRef = mstr;
    type Key = NativeString;
    type Value = DataValue;

    fn get(&self, key: &Self::KeyRef) -> Option<Self::Value> {
        self.0.read().get(key).cloned()
    }

    fn put(&mut self, key: Self::Key, value: impl Into<Self::Value>) {
        self.0.write().insert(key, value.into());
    }

    fn set(&self, key: &Self::KeyRef, value: impl Into<Self::Value>) -> bool {
        self.0
            .write()
            .get_mut(key)
            .map(|val| {
                *val = value.into();
                true
            })
            .unwrap_or(false)
    }
}

impl FieldMapStorage {
    pub fn with_capacity(n: usize) -> Self {
        FieldMapStorage(RwLock::new(HashMap::with_capacity(n)))
    }
}
