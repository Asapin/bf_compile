extern crate clap;

mod code_gen;
mod enums;
mod lexer;
mod memory;
mod optimizer;
mod vm;

use clap::{App, Arg};
use std::fs;
use vm::VM;

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

    VM::run(&program).unwrap();
}
