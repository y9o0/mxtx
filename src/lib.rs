use crate::sema::core::{Answer, Parser};

pub mod ast;
pub mod pratt;
pub mod scanner;
pub mod sema;

pub fn equation(equation: &str) -> Answer {
    let result = Parser::new(equation.to_string()).equation().unwrap();
    result
}
