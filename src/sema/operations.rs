use crate::pratt::Precedence::Comparison;

use super::*;

impl Parser {
    pub fn binary(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();
        let op = self.previous.token_type;

        let prec = Self::get_rule(op).precedence;

        let min = if op == TokenType::Pow {
            prec
        } else {
            prec.next()
        };

        let right = self.parse_precedence(min)?;

        Ok(Expr::Binary {
            left: Box::new(left.to_owned()),
            right: Box::new(right),
            operator: op,
        })
    }

    pub fn compersion(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();
        let op = self.previous.token_type;

        let right = self.parse_precedence(Comparison.next())?;

        if matches!(
            self.current.token_type,
            TokenType::Equal
                | TokenType::Greater
                | TokenType::GreaterEq
                | TokenType::Lesser
                | TokenType::LesserEq
                | TokenType::NotEq
        ) {
            let expr = self.compersion(&right)?;

            return Ok(Expr::Chain {
                left_part: Box::new(Expr::Binary {
                    left: Box::new(left.to_owned()),
                    right: Box::new(right),
                    operator: op,
                }),
                right_part: Box::new(expr),
            });
        } else {
            return Ok(Expr::Binary {
                left: Box::new(left.to_owned()),
                right: Box::new(right),
                operator: op,
            });
        }
    }

    pub fn unary(&mut self) -> SemaResult<Expr> {
        let op = self.previous.token_type;

        let right = self.parse_precedence(crate::pratt::Precedence::Unary)?;

        Ok(Expr::Radical {
            operator: op,
            right: Box::new(right),
        })
    }

    pub fn postfix(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();

        let op = self.previous.token_type;

        Ok(Expr::PostFix {
            left: Box::new(left.to_owned()),
            operator: op,
        })
    }
}
