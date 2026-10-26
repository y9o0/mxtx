use anyhow::Result as Ares;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Value {
    pub fn as_num(self) -> Ares<f64> {
        match self {
            Value::Number(x) => Ok(x),
        }
    }
}
