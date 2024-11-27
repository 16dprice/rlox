use std::fmt;

use crate::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: String,
}

impl Function {
    pub fn new() -> Function {
        Function {
            arity: 0,
            chunk: Chunk::new(),
            name: String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
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
            Value::Function(func) => {
                write!(f, "<fn {}>", func.name)
            }
        }
    }
}
