use std::collections::HashMap;

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

#[derive(Clone, Copy)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn from_u8(i: u8) -> Precedence {
        match i {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            _ => Precedence::Primary,
        }
    }
}

type ParseFnPtr = fn(&mut Compiler) -> ();

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<ParseFnPtr>,
    infix: Option<ParseFnPtr>,
    precedence: Precedence,
}

pub struct Compiler {
    scanner: Scanner,
    pub compiling_chunk: Chunk,
    parser: Parser,
    precedence_map: HashMap<TokenType, ParseRule>,
}

impl Compiler {
    pub fn new(source: String, chunk: Chunk) -> Compiler {
        let mut compiler = Compiler {
            scanner: Scanner::new(source),
            compiling_chunk: chunk,
            parser: Parser::new(),
            precedence_map: HashMap::new(),
        };

        compiler.precedence_map.insert(
            TokenType::LeftParen,
            ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::RightParen,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::LeftBrace,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::RightBrace,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Comma,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Dot,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Minus,
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Plus,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Semicolon,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Slash,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Star,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Bang,
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::BangEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Equal,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::EqualEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Greater,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );
        compiler.precedence_map.insert(
            TokenType::GreaterEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Less,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );
        compiler.precedence_map.insert(
            TokenType::LessEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Identifier,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::String,
            ParseRule {
                prefix: Some(Compiler::string),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Number,
            ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::And,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::And,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Class,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Else,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::False,
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::For,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Fun,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::If,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Nil,
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Or,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::Or,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Print,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Return,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Super,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::This,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::True,
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Var,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::While,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Error,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
        compiler.precedence_map.insert(
            TokenType::Eof,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );

        return compiler;
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

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
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

    fn literal(&mut self) {
        let token = self.parser.previous.token_type as u8;
        let true_ = TokenType::True as u8;
        let false_ = TokenType::False as u8;
        let nil_ = TokenType::Nil as u8;

        if token == true_ {
            self.emit_byte(OpCode::True as u8);
        } else if token == false_ {
            self.emit_byte(OpCode::False as u8);
        } else if token == nil_ {
            self.emit_byte(OpCode::Nil as u8);
        }

        return;
    }

    fn string(&mut self) {}

    fn number(&mut self) {
        self.emit_byte(OpCode::Constant as u8);

        let lexeme = &self.scanner.source[self.parser.previous.start
            ..(self.parser.previous.start + self.parser.previous.length)];

        match lexeme.parse::<f64>() {
            Ok(value) => {
                let constant_index = self.compiling_chunk.write_constant(value);
                self.emit_byte(constant_index as u8);
            }
            Err(e) => self
                .error(format!("couldn't parse {} into number, got error: {}", lexeme, e).as_str()),
        }
    }

    fn unary(&mut self) {
        let op_type = self.parser.previous.token_type as u8;

        self.parse_precedence(Precedence::Unary);

        if op_type == TokenType::Bang as u8 {
            self.emit_byte(OpCode::Not as u8);
        } else if op_type == TokenType::Minus as u8 {
            self.emit_byte(OpCode::Negate as u8);
        }

        return;
    }

    fn binary(&mut self) {
        let op_type = self.parser.previous.token_type;

        let parse_rule = match self.precedence_map.get(&op_type).cloned() {
            Some(pr) => pr,
            _ => {
                self.error(format!("Expect parse rule for {:?}.", &op_type).as_str());
                return;
            }
        };

        self.parse_precedence(Precedence::from_u8(parse_rule.precedence as u8 + 1));

        match op_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::BangEqual => self.emit_bytes(OpCode::Equal as u8, OpCode::Not as u8),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal as u8),
            TokenType::Greater => self.emit_byte(OpCode::Greater as u8),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::Less as u8, OpCode::Not as u8),
            TokenType::Less => self.emit_byte(OpCode::Less as u8),
            TokenType::LessEqual => self.emit_bytes(OpCode::Greater as u8, OpCode::Not as u8),
            _ => println!("need to implement binary opcode {:?}", op_type),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let parse_rule = match self
            .precedence_map
            .get(&self.parser.previous.token_type)
            .cloned()
        {
            Some(pr) => pr,
            _ => {
                self.error(
                    format!(
                        "Expect parse rule for {:?}.",
                        &self.parser.previous.token_type
                    )
                    .as_str(),
                );
                return;
            }
        };

        let Some(prefix_func) = parse_rule.prefix else {
            self.error("Expect expression.");
            return;
        };

        prefix_func(self);

        loop {
            let parse_rule = match self
                .precedence_map
                .get(&self.parser.current.token_type)
                .cloned()
            {
                Some(pr) => pr,
                _ => {
                    self.error(
                        format!(
                            "Expect parse rule for {:?}.",
                            &self.parser.current.token_type
                        )
                        .as_str(),
                    );
                    return;
                }
            };

            if precedence as u8 > parse_rule.precedence as u8 {
                return;
            }

            self.advance();

            match parse_rule.infix {
                Some(infix_func) => infix_func(self),
                _ => return,
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
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
