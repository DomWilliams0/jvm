use itertools::Itertools;
use std::path::PathBuf;

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
