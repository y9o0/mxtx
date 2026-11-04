#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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

    Pi,           // pi or π
    EulersNumber, // e

    Sin, // sin
    Cos, // Cos
    Tan, // tan

    Eof, // end of file

    #[default]
    Nai, // default token
}

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Ident(String),
    None,
}

impl Literal {
    pub fn is_num(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }
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
