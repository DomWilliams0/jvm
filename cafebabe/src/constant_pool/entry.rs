use crate::constant_pool::item::{ClassRefItem, Tag, Utf8Item};

pub trait Entry {
    const TAG: Tag;
}

impl<'c> Entry for Utf8Item<'c> {
    const TAG: Tag = Tag::Utf8;
}

impl Entry for ClassRefItem {
    const TAG: Tag = Tag::Class;
}
