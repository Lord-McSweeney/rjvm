use rjvm_core::{Context, Error, NativeMethod, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/Math.atan2.(DD)D", math_atan2),
        ("java/lang/Math.floor.(D)D", math_floor),
        ("java/lang/Math.log.(D)D", math_log),
        ("java/lang/Math.pow.(DD)D", math_pow),
        ("java/lang/Math.sqrt.(D)D", math_sqrt),
    ];

    context.register_native_mappings(mappings);
}

fn math_atan2(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let y = args[0].double();
    let x = args[2].double();

    // TODO docs say this has some special-cases

    Ok(Some(Value::Double(libm::atan2(y, x))))
}

fn math_floor(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::floor(value))))
}

fn math_log(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::log(value))))
}

fn math_pow(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let base = args[0].double();
    let exp = args[2].double();

    Ok(Some(Value::Double(libm::pow(base, exp))))
}

fn math_sqrt(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::sqrt(value))))
}
