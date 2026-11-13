use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),

    Bool(bool),

    Ident(String),

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

    Where {
        left: Box<Expr>,
        right: Rc<RefCell<HashMap<String, f64>>>,
    },

    Group(Box<Expr>),

    Absolute(Box<Expr>),

    PostFix {
        left: Box<Expr>,
        operator: TokenType,
    },

    Domain {
        cond: Box<Expr>,
        left: Box<Expr>,
    },

    Range {
        left: Box<Expr>,
        cond: Box<Expr>,
    },
}
