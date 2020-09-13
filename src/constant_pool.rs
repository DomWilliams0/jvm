use cafebabe::mutf8::MString;
use cafebabe::{ClassResult, Item};
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Entry {
    String(MString),
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
}

impl Entry {
    pub fn is_loadable(&self) -> bool {
        match self {
            Entry::String(_) => true,
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
