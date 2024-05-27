use crate::{chunk::Chunk, compiler::Compiler, value::Value};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    value_stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: None,
            ip: 0,
            value_stack: vec![],
        }
    }

    pub fn interpret(self, source: String) -> InterpretResult {
        let chunk = Chunk::new();
        let mut compiler = Compiler::new(source, chunk);

        if !compiler.compile(None) {
            return InterpretResult::CompileError;
        }

        return InterpretResult::Ok;
    }
}
