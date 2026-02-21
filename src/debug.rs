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
        Value::Class(c) => format!("{}", c.name),
        Value::Instance(i) => format!("{}", i.borrow().class.name),
    }
}

fn next_offset(offset: usize, advance: usize) -> usize {
    let next = offset.saturating_add(advance);
    if next > offset {
        next
    } else {
        offset + 1
    }
}

fn constant_operand_string(chunk: &Chunk, operand: Option<u8>) -> String {
    match operand {
        Some(index) => match chunk.constants.get(index as usize) {
            Some(constant) => get_value_debug_string(constant),
            None => format!("<invalid constant {}>", index),
        },
        None => "<missing operand>".to_string(),
    }
}

fn read_jump_operand(chunk: &Chunk, offset: usize) -> Option<u16> {
    let high = *chunk.code.get(offset + 1)? as u16;
    let low = *chunk.code.get(offset + 2)? as u16;
    Some((high << 8) | low)
}

fn read_line(chunk: &Chunk, offset: usize) -> usize {
    chunk.lines.get(offset).copied().unwrap_or(0)
}

pub mod print_debug {
    use super::*;

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        next_offset(offset, 1)
    }

    fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        if offset >= chunk.code.len() {
            return next_offset(offset, 1);
        }

        print!("CHUNK OFFSET - {:0>4} | ", offset);

        let line = read_line(chunk, offset);
        let previous_line = if offset > 0 {
            Some(read_line(chunk, offset - 1))
        } else {
            None
        };

        if previous_line == Some(line) {
            print!("LINE -    | ");
        } else {
            print!("LINE - {:0>4} ", line);
        }

        let op_byte = chunk.code[offset];
        let Some(instruction) = OpCode::from_u8(op_byte) else {
            println!("OP_UNKNOWN {}", op_byte);
            return next_offset(offset, 1);
        };

        match instruction {
            OpCode::Return => {
                println!("OP_RETURN");
                next_offset(offset, 1)
            }
            OpCode::Constant => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::Constant,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::True => simple_instruction("OP_TRUE", offset),
            OpCode::False => simple_instruction("OP_FALSE", offset),
            OpCode::Nil => simple_instruction("OP_NIL", offset),
            OpCode::Equal => simple_instruction("OP_EQUAL", offset),
            OpCode::Greater => simple_instruction("OP_GREATER", offset),
            OpCode::Less => simple_instruction("OP_LESS", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Not => simple_instruction("OP_NOT", offset),
            OpCode::Pop => simple_instruction("OP_POP", offset),
            OpCode::Print => simple_instruction("OP_PRINT", offset),
            OpCode::DefineGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::DefineGlobal,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::GetGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::GetGlobal,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::SetGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::SetGlobal,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::GetLocal => {
                let operand = chunk.code.get(offset + 1).copied();
                match operand {
                    Some(slot) => println!("{}: {}", OpCode::GetLocal, slot),
                    None => println!("{}: <missing operand>", OpCode::GetLocal),
                }

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::SetLocal => {
                let operand = chunk.code.get(offset + 1).copied();
                match operand {
                    Some(slot) => println!("{}: {}", OpCode::SetLocal, slot),
                    None => println!("{}: <missing operand>", OpCode::SetLocal),
                }

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::JumpIfFalse => {
                match read_jump_operand(chunk, offset) {
                    Some(jump) => {
                        let target = offset.saturating_add(3).saturating_add(jump as usize);
                        println!("{} {} -> {}", OpCode::JumpIfFalse, offset, target);
                    }
                    None => println!("{}: <missing jump operand>", OpCode::JumpIfFalse),
                }

                next_offset(
                    offset,
                    if read_jump_operand(chunk, offset).is_some() {
                        3
                    } else {
                        1
                    },
                )
            }
            OpCode::Jump => {
                match read_jump_operand(chunk, offset) {
                    Some(jump) => {
                        let target = offset.saturating_add(3).saturating_add(jump as usize);
                        println!("{} {} -> {}", OpCode::Jump, offset, target);
                    }
                    None => println!("{}: <missing jump operand>", OpCode::Jump),
                }

                next_offset(
                    offset,
                    if read_jump_operand(chunk, offset).is_some() {
                        3
                    } else {
                        1
                    },
                )
            }
            OpCode::Loop => {
                match read_jump_operand(chunk, offset) {
                    Some(jump) => {
                        let target = offset.saturating_add(3).saturating_sub(jump as usize);
                        println!("{} {} -> {}", OpCode::Loop, offset, target);
                    }
                    None => println!("{}: <missing jump operand>", OpCode::Loop),
                }

                next_offset(
                    offset,
                    if read_jump_operand(chunk, offset).is_some() {
                        3
                    } else {
                        1
                    },
                )
            }
            OpCode::Call => {
                let operand = chunk.code.get(offset + 1).copied();
                match operand {
                    Some(arg_count) => println!("OP_CALL {}", arg_count),
                    None => println!("OP_CALL <missing arg count>"),
                }

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::Closure => {
                let operand = chunk.code.get(offset + 1).copied();
                let Some(slot) = operand else {
                    println!("OP_CLOSURE <missing function operand>");
                    return next_offset(offset, 1);
                };

                let value = chunk.constants.get(slot as usize);
                let mut advance = 2;

                match value {
                    Some(Value::Function(function)) => {
                        println!("OP_CLOSURE {:?}", function.name);

                        let upvalue_count = function.upvalue_count as usize;
                        for idx in 0..upvalue_count {
                            let is_local = chunk.code.get(offset + 2 + idx * 2).copied();
                            let index = chunk.code.get(offset + 3 + idx * 2).copied();

                            println!(
                                "upvalue {}: is_local={}, index={}",
                                idx,
                                is_local
                                    .map(|v| v.to_string())
                                    .unwrap_or("<missing>".to_string()),
                                index
                                    .map(|v| v.to_string())
                                    .unwrap_or("<missing>".to_string())
                            );
                        }
                        advance += upvalue_count * 2;
                    }
                    Some(other) => {
                        println!(
                            "OP_CLOSURE <non-function constant: {}>",
                            get_value_debug_string(other)
                        );
                    }
                    None => {
                        println!("OP_CLOSURE <invalid constant {}>", slot);
                    }
                }

                next_offset(offset, advance)
            }
            OpCode::GetUpvalue => {
                let operand = chunk.code.get(offset + 1).copied();
                match operand {
                    Some(slot) => println!("{}: {}", OpCode::GetUpvalue, slot),
                    None => println!("{}: <missing operand>", OpCode::GetUpvalue),
                }

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::SetUpvalue => {
                let operand = chunk.code.get(offset + 1).copied();
                match operand {
                    Some(slot) => println!("{}: {}", OpCode::SetUpvalue, slot),
                    None => println!("{}: <missing operand>", OpCode::SetUpvalue),
                }

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::CloseUpvalue => {
                simple_instruction(format!("{}", OpCode::CloseUpvalue).as_str(), offset)
            }
            OpCode::Class => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::Class,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::GetProperty => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::GetProperty,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
            OpCode::SetProperty => {
                let operand = chunk.code.get(offset + 1).copied();
                println!(
                    "{}: {}",
                    OpCode::SetProperty,
                    constant_operand_string(chunk, operand)
                );

                next_offset(offset, if operand.is_some() { 2 } else { 1 })
            }
        }
    }

    pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
        println!("==== {} ====\n\n", name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            let next = disassemble_instruction(chunk, offset);
            offset = if next > offset { next } else { offset + 1 };
        }

        println!("\n\n==== END CHUNK DISASSEMBLY ====\n\n");
    }
}

pub mod write_debug {
    use std::{fs::File, io::Write};

    use super::*;

    fn simple_instruction(name: &str, offset: usize) -> (String, usize) {
        (format!("{}\n", name), next_offset(offset, 1))
    }

    fn disassemble_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
        if offset >= chunk.code.len() {
            return ("OP_EOF\n".to_string(), next_offset(offset, 1));
        }

        let op_byte = chunk.code[offset];
        let Some(instruction) = OpCode::from_u8(op_byte) else {
            return (format!("OP_UNKNOWN {}\n", op_byte), next_offset(offset, 1));
        };

        match instruction {
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            OpCode::Constant => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_CONSTANT\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::True => simple_instruction("OP_TRUE", offset),
            OpCode::False => simple_instruction("OP_FALSE", offset),
            OpCode::Nil => simple_instruction("OP_NIL", offset),
            OpCode::Equal => simple_instruction("OP_EQUAL", offset),
            OpCode::Greater => simple_instruction("OP_GREATER", offset),
            OpCode::Less => simple_instruction("OP_LESS", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Not => simple_instruction("OP_NOT", offset),
            OpCode::Pop => simple_instruction("OP_POP", offset),
            OpCode::Print => simple_instruction("OP_PRINT", offset),
            OpCode::DefineGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_DEFINE_GLOBAL\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::GetGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_GET_GLOBAL\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::SetGlobal => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_SET_GLOBAL\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::GetLocal => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_GET_LOCAL\nSLOT: {}\n",
                        operand
                            .map(|v| v.to_string())
                            .unwrap_or("<missing operand>".to_string())
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::SetLocal => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_SET_LOCAL\nSLOT: {}\n",
                        operand
                            .map(|v| v.to_string())
                            .unwrap_or("<missing operand>".to_string())
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::JumpIfFalse => {
                let jump = read_jump_operand(chunk, offset);
                (
                    match jump {
                        Some(jump) => {
                            let target = offset.saturating_add(3).saturating_add(jump as usize);
                            format!("{} {} -> {}\n", OpCode::JumpIfFalse, offset, target)
                        }
                        None => format!("{}: <missing jump operand>\n", OpCode::JumpIfFalse),
                    },
                    next_offset(offset, if jump.is_some() { 3 } else { 1 }),
                )
            }
            OpCode::Jump => {
                let jump = read_jump_operand(chunk, offset);
                (
                    match jump {
                        Some(jump) => {
                            let target = offset.saturating_add(3).saturating_add(jump as usize);
                            format!("{} {} -> {}\n", OpCode::Jump, offset, target)
                        }
                        None => format!("{}: <missing jump operand>\n", OpCode::Jump),
                    },
                    next_offset(offset, if jump.is_some() { 3 } else { 1 }),
                )
            }
            OpCode::Loop => {
                let jump = read_jump_operand(chunk, offset);
                (
                    match jump {
                        Some(jump) => {
                            let target = offset.saturating_add(3).saturating_sub(jump as usize);
                            format!("{} {} -> {}\n", OpCode::Loop, offset, target)
                        }
                        None => format!("{}: <missing jump operand>\n", OpCode::Loop),
                    },
                    next_offset(offset, if jump.is_some() { 3 } else { 1 }),
                )
            }
            OpCode::Call => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_CALL {}\n",
                        operand
                            .map(|v| v.to_string())
                            .unwrap_or("<missing arg count>".to_string())
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::Closure => {
                let operand = chunk.code.get(offset + 1).copied();
                let Some(slot) = operand else {
                    return (
                        "OP_CLOSURE <missing function operand>\n".to_string(),
                        next_offset(offset, 1),
                    );
                };

                match chunk.constants.get(slot as usize) {
                    Some(Value::Function(function)) => {
                        let mut output = format!("OP_CLOSURE {:?}\n", function.name);
                        let upvalue_count = function.upvalue_count as usize;

                        for idx in 0..upvalue_count {
                            let is_local = chunk.code.get(offset + 2 + idx * 2).copied();
                            let index = chunk.code.get(offset + 3 + idx * 2).copied();

                            output.push_str(
                                format!(
                                    "UPVALUE {} is_local={} index={}\n",
                                    idx,
                                    is_local
                                        .map(|v| v.to_string())
                                        .unwrap_or("<missing>".to_string()),
                                    index
                                        .map(|v| v.to_string())
                                        .unwrap_or("<missing>".to_string())
                                )
                                .as_str(),
                            );
                        }

                        (output, next_offset(offset, 2 + upvalue_count * 2))
                    }
                    Some(other) => (
                        format!(
                            "OP_CLOSURE <non-function constant: {}>\n",
                            get_value_debug_string(other)
                        ),
                        next_offset(offset, 2),
                    ),
                    None => (
                        format!("OP_CLOSURE <invalid constant {}>\n", slot),
                        next_offset(offset, 2),
                    ),
                }
            }
            OpCode::GetUpvalue => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_GET_UPVALUE {}\n",
                        operand
                            .map(|v| v.to_string())
                            .unwrap_or("<missing operand>".to_string())
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::SetUpvalue => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_SET_UPVALUE {}\n",
                        operand
                            .map(|v| v.to_string())
                            .unwrap_or("<missing operand>".to_string())
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::CloseUpvalue => simple_instruction("OP_CLOSE_UPVALUE", offset),
            OpCode::Class => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_CLASS\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::GetProperty => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_GET_PROPERTY\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
            OpCode::SetProperty => {
                let operand = chunk.code.get(offset + 1).copied();
                (
                    format!(
                        "OP_SET_PROPERTY\nCONSTANT: {}\n",
                        constant_operand_string(chunk, operand)
                    ),
                    next_offset(offset, if operand.is_some() { 2 } else { 1 }),
                )
            }
        }
    }

    pub fn write_chunk_to_file(source: String, chunk: &Chunk, output_path: &str) {
        let mut file = match File::create(output_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Could not open file {}: {}", output_path, e);
                return;
            }
        };

        let mut offset = 0;
        let mut current_line = 0;
        let source_lines: Vec<&str> = source.split('\n').collect();

        while offset < chunk.code.len() {
            let line = read_line(chunk, offset);
            if line != 0 && line != current_line {
                current_line = line;
                let source_line = source_lines
                    .get(current_line - 1)
                    .copied()
                    .unwrap_or("<source line unavailable>");

                if file
                    .write_all(format!("\n\n{}\n\n", source_line).as_bytes())
                    .is_err()
                {
                    eprintln!("Couldn't write source line to {}", output_path);
                    return;
                }
            }

            let (debug_string, next) = disassemble_instruction(chunk, offset);

            if file.write_all(debug_string.as_bytes()).is_err() {
                eprintln!("Couldn't write disassembly to {}", output_path);
                return;
            }

            offset = if next > offset { next } else { offset + 1 };
        }
    }
}
