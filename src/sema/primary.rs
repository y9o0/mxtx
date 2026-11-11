use std::f64::consts::{E, GOLDEN_RATIO, PI};

use crate::{
    scanner::tokens::{
        Literal,
        TokenType::{self, Underscore},
    },
    sema::error::SemaErr,
};

use super::*;

impl Parser {
    pub fn numbers(&mut self) -> SemaResult<Expr> {
        match self.previous.literal {
            Some(ref v) => {
                let Literal::Number(x) = v else {
                    return Err(error::SemaErr::Expected {
                        line: self.line,
                        message: format!(
                            "Expected a number literal found '{:?}'",
                            self.previous.token_type
                        ),
                    });
                };

                Ok(Expr::Number(*x))
            }
            None => {
                return Err(error::SemaErr::Expected {
                    line: self.line,
                    message: format!("Expcted a literal found '{:?}' ", self.previous.token_type),
                });
            }
        }
    }

    pub fn ident(&mut self) -> SemaResult<Expr> {
        match self.previous.literal {
            Some(ref v) => {
                let Literal::Ident(x) = v else {
                    return Err(error::SemaErr::Expected {
                        line: self.line,
                        message: format!(
                            "Expected a string literal found '{:?}'",
                            self.previous.token_type
                        ),
                    });
                };

                self.checkers.has_ident = true;
                Ok(Expr::Ident(x.to_string()))
            }
            None => {
                return Err(error::SemaErr::Expected {
                    line: self.line,
                    message: format!("Expcted a literal found '{:?}' ", self.previous.token_type),
                });
            }
        }
    }

    pub fn group(&mut self) -> SemaResult<Expr> {
        let inner = self.parse_precedence(crate::pratt::Precedence::None)?;
        self.match_err(crate::scanner::tokens::TokenType::RightParen)?;
        Ok(Expr::Group(Box::new(inner)))
    }

    pub fn symbols(&mut self) -> SemaResult<Expr> {
        let sym = self.previous.token_type;

        let result = match sym {
            TokenType::Pi => Expr::Number(PI),
            TokenType::EulersNumber => Expr::Number(E),
            TokenType::GoldenRatio => Expr::Number(GOLDEN_RATIO),
            _ => {
                return Err(SemaErr::Syntax {
                    expected: TokenType::Pi,
                    found: sym,
                    line: self.line,
                });
            }
        };

        Ok(result)
    }

    pub fn absolute(&mut self) -> SemaResult<Expr> {
        let inner = self.parse_precedence(crate::pratt::Precedence::None)?;
        self.match_err(TokenType::Pipe)?;
        Ok(Expr::Absolute(Box::new(inner)))
    }

    pub fn logarithm(&mut self) -> SemaResult<Expr> {
        let function = self.previous.token_type;
        let is_log = function == TokenType::Log;
        let mut base: Option<Box<Expr>> = None;

        if self.current.token_type == TokenType::Underscore && is_log {
            self.advance();

            base = Some(Box::new(
                self.parse_precedence(crate::pratt::Precedence::None)?,
            ));
        }

        if self.current.token_type == TokenType::Underscore && function == TokenType::Ln {
            return Err(SemaErr::Syntax {
                expected: TokenType::LeftParen,
                found: Underscore,
                line: self.line,
            });
        }

        self.match_err(TokenType::LeftParen)?;
        let expr = self.parse_precedence(crate::pratt::Precedence::Unary)?;
        self.match_err(TokenType::RightParen)?;

        Ok(Expr::Logarithm {
            function,
            base,
            body: Box::new(expr),
        })
    }
}
