use crate::lexer::Token;

#[derive(Debug)]
pub struct Function<'t> {
    pub name: String,
    pub param: Vec<String>,
    pub equation: Vec<Token<'t>>,
}
