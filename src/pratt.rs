use crate::{ast::Expr, lexer::Token, parser::Parser};
use anyhow::{Result as Ares, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    None,
    Term,    // + -
    Factor,  // * /
    Radical, // (sqrt | √) ^
    Unary,   // -
    Group,   // ()
    Primary,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseRule<'s> {
    pub prefix: Option<fn(&mut Parser<'s>) -> Ares<Expr>>,
    pub infix: Option<fn(&mut Parser<'s>, &Expr) -> Ares<Expr>>,
    pub precedence: Precedence,
}

// it start from the lowst to the highst!
impl Precedence {
    pub fn next(self) -> Precedence {
        match self {
            Precedence::None => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Radical,
            Precedence::Radical => Precedence::Group,
            _ => Precedence::Primary,
        }
    }
}

impl Parser<'_> {
    pub fn get_rule(token: Token) -> ParseRule {
        match token {
            Token::Num(_) => ParseRule {
                prefix: Some(Parser::number),
                infix: None,
                precedence: Precedence::None,
            },
            Token::Plus => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            Token::Sub => ParseRule {
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            Token::Mul => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            Token::Div => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            Token::Pow => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Radical,
            },
            Token::LeftParen => ParseRule {
                prefix: Some(Parser::group),
                infix: None,
                precedence: Precedence::Group,
            },
            Token::Sqrt => ParseRule {
                prefix: Some(Parser::radical),
                infix: None,
                precedence: Precedence::Radical,
            },
            Token::Pi => ParseRule {
                prefix: Some(Parser::pi),
                infix: None,
                precedence: Precedence::None,
            },
            _ => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    pub fn parse_precedence(&mut self, min: Precedence) -> Ares<Expr> {
        let mut expr: Expr;

        self.advance()?;

        let rule = Self::get_rule(self.tokens.previous);

        if let Some(prefix) = rule.prefix {
            expr = prefix(self)?;
        } else {
            return Err(anyhow!("Expected an expr found None!"));
        };

        while min <= Self::get_rule(self.tokens.current).precedence {
            let Some(infix) = Self::get_rule(self.tokens.current).infix else {
                break;
            };

            expr = infix(self, &expr)?;
        }

        Ok(expr)
    }
}
