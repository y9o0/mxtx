use std::collections::HashMap;

use crate::scanner::tokens::TokenType;

pub mod codegen;
pub mod error;
pub mod exprs;

use {
    error::{GenErr, GenResult},
    exprs::Expr,
};

impl Expr {
    pub fn unpack_num(&self) -> GenResult<f64> {
        match self {
            Expr::Number(x) => Ok(*x),
            _ => {
                return Err(GenErr::Unexpected {
                    at: "codegen".to_string(),
                    message: format!("'{:?}'", self),
                });
            }
        }
    }

    pub fn unpack_ident(self) -> GenResult<String> {
        match self {
            Expr::Ident(x) => Ok(x),
            _ => {
                return Err(GenErr::Unexpected {
                    at: "codegen".to_string(),
                    message: format!("'{:?}'", self),
                });
            }
        }
    }

    pub fn rep(&self, values: &HashMap<String, f64>) -> GenResult<Expr> {
        match self {
            Expr::Number(x) => Ok(Expr::Number(*x)),

            Expr::Bool(x) => Ok(Expr::Bool(*x)),

            Expr::Ident(x) => {
                let value = match values.get(x) {
                    Some(x) => x,
                    None => {
                        return Err(GenErr::Unexpected {
                            at: "rep".to_string(),
                            message: format!("expected '{}' to be declared but found none!", x),
                        });
                    }
                };

                Ok(Expr::Number(*value))
            }

            Expr::Absolute(x) => Ok(Expr::Absolute(Box::new(x.rep(values)?))),

            Expr::Group(x) => Ok(Expr::Group(Box::new(x.rep(values)?))),

            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let left = left.rep(values)?;
                let right = right.rep(values)?;

                Ok(Expr::Binary {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: *operator,
                })
            }
            Expr::Radical { operator, right } => Ok(Expr::Radical {
                operator: *operator,
                right: Box::new(right.rep(values)?),
            }),

            Expr::Trig { function, body } => Ok(Expr::Trig {
                function: *function,
                body: Box::new(body.rep(values)?),
            }),

            Expr::Logarithm {
                function,
                base,
                body,
            } => {
                let mut b: Option<Box<Expr>> = None;

                if let Some(base) = base {
                    b = Some(Box::new(base.rep(values)?));
                }

                Ok(Expr::Logarithm {
                    function: *function,
                    base: b,
                    body: Box::new(body.rep(values)?),
                })
            }

            Expr::Suffix { left, operator } => Ok(Expr::Suffix {
                left: Box::new(left.rep(values)?),
                operator: *operator,
            }),

            _ => {
                return Err(GenErr::Unexpected {
                    at: "rep".to_string(),
                    message: format!("Unexpected expr '{:?}'", self),
                });
            }
        }
    }
}
