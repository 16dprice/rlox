use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

fn get_value_debug_string(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => format!("{}", v),
        Value::Number(v) => format!("{}", v),
        Value::String(v) => format!("{}", v),
    }
}

pub mod print_debug {
    use super::*;

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
            print!("{}", get_value_debug_string(constant));
            println!("'");

            return offset + 2;
        } else if instruction == OpCode::Add as u8 {
            return simple_instruction("OP_ADD", offset);
        } else if instruction == OpCode::Subtract as u8 {
            return simple_instruction("OP_SUBTRACT", offset);
        } else if instruction == OpCode::Multiply as u8 {
            return simple_instruction("OP_MULTIPLY", offset);
        } else if instruction == OpCode::Divide as u8 {
            return simple_instruction("OP_DIVIDE", offset);
        } else if instruction == OpCode::True as u8 {
            return simple_instruction("OP_TRUE", offset);
        } else if instruction == OpCode::False as u8 {
            return simple_instruction("OP_FALSE", offset);
        } else if instruction == OpCode::Nil as u8 {
            return simple_instruction("OP_NIL", offset);
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
}

pub mod write_debug {
    use std::{fs::File, io::Write};

    use super::*;

    fn disassemble_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
        let instruction = chunk.code[offset];

        if instruction == OpCode::Return as u8 {
            return (String::from("OP_RETURN\n"), offset + 1);
        } else if instruction == OpCode::Constant as u8 {
            let constant = &chunk.constants[chunk.code[offset + 1] as usize];

            return (
                format!(
                    "OP_CONSTANT\nCONSTANT: {}\n",
                    get_value_debug_string(constant)
                ),
                offset + 2,
            );
        } else if instruction == OpCode::Equal as u8 {
            return (String::from("OP_EQUAL\n"), offset + 1);
        }

        return (String::from("asdf\n"), offset + 1);
    }

    pub fn write_chunk_to_file(chunk: &Chunk, output_path: &str) {
        let mut file = File::create(output_path)
            .expect(format!("Could not open file {}", output_path).as_str());

        let mut offset = 0;
        let mut debug_string: String;
        while offset < chunk.code.len() {
            (debug_string, offset) = disassemble_instruction(chunk, offset);

            file.write_all(debug_string.as_bytes())
                .expect("Couldn't write to file");
        }
    }
}
