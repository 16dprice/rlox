use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: Option<String>,
    pub upvalue_count: u8,
}

impl Function {
    pub fn new() -> Function {
        Function {
            arity: 0,
            chunk: Chunk::new(),
            name: None,
            upvalue_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: u8,
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: Function,
    pub upvalues: Vec<Rc<RefCell<Upvalue>>>,
}

impl Closure {
    pub fn new(func: Function) -> Closure {
        let mut upvalues = Vec::new();
        for _ in 0..func.upvalue_count {
            upvalues.push(Rc::new(RefCell::new(Upvalue {
                location: 0,
                closed: None,
            })));
        }

        Closure {
            function: func,
            upvalues,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Upvalue {
    pub location: usize,
    pub closed: Option<Box<Value>>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Class,
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
    NativeFunction(NativeFunction),
    Closure(Closure),
    Upvalue(Upvalue),
    Class(Class),
    Instance(Rc<RefCell<Instance>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => {
                write!(f, "Nil")
            }
            Value::Boolean(b) => {
                if *b {
                    write!(f, "BOOLEAN: true")
                } else {
                    write!(f, "BOOLEAN: false")
                }
            }
            Value::Number(n) => {
                write!(f, "NUMBER: {}", n)
            }
            Value::String(s) => {
                write!(f, "STRING: {}", s)
            }
            Value::Function(func) => match &func.name {
                Some(name) => {
                    write!(f, "<fn {}>", name)
                }
                None => {
                    write!(f, "<script>")
                }
            },
            Value::NativeFunction(func) => {
                write!(f, "<native fn {}>", func.name)
            }
            Value::Closure(closure) => match &closure.function.name {
                Some(name) => {
                    write!(f, "<closure {}>", name)
                }
                None => {
                    write!(f, "<closure>")
                }
            },
            Value::Upvalue(up) => {
                write!(f, "<upvalue {}>", up.location)
            }
            Value::Class(c) => {
                write!(f, "{}", c.name)
            }
            Value::Instance(i) => {
                write!(f, "{} instance", i.borrow().class.name)
            }
        }
    }
}
