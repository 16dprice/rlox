use crate::value::Value;

#[derive(Debug)]
#[allow(dead_code)]
pub enum OpCode {
    Return,
    Constant,
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
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

    pub fn write_code(&mut self, code: u8, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, constant: f64) -> usize {
        self.constants.push(Value::Number(constant));
        return self.constants.len() - 1;
    }
}