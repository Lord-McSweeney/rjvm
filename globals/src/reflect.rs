// Helper functions for reflection methods

use rjvm_core::{Class, Context, Descriptor, Error, Method, MethodFlags, Object, Value};

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

// Change the provided args into a form suitable for calling the given method.
pub(crate) fn args_for_instance_call(
    _context: &Context,
    method: Method,
    receiver: Object,
    args: &[Value],
) -> Result<Vec<Value>, Error> {
    let mut unboxed_args = Vec::with_capacity(args.len() + 1);
    unboxed_args.push(Value::Object(Some(receiver)));
    for (i, arg) in args.iter().enumerate() {
        // All args are passed as objects
        let arg = arg.object();

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
