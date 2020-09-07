use crate::{ClassError, ClassResult};
use byte::ctx::Bytes;
use byte::{
    ctx::{Endian, BE},
    BytesExt, TryRead,
};
use std::convert::TryInto;

#[derive(Clone)]
pub struct Buffer<'b> {
    bytes: &'b [u8],

    /// Next byte to read
    cursor: usize,
}

impl<'b> From<&'b [u8]> for Buffer<'b> {
    fn from(buf: &'b [u8]) -> Self {
        Buffer::new(buf)
    }
}
impl<'b> Buffer<'b> {
    pub fn new(buf: &'b [u8]) -> Self {
        Self {
            bytes: buf,
            cursor: 0,
        }
    }

    pub fn read<T: TryRead<'b, Endian>>(&mut self) -> ClassResult<T> {
        self.bytes
            .read_with(&mut self.cursor, BE)
            .map_err(ClassError::Reading)
    }

    pub fn sub_buffer(&mut self, length: usize) -> ClassResult<Self> {
        let sub_buffer = Self::new(&self.bytes[self.cursor..self.cursor + length]);
        self.cursor += length;
        Ok(sub_buffer)
    }

    pub fn read_slice(&mut self, n: usize) -> ClassResult<&'b [u8]> {
        self.bytes
            .read_with(&mut self.cursor, Bytes::Len(n))
            .map_err(ClassError::Reading)
    }

    pub fn read_n_and<T, F: FnMut(&'b [u8]) -> bool>(
        &mut self,
        n: usize,
        mut f: F,
    ) -> ClassResult<()> {
        let size = std::mem::size_of::<T>();
        let len = size * n;
        let slice: &[u8] = self
            .bytes
            .read_with(&mut self.cursor, Bytes::Len(len))
            .map_err(ClassError::Reading)?;

        // can't just return the slice as endianness may be different
        for i in (0..len).step_by(size) {
            let bytes = &slice[i..i + size];
            if !f(bytes) {
                return Err(ClassError::ReadingN(std::any::type_name::<T>()));
            }
        }
        Ok(())
    }

    pub fn read_n<T, F: Fn(&'b [u8]) -> Option<T>>(
        &mut self,
        n: usize,
        f: F,
    ) -> ClassResult<Vec<T>> {
        let mut vec = Vec::with_capacity(n);
        self.read_n_and::<T, _>(n, |bytes| {
            if let Some(val) = f(bytes) {
                vec.push(val);
                true
            } else {
                false
            }
        })?;
        Ok(vec)
    }

    pub fn read_n_u16(&mut self, n: usize) -> ClassResult<Vec<u16>> {
        self.read_n(n, |bytes| {
            TryInto::<&[u8; 2]>::try_into(bytes)
                .ok()
                .map(|bytes| u16::from_be_bytes(*bytes))
        })
    }

    pub fn position(&self) -> usize {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;

    #[test]
    fn read_bytes() {
        let numbers = (0u8..10).collect::<Vec<_>>();
        let mut buf = Buffer::from(numbers.as_slice());

        let nums: Vec<_> = (0..10).map(|_| buf.read::<u8>().unwrap()).collect();
        assert_eq!(nums, numbers);

        // exhausted
        assert!(buf.read::<u8>().is_err());
    }

    #[test]
    fn read_endian() {
        let numbers = vec![0, 0, 0, 1, 0, 0, 0, 2];
        let mut buf = Buffer::from(numbers.as_slice());

        let one = buf.read::<u32>().unwrap();
        let two = buf.read::<u32>().unwrap();

        assert_eq!(one, 1);
        assert_eq!(two, 2);
    }

    #[test]
    fn read_n() {
        let numbers = vec![1, 2, 3, 4];
        let mut buf = Buffer::from(numbers.as_slice());

        let shorts: Vec<u16> = buf.read_n_u16(2).unwrap();
        assert_eq!(shorts.len(), 2);
        assert_eq!(shorts[0], 0x0102);
        assert_eq!(shorts[1], 0x0304);
    }
}
