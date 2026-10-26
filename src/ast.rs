use crate::values::Value;
use anyhow::{Result as Ares, anyhow};

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    Nai,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number {
        value: Value,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: BinOp,
    },
    Radical {
        operator: BinOp,
        right: Box<Expr>,
    },
    Group(Box<Expr>),
}

impl Expr {
    pub fn codegen(expr: Expr) -> Ares<Value> {
        match expr {
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let left = Expr::codegen(*left)?;
                let right = Expr::codegen(*right)?;

                match operator {
                    BinOp::Add => Ok(Value::Number(left.as_num()? + right.as_num()?)),
                    BinOp::Sub => Ok(Value::Number(left.as_num()? - right.as_num()?)),
                    BinOp::Mul => Ok(Value::Number(left.as_num()? * right.as_num()?)),
                    BinOp::Div => Ok(Value::Number(left.as_num()? / right.as_num()?)),
                    BinOp::Pow => Ok(Value::Number(left.as_num()?.powf(right.as_num()?))),
                    _ => Err(anyhow!("Unexpected operator")),
                }
            }
            Expr::Number { value } => Ok(value),
            Expr::Group(x) => Expr::codegen(*x),
            Expr::Radical { operator, right } => {
                let right = Expr::codegen(*right)?;

                match operator {
                    BinOp::Sqrt => Ok(Value::Number(right.as_num()?.sqrt())),
                    _ => Err(anyhow!("Unexpected operator")),
                }
            }
        }
    }
}
