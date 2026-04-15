use rjvm_core::{Context, Error, NativeMethod, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/reflect/FieldAccess.getObjectStaticNative.(Ljava/lang/reflect/Field;)Ljava/lang/Object;", get_object_static),
        ("java/lang/reflect/FieldAccess.getObjectInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)Ljava/lang/Object;", get_object_instance),
    ];

    context.register_native_mappings(mappings);
}

fn get_object_static(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Field should never be null
    let field_obj = args[0].object().unwrap();
    let field_id = field_obj.get_field(0).int();
    let field = context.field_object_by_id(field_id);

    let cls = field.defining_class();
    let id = field.id();

    // Guaranteed to be a `Value::Object` by Java code
    let value = cls.static_fields()[id].value();

    Ok(Some(value))
}

fn get_object_instance(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Field should never be null
    let field_obj = args[0].object().unwrap();
    let field_id = field_obj.get_field(0).int();
    let field = context.field_object_by_id(field_id);

    // Object null-checked by Java code
    let object = args[1].object().unwrap();

    let id = field.id();

    // Guaranteed to be a `Value::Object` by Java code
    let value = object.get_field(id);

    Ok(Some(value))
}
