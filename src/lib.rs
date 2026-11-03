use crate::sema::core::Parser;

pub mod ast;
pub mod pratt;
pub mod scanner;
pub mod sema;

mod tests {
    #[cfg(test)]
    mod parser {}
    #[cfg(test)]
    mod lexer {}
}

pub fn equation(equation: &str) -> f64 {
    Parser::new(equation.to_string()).equation().unwrap()
}
