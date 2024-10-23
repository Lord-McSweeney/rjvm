use super::error::Error;
use super::reader::{FileData, Reader};

use crate::gc::GcCtx;
use crate::string::JvmString;

const UTF8: u8 = 1;
const CLASS: u8 = 7;
const STRING: u8 = 8;
const FIELD_REF: u8 = 9;
const METHOD_REF: u8 = 10;
const NAME_AND_TYPE: u8 = 12;

pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    /// Ensure that constant pool entries point to other entries of the correct type.
    fn validate(&self) -> Result<(), Error> {
        for entry in &self.entries {
            match *entry {
                // Utf8 has no checks on it
                ConstantPoolEntry::Utf8 { .. } => {}

                // Class must point to a Utf8
                ConstantPoolEntry::Class { name_idx } => {
                    self.ensure_entry_type(name_idx, UTF8)?;
                }

                // String must point to a Utf8
                ConstantPoolEntry::String { string_idx } => {
                    self.ensure_entry_type(string_idx, UTF8)?;
                }

                // FieldRef and MethodRef must point to a Class and NameAndType
                ConstantPoolEntry::FieldRef {
                    class_idx,
                    name_and_type_idx,
                }
                | ConstantPoolEntry::MethodRef {
                    class_idx,
                    name_and_type_idx,
                } => {
                    self.ensure_entry_type(class_idx, CLASS)?;
                    self.ensure_entry_type(name_and_type_idx, NAME_AND_TYPE)?;
                }

                // NameAndType must point to a Utf8 and Utf8
                ConstantPoolEntry::NameAndType {
                    name_idx,
                    descriptor_idx,
                } => {
                    self.ensure_entry_type(name_idx, UTF8)?;
                    self.ensure_entry_type(descriptor_idx, UTF8)?;
                }
            }
        }

        Ok(())
    }

    fn ensure_entry_type(&self, index: u16, tag: u8) -> Result<(), Error> {
        if self.entry(index)?.tag() != tag {
            Err(Error::ConstantPoolTypeMismatch)
        } else {
            Ok(())
        }
    }

    pub fn entry(&self, index: u16) -> Result<ConstantPoolEntry, Error> {
        if index == 0 {
            Err(Error::ExpectedNonZero)
        } else {
            Ok(self.entries[index as usize - 1])
        }
    }

    pub fn get_class(&self, index: u16) -> Result<JvmString, Error> {
        match self.entry(index)? {
            ConstantPoolEntry::Class { name_idx } => {
                let entry = self.entry(name_idx)?;

                let ConstantPoolEntry::Utf8 { string } = entry else {
                    // Guaranteed by validation
                    unreachable!();
                };

                Ok(string)
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }

    pub fn get_utf8(&self, index: u16) -> Result<JvmString, Error> {
        match self.entry(index)? {
            ConstantPoolEntry::Utf8 { string } => Ok(string),
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ConstantPoolEntry {
    Utf8 {
        string: JvmString,
    },
    Class {
        name_idx: u16,
    },
    String {
        string_idx: u16,
    },
    FieldRef {
        class_idx: u16,
        name_and_type_idx: u16,
    },
    MethodRef {
        class_idx: u16,
        name_and_type_idx: u16,
    },
    NameAndType {
        name_idx: u16,
        descriptor_idx: u16,
    },
}

impl ConstantPoolEntry {
    fn tag(self) -> u8 {
        match self {
            ConstantPoolEntry::Utf8 { .. } => UTF8,
            ConstantPoolEntry::Class { .. } => CLASS,
            ConstantPoolEntry::String { .. } => STRING,
            ConstantPoolEntry::FieldRef { .. } => FIELD_REF,
            ConstantPoolEntry::MethodRef { .. } => METHOD_REF,
            ConstantPoolEntry::NameAndType { .. } => NAME_AND_TYPE,
        }
    }
}

fn read_constant_pool_entry(
    gc_ctx: &GcCtx,
    data: &mut FileData,
) -> Result<ConstantPoolEntry, Error> {
    let tag = data.read_u8()?;
    match tag {
        UTF8 => {
            let length = data.read_u16()?;

            let string = data.read_string(length as usize)?;
            let string = JvmString::new(gc_ctx, string);

            Ok(ConstantPoolEntry::Utf8 { string })
        }
        CLASS => {
            let name_idx = data.read_u16()?;

            Ok(ConstantPoolEntry::Class { name_idx })
        }
        STRING => {
            let string_idx = data.read_u16()?;

            Ok(ConstantPoolEntry::String { string_idx })
        }
        FIELD_REF => {
            let class_idx = data.read_u16()?;
            let name_and_type_idx = data.read_u16()?;

            Ok(ConstantPoolEntry::FieldRef {
                class_idx,
                name_and_type_idx,
            })
        }
        METHOD_REF => {
            let class_idx = data.read_u16()?;
            let name_and_type_idx = data.read_u16()?;

            Ok(ConstantPoolEntry::MethodRef {
                class_idx,
                name_and_type_idx,
            })
        }
        NAME_AND_TYPE => {
            let name_idx = data.read_u16()?;
            let descriptor_idx = data.read_u16()?;

            Ok(ConstantPoolEntry::NameAndType {
                name_idx,
                descriptor_idx,
            })
        }
        _ => unimplemented!("Constant pool entry type: {}", tag),
    }
}

pub fn read_constant_pool(gc_ctx: &GcCtx, data: &mut FileData) -> Result<ConstantPool, Error> {
    let entry_count = match data.read_u16()? {
        0 => return Err(Error::ExpectedNonZero),
        entry_count => entry_count - 1,
    };

    let mut entries = Vec::with_capacity(entry_count as usize);

    for _ in 0..entry_count {
        let entry = read_constant_pool_entry(gc_ctx, data)?;

        entries.push(entry);
    }

    let constant_pool = ConstantPool { entries };
    constant_pool.validate()?;

    Ok(constant_pool)
}
