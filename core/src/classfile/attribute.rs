use super::constant_pool::ConstantPool;
use super::error::Error;

use crate::gc::Trace;
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::vec::Vec;

#[derive(Clone)]
pub struct Attribute {
    name: JvmString,
    data: Vec<u8>,
}

impl Attribute {
    pub fn read_from(data: &mut FileData<'_>, constant_pool: &ConstantPool) -> Result<Self, Error> {
        let name_idx = data.read_u16_be()?;
        let name = constant_pool.get_utf8(name_idx)?;

        let length = data.read_u32_be()?;
        let data = data.read_bytes(length as usize)?;

        Ok(Self { name, data })
    }

    pub fn name(&self) -> JvmString {
        self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Trace for Attribute {
    fn trace(&self) {
        self.name.trace();
    }
}
