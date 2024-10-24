// TODO this assumes descriptors are ASCII, but that doesn't seem to be guaranteed

use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Descriptor {
    Class(JvmString),
    Array(Box<Descriptor>),
    Boolean,
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    Short,
    Void,
}

impl Descriptor {
    fn from_data_counting(
        gc_ctx: GcCtx,
        descriptor: &[u8],
        void_allowed: bool,
    ) -> Option<(Descriptor, usize)> {
        let mut consumed_bytes = 1;

        let result = match descriptor[0] {
            b'L' => {
                let mut class_name = String::with_capacity(24);
                loop {
                    if consumed_bytes >= descriptor.len() {
                        return None;
                    }

                    if descriptor[consumed_bytes] == b';' {
                        consumed_bytes += 1;
                        break;
                    }

                    class_name.push(descriptor[consumed_bytes] as char);

                    consumed_bytes += 1;
                }

                Descriptor::Class(JvmString::new(gc_ctx, class_name))
            }
            b'B' => Descriptor::Byte,
            b'C' => Descriptor::Character,
            b'D' => Descriptor::Double,
            b'F' => Descriptor::Float,
            b'I' => Descriptor::Integer,
            b'J' => Descriptor::Long,
            b'S' => Descriptor::Short,
            b'Z' => Descriptor::Boolean,
            b'V' if void_allowed => Descriptor::Void,
            b'[' => {
                let inner = Descriptor::from_data_counting(gc_ctx, &descriptor[1..], false)?;
                consumed_bytes += inner.1;

                Descriptor::Array(Box::new(inner.0))
            }
            _ => return None,
        };

        Some((result, consumed_bytes))
    }

    pub fn from_string(gc_ctx: GcCtx, descriptor: JvmString) -> Option<Self> {
        Self::from_data_counting(gc_ctx, descriptor.as_bytes(), false).map(|o| o.0)
    }
}

impl Trace for Descriptor {
    fn trace(&self) {
        match self {
            Descriptor::Class(name) => name.trace(),
            Descriptor::Array(inner_desc) => inner_desc.trace(),
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
pub struct MethodDescriptor(Gc<MethodDescriptorData>);

impl PartialEq for MethodDescriptor {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Eq for MethodDescriptor {}

impl Hash for MethodDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.0).hash(state);
    }
}

#[derive(Eq, Hash, PartialEq)]
struct MethodDescriptorData {
    pub args: Box<[Descriptor]>,
    pub return_type: Descriptor,
}

impl MethodDescriptor {
    pub fn from_string(gc_ctx: GcCtx, descriptor: JvmString) -> Option<Self> {
        let desc_bytes = descriptor.as_bytes();

        if desc_bytes.len() == 0 || desc_bytes[0] != b'(' {
            return None;
        }

        let mut args = Vec::with_capacity(2);
        let return_type;

        // Start from 1 to skip over the extra '(' at the beginning of every descriptor
        let mut i = 1;
        loop {
            if i >= desc_bytes.len() {
                return None;
            }

            match desc_bytes[i] {
                b')' => {
                    i += 1;

                    return_type = Descriptor::from_data_counting(gc_ctx, &desc_bytes[i..], true)?.0;
                    break;
                }
                _ => {
                    let arg_desc = Descriptor::from_data_counting(gc_ctx, &desc_bytes[i..], false)?;
                    i += arg_desc.1 - 1;

                    args.push(arg_desc.0);
                }
            }

            i += 1;
        }

        Some(Self(Gc::new(
            gc_ctx,
            MethodDescriptorData {
                args: args.into_boxed_slice(),
                return_type,
            },
        )))
    }
}

impl Trace for MethodDescriptor {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for MethodDescriptorData {
    fn trace(&self) {
        self.args.trace();
        self.return_type.trace();
    }
}
