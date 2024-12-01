use std::{
    array,
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{Compiler, FunctionType},
    scanner::Scanner,
    value::{Function, NativeFunction, Value},
};

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub struct CallFrame {
    pub function: Function,
    ip: usize,
    slot: usize, // <-- pointer into vm value stack
}

pub trait ValueStack {
    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Option<Value>;
    fn last_value(&mut self) -> Option<Value>;
    fn get_value_at_idx(&self, index: usize) -> Value;
    fn set_value_at_idx(&mut self, index: usize, value: Value);
    fn peek(&self, distance: usize) -> Value;
    fn size(&self) -> usize;

    #[allow(dead_code)]
    fn print_debug(&self) -> ();
}

impl ValueStack for Vec<Value> {
    fn push(&mut self, value: Value) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        return self.pop();
    }

    fn last_value(&mut self) -> Option<Value> {
        return self.last().cloned();
    }

    fn get_value_at_idx(&self, index: usize) -> Value {
        return self[index].clone();
    }

    fn set_value_at_idx(&mut self, index: usize, value: Value) {
        self[index] = value;
    }

    fn peek(&self, distance: usize) -> Value {
        return self.get_value_at_idx(self.len() - 1 - distance);
    }

    fn print_debug(&self) -> () {
        let mut count = 0;
        for val in self.iter() {
            println!("Value {} -- {:?}", count, val);
            count += 1;
        }
    }

    fn size(&self) -> usize {
        return self.len();
    }
}

const MAX_FRAMES: usize = 64;

pub struct VM<T: ValueStack> {
    pub chunk: Chunk,
    pub value_stack: T,

    globals: HashMap<String, Value>,

    pub frames: [CallFrame; MAX_FRAMES],
    frame_count: usize,
}

impl<T: ValueStack> VM<T> {
    pub fn new() -> VM<Vec<Value>> {
        let mut vm = VM {
            chunk: Chunk::new(),
            value_stack: Vec::new(),
            globals: HashMap::new(),

            frames: array::from_fn(move |_| CallFrame {
                function: Function::new(),
                ip: 0,
                slot: 0,
            }),
            frame_count: 0,
        };

        vm.globals.insert(
            String::from("clock"),
            Value::NativeFunction(NativeFunction {
                name: String::from("clock"),
                arity: 0,
            }),
        );

        return vm;
    }

    #[allow(dead_code)]
    pub fn new_with_value_stack(value_stack: T) -> VM<T> {
        VM {
            chunk: Chunk::new(),
            value_stack,
            globals: HashMap::new(),

            frames: array::from_fn(move |_| CallFrame {
                function: Function::new(),
                ip: 0,
                slot: 0,
            }),
            frame_count: 0,
        }
    }

    fn is_falsey(value: Value) -> bool {
        match value {
            Value::Nil => return true,
            Value::Boolean(tf) => return !tf,
            _ => return false,
        }
    }

