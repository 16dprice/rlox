#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Misc.
    Error,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub struct Scanner {
    pub source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current + 1 == self.source.len()
    }

    fn get_char_at_index(&self, index: usize) -> char {
        return self
            .source
            .chars()
            .nth(index)
            .expect(format!("Couldn't get char at index {}", index).as_str());
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.get_char_at_index(self.current - 1);
    }

    // This will probably be incredibly slow over time since it converts
    // the source to a list of chars every time. It may be more economical
    // to just instantiate a vector of chars when the `new` func is called.
    fn peek(&self) -> char {
        return self.get_char_at_index(self.current);
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        return Some(self.get_char_at_index(self.current + 1));
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            let c = self.peek();

            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    // TODO: Handle comments!
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' {
            match self.peek_next() {
                None => {}
                Some(c) => {
                    if is_digit(c) {
                        self.advance();
                        while is_digit(self.peek()) {
                            self.advance();
                        }
                    }
                }
            }
        }

        return self.make_token(TokenType::Number);
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if is_digit(c) {
            return self.number();
        } else {
            return self.make_token(TokenType::And);
        }
    }
}
