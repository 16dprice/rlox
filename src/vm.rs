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

pub trait ValueStack {
    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Option<Value>;
}

impl ValueStack for Vec<Value> {
    fn push(&mut self, value: Value) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        return self.pop();
    }
}

pub struct VM<T: ValueStack> {
    pub chunk: Chunk,
    ip: usize,
    pub value_stack: T,
}

impl<T: ValueStack> VM<T> {
    pub fn new() -> VM<Vec<Value>> {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            value_stack: Vec::new(),
        }
    }

    pub fn new_with_value_stack(value_stack: T) -> VM<T> {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            value_stack,
        }
    }

    fn is_falsey(&self, value: Value) -> bool {
        match value {
            Value::Nil => return true,
            Value::Boolean(tf) => return !tf,
            _ => return false,
        }
    }

    fn print_value(&self, value: Value) {
        match value {
            Value::String(s) => {
                for i in s.split("\\n") {
                    println!("{}", i);
                }
            }
            Value::Number(n) => println!("{}", n),
            Value::Boolean(b) => {
                if b {
                    println!("true");
                } else {
                    println!("false");
                }
            }
            Value::Nil => println!("nil"),
        }
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! get_instruction {
            () => {
                OpCode::from_u8(self.chunk.code[self.ip])
            };
        }

        macro_rules! binary_op {
            ($op:tt) => {
                let b = self.value_stack.pop();
                let a = self.value_stack.pop();

                match b {
                    Some(Value::Number(num2)) => match a {
                        Some(Value::Number(num1)) => {
                            self.value_stack.push(Value::Number(num1 $op num2));
                        }
                        _ => return InterpretResult::RuntimeError,
                    },
                    Some(Value::Boolean(_)) => return InterpretResult::RuntimeError,
                    Some(Value::Nil) => return InterpretResult::RuntimeError,
                    Some(Value::String(_)) => return InterpretResult::RuntimeError,
                    None => return InterpretResult::RuntimeError,
                }
            };
        }

        loop {
            let instruction = get_instruction!().unwrap();

            match instruction {
                OpCode::Return => {
                    return InterpretResult::Ok;
                }
                OpCode::Constant => {
                    self.ip += 1;
                    let constant_index = get_instruction!().unwrap();
                    let value = &self.chunk.constants[constant_index as usize];

                    self.value_stack.push(value.clone());
                }
                OpCode::Add => {
                    let b = self.value_stack.pop();
                    let a = self.value_stack.pop();

                    match b {
                        Some(Value::Number(num2)) => match a {
                            Some(Value::Number(num1)) => {
                                self.value_stack.push(Value::Number(num1 + num2));
                            }
                            _ => return InterpretResult::RuntimeError,
                        },
                        Some(Value::String(s2)) => match a {
                            Some(Value::String(s1)) => {
                                self.value_stack
                                    .push(Value::String(format!("{}{}", s1, s2)));
                            }
                            _ => return InterpretResult::RuntimeError,
                        },
                        Some(Value::Boolean(_)) => return InterpretResult::RuntimeError,
                        Some(Value::Nil) => return InterpretResult::RuntimeError,
                        None => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Subtract => {
                    binary_op!(-);
                }
                OpCode::Multiply => {
                    binary_op!(*);
                }
                OpCode::Divide => {
                    binary_op!(/);
                }
                OpCode::True => {
                    self.value_stack.push(Value::Boolean(true));
                }
                OpCode::False => {
                    self.value_stack.push(Value::Boolean(false));
                }
                OpCode::Nil => {
                    self.value_stack.push(Value::Nil);
                }
                OpCode::Not => {
                    let v = self.value_stack.pop();

                    match v {
                        Some(value) => self.value_stack.push(Value::Boolean(self.is_falsey(value))),
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Negate => {
                    let v = self.value_stack.pop();

                    match v {
                        Some(Value::Number(n)) => self.value_stack.push(Value::Number(-n)),
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Equal => {
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
                OpCode::Greater => {
                    let b = self.value_stack.pop();
                    let a = self.value_stack.pop();

                    match b {
                        Some(Value::Number(num2)) => match a {
                            Some(Value::Number(num1)) => {
                                self.value_stack.push(Value::Boolean(num1 > num2))
                            }
                            _ => return InterpretResult::RuntimeError,
                        },
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Less => {
                    let b = self.value_stack.pop();
                    let a = self.value_stack.pop();

                    match b {
                        Some(Value::Number(num2)) => match a {
                            Some(Value::Number(num1)) => {
                                self.value_stack.push(Value::Boolean(num1 < num2))
                            }
                            _ => return InterpretResult::RuntimeError,
                        },
                        _ => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::Print => match self.value_stack.pop() {
                    Some(v) => self.print_value(v),
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Pop => {
                    self.value_stack.pop();
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    struct TestValueStack<'a> {
        all_values: &'a mut Vec<Value>,
        values: Vec<Value>,
    }

    impl<'a> ValueStack for TestValueStack<'a> {
        fn push(&mut self, value: Value) {
            self.all_values.push(value.clone());
            self.values.push(value);
        }

        fn pop(&mut self) -> Option<Value> {
            return self.values.pop();
        }
    }

    impl<'a> TestValueStack<'a> {
        pub fn new(all_values: &'a mut Vec<Value>) -> TestValueStack<'a> {
            TestValueStack {
                all_values,
                values: vec![],
            }
        }
    }

    fn get_last_value_on_value_stack(source: String, value_stack: TestValueStack) -> Option<Value> {
        let source = String::from(source);
        let mut vm = VM::new_with_value_stack(value_stack);

        vm.interpret(source);

        return vm.value_stack.all_values.pop();
    }

    #[test]
    fn basic_arithmetic() {
        let last_value = get_last_value_on_value_stack(
            String::from("1 + 2;"),
            TestValueStack::new(&mut Vec::new()),
        );

        match last_value {
            Some(Value::Number(n)) => {
                if n != 3.0 {
                    panic!("Expected 3.0, got {}", n);
                }
            }
            _ => panic!("Expected 3.0, got {:?}", last_value),
        }
    }

    #[test]
    fn simple_greater_than() {
        // Expect false
        let last_value = get_last_value_on_value_stack(
            String::from("2 > 3;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(false)) => {}
            _ => panic!("Expected false, got {:?}", last_value),
        }

        // Expect true
        let last_value = get_last_value_on_value_stack(
            String::from("3 > 2;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(true)) => {}
            _ => panic!("Expected true, got {:?}", last_value),
        }
    }

    #[test]
    fn simple_less_than() {
        // Expect false
        let last_value = get_last_value_on_value_stack(
            String::from("3 < 2;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(false)) => {}
            _ => panic!("Expected false, got {:?}", last_value),
        }

        // Expect true
        let last_value = get_last_value_on_value_stack(
            String::from("2 < 3;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(true)) => {}
            _ => panic!("Expected true, got {:?}", last_value),
        }
    }

    #[test]
    fn string_concatenation() {
        let last_value = get_last_value_on_value_stack(
            String::from("\"one \" + \"two \" + \"three\";"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::String(s)) => {
                if !s.eq("one two three") {
                    panic!("Expected 'one two three', got {:?}", s);
                }
            }
            _ => panic!("Expected 'one two three', got {:?}", last_value),
        }
    }
}
