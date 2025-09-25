use glob::glob;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const MODULES: &'static [&'static str] = &["base", "desktop"];

fn compile_module(module: &str) {
    let mut out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    out_dir.push(module);

    fs::remove_dir_all(&out_dir).unwrap();
    fs::create_dir(&out_dir).unwrap();

    let mut compile_command = Command::new("javac");
    compile_command.args(["-d", &out_dir.to_string_lossy()]);

    // Prevent javac from generating redundant class files
    compile_command.args(["-implicit:none"]);

    let mut classpaths = Vec::new();
    for module in MODULES {
        classpaths.push(format!("./globals/{}", module));
    }

    // Set classpath correctly
    compile_command.args(["-cp", &classpaths.join(":")]);

    // Compile .java files into .class files
    let mut source_file_list = glob(&format!("./globals/{}/*/*/*.java", module))
        .expect("Valid pattern")
        .map(|p| p.expect("Files should read"))
        .collect::<Vec<_>>();

    // Also include files with a three-component package name.
    source_file_list.extend_from_slice(
        &glob(&format!("./globals/{}/*/*/*/*.java", module))
            .expect("Valid pattern")
            .map(|p| p.expect("Files should read"))
            .collect::<Vec<_>>(),
    );

    compile_command.args(source_file_list);

    let compile_status = compile_command.status().expect("javac should run");

    if !compile_status.success() {
        panic!("javac returned error");
    }

    // So that we can restore it
    let saved_current_dir = env::current_dir().unwrap();

    // This makes the next operations easier
    env::set_current_dir(out_dir.clone()).expect("Should set");

    // Now gather the .class files together into a .jar archive
    let mut archive_command = Command::new("jar");
    archive_command.args([
        "cf",
        &out_dir
            .join(&format!("../classes-{}.jar", module))
            .to_string_lossy(),
    ]);

    let mut class_file_list = glob(&out_dir.join("*/*/*.class").to_string_lossy())
        .expect("Valid pattern")
        .map(|p| p.expect("Files should read"))
        .collect::<Vec<_>>();

    // Also include files with a three-component package name.
    class_file_list.extend_from_slice(
        &glob(&out_dir.join("*/*/*/*.class").to_string_lossy())
            .expect("Valid pattern")
            .map(|p| p.expect("Files should read"))
            .collect::<Vec<_>>(),
    );

    archive_command.args(
        class_file_list
            .iter()
            // This is very hacky: glob gives us absolute paths, so we need
            // to strip the OUT_DIR prefix to make them appear relative. Also,
            // jar doesn't accept files if they're not prefixed with "./", so
            // we need to add it manually.
            .map(|path| {
                let mut string = path
                    .strip_prefix(out_dir.clone())
                    .expect("Paths should be prefixed with out_dir")
                    .to_path_buf()
                    .into_os_string()
                    .into_string()
                    .expect("Should be valid string");

                string.insert_str(0, "./");

                string
            })
            .collect::<Vec<_>>(),
    );

    let archive_status = archive_command.status().expect("jar should run");

    if !archive_status.success() {
        panic!("jar returned error");
    }

    env::set_current_dir(&saved_current_dir).expect("Should set");
}

fn compile_globals() {
    for module in MODULES {
        compile_module(module);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=globals/");

    compile_globals();
}
