use crate::ast::TKind;
use crate::ast::Token;

pub struct Lexer<'a> {
    src: &'a [u8],
    head: usize,
    curr: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            head: 0,
            curr: 0,
        }
    }

    pub fn scan(&mut self) -> Token<'a> {
        self.skip_spacing();
        if self.is_eof() {
            return Token::eof();
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
            c if is_ident(c) => TKind::Ident,
            c if is_digit(c) => self.scan_num(),
            _ => TKind::Err,
        };

        let token = Token::new(kind, self.bytes());
        self.shift();
        token
    }

    fn scan_num(&mut self) -> TKind {
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

    fn is_eof(&self) -> bool {
        self.curr >= self.src.len()
    }

    fn curr(&self) -> u8 {
        self.src[self.curr]
    }

    fn bytes(&self) -> &'a [u8] {
        &self.src[self.head..self.curr]
    }

    fn peek(&self, look_ahead: usize) -> u8 {
        if self.curr + look_ahead < self.src.len() {
            self.src[self.curr + look_ahead]
        } else {
            b'\0'
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.src[self.curr];
        self.curr += 1;
        ch
    }

    fn consume(&mut self) {
        self.curr += 1;
    }

    fn shift(&mut self) {
        self.head = self.curr;
    }

    fn skip_spacing(&mut self) {
        while !self.is_eof() && is_spacing(self.curr()) {
            self.consume();
        }
        self.shift();
    }
}

fn is_ident(c: u8) -> bool {
    c == b'_'
}

fn is_digit(c: u8) -> bool {
    (c as char).is_ascii_digit()
}

fn is_spacing(c: u8) -> bool {
    (c as char).is_whitespace()
}
