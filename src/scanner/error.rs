#[derive(thiserror::Error, Debug)]

pub enum ScanErr {
    #[error("Unexpected Token: in {at} at line {line} ")]
    Unexpected { line: u32, at: String },
}

pub type ScanResult<T> = std::result::Result<T, ScanErr>;
