use std::{
    array,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{Compiler, FunctionType},
    scanner::Scanner,
    value::{Closure, Function, Instance, NativeFunction, Upvalue, Value},
};

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub struct CallFrame {
    pub closure: Closure,
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

    open_upvalue_head: Option<Box<Upvalue>>,
}

impl<T: ValueStack> VM<T> {
    pub fn new() -> VM<Vec<Value>> {
        let mut vm = VM {
            chunk: Chunk::new(),
            value_stack: Vec::new(),

            globals: HashMap::new(),

            frames: array::from_fn(move |_| CallFrame {
                closure: Closure::new(Function::new()),
                ip: 0,
                slot: 0,
            }),
            frame_count: 0,

            open_upvalue_head: None,
        };

        vm.globals.insert(
            String::from("clock"),
            Value::NativeFunction(NativeFunction {
                name: String::from("clock"),
                arity: 0,
            }),
        );
        vm.globals.insert(
            String::from("limit"),
            Value::NativeFunction(NativeFunction {
                name: String::from("limit"),
                arity: 1,
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
                closure: Closure::new(Function::new()),
                ip: 0,
                slot: 0,
            }),
            frame_count: 0,

            open_upvalue_head: None,
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
            Value::Closure(closure) => match &closure.function.name {
                Some(name) => {
                    println!("<closure {}>", name);
                }
                None => {
                    println!("<closure>");
                }
            },
            Value::Upvalue(upvalue) => println!("{:?}", upvalue),
            Value::Class(c) => println!("{}", c.name),
            Value::Instance(i) => println!("{} instance", i.borrow().class.name),
        }
    }

    // print all but the current frame
    fn stack_trace(&self) -> String {
        let mut output = String::new();

        for frame_idx in 0..self.frame_count {
            let frame = &self.frames[frame_idx];
            let line = frame.closure.function.chunk.lines[frame.ip];

            match &frame.closure.function.name {
                Some(s) => {
                    output.push_str(
                        format!("Frame {} -- Call from {} on line {}\n", frame_idx, s, line)
                            .as_str(),
                    );
                }
                None => {
                    output.push_str(
                        format!("Frame {} -- Call from main on line {}\n", frame_idx, line)
                            .as_str(),
                    );
                }
            }
        }

        return output;
    }

    fn runtime_error(&self, message: &str) {
        let stack_trace = self.stack_trace();
        println!("{}\n{}", stack_trace, message);
    }

    fn call(&mut self, closure: Closure, arg_count: u8) -> bool {
        if arg_count != closure.function.arity {
            self.runtime_error(
                format!(
                    "Expected {} arguments but got {}",
                    closure.function.arity, arg_count
                )
                .as_str(),
            );
            return false;
        }

        if self.frame_count == MAX_FRAMES {
            self.runtime_error("Stack overflow.");
            return false;
        }

        self.frames[self.frame_count].closure = closure;
        self.frames[self.frame_count].ip = 0;
        self.frames[self.frame_count].slot = self.value_stack.size() - (arg_count as usize) - 1;

        self.frame_count += 1;

        return true;
    }

    #[allow(unreachable_code)]
    fn call_native(&mut self, func: NativeFunction, arg_count: u8) -> bool {
        if arg_count != func.arity {
            self.runtime_error(
                format!("Expected {} arguments but got {}", func.arity, arg_count).as_str(),
            );
            return false;
        }

        if self.frame_count == MAX_FRAMES {
            self.runtime_error("Stack overflow.");
            return false;
        }

        match func.name.as_str() {
            "clock" => {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("time went backwards.");

                self.value_stack.pop(); // pop off the function itself
                self.value_stack
                    .push(Value::Number(since_the_epoch.as_millis() as f64));

                return true;
            }
            "limit" => {
                todo!("Clean this up to do more interesting things");
                let maybe_number = self.value_stack.pop();
                self.value_stack.pop(); // pop off the function itself

                match maybe_number {
                    Some(Value::Closure(f)) => {
                        self.value_stack.push(Value::String(format!("{:?}", f)));
                        return true;
                    }
                    Some(Value::Number(number)) => {
                        let f = |x: f64| -> f64 {
                            if x < 0.0 {
                                return -1.0;
                            } else {
                                return 1.0;
                            }
                        };

                        let delta = 1.0 / 2.0_f64.powf(32.0);

                        let limit_from_left = f(number - delta);
                        let limit_from_right = f(number + delta);

                        let tol = 10.0_f64.powi(-6);

                        if (limit_from_left - limit_from_right).abs() < tol {
                            self.value_stack
                                .push(Value::Number((limit_from_left + limit_from_right) / 2.0));
                        } else {
                            self.value_stack.push(Value::Nil);
                        }

                        return true;
                    }
                    _ => {
                        self.runtime_error(
                            format!("Can't call <limit> with input {:?}", maybe_number).as_str(),
                        );
                        return false;
                    }
                }
            }
            s => {
                self.runtime_error(format!("No native function named '{}'", s).as_str());
                return false;
            }
        }
    }

