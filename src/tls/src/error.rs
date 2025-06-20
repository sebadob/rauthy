use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),
    #[error("InvalidData: {0}")]
    InvalidData(String),
    #[error("IO: {0}")]
    IO(String),
    #[error("KeyMismatch: {0}")]
    KeyMismatch(String),
    #[error("Notify: {0}")]
    Notify(String),
    #[error("NotFound: {0}")]
    NotFound(&'static str),
    #[error("PrivateKey: {0}")]
    PrivateKey(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<rustls::Error> for Error {
    fn from(err: rustls::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Self {
        Self::Notify(err.to_string())
    }
}
