use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::Value,
};

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    pub value_stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            value_stack: vec![],
        }
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! get_instruction {
            () => {
                self.chunk.code[self.ip]
            };
        }

        loop {
            let instruction = get_instruction!();

            if instruction == OpCode::Return as u8 {
                println!("{:?}", self.value_stack.pop());
                return InterpretResult::Ok;
            } else if instruction == OpCode::Constant as u8 {
                self.ip += 1;
                let constant_index = get_instruction!();
                let value = &self.chunk.constants[constant_index as usize];

                self.value_stack.push(*value);
            } else if instruction == OpCode::Add as u8 {
                let b = self.value_stack.pop().expect("");
                let a = self.value_stack.pop().expect("");

                match b {
                    Value::Number(num1) => match a {
                        Value::Number(num2) => {
                            self.value_stack.push(Value::Number(num1 + num2));
                        }
                        _ => return InterpretResult::RuntimeError,
                    },
                    Value::Boolean(_) => return InterpretResult::RuntimeError,
                }
            }

            self.ip += 1;
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let chunk = Chunk::new();
        let mut compiler = Compiler::new(source, chunk);

        if !compiler.compile(None) {
            return InterpretResult::CompileError;
        }

        self.ip = 0;
        self.chunk = compiler.compiling_chunk;

        return self.run();
    }
}
