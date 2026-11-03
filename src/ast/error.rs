#[derive(Debug, thiserror::Error)]
pub enum GenErr {
    #[error("Unexpected '{}' at '{}'", message, at)]
    Unexpected { at: String, message: String },
}

pub type GenResult<T> = std::result::Result<T, GenErr>;
