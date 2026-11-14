use crate::ast::error::GenErr::{OutofDomain, OutofRange};

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

                let ln = left.unpack_num("binary")?;
                let rn = right.unpack_num("binary")?;

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
                    TokenType::Mod => Expr::Number(ln % rn),
                    TokenType::NotEq => Expr::Bool(ln != rn),
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

                let rn = right.unpack_num("radical")?;

                Ok(match operator {
                    TokenType::Sqrt => Expr::Number(rn.sqrt()),
                    TokenType::Minus => Expr::Number(-rn),
                    TokenType::Bang => Expr::Number((1..=rn as u64).fold(1.0, |a, i| a * i as f64)),
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

                let result = body.unpack_num("trig")?;

                Ok(match function {
                    TokenType::Sin => Expr::Number(result.sin()),
                    TokenType::Cos => Expr::Number(result.cos()),
                    TokenType::Tan => Expr::Number(result.tan()),
                    TokenType::ArcSin => Expr::Number(result.asin()),
                    TokenType::ArcCos => Expr::Number(result.acos()),
                    TokenType::ArcTan => Expr::Number(result.atan()),
                    TokenType::Csc => Expr::Number(1.0 / result.sin()),
                    TokenType::Sec => Expr::Number(1.0 / result.cos()),
                    TokenType::Cot => Expr::Number(1.0 / result.tan()),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codegen 'trig'".to_string(),
                            message: format!("'{:?}'", function),
                        });
                    }
                })
            }
            Expr::Bool(x) => Ok(Expr::Bool(*x)),
            Expr::Absolute(x) => {
                let result = Self::codegen(&*x)?;

                let rn = result.unpack_num("abs")?;

                Ok(Expr::Number(rn.abs()))
            }
            Expr::Logarithm {
                function,
                base,
                body,
            } => {
                let body = Self::codegen(body)?;

                let mut b: Option<Expr> = None;

                if let Some(x) = base {
                    b = Some(Self::codegen(&x)?);
                }

                Ok(match function {
                    TokenType::Log => Expr::Number(
                        body.unpack_num("logs")?
                            .log(b.unwrap_or(Expr::Number(10.0)).unpack_num("logs")?),
                    ),
                    TokenType::Ln => Expr::Number(body.unpack_num("logs")?.ln()),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codegen 'log'".to_string(),
                            message: format!("'{:?}'", function),
                        });
                    }
                })
            }
            Expr::Ident(x) => Ok(Expr::Ident(x.to_string())),

            Expr::PostFix { left, operator } => {
                let left = Self::codegen(left)?;

                let ln = left.unpack_num("postfix")?;

                Ok(match operator {
                    TokenType::Bang => Expr::Number((1..=ln as u64).fold(1.0, |a, i| a * i as f64)),
                    _ => {
                        return Err(GenErr::Unexpected {
                            at: "codegen 'suffix'".to_string(),
                            message: format!("Unexpected operator '{:?}'", operator),
                        });
                    }
                })
            }

            Expr::Where { left, right } => {
                let left = left.to_owned().rep(&mut right.borrow_mut())?;
                let result = Self::codegen(&left)?;

                Ok(result)
            }

            Expr::Domain { cond, left } => {
                let cond_expr = Self::codegen(cond)?;
                let bool = cond_expr.unpack_bool("domain")?;

                if !*bool {
                    return Err(OutofDomain);
                } else {
                    let expr = Self::codegen(left)?;
                    Ok(expr)
                }
            }

            Expr::Range { left, cond } => {
                let left = Expr::codegen(left)?;

                let result = Self::codegen(cond)?;
                let bool = result.unpack_bool("range")?;

                if !*bool {
                    return Err(OutofRange);
                } else {
                    let expr = Self::codegen(&left)?;
                    Ok(expr)
                }
            }

            Expr::Chain {
                left_part,
                right_part,
            } => {
                let left_part = Self::codegen(left_part)?;
                let right_part = Self::codegen(right_part)?;

                let blp = left_part.unpack_bool("chain")?;
                let brp = right_part.unpack_bool("chain")?;

                if *blp && *brp {
                    return Ok(Expr::Bool(true));
                } else {
                    return Ok(Expr::Bool(false));
                }
            }
        }
    }
}
