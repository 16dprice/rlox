use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => {
                write!(f, "Nil")
            }
            Value::Boolean(b) => {
                if *b {
                    write!(f, "Boolean: true")
                } else {
                    write!(f, "Boolean: false")
                }
            }
            Value::Number(n) => {
                write!(f, "Number: {}", n)
            }
            Value::String(s) => {
                write!(f, "String: {}", s)
            }
        }
    }
}
