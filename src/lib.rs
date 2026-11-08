pub use crate::ast::exprs::Expr;
pub use crate::sema::core::{Answer, Parser};

pub(self) mod ast;
pub(self) mod macros;
pub(self) mod pratt;
pub(self) mod scanner;
pub mod sema;

pub fn equation(equation: &str) -> Answer {
    let result = Parser::new(equation.to_string()).equation().unwrap();
    result
}
