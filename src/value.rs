use std::fmt;

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
        }
    }
}
