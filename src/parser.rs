use crate::{
    definitions::Function,
    lexer::{Token, tokenize},
};
use anyhow::{Context, Result as Ares, anyhow};

#[derive(Default, Clone)]
pub struct MoveToken<'t> {
    pool: Vec<Token<'t>>,
    current: Token<'t>,
    previous: Token<'t>,
}

pub struct Parser<'t> {
    tokens: MoveToken<'t>,
    line: u8,
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

    pub fn parse_function(&mut self) -> Ares<Function> {
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
        self.advance()?;

        if self.tokens.current != Token::RightParen {
            let Token::Ident(param) = self.tokens.previous else {
                return Err(anyhow!(format!(
                    "Expected token '(idnet) param' found {:?}",
                    self.tokens.previous
                )));
            };

            params.push(param);
        }

        while self.match_consume(Token::Comma)? {
            let Token::Ident(param) = self.tokens.current else {
                return Err(anyhow!(format!(
                    "Expected token '(idnet) param' found {:?}",
                    self.tokens.current
                )));
            };

            params.push(param);

            self.advance()?;

            if self.tokens.current == Token::RightParen {
                break;
            }
        }

        self.match_or_err(Token::RightParen, true, None)?;
        self.advance()?;

        Ok(Function {
            name: name.to_string(),
            param: params.iter().map(|s| s.to_string()).collect(),
        })
    }

    fn advance(&mut self) -> Ares<()> {
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

    fn match_consume(&mut self, expected: Token) -> Ares<bool> {
        self.advance()?;

        if self.tokens.previous == expected {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn function(equation: &str) -> Ares<()> {
    let parser = Parser::new(equation)?.parse_function()?;
    println!("{:?}", parser);
    Ok(())
}
