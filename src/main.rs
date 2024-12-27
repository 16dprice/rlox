mod chunk;
mod compiler;
mod debug;
mod math;
mod scanner;
mod value;
mod vm;

use compiler::{Compiler, FunctionType};
use debug::print_debug::disassemble_chunk;
use debug::write_debug::write_chunk_to_file;
use scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use value::Value;
use vm::VM;

use mini_json;

#[allow(dead_code)]
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

    // disassemble_chunk(&vm.frames[0].closure.function.chunk, "TOP LEVEL CHUNK");
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
    let json_object = mini_json::parse_from_file("/Users/djprice/Code/rlox/data/json/object.json");
    match json_object {
        Ok(object) => {
            println!("{}", object);
        }
        _ => {}
    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     // assert!(args.len() >= 2);

//     // let mode = &args[1];
//     let mode = String::from("file");
//     match mode.as_str() {
//         "repl" => {
//             repl();
//         }
//         "file" => {
//             if args.len() >= 3 {
//                 run_file(&args[2]);
//             } else {
//                 run_file("./data/test.rlox");
//             }
//         }
//         "debug" => {
//             if args.len() >= 3 {
//                 debug_to_file(&args[2]);
//             } else {
//                 debug_to_file("./data/test.rlox");
//             }
//         }
//         _ => {
//             panic!("Unsupported mode: {mode}");
//         }
//     }
// }
