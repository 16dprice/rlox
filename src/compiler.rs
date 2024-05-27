use crate::chunk::Chunk;
use crate::scanner::{Scanner, Token};

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    // pub fn new() -> Parser {
    //     Parser {
    //         current: None,
    //         previous: None,
    //         had_error: false,
    //         panic_mode: false,
    //     }
    // }
}

pub struct Compiler<'a, 'b> {
    scanner: &'a mut Scanner,
    compiling_chunk: &'b mut Chunk,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(scanner: &'a mut Scanner, chunk: &'b mut Chunk) -> Compiler<'a, 'b> {
        Compiler {
            scanner,
            compiling_chunk: chunk,
        }
    }

    pub fn compile(&mut self, mut chunk: Option<&'b mut Chunk>) {
        if let Some(ref mut c) = chunk {
            println!("{:?}", c);
        }
    }
}
