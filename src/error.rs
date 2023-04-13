#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not create adapter")]
    AdapterCreationFailed,
    #[error("unknown error: {0}")]
    UnknownError(String),
}
