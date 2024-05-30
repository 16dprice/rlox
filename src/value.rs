#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

pub fn print_value(value: &Value) {
    match value {
        Value::Nil => print!("nil"),
        Value::Boolean(v) => print!("{}", v),
        Value::Number(v) => print!("{}", v),
        Value::String(v) => print!("{}", v),
    }
}
