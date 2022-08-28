use std::borrow::Cow;
use std::collections::HashMap;

use crate::classpath::ClassPath;
use crate::{CLASS_VERSION, JAVA_SPEC_VERSION, JAVA_VERSION, JAVA_VM_SPEC_VERSION};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SystemProperties(HashMap<&'static str, SystemProperty>);

#[derive(Debug)]
pub struct SystemProperty(Cow<'static, str>);

impl Default for SystemProperties {
    fn default() -> Self {
        Self::new(
            dirs::data_dir(),
            std::env::temp_dir(),
            whoami::distro(),
            whoami::username(),
            dirs::home_dir(),
            std::env::current_dir().ok(),
            ".",
        )
    }
}

impl SystemProperties {
    pub fn new(
        java_home: impl Into<SystemProperty>,
        tmp_dir: impl Into<SystemProperty>,
        distro: String,
        username: String,
        home_dir: impl Into<SystemProperty>,
        current_dir: impl Into<SystemProperty>,
        library_path: impl Into<SystemProperty>,
    ) -> Self {
        let mut map = HashMap::with_capacity(64);

        macro_rules! prop {
            ($key:expr, $val:expr) => {
                map.insert($key, SystemProperty::from($val));
            };
        }

        // TODO these properties are not all correct
        prop!("java.version", JAVA_VERSION);
        prop!("java.vendor", "GNU Classpath");
        prop!("java.vendor.url", "https://www.gnu.org/software/classpath/");
        prop!("java.home", java_home.into()); // TODO
        prop!("java.vm.specification.version", JAVA_VM_SPEC_VERSION);
        prop!("java.vm.specification.vendor", "Oracle America, Inc");
        prop!(
            "java.vm.specification.name",
            "The Java® Virtual Machine Specification"
        );
        prop!("java.vm.version", env!("CARGO_PKG_VERSION"));
        prop!("java.vm.vendor", env!("CARGO_PKG_AUTHORS"));
        prop!("java.vm.name", "UntitledJvm");
        prop!("java.specification.version", "TODO"); // TODO get from Configuration class?
        prop!("java.specification.vendor", JAVA_SPEC_VERSION);
        prop!("java.specification.name", "Oracle America, Inc");
        prop!("java.class.version", CLASS_VERSION);
        prop!("java.library.path", library_path.into()); // TODO
        prop!("java.io.tmpdir", tmp_dir.into());
        prop!("java.compiler", "N/A");
        prop!("java.ext.dirs", "."); // TODO
        prop!("os.name", std::env::consts::OS);
        prop!("os.arch", std::env::consts::ARCH);
        prop!("os.version", distro);
        prop!("file.separator", std::path::MAIN_SEPARATOR.to_string());
        prop!("path.separator", ":");
        prop!("line.separator", if cfg!(windows) { "\r\n" } else { "\n" });
        prop!("user.name", username);
        prop!("user.home", home_dir.into());
        prop!("user.dir", current_dir.into());

        SystemProperties(map)
    }

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

impl From<PathBuf> for SystemProperty {
    fn from(p: PathBuf) -> Self {
        Self(match p.to_string_lossy() {
            Cow::Borrowed(p) => Cow::Owned(p.to_owned()),
            Cow::Owned(p) => Cow::Owned(p),
        })
    }
}

impl From<Option<PathBuf>> for SystemProperty {
    fn from(p: Option<PathBuf>) -> Self {
        p.unwrap_or_else(PathBuf::new).into()
    }
}

impl AsRef<str> for SystemProperty {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
