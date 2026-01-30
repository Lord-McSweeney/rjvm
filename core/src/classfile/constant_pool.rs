use super::error::Error;

use crate::gc::{GcCtx, Trace};
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::vec::Vec;

const PLACEHOLDER: u8 = 0;
const UTF8: u8 = 1;
const INTEGER: u8 = 3;
const FLOAT: u8 = 4;
const LONG: u8 = 5;
const DOUBLE: u8 = 6;
const CLASS: u8 = 7;
const STRING: u8 = 8;
const FIELD_REF: u8 = 9;
const METHOD_REF: u8 = 10;
const INTERFACE_METHOD_REF: u8 = 11;
const NAME_AND_TYPE: u8 = 12;
const METHOD_HANDLE: u8 = 15;
const METHOD_TYPE: u8 = 16;
const INVOKE_DYNAMIC: u8 = 18;

pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    /// Ensure that constant pool entries point to other entries of the correct type.
    fn validate(&self) -> Result<(), Error> {
        for entry in &self.entries {
            match *entry {
                // Placeholders have no checks on them
                ConstantPoolEntry::Placeholder => {}

                // Utf8 has no checks on it
                ConstantPoolEntry::Utf8 { .. } => {}

                // Integer has no checks on it
                ConstantPoolEntry::Integer { .. } => {}

                // Float has no checks on it
                ConstantPoolEntry::Float { .. } => {}

                // Long has no checks on it
                ConstantPoolEntry::Long { .. } => {}

                // Double has no checks on it
                ConstantPoolEntry::Double { .. } => {}

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
                }
                | ConstantPoolEntry::InterfaceMethodRef {
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

                // MethodHandle has some complicated rules for verification, see
                // JVMS 4
                ConstantPoolEntry::MethodHandle { method_handle } => match method_handle {
                    MethodHandle::GetField(cpool_idx)
                    | MethodHandle::GetStatic(cpool_idx)
                    | MethodHandle::PutField(cpool_idx)
                    | MethodHandle::PutStatic(cpool_idx) => {
                        self.ensure_entry_type(cpool_idx, FIELD_REF)?;
                    }
                    MethodHandle::InvokeVirtual(cpool_idx) => {
                        self.ensure_entry_type(cpool_idx, METHOD_REF)?;

                        let method_name = self
                            .get_method_ref(cpool_idx)
                            .expect("Just checked it to be MethodRef")
                            .1;

                        if *method_name == "<init>" || *method_name == "<clinit>" {
                            return Err(Error::ConstantPoolVerifyError);
                        }
                    }
                    MethodHandle::InvokeStatic(cpool_idx)
                    | MethodHandle::InvokeSpecial(cpool_idx) => {
                        // NOTE JVMS is wrong here- it says it needs to be a
                        // MethodRef, but javac can also generate an
                        // InterfaceMethodRef
                        let tag = self.entry(cpool_idx)?.tag();
                        if tag != METHOD_REF && tag != INTERFACE_METHOD_REF {
                            return Err(Error::ConstantPoolTypeMismatch);
                        }

                        let method_name = self
                            .get_any_method_ref(cpool_idx)
                            .expect("Just checked it to be correct")
                            .1;

                        if *method_name == "<init>" || *method_name == "<clinit>" {
                            return Err(Error::ConstantPoolVerifyError);
                        }
                    }
                    MethodHandle::NewInvokeSpecial(cpool_idx) => {
                        self.ensure_entry_type(cpool_idx, METHOD_REF)?;

                        let method_name = self
                            .get_method_ref(cpool_idx)
                            .expect("Just checked it to be MethodRef")
                            .1;

                        if *method_name != "<init>" {
                            return Err(Error::ConstantPoolVerifyError);
                        }
                    }
                    MethodHandle::InvokeInterface(cpool_idx) => {
                        self.ensure_entry_type(cpool_idx, INTERFACE_METHOD_REF)?;

                        let interface_method_name = self
                            .get_interface_method_ref(cpool_idx)
                            .expect("Just checked it to be InterfaceMethodRef")
                            .1;

                        if *interface_method_name == "<init>"
                            || *interface_method_name == "<clinit>"
                        {
                            return Err(Error::ConstantPoolVerifyError);
                        }
                    }
                },

                // MethodType must point to a Utf8
                ConstantPoolEntry::MethodType { descriptor_idx } => {
                    self.ensure_entry_type(descriptor_idx, UTF8)?;
                }

                // InvokeDynamic must point to a NameAndType
                ConstantPoolEntry::InvokeDynamic {
                    name_and_type_idx, ..
                } => {
                    // Should we verify the `bootstrap_method_idx` here?
                    self.ensure_entry_type(name_and_type_idx, NAME_AND_TYPE)?;
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

    pub fn get_utf8(&self, index: u16) -> Result<JvmString, Error> {
        match self.entry(index)? {
            ConstantPoolEntry::Utf8 { string } => Ok(string),
            _ => Err(Error::ConstantPoolTypeMismatch),
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

    pub fn get_field_ref(&self, index: u16) -> Result<(JvmString, JvmString, JvmString), Error> {
        match self.entry(index)? {
            ConstantPoolEntry::FieldRef {
                class_idx,
                name_and_type_idx,
            } => {
                let class = self.get_class(class_idx)?;
                let name_and_type = self.get_name_and_type(name_and_type_idx)?;

                Ok((class, name_and_type.0, name_and_type.1))
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }

    pub fn get_method_ref(&self, index: u16) -> Result<(JvmString, JvmString, JvmString), Error> {
        match self.entry(index)? {
            ConstantPoolEntry::MethodRef {
                class_idx,
                name_and_type_idx,
            } => {
                let class = self.get_class(class_idx)?;
                let name_and_type = self.get_name_and_type(name_and_type_idx)?;

                Ok((class, name_and_type.0, name_and_type.1))
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }

    pub fn get_interface_method_ref(
        &self,
        index: u16,
    ) -> Result<(JvmString, JvmString, JvmString), Error> {
        match self.entry(index)? {
            ConstantPoolEntry::InterfaceMethodRef {
                class_idx,
                name_and_type_idx,
            } => {
                let class = self.get_class(class_idx)?;
                let name_and_type = self.get_name_and_type(name_and_type_idx)?;

                Ok((class, name_and_type.0, name_and_type.1))
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }

    /// Get either a `MethodRef` or a `InterfaceMethodRef` at the location
    pub fn get_any_method_ref(
        &self,
        index: u16,
    ) -> Result<(JvmString, JvmString, JvmString), Error> {
        match self.entry(index)? {
            ConstantPoolEntry::MethodRef {
                class_idx,
                name_and_type_idx,
            }
            | ConstantPoolEntry::InterfaceMethodRef {
                class_idx,
                name_and_type_idx,
            } => {
                let class = self.get_class(class_idx)?;
                let name_and_type = self.get_name_and_type(name_and_type_idx)?;

                Ok((class, name_and_type.0, name_and_type.1))
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }

    pub fn get_name_and_type(&self, index: u16) -> Result<(JvmString, JvmString), Error> {
        match self.entry(index)? {
            ConstantPoolEntry::NameAndType {
                name_idx,
                descriptor_idx,
            } => {
                let name = self.get_utf8(name_idx)?;
                let descriptor = self.get_utf8(descriptor_idx)?;

                Ok((name, descriptor))
            }
            _ => Err(Error::ConstantPoolTypeMismatch),
        }
    }
}

impl Trace for ConstantPool {
    fn trace(&self) {
        self.entries.trace();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ConstantPoolEntry {
    Placeholder,
    Utf8 {
        string: JvmString,
    },
    Integer {
        value: i32,
    },
    Float {
        value: f32,
    },
    Long {
        value: i64,
    },
    Double {
        value: f64,
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
    InterfaceMethodRef {
        class_idx: u16,
        name_and_type_idx: u16,
    },
    NameAndType {
        name_idx: u16,
        descriptor_idx: u16,
    },
    MethodHandle {
        method_handle: MethodHandle,
    },
    MethodType {
        descriptor_idx: u16,
    },
    InvokeDynamic {
        bootstrap_method_idx: u16,
        name_and_type_idx: u16,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum MethodHandle {
    GetField(u16),
    GetStatic(u16),
    PutField(u16),
    PutStatic(u16),
    InvokeVirtual(u16),
    InvokeStatic(u16),
    InvokeSpecial(u16),
    NewInvokeSpecial(u16),
    InvokeInterface(u16),
}

impl ConstantPoolEntry {
    fn tag(self) -> u8 {
        match self {
            ConstantPoolEntry::Placeholder { .. } => PLACEHOLDER,
            ConstantPoolEntry::Utf8 { .. } => UTF8,
            ConstantPoolEntry::Integer { .. } => INTEGER,
            ConstantPoolEntry::Float { .. } => FLOAT,
            ConstantPoolEntry::Long { .. } => LONG,
            ConstantPoolEntry::Double { .. } => DOUBLE,
            ConstantPoolEntry::Class { .. } => CLASS,
            ConstantPoolEntry::String { .. } => STRING,
            ConstantPoolEntry::FieldRef { .. } => FIELD_REF,
            ConstantPoolEntry::MethodRef { .. } => METHOD_REF,
            ConstantPoolEntry::InterfaceMethodRef { .. } => INTERFACE_METHOD_REF,
            ConstantPoolEntry::NameAndType { .. } => NAME_AND_TYPE,
            ConstantPoolEntry::MethodHandle { .. } => METHOD_HANDLE,
            ConstantPoolEntry::MethodType { .. } => METHOD_TYPE,
            ConstantPoolEntry::InvokeDynamic { .. } => INVOKE_DYNAMIC,
        }
    }
}

impl Trace for ConstantPoolEntry {
    fn trace(&self) {
        match self {
            ConstantPoolEntry::Utf8 { string } => string.trace(),
            _ => {}
        }
    }
}

fn read_constant_pool_entry(
    gc_ctx: GcCtx,
    data: &mut FileData<'_>,
) -> Result<ConstantPoolEntry, Error> {
    let tag = data.read_u8()?;
    match tag {
        UTF8 => {
            let length = data.read_u16_be()?;

            let string = data.read_string(length as usize)?;
            let string = JvmString::new(gc_ctx, string);

            Ok(ConstantPoolEntry::Utf8 { string })
        }
        INTEGER => {
            let value = data.read_u32_be()? as i32;

            Ok(ConstantPoolEntry::Integer { value })
        }
        LONG => {
            let high_value = data.read_u32_be()? as u64;
            let low_value = data.read_u32_be()? as u64;

            let value = (high_value << 32) + low_value;

            Ok(ConstantPoolEntry::Long {
                value: value as i64,
            })
        }
        FLOAT => {
            let value = data.read_u32_be()?;

            Ok(ConstantPoolEntry::Float {
                value: f32::from_bits(value),
            })
        }
        DOUBLE => {
            let high_value = data.read_u32_be()? as u64;
            let low_value = data.read_u32_be()? as u64;

            let value = (high_value << 32) + low_value;

            Ok(ConstantPoolEntry::Double {
                value: f64::from_bits(value),
            })
        }
        CLASS => {
            let name_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::Class { name_idx })
        }
        STRING => {
            let string_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::String { string_idx })
        }
        FIELD_REF => {
            let class_idx = data.read_u16_be()?;
            let name_and_type_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::FieldRef {
                class_idx,
                name_and_type_idx,
            })
        }
        METHOD_REF => {
            let class_idx = data.read_u16_be()?;
            let name_and_type_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::MethodRef {
                class_idx,
                name_and_type_idx,
            })
        }
        INTERFACE_METHOD_REF => {
            let class_idx = data.read_u16_be()?;
            let name_and_type_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::InterfaceMethodRef {
                class_idx,
                name_and_type_idx,
            })
        }
        NAME_AND_TYPE => {
            let name_idx = data.read_u16_be()?;
            let descriptor_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::NameAndType {
                name_idx,
                descriptor_idx,
            })
        }
        METHOD_HANDLE => {
            let kind = data.read_u8()?;
            let cpool_idx = data.read_u16_be()?;

            let method_handle = match kind {
                1 => MethodHandle::GetField(cpool_idx),
                2 => MethodHandle::GetStatic(cpool_idx),
                3 => MethodHandle::PutField(cpool_idx),
                4 => MethodHandle::PutStatic(cpool_idx),
                5 => MethodHandle::InvokeVirtual(cpool_idx),
                6 => MethodHandle::InvokeStatic(cpool_idx),
                7 => MethodHandle::InvokeSpecial(cpool_idx),
                8 => MethodHandle::NewInvokeSpecial(cpool_idx),
                9 => MethodHandle::InvokeInterface(cpool_idx),
                _ => return Err(Error::ConstantPoolVerifyError),
            };

            Ok(ConstantPoolEntry::MethodHandle { method_handle })
        }
        METHOD_TYPE => {
            let descriptor_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::MethodType { descriptor_idx })
        }
        INVOKE_DYNAMIC => {
            let bootstrap_method_idx = data.read_u16_be()?;
            let name_and_type_idx = data.read_u16_be()?;

            Ok(ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_idx,
                name_and_type_idx,
            })
        }
        _ => return Err(Error::ConstantPoolInvalidEntry),
    }
}

pub fn read_constant_pool(gc_ctx: GcCtx, data: &mut FileData<'_>) -> Result<ConstantPool, Error> {
    let entry_count = match data.read_u16_be()? {
        0 => return Err(Error::ExpectedNonZero),
        entry_count => entry_count - 1,
    };

    let mut entries = Vec::with_capacity(entry_count as usize);

    while entries.len() < entry_count as usize {
        let entry = read_constant_pool_entry(gc_ctx, data)?;

        entries.push(entry);

        if matches!(
            entry,
            ConstantPoolEntry::Long { .. } | ConstantPoolEntry::Double { .. }
        ) {
            // Longs and Doubles "take up" two cpool entries
            entries.push(ConstantPoolEntry::Placeholder);
        }
    }

    let constant_pool = ConstantPool { entries };
    constant_pool.validate()?;

    Ok(constant_pool)
}
