use crate::{chunk::Chunk, value::Value};

struct VM {
    chunk: Chunk,
    ip: usize,
    value_stack: Vec<Value>,
}
