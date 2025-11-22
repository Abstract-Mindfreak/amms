use thiserror::Error;
use uuid::Uuid;

/// Main error type for the MMSS system
#[derive(Error, Debug)]
pub enum Error {
    /// Error during task execution
    #[error("Task execution failed: {0}")]
    TaskExecution(String),

    /// Task not found
    #[error("Task with ID {0} not found")]
    TaskNotFound(Uuid),

    /// Invalid parameter in task
    #[error("Invalid parameter '{0}': {1}")]
    InvalidParameter(String, String),

    /// LLM communication error
    #[error("LLM communication error: {0}")]
    LlmCommunication(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Other errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type for the MMSS system
pub type Result<T> = std::result::Result<T, Error>;
