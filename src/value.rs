#[derive(Debug, Clone, Copy)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
}

pub fn print_value(value: &Value) {
    match value {
        Value::Nil => print!("nil"),
        Value::Boolean(v) => print!("{}", v),
        Value::Number(v) => print!("{}", v),
    }
}
