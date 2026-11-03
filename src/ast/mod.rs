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
}
