use super::*;

impl Parser {
    pub fn binary(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();
        let op = self.previous.token_type;

        let right = self.parse_precedence(Self::get_rule(op).precedence.next())?;

        Ok(Expr::Binary {
            left: Box::new(left.to_owned()),
            right: Box::new(right),
            operator: op,
        })
    }

    pub fn unary(&mut self) -> SemaResult<Expr> {
        let op = self.previous.token_type;

        let right = self.parse_precedence(crate::pratt::Precedence::None)?;

        Ok(Expr::Radical {
            operator: op,
            right: Box::new(right),
        })
    }

    pub fn suffix(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();

        let op = self.previous.token_type;

        Ok(Expr::Suffix {
            left: Box::new(left.to_owned()),
            operator: op,
        })
    }
}
