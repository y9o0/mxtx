use std::f64::consts::{E, PI};

use crate::{
    ast::Expr,
    definitions::Function,
    lexer::{
        Token::{self},
        tokenize,
    },
    values::Value,
};

use anyhow::{Context, Ok, Result as Ares, anyhow};

#[derive(Default, Clone, Debug)]
pub struct MoveToken<'t> {
    pub pool: Vec<Token<'t>>,
    pub current: Token<'t>,
    pub previous: Token<'t>,
}

#[derive(Debug)]
pub struct Parser<'t> {
    pub tokens: MoveToken<'t>,
    pub line: u8,
}

pub trait OutCome<R> {
    fn result(result: Ares<Expr>) -> R;
    fn option(option: Option<Expr>) -> R;
}

impl<T, R: From<anyhow::Result<Expr>>> OutCome<R> for anyhow::Result<T> {
    fn result(result: Ares<Expr>) -> R {
        R::from(result)
    }

    fn option(option: Option<Expr>) -> R {
        let result = match option {
            Some(x) => Ok(x),
            None => Err(anyhow!("None!")),
        };

        R::from(result)
    }
}

impl<'t> Parser<'t> {
    pub fn new(source: &'t str) -> Ares<Self> {
        let tokens = tokenize(source)?;

        Ok(Self {
            tokens: MoveToken {
                current: Token::default(),
                previous: Token::default(),
                pool: tokens,
            },
            line: 0,
        })
    }

    pub fn parse_function(mut self) -> Ares<Function<'t>> {
        self.advance()?;
        let Token::Ident(name) = self.tokens.current else {
            return Err(anyhow!(format!(
                "Expected token '(Ident) function name' found {:?}",
                self.tokens.current
            )));
        };

        self.advance()?;
        self.match_or_err(Token::LeftParen, true, None)?;

        self.advance()?;
        let mut params: Vec<&str> = Vec::new();
        if self.tokens.current != Token::RightParen {
            loop {
                let Token::Ident(x) = self.tokens.current else {
                    return Err(anyhow!(
                        "Expected param name, found {:?}",
                        self.tokens.current
                    ));
                };
                params.push(x);
                self.advance()?;
                if self.tokens.current == Token::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }

        self.match_or_err(Token::RightParen, true, None)?;
        self.advance()?;

        self.match_or_err(Token::Equal, true, None)?;
        self.advance()?;

        if self.tokens.current == Token::Eof {
            return Err(anyhow!("Expected an equation found nothing!"));
        }

        let mut eq = Vec::new();
        while self.tokens.current != Token::Eof {
            self.advance()?;

            let tokens = self.tokens.previous;

            eq.push(tokens);
        }

        Ok(Function {
            name: name.to_string(),
            param: params.iter().map(|s| s.to_string()).collect(),
            equation: eq,
        })
    }

    pub fn advance(&mut self) -> Ares<()> {
        self.tokens.previous = self.tokens.current;

        self.tokens.current = *self.tokens.pool.get(self.line as usize).context(format!(
            "Couldn't find anything at position -{}-",
            self.line
        ))?;

        self.line += 1;
        Ok(())
    }

    fn match_or_err(
        &self,
        expected: Token<'static>,
        at_cur: bool,
        extra: Option<&str>,
    ) -> Ares<()> {
        let found;

        if at_cur {
            found = self.tokens.current
        } else {
            found = self.tokens.previous
        }

        if expected != found {
            if let Some(ext) = extra {
                return Err(anyhow!(
                    "Unexpected token expected ({} : '{:?}') found {:?} at {}",
                    ext,
                    expected,
                    self.tokens.current,
                    self.line
                ));
            } else {
                return Err(anyhow!(
                    "Unexpected token expected '{:?}' found {:?} at {}",
                    expected,
                    self.tokens.current,
                    self.line
                ));
            }
        } else {
            Ok(())
        }
    }

