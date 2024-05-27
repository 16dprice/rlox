mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;

use chunk::Chunk;
use compiler::Compiler;
use debug::disassemble_chunk;
use std::fs::File;
use std::io::{self, Read, Write};

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
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not write file to string");

    let first_chunk = Chunk::new();
    let mut compiler = Compiler::new(contents, first_chunk);

    let second_chunk = Chunk::new();

    compiler.compile(Some(second_chunk));

    disassemble_chunk(&compiler.compiling_chunk, "First Chunk!");

    // TODO: run the file
}

fn main() {
    let use_repl = false;

    if use_repl {
        repl();
    } else {
        run_file("/Users/dj/Code/rlox/data/test.rlox");
    }
}
