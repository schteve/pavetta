use std::fmt::Display;

#[derive(Clone)]
pub struct SourceLocation {
    pub index: (usize, usize),
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    // Symbols
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Semicolon,

    // Keywords
    Int,
    Void,
    Return,

    // Literals
    Identifier(String),
    Constant(i32),
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: SourceLocation,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::BraceOpen => "BRACE_OPEN",
            Self::BraceClose => "BRACE_CLOSE",
            Self::ParenOpen => "PAREN_OPEN",
            Self::ParenClose => "PAREN_CLOSE",
            Self::Semicolon => "SEMICOLON",
            Self::Int => "INT",
            Self::Void => "VOID",
            Self::Return => "RETURN",
            Self::Identifier(s) => &format!("IDENT({})", s),
            Self::Constant(i) => &format!("CONSTANT({})", i),
        };

        write!(f, "{}", s)
    }
}
