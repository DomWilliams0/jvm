use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::classpath::ClassPath;

#[derive(Debug)]
pub struct SystemProperties(HashMap<&'static str, SystemProperty>);

#[derive(Debug)]
pub struct SystemProperty(Cow<'static, str>);

impl Default for SystemProperties {
    fn default() -> Self {
        let defaults = [
            ("java.vm.name", "UntitledJvm"),
            ("java.vm.vendor", "Dom Williams"),
            // TODO remaining static ones
            // TODO dynamic ones e.g. user.home
        ];

        SystemProperties(HashMap::from_iter(
            defaults
                .iter()
                .map(|(key, val)| (*key, SystemProperty::from(*val))),
        ))
    }
}

impl SystemProperties {
    pub fn get(&self, key: &str) -> Option<&SystemProperty> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: &'static str, value: impl Into<SystemProperty>) {
        self.0.insert(key, value.into());
    }

    pub fn set_path(&mut self, key: &'static str, value: &ClassPath) {
        self.set(key, value.to_string())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.0.iter().map(|(key, val)| (*key, val.0.as_ref()))
    }
}

impl From<&'static str> for SystemProperty {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl From<String> for SystemProperty {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

// required:
// java.version
// java.vendor
// java.vendor.url
// java.home
// java.vm.specification.version
// java.vm.specification.vendor
// java.vm.specification.name
// java.vm.version
// java.vm.vendor
// java.vm.name
// java.specification.version
// java.specification.vendor
// java.specification.name
// java.class.version
// java.class.path
// java.library.path
// java.io.tmpdir
// java.compiler
// java.ext.dirs
// os.name
// os.arch
// os.version
// file.separator
// path.separator
// line.separator
// user.name
// user.home
// user.dir
