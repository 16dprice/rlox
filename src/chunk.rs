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
    code: Vec<OpCode>,
    lines: Vec<u32>,
    // constants: ValueArray
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, code: OpCode, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }
}
