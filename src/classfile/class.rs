use super::constant_pool::{read_constant_pool, ConstantPool};
use super::error::Error;
use super::reader::{FileData, Reader};
use crate::gc::GcCtx;

pub struct ClassFile {
    constant_pool: ConstantPool,
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

    Ok(ClassFile { constant_pool })
}