    fn call_value(&mut self, callee: Value, arg_count: u8) -> bool {
        match callee {
            Value::Class(class) => {
                self.value_stack.set_value_at_idx(
                    self.value_stack.size() - arg_count as usize - 1,
                    Value::Instance(Rc::new(RefCell::new(Instance {
                        class: class.clone(),
                        fields: HashMap::new(),
                    }))),
                );
                return true;
            }
            Value::Closure(closure) => {
                return self.call(closure, arg_count);
            }
            Value::NativeFunction(func) => {
                return self.call_native(func, arg_count);
            }
            v => {
                let v = v.to_owned();
                self.runtime_error(format!("Can't call value {:?}", v).as_str());
                return false;
            }
        }
    }

    fn capture_upvalue(&mut self, index: usize) -> Upvalue {
        let mut previous_upvalue: Option<Box<Upvalue>> = None;
        let mut upvalue = self.open_upvalue_head.clone();

        while upvalue.clone().is_some()
            && upvalue.clone().unwrap().location > self.frames[self.frame_count - 1].slot + index
        {
            previous_upvalue = upvalue.clone();
            upvalue = upvalue.unwrap().next;
        }

        // if the upvalue is the one we're looking for
        if upvalue.is_some()
            && upvalue.clone().unwrap().location == self.frames[self.frame_count - 1].slot + index
        {
            return *(upvalue.clone()).unwrap();
        }

        let mut new_upvalue = Upvalue {
            location: self.frames[self.frame_count - 1].slot + index,
            index,
            next: None,
            closed: None,
        };
        new_upvalue.next = upvalue;

        if previous_upvalue.is_none() {
            self.open_upvalue_head = Some(Box::new(new_upvalue.clone()));
        } else {
            previous_upvalue.unwrap().next = Some(Box::new(new_upvalue.clone()));
        }

        return new_upvalue;
    }

