use std::f64;

use crate::sema::core::Parser;

pub mod ast;
pub mod pratt;
pub mod scanner;
pub mod sema;

pub enum Result {
    Num(f64),
    Bool(bool),
}

impl Result {
    pub fn as_num(self) -> f64 {
        match self {
            Self::Bool(x) => (x as i32) as f64,
            Self::Num(x) => x,
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Self::Bool(x) => x,
            Self::Num(x) => as_bool(x),
        }
    }
}

fn as_bool(value: f64) -> bool {
    if value == 0.0 {
        return false;
    }

    true
}

pub fn equation(equation: &str) -> Result {
    let result = Parser::new(equation.to_string()).equation().unwrap();

    if let Some(_) = equation.find(|s| matches!(s, '>' | '<' | '=')) {
        return Result::Bool(as_bool(result));
    }

    Result::Num(result)
}
