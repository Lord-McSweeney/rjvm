use super::error::Error;

pub struct FileData {
    data: Vec<u8>,
    position: usize,
}

impl FileData {
    pub fn new(data: Vec<u8>) -> Self {
        FileData { data, position: 0 }
    }
}

pub trait Reader {
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_u16(&mut self) -> Result<u16, Error>;
    fn read_u32(&mut self) -> Result<u32, Error>;

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, Error>;
    fn read_string(&mut self, length: usize) -> Result<String, Error>;

    fn seek(&mut self, pos: usize);
    fn seek_relative(&mut self, amount: isize);
    fn position(&self) -> usize;

    fn is_at_end(&self) -> bool;
}

impl Reader for FileData {
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

    fn seek(&mut self, pos: usize) {
        self.position = pos;
    }

    fn seek_relative(&mut self, amount: isize) {
        if amount >= 0 {
            self.position += amount as usize;
        } else {
            self.position -= (-amount) as usize;
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn is_at_end(&self) -> bool {
        self.position == self.data.len()
    }
}
