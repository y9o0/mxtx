#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,  // (
    RightParen, // )

    Equal, // =
    Comma, // ,
    Plus,  // +
    Minus, // -
    Mul,   // *
    Div,   // /
    Pow,   // ^
    Sqrt,  // sqrt | √

    Num,   // number literals
    Ident, // string literals

    Eof, // end of file

    #[default]
    Nai, // default token
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Ident(String),
    None,
}

pub type TokenContent = (TokenType, String, u32, Option<Literal>);

impl Token {
    pub fn new(content: TokenContent) -> Self {
        Self {
            token_type: content.0,
            lexeme: content.1,
            literal: content.3,
            line: content.2,
        }
    }
}
