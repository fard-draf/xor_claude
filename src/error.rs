use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Fatal error")]
    FatalError,

    #[error("Empty file")]
    EmptyFile,

    #[error("Invalid format")]
    UnvalidFormat,

    #[error("Invalid Key: must be exactly 25 ASCII characters")]
    UnvalidKey,

    #[error("Compression failed")]
    CompressionError,

    #[error("Decompression failed")]
    DecompressionError,
}

pub type Result<T> = std::result::Result<T, AppError>;
