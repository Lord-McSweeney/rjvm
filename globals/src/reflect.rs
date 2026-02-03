// Helper functions for reflection methods

use alloc::boxed::Box;
use alloc::vec::Vec;
use rjvm_core::{
    Class, Context, Descriptor, Error, Gc, JvmString, Method, MethodFlags, Object, PrimitiveType,
    Value,
};

pub(crate) fn constructors_for_class(context: &Context, class: Class) -> Box<[Method]> {
    class
        .instance_method_vtable()
        .elements_for_name(context.common.init_name)
        .iter()
        .filter_map(|m| {
            // Make sure we're only picking up public initializers defined in this class
            if m.class() == class && m.flags().contains(MethodFlags::PUBLIC) {
                Some(*m)
            } else {
                None
            }
        })
        .collect::<Box<_>>()
}

pub(crate) fn get_class_method(
    class: Class,
    name: JvmString,
    args: &[Descriptor],
) -> Option<Method> {
    // TODO: Implement return type specificity rules

    let instance_elems = class.instance_method_vtable().elements_for_name(name);

    let instance_method = instance_elems.iter().find(|m| {
        // Make sure we're only picking up public methods
        m.flags().contains(MethodFlags::PUBLIC) && m.descriptor().args() == args
    });

    if let Some(instance_method) = instance_method {
        return Some(*instance_method);
    }

    // We need to do static methods separately because they aren't inherited

    let mut current_class = Some(class);
    while let Some(cls) = current_class {
        let static_vtable = cls.static_method_vtable();
        let static_methods = cls.static_methods();

        let slots = static_vtable.slots_for_name(name);
        let static_method_index = slots.iter().find(|i| {
            let method = static_methods[**i];
            // Make sure we're only picking up public methods
            method.flags().contains(MethodFlags::PUBLIC) && method.descriptor().args() == args
        });

        if let Some(static_method_index) = static_method_index {
            return Some(static_methods[*static_method_index]);
        }

        current_class = cls.super_class();
    }

    None
}

pub(crate) fn descriptor_for_class(context: &Context, class: Class) -> Descriptor {
    match class.primitive_type() {
        Some(PrimitiveType::Boolean) => Descriptor::Boolean,
        Some(PrimitiveType::Byte) => Descriptor::Byte,
        Some(PrimitiveType::Char) => Descriptor::Character,
        Some(PrimitiveType::Double) => Descriptor::Double,
        Some(PrimitiveType::Float) => Descriptor::Float,
        Some(PrimitiveType::Int) => Descriptor::Integer,
        Some(PrimitiveType::Long) => Descriptor::Long,
        Some(PrimitiveType::Short) => Descriptor::Short,
        Some(PrimitiveType::Void) => Descriptor::Void,
        None => {
            if let Some(inner_type) = class.array_value_type() {
                let inner_desc = inner_type.descriptor(context.gc_ctx);
                Descriptor::Array(Gc::new(context.gc_ctx, inner_desc))
            } else {
                Descriptor::Class(class.name())
            }
        }
    }
}

// Change the provided args into a form suitable for calling the given method.
pub(crate) fn args_for_instance_call(
    _context: &Context,
    method: Method,
    receiver: Option<Object>,
    args: &[Value],
) -> Result<Vec<Value>, Error> {
    let mut unboxed_args = Vec::with_capacity(args.len() + 1);

    if let Some(receiver) = receiver {
        unboxed_args.push(Value::Object(Some(receiver)));
    }

    for (i, arg) in args.iter().enumerate() {
        // All args are passed as objects
        let arg = arg.object();

        // TODO implement more argument type verification

        // TODO implement more unboxing
        // NOTE remember to account for the physical vs virtual difference in
        // args when implementing unboxing for Long and Double
        if matches!(method.descriptor().args()[i], Descriptor::Integer) {
            if let Some(arg) = arg {
                // FIXME this is really hacky
                if *arg.class().name() == "java/lang/Integer" {
                    unboxed_args.push(arg.get_field(0));
                    continue;
                }
            } else {
                todo!("Throw an IllegalArgumentException");
            }
        }
        unboxed_args.push(Value::Object(arg));
    }

    Ok(unboxed_args)
}
