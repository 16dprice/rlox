use std::fmt;

use crate::value::Value;

#[derive(Debug)]
#[allow(dead_code)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    Nil = 2,
    True = 3,
    False = 4,
    Equal = 5,
    Greater = 6,
    Less = 7,
    Negate = 8,
    Add = 9,
    Subtract = 10,
    Multiply = 11,
    Divide = 12,
    Not = 13,
    Print = 14,
    Pop = 15,
    DefineGlobal = 16,
    GetGlobal = 17,
    SetGlobal = 18,
    GetLocal = 19,
    SetLocal = 20,
    JumpIfFalse = 21,
    Jump = 22,
    Loop = 23,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::Return => {
                write!(f, "OP_RETURN")
            }
            OpCode::Constant => {
                write!(f, "OP_CONSTANT")
            }
            OpCode::Nil => {
                write!(f, "OP_NIL")
            }
            OpCode::True => {
                write!(f, "OP_TRUE")
            }
            OpCode::False => {
                write!(f, "OP_FALSE")
            }
            OpCode::Equal => {
                write!(f, "OP_EQUAL")
            }
            OpCode::Greater => {
                write!(f, "OP_GREATER")
            }
            OpCode::Less => {
                write!(f, "OP_LESS")
            }
            OpCode::Negate => {
                write!(f, "OP_NEGATE")
            }
            OpCode::Add => {
                write!(f, "OP_ADD")
            }
            OpCode::Subtract => {
                write!(f, "OP_SUBTRACT")
            }
            OpCode::Multiply => {
                write!(f, "OP_MULTIPLY")
            }
            OpCode::Divide => {
                write!(f, "OP_DIVIDE")
            }
            OpCode::Not => {
                write!(f, "OP_NOT")
            }
            OpCode::Print => {
                write!(f, "OP_PRINT")
            }
            OpCode::Pop => {
                write!(f, "OP_POP")
            }
            OpCode::DefineGlobal => {
                write!(f, "OP_DEFINE_GLOBAL")
            }
            OpCode::GetGlobal => {
                write!(f, "OP_GET_GLOBAL")
            }
            OpCode::SetGlobal => {
                write!(f, "OP_SET_GLOBAL")
            }
            OpCode::GetLocal => {
                write!(f, "OP_GET_LOCAL")
            }
            OpCode::SetLocal => {
                write!(f, "OP_SET_LOCAL")
            }
            OpCode::JumpIfFalse => {
                write!(f, "OP_JUMP_IF_FALSE")
            }
            OpCode::Jump => {
                write!(f, "OP_JUMP")
            }
            OpCode::Loop => {
                write!(f, "OP_LOOP")
            }
        }
    }
}

impl OpCode {
    pub fn from_u8(o: u8) -> Option<OpCode> {
        match o {
            0 => Some(OpCode::Return),
            1 => Some(OpCode::Constant),
            2 => Some(OpCode::Nil),
            3 => Some(OpCode::True),
            4 => Some(OpCode::False),
            5 => Some(OpCode::Equal),
            6 => Some(OpCode::Greater),
            7 => Some(OpCode::Less),
            8 => Some(OpCode::Negate),
            9 => Some(OpCode::Add),
            10 => Some(OpCode::Subtract),
            11 => Some(OpCode::Multiply),
            12 => Some(OpCode::Divide),
            13 => Some(OpCode::Not),
            14 => Some(OpCode::Print),
            15 => Some(OpCode::Pop),
            16 => Some(OpCode::DefineGlobal),
            17 => Some(OpCode::GetGlobal),
            18 => Some(OpCode::SetGlobal),
            19 => Some(OpCode::GetLocal),
            20 => Some(OpCode::SetLocal),
            21 => Some(OpCode::JumpIfFalse),
            22 => Some(OpCode::Jump),
            23 => Some(OpCode::Loop),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write_code(&mut self, code: u8, line: usize) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn write_number(&mut self, constant: f64) -> usize {
        self.constants.push(Value::Number(constant));
        return self.constants.len() - 1;
    }

    pub fn write_string(&mut self, s: String) -> usize {
        self.constants.push(Value::String(s));
        return self.constants.len() - 1;
    }
}
