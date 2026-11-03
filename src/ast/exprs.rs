use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),

    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: TokenType,
    },

    Radical {
        operator: TokenType,
        right: Box<Expr>,
    },

    Group(Box<Expr>),
}
