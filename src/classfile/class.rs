use super::attribute::Attribute;
use super::constant_pool::{read_constant_pool, ConstantPool};
use super::error::Error;
use super::field::Field;
use super::flags::ClassFlags;
use super::method::Method;
use super::reader::{FileData, Reader};

use crate::gc::GcCtx;
use crate::string::JvmString;

pub struct ClassFile {
    constant_pool: ConstantPool,

    flags: ClassFlags,

    this_class: JvmString,
    super_class: Option<JvmString>,

    interfaces: Box<[JvmString]>,

    fields: Box<[Field]>,
    methods: Box<[Method]>,
    attributes: Box<[Attribute]>,
}

pub fn read_class(gc_ctx: &GcCtx, data: Vec<u8>) -> Result<ClassFile, Error> {
    let mut reader = FileData::new(data);

    let magic = reader.read_u32()?;
    if magic != 0xCAFEBABE {
        return Err(Error::MagicMismatch);
    }

    let _minor_version = reader.read_u16()?;
    let _major_version = reader.read_u16()?;

    let constant_pool = read_constant_pool(gc_ctx, &mut reader)?;

    let flag_bits = reader.read_u16()?;
    let flags = ClassFlags::from_bits_truncate(flag_bits);

    // Read this-class name
    let this_class_idx = reader.read_u16()?;
    let this_class = constant_pool.get_class(this_class_idx)?;

    // Read superclass name
    let super_class_idx = reader.read_u16()?;
    let super_class = if super_class_idx == 0 {
        None
    } else {
        Some(constant_pool.get_class(super_class_idx)?)
    };

    let interface_count = reader.read_u16()?;
    let mut interface_list = Vec::with_capacity(interface_count as usize);
    for _ in 0..interface_count {
        let interface_idx = reader.read_u16()?;
        let interface = constant_pool.get_class(interface_idx)?;

        interface_list.push(interface);
    }

    let field_count = reader.read_u16()?;
    let mut field_list = Vec::with_capacity(field_count as usize);
    for _ in 0..field_count {
        field_list.push(Field::read_from(&mut reader, &constant_pool)?);
    }

    let method_count = reader.read_u16()?;
    let mut method_list = Vec::with_capacity(method_count as usize);
    for _ in 0..method_count {
        method_list.push(Method::read_from(&mut reader, &constant_pool)?);
    }

    let attribute_count = reader.read_u16()?;
    let mut attribute_list = Vec::with_capacity(attribute_count as usize);
    for _ in 0..attribute_count {
        attribute_list.push(Attribute::read_from(&mut reader, &constant_pool)?);
    }

    Ok(ClassFile {
        constant_pool,
        flags,
        this_class,
        super_class,
        interfaces: interface_list.into_boxed_slice(),
        fields: field_list.into_boxed_slice(),
        methods: method_list.into_boxed_slice(),
        attributes: attribute_list.into_boxed_slice(),
    })
}
