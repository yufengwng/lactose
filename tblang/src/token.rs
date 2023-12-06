#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TKind {
    TkErr,
    TkEof,
    TkSemi,
    TkNLine,
    TkLparen,
    TkRparen,
    TkPlus,
    TkMinus,
    TkStar,
    TkSlash,
    TkPercent,
    TkCaret,
    TkLt,
    TkGt,
    TkLtEq,
    TkGtEq,
    TkEqEq,
    TkNotEq,
    TkEq,
    TkTrue,
    TkFalse,
    TkInt,
    TkBin,
    TkHex,
    TkReal,
    TkIdent,
}

#[derive(Clone)]
pub struct Token<'a> {
    pub kind: TKind,
    pub line: usize,
    pub col: usize,
    span: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn eof() -> Self {
        Self::new(TKind::TkEof, &[])
    }

    pub fn new(kind: TKind, span: &'a [u8]) -> Self {
        Self { kind, span, line: 1, col: 1 }
    }

    pub fn lexeme(&self) -> &'a str {
        std::str::from_utf8(self.span).unwrap()
    }
}
