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
            _ => None,
        }
    }
}

#[derive(Debug)]
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

    pub fn write_constant(&mut self, constant: f64) -> usize {
        self.constants.push(Value::Number(constant));
        return self.constants.len() - 1;
    }

    pub fn write_string(&mut self, s: String) -> usize {
        self.constants.push(Value::String(s));
        return self.constants.len() - 1;
    }
}
