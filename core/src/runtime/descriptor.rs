// TODO this assumes descriptors are ASCII, but that doesn't seem to be guaranteed

use super::class::Class;
use super::context::Context;
use super::error::Error;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Descriptor {
    Class(JvmString),
    Array(Gc<Descriptor>),
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
    // TODO: This function allocates and should not be used in hot code
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

                Descriptor::Array(Gc::new(gc_ctx, inner.0))
            }
            _ => return None,
        };

        Some((result, consumed_bytes))
    }

    pub fn from_string(gc_ctx: GcCtx, descriptor: JvmString) -> Option<Self> {
        Self::from_data_counting(gc_ctx, descriptor.as_bytes(), false).map(|o| o.0)
    }

    pub fn default_value(self) -> Value {
        match self {
            Descriptor::Class(_) | Descriptor::Array(_) => Value::Object(None),
            Descriptor::Boolean => Value::Integer(0),
            Descriptor::Byte => Value::Integer(0),
            Descriptor::Character => Value::Integer(0),
            Descriptor::Double => Value::Double(0.0),
            Descriptor::Integer => Value::Integer(0),
            Descriptor::Long => Value::Long(0),
            _ => unimplemented!(),
        }
    }

    pub fn to_string(self) -> String {
        let mut result = String::with_capacity(8);

        match self {
            Descriptor::Class(class_name) => {
                result.push('L');
                result.push_str(&class_name);
                result.push(';');
            }
            Descriptor::Array(inner_descriptor) => {
                result.push('[');
                result.push_str(&inner_descriptor.to_string());
            }
            Descriptor::Byte => result.push('B'),
            Descriptor::Character => result.push('C'),
            Descriptor::Double => result.push('D'),
            Descriptor::Float => result.push('F'),
            Descriptor::Integer => result.push('I'),
            Descriptor::Long => result.push('J'),
            Descriptor::Short => result.push('S'),
            Descriptor::Boolean => result.push('Z'),
            Descriptor::Void => result.push('V'),
        }

        result
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolvedDescriptor {
    Class(Class),
    Array(Class),
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

impl ResolvedDescriptor {
    pub fn from_descriptor(context: Context, descriptor: Descriptor) -> Result<Self, Error> {
        Ok(match descriptor {
            Descriptor::Class(class_name) => {
                let class = context.lookup_class(class_name)?;

                ResolvedDescriptor::Class(class)
            }
            Descriptor::Array(inner_descriptor) => {
                let inner_resolved =
                    ResolvedDescriptor::from_descriptor(context, *inner_descriptor)?;
                let class = context.array_class_for(inner_resolved);

                ResolvedDescriptor::Array(class)
            }
            Descriptor::Boolean => ResolvedDescriptor::Boolean,
            Descriptor::Byte => ResolvedDescriptor::Byte,
            Descriptor::Character => ResolvedDescriptor::Character,
            Descriptor::Double => ResolvedDescriptor::Double,
            Descriptor::Float => ResolvedDescriptor::Float,
            Descriptor::Integer => ResolvedDescriptor::Integer,
            Descriptor::Long => ResolvedDescriptor::Long,
            Descriptor::Short => ResolvedDescriptor::Short,
            Descriptor::Void => ResolvedDescriptor::Void,
        })
    }

    pub fn is_primitive(self) -> bool {
        match self {
            ResolvedDescriptor::Class(_) => false,
            ResolvedDescriptor::Array(_) => false,
            ResolvedDescriptor::Byte => true,
            ResolvedDescriptor::Character => true,
            ResolvedDescriptor::Double => true,
            ResolvedDescriptor::Float => true,
            ResolvedDescriptor::Integer => true,
            ResolvedDescriptor::Long => true,
            ResolvedDescriptor::Short => true,
            ResolvedDescriptor::Boolean => true,
            ResolvedDescriptor::Void => unreachable!(),
        }
    }

    pub fn to_string(self) -> String {
        let mut result = String::with_capacity(8);

        match self {
            ResolvedDescriptor::Class(class) => {
                result.push('L');
                result.push_str(&class.name());
                result.push(';');
            }
            ResolvedDescriptor::Array(class) => {
                result.push_str(&class.name());
            }
            ResolvedDescriptor::Byte => result.push('B'),
            ResolvedDescriptor::Character => result.push('C'),
            ResolvedDescriptor::Double => result.push('D'),
            ResolvedDescriptor::Float => result.push('F'),
            ResolvedDescriptor::Integer => result.push('I'),
            ResolvedDescriptor::Long => result.push('J'),
            ResolvedDescriptor::Short => result.push('S'),
            ResolvedDescriptor::Boolean => result.push('Z'),
            ResolvedDescriptor::Void => result.push('V'),
        }

        result
    }

    pub fn class(self) -> Option<Class> {
        match self {
            ResolvedDescriptor::Class(class) | ResolvedDescriptor::Array(class) => Some(class),
            _ => None,
        }
    }
}

impl Trace for ResolvedDescriptor {
    fn trace(&self) {
        match self {
            ResolvedDescriptor::Class(class) | ResolvedDescriptor::Array(class) => class.trace(),
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug, Eq, Hash, PartialEq)]
struct MethodDescriptorData {
    args: Box<[Descriptor]>,
    return_type: Descriptor,
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

    pub fn args(&self) -> &[Descriptor] {
        &self.0.args
    }

    pub fn return_type(self) -> Descriptor {
        self.0.return_type
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
