#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}
