use rjvm_core::Context;

// Native implementations of functions declared in globals

pub fn register_native_mappings(context: &Context) {
    crate::impls::field_access::register_native_mappings(context);
    crate::impls::loader::register_native_mappings(context);
    crate::impls::math::register_native_mappings(context);
    crate::impls::misc::register_native_mappings(context);
    crate::impls::reflect::register_native_mappings(context);
    crate::impls::system::register_native_mappings(context);
}
