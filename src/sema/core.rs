use ::core::fmt;

use crate::scanner::tokens::TokenType;

use super::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Token,
    pub previous: Token,
    pub line: u32,
    pub checkers: Checkers,
}

#[derive(Debug)]
pub struct Checkers {
    pub has_ident: bool,
    pub has_where: bool,
    pub iaf: bool,
    pub irange: bool,
}

pub enum Answer {
    Num(f64),
    Bool(bool),
    Eval(Expr),
}

pub trait FromAnswer {
    fn from_answer(a: Answer) -> Option<Self>
    where
        Self: Sized;
}

impl FromAnswer for f64 {
    fn from_answer(a: Answer) -> Option<Self>
    where
        Self: Sized,
    {
        match a {
            Answer::Num(x) => Some(x),
            _ => None,
        }
    }
}

impl FromAnswer for bool {
    fn from_answer(a: Answer) -> Option<Self>
    where
        Self: Sized,
    {
        match a {
            Answer::Bool(x) => Some(x),
            _ => None,
        }
    }
}

impl Answer {
    pub fn get<T>(self) -> Option<T>
    where
        T: FromAnswer,
    {
        T::from_answer(self)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(x) => write!(f, "{}", x),
            Self::Num(x) => write!(f, "{}", x),
            Self::Eval(x) => write!(f, "{:?}", x),
        }
    }
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            tokens: Scanner::new(source).scanner().unwrap(),
            current: Token::default(),
            previous: Token::default(),
            line: 0,

            checkers: Checkers {
                has_ident: false,
                has_where: false,
                iaf: false,
                irange: false,
            },
        }
    }

    pub fn equation(&mut self) -> SemaResult<Answer> {
        self.advance();

        if self.matched(TokenType::IaF, true) {
            self.checkers.iaf = true
        }

        let expr = self.parse_precedence(crate::pratt::Precedence::None)?;

        if self.checkers.has_ident && !self.checkers.has_where {
            return Ok(Answer::Eval(expr));
        }

        let result = Expr::codegen(&expr).unwrap();

        Ok(match result {
            Expr::Number(x) => Answer::Num(x),
            Expr::Bool(x) => Answer::Bool(x),
            _ => {
                return Err(error::SemaErr::Unexpected {
                    line: self.line,
                    message: format!("result '{:?}'", result),
                });
            }
        })
    }

    pub fn match_err(&mut self, expected: TokenType) -> SemaResult<()> {
        if self.current.token_type != expected {
            return Err(error::SemaErr::Syntax {
                expected,
                found: self.current.token_type,
                line: self.line,
            });
        }

        self.advance();

        Ok(())
    }

    pub fn match_peek(&mut self, expected: TokenType) -> SemaResult<()> {
        if self.current.token_type != expected {
            return Err(error::SemaErr::Syntax {
                expected,
                found: self.current.token_type,
                line: self.line,
            });
        }

        Ok(())
    }

    pub fn matched(&mut self, expected: TokenType, at_cur: bool) -> bool {
        let tok = if at_cur {
            self.current.token_type
        } else {
            self.previous.token_type
        };

        if tok == expected {
            self.advance();

            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) {
        self.previous = std::mem::take(&mut self.current);

        self.current = std::mem::take(&mut self.tokens[self.line as usize]);

        self.line += 1;
    }
}
