use crate::chunk::Chunk;
use crate::scanner::{Scanner, Token, TokenType};

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: Token::default(),
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
        }
    }
}

pub struct Compiler {
    scanner: Scanner,
    compiling_chunk: Chunk,
    parser: Parser,
}

impl Compiler {
    pub fn new(source: String, chunk: Chunk) -> Compiler {
        Compiler {
            scanner: Scanner::new(source),
            compiling_chunk: chunk,
            parser: Parser::new(),
        }
    }

    fn error_at(&self, token: &Token, message: &str) {
        println!("{:?} - {}", token, message);
    }

    fn error(&self, message: &str) {
        self.error_at(&self.parser.previous, message);
    }

    fn error_at_current(&self, message: &str) {
        self.error_at(&self.parser.current, message);
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = self.scanner.scan_token();

            match self.parser.current.token_type {
                TokenType::Error => self.error_at_current("error"),
                _ => break,
            }
        }
    }

    fn expression(&self) {
        println!("expression called");
    }

    pub fn compile(&mut self, chunk: Option<Chunk>) -> bool {
        if let Some(c) = chunk {
            self.compiling_chunk = c;
        }

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();
        self.expression();

        return !self.parser.had_error;
    }
}
