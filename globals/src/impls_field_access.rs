use rjvm_core::{Context, Error, NativeMethod, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/reflect/FieldAccess.getDoubleStaticNative.(Ljava/lang/reflect/Field;)D", get_static_field),
        ("java/lang/reflect/FieldAccess.getDoubleInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)D", get_instance_field),

        ("java/lang/reflect/FieldAccess.getFloatStaticNative.(Ljava/lang/reflect/Field;)F", get_static_field),
        ("java/lang/reflect/FieldAccess.getFloatInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)F", get_instance_field),

        ("java/lang/reflect/FieldAccess.getIntStaticNative.(Ljava/lang/reflect/Field;)I", get_static_field),
        ("java/lang/reflect/FieldAccess.getIntInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)I", get_instance_field),

        ("java/lang/reflect/FieldAccess.getLongStaticNative.(Ljava/lang/reflect/Field;)J", get_static_field),
        ("java/lang/reflect/FieldAccess.getLongInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)J", get_instance_field),

        ("java/lang/reflect/FieldAccess.getObjectStaticNative.(Ljava/lang/reflect/Field;)Ljava/lang/Object;", get_static_field),
        ("java/lang/reflect/FieldAccess.getObjectInstanceNative.(Ljava/lang/reflect/Field;Ljava/lang/Object;)Ljava/lang/Object;", get_instance_field),
    ];

    context.register_native_mappings(mappings);
}

fn get_static_field(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Field should never be null
    let field_obj = args[0].object().unwrap();
    let field_id = field_obj.get_field(0).int();
    let field = context.field_object_by_id(field_id);

    let cls = field.defining_class();
    let id = field.id();

    let value = cls.static_fields()[id].value();

    Ok(Some(value))
}

fn get_instance_field(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Field should never be null
    let field_obj = args[0].object().unwrap();
    let field_id = field_obj.get_field(0).int();
    let field = context.field_object_by_id(field_id);

    // Object null-checked by Java code
    let object = args[1].object().unwrap();

    let id = field.id();

    let value = object.get_field(id);

    Ok(Some(value))
}
