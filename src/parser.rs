use anyhow;

use crate::{
    ast,
    token::{Token, TokenKind},
};

macro_rules! expect {
    ($self:ident, $kind:pat) => {
        let token = $self.take_token();
        let Some(Token { kind: $kind, .. }) = token else {
            if token.is_none() {
                return Err(anyhow::anyhow!("Expected {}, found EOF", stringify!($kind)));
            } else {
                return Err(anyhow::anyhow!(
                    "Expected {}, found {}",
                    stringify!($kind),
                    token.unwrap().kind
                ));
            }
        };
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    curr_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            curr_idx: 0,
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.curr_idx).cloned()
    }

    fn take_token(&mut self) -> Option<Token> {
        let t = self.peek();
        if t.is_some() {
            self.curr_idx += 1;
        }
        t
    }

    pub fn parse_ast(&mut self) -> anyhow::Result<ast::Program> {
        let program = self.parse_program()?;
        if let Some(next_tok) = self.peek() {
            return Err(anyhow::anyhow!(
                "Parse finished but some tokens were not consumed (next: {}",
                next_tok.kind
            ));
        }
        Ok(program)
    }

    fn parse_program(&mut self) -> anyhow::Result<ast::Program> {
        let function = self.parse_function()?;
        Ok(ast::Program { function })
    }

    fn parse_function(&mut self) -> anyhow::Result<ast::Function> {
        expect!(self, TokenKind::Int);

        let ident = self.parse_identifier()?;

        expect!(self, TokenKind::ParenOpen);
        expect!(self, TokenKind::Void);
        expect!(self, TokenKind::ParenClose);
        expect!(self, TokenKind::BraceOpen);

        let stmt = self.parse_statement()?;

        expect!(self, TokenKind::BraceClose);

        Ok(ast::Function { ident, stmt })
    }

    fn parse_statement(&mut self) -> anyhow::Result<ast::Stmt> {
        expect!(self, TokenKind::Return);

        let expr = self.parse_expr()?;

        expect!(self, TokenKind::Semicolon);

        Ok(ast::Stmt { expr })
    }

    fn parse_expr(&mut self) -> anyhow::Result<ast::Expr> {
        let int = self.parse_int()?;

        Ok(ast::Expr { int })
    }

    fn parse_identifier(&mut self) -> anyhow::Result<String> {
        expect!(self, TokenKind::Identifier(name));
        Ok(name)
    }

    fn parse_int(&mut self) -> anyhow::Result<i32> {
        expect!(self, TokenKind::Constant(i));

        Ok(i)
    }
}
