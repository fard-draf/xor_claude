use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Fatal error")]
    FatalError,

    #[error("Empty file")]
    EmptyFile,

    #[error("Unvalid format")]
    UnvalidFormat,

    #[error("Unvalid Key")]
    UnvalidKey,
    // #[error("Unvalid format")]
    // UnvalidFormat,

    // #[error("Empty file")]
    // EmptyFile,

    // #[error("Unvalid format")]
    // UnvalidFormat,

    // #[error("Empty file")]
    // EmptyFile,

    // #[error("Unvalid format")]
    // UnvalidFormat,
}

pub type Result<T> = std::result::Result<T, AppError>;
