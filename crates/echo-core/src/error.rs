use thiserror::Error;

#[derive(Debug, Error)]
pub enum EchoError {
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("Presentation error: {0}")]
    Presentation(String),
    #[error("Output Look not found: {0}")]
    LookNotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
