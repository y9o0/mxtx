use super::*;

impl Parser {
    pub fn trig_functions(&mut self) -> SemaResult<Expr> {
        let function = self.previous.token_type;
        self.match_peek(TokenType::LeftParen)?;
        let expr = self.parse_precedence(crate::pratt::Precedence::None)?;

        Ok(Expr::Trig {
            function,
            body: Box::new(expr),
        })
    }
}
