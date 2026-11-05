use super::*;

impl Expr {
    pub fn codegen(expr: &Expr) -> GenResult<Expr> {
        match expr {
            Expr::Number(x) => Ok(Expr::Number(*x)),
            Expr::Group(x) => Self::codegen(*&x),

            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let left = Self::codegen(left)?;
                let right = Self::codegen(right)?;

                let ln = left.unpack_num()?;
                let rn = right.unpack_num()?;

                return Ok(match operator {
                    TokenType::Plus => Expr::Number(ln + rn),
                    TokenType::Minus => Expr::Number(ln - rn),
                    TokenType::Div => Expr::Number(ln / rn),
                    TokenType::Mul => Expr::Number(ln * rn),
                    TokenType::Pow => Expr::Number(ln.powf(rn)),
                    TokenType::Greater => Expr::Bool(ln > rn),
                    TokenType::GreaterEq => Expr::Bool(ln >= rn),
                    TokenType::Lesser => Expr::Bool(ln < rn),
                    TokenType::LesserEq => Expr::Bool(ln <= rn),
                    TokenType::Equal => Expr::Bool(ln == rn),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codgen 'binary'".to_string(),
                            message: format!("'{:?}'", operator),
                        });
                    }
                });
            }
            Expr::Radical { operator, right } => {
                let right = Self::codegen(right)?;

                let rn = right.unpack_num()?;

                Ok(match operator {
                    TokenType::Sqrt => Expr::Number(rn.sqrt()),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codegen 'radical'".to_string(),
                            message: format!("'{:?}'", operator),
                        });
                    }
                })
            }
            Expr::Trig { function, body } => {
                let body = Self::codegen(body)?;

                let result = body.unpack_num()?;

                Ok(match function {
                    TokenType::Sin => Expr::Number(result.sin()),
                    TokenType::Cos => Expr::Number(result.cos()),
                    TokenType::Tan => Expr::Number(result.tan()),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codegen 'trig'".to_string(),
                            message: format!("'{:?}'", function),
                        });
                    }
                })
            }
            Expr::Bool(x) => Ok(Expr::Bool(*x)),
        }
    }
}
