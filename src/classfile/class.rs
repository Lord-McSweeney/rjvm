use super::constant_pool::{read_constant_pool, ConstantPool, ConstantPoolEntry};
use super::error::Error;
use super::flags::ClassFlags;
use super::reader::{FileData, Reader};

use crate::gc::GcCtx;
use crate::string::JvmString;

pub struct ClassFile {
    constant_pool: ConstantPool,

    flags: ClassFlags,

    this_class_name: JvmString,
    super_class_name: Option<JvmString>,
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
    let this_class = constant_pool.entry(this_class_idx)?;

    let this_class_name = match this_class {
        ConstantPoolEntry::Class { name_idx } => {
            let entry = constant_pool.entry(name_idx)?;

            let ConstantPoolEntry::Utf8 { string } = entry else {
                // Guaranteed by validation
                unreachable!();
            };

            string
        }
        _ => return Err(Error::ConstantPoolTypeMismatch),
    };

    // Read superclass name
    let super_class_idx = reader.read_u16()?;

    let super_class_name = if super_class_idx == 0 {
        None
    } else {
        let super_class = constant_pool.entry(super_class_idx)?;

        Some(match super_class {
            ConstantPoolEntry::Class { name_idx } => {
                let entry = constant_pool.entry(name_idx)?;

                let ConstantPoolEntry::Utf8 { string } = entry else {
                    // Guaranteed by validation
                    unreachable!();
                };

                string
            }
            _ => return Err(Error::ConstantPoolTypeMismatch),
        })
    };

    Ok(ClassFile {
        constant_pool,
        flags,
        this_class_name,
        super_class_name,
    })
}
