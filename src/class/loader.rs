use std::path::PathBuf;

pub struct ClassLoader {
    classpath: Vec<PathBuf>,
}

impl ClassLoader {
    pub fn new(classpath: Vec<PathBuf>) -> Self {
        Self {
            classpath,
        }
    }

    pub fn find(&self, class_name: &str) -> Option<PathBuf> {

        self.classpath.iter()
            .find_map(|dir| {
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