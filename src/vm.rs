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
    pub chunk: Chunk,
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

    fn is_falsey(&self, value: Value) -> bool {
        match value {
            Value::Nil => return true,
            Value::Boolean(tf) => return !tf,
            _ => return false,
        }
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! get_instruction {
            () => {
                self.chunk.code[self.ip]
            };
        }

        macro_rules! binary_op {
            ($op:tt) => {
                let b = self.value_stack.pop().expect("");
                let a = self.value_stack.pop().expect("");

                match b {
                    Value::Number(num2) => match a {
                        Value::Number(num1) => {
                            self.value_stack.push(Value::Number(num1 $op num2));
                        }
                        _ => return InterpretResult::RuntimeError,
                    },
                    Value::Boolean(_) => return InterpretResult::RuntimeError,
                    Value::Nil => return InterpretResult::RuntimeError,
                    Value::String(_) => return InterpretResult::RuntimeError,
                }
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

                self.value_stack.push(value.clone());
            } else if instruction == OpCode::Add as u8 {
                binary_op!(+);
            } else if instruction == OpCode::Subtract as u8 {
                binary_op!(-);
            } else if instruction == OpCode::Multiply as u8 {
                binary_op!(*);
            } else if instruction == OpCode::Divide as u8 {
                binary_op!(/);
            } else if instruction == OpCode::True as u8 {
                self.value_stack.push(Value::Boolean(true));
            } else if instruction == OpCode::False as u8 {
                self.value_stack.push(Value::Boolean(false));
            } else if instruction == OpCode::Nil as u8 {
                self.value_stack.push(Value::Nil);
            } else if instruction == OpCode::Not as u8 {
                let v = self.value_stack.pop();

                match v {
                    Some(value) => self.value_stack.push(Value::Boolean(self.is_falsey(value))),
                    _ => return InterpretResult::RuntimeError,
                }
            } else if instruction == OpCode::Negate as u8 {
                let v = self.value_stack.pop();

                match v {
                    Some(Value::Number(n)) => self.value_stack.push(Value::Number(-n)),
                    _ => return InterpretResult::RuntimeError,
                }
            } else if instruction == OpCode::Equal as u8 {
                let b = self.value_stack.pop();
                let a = self.value_stack.pop();

                match b {
                    Some(Value::Number(num2)) => match a {
                        Some(Value::Number(num1)) => {
                            self.value_stack.push(Value::Boolean(num1 == num2))
                        }
                        None => return InterpretResult::RuntimeError,
                        _ => self.value_stack.push(Value::Boolean(false)),
                    },
                    Some(Value::Boolean(tf2)) => match a {
                        Some(Value::Boolean(tf1)) => {
                            self.value_stack.push(Value::Boolean(tf1 == tf2))
                        }
                        None => return InterpretResult::RuntimeError,
                        _ => self.value_stack.push(Value::Boolean(false)),
                    },
                    Some(Value::Nil) => match a {
                        Some(Value::Nil) => self.value_stack.push(Value::Boolean(true)),
                        None => return InterpretResult::RuntimeError,
                        _ => self.value_stack.push(Value::Boolean(false)),
                    },
                    Some(Value::String(s2)) => match a {
                        Some(Value::String(s1)) => {
                            self.value_stack.push(Value::Boolean(s1.eq(&s2)));
                        }
                        _ => self.value_stack.push(Value::Boolean(false)),
                    },
                    None => return InterpretResult::RuntimeError,
                }
            }

            // TODO: handle Greater and Less OpCodes

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
