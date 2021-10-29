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
        self.skip_blanks();
        if self.is_eof() {
            return Token::eof();
        }

        let kind = match self.advance() {
            b'_' => TKind::Ident,
            b';' => TKind::Semi,
            b'(' => TKind::Lparen,
            b')' => TKind::Rparen,
            b'^' => TKind::Caret,
            b'+' => TKind::Plus,
            b'-' => TKind::Minus,
            b'*' => TKind::Star,
            b'/' => TKind::Slash,
            b'%' => TKind::Percent,
            b'<' => {
                if self.matches(b'=') {
                    TKind::LtEq
                } else {
                    TKind::Lt
                }
            }
            b'>' => {
                if self.matches(b'=') {
                    TKind::GtEq
                } else {
                    TKind::Gt
                }
            }
            b'=' if self.matches(b'=') => TKind::EqEq,
            b'!' if self.matches(b'=') => TKind::NotEq,
            c if is_digit(c) => self.scan_num(),
            c if is_alpha(c) => self.scan_bool(),
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

    fn scan_bool(&mut self) -> TKind {
        self.curr = self.head + 4;
        let t_str = "true".as_bytes();
        if self.curr <= self.src.len() && self.bytes() == t_str {
            return TKind::True;
        }

        self.curr = self.head + 5;
        let f_str = "false".as_bytes();
        if self.curr <= self.src.len() && self.bytes() == f_str {
            return TKind::False;
        }

        // Restore current char pointer and signal error.
        self.curr = self.head + 1;
        TKind::Err
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

    fn matches(&mut self, c: u8) -> bool {
        if self.peek(0) == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self) {
        self.curr += 1;
    }

    fn shift(&mut self) {
        self.head = self.curr;
    }

    fn skip_blanks(&mut self) {
        while !self.is_eof() && is_spacing(self.curr()) {
            self.consume();
        }
        self.shift();
    }
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_spacing(c: u8) -> bool {
    (c as char).is_whitespace()
}
