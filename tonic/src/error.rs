use thiserror::Error;
use tonic::Code;

use crate::db::error::DbError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("stream start error")]
    StreamStartError,
    #[error("stream error")]
    StreamError,
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    #[error("unknown service error")]
    Unknown,
}

impl From<DbError> for ServiceError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::Unknown => ServiceError::Unknown,
            DbError::ExecutionError(e) => ServiceError::DatabaseError(e),
            DbError::FailedPrecondition(e) => ServiceError::FailedPrecondition(e),
        }
    }
}

impl<'a> From<&'a ServiceError> for Code {
    fn from(value: &'a ServiceError) -> Self {
        match value {
            ServiceError::ParseError(_) => Code::InvalidArgument,
            ServiceError::StreamStartError => Code::Internal,
            ServiceError::StreamError => Code::Internal,
            ServiceError::DatabaseError(_e) => Code::Internal,
            ServiceError::FailedPrecondition(_s) => Code::FailedPrecondition,
            ServiceError::Unknown => Code::Unknown,
        }
    }
}

impl From<ServiceError> for tonic::Status {
    fn from(value: ServiceError) -> Self {
        tonic::Status::new((&value).into(), value.to_string())
    }
}
