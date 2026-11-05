use super::*;

impl Parser {
    pub fn trig_functions(&mut self) -> SemaResult<Expr> {
        let function = self.previous.token_type;

        self.match_err(TokenType::LeftParen)?;
        let expr = self.group()?;

        Ok(Expr::Trig {
            function,
            body: Box::new(expr),
        })
    }
}
