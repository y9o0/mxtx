use std::{f64::consts::E, f64::consts::PI};

use crate::{
    scanner::tokens::{Literal, TokenType},
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

                return Ok(Expr::Number(*x));
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

        Ok(match sym {
            TokenType::Pi => Expr::Number(PI),
            TokenType::EulersNumber => Expr::Number(E),
            _ => {
                return Err(SemaErr::Syntax {
                    expected: TokenType::Pi,
                    found: sym,
                    line: self.line,
                });
            }
        })
    }
}
