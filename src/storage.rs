use crate::types::{DataType, DataValue};

use parking_lot::RwLock;

use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct FieldId(u32);

// pub struct FieldStructure {
//     /// `self.start_indices[fieldid.class] = start offset of this class in storage vec`
//     start_indices: Vec<u32>,
//     owning_cls: VmRef<Class>,
// }
// impl FieldStructure {}
// TODO field storage should be inline in VmRef<Object>
// TODO compact field storage i.e. not using DataValue enum
// TODO phantom generic type to tag this as Static or Instance fields
#[derive(Debug)] // TODO fieldstorage better debug impl
pub struct FieldStorage(RwLock<Box<[DataValue]>>);

#[derive(Debug)]
pub struct FieldStorageLayout {
    /// Indexed by field_id.class_id, the cumulative index of the first field of that class in storage
    counts: Box<[u32]>,

    types: Box<[DataType]>,
}

pub struct FieldStorageLayoutBuilder {
    counts: Vec<u32>,
    types: Vec<DataType>,
}

impl FieldStorageLayoutBuilder {
    pub fn with_capacity(classes: usize, fields: usize) -> Self {
        FieldStorageLayoutBuilder {
            counts: Vec::with_capacity(classes),
            types: Vec::with_capacity(fields),
        }
    }

    pub fn add_fields_from_class(&mut self, tys: impl Iterator<Item = DataType>) {
        // store start offset of this class
        self.counts.push(self.types.len() as u32);
        self.types.extend(tys)
    }

    pub fn build(self) -> FieldStorageLayout {
        FieldStorageLayout {
            types: self.types.into_boxed_slice(),
            counts: self.counts.into_boxed_slice(),
        }
    }
}

impl FieldStorageLayout {
    pub fn empty() -> Self {
        FieldStorageLayout {
            counts: Vec::new().into_boxed_slice(),
            types: Vec::new().into_boxed_slice(),
        }
    }

    pub fn new_storage(&self) -> FieldStorage {
        let values = self
            .types
            .iter()
            .map(|ty| ty.clone().default_value())
            .collect();
        FieldStorage(RwLock::new(values))
    }

    pub fn get_id(&self, class_index: usize, field_index: usize) -> Option<FieldId> {
        let start_idx = *self.counts.get(class_index)? as usize;
        Some(FieldId((start_idx + field_index) as u32))
    }

    pub fn get_self_id(&self, field_index: usize) -> Option<FieldId> {
        self.get_id(0, field_index)
    }
}

impl FieldStorage {
    pub fn empty() -> Self {
        FieldStorage(RwLock::new(Box::from([])))
    }

    pub fn try_get(&self, id: FieldId) -> Option<DataValue> {
        self.0.read().get(id.0 as usize).cloned()
    }

    pub fn try_set(&self, id: FieldId, value: DataValue) -> bool {
        let mut guard = self.0.write();
        guard
            .get_mut(id.0 as usize)
            .map(|val| {
                debug_assert!(
                    val.data_type() == value.data_type(),
                    "field {:?} type is {:?} but trying to set to {:?}",
                    id,
                    val.data_type(),
                    value.data_type()
                );
                *val = value;
                true
            })
            .unwrap_or(false)
    }

    pub fn ensure_get(&self, id: FieldId) -> DataValue {
        self.try_get(id)
            .unwrap_or_else(|| panic!("no such field {:?}", id))
    }

    pub fn ensure_set(&self, id: FieldId, value: impl Into<DataValue>) {
        assert!(self.try_set(id, value.into()), "no such field {:?}", id);
    }
}

// TODO test this once structure is settled
/*#[cfg(test)]
mod tests {
    use crate::storage::FieldMapStorage;
    use crate::types::{DataType, DataValue, PrimitiveDataType};
    use cafebabe::mutf8::{mstr, MString};

    #[test]
    fn inherited_fields() {
        let name_super = MString::from_utf8(b"super");
        let name_instance = MString::from_utf8(b"instance");
        let type_int = DataType::Primitive(PrimitiveDataType::Int);

        let field_a = MString::from_utf8(b"a");
        let field_b = MString::from_utf8(b"b");
        let field_c = MString::from_utf8(b"c");

        let set_int = |fields: &mut FieldMapStorage, field: &mstr, val: i32| {
            let id = fields
                .resolve_id(field, DataType::Primitive(PrimitiveDataType::Int))
                .expect("no field");
            fields.ensure_set(&id, DataValue::Int(val))
        };

        let fields = {
            // super fields
            let mut super_fields = FieldMapStorage::with_capacity(2);
            super_fields.put(name_super.clone(), field_a.clone(), type_int.clone());
            set_int(&mut super_fields, &field_a, 100);
            super_fields.put(name_super, field_b.clone(), type_int.clone());
            set_int(&mut super_fields, &field_b, 100);

            // instance fields inherit
            let mut fields = FieldMapStorage::with_capacity(20);

            fields.put(name_instance.clone(), field_b.clone(), type_int.clone()); // shadows super
            set_int(&mut fields, &field_b, 200);
            fields.put(name_instance, field_c.clone(), type_int);
            set_int(&mut fields, &field_c, 300);

            fields.put_all_from(&super_fields);

            fields
        };

        let get_int = |field: &MString| {
            let id = fields
                .resolve_id(field, DataType::Primitive(PrimitiveDataType::Int))
                .expect("no field");
            fields.get(&id).and_then(|val| val.as_int())
        };

        assert_eq!(get_int(&field_a), Some(100)); // inherit super value
        assert_eq!(get_int(&field_b), Some(200)); // shadow super value
        assert_eq!(get_int(&field_c), Some(300)); // instance value
    }
}
*/
