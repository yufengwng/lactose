use crate::token::TKind;
use crate::token::TKind::*;
use crate::token::Token;

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
            b'_' => TkIdent,
            b';' => TkSemi,
            b'(' => TkLparen,
            b')' => TkRparen,
            b'^' => TkCaret,
            b'+' => TkPlus,
            b'-' => TkMinus,
            b'*' => TkStar,
            b'/' => TkSlash,
            b'%' => TkPercent,
            b'<' => {
                if self.matches(b'=') {
                    TkLtEq
                } else {
                    TkLt
                }
            }
            b'>' => {
                if self.matches(b'=') {
                    TkGtEq
                } else {
                    TkGt
                }
            }
            b'=' if self.matches(b'=') => TkEqEq,
            b'!' if self.matches(b'=') => TkNotEq,
            b'0' if self.matches(b'b') => self.scan_bin(),
            b'0' if self.matches(b'x') => self.scan_hex(),
            c if is_digit(c) => self.scan_num(),
            c if is_alpha(c) => self.scan_bool(),
            _ => TkErr,
        };

        let token = Token::new(kind, self.bytes());
        self.shift();
        token
    }

    fn scan_bin(&mut self) -> TKind {
        let mut valid = true;
        let mut has_bin = false;
        while !self.is_eof() {
            let c = self.curr();
            let is_sep = is_digit_sep(c);
            if !is_sep && !is_digit(c) {
                break;
            }
            let is_bin = is_bin_digit(c);
            has_bin = has_bin || is_bin;
            valid = valid && (is_bin || is_sep);
            self.consume();
        }
        return if valid && has_bin { TkBin } else { TkErr };
    }

    fn scan_hex(&mut self) -> TKind {
        let mut valid = true;
        let mut has_hex = false;
        while !self.is_eof() {
            let c = self.curr();
            let is_sep = is_digit_sep(c);
            if !is_sep && !is_alphanum(c) {
                break;
            }
            let is_hex = is_hex_digit(c);
            has_hex = has_hex || is_hex;
            valid = valid && (is_hex || is_sep);
            self.consume();
        }
        return if valid && has_hex { TkHex } else { TkErr };
    }

    fn scan_num(&mut self) -> TKind {
        let mut valid = true;
        while !self.is_eof() {
            let c = self.curr();
            let is_sep = is_digit_sep(c);
            if !is_sep && !is_alphanum(c) {
                break;
            }
            valid = valid && (is_digit(c) || is_sep);
            self.consume();
        }
        if !valid {
            return TkErr;
        }

        if is_digit(self.peek(1)) && self.curr() == b'.' {
            self.consume();
            self.consume();
            while !self.is_eof() && is_digit(self.curr()) {
                self.consume();
            }
            return TkReal;
        } else {
            return TkInt;
        }
    }

    fn scan_bool(&mut self) -> TKind {
        self.curr = self.head + 4;
        let t_str = "true".as_bytes();
        if self.curr <= self.src.len() && self.bytes() == t_str {
            return TkTrue;
        }

        self.curr = self.head + 5;
        let f_str = "false".as_bytes();
        if self.curr <= self.src.len() && self.bytes() == f_str {
            return TkFalse;
        }

        // Restore current char pointer and signal error.
        self.curr = self.head + 1;
        TkErr
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

fn is_digit_sep(c: u8) -> bool {
    c == b'_'
}

fn is_bin_digit(c: u8) -> bool {
    c == b'0' || c == b'1'
}

fn is_hex_digit(c: u8) -> bool {
    c.is_ascii_hexdigit()
}

fn is_alphanum(c: u8) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_spacing(c: u8) -> bool {
    (c as char).is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::TKind::*;

    #[test]
    fn literal_bin() {
        let src = "0b1101";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkBin);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_bin_underscore() {
        let src = "0b_1101_1111_00_10_";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkBin);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_bin_missing_digits() {
        let src = "0b__";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkErr);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_bin_invalid_digits() {
        let src = "0b0123";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkErr);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_hex() {
        let src = "0xcafe1101";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkHex);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_hex_underscore() {
        let src = "0x_cafe_1101_0b_";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkHex);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_hex_missing_digits() {
        let src = "0x__";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkErr);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_hex_invalid_digits() {
        let src = "0xabcdefghi";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkErr);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_int() {
        let src = "123";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkInt);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_int_start_zero() {
        let src = "02012";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkInt);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_int_underscore() {
        let src = "0_1_2_3_";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkInt);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }

    #[test]
    fn literal_int_invalid_digits() {
        let src = "7890abc";
        let mut lexer = Lexer::new(src);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkErr);
        let tok = lexer.scan();
        assert_eq!(tok.kind, TkEof);
    }
}
