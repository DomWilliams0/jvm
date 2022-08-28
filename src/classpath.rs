use log::*;
use std::path::PathBuf;

use itertools::Itertools;

// TODO enum for path type, zip/jar or directory

#[cfg(feature = "miri")]
mod classpath_zip {
    use crate::classpath::FindClassError;
    use lazy_static::lazy_static;
    use parking_lot::Mutex;
    use rc_zip::prelude::ReadAt;
    use rc_zip::{Archive, EntryContents, ReadZipWithSize};
    use std::ffi::OsStr;
    use std::io::Cursor;
    use std::io::Read;
    use std::path::Path;

    const CLASSPATH_ZIP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classpath.zip"));

    static ARCHIVE: Mutex<Option<Archive>> = parking_lot::const_mutex(None);

    pub fn read_file(path: &Path) -> Option<Result<Vec<u8>, std::io::Error>> {
        let mut guard = ARCHIVE.lock();
        if guard.is_none() {
            log::debug!("parsing classpath zip");
            let archive: Archive = (&CLASSPATH_ZIP as &dyn ReadAt)
                .read_zip_with_size(CLASSPATH_ZIP.len() as u64)
                .expect("failed to read classpath zip");
            log::debug!("done!");
            *guard = Some(archive);
        }

        let archive = guard.as_mut().unwrap(); // definitely initialized now

        let ret = if let Some(entry) = archive.by_name(&*path.to_string_lossy()) {
            let mut reader = entry.reader(|offset| {
                let mut cursor = Cursor::new(CLASSPATH_ZIP);
                cursor.set_position(offset as u64);
                cursor
            });
            let mut bytes = Vec::with_capacity(entry.uncompressed_size as usize);
            Some(reader.read_to_end(&mut bytes).map(|_| bytes))
        } else {
            None
        };

        ret
    }
}

#[derive(Default, Debug)]
pub struct ClassPath(Vec<PathBuf>);

pub enum FindClassError {
    NotFound,
    Io(std::io::Error),
}

impl ClassPath {
    pub fn new(classpath: Vec<PathBuf>) -> Self {
        Self(classpath)
    }

    pub fn from_colon_separated(classpath: &str) -> Self {
        Self(classpath.split(':').map(PathBuf::from).collect())
    }

    pub fn find(&self, class_name: &str) -> Option<PathBuf> {
        self.0.iter().find_map(|dir| {
            let mut file = dir.join(class_name);
            file.set_extension("class");

            #[cfg(feature = "miri")]
            {
                classpath_zip::read_file(&*file.as_path())
                    .map(|opt| opt.map_err(FindClassError::Io))
            }

            #[cfg(not(feature = "miri"))]
            {
                trace!("checking {}", file.display());
                if file.is_file() {
                    trace!("found class at {}", file.display());
                    Some(file)
                } else {
                    None
                }
            }
        })
    }

    pub fn find_and_load(&self, class_name: &str) -> Result<Vec<u8>, FindClassError> {
        self.find(class_name)
            .map(|file| std::fs::read(file).map_err(FindClassError::Io))
            .unwrap_or(Err(FindClassError::NotFound))
    }
}

impl ToString for ClassPath {
    fn to_string(&self) -> String {
        self.0.iter().map(|path| path.display()).join(":")
    }
}
