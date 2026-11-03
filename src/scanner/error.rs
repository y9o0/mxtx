#[derive(thiserror::Error, Debug)]
pub enum ScanErr {
    #[error("Unexpected Token: in {at} at line {line} ")]
    Unexpected { line: u32, at: String },
    #[error("Unterminated string at line : {}", line)]
    Unterminated { line: u32 },
    #[error("{} at {}", message, line)]
    Expected { line: u32, message: String },
}

pub type ScanResult<T> = std::result::Result<T, ScanErr>;
