use std::env;
use std::fs;
use std::path::Path;
use bf_interpreter::vm::VM;

fn main() {
    let program = fs::read_to_string("d:/projects/rust/bf_programs/beer.b").expect("Couldn't read file");
    let result = VM::run(&program).expect("Couldn't execute program");
    let result_content = format!(
        "pub fn result() -> &'static str {{
            \"{}\"
        }}", result
    );

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("result.rs");
    fs::write(&dest_path, result_content).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}