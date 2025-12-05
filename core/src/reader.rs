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
}

pub trait Reader {
    fn read_u8(&mut self) -> Result<u8, ReadError>;
    fn read_u16(&mut self) -> Result<u16, ReadError>;
    fn read_u32(&mut self) -> Result<u32, ReadError>;

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, ReadError>;
    fn read_string(&mut self, length: usize) -> Result<String, ReadError>;

    fn position(&self) -> usize;
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

    fn read_u16(&mut self) -> Result<u16, ReadError> {
        Ok(((self.read_u8()? as u16) << 8) + (self.read_u8()? as u16))
    }

    fn read_u32(&mut self) -> Result<u32, ReadError> {
        Ok(((self.read_u8()? as u32) << 24)
            + ((self.read_u8()? as u32) << 16)
            + ((self.read_u8()? as u32) << 8)
            + (self.read_u8()? as u32))
    }

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, ReadError> {
        let mut bytes = Vec::new();
        for _ in 0..count {
            if self.position == self.data.len() {
                return Err(ReadError::EndOfFile);
            }

            bytes.push(self.read_u8()?);
        }

        Ok(bytes)
    }

    fn read_string(&mut self, length: usize) -> Result<String, ReadError> {
        let bytes = self.read_bytes(length)?;

        String::from_utf8(bytes).map_err(|_| ReadError::InvalidString)
    }

    fn position(&self) -> usize {
        self.position
    }
}

pub enum ReadError {
    EndOfFile,
    InvalidString,
}
