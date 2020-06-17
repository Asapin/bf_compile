extern crate bf_interpreter;
extern crate clap;

use bf_interpreter::vm::VM;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("Brainfuck interpreter")
        .version("1.0")
        .author("Vladimir K. <1559761+Asapin@users.noreply.github.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches.value_of("INPUT").unwrap();
    let program = fs::read_to_string(file_path).expect("Couldn't read file");

    let result = VM::run(&program).unwrap();
    println!("{}", result);
}
