use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::field::Field;
use super::value::{Value, ValueType};

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub struct Object(Gc<ObjectData>);

impl Object {
    pub fn from_class(gc_ctx: GcCtx, class: Class) -> Self {
        let fields = class.instance_fields().to_vec().into_boxed_slice();

        Self(Gc::new(
            gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    pub fn char_array(context: Context, chars: &[u16]) -> Self {
        let value_list = chars
            .iter()
            .map(|b| Value::Integer(*b as i32))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_char_desc)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn call_construct(
        self,
        context: Context,
        descriptor: MethodDescriptor,
        args: &[Value],
    ) -> Result<(), Error> {
        let init_name = context.common.init_name;

        let instance_method_vtable = self.0.class.instance_method_vtable();
        let instance_methods = self.0.class.instance_methods();

        let method_idx = instance_method_vtable
            .lookup((init_name, descriptor))
            .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

        instance_methods[method_idx].exec(context, args)?;

        Ok(())
    }
}

impl Trace for Object {
    fn trace(&self) {
        self.0.trace();
    }
}

struct ObjectData {
    class: Class,

    data: FieldOrArrayData,
}

impl Trace for ObjectData {
    fn trace(&self) {
        self.class.trace();
        self.data.trace();
    }
}

enum FieldOrArrayData {
    Fields(Box<[Field]>),
    Array(Box<[Value]>),
}

impl Trace for FieldOrArrayData {
    fn trace(&self) {
        match self {
            FieldOrArrayData::Fields(data) => data.trace(),
            FieldOrArrayData::Array(data) => data.trace(),
        }
    }
}
