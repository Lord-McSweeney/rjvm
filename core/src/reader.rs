use alloc::string::String;
use alloc::vec::Vec;

pub struct FileData<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> FileData<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        FileData { data, position: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub trait Reader {
    fn read_u8(&mut self) -> Result<u8, ReadError>;

    fn read_u16_be(&mut self) -> Result<u16, ReadError>;
    fn read_u32_be(&mut self) -> Result<u32, ReadError>;
    fn read_u16_le(&mut self) -> Result<u16, ReadError>;
    fn read_u32_le(&mut self) -> Result<u32, ReadError>;

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, ReadError>;
    fn read_string(&mut self, length: usize) -> Result<String, ReadError>;

    fn position(&self) -> usize;
    fn seek(&mut self, position: usize) -> Result<(), ReadError>;
}

impl Reader for FileData<'_> {
    fn read_u8(&mut self) -> Result<u8, ReadError> {
        let pos = self.position;

        self.position += 1;

        if self.position > self.data.len() {
            Err(ReadError::EndOfFile)
        } else {
            Ok(self.data[pos])
        }
    }

    fn read_u16_be(&mut self) -> Result<u16, ReadError> {
        Ok(((self.read_u8()? as u16) << 8) + (self.read_u8()? as u16))
    }

    fn read_u32_be(&mut self) -> Result<u32, ReadError> {
        Ok(((self.read_u8()? as u32) << 24)
            + ((self.read_u8()? as u32) << 16)
            + ((self.read_u8()? as u32) << 8)
            + (self.read_u8()? as u32))
    }

    fn read_u16_le(&mut self) -> Result<u16, ReadError> {
        Ok((self.read_u8()? as u16) + ((self.read_u8()? as u16) << 8))
    }

    fn read_u32_le(&mut self) -> Result<u32, ReadError> {
        Ok((self.read_u8()? as u32)
            + ((self.read_u8()? as u32) << 8)
            + ((self.read_u8()? as u32) << 16)
            + ((self.read_u8()? as u32) << 24))
    }

    #[inline(never)]
    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, ReadError> {
        if self.position + count > self.len() {
            return Err(ReadError::EndOfFile);
        }

        // This `map` and `collect` result in much better codegen than a loop-
        // make sure to benchmark/check codegen before changing this
        let bytes = (0..count).map(|e| self.data[self.position + e]).collect::<Vec<_>>();
        self.position += count;

        Ok(bytes)
    }

    fn read_string(&mut self, length: usize) -> Result<String, ReadError> {
        let bytes = self.read_bytes(length)?;

        String::from_utf8(bytes).map_err(|_| ReadError::InvalidString)
    }

    fn position(&self) -> usize {
        self.position
    }

    fn seek(&mut self, position: usize) -> Result<(), ReadError> {
        if position >= self.data.len() {
            return Err(ReadError::EndOfFile);
        }

        self.position = position;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ReadError {
    EndOfFile,
    InvalidMagic,
    InvalidString,
}
