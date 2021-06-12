use cafebabe::mutf8::mstr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub struct ClassLoadGraph(Box<dyn Write>);

impl ClassLoadGraph {
    pub fn with_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let write = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.as_ref())?;
        let mut this = ClassLoadGraph(Box::new(write));
        this.init()?;
        Ok(this)
    }

    fn init(&mut self) -> std::io::Result<()> {
        self.0.write_all(b"digraph {\n")
    }

    pub fn register_class_load(&mut self, class: &mstr, initiator: Option<&mstr>) {
        let _ = match initiator {
            Some(parent) => writeln!(&mut self.0, "{:?} -> {:?}\n", parent, class),
            None => writeln!(&mut self.0, "{:?}\n", class),
        };
        // TODO log IO error
    }
}

impl Drop for ClassLoadGraph {
    fn drop(&mut self) {
        let _ = self.0.write_all(b"\n}");
    }
}
