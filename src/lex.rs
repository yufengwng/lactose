#[derive(PartialEq)]
pub enum TKind {
    Err,
    Num,
    Lparen,
    Rparen,
    Caret,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}

pub struct Token<'a> {
    pub kind: TKind,
    lexeme: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(kind: TKind, lexeme: &'a [u8]) -> Self {
        Self { kind, lexeme }
    }

    pub fn lexeme(&self) -> &'a str {
        std::str::from_utf8(self.lexeme).unwrap()
    }
}

pub struct Lexer<'a> {
    src: &'a [u8],
    len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src: src.as_bytes(), len: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.is_eof() {
            return None;
        }

        self.skip_noise();
        if self.is_eof() {
            return None;
        }

        let kind = match self.advance() {
            b'(' => TKind::Lparen,
            b')' => TKind::Rparen,
            b'^' => TKind::Caret,
            b'+' => TKind::Plus,
            b'-' => TKind::Minus,
            b'*' => TKind::Star,
            b'/' => TKind::Slash,
            b'%' => TKind::Percent,
            c if is_digit(c) => {
                while !self.is_eof() && is_digit(self.curr()) {
                    self.consume();
                }
                if is_digit(self.peek(1)) && self.curr() == b'.' {
                    self.consume();
                    self.consume();
                    while !self.is_eof() && is_digit(self.curr()) {
                        self.consume();
                    }
                }
                TKind::Num
            }
            _ => TKind::Err,
        };

        let token = self.make_token(kind);
        self.shift_source();
        Some(token)
    }

    fn is_eof(&self) -> bool {
        self.len >= self.src.len()
    }

    fn curr(&self) -> u8 {
        self.src[self.len]
    }

    fn peek(&self, look_ahead: usize) -> u8 {
        if self.len + look_ahead < self.src.len() {
            self.src[self.len + look_ahead]
        } else {
            b'\0'
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.src[self.len];
        self.len += 1;
        ch
    }

    fn consume(&mut self) {
        self.len += 1;
    }

    fn shift_source(&mut self) {
        self.src = &self.src[self.len..];
        self.len = 0;
    }

    fn make_token(&self, kind: TKind) -> Token<'a> {
        Token::new(kind, &self.src[0..self.len])
    }

    fn skip_noise(&mut self) {
        while !self.is_eof() && is_spacing(self.curr()) {
            self.consume();
        }
        self.shift_source();
    }
}

fn is_digit(c: u8) -> bool {
    (c as char).is_ascii_digit()
}

fn is_spacing(c: u8) -> bool {
    (c as char).is_whitespace()
}
