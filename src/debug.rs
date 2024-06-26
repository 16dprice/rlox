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

        let instruction = OpCode::from_u8(chunk.code[offset]).unwrap();

        match instruction {
            OpCode::Return => {
                println!("OP_RETURN");
                return offset + 1;
            }
            OpCode::Constant => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];
                print!("'");
                print!("{}", get_value_debug_string(constant));
                println!("'");

                return offset + 2;
            }
            OpCode::Add => {
                return simple_instruction("OP_ADD", offset);
            }
            OpCode::Subtract => {
                return simple_instruction("OP_SUBTRACT", offset);
            }
            OpCode::Multiply => {
                return simple_instruction("OP_MULTIPLY", offset);
            }
            OpCode::Divide => {
                return simple_instruction("OP_DIVIDE", offset);
            }
            OpCode::True => {
                return simple_instruction("OP_TRUE", offset);
            }
            OpCode::False => {
                return simple_instruction("OP_FALSE", offset);
            }
            OpCode::Nil => {
                return simple_instruction("OP_NIL", offset);
            }
            OpCode::Equal => {
                return simple_instruction("OP_EQUAL", offset);
            }
            OpCode::Greater => {
                return simple_instruction("OP_GREATER", offset);
            }
            OpCode::Less => {
                return simple_instruction("OP_LESS", offset);
            }
            OpCode::Negate => {
                return simple_instruction("OP_NEGATE", offset);
            }
            OpCode::Not => {
                return simple_instruction("OP_NOT", offset);
            }
            OpCode::Pop => {
                return simple_instruction("OP_POP", offset);
            }
            OpCode::Print => {
                return simple_instruction("OP_PRINT", offset);
            }
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

    fn simple_instruction(name: &str, offset: usize) -> (String, usize) {
        return (format!("{}\n", name), offset + 1);
    }

    fn disassemble_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
        let instruction = OpCode::from_u8(chunk.code[offset]).unwrap();

        match instruction {
            OpCode::Return => {
                return simple_instruction("OP_RETURN", offset);
            }
            OpCode::Constant => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::Add => {
                return simple_instruction("OP_ADD", offset);
            }
            OpCode::Subtract => {
                return simple_instruction("OP_SUBTRACT", offset);
            }
            OpCode::Multiply => {
                return simple_instruction("OP_MULTIPLY", offset);
            }
            OpCode::Divide => {
                return simple_instruction("OP_DIVIDE", offset);
            }
            OpCode::True => {
                return simple_instruction("OP_TRUE", offset);
            }
            OpCode::False => {
                return simple_instruction("OP_FALSE", offset);
            }
            OpCode::Nil => {
                return simple_instruction("OP_NIL", offset);
            }
            OpCode::Equal => {
                return simple_instruction("OP_EQUAL", offset);
            }
            OpCode::Greater => {
                return simple_instruction("OP_GREATER", offset);
            }
            OpCode::Less => {
                return simple_instruction("OP_LESS", offset);
            }
            OpCode::Negate => {
                return simple_instruction("OP_NEGATE", offset);
            }
            OpCode::Not => {
                return simple_instruction("OP_NOT", offset);
            }
            OpCode::Pop => {
                return simple_instruction("OP_POP", offset);
            }
            OpCode::Print => {
                return simple_instruction("OP_PRINT", offset);
            }
        }
    }

    pub fn write_chunk_to_file(source: String, chunk: &Chunk, output_path: &str) {
        let mut file = File::create(output_path)
            .expect(format!("Could not open file {}", output_path).as_str());

        let mut offset = 0;
        let mut debug_string: String;
        let mut current_line = 0;
        let source_lines: Vec<&str> = source.split('\n').collect();

        while offset < chunk.code.len() {
            if chunk.lines[offset] != current_line {
                current_line = chunk.lines[offset];
                file.write_all(format!("\n\n{}\n\n", source_lines[current_line - 1]).as_bytes())
                    .expect("Couldn't write to file");
            }

            (debug_string, offset) = disassemble_instruction(chunk, offset);

            file.write_all(debug_string.as_bytes())
                .expect("Couldn't write to file");
        }
    }
}
