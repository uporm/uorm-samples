
use serde::Serialize;
use thiserror::Error;
use uorm::error::DbError;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct BizError {
    pub code: i32,
    pub args: Vec<(String, String)>,
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("{0}")]
    DbError(#[from] DbError),
    #[error("{0}")]
    Val(#[from] ValidationErrors),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("System error: {0}")]
    System(String),
    #[error("{0:?}")]
    Error(BizError),
}

pub type Result<T> = std::result::Result<T, WebError>;
