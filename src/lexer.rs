use std::str;

use anyhow;

use crate::token::{SourceLocation, Token, TokenKind};

pub struct Lexer {
    source: Vec<u8>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        assert!(source.is_ascii());
        Self {
            source: source.as_bytes().to_owned(),
        }
    }

    pub fn run(&self) -> anyhow::Result<Vec<Token>> {
        let mut tokens = Vec::new();

        let mut start_idx = 0;
        let mut line = 1;
        let mut line_idx = 0;
        while let Some(curr) = self.source.get(start_idx).copied() {
            let mut end_idx = start_idx + 1; // Used to span token and thus to advance start idx on next iteration

            let kind = match curr as char {
                ' ' | '\t' | '\r' => None,
                '\n' => {
                    line += 1;
                    line_idx = end_idx;
                    None
                }

                '{' => Some(TokenKind::BraceOpen),
                '}' => Some(TokenKind::BraceClose),
                '(' => Some(TokenKind::ParenOpen),
                ')' => Some(TokenKind::ParenClose),
                ';' => Some(TokenKind::Semicolon),

                x if x.is_ascii_alphabetic() || x == '_' => {
                    while let Some(c) = self.source.get(end_idx).copied() {
                        if c.is_ascii_alphanumeric() || c == b'_' {
                            end_idx += 1;
                        } else {
                            break;
                        }
                    }
                    let span = &self.source[start_idx..end_idx];

                    let kind = match span {
                        b"int" => TokenKind::Int,
                        b"void" => TokenKind::Void,
                        b"return" => TokenKind::Return,
                        slice => {
                            let ident = str::from_utf8(slice).expect("Source must be ASCII");
                            TokenKind::Identifier(ident.to_owned())
                        }
                    };
                    Some(kind)
                }

                x if x.is_ascii_digit() => {
                    let mut valid = true;
                    while let Some(c) = self.source.get(end_idx).copied() {
                        if c.is_ascii_digit() {
                            end_idx += 1;
                        } else if c.is_ascii_alphabetic() || c == b'_' {
                            // Don't fail immediately so we can report the entire token
                            valid = false;
                            end_idx += 1;
                        } else {
                            break;
                        }
                    }
                    let span = &self.source[start_idx..end_idx];
                    let s = str::from_utf8(span).expect("Source must be ASCII");

                    if !valid {
                        return Err(anyhow::anyhow!("Invalid constant: {s}"));
                    }

                    // We only pulled digits so parsing is straightforward
                    let constant = s
                        .parse::<i32>()
                        .expect("Guaranteed integer failed to parse");
                    Some(TokenKind::Constant(constant))
                }

                x => return Err(anyhow::anyhow!("Invalid character: {} ({x})", x as char)),
            };

            if let Some(k) = kind {
                let token = Token {
                    kind: k,
                    loc: SourceLocation {
                        index: (start_idx, end_idx),
                        line,
                        col: start_idx - line_idx,
                    },
                };
                tokens.push(token);
            }

            start_idx = end_idx;
        }

        Ok(tokens)
    }
}
