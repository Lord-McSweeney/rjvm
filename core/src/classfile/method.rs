use super::attribute::Attribute;
use super::constant_pool::ConstantPool;
use super::error::Error;
use super::flags::MethodFlags;
use super::reader::{FileData, Reader};

use crate::gc::Trace;
use crate::string::JvmString;

pub struct Method {
    flags: MethodFlags,
    name: JvmString,
    descriptor: JvmString,

    attributes: Box<[Attribute]>,
}

impl Method {
    pub fn read_from(data: &mut FileData, constant_pool: &ConstantPool) -> Result<Self, Error> {
        let flag_bits = data.read_u16()?;
        let flags = MethodFlags::from_bits_truncate(flag_bits);

        let name_idx = data.read_u16()?;
        let name = constant_pool.get_utf8(name_idx)?;

        let descriptor_idx = data.read_u16()?;
        let descriptor = constant_pool.get_utf8(descriptor_idx)?;

        let attribute_count = data.read_u16()?;
        let mut attribute_list = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            attribute_list.push(Attribute::read_from(data, constant_pool)?);
        }

        Ok(Self {
            flags,
            name,
            descriptor,
            attributes: attribute_list.into_boxed_slice(),
        })
    }

    pub fn flags(&self) -> MethodFlags {
        self.flags
    }

    pub fn name(&self) -> JvmString {
        self.name
    }

    pub fn descriptor(&self) -> JvmString {
        self.descriptor
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl Trace for Method {
    fn trace(&self) {
        self.name.trace();
        self.descriptor.trace();
        self.attributes.trace();
    }
}
