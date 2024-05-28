mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use debug::disassemble_chunk;
use std::fs::File;
use std::io::{self, Read, Write};
use vm::VM;

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            break;
        }
    }
}

fn run_file(file_path: &str) {
    let mut file =
        File::open(file_path).expect(format!("Could not open file {}", file_path).as_str());
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Could not write file to string");

    let mut vm = VM::new();
    let result = vm.interpret(source);

    disassemble_chunk(&vm.chunk, "First Chunk!");
}

fn main() {
    let use_repl = false;

    if use_repl {
        repl();
    } else {
        run_file("./data/test.rlox");
    }
}
