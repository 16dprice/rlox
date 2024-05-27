#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Number(f64),
}

pub fn print_value(value: &Value) {
    match value {
        Value::Boolean(v) => print!("{}", v),
        Value::Number(v) => print!("{}", v),
    }
}
