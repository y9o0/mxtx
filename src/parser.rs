use crate::{
    ast::Expr,
    definitions::Function,
    lexer::{
        Token::{self},
        tokenize,
    },
    values::Value,
};

use anyhow::{Context, Result as Ares, anyhow};

#[derive(Default, Clone)]
pub struct MoveToken<'t> {
    pub pool: Vec<Token<'t>>,
    pub current: Token<'t>,
    pub previous: Token<'t>,
}

pub struct Parser<'t> {
    pub tokens: MoveToken<'t>,
    pub line: u8,
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
    pub fn number(&mut self) -> Ares<Expr> {
        let Token::Num(x) = self.tokens.previous else {
            return Err(anyhow!(
                "Expected a number found '{:?}'",
                self.tokens.previous
            ));
        };

        Ok(Expr::Number {
            value: crate::values::Value::Number(x),
        })
    }

    pub fn binary(&mut self, left: &Expr) -> Ares<Expr> {
        self.advance()?;
        let op = self.tokens.previous;

        let bin_op = op.to_binop();

        let right = self.parse_precedence(Self::get_rule(op).precedence.next())?;

        Ok(Expr::Binary {
            left: Box::new(left.to_owned()),
            right: Box::new(right),
            operator: bin_op,
        })
    }

    pub fn unary(&mut self) -> Ares<Expr> {
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
    }

    pub fn group(&mut self) -> Ares<Expr> {
        let mut tokens = Vec::new();
        while self.tokens.current != Token::RightParen {
            self.advance()?;
            let token = self.tokens.previous;
            tokens.push(token);
        }

        self.advance()?;
        self.match_or_err(Token::RightParen, false, None)?;

        let text = tokens
            .iter()
            .map(|t| t.to_owned().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let mut parser = Parser::new(&text)?;
        parser.advance()?;
        let expr = parser.parse_precedence(crate::pratt::Precedence::None)?;

        Ok(Expr::Group(Box::new(expr)))
    }

    pub fn radical(&mut self) -> Ares<Expr> {
        let op = self.tokens.previous.to_binop();

        let right = self.parse_precedence(crate::pratt::Precedence::None)?;

        Ok(Expr::Radical {
            operator: op,
            right: Box::new(right),
        })
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
    let expr = parser.parse_precedence(crate::pratt::Precedence::None)?;
    let Value::Number(x) = Expr::codegen(expr)?;
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
                    None => return Err(anyhow!("Unknown variable '{}'", x)),
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

        let expr = expr.parse_precedence(crate::pratt::Precedence::None)?;
        let Value::Number(x) = Expr::codegen(expr)?;

        Ok(x)
    }
}