    fn close_upvalues(&mut self, closure: &mut Closure) {
        let slot = self.frames[self.frame_count - 1].slot;

        for idx in 0..closure.upvalues.len() {
            match closure.upvalues[idx].closed {
                None => {
                    if closure.upvalues[idx].location > slot {
                        closure.upvalues[idx].closed = Some(Box::new(
                            self.value_stack
                                .get_value_at_idx(closure.upvalues[idx].location)
                                .clone(),
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    #[allow(dead_code)]
    fn debug_open_upvalue_list(&mut self) {
        let mut head = self.open_upvalue_head.clone();

        println!("======== START UPVALUE LIST ========\n");

        while head.is_some() {
            println!("UPVALUE LIST VALUE {:?}\n", head);
            head = head.unwrap().next;
        }

        println!("\n======== END UPVALUE LIST ========");
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
                frame!().closure.function.chunk.code[ip - 1]
            }};
        }

        macro_rules! get_instruction {
            () => {{
                frame!().ip += 1;
                let ip = frame!().ip;
                OpCode::from_u8(frame!().closure.function.chunk.code[ip - 1])
            }};
        }

        macro_rules! read_constant {
            () => {{
                frame!().ip += 1;
                let ip = frame!().ip;
                let constant_index = frame!().closure.function.chunk.code[ip - 1];
                &frame!().closure.function.chunk.constants[constant_index as usize]
            }};
        }

        macro_rules! read_short {
            () => {{
                frame!().ip += 2;
                let ip = frame!().ip;
                let first = (frame!().closure.function.chunk.code[ip - 2] as u16) << 8;
                let second = frame!().closure.function.chunk.code[ip - 1] as u16;

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
                            let line = frame!().closure.function.chunk.lines[ip];

                            println!("[Error on line {}]\nPerforming binary operation because LHS isn't a number. LHS = {:?}", line, a);
                            return InterpretResult::RuntimeError;
                        }
                    },
                    _ => {
                        let ip = frame!().ip;
                        let line = frame!().closure.function.chunk.lines[ip];

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
                    let mut result = self.value_stack.pop().unwrap();
                    let slot = frame!().slot;

                    match result {
                        Value::Closure(ref mut closure) => {
                            self.close_upvalues(closure);
                        }
                        _ => {}
                    }

                    self.frame_count -= 1;

                    if self.frame_count == 0 {
                        self.value_stack.pop();
                        return InterpretResult::Ok;
                    }

                    while self.value_stack.size() > slot {
                        self.value_stack.pop();
                    }
                    self.value_stack.push(result);
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
                            value => {
                                let value = value.to_owned();
                                self.runtime_error(
                                    format!(
                                        "LHS of addition can't be added to a number: {:?}",
                                        value
                                    )
                                    .as_str(),
                                );
                                return InterpretResult::RuntimeError;
                            }
                        },
                        Some(Value::String(s2)) => match a {
                            Some(Value::String(s1)) => {
                                self.value_stack
                                    .push(Value::String(format!("{}{}", s1, s2)));
                            }
                            Some(Value::Number(n)) => {
                                self.value_stack.push(Value::String(format!("{}{}", n, s2)));
                            }
                            value => {
                                let value = value.to_owned();
                                self.runtime_error(
                                    format!(
                                        "LHS of addition can't be added to a string: {:?}",
                                        value
                                    )
                                    .as_str(),
                                );
                                return InterpretResult::RuntimeError;
                            }
                        },
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("RHS of addition is an invalid addend: {:?}", value)
                                    .as_str(),
                            );
                            return InterpretResult::RuntimeError;
                        }
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
                        None => {
                            self.runtime_error("Can't perform negation on 'None' value.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Negate => {
                    let v = self.value_stack.pop();

                    match v {
                        Some(Value::Number(n)) => self.value_stack.push(Value::Number(-n)),
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Can't negate non-numeric value: {:?}", value).as_str(),
                            );
                            return InterpretResult::RuntimeError;
                        }
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
                        _ => self.value_stack.push(Value::Boolean(false)),
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
                            value => {
                                let value = value.to_owned();
                                self.runtime_error(
                                    format!("Can't perform > operation on value {:?}", value)
                                        .as_str(),
                                );
                                return InterpretResult::RuntimeError;
                            }
                        },
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Can't perform > operation on value {:?}", value).as_str(),
                            );
                            return InterpretResult::RuntimeError;
                        }
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
                            value => {
                                let value = value.to_owned();
                                self.runtime_error(
                                    format!("Can't perform < operation on value {:?}", value)
                                        .as_str(),
                                );
                                return InterpretResult::RuntimeError;
                            }
                        },
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Can't perform < operation on value {:?}", value).as_str(),
                            );
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Print => match self.value_stack.pop() {
                    Some(Value::Upvalue(upvalue)) => match upvalue.closed {
                        None => {
                            /*
                             * The issue is that in the C version of the code, the value of
                             * an upvalue is accessed directly by just dereferencing the location
                             * property, which points directly to the place in memory where
                             * the value itself lives.
                             *
                             * In the Rust paradigm here, that's all fucked because the location
                             * is meant to point to an index in the value stack. When a value gets
                             * closed, the value stack by definition no longer has the value in it.
                             *
                             * So, any pointer to an index in the value stack means nothing. How in
                             * the world could I fix this?
                             */
                            VM::<T>::print_value(
                                self.value_stack.get_value_at_idx(upvalue.location),
                            );
                        }
                        Some(closed) => {
                            println!("here?");
                            VM::<T>::print_value(*closed);
                        }
                    },
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
                        Value::Class(c) => {
                            let value = self.value_stack.last_value().unwrap();

                            self.globals.insert(c.name.to_owned(), value);
                            self.value_stack.pop();
                        }
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Can't define global with non-string constant {:?}", value)
                                    .as_str(),
                            );
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
                                    let var_name = s.to_owned();
                                    self.runtime_error(
                                        format!("Global var '{}' does not exist.", var_name)
                                            .as_str(),
                                    );
                                    return InterpretResult::RuntimeError;
                                }
                            }
                        }
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Invalid global accessor: {:?}", value).as_str(),
                            );
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::SetGlobal => {
                    let name = read_constant!();

                    match name {
                        Value::String(s) => {
                            if !self.globals.contains_key(s) {
                                let s = s.to_owned();
                                self.runtime_error(
                                    format!("Global var '{}' does not exist.", s).as_str(),
                                );
                                return InterpretResult::RuntimeError;
                            }
                            let value = self.value_stack.last_value().unwrap();
                            self.globals.insert(s.to_owned(), value);
                        }
                        value => {
                            let value = value.to_owned();
                            self.runtime_error(
                                format!("Invalid global accessor: {:?}", value).as_str(),
                            );
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
                        // Proper error reporting already happens inside of call_value
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::Closure => {
                    let value = read_constant!();

                    match value {
                        Value::Function(func) => {
                            let mut closure = Closure::new(func.to_owned());

                            for idx in 0..closure.upvalues.len() {
                                let is_local = read_byte!();
                                let index = read_byte!() as usize;

                                // If is_local == 1, then the index value points to a local in the enclosing scope
                                // else, it points to an upvalue in the enclosing scope
                                if is_local == 1 {
                                    closure.upvalues[idx] = self.capture_upvalue(index);
                                } else {
                                    if index >= frame!().closure.upvalues.len() {
                                        self.runtime_error("error creating higher upvalue");
                                    }
                                    closure.upvalues[idx] =
                                        frame!().closure.upvalues[index].clone();
                                }
                            }

                            self.value_stack.push(Value::Closure(closure));
                        }
                        v => {
                            let v = v.to_owned();
                            self.runtime_error(
                                format!("Can't create closure from {:?}", v).as_str(),
                            );

                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::GetUpvalue => {
                    let slot = read_byte!();

                    let upvalue = frame!().closure.upvalues[slot as usize].clone();

                    match upvalue.closed {
                        Some(v) => {
                            self.value_stack.push(*v);
                        }
                        None => {
                            self.value_stack.push(Value::Upvalue(upvalue));
                        }
                    }
                }
                OpCode::SetUpvalue => {
                    let slot = read_byte!();
                    let value_on_top_of_stack = self.value_stack.peek(0).clone();
                    let closed_value = &frame!().closure.upvalues[slot as usize].closed;

                    // If the upvalue that we're setting has been closed, we should set the closed value
                    // Else, we should set the value in the value stack that it points at
                    match closed_value {
                        Some(_) => {
                            frame!().closure.upvalues[slot as usize].closed =
                                Some(Box::new(value_on_top_of_stack));
                        }
                        None => {
                            let location = frame!().closure.upvalues[slot as usize].location;
                            self.value_stack
                                .set_value_at_idx(location, value_on_top_of_stack);
                        }
                    }
                }
                OpCode::CloseUpvalue => {
                    todo!("what do i do here");
                    // self.close_upvalues(self.value_stack.size() - 1);
                    // self.value_stack.pop();
                }
                OpCode::Class => {
                    let value = read_constant!();
                    self.value_stack.push(value.clone());
                }
                OpCode::GetProperty => {
                    let instance = self.value_stack.peek(0);
                    let property_name = read_constant!().clone();

                    match instance {
                        Value::Instance(instance) => match property_name {
                            Value::String(property_name) => {
                                let owned_instance = Rc::clone(&instance);
                                let borrowed_instance = owned_instance.borrow();
                                let value_of_property =
                                    borrowed_instance.fields.get(&property_name);

                                match value_of_property {
                                    Some(value) => {
                                        self.value_stack.pop();
                                        self.value_stack.push(value.clone());
                                    }
                                    None => {
                                        self.runtime_error(
                                            format!("Undefined property '{}'.", property_name)
                                                .as_str(),
                                        );
                                    }
                                }
                            }
                            _ => {
                                self.runtime_error(
                                        format!("Value {:?} is not a valid property accessor (must be a string).", property_name).as_str(),
                                    );
                            }
                        },
                        _ => {
                            self.runtime_error(
                                format!("Value {:?} is not an instance.", instance).as_str(),
                            );
                        }
                    }
                }
                OpCode::SetProperty => {
                    let instance = self.value_stack.peek(1);
                    let value_to_set_as = self.value_stack.peek(0);
                    let property_name = read_constant!().clone();

                    match instance {
                        Value::Instance(instance) => {
                            let mut new_instance = instance.borrow_mut();
                            match property_name {
                                Value::String(property_name) => {
                                    new_instance
                                        .fields
                                        .insert(property_name.clone(), value_to_set_as);
                                }
                                _ => {
                                    self.runtime_error(
                                        format!("Value {:?} is not a valid property accessor (must be a string).", property_name).as_str(),
                                    );
                                }
                            }
                        }
                        _ => {
                            self.runtime_error(
                                format!("Value {:?} is not an instance.", instance).as_str(),
                            );
                        }
                    }

                    let value = self.value_stack.pop();
                    self.value_stack.pop();
                    self.value_stack.push(value.unwrap());
                }
            }
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let scanner = Scanner::new(source);
        let mut compiler = Compiler::new(scanner, FunctionType::Script, None);

        let compile_result = compiler.compile(None);
        match compile_result {
            None => return InterpretResult::CompileError,
            Some(func) => {
                let closure = Closure::new(func.to_owned());

                self.value_stack.push(Value::Closure(closure.clone()));
                self.call(closure.to_owned(), 0);
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
