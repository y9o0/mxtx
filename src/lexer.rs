use anyhow::Result as Ares;
use logos::Logos;

#[derive(Debug, PartialEq, Logos, Default, Clone, Copy)]
#[logos(skip r"[\t \n \f]+")]
pub enum Token<'t> {
    #[regex("[a-zA-Z]+", |ident| ident.slice())]
    Ident(&'t str),

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    #[token("=")]
    Equal,
    #[token(",")]
    Comma,

    Eof,

    #[default]
    Nai,
}

pub fn tokenize(source: &str) -> Ares<Vec<Token<'_>>> {
    let mut lexer = Token::lexer(source);

    let mut tokens = Vec::new();
    while let Some(lexed) = lexer.next() {
        let token = lexed.unwrap();
        tokens.push(token);
    }

    tokens.push(Token::Eof);

    Ok(tokens)
}
