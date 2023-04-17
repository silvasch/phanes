#[derive(Debug, thiserror::Error)]
pub enum PhanesError {
    #[error("unknown error: {0}")]
    UnknownError(String),
}
