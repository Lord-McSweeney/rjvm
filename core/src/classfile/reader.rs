use super::error::Error;

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
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_u16(&mut self) -> Result<u16, Error>;
    fn read_u32(&mut self) -> Result<u32, Error>;

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, Error>;
    fn read_string(&mut self, length: usize) -> Result<String, Error>;

    fn position(&self) -> usize;
}

impl Reader for FileData<'_> {
    fn read_u8(&mut self) -> Result<u8, Error> {
        let pos = self.position;

        self.position += 1;

        if self.position > self.data.len() {
            Err(Error::EndOfFile)
        } else {
            Ok(self.data[pos])
        }
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(((self.read_u8()? as u16) << 8) + (self.read_u8()? as u16))
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(((self.read_u8()? as u32) << 24)
            + ((self.read_u8()? as u32) << 16)
            + ((self.read_u8()? as u32) << 8)
            + (self.read_u8()? as u32))
    }

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        for _ in 0..count {
            if self.position == self.data.len() {
                return Err(Error::EndOfFile);
            }

            bytes.push(self.read_u8()?);
        }

        Ok(bytes)
    }

    fn read_string(&mut self, length: usize) -> Result<String, Error> {
        let bytes = self.read_bytes(length)?;

        String::from_utf8(bytes).map_err(|_| Error::InvalidString)
    }

    fn position(&self) -> usize {
        self.position
    }
}
