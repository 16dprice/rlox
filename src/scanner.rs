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

    fn advance(&mut self) -> char {
        self.current += 1;
        return self
            .source
            .chars()
            .nth(self.current - 1)
            .expect(format!("Couldn't get char at index {}", self.current - 1).as_str());
    }

    // This will probably be incredibly slow over time since it converts
    // the source to a list of chars every time. It may be more economical
    // to just instantiate a vector of chars when the `new` func is called.
    fn peek(&self) -> char {
        return self
            .source
            .chars()
            .nth(self.current)
            .expect(format!("Couldn't get char at index {}", self.current).as_str());
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

    pub fn scan_token(&mut self) -> Token {
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        self.skip_whitespace();

        self.start = self.current;

        let token = self.make_token(TokenType::And);
        self.advance();

        return token;
    }
}
