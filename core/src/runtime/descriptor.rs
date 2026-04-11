// TODO this assumes descriptors are ASCII, but that doesn't seem to be guaranteed

use super::class::{Class, PrimitiveType};
use super::context::Context;
use super::error::Error;
use super::loader::ClassLoader;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::hash::{Hash, Hasher};

/// A parsed but not-yet-resolved descriptor.
///
/// This can be any kind of descriptor, including a field, method argument, and
/// method return type descriptor.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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

impl fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
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

                Descriptor::Array(Gc::new(gc_ctx, inner.0))
            }
            _ => return None,
        };

        Some((result, consumed_bytes))
    }

    /// Attempt to create a `Descriptor` from a [`JvmString`]. If the passed
    /// `JvmString` is not a valid descriptor, this method will return `None`.
    ///
    /// This method will return `None` for a void (`V`) descriptor.
    pub fn try_from_string(gc_ctx: GcCtx, descriptor: JvmString) -> Option<Self> {
        let result = Self::from_data_counting(gc_ctx, descriptor.as_bytes(), false);

        result
            .filter(|r| r.1 == descriptor.len()) // Trailing garbage = invalid descriptor
            .map(|r| r.0)
    }

    /// Like [`Descriptor::try_from_string`], but returns a `ClassFormatError`
    /// if the descriptor is invalid. The `ClassFormatError`'s message will
    /// state that a field signature is invalid.
    pub(crate) fn from_string(context: &Context, descriptor: JvmString) -> Result<Self, Error> {
        if let Some(result) = Self::try_from_string(context.gc_ctx, descriptor) {
            Ok(result)
        } else {
            // `MethodDescriptor::from_string` uses `try_from_string` instead of
            // this method, so this is fine
            Err(context.class_format_error(&format!("Illegal field signature \"{}\"", descriptor)))
        }
    }

    /// The default [`Value`] for a descriptor. This is the value initially
    /// stored in a field typed with this descriptor (before any Java code
    /// initializes the field).
    ///
    /// For example, when called on a `Descriptor::Class`, this method will
    /// return the [`Value`] representing `null`, and when called on a
    /// `Descriptor::Float`, this method will return the [`Value`] representing
    /// the float value `0.0`.
    pub fn default_value(self) -> Value {
        match self {
            Descriptor::Class(_) | Descriptor::Array(_) => Value::Object(None),
            Descriptor::Boolean => Value::Integer(0),
            Descriptor::Byte => Value::Integer(0),
            Descriptor::Character => Value::Integer(0),
            Descriptor::Double => Value::Double(0.0),
            Descriptor::Float => Value::Float(0.0),
            Descriptor::Integer => Value::Integer(0),
            Descriptor::Long => Value::Long(0),
            Descriptor::Short => Value::Integer(0),
            Descriptor::Void => panic!("Void descriptor cannot be used for a value"),
        }
    }

    /// Forms a `String` from this `Descriptor`.
    ///
    /// There is only one possible way to represent any `Descriptor`, so if this
    /// `Descriptor` was created using [`Descriptor::try_from_string`], this
    /// method will return the exact string that was passed to that method.
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

    /// Returns true when called on `Descriptor::Double` or `Descriptor::Long`.
    pub fn is_wide(self) -> bool {
        matches!(self, Descriptor::Double | Descriptor::Long)
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

/// A descriptor that has been both parsed and resolved.
///
/// See [`Descriptor`] for the not-yet-resolved version of this.
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
    /// Resolve a [`Descriptor`] to a `ResolvedDescriptor`.
    ///
    /// This method will return an error if the [`Descriptor`] cannot be
    /// resolved, such as if it references a [`Class`] that does not exist.
    pub fn from_descriptor(
        context: &Context,
        loader: ClassLoader,
        descriptor: Descriptor,
    ) -> Result<Self, Error> {
        Ok(match descriptor {
            Descriptor::Class(class_name) => {
                let class = loader.lookup_class(context, class_name)?;

                ResolvedDescriptor::Class(class)
            }
            Descriptor::Array(inner_descriptor) => {
                let inner_resolved =
                    ResolvedDescriptor::from_descriptor(context, loader, *inner_descriptor)?;
                let class = ClassLoader::array_class_for(context, inner_resolved);

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

    /// Like `from_descriptor`, but returns `Ok(None)` if the class fails to
    /// be looked up
    pub(crate) fn try_from_descriptor(
        context: &Context,
        loader: ClassLoader,
        descriptor: Descriptor,
    ) -> Result<Option<Self>, Error> {
        Ok(match descriptor {
            Descriptor::Class(class_name) => {
                let class = loader.load_class(context, class_name)?;

                class.map(ResolvedDescriptor::Class)
            }
            Descriptor::Array(inner_descriptor) => {
                let inner_resolved =
                    ResolvedDescriptor::try_from_descriptor(context, loader, *inner_descriptor)?;

                if let Some(inner_resolved) = inner_resolved {
                    let class = ClassLoader::array_class_for(context, inner_resolved);

                    Some(ResolvedDescriptor::Array(class))
                } else {
                    None
                }
            }
            Descriptor::Boolean => Some(ResolvedDescriptor::Boolean),
            Descriptor::Byte => Some(ResolvedDescriptor::Byte),
            Descriptor::Character => Some(ResolvedDescriptor::Character),
            Descriptor::Double => Some(ResolvedDescriptor::Double),
            Descriptor::Float => Some(ResolvedDescriptor::Float),
            Descriptor::Integer => Some(ResolvedDescriptor::Integer),
            Descriptor::Long => Some(ResolvedDescriptor::Long),
            Descriptor::Short => Some(ResolvedDescriptor::Short),
            Descriptor::Void => Some(ResolvedDescriptor::Void),
        })
    }

    /// Create a [`Descriptor`] from this [`ResolvedDescriptor`].
    pub fn descriptor(self, gc_ctx: GcCtx) -> Descriptor {
        match self {
            ResolvedDescriptor::Class(class) => Descriptor::Class(class.name()),
            ResolvedDescriptor::Array(class) => {
                let inner_type = class
                    .array_value_type()
                    .expect("Array class should have component type");
                let inner_desc = inner_type.descriptor(gc_ctx);

                Descriptor::Array(Gc::new(gc_ctx, inner_desc))
            }
            ResolvedDescriptor::Byte => Descriptor::Byte,
            ResolvedDescriptor::Character => Descriptor::Character,
            ResolvedDescriptor::Double => Descriptor::Double,
            ResolvedDescriptor::Float => Descriptor::Float,
            ResolvedDescriptor::Integer => Descriptor::Integer,
            ResolvedDescriptor::Long => Descriptor::Long,
            ResolvedDescriptor::Short => Descriptor::Short,
            ResolvedDescriptor::Boolean => Descriptor::Boolean,
            ResolvedDescriptor::Void => Descriptor::Void,
        }
    }

    /// Get the `Class` corresponding to this `ResolvedDescriptor`.
    ///
    /// If this is a `ResolvedDescriptor::Array` or `ResolvedDescriptor::Class`,
    /// return the class directly. Otherwise, return the primitive class
    /// corresponding to this `ResolvedDescriptor`.
    pub fn reflection_class(self, gc_ctx: GcCtx) -> Class {
        let primitive_type = match self {
            ResolvedDescriptor::Class(class) | ResolvedDescriptor::Array(class) => {
                return class;
            }
            ResolvedDescriptor::Byte => PrimitiveType::Byte,
            ResolvedDescriptor::Character => PrimitiveType::Char,
            ResolvedDescriptor::Double => PrimitiveType::Double,
            ResolvedDescriptor::Float => PrimitiveType::Float,
            ResolvedDescriptor::Integer => PrimitiveType::Int,
            ResolvedDescriptor::Long => PrimitiveType::Long,
            ResolvedDescriptor::Short => PrimitiveType::Short,
            ResolvedDescriptor::Boolean => PrimitiveType::Boolean,
            ResolvedDescriptor::Void => PrimitiveType::Void,
        };

        Class::for_primitive(gc_ctx, primitive_type)
    }

    /// Whether this `ResolvedDescriptor` represents a primitive.
    ///
    /// This method will return false when called on `ResolvedDescriptor::Class`
    /// or `ResolvedDescriptor::Array`.
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

    /// Forms a `String` from this `Descriptor`.
    ///
    /// This should be equivalent to calling [`ResolvedDescriptor::descriptor`],
    /// and then calling `Descriptor::to_string`, but is more efficient.
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

    /// Get the [`Class`] corresponding to this `ResolvedDescriptor`.
    ///
    /// If this is not a `ResolvedDescriptor::Array` or
    /// `ResolvedDescriptor::Class`, this method will return `None`.
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

/// A parsed but not-yet-resolved descriptor representing a method's signature.
///
/// This struct stores a list of [`Descriptor`]s for the arguments and one
/// `Descriptor` for the return type.
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
    args: Box<[Descriptor]>,
    physical_arg_count: usize,
    return_type: Descriptor,
}

impl MethodDescriptor {
    /// Parse a `MethodDescriptor` from a [`JvmString`]. This method uses a
    /// cache behind-the-scenes to avoid unnecessary allocations. It will return
    /// an `Error` if the method descriptor is invalid.
    pub fn from_string(context: &Context, descriptor: JvmString) -> Result<Self, Error> {
        if let Some(method_desc) = context.get_cached_method_descriptor(descriptor) {
            Ok(method_desc)
        } else if let Some(method_desc) = Self::new_from_string(context.gc_ctx, descriptor) {
            context.put_cached_method_descriptor(descriptor, method_desc);

            Ok(method_desc)
        } else {
            Err(context.class_format_error(&format!("Illegal method signature \"{}\"", descriptor)))
        }
    }

    // Creates a new `MethodDescriptor` from the given `JvmString`. This is
    // useful when no `Context` is available.
    pub(crate) fn new_from_string(gc_ctx: GcCtx, descriptor: JvmString) -> Option<Self> {
        let desc_bytes = descriptor.as_bytes();

        if desc_bytes.len() == 0 || desc_bytes[0] != b'(' {
            return None;
        }

        let mut args = Vec::with_capacity(2);
        let return_type;

        let mut physical_arg_count = 0;

        // Start from 1 to skip over the extra '(' at the beginning of every descriptor
        let mut i = 1;
        loop {
            if i >= desc_bytes.len() {
                return None;
            }

            match desc_bytes[i] {
                b')' => {
                    i += 1;

                    let return_desc =
                        Descriptor::from_data_counting(gc_ctx, &desc_bytes[i..], true)?;

                    // Trailing garbage = invalid descriptor
                    if i + return_desc.1 != descriptor.len() {
                        return None;
                    }

                    return_type = return_desc.0;
                    break;
                }
                _ => {
                    let arg_desc = Descriptor::from_data_counting(gc_ctx, &desc_bytes[i..], false)?;
                    i += arg_desc.1 - 1;

                    args.push(arg_desc.0);

                    if arg_desc.0.is_wide() {
                        physical_arg_count += 2;
                    } else {
                        physical_arg_count += 1;
                    }
                }
            }

            i += 1;
        }

        Some(Self(Gc::new(
            gc_ctx,
            MethodDescriptorData {
                args: args.into_boxed_slice(),
                physical_arg_count,
                return_type,
            },
        )))
    }

    /// The arguments of this `MethodDescriptor`.
    pub fn args(&self) -> &[Descriptor] {
        &self.0.args
    }

    /// The "physical" argument count of this descriptor. This counts two
    /// arguments for doubles and longs.
    pub fn physical_arg_count(&self) -> usize {
        self.0.physical_arg_count
    }

    /// The return type of this `MethodDescriptor`.
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

impl fmt::Debug for MethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut result = String::new();
        result.push('(');
        for arg in &self.0.args {
            result.push_str(&arg.to_string());
        }
        result.push(')');
        result.push_str(&self.0.return_type.to_string());

        write!(f, "{}", result)
    }
}

/// A descriptor representing a method's signature that has been both parsed and
/// resolved.
///
/// This struct stores a list of [`ResolvedDescriptor`]s for the arguments and
/// one `ResolvedDescriptor` for the return type.
///
/// See [`MethodDescriptor`] for the not-yet-resolved version of this struct.
#[derive(Clone, Copy)]
pub struct ResolvedMethodDescriptor(Gc<ResolvedMethodDescriptorData>);

struct ResolvedMethodDescriptorData {
    args: Box<[ResolvedDescriptor]>,
    physical_arg_count: usize,
    return_type: ResolvedDescriptor,
}

impl ResolvedMethodDescriptor {
    pub(crate) fn from_method_descriptor(
        context: &Context,
        loader: ClassLoader,
        descriptor: MethodDescriptor,
    ) -> Result<Self, Error> {
        let args = descriptor
            .args()
            .iter()
            .map(|arg| ResolvedDescriptor::from_descriptor(context, loader, *arg))
            .collect::<Result<Box<[_]>, Error>>()?;

        let return_type =
            ResolvedDescriptor::from_descriptor(context, loader, descriptor.return_type())?;

        Ok(Self(Gc::new(
            context.gc_ctx,
            ResolvedMethodDescriptorData {
                args,
                physical_arg_count: descriptor.physical_arg_count(),
                return_type,
            },
        )))
    }

    /// The arguments of this `ResolvedMethodDescriptor`.
    pub fn args(&self) -> &[ResolvedDescriptor] {
        &self.0.args
    }

    /// The "physical" argument count of this descriptor. This counts two
    /// arguments for doubles and longs.
    pub fn physical_arg_count(&self) -> usize {
        self.0.physical_arg_count
    }

    /// The return type of this `ResolvedMethodDescriptor`.
    pub fn return_type(self) -> ResolvedDescriptor {
        self.0.return_type
    }
}
