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

            self.advance();
            let value = self.numbers()?.unpack_num().unwrap();

            values.insert(ident, value);

            if self.previous.token_type == TokenType::And {
                self.advance();

                continue;
            }
        }

        Ok(Expr::Where {
            left: Box::new(left.to_owned()),
            right: values,
        })
    }
}
