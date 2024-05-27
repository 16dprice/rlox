mod chunk;
mod debug;
mod scanner;
mod value;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use scanner::{Scanner, TokenType};
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

    let mut scanner = Scanner::new(contents);
    let string_stuff = scanner.source.as_str();

    println!("{}", string_stuff.len());

    loop {
        let token = scanner.scan_token();
        match token.token_type {
            TokenType::Eof => break,
            _ => println!("{:?}", token),
        }
    }

    // run the file
}

fn main() {
    let use_repl = false;

    if use_repl {
        repl();
    } else {
        run_file("/Users/dj/Code/rlox/data/test.rlox");
    }
}
