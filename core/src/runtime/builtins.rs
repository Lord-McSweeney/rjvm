use super::class::Class;
use super::context::Context;

use crate::gc::Trace;
use crate::string::JvmString;

// The builtin classes, looked-up at VM startup. NOTE: `java/lang/Object` is
// accessed with `context.object_class()` because it needs to be loaded before
// the rest of the builtins
pub struct BuiltinClasses {
    pub java_lang_class: Class,
    pub java_lang_string: Class,
    pub java_lang_throwable: Class,

    pub java_lang_arithmetic_exception: Class,
    pub java_lang_array_index_oob_exception: Class,
    pub java_lang_array_store_exception: Class,
    pub java_lang_class_cast_exception: Class,
    pub java_lang_clone_not_supported_exception: Class,
    pub java_lang_cloneable: Class,
    pub java_lang_illegal_access_error: Class,
    pub java_lang_instantiation_error: Class,
    pub java_lang_negative_array_size_exception: Class,
    pub java_lang_no_class_def_found_error: Class,
    pub java_lang_no_such_field_error: Class,
    pub java_lang_no_such_method_error: Class,
    pub java_lang_null_pointer_exception: Class,
    pub java_lang_reflect_constructor: Class,

    pub jvm_internal_concrete_class_loader: Class,

    pub array_byte: Class,
    pub array_char: Class,
    pub array_double: Class,
    pub array_float: Class,
    pub array_int: Class,
    pub array_long: Class,
    pub array_short: Class,
    pub array_bool: Class,
}

macro_rules! builtin_classes {
    ($context:expr, [$(($class_name:literal, $field:ident)),* $(,)?]) => {
        BuiltinClasses {
            $(
                $field: {
                    let string = JvmString::new($context.gc_ctx, $class_name.to_string());

                    $context
                        .bootstrap_loader()
                        .find_class($context, string)
                        .expect("Builtin class parsing failed")
                        .unwrap_or_else(|| panic!("Builtin class {} was not found", $class_name))
                },
            )*
        }
    }
}

impl BuiltinClasses {
    #[rustfmt::skip]
    pub fn new(context: &Context) -> Self {
        builtin_classes!(
            context,
            [
                // String, then Throwable, then Class
                ("java/lang/String", java_lang_string),
                ("java/lang/Throwable", java_lang_throwable),
                ("java/lang/Class", java_lang_class),

                ("java/lang/ArithmeticException", java_lang_arithmetic_exception),
                ("java/lang/ArrayIndexOutOfBoundsException", java_lang_array_index_oob_exception),
                ("java/lang/ArrayStoreException", java_lang_array_store_exception),
                ("java/lang/ClassCastException", java_lang_class_cast_exception),
                ("java/lang/CloneNotSupportedException", java_lang_clone_not_supported_exception),
                ("java/lang/Cloneable", java_lang_cloneable),
                ("java/lang/IllegalAccessError", java_lang_illegal_access_error),
                ("java/lang/InstantiationError", java_lang_instantiation_error),
                ("java/lang/NegativeArraySizeException", java_lang_negative_array_size_exception),
                ("java/lang/NoClassDefFoundError", java_lang_no_class_def_found_error),
                ("java/lang/NoSuchFieldError", java_lang_no_such_field_error),
                ("java/lang/NoSuchMethodError", java_lang_no_such_method_error),
                ("java/lang/NullPointerException", java_lang_null_pointer_exception),
                ("java/lang/reflect/Constructor", java_lang_reflect_constructor),

                ("jvm/internal/ConcreteClassLoader", jvm_internal_concrete_class_loader),

                ("[B", array_byte),
                ("[C", array_char),
                ("[D", array_double),
                ("[F", array_float),
                ("[I", array_int),
                ("[J", array_long),
                ("[S", array_short),
                ("[Z", array_bool),
            ]
        )
    }
}

impl Trace for BuiltinClasses {
    fn trace(&self) {
        self.java_lang_class.trace();
        self.java_lang_string.trace();
        self.java_lang_throwable.trace();

        self.java_lang_arithmetic_exception.trace();
        self.java_lang_array_index_oob_exception.trace();
        self.java_lang_array_store_exception.trace();
        self.java_lang_class_cast_exception.trace();
        self.java_lang_clone_not_supported_exception.trace();
        self.java_lang_cloneable.trace();
        self.java_lang_illegal_access_error.trace();
        self.java_lang_instantiation_error.trace();
        self.java_lang_negative_array_size_exception.trace();
        self.java_lang_no_class_def_found_error.trace();
        self.java_lang_no_such_field_error.trace();
        self.java_lang_no_such_method_error.trace();
        self.java_lang_null_pointer_exception.trace();
        self.java_lang_reflect_constructor.trace();

        self.jvm_internal_concrete_class_loader.trace();

        self.array_byte.trace();
        self.array_char.trace();
        self.array_double.trace();
        self.array_float.trace();
        self.array_int.trace();
        self.array_long.trace();
        self.array_short.trace();
        self.array_bool.trace();
    }
}
