use crate::scanner::{
    error::ScanResult,
    tokens::{Literal, TokenContent, TokenType},
};

use super::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    start: u32,
    current: u32,
    line: u32,
}

impl<'l> Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scanner(&mut self) -> ScanResult<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan()?
        }
        self.add_token(TokenType::Eof, None);

        return Ok(std::mem::take(&mut self.tokens));
    }

    fn scan(&mut self) -> ScanResult<()> {
        let char = self.advance();

        match char {
            Some(c) => match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                ',' => self.add_token(TokenType::Comma, None),
                '/' => {
                    if self.match_next('/') {
                        while self.peek() != '\0' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Div, None)
                    }
                }
                '+' => self.add_token(TokenType::Plus, None),
                '-' => self.add_token(TokenType::Minus, None),
                '*' => self.add_token(TokenType::Mul, None),
                '^' => self.add_token(TokenType::Pow, None),
                '√' => self.add_token(TokenType::Sqrt, None),
                '÷' => self.add_token(TokenType::Div, None),
                '·' => self.add_token(TokenType::Mul, None),
                '!' => self.add_token(TokenType::Bang, None),
                '=' => self.add_token(TokenType::Equal, None),
                '_' => self.add_token(TokenType::Underscore, None),
                'φ' => self.add_token(TokenType::GoldenRatio, None),
                '>' => {
                    if self.match_next('=') {
                        self.add_token(TokenType::GreaterEq, None);
                    } else {
                        self.add_token(TokenType::Greater, None);
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        self.add_token(TokenType::LesserEq, None);
                    } else {
                        self.add_token(TokenType::Lesser, None);
                    }
                }
                '|' => self.add_token(TokenType::Pipe, None),
                'π' => self.add_token(TokenType::Pi, None),
                'e' => self.add_token(TokenType::EulersNumber, None),
                x if x.is_alphabetic() => {
                    self.string()?;

                    let Some(last_token) = self.tokens.pop() else {
                        return Err(error::ScanErr::Unexpected {
                            line: self.line,
                            at: "Expected a token found none!".to_string(),
                        });
                    };

                    let string: &str = match last_token.literal {
                        Some(Literal::Ident(ref x)) => x,
                        _ => {
                            return Err(error::ScanErr::Expected {
                                line: self.line,
                                message: "Expected a string literal found none!".to_string(),
                            });
                        }
                    };

                    if !self.key_words(string) {
                        self.tokens.push(last_token);
                    }
                }
                x if x.is_numeric() => self.number()?,
                ' ' | '\r' | '\t' | '\n' => {
                    self.line += 1;
                }

                _ => {
                    return Err(error::ScanErr::Unexpected {
                        line: self.line,
                        at: c.to_string(),
                    });
                }
            },
            _ => {
                self.add_token(TokenType::Eof, None);
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let txt = &self.source[self.start as usize..self.current as usize];
        let content = TokenContent::from((token_type, txt.iter().collect(), self.line, literal));
        self.tokens.push(Token::new(content));
    }

    fn advance(&mut self) -> Option<char> {
        let Some(c) = self.source.get(self.current as usize) else {
            return None;
        };

        self.current += 1;
        self.line += 1;

        Some(*c)
    }

    fn match_next(&mut self, expected: char) -> bool {
        let Some(current) = self.source.get(self.current as usize) else {
            return false;
        };

        if self.is_at_end() || *current != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source[self.current as usize];
        }
    }

    fn string(&mut self) -> ScanResult<()> {
        while !self.is_at_end() && self.peek().is_alphabetic() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(error::ScanErr::Unterminated { line: self.line });
        }

        let string = self.source[(self.start) as usize..(self.current) as usize]
            .iter()
            .collect::<String>();

        self.add_token(TokenType::Ident, Some(Literal::Ident(string)));

        Ok(())
    }

    fn number(&mut self) -> ScanResult<()> {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let string = self.source[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();

        let Ok(number) = string.trim().parse::<f64>() else {
            return Err(error::ScanErr::Unexpected {
                line: self.line,
                at: string,
            });
        };

        self.add_token(TokenType::Num, Some(Literal::Number(number)));
        Ok(())
    }

    fn peek_next(&mut self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        } else {
            self.source[(self.current + 1) as usize]
        }
    }

    fn key_words(&mut self, ident: &str) -> bool {
        let matched_on: TokenType = match ident.trim() {
            "sqrt" => TokenType::Sqrt,
            "pi" => TokenType::Pi,
            "gr" => TokenType::GoldenRatio,
            "sin" => TokenType::Sin,
            "cos" => TokenType::Cos,
            "tan" => TokenType::Tan,
            "log" => TokenType::Log,
            "ln" => TokenType::Ln,
            "mod" => TokenType::Mod,
            "where" => TokenType::Where,
            "and" => TokenType::And,
            _ => {
                return false;
            }
        };

        self.add_token(matched_on, None);
        true
    }
}