    #[allow(warnings)]
    fn match_consume(&mut self, expected: Token) -> Ares<bool> {
        self.advance()?;

        if self.tokens.previous == expected {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Parser<'_> {
    pub fn number<R: OutCome<R> + Into<Ares<Expr>>>(&mut self) -> R {
        let result = || -> Ares<Expr> {
            let Token::Num(x) = self.tokens.previous else {
                return Err(anyhow!(
                    "Expected a number found '{:?}'",
                    self.tokens.previous
                ));
            };

            Ok(Expr::Number {
                value: crate::values::Value::Number(x),
            })
        };

        R::result(result())
    }

    pub fn binary<R: OutCome<R> + Into<Ares<Expr>>>(&mut self, left: &Expr) -> R {
        let mut result = || -> Ares<Expr> {
            self.advance()?;
            let op = self.tokens.previous;

            let bin_op = op.to_binop();

            let right = self
                .parse_precedence::<R>(Self::get_rule::<R>(op).precedence.next())
                .into()?;

            Ok(Expr::Binary {
                left: Box::new(left.to_owned()),
                right: Box::new(right),
                operator: bin_op,
            })
        };

        R::result(result())
    }

    pub fn unary<R: OutCome<R> + Into<Ares<Expr>>>(&mut self) -> R {
        let mut result = || -> Ares<Expr> {
            let op = self.tokens.previous;
            self.advance()?;

            let Token::Num(x) = self.tokens.previous else {
                return Err(anyhow!(
                    "Expected a number found '{:?}'",
                    self.tokens.previous
                ));
            };

            Ok(match op {
                Token::Sub => Expr::Number {
                    value: Value::Number(-x),
                },
                _ => {
                    return Err(anyhow!("Unexpected unray operater '{:?}'", op));
                }
            })
        };

        R::result(result())
    }

    pub fn group<R: OutCome<R> + Into<Ares<Expr>>>(&mut self) -> R {
        let mut result = || -> Ares<Expr> {
            let expr = self
                .parse_precedence::<R>(crate::pratt::Precedence::None)
                .into()?;

            self.advance()?;
            self.match_or_err(Token::RightParen, false, Some("closing ')' of group"))?;

            Ok(Expr::Group(Box::new(expr)))
        };

        R::result(result())
    }

    pub fn radical<R: OutCome<R> + Into<Ares<Expr>>>(&mut self) -> R {
        let mut result = || -> Ares<Expr> {
            let op = self.tokens.previous.to_binop();

            let right = self
                .parse_precedence::<R>(crate::pratt::Precedence::None)
                .into()?;

            Ok(Expr::Radical {
                operator: op,
                right: Box::new(right),
            })
        };

        R::result(result())
    }

    pub fn symbols<R: OutCome<R> + Into<Ares<Expr>>>(&mut self) -> R {
        let option = || -> Option<Expr> {
            let Token::Ident(x) = self.tokens.previous else {
                return None;
            };

            Some(match x {
                "pi" | "π" => Expr::Number {
                    value: Value::Number(PI),
                },
                _ => return None,
            })
        };

        R::option(option())
    }
}

pub fn function(equation: &str) -> Ares<Function<'_>> {
    let parser = Parser::new(equation)?;
    let func = parser.parse_function()?;

    Ok(func)
}

pub fn equation(equation: &str) -> Ares<f64> {
    let mut parser = Parser::new(equation)?;
    parser.advance()?;

    let expr = parser.parse_precedence::<Ares<Expr>>(crate::pratt::Precedence::None)?;

    let Value::Number(x) = Expr::codegen(expr.clone())? else {
        return Err(anyhow!("Expected a number found '{:?}'", expr));
    };

    Ok(x)
}

impl Function<'_> {
    pub fn eval(&mut self, params: &[f64]) -> Ares<f64> {
        if self.param.len() != params.len() {
            return Err(anyhow!(
                "Expected '{}' amount of params due to '{}' has '{}' amount of params",
                &self.param.len(),
                &self.name,
                self.param.len()
            ));
        }

        let bind: std::collections::HashMap<&str, &f64> = self
            .param
            .iter()
            .map(String::as_str)
            .zip(params.iter())
            .collect();

        let mut toks: Vec<Token> = Vec::with_capacity(self.equation.len());

        for i in &self.equation {
            toks.push(match i {
                Token::Ident(x) => match bind.get(x) {
                    Some(x) => Token::Num(**x),
                    None => {
                        if x == &"pi" || x == &"π" {
                            continue;
                        } else {
                            return Err(anyhow!("Unknown variable '{}'", x));
                        }
                    }
                },
                other => *other,
            });
        }

        let equation = toks
            .iter()
            .map(|t| t.to_owned().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let mut expr = Parser::new(&equation)?;
        expr.advance()?;

        let expr = expr.parse_precedence::<Ares<Expr>>(crate::pratt::Precedence::None)?;

        let Value::Number(x) = Expr::codegen(expr.clone())? else {
            return Err(anyhow!("Expected a number found '{:?}'", expr));
        };

        Ok(x)
    }
}
