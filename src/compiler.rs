use std::collections::HashMap;
use std::u8;

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

type ParseFnPtr = fn(&mut Compiler, bool) -> ();

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<ParseFnPtr>,
    infix: Option<ParseFnPtr>,
    precedence: Precedence,
}

#[derive(Clone, Copy)]
struct Local {
    name: Token,
    depth: Option<u16>,
}

pub struct Compiler {
    scanner: Scanner,
    pub compiling_chunk: Chunk,
    parser: Parser,
    precedence_map: HashMap<TokenType, ParseRule>,

    // Used for local variable storage
    local_count: u8,
    scope_depth: u16,
    locals: [Local; u8::MAX as usize + 1],
}

impl Compiler {
    pub fn new(source: String, chunk: Chunk) -> Compiler {
        let mut compiler = Compiler {
            scanner: Scanner::new(source),
            compiling_chunk: chunk,
            parser: Parser::new(),
            precedence_map: HashMap::new(),

            local_count: 0,
            scope_depth: 0,
            locals: [Local {
                name: Token::default(),
                depth: Some(0),
            }; u8::MAX as usize + 1],
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
                prefix: Some(Compiler::variable),
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

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while self.local_count > 0
            && self.locals[self.local_count as usize - 1].depth.unwrap() > self.scope_depth
        {
            self.emit_byte(OpCode::Pop as u8);
            self.local_count -= 1;
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type as u8 == token_type as u8 {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        return self.parser.current.token_type as u8 == token_type as u8;
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        return true;
    }

    fn literal(&mut self, _can_assign: bool) {
        let token = self.parser.previous.token_type as u8;

        if token == TokenType::True as u8 {
            self.emit_byte(OpCode::True as u8);
        } else if token == TokenType::False as u8 {
            self.emit_byte(OpCode::False as u8);
        } else if token == TokenType::Nil as u8 {
            self.emit_byte(OpCode::Nil as u8);
        }

        return;
    }

    fn string(&mut self, _can_assign: bool) {
        self.emit_byte(OpCode::Constant as u8);

        let start = self.parser.previous.start + 1;
        let end = start + self.parser.previous.length - 2;
        let lexeme = &self.scanner.source[start..end];

        let constant_index = self.compiling_chunk.write_string(String::from(lexeme));
        self.emit_byte(constant_index as u8);
    }

    fn identifiers_equal(&mut self, a: Token, b: Token) -> bool {
        if a.length != b.length {
            return false;
        }

        let a_lexeme = &self.scanner.source[a.start..(a.start + a.length)];
        let b_lexeme = &self.scanner.source[b.start..(b.start + b.length)];

        return a_lexeme.eq(b_lexeme);
    }

    fn resolve_local(&mut self, name: Token) -> Option<usize> {
        // iterates from (self.local_count - 1) to 0
        for idx in (0..self.local_count as usize).rev() {
            let local = self.locals[idx];

            if self.identifiers_equal(name, local.name) {
                match local.depth {
                    None => {
                        self.error("Can't read local variable in its own initializer");
                    }
                    _ => {}
                }
                return Some(idx);
            }
        }
        return None;
    }

    fn named_variable(&mut self, name: Token, can_assign: bool) {
        let get_operation: OpCode;
        let set_operation: OpCode;

        let local_index = self.resolve_local(name);
        let index: usize;

        // if the index exists, then the variable is a local
        // otherwise, it's a global
        match local_index {
            Some(i) => {
                index = i;

                get_operation = OpCode::GetLocal;
                set_operation = OpCode::SetLocal;
            }
            None => {
                let lexeme = &self.scanner.source[name.start..(name.start + name.length)];
                index = self.compiling_chunk.write_string(lexeme.to_owned());

                get_operation = OpCode::GetGlobal;
                set_operation = OpCode::SetGlobal;
            }
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_bytes(set_operation as u8, index as u8);
        } else {
            self.emit_bytes(get_operation as u8, index as u8);
        }
    }

    fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous, can_assign)
    }

    fn number(&mut self, _can_assign: bool) {
        self.emit_byte(OpCode::Constant as u8);

        let lexeme = &self.scanner.source[self.parser.previous.start
            ..(self.parser.previous.start + self.parser.previous.length)];

        match lexeme.parse::<f64>() {
            Ok(value) => {
                let constant_index = self.compiling_chunk.write_number(value);
                self.emit_byte(constant_index as u8);
            }
            Err(e) => self
                .error(format!("couldn't parse {} into number, got error: {}", lexeme, e).as_str()),
        }
    }

    fn unary(&mut self, _can_assign: bool) {
        let op_type = self.parser.previous.token_type as u8;

        self.parse_precedence(Precedence::Unary);

        if op_type == TokenType::Bang as u8 {
            self.emit_byte(OpCode::Not as u8);
        } else if op_type == TokenType::Minus as u8 {
            self.emit_byte(OpCode::Negate as u8);
        }

        return;
    }

    fn binary(&mut self, _can_assign: bool) {
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

    fn grouping(&mut self, _can_assign: bool) {
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

        let can_assign = precedence as u8 <= Precedence::Assignment as u8;
        prefix_func(self, can_assign);

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
                Some(infix_func) => infix_func(self, can_assign),
                _ => return,
            }

            if can_assign && self.match_token(TokenType::Equal) {
                self.error("Invalid assignment target.");
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        self.emit_byte(OpCode::Pop as u8);
    }

    fn add_local(&mut self, name: Token) {
        if self.local_count as usize == u8::MAX as usize + 1 {
            self.error("Too many local variables in block");
            return;
        }

        let mut current_local = self.locals[self.local_count as usize];

        // current_local.name = name;
        // current_local.depth = None;

        self.locals[self.local_count as usize].name = name;
        self.locals[self.local_count as usize].depth = None;

        self.local_count += 1;
    }

    fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }

        let name = self.parser.previous;

        // iterates from (self.local_count - 1) to 0
        for idx in (0..self.local_count as usize).rev() {
            let local = self.locals[idx];

            if local.depth == None && local.depth.unwrap() < self.scope_depth {
                continue;
            }

            if self.identifiers_equal(name, local.name) {
                self.error("Already a variable with this name in this scope.");
            }
        }

        self.add_local(name);
    }

