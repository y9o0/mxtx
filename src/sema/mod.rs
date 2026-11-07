pub mod core;
pub mod error;
pub mod operations;
pub mod primary;
pub mod stmts;
pub mod trig;

use crate::ast::exprs::Expr;
use crate::scanner::tokens::TokenType;
use crate::scanner::{core::Scanner, tokens::Token};
use crate::sema::core::Parser;
use crate::sema::error::SemaResult;
