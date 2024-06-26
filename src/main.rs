mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use chunk::Chunk;
use compiler::Compiler;
use debug::print_debug::disassemble_chunk;
use debug::write_debug::write_chunk_to_file;
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

        let mut value_stack = vec![];
        let mut vm = VM::new(&mut value_stack);
        vm.interpret(String::from(input));

        disassemble_chunk(&vm.chunk, "Repl chunk");

        let value_stack_top = vm.value_stack.pop();
        println!("Top of VM Value Stack - {:?}", value_stack_top);
    }
}

fn run_file(file_path: &str) {
    let mut file =
        File::open(file_path).expect(format!("Could not open file {}", file_path).as_str());
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Could not write file to string");

    let mut value_stack = vec![];
    let mut vm = VM::new(&mut value_stack);
    vm.interpret(source);

    disassemble_chunk(&vm.chunk, "First Chunk!");
}

fn debug_to_file(file_path: &str) {
    let mut file =
        File::open(file_path).expect(format!("Could not open file {}", file_path).as_str());
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Could not write file to string");

    let chunk = Chunk::new();
    let mut compiler = Compiler::new(source.clone(), chunk);

    if !compiler.compile(None) {
        return;
    }

    let output_path = "./data/debug.txt";
    write_chunk_to_file(source, &compiler.compiling_chunk, output_path);
}

fn main() {
    let use_repl = true;

    debug_to_file("./data/test.rlox");
    // run_file("./data/test.rlox");

    // if use_repl {
    //     repl();
    // } else {
    //     run_file("./data/test.rlox");
    // }
}
