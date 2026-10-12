use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("PipeWire error: {0}")]
    PipeWire(String),

    #[error("Audio engine error: {0}")]
    Audio(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[allow(dead_code)]
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