    fn parse_variable(&mut self, message: &str) -> u8 {
        self.consume(TokenType::Identifier, message);

        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }

        let lexeme = &self.scanner.source[self.parser.previous.start
            ..(self.parser.previous.start + self.parser.previous.length)];

        let index = self.compiling_chunk.write_string(lexeme.to_owned());
        return index as u8;
    }

    fn mark_initialized(&mut self) {
        self.locals[self.local_count as usize - 1].depth = Some(self.scope_depth);
    }

    fn define_variable(&mut self, global_index: u8) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }

        self.emit_bytes(OpCode::DefineGlobal as u8, global_index);
    }

    fn var_declaration(&mut self) {
        let global_index = self.parse_variable("Expect variable name.");

        if self.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Nil as u8);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global_index);
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        let synchronized_tokens: [u8; 8] = [
            TokenType::Class as u8,
            TokenType::Fun as u8,
            TokenType::Var as u8,
            TokenType::For as u8,
            TokenType::If as u8,
            TokenType::While as u8,
            TokenType::Print as u8,
            TokenType::Return as u8,
        ];

        while self.parser.current.token_type as u8 != TokenType::Eof as u8 {
            if self.parser.previous.token_type as u8 == TokenType::Semicolon as u8 {
                return;
            }

            let current_token_type = self.parser.current.token_type as u8;
            if synchronized_tokens.contains(&current_token_type) {
                return;
            }

            self.advance();
        }
    }

    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after value.");
            self.emit_byte(OpCode::Print as u8);
        } else if self.match_token(TokenType::If) {
            todo!("if statement not yet implemented");
        } else if self.match_token(TokenType::While) {
            todo!("while statement not yet implemented");
        } else if self.match_token(TokenType::For) {
            todo!("for statement not yet implemented");
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
        }
    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else if self.match_token(TokenType::Fun) {
            todo!("fun token handling hasn't been implemented");
        } else if self.match_token(TokenType::Class) {
            todo!("class token handling hasn't been implemented");
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    pub fn compile(&mut self, chunk: Option<Chunk>) -> bool {
        if let Some(c) = chunk {
            self.compiling_chunk = c;
        }

        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.advance();

        while !self.match_token(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();

        return !self.parser.had_error;
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    use super::*;

    #[test]
    fn basic_arithmetic_opcodes() {
        let source = String::from("1 + 2;");

        let chunk = Chunk::new();
        let mut compiler = Compiler::new(source, chunk);

        let compile_result = compiler.compile(None);

        assert_eq!(compile_result, true);

        let two = compiler.compiling_chunk.constants.pop();
        let one = compiler.compiling_chunk.constants.pop();

        match two {
            Some(Value::Number(n)) => {
                if n != 2.0 {
                    panic!("Expected 2.0, got {}", n)
                }
            }
            _ => panic!("Expected number, got {:?}", two),
        }
        match one {
            Some(Value::Number(n)) => {
                if n != 1.0 {
                    panic!("Expected 1.0, got {}", n)
                }
            }
            _ => panic!("Expected number, got {:?}", two),
        }
    }
}
