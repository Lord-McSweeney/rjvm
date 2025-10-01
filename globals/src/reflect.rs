// Helper functions for reflection methods

use rjvm_core::{Class, Context, Method, MethodFlags, Object, Value};

pub(crate) fn constructors_for_class(context: Context, class: Class) -> Box<[Method]> {
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
pub(crate) fn args_for_call(
    _method: Method,
    receiver: Option<Object>,
    args: &[Value],
) -> Vec<Value> {
    let mut result = Vec::with_capacity(args.len() + 1);

    if let Some(receiver) = receiver {
        result.push(Value::Object(Some(receiver)));
    }
    // TODO implement unboxing
    result.extend_from_slice(args);

    result
}
