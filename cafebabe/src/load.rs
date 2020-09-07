use crate::class::ClassFile;
use crate::error::ClassResult;

pub fn load_from_buffer(buf: &[u8]) -> ClassResult<ClassFile> {
    ClassFile::load(buf)
}
