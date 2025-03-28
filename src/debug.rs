use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

fn get_value_debug_string(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => format!("{}", v),
        Value::Number(v) => format!("{}", v),
        Value::String(v) => format!("'{}'", v),
        Value::Function(v) => match &v.name {
            Some(name) => {
                format!("<fn {}>", name)
            }
            None => {
                format!("<script>")
            }
        },
        Value::NativeFunction(v) => format!("<native fn {}>", v.name),
        Value::Closure(v) => match &v.function.name {
            Some(name) => {
                format!("<fn {}>", name)
            }
            None => {
                format!("<script>")
            }
        },
        Value::Upvalue(up) => format!("<upvalue {:?}>", up),
        Value::Class(c) => format!("{}", c.name),
        Value::Instance(i) => format!("{}", i.borrow().class.name),
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
                println!("{}: {}", OpCode::Constant, get_value_debug_string(constant));

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
            OpCode::DefineGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];
                println!(
                    "{}: {}",
                    OpCode::DefineGlobal,
                    get_value_debug_string(constant)
                );

                return offset + 2;
            }
            OpCode::GetGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];
                println!(
                    "{}: {}",
                    OpCode::GetGlobal,
                    get_value_debug_string(constant)
                );

                return offset + 2;
            }
            OpCode::SetGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];
                println!(
                    "{}: {}",
                    OpCode::SetGlobal,
                    get_value_debug_string(constant)
                );

                return offset + 2;
            }
            OpCode::GetLocal => {
                let slot = chunk.code[offset + 1];
                println!("{}: {}", OpCode::GetLocal, slot);
                return offset + 2;
            }
            OpCode::SetLocal => {
                let slot = chunk.code[offset + 1];
                println!("{}: {}", OpCode::SetLocal, slot);
                return offset + 2;
            }
            OpCode::JumpIfFalse => {
                let jump = (chunk.code[offset + 1] as u16) << 8 | chunk.code[offset + 2] as u16;
                println!(
                    "{} {} -> {}",
                    OpCode::JumpIfFalse,
                    offset,
                    offset + 3 + jump as usize
                );

                return offset + 3;
            }
            OpCode::Jump => {
                let jump = (chunk.code[offset + 1] as u16) << 8 | chunk.code[offset + 2] as u16;
                println!(
                    "{} {} -> {}",
                    OpCode::Jump,
                    offset,
                    offset + 3 + jump as usize
                );
                return offset + 3;
            }
            OpCode::Loop => {
                println!("op code loop");
                return offset + 3;
            }
            OpCode::Call => {
                let slot = chunk.code[offset + 1];
                println!("OP_CALL {}", slot);
                return offset + 2;
            }
            OpCode::Closure => {
                let slot = chunk.code[offset + 1];
                let value = &chunk.constants[slot as usize];
                let mut offset_inc_value = 2;

                match value {
                    Value::Function(function) => {
                        println!("OP_CLOSURE {:?}", function.name);

                        for idx in 0..(function.upvalue_count as usize) {
                            // at idx = 0, the index for the array access here is offset + 1 + 0 + 1
                            // = offset + 2
                            // which is what we want because offset + 1 is the index of the function value itself
                            // and so the following chunk code location is the location of the is_local byte
                            // and then the following code location after that is the index byte
                            let is_local = chunk.code[(offset + 1) + (2 * idx + 1)];
                            let index = chunk.code[(offset + 1) + (2 * idx + 2)];

                            println!("is local: {}\nindex: {}", is_local, index);
                        }
                        offset_inc_value += 2 * function.upvalue_count;
                    }
                    v => panic!("Expect function at slot {} but received {:?}", slot, v),
                }

                return offset + offset_inc_value as usize;
            }
            OpCode::GetUpvalue => {
                let slot = chunk.code[offset + 1];
                println!("{}: {}", OpCode::GetUpvalue, slot);

                return offset + 2;
            }
            OpCode::SetUpvalue => {
                let slot = chunk.code[offset + 1];
                println!("{}: {}", OpCode::SetUpvalue, slot);

                return offset + 2;
            }
            OpCode::CloseUpvalue => {
                return simple_instruction(format!("{}", OpCode::CloseUpvalue).as_str(), offset)
            }
            OpCode::Class => {
                todo!("class in disassemble_instruction");
            }
            OpCode::GetProperty => {
                todo!("get property");
            }
            OpCode::SetProperty => {
                todo!("set property");
            }
        }
    }

    pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
        println!("==== {} ====\n\n", name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = disassemble_instruction(chunk, offset);
        }

        println!("\n\n==== END CHUNK DISASSEMBLY ====\n\n");
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
            OpCode::DefineGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_DEFINE_GLOBAL\nOP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::GetGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_GET_GLOBAL\nOP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::SetGlobal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_SET_GLOBAL\nOP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::GetLocal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_GET_LOCAL\nOP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::SetLocal => {
                let constant = &chunk.constants[chunk.code[offset + 1] as usize];

                return (
                    format!(
                        "OP_SET_LOCAL\nOP_CONSTANT\nCONSTANT: {}\n",
                        get_value_debug_string(constant)
                    ),
                    offset + 2,
                );
            }
            OpCode::JumpIfFalse => {
                let jump = (chunk.code[offset + 1] as u16) << 8 | chunk.code[offset + 2] as u16;
                return (
                    format!(
                        "{} {} -> {}\n",
                        OpCode::JumpIfFalse,
                        offset,
                        offset + 3 + jump as usize
                    ),
                    offset + 3,
                );
            }
            OpCode::Jump => {
                let jump = (chunk.code[offset + 1] as u16) << 8 | chunk.code[offset + 2] as u16;
                return (
                    format!(
                        "{} {} -> {}\n",
                        OpCode::Jump,
                        offset,
                        offset + 3 + jump as usize
                    ),
                    offset + 3,
                );
            }
            OpCode::Loop => return ("opcode loop".to_owned(), offset + 3),
            OpCode::Call => {
                let slot = chunk.code[offset + 1];
                return (format!("OP_CALL {}", slot), offset + 2);
            }
            OpCode::Closure => {
                let slot = chunk.code[offset + 1];
                return (format!("OP_CLOSURE {}", slot), offset + 2);
            }
            OpCode::GetUpvalue => {
                todo!("get upvalue");
            }
            OpCode::SetUpvalue => {
                todo!("set upvalue");
            }
            OpCode::CloseUpvalue => {
                todo!("close upvalue in debug");
            }
            OpCode::Class => {
                todo!("class in debug to file");
            }
            OpCode::GetProperty => {
                todo!("get property");
            }
            OpCode::SetProperty => {
                todo!("set property");
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
