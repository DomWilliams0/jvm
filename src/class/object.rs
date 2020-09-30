use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::num::NonZeroI32;

use itertools::{repeat_n, Itertools};
use lazy_static::lazy_static;
use log::*;
use parking_lot::{Mutex, MutexGuard};

use cafebabe::mutf8::{mstr, StrExt};
use cafebabe::AccessFlags;

use crate::alloc::{vmref_alloc_object, vmref_ptr, VmRef};
use crate::class::class::{Class, ClassType, FieldSearchType, SuperIteration};
use crate::error::{Throwables, VmResult};
use crate::monitor::{Monitor, MonitorGuard};
use crate::storage::{FieldId, FieldStorage};
use crate::thread;
use crate::types::{DataType, DataValue, PrimitiveDataType};

pub enum ObjectStorage {
    Fields(FieldStorage),
    // TODO arrays should live on the GC java heap
    // TODO arrays should be specialised and not hold massive DataValues
    Array(Mutex<Box<[DataValue]>>),
}

pub struct Object {
    class: VmRef<Class>,
    pub(in crate::class) monitor: Monitor,
    storage: ObjectStorage,
    // TODO mutex only needed in edge case, try with atomic op first
    hashcode: Mutex<Option<NonZeroI32>>,
}

pub struct ObjectFieldPrinter<'a> {
    obj: &'a Object,
}

lazy_static! {
    static ref NULL: VmRef<Object> = VmRef::new(Object::new_null());
}

/// Null object singleton
pub fn null() -> VmRef<Object> {
    NULL.clone()
}

impl Object {
    /// Only use this to create the sentinel NULL value
    fn new_null() -> Self {
        // TODO just allocate an object instead of this unsafeness
        let null_class = MaybeUninit::zeroed();
        let null_class = unsafe { null_class.assume_init() };
        let storage = ObjectStorage::Fields(FieldStorage::empty());
        Object {
            class: null_class,
            monitor: Monitor::new(),
            storage,
            hashcode: Mutex::new(None),
        }
    }

    pub fn with_storage(class: VmRef<Class>, storage: ObjectStorage) -> Self {
        Object {
            class,
            monitor: Monitor::new(),
            storage,
            hashcode: Mutex::new(None),
        }
    }

    pub(crate) fn new(class: VmRef<Class>) -> Self {
        let fields = class.instance_fields_layout().new_storage();
        Self::with_storage(class, ObjectStorage::Fields(fields))
    }
    pub(crate) fn new_array(array_cls: VmRef<Class>, len: usize) -> Self {
        let elem_cls = match array_cls.class_type() {
            ClassType::Array(elem) => elem,
            _ => unreachable!(),
        };

        let elem_type = match elem_cls.class_type().clone() {
            ClassType::Primitive(prim) => DataType::Primitive(prim),
            ClassType::Normal => DataType::Reference(Cow::Owned(elem_cls.name().to_owned())),
            ClassType::Array(_) => unreachable!(),
        };

        Self::new_array_with_elements(array_cls, repeat_n(elem_type.default_value(), len))
    }

    pub(crate) fn new_array_with_elements(
        array_cls: VmRef<Class>,
        elems: impl ExactSizeIterator<Item = DataValue>,
    ) -> Self {
        debug_assert!(matches!(array_cls.class_type(), ClassType::Array(_)));

        let data: Box<[DataValue]> = elems.collect();
        Self::with_storage(array_cls, ObjectStorage::Array(Mutex::new(data)))
    }

    pub(crate) fn new_string(contents: &mstr) -> VmResult<Object> {
        // encode for java/lang/String
        let utf16 = contents.to_utf8().encode_utf16().collect_vec();

        let tls = thread::get();
        let classloader = tls.global().class_loader();

        // alloc string instance
        let string_class = classloader.get_bootstrap_class("java/lang/String");
        let string_instance = Object::new(string_class);
        let fields = string_instance.fields().unwrap();

        // alloc char array
        let char_array_cls = classloader.get_primitive_array(PrimitiveDataType::Char);
        let char_array = vmref_alloc_object(|| Ok(Object::new_array(char_array_cls, utf16.len())))?;

        // populate char array
        {
            let mut array_contents = char_array.array().unwrap();
            let slice = &mut array_contents[0..utf16.len()];
            for (i, char) in utf16.into_iter().enumerate() {
                slice[i] = DataValue::Char(char);
            }
        }

        // TODO limit array length to i32::MAX somewhere

        let set_field = |name: &'static str, value: DataValue| -> VmResult<()> {
            let name = name.to_mstr();
            let datatype = value.data_type();
            trace!("setting string field {:?} to {:?}", name, value);
            let field_id = string_instance
                .find_field_in_this_only(name.as_ref(), &datatype, FieldSearchType::Instance)
                .ok_or_else(|| Throwables::Other("java/lang/NoSuchFieldError"))?;

            fields.ensure_set(field_id, value);
            Ok(())
        };

        set_field("value", DataValue::Reference(char_array))?;

