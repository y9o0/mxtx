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
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codgen 'binary'".to_string(),
                            message: format!("'{:?}'", operator),
                        });
                    }
                });
            }
            _ => {
                return Err(GenErr::Unexpected {
                    at: "codegen".to_string(),
                    message: format!("'{:?}' expr", expr),
                });
            }
        }
    }
}
