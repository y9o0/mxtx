pub mod ast;
pub mod definitions;
pub mod lexer;
pub mod parser;
pub mod pratt;
pub mod scanner;
pub mod values;

mod tests {
    #[cfg(test)]
    mod parser {}
    #[cfg(test)]
    mod lexer {}
}
