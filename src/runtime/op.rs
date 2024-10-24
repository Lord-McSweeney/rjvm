use super::class::Class;
use super::context::Context;
use super::descriptor::Descriptor;
use super::error::{Error, NativeError};

use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::Trace;
use crate::string::JvmString;

pub enum Op {
    GetStatic(Class, (JvmString, Descriptor)),
}

impl Trace for Op {
    fn trace(&self) {
        match self {
            Op::GetStatic(class, (field_name, descriptor)) => {
                class.trace();
                field_name.trace();
                descriptor.trace();
            }
        }
    }
}

const GET_STATIC: u8 = 0xB2;

impl Op {
    pub fn read_from(
        context: Context,
        constant_pool: &ConstantPool,
        data: &mut FileData,
    ) -> Result<Self, Error> {
        let opcode = data.read_u8()?;
        match opcode {
            GET_STATIC => {
                let field_ref_idx = data.read_u16()?;
                let field_ref = constant_pool.get_field_ref(field_ref_idx)?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = context.lookup_class(class_name)?;
                let descriptor = Descriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                Ok(Op::GetStatic(class, (field_name, descriptor)))
            }
            other => unimplemented!("Unimplemented opcode {}", other),
        }
    }
}
