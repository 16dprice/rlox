use crate::chunk::{Chunk, OpCode};
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
    pub compiling_chunk: Chunk,
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

    fn error_at(&mut self, token: Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;

        print!("[line {}] Error", token.line);

        if token.token_type as u8 == TokenType::Eof as u8 {
            print!(" at end");
        } else if token.token_type as u8 == TokenType::Error as u8 {
        } else {
            let source_string = &self.scanner.source[token.start..(token.start + token.length)];
            print!(" at {}", source_string);
        }

        println!(": {}", message);

        self.parser.had_error = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.parser.previous, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current, message);
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

    fn emit_byte(&mut self, byte: u8) {
        self.compiling_chunk
            .write_code(byte, self.parser.previous.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type as u8 == token_type as u8 {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn expression(&mut self) {
        self.advance();
        self.emit_byte(OpCode::Constant as u8);

        let constant_index = self.compiling_chunk.write_constant(5.0);
        self.emit_byte(constant_index as u8);

        self.advance();
        self.emit_byte(OpCode::Constant as u8);

        let constant_index = self.compiling_chunk.write_constant(2.0);
        self.emit_byte(constant_index as u8);

        self.advance();
        self.emit_byte(OpCode::Add as u8);
    }

    pub fn compile(&mut self, chunk: Option<Chunk>) -> bool {
        if let Some(c) = chunk {
            self.compiling_chunk = c;
        }

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();
        self.expression();

        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();

        return !self.parser.had_error;
    }
}
