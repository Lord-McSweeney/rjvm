use rjvm_core::Context;

// Native implementations of functions declared in globals

pub fn register_native_mappings(context: &Context) {
    crate::impls_loader::register_native_mappings(context);
    crate::impls_math::register_native_mappings(context);
    crate::impls_misc::register_native_mappings(context);
    crate::impls_reflect::register_native_mappings(context);
    crate::impls_system::register_native_mappings(context);
}
