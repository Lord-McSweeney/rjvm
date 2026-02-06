use super::attribute::Attribute;
use super::constant_pool::{ConstantPool, read_constant_pool};
use super::error::Error;
use super::field::Field;
use super::flags::ClassFlags;
use super::method::Method;

use crate::gc::{Gc, GcCtx, Trace};
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct ClassFile(Gc<ClassFileData>);

struct ClassFileData {
    constant_pool: ConstantPool,

    flags: ClassFlags,

    this_class: JvmString,
    super_class: Option<JvmString>,

    interfaces: Box<[JvmString]>,

    fields: Box<[Field]>,
    methods: Box<[Method]>,
    attributes: Box<[Attribute]>,
}

impl ClassFile {
    pub fn from_data(gc_ctx: GcCtx, data: Vec<u8>) -> Result<Self, Error> {
        let mut reader = FileData::new(&data);

        let magic = reader.read_u32_be()?;
        if magic != 0xCAFEBABE {
            return Err(Error::InvalidMagic);
        }

        let _minor_version = reader.read_u16_be()?;
        let _major_version = reader.read_u16_be()?;

        let constant_pool = read_constant_pool(gc_ctx, &mut reader)?;

        let flag_bits = reader.read_u16_be()?;
        let flags = ClassFlags::from_bits_truncate(flag_bits);

        // Read this-class name
        let this_class_idx = reader.read_u16_be()?;
        let this_class = constant_pool.get_class(this_class_idx)?;

        // Read superclass name
        let super_class_idx = reader.read_u16_be()?;
        let super_class = if super_class_idx == 0 {
            None
        } else {
            Some(constant_pool.get_class(super_class_idx)?)
        };

        let interface_count = reader.read_u16_be()?;
        let mut interface_list = Vec::with_capacity(interface_count as usize);
        for _ in 0..interface_count {
            let interface_idx = reader.read_u16_be()?;
            let interface = constant_pool.get_class(interface_idx)?;

            interface_list.push(interface);
        }

        let field_count = reader.read_u16_be()?;
        let mut field_list = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            field_list.push(Field::read_from(&mut reader, &constant_pool)?);
        }

        let method_count = reader.read_u16_be()?;
        let mut method_list = Vec::with_capacity(method_count as usize);
        for _ in 0..method_count {
            method_list.push(Method::read_from(&mut reader, &constant_pool)?);
        }

        let attribute_count = reader.read_u16_be()?;
        let mut attribute_list = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            attribute_list.push(Attribute::read_from(&mut reader, &constant_pool)?);
        }

        Ok(Self(Gc::new(
            gc_ctx,
            ClassFileData {
                constant_pool,
                flags,
                this_class,
                super_class,
                interfaces: interface_list.into_boxed_slice(),
                fields: field_list.into_boxed_slice(),
                methods: method_list.into_boxed_slice(),
                attributes: attribute_list.into_boxed_slice(),
            },
        )))
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.0.constant_pool
    }

    pub fn flags(self) -> ClassFlags {
        self.0.flags
    }

    pub fn this_class(self) -> JvmString {
        self.0.this_class
    }

    pub fn super_class(self) -> Option<JvmString> {
        self.0.super_class
    }

    pub fn interfaces(&self) -> &[JvmString] {
        &self.0.interfaces
    }

    pub fn fields(&self) -> &[Field] {
        &self.0.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.0.methods
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.0.attributes
    }
}

impl Trace for ClassFile {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for ClassFileData {
    fn trace(&self) {
        self.constant_pool.trace();
        self.this_class.trace();
        self.super_class.trace();
        self.interfaces.trace();
        self.fields.trace();
        self.methods.trace();
        self.attributes.trace();
    }
}
