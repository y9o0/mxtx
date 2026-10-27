use anyhow::{Result as Ares, anyhow};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    None,
}

impl Value {
    pub fn as_num(self) -> Ares<f64> {
        match self {
            Value::Number(x) => Ok(x),
            _ => Err(anyhow!(
                "the value '{:?}' can not be converted into numbers",
                self
            )),
        }
    }
}
