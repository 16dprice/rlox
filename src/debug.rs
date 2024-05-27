use crate::chunk::{Chunk, OpCode};
use crate::value::print_value;

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("CHUNK OFFSET - {:0>4} | ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("LINE -    | ");
    } else {
        print!("LINE - {:0>4} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];

    if instruction == OpCode::Return as u8 {
        println!("OP_RETURN");
        return offset + 1;
    } else if instruction == OpCode::Constant as u8 {
        let constant = &chunk.constants[chunk.code[offset + 1] as usize];
        print!("'");
        print_value(constant);
        println!("'");

        return offset + 2;
    } else if instruction == OpCode::Add as u8 {
        return simple_instruction("OP_ADD", offset);
    } else {
        println!("Unkown opcode {:0>4}", instruction);
        return offset + 1;
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}
