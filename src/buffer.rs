use crate::error::DecodeError;

pub struct Buffer {
    inner: &'static [u8],
    cursor: usize,
}

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Self {
        let c: &[u8] = buf.leak();
        Self {
            inner: c,
            cursor: 0,
        }
    }

    pub fn read_n(&mut self, count: usize) -> Result<&[u8], DecodeError> {
        if self.inner.len() < self.cursor + count {
            println!(
                "Will not be able to read {} bytes - throwing an error",
                count
            );
            return Err(DecodeError::UnexpectedEndOfFile);
        }

        let end = self.cursor + count;
        let c = &self.inner[self.cursor..end];
        self.cursor = end;

        Ok(c)
    }

    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        let mut res: [u8; 4] = Default::default();
        let bytes = self.read_n(4)?;

        res.copy_from_slice(&bytes[0..4]);

        Ok(u32::from_be_bytes(res))
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let byte = self.read_n(1)?[0];
        Ok(byte)
    }
}
