use anyhow::Result as Ares;
use logos::Logos;

use crate::ast::BinOp::{self};

#[derive(Debug, PartialEq, Logos, Default, Clone, Copy)]
#[logos(skip r"[\t \n \f]+")]
pub enum Token<'t> {
    #[regex("[a-zA-Z]+", |ident| ident.slice())]
    Ident(&'t str),
    #[regex(
        r"[0-9]+(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?",
        |num| num.slice().parse::<f64>().unwrap()
    )]
    Num(f64),

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    #[token("=")]
    Equal,
    #[token(",")]
    Comma,
    #[token("+")]
    Plus,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("^")]
    Pow,

    #[token("sqrt")]
    #[token("√")]
    Sqrt,

    #[token("pi")]
    #[token("π")]
    Pi,

    Eof,

    #[default]
    Nai,
}

impl Token<'_> {
    pub fn to_pos(&self) -> usize {
        match self {
            Token::Ident(_) => 0,
            Token::Num(_) => 1,
            Token::LeftParen => 2,
            Token::RightParen => 3,
            Token::Equal => 4,
            Token::Comma => 5,
            Token::Plus => 6,
            Token::Sub => 7,
            Token::Mul => 8,
            Token::Div => 9,
            Token::Pow => 10,
            Token::Sqrt => 11,
            Token::Pi => 12,
            Token::Eof => 0x0,
            Token::Nai => 0x1,
        }
    }
}

impl Token<'_> {
    pub fn to_string(&mut self) -> String {
        match self {
            Token::Ident(x) => x.to_string(),
            Token::Num(x) => x.to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Equal => "=".to_string(),
            Token::Comma => ",".to_string(),
            Token::Plus => "+".to_string(),
            Token::Eof => "Eof".to_string(),
            Token::Nai => "Nai".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Pow => "^".to_string(),
            Token::Sqrt => "sqrt".to_string(),
            Token::Pi => "pi".to_string(),
        }
    }
    pub fn to_binop(self) -> BinOp {
        match self {
            Token::Div => BinOp::Div,
            Token::Plus => BinOp::Add,
            Token::Mul => BinOp::Mul,
            Token::Sub => BinOp::Sub,
            Token::Pow => BinOp::Pow,
            Token::Sqrt => BinOp::Sqrt,
            _ => BinOp::Nai,
        }
    }
}

pub fn tokenize(source: &str) -> Ares<Vec<Token<'_>>> {
    let mut lexer = Token::lexer(source);

    let mut tokens = Vec::new();
    while let Some(lexed) = lexer.next() {
        match lexed {
            Ok(x) => tokens.push(x),
            Err(_) => anyhow::bail!(
                "unexpected input {:?} at byte {}..{}",
                lexer.slice(),
                lexer.span().start,
                lexer.span().end
            ),
        }
    }

    tokens.push(Token::Eof);

    Ok(tokens)
}
