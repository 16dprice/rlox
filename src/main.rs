use rlox::compiler::{Compiler, FunctionType};
use rlox::debug::print_debug::disassemble_chunk;
use rlox::debug::write_debug::write_chunk_to_file;
use rlox::scanner::Scanner;
use rlox::value::Value;
use rlox::vm::VM;
use std::env;
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

        let mut vm = VM::<Vec<Value>>::new();
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

    let mut vm = VM::<Vec<Value>>::new();

    println!("==== BEGIN PROGRAM OUTPUT ====\n\n");
    vm.interpret(source);
    println!("\n\n==== END PROGRAM OUTPUT ====\n\n");
}

fn debug_to_file(file_path: &str) {
    let mut file =
        File::open(file_path).expect(format!("Could not open file {}", file_path).as_str());
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Could not write file to string");

    let scanner = Scanner::new(source.clone());
    let mut compiler = Compiler::new(scanner, FunctionType::Script, None);

    let compile_result = compiler.compile(None);
    if compile_result.is_none() {
        return;
    }

    let output_path = "./data/debug.txt";
    write_chunk_to_file(source, &compiler.current_chunk(), output_path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("file");

    match mode {
        "repl" => {
            repl();
        }
        "file" => {
            if args.len() >= 3 {
                run_file(&args[2]);
            } else {
                run_file("./data/test.rlox");
            }
        }
        "debug" => {
            if args.len() >= 3 {
                debug_to_file(&args[2]);
            } else {
                debug_to_file("./data/test.rlox");
            }
        }
        _ => {
            panic!("Unsupported mode: {mode}");
        }
    }
}
