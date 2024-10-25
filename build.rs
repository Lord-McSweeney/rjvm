use glob::glob;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn compile_globals() {
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    let mut compile_command = Command::new("javac");
    compile_command.args(["-d", &out_dir.to_string_lossy()]);

    // Compile .java files into .class files
    let source_file_list = glob("./src/runtime/globals/java/*/*.java").expect("Valid pattern");
    compile_command.args(
        source_file_list
            .map(|p| p.expect("Files should read"))
            .collect::<Vec<_>>(),
    );

    let compile_status = compile_command.status().expect("javac should run");

    if !compile_status.success() {
        panic!("javac returned error");
    }

    // This makes the next operations easier
    env::set_current_dir(out_dir.clone()).expect("Should set");

    // Now gather the .class files together into a .jar archive
    let mut archive_command = Command::new("jar");
    archive_command.args(["cf", &out_dir.join("classes.jar").to_string_lossy()]);

    let class_file_list =
        glob(&out_dir.join("java/*/*.class").to_string_lossy()).expect("Valid pattern");

    archive_command.args(
        class_file_list
            // This is very hacky: glob gives us absolute paths, so we need
            // to strip the OUT_DIR prefix to make them appear relative. Also,
            // jar doesn't accept files if they're not prefixed with "./", so
            // we need to add it manually.
            .map(|path| {
                let mut string = path
                    .expect("Files should read")
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
}

fn main() {
    println!("cargo:rerun-if-changed=src/runtime/globals/");

    compile_globals();
}
