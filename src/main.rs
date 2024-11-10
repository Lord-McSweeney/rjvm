mod classfile;
mod gc;
mod jar;
mod runtime;
mod string;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::runtime::class::Class;
use crate::runtime::context::Context;
use crate::runtime::descriptor::MethodDescriptor;
use crate::runtime::object::Object;
use crate::runtime::value::Value;
use crate::string::JvmString;

use std::env;
use std::fs;

struct Root {}

impl Trace for Root {
    fn trace(&self) {}
}

enum FileType {
    Class,
    Jar,
}

fn get_main_class_from_manifest(manifest_data: Vec<u8>) -> Option<String> {
    let stringified_data = String::from_utf8_lossy(&manifest_data);

    let headers = stringified_data.split("\r\n");

    let mut main_class = None;
    for header in headers {
        let split_once = header.split_once(": ");
        if let Some((before, after)) = split_once {
            if before == "Main-Class" {
                main_class = Some(after.to_string());
            }
        }
    }

    main_class.map(|c| c.replace('.', "/").to_string())
}

fn main() {
    let gc_ctx = GcCtx::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Run as {} [file.class]", args[0]);
        return;
    }

    let file_info = if args[1] == "--jar" {
        if args.len() < 3 {
            eprintln!("--jar flag require passing an argument to it");
            return;
        } else {
            (FileType::Jar, args[2].clone())
        }
    } else {
        (FileType::Class, args[1].clone())
    };

    let read_file = match fs::read(&file_info.1) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error: {}", error.to_string());
            return;
        }
    };

    let context = Context::new(gc_ctx);

    let main_class = match file_info.0 {
        FileType::Class => {
            let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();
            let main_class =
                Class::from_class_file(context, class_file).expect("Failed to load main class");

            context.register_class(main_class);

            main_class
                .load_methods(context)
                .expect("Failed to load main class method data");

            main_class
        }
        FileType::Jar => {
            let jar_data = Jar::from_bytes(gc_ctx, read_file).expect("Invalid jar file passed");
            context.add_jar(jar_data);

            let has_manifest = jar_data.has_file("META-INF/MANIFEST.MF".to_string());
            if !has_manifest {
                eprintln!("Cannot execute JAR file without MANIFEST.MF file");
                return;
            }

            let manifest_data = jar_data
                .read_file("META-INF/MANIFEST.MF".to_string())
                .expect("MANIFEST should read");

            let main_class_name = get_main_class_from_manifest(manifest_data);
            if let Some(main_class_name) = main_class_name {
                let main_class_name = JvmString::new(gc_ctx, main_class_name);

                let has_main_class = jar_data.has_class(main_class_name);
                if !has_main_class {
                    eprintln!("Main class specified in MANIFEST.MF was not present in JAR!");
                    return;
                }

                let main_class_data = jar_data
                    .read_class(main_class_name)
                    .expect("Main class should read");

                let class_file = ClassFile::from_data(context.gc_ctx, main_class_data).unwrap();
                let main_class =
                    Class::from_class_file(context, class_file).expect("Failed to load main class");

                context.register_class(main_class);

                main_class
                    .load_methods(context)
                    .expect("Failed to load main class method data");

                main_class
            } else {
                eprintln!("Cannot execute JAR file without main class specified");
                return;
            }
        }
    };

    let string_class = context
        .lookup_class(context.common.java_lang_string)
        .expect("String class should exist");

    // TODO actually pass args
    let args_array = Object::obj_array(context, string_class, &[]);

    let main_name = JvmString::new(gc_ctx, "main".to_string());
    let main_descriptor_name = JvmString::new(gc_ctx, "([Ljava/lang/String;)V".to_string());

    let main_descriptor =
        MethodDescriptor::from_string(gc_ctx, main_descriptor_name).expect("Valid descriptor");

    let result = main_class.call_static(
        context,
        &[Value::Object(Some(args_array))],
        (main_name, main_descriptor),
    );

    if let Err(error) = result {
        eprintln!("Error while running main: {:?}", error);
    }
}
