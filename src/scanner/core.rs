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
                '/' => self.add_token(TokenType::Div, None),
                '+' => self.add_token(TokenType::Plus, None),
                '-' => self.add_token(TokenType::Minus, None),
                '*' => self.add_token(TokenType::Mul, None),
                '^' => self.add_token(TokenType::Pow, None),
                '√' => self.add_token(TokenType::Sqrt, None),
                '=' => self.add_token(TokenType::Equal, None),
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
}
