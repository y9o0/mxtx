use crate::scanner::tokens::TokenType;

#[derive(Debug, thiserror::Error)]
pub enum SemaErr {
    #[error("{} at line : {}", message, line)]
    Expected { line: u32, message: String },

    #[error("Expected '{:?}' but found '{:?}' at line '{}'", expected, found, line)]
    Syntax {
        expected: TokenType,
        found: TokenType,
        line: u32,
    },

    #[error("Unexpected '{}' at line '{}'", message, line)]
    Unexpected { line: u32, message: String },

    #[error("{}", message)]
    OutOfDomain { message: String },
}

pub type SemaResult<T> = std::result::Result<T, SemaErr>;
