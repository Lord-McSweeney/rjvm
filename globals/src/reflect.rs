// Implementations for reflection-related functions

use rjvm_core::{Class, Context, Method, MethodFlags};

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
