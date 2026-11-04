use crate::scanner::tokens::TokenType;

use super::*;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Token,
    pub previous: Token,
    pub line: u32,
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

    pub fn equation(&mut self) -> SemaResult<f64> {
        self.advance();

        let expr = self.parse_precedence(crate::pratt::Precedence::None)?;
        let result = Expr::codegen(&expr).unwrap();
        let num = result.unpack_num().unwrap();

        Ok(num)
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
