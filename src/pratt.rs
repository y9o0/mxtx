use crate::{
    ast::exprs::Expr,
    scanner::tokens::TokenType,
    sema::{
        core::Parser,
        error::{self, SemaResult},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    None,
    Comparison, // > < >= <=
    Term,       // + -
    Factor,     // * / mod
    Radical,    // (sqrt | √) ^
    Unary,      // - sin() cos()
    Group,      // () | |
    Primary,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser) -> SemaResult<Expr>>,
    pub infix: Option<fn(&mut Parser, &Expr) -> SemaResult<Expr>>,
    pub precedence: Precedence,
}

// it start from the lowst to the highst!
impl Precedence {
    pub fn next(self) -> Precedence {
        match self {
            Precedence::None => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Radical,
            Precedence::Radical => Precedence::Group,
            _ => Precedence::Primary,
        }
    }
}

impl Parser {
    pub fn get_rule(token: TokenType) -> ParseRule {
        match token {
            TokenType::Num => ParseRule {
                prefix: Some(Parser::numbers),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Plus => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Minus => ParseRule {
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Mul => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Div => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Pow => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Radical,
            },
            TokenType::LeftParen => ParseRule {
                prefix: Some(Parser::group),
                infix: None,
                precedence: Precedence::Group,
            },
            TokenType::Sqrt => ParseRule {
                prefix: Some(Parser::unary),
                infix: None,
                precedence: Precedence::Radical,
            },
            TokenType::Pi => ParseRule {
                prefix: Some(Parser::symbols),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EulersNumber => ParseRule {
                prefix: Some(Parser::symbols),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Sin => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::Unary,
            },
            TokenType::Cos => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::Unary,
            },
            TokenType::Tan => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Greater => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::GreaterEq => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Lesser => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::LesserEq => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Equal => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::None,
            },
            TokenType::Pipe => ParseRule {
                prefix: Some(Parser::absolute),
                infix: None,
                precedence: Precedence::Group,
            },
            TokenType::Log => ParseRule {
                prefix: Some(Parser::logarithm),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Ln => ParseRule {
                prefix: Some(Parser::logarithm),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Ident => ParseRule {
                prefix: Some(Parser::ident),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Where => ParseRule {
                prefix: None,
                infix: Some(Parser::where_stmt),
                precedence: Precedence::None,
            },
            TokenType::Bang => ParseRule {
                prefix: None,
                infix: Some(Parser::suffix),
                precedence: Precedence::Unary,
            },
            TokenType::GoldenRatio => ParseRule {
                prefix: Some(Parser::symbols),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Mod => ParseRule {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::ArcSin => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::ArcCos => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::ArcTan => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Csc => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Sec => ParseRule {
                prefix: Some(Parser::trig_functions),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Cot => ParseRule {
                prefix: Some(Parser::trig_functions),
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

    pub fn parse_precedence(&mut self, min: Precedence) -> SemaResult<Expr> {
        let mut expr: Expr;

        self.advance();

        let rule = Self::get_rule(self.previous.token_type);

        if let Some(prefix) = rule.prefix {
            expr = prefix(self)?;
        } else {
            return Err(error::SemaErr::Expected {
                line: self.line,
                message: "Expected an expr found None!".to_string(),
            });
        };

        while min <= Self::get_rule(self.current.token_type).precedence {
            let Some(infix) = Self::get_rule(self.current.token_type).infix else {
                break;
            };

            expr = infix(self, &expr)?;
        }

        Ok(expr)
    }
}