    fn print_value(value: Value) {
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
            Value::Function(func) => match func.name {
                Some(name) => {
                    println!("<fn {}>", name)
                }
                None => {
                    println!("<script>")
                }
            },
            Value::NativeFunction(_func) => {
                println!("<native fn>");
            }
        }
    }

    fn call(&mut self, func: Function, arg_count: u8) -> bool {
        if arg_count != func.arity {
            // TODO: runtime error
            println!("Expected {} arguments but got {}", func.arity, arg_count);
            return false;
        }

        if self.frame_count == MAX_FRAMES {
            // TODO: runtime error
            println!("Stack overflow.");
            return false;
        }

        self.frames[self.frame_count].function = func;
        self.frames[self.frame_count].ip = 0;
        self.frames[self.frame_count].slot = self.value_stack.size() - (arg_count as usize) - 1;

        self.frame_count += 1;

        return true;
    }

    fn call_native(&mut self, func: NativeFunction, arg_count: u8) -> bool {
        if arg_count != func.arity {
            // TODO: runtime error
            println!("Expected {} arguments but got {}", func.arity, arg_count);
            return false;
        }

        if self.frame_count == MAX_FRAMES {
            // TODO: runtime error
            println!("Stack overflow.");
            return false;
        }

        match func.name.as_str() {
            "clock" => {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("time went backwards.");
                self.value_stack
                    .push(Value::Number(since_the_epoch.as_millis() as f64));
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn call_value(&mut self, callee: Value, arg_count: u8) -> bool {
        match callee {
            Value::Function(func) => {
                return self.call(func, arg_count);
            }
            Value::NativeFunction(func) => {
                return self.call_native(func, arg_count);
            }
            _ => {
                // TODO: runtime error
                return false;
            }
        }
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! frame {
            () => {
                &mut self.frames[self.frame_count - 1]
            };
        }

        macro_rules! read_byte {
            () => {{
                frame!().ip += 1;
                let ip = frame!().ip;
                frame!().function.chunk.code[ip - 1]
            }};
        }

        macro_rules! get_instruction {
            () => {{
                frame!().ip += 1;
                let ip = frame!().ip;
                OpCode::from_u8(frame!().function.chunk.code[ip - 1])
            }};
        }

        macro_rules! read_constant {
            () => {{
                frame!().ip += 1;
                let ip = frame!().ip;
                let constant_index = frame!().function.chunk.code[ip - 1];
                &frame!().function.chunk.constants[constant_index as usize]
            }};
        }

        macro_rules! read_short {
            () => {{
                frame!().ip += 2;
                let ip = frame!().ip;
                let first = (frame!().function.chunk.code[ip - 2] as u16) << 8;
                let second = frame!().function.chunk.code[ip - 1] as u16;

                first | second
            }};
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
                        _ => {
                            let ip = frame!().ip;
                            let line = frame!().function.chunk.lines[ip];

                            println!("[Error on line {}]\nPerforming binary operation because LHS isn't a number. LHS = {:?}", line, a);
                            return InterpretResult::RuntimeError;
                        }
                    },
                    _ => {
                        let ip = frame!().ip;
                        let line = frame!().function.chunk.lines[ip];

                        println!("[Error on line {}]\nPerforming binary operation because RHS isn't a number. RHS = {:?}", line, b);
                        return InterpretResult::RuntimeError;
                    }
                }
            };
        }

        loop {
            let instruction = get_instruction!().unwrap();

            match instruction {
                OpCode::Return => {
                    let result = self.value_stack.pop();
                    let slot = frame!().slot;

                    self.frame_count -= 1;

                    if self.frame_count == 0 {
                        self.value_stack.pop();
                        return InterpretResult::Ok;
                    }

                    while self.value_stack.size() > slot {
                        self.value_stack.pop();
                    }
                    self.value_stack.push(result.unwrap());
                }
                OpCode::Constant => {
                    let constant = read_constant!();
                    self.value_stack.push(constant.clone());
                }
                OpCode::Add => {
                    let b = self.value_stack.pop();
                    let a = self.value_stack.pop();

                    match b {
                        Some(Value::Number(num2)) => match a {
                            Some(Value::Number(num1)) => {
                                self.value_stack.push(Value::Number(num1 + num2));
                            }
                            Some(Value::String(s1)) => self
                                .value_stack
                                .push(Value::String(format!("{}{}", s1, num2))),
                            _ => return InterpretResult::RuntimeError,
                        },
                        Some(Value::String(s2)) => match a {
                            Some(Value::String(s1)) => {
                                self.value_stack
                                    .push(Value::String(format!("{}{}", s1, s2)));
                            }
                            Some(Value::Number(n)) => {
                                self.value_stack.push(Value::String(format!("{}{}", n, s2)));
                            }
                            _ => return InterpretResult::RuntimeError,
                        },
                        _ => return InterpretResult::RuntimeError,
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
                        Some(value) => self
                            .value_stack
                            .push(Value::Boolean(VM::<T>::is_falsey(value))),
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
                        Some(Value::Function(_)) => return InterpretResult::RuntimeError,
                        Some(Value::NativeFunction(_)) => return InterpretResult::RuntimeError,
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
                    Some(v) => VM::<T>::print_value(v),
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Pop => {
                    self.value_stack.pop();
                }
                OpCode::DefineGlobal => {
                    let name = read_constant!();

                    match name {
                        Value::String(s) => {
                            let value = self.value_stack.last_value().unwrap();

                            self.globals.insert(s.to_owned(), value);
                            self.value_stack.pop();
                        }
                        _ => {
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::GetGlobal => {
                    let name = read_constant!();

                    match name {
                        Value::String(s) => {
                            let optional_value = self.globals.get(s);
                            match optional_value {
                                Some(value) => {
                                    self.value_stack.push(value.to_owned());
                                }
                                None => {
                                    // TODO: Add better error handling here
                                    return InterpretResult::RuntimeError;
                                }
                            }
                        }
                        _ => {
                            // TODO: Add better error handling here
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::SetGlobal => {
                    let name = read_constant!();

                    match name {
                        Value::String(s) => {
                            if !self.globals.contains_key(s) {
                                // TODO: Add better error handling here
                                return InterpretResult::RuntimeError;
                            }
                            let value = self.value_stack.last_value().unwrap();
                            self.globals.insert(s.to_owned(), value);
                        }
                        _ => {
                            // TODO: Add better error handling here
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::GetLocal => {
                    let slot = read_byte!() + frame!().slot as u8;
                    self.value_stack
                        .push(self.value_stack.get_value_at_idx(slot as usize));
                }
                OpCode::SetLocal => {
                    let slot = read_byte!() + frame!().slot as u8;
                    let top_value = self.value_stack.peek(0);
                    self.value_stack.set_value_at_idx(slot as usize, top_value);
                }
                OpCode::JumpIfFalse => {
                    let offset = read_short!();
                    if VM::<T>::is_falsey(self.value_stack.peek(0)) {
                        frame!().ip += offset as usize;
                    }
                }
                OpCode::Jump => {
                    let offset = read_short!();
                    frame!().ip += offset as usize;
                }
                OpCode::Loop => {
                    let offset = read_short!();
                    frame!().ip -= offset as usize;
                }
                OpCode::Call => {
                    let arg_count = read_byte!();
                    let callee = self.value_stack.peek(arg_count as usize).clone();

                    if !self.call_value(callee, arg_count) {
                        return InterpretResult::RuntimeError;
                    }
                }
            }
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let scanner = Scanner::new(source);
        let mut compiler = Compiler::new(scanner, FunctionType::Script);

        let compile_result = compiler.compile(None);
        match compile_result {
            None => return InterpretResult::CompileError,
            Some(func) => {
                self.value_stack.push(Value::Function(func.to_owned()));
                self.call(func.to_owned(), 0);
            }
        }

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

        fn last_value(&mut self) -> Option<Value> {
            return self.values.last().cloned();
        }

        fn get_value_at_idx(&self, index: usize) -> Value {
            return self.values[index].clone();
        }

        fn set_value_at_idx(&mut self, index: usize, value: Value) {
            self.values[index] = value;
        }

        fn peek(&self, distance: usize) -> Value {
            return self.get_value_at_idx(self.values.len() - 1 - distance);
        }

        fn print_debug(&self) -> () {
            println!("{:?}", self.values);
        }

        fn size(&self) -> usize {
            return self.values.len();
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

    // The last value is always implicitly `Nil` due to the function semantics of the language
    // so the second to last value is the one that's the result of actual computation.
    fn get_second_to_last_value_on_value_stack(
        source: String,
        value_stack: TestValueStack,
    ) -> Option<Value> {
        let source = String::from(source);
        let mut vm = VM::new_with_value_stack(value_stack);

        vm.interpret(source);

        vm.value_stack.all_values.pop();
        return vm.value_stack.all_values.pop();
    }

    #[test]
    fn basic_arithmetic() {
        let last_value = get_second_to_last_value_on_value_stack(
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
        let last_value = get_second_to_last_value_on_value_stack(
            String::from("2 > 3;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(false)) => {}
            _ => panic!("Expected false, got {:?}", last_value),
        }

        // Expect true
        let last_value = get_second_to_last_value_on_value_stack(
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
        let last_value = get_second_to_last_value_on_value_stack(
            String::from("3 < 2;"),
            TestValueStack::new(&mut Vec::new()),
        );
        match last_value {
            Some(Value::Boolean(false)) => {}
            _ => panic!("Expected false, got {:?}", last_value),
        }

        // Expect true
        let last_value = get_second_to_last_value_on_value_stack(
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
        let last_value = get_second_to_last_value_on_value_stack(
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
