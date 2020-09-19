use crate::alloc::{InternedString, NativeString};
use crate::types::DataType;
use cafebabe::mutf8::MString;
use cafebabe::{
    ClassError, ClassRefEntry, ClassResult, FieldRefEntry, InterfaceMethodRefEntry, Item,
    MethodRefEntry,
};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Entry {
    // TODO store interned string instance here
    String(MString),
    MethodRef(MethodRef),
    InterfaceMethodRef(MethodRef),
    FieldRef(FieldRef),
    ClassRef(ClassRef),
    Float(f32),
}

// TODO method and field refs should be resolved vtable indices instead of loads of strings

#[derive(Debug)]
pub struct MethodRef {
    pub class: InternedString,
    pub name: NativeString,
    pub desc: NativeString,
}

#[derive(Debug)]
pub struct FieldRef {
    pub class: InternedString,
    pub name: NativeString,
    pub desc: DataType<'static>,
}

#[derive(Debug)]
pub struct ClassRef {
    pub name: InternedString,
}

pub struct RuntimeConstantPool(Vec<Option<Entry>>);

impl RuntimeConstantPool {
    fn with_size(n: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        vec.resize_with(n, || None);
        RuntimeConstantPool(vec)
    }

    pub fn empty() -> Self {
        RuntimeConstantPool(Vec::new())
    }

    pub fn from_cafebabe(pool: &cafebabe::ConstantPool) -> ClassResult<Self> {
        let mut my_pool = Self::with_size(pool.size());

        for (idx, item) in pool.entries() {
            match item {
                Item::String { string } => {
                    let string = pool.string_entry(*string)?;
                    my_pool.put_entry(idx, Entry::String(string.to_owned()));
                }
                Item::MethodRef { .. } => {
                    let methodref = pool.entry::<MethodRefEntry>(idx)?;
                    my_pool.put_entry(
                        idx,
                        Entry::MethodRef(MethodRef {
                            class: methodref.class.to_owned(),
                            name: methodref.name.to_owned(),
                            desc: methodref.desc.to_owned(),
                        }),
                    );
                }
                Item::InterfaceMethodRef { .. } => {
                    let methodref = pool.entry::<InterfaceMethodRefEntry>(idx)?;
                    my_pool.put_entry(
                        idx,
                        Entry::InterfaceMethodRef(MethodRef {
                            class: methodref.class.to_owned(),
                            name: methodref.name.to_owned(),
                            desc: methodref.desc.to_owned(),
                        }),
                    );
                }
                Item::FieldRef { .. } => {
                    let fieldref = pool.entry::<FieldRefEntry>(idx)?;
                    my_pool.put_entry(
                        idx,
                        Entry::FieldRef(FieldRef {
                            class: fieldref.class.to_owned(),
                            name: fieldref.name.to_owned(),
                            desc: DataType::from_descriptor(fieldref.desc)
                                .ok_or_else(|| {
                                    ClassError::TypeDescriptor(fieldref.desc.to_owned())
                                })?
                                .to_owned(),
                        }),
                    );
                }
                Item::Class { .. } => {
                    let classref = pool.entry::<ClassRefEntry>(idx)?;
                    my_pool.put_entry(
                        idx,
                        Entry::ClassRef(ClassRef {
                            name: classref.name.to_owned(),
                        }),
                    );
                }
                Item::Float { float } => my_pool.put_entry(idx, Entry::Float(*float)),

                _ => continue,
            }
        }

        Ok(my_pool)
    }

    fn entries(&self) -> impl Iterator<Item = (usize, &Entry)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, item)| item.as_ref().map(|item| ((i + 1), item)))
    }

    fn put_entry(&mut self, idx: u16, entry: Entry) {
        // adjust for 1-indexing
        let idx = (idx - 1) as usize;
        self.0[idx] = Some(entry);
    }

    pub fn entry(&self, idx: u16) -> Option<&Entry> {
        // adjust for 1-indexing
        let idx = (idx - 1) as usize;
        self.0.get(idx).and_then(|i| i.as_ref())
    }

    pub fn loadable_entry(&self, idx: u16) -> Option<&Entry> {
        self.entry(idx)
            .and_then(|e| if e.is_loadable() { Some(e) } else { None })
    }

    /// Method ref only
    pub fn method_entry(&self, idx: u16) -> Option<&MethodRef> {
        self.entry(idx).and_then(|e| match e {
            Entry::MethodRef(m) => Some(m),
            _ => None,
        })
    }

    pub fn method_or_interface_entry(&self, idx: u16) -> Option<&MethodRef> {
        self.entry(idx).and_then(|e| match e {
            Entry::MethodRef(m) | Entry::InterfaceMethodRef(m) => Some(m),
            _ => None,
        })
    }

    pub fn field_entry(&self, idx: u16) -> Option<&FieldRef> {
        self.entry(idx).and_then(|e| match e {
            Entry::FieldRef(f) => Some(f),
            _ => None,
        })
    }

    pub fn class_entry(&self, idx: u16) -> Option<&ClassRef> {
        self.entry(idx).and_then(|e| match e {
            Entry::ClassRef(f) => Some(f),
            _ => None,
        })
    }
}

impl Entry {
    /// Symbolic references to classes and interfaces
    ///
    /// Symbolic references to method handles
    ///
    /// Symbolic references to method types
    ///
    /// Symbolic references to dynamically-computed constants
    ///
    /// Static constants
    pub fn is_loadable(&self) -> bool {
        match self {
            Entry::String(_)
            | Entry::MethodRef(_)
            | Entry::InterfaceMethodRef(_)
            | Entry::FieldRef(_)
            | Entry::ClassRef(_)
            | Entry::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_long_or_double(&self) -> bool {
        // TODO A numeric constant of type long or double OR A symbolic reference to a
        //  dynamically-computed constant whose field descriptor is J (denoting long) or D (denoting double)
        false
    }
}

impl Debug for RuntimeConstantPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuntimeConstantPool(")?;
        f.debug_list().entries(self.entries()).finish()?;
        write!(f, ")")
    }
}
