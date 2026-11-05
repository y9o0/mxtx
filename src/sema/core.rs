use ::core::fmt;

use crate::scanner::tokens::TokenType;

use super::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Token,
    pub previous: Token,
    pub line: u32,
}

pub enum Answer {
    Num(f64),
    Bool(bool),
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
        }
    }

    pub fn equation(&mut self) -> SemaResult<Answer> {
        self.advance();

        let expr = self.parse_precedence(crate::pratt::Precedence::None)?;
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

    pub fn advance(&mut self) {
        self.previous = std::mem::take(&mut self.current);

        self.current = std::mem::take(&mut self.tokens[self.line as usize]);

        self.line += 1;
    }
}
