use std::collections::HashMap;

use super::*;

impl Parser {
    pub fn where_stmt(&mut self, left: &Expr) -> SemaResult<Expr> {
        let mut values: HashMap<String, f64> = HashMap::new();

        while self.current.token_type != TokenType::Eof {
            self.advance();

            self.advance();
            let ident = self.ident()?.unpack_ident().unwrap();

            self.match_err(TokenType::Equal)?;

            let expr = self.parse_precedence(crate::pratt::Precedence::None)?;
            let result = Expr::codegen(&expr).unwrap().unpack_num().unwrap();

            values.insert(ident, result);

            if self.previous.token_type == TokenType::And {
                self.advance();

                continue;
            }
        }

        self.checkers.has_where = true;

        Ok(Expr::Where {
            left: Box::new(left.to_owned()),
            right: values,
        })
    }
}
