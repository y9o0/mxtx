use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),

    Bool(bool),

    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: TokenType,
    },

    Radical {
        operator: TokenType,
        right: Box<Expr>,
    },

    Trig {
        function: TokenType,
        body: Box<Expr>,
    },

    Logarithm {
        function: TokenType,
        base: Option<Box<Expr>>,
        body: Box<Expr>,
    },

    Group(Box<Expr>),

    Absolute(Box<Expr>),
}
