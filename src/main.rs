mod classfile;
mod gc;
mod jar;
mod runtime;
mod string;

use crate::classfile::class::ClassFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::jar::Jar;
use crate::runtime::class::Class;
use crate::runtime::context::{Context, ResourceLoadType, ResourceLoader};
use crate::runtime::descriptor::MethodDescriptor;
use crate::runtime::object::Object;
use crate::runtime::value::Value;
use crate::string::JvmString;

use std::env;
use std::fs;
use std::path::PathBuf;

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

struct DesktopResourceLoader {}

impl ResourceLoader for DesktopResourceLoader {
    fn load_resource(
        &self,
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadType::FileSystem => {
                let mut path_buf = PathBuf::from(class_name);
                path_buf.pop();
                path_buf.push(resource_name);

                fs::read(path_buf).ok()
            }
            ResourceLoadType::Jar(jar) => {
                let resolved_name = if let Some(absolute_path) = class_name.strip_prefix('/') {
                    // TODO do absolute paths actually work?
                    absolute_path.to_string()
                } else {
                    // TODO should this handle paths starting with "./", maybe?
                    let mut path_sections = class_name.split('/').collect::<Vec<_>>();
                    path_sections.pop();
                    path_sections.push(resource_name);

                    path_sections.join("/")
                };

                if jar.has_file(&resolved_name) {
                    jar.read_file(&resolved_name).ok()
                } else {
                    None
                }
            }
        }
    }
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

    let loader = DesktopResourceLoader {};

    let context = Context::new(gc_ctx, Box::new(loader));

    let main_class = match file_info.0 {
        FileType::Class => {
            let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();
            let main_class =
                Class::from_class_file(context, ResourceLoadType::FileSystem, class_file)
                    .expect("Failed to load main class");

            context.register_class(main_class);

            main_class
                .load_methods(context)
                .expect("Failed to load main class method data");

            main_class
        }
        FileType::Jar => {
            let manifest_name = "META-INF/MANIFEST.MF".to_string();

            let jar_data = Jar::from_bytes(gc_ctx, read_file).expect("Invalid jar file passed");
            context.add_jar(jar_data);

            let has_manifest = jar_data.has_file(&manifest_name);
            if !has_manifest {
                eprintln!("Cannot execute JAR file without MANIFEST.MF file");
                return;
            }

            let manifest_data = jar_data
                .read_file(&manifest_name)
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
                    Class::from_class_file(context, ResourceLoadType::Jar(jar_data), class_file)
                        .expect("Failed to load main class");

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
    let args_array = Value::Object(Some(Object::obj_array(context, string_class, &[])));

    let main_name = JvmString::new(gc_ctx, "main".to_string());
    let main_descriptor_name = JvmString::new(gc_ctx, "([Ljava/lang/String;)V".to_string());

    let main_descriptor =
        MethodDescriptor::from_string(gc_ctx, main_descriptor_name).expect("Valid descriptor");

    let method_idx = main_class
        .static_method_vtable()
        .lookup((main_name, main_descriptor));

    if let Some(method_idx) = method_idx {
        let method = main_class.static_methods()[method_idx];
        let result = method.exec(context, &[args_array]);

        if let Err(error) = result {
            eprintln!("Error while running main: {:?}", error);
        }
    } else {
        eprintln!(
            "Class {} has no `void main(String[] args)` method",
            main_class.dot_name()
        );
    }
}