        Ok(string_instance)
    }
    pub fn is_null(&self) -> bool {
        VmRef::ptr_eq(&self.class, &NULL.class)
    }

    /// None if null
    pub fn class(&self) -> Option<VmRef<Class>> {
        if self.is_null() {
            None
        } else {
            Some(self.class.clone())
        }
    }

    pub fn enter_monitor(&self) -> MonitorGuard {
        self.monitor.enter()
    }

    pub fn fields(&self) -> Option<&FieldStorage> {
        match &self.storage {
            ObjectStorage::Fields(f) => Some(f),
            _ => None,
        }
    }

    pub fn array(&self) -> Option<MutexGuard<Box<[DataValue]>>> {
        match &self.storage {
            ObjectStorage::Array(mutex) => Some(mutex.lock()),
            _ => None,
        }
    }

    pub fn array_unchecked(&self) -> MutexGuard<Box<[DataValue]>> {
        match &self.storage {
            ObjectStorage::Array(mutex) => mutex.lock(),
            _ => unreachable!(),
        }
    }

    pub fn array_get_unchecked(&self, idx: usize) -> DataValue {
        let val = self.array_unchecked().get(idx).unwrap().clone();
        trace!("get array element {:?}[{}] = {:?}", self, idx, val);
        val
    }

    pub fn array_set_unchecked(&self, idx: usize, val: DataValue) {
        trace!("set array element {:?}[{}] = {:?}", self, idx, val);
        let mut array = self.array_unchecked();
        let elem = array.get_mut(idx).unwrap();
        *elem = val;
    }

    pub fn find_field_in_this_only(
        &self,
        name: &mstr,
        desc: &DataType,
        search: FieldSearchType,
    ) -> Option<FieldId> {
        let field_index = self.class.find_field_index(name, desc, search)?;
        self.class.instance_fields_layout().get_self_id(field_index)
    }

    pub fn find_field(
        &self,
        name: &mstr,
        desc: &DataType,
        search: FieldSearchType,
    ) -> Option<DataValue> {
        let field_id = self.class.find_field_recursive(name, desc, search)?;
        Some(self.field(field_id))
    }

    pub fn field(&self, field_id: FieldId) -> DataValue {
        debug_assert!(!self.is_null(), "object is null");
        let fields = self.fields().expect("object has no field storage");

        fields
            .try_get(field_id)
            .unwrap_or_else(|| panic!("bad field {:?}", field_id))
    }

    pub fn array_length(&self) -> Option<i32> {
        self.array().map(|arr| arr.len() as i32)
    }

    pub fn is_array(&self) -> bool {
        matches!(self.storage, ObjectStorage::Array(_))
    }

    pub fn storage(&self) -> &ObjectStorage {
        &self.storage
    }

    /// Calculates and stores on first call
    pub fn identity_hashcode(self: &VmRef<Self>) -> i32 {
        let mut guard = self.hashcode.lock();
        match *guard {
            Some(hash) => hash.get(),
            None => {
                let ptr = vmref_ptr(self);
                let hash = (ptr & 0xffffffff) as i32;
                *guard = unsafe {
                    debug_assert_ne!(hash, 0, "lmao null pointer what");
                    Some(NonZeroI32::new_unchecked(hash))
                };
                hash
            }
        }
    }

    pub fn print_fields(&self) -> ObjectFieldPrinter {
        ObjectFieldPrinter { obj: self }
    }

    pub fn with_string_value<R>(&self, mut f: impl FnMut(&str) -> R) -> Option<R> {
        if self.class.name().as_bytes() == b"java/lang/String" {
            if let Some(DataValue::Reference(chars)) = self.find_field(
                "value".as_mstr(),
                &DataType::Reference(Cow::Borrowed("[C".as_mstr())),
                FieldSearchType::Instance,
            ) {
                if !chars.is_null() {
                    let chars = chars.array_unchecked();
                    let chars = chars
                        .iter()
                        .map(|val| match val {
                            DataValue::Char(c) => *c,
                            _ => unreachable!(),
                        })
                        .collect_vec();

                    // TODO do this without all the allocations
                    let tmp_str = String::from_utf16_lossy(&chars);
                    return Some(f(&tmp_str));
                }
            } else {
                unreachable!("bad string class")
            }
        }

        None
    }
}

impl Debug for ObjectFieldPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cls = match self.obj.class() {
            None => return write!(f, "(null)"),
            Some(cls) => cls,
        };

        write!(f, "Fields for {:?}: ", self.obj)?;

        let field_storage = match self.obj.fields() {
            None => return write!(f, "None"),
            Some(fields) => fields,
        };

        let layout = cls.instance_fields_layout();

        let mut cls_idx = 0;
        let mut result = Ok(());
        cls.field_resolution_order(|fields| {
            for (i, field) in fields.iter().filter(|f| !f.flags().is_static()).enumerate() {
                // TODO statics too

                let field_id = layout.get_id(cls_idx, i).unwrap();
                let val = field_storage.ensure_get(field_id);

                result = write!(
                    f,
                    "\n * {} ({:?} {:?}) => {:?}",
                    field.name(),
                    field.desc(),
                    field.flags(),
                    val
                );
                if result.is_err() {
                    return SuperIteration::Stop;
                }
            }

            cls_idx += 1;
            SuperIteration::KeepGoing
        });

        result
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "null")
        } else {
            // TODO not quite correct toString
            // let ptr = vmref_ptr(&self.class);
            let ptr = self as *const _ as u64;
            write!(f, "{}@{:#x}", self.class.name(), ptr)?;

            // helpful for debugging
            let result = self.with_string_value(|str_val| write!(f, " ({:?})", str_val));

            if let Some(result) = result {
                result?; // lol
            }

            Ok(())
        }
    }
}

impl Clone for ObjectStorage {
    fn clone(&self) -> Self {
        match self {
            Self::Fields(fields) => Self::Fields(fields.clone()),
            Self::Array(array) => Self::Array(Mutex::new(array.lock().clone())),
        }
    }
}