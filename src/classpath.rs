use log::*;
use std::path::PathBuf;

use itertools::Itertools;

// TODO enum for path type, zip/jar or directory

#[derive(Default, Debug)]
pub struct ClassPath(Vec<PathBuf>);

impl ClassPath {
    pub fn new(classpath: Vec<PathBuf>) -> Self {
        Self(classpath)
    }

    pub fn find(&self, class_name: &str) -> Option<PathBuf> {
        self.0.iter().find_map(|dir| {
            let mut file = dir.join(class_name);
            file.set_extension("class");
            trace!("checking {}", file.display());
            if file.is_file() {
                Some(file)
            } else {
                None
            }
        })
    }
}

impl ToString for ClassPath {
    fn to_string(&self) -> String {
        self.0.iter().map(|path| path.display()).join(":")
    }
}
