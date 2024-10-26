mod classfile;
mod gc;
mod jar;
mod runtime;
mod string;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::runtime::class::Class;
use crate::runtime::context::Context;
use crate::runtime::descriptor::MethodDescriptor;
use crate::runtime::value::Value;
use crate::string::JvmString;

use std::env;
use std::fs;

struct Root {}

impl Trace for Root {
    fn trace(&self) {}
}

fn main() {
    let gc_ctx = GcCtx::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Run as {} [file.class]", args[0]);
        return;
    }

    let read_file = match fs::read(&args[1]) {
        Ok(data) => data,
        Err(error) => {
            println!("Error: {}", error.to_string());
            return;
        }
    };

    let context = Context::new(gc_ctx);

    let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();
    let main_class =
        Class::from_class_file(context, class_file).expect("Failed to load main class");

    context.register_class(main_class);

    main_class
        .load_method_data(context)
        .expect("Failed to load main class method data");

    let main_name = JvmString::new(gc_ctx, "main".to_string());
    let main_descriptor_name = JvmString::new(gc_ctx, "([Ljava/lang/String;)V".to_string());

    let main_descriptor =
        MethodDescriptor::from_string(gc_ctx, main_descriptor_name).expect("Valid descriptor");

    main_class
        .call_static(
            context,
            &[Value::Object(None)],
            (main_name, main_descriptor),
        )
        .expect("Failed to run main");
}
