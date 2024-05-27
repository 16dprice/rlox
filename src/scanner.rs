#[derive(Debug)]
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

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
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
            line: 1,
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
        self.current == self.source.len()
    }

    // This will probably be incredibly slow over time since it converts
    // the source to a list of chars every time. It may be more economical
    // to just instantiate a vector of chars when the `new` func is called.
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.get_char_at_index(self.current);
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        return Some(self.get_char_at_index(self.current + 1));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        return true;
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
                '/' => match self.peek_next() {
                    Some('/') => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    _ => break,
                },
                _ => {
                    break;
                }
            }
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start != start + length {
            return TokenType::Identifier;
        }
        if &self.source[(self.start + start)..(self.start + start + length)] != rest {
            return TokenType::Identifier;
        }

        return token_type;
    }

    fn identifier_type(&self) -> TokenType {
        let c = self.source.chars().nth(self.start).expect(
            format!(
                "Expected to be able to get char at index {} in source",
                self.start
            )
            .as_str(),
        );

        return match c {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    let c2 = self.source.chars().nth(self.start + 1).expect(
                        format!(
                            "Expected to be able to get char at index {} in source",
                            self.start + 1
                        )
                        .as_str(),
                    );

                    return match c2 {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    };
                } else {
                    return TokenType::Identifier;
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    let c2 = self.source.chars().nth(self.start + 1).expect(
                        format!(
                            "Expected to be able to get char at index {} in source",
                            self.start + 1
                        )
                        .as_str(),
                    );

                    return match c2 {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    };
                } else {
                    return TokenType::Identifier;
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        };
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
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

    fn string(&mut self) -> Token {
        loop {
            if self.is_at_end() {
                return self.make_token(TokenType::Error);
            }

            let c = self.peek();

            if c == '\n' {
                self.line += 1;
            }

            if c != '"' {
                self.advance();
            } else {
                break;
            }
        }

        self.advance();
        return self.make_token(TokenType::String);
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }
        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),

            '!' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }

            '"' => return self.string(),

            _ => return self.make_token(TokenType::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let source = String::from("1 + 2");

        let mut scanner = Scanner::new(source);

        let one = scanner.scan_token();
        let plus = scanner.scan_token();
        let two = scanner.scan_token();

        match one.token_type {
            TokenType::Number => {}
            _ => panic!("Expected Number token, got {:?}", one.token_type),
        }

        match plus.token_type {
            TokenType::Plus => {}
            _ => panic!("Expected Plus token, got {:?}", plus.token_type),
        }

        match two.token_type {
            TokenType::Number => {}
            _ => panic!("Expected Number token, got {:?}", two.token_type),
        }
    }

    #[test]
    fn failing_test() {
        assert!(true == false);
    }
}
