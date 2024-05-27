mod chunk;
mod debug;
mod value;

use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use std::io::{self, Write};

fn main() {
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
