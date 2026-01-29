use hyper::StatusCode;
use thiserror::Error;

/// An error returned by a smarty api.
#[derive(Debug, Error)]
pub enum SmartyError {
    #[error("failed to process request")]
    RequestProcess(#[from] reqwest::Error),
    #[error("request middleware failed")]
    Middleware(#[from] anyhow::Error),
    #[error("failed to parse url")]
    Parse(#[from] url::ParseError),
    #[error("http error")]
    HttpError { code: StatusCode, detail: String },
    #[error("validation error: {0}")]
    ValidationError(String),
}
