use thiserror::Error;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Error)]
pub enum ClientError {
    // internal errors
    #[error("there was an error while building an http request")]
    HttpError(#[from] http::Error),
    #[error("isahc error of kind {0}")]
    IsahcError(isahc::error::ErrorKind),
    #[error("error while trying to (de-)serialize data with serde_json")]
    SerdeJsonError(#[from] serde_json::Error),
    // x0 response errors
    #[error("x0 returned an unexpected status code")]
    UnexpectedStatus,
}

impl From<isahc::Error> for ClientError {
    fn from(error: isahc::Error) -> Self {
        ClientError::IsahcError(error.kind().clone())
    }
}
