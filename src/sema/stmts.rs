use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::*;

impl Parser {
    pub fn where_stmt(&mut self, left: &Expr) -> SemaResult<Expr> {
        let mut values: HashMap<String, f64> = HashMap::new();

        while self.current.token_type != TokenType::Eof {
            self.advance();

            self.advance();
            let ident = self.ident()?.unpack_ident("where stmt").unwrap();

            self.match_err(TokenType::Equal)?;

            let expr = self.parse_precedence(crate::pratt::Precedence::None)?;
            let result = Expr::codegen(&expr)
                .unwrap()
                .unpack_num("where stmt")
                .unwrap();

            values.insert(ident, result);

            if self.previous.token_type == TokenType::And {
                self.advance();

                continue;
            }
        }

        self.checkers.has_where = true;

        Ok(Expr::Where {
            left: Box::new(left.to_owned()),
            right: Rc::new(RefCell::new(values)),
        })
    }

    pub fn relations(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();

        let tok = self.previous.token_type;

        match tok {
            TokenType::Range => self.checkers.irange = true,
            TokenType::Domain => self.checkers.idomain = true,
            _ => {}
        }

        if !self.checkers.has_ident && !self.checkers.iaf {
            return Err(error::SemaErr::Unexpected {
                line: self.current.line,
                message: "can not use domian outside of a function".to_string(),
            });
        }

        self.match_err(TokenType::Colon)?;
        let expr = self.parse_precedence(crate::pratt::Precedence::Logic)?;

        match tok {
            TokenType::Domain => Ok(Expr::Domain {
                cond: Box::new(expr),
                left: Box::new(left.to_owned()),
            }),
            TokenType::Range => Ok(Expr::Range {
                left: Box::new(left.to_owned()),
                cond: Box::new(expr),
            }),
            _ => {
                return Err(error::SemaErr::Unexpected {
                    line: self.current.line,
                    message: format!("token '{:?}'", tok),
                });
            }
        }
    }

    pub fn out(&mut self) -> SemaResult<Expr> {
        if !self.checkers.irange {
            return Err(error::SemaErr::Unexpected {
                line: self.current.line,
                message: "can not use out as an ident or out of range".to_string(),
            });
        }

        Ok(Expr::Ident("out".to_string()))
    }

    pub fn and(&mut self, left: &Expr) -> SemaResult<Expr> {
        self.advance();

        let right = self.parse_precedence(crate::pratt::Precedence::Comparison)?;

        Ok(Expr::Chain {
            left_part: Box::new(left.to_owned()),
            right_part: Box::new(right),
        })
    }
}
