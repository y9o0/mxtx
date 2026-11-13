#[derive(Debug, thiserror::Error)]
pub enum GenErr {
    #[error("Unexpected '{}' at '{}'", message, at)]
    Unexpected { at: String, message: String },

    #[error("the value that has been passed is out of the domian of the function")]
    OutofDomain,

    #[error("the value that has been passed is out of the domian of the function")]
    OutofRange,
}

pub type GenResult<T> = std::result::Result<T, GenErr>;
