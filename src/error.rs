use thiserror::Error;
use tonic::Code;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("stream start error")]
    StreamStartError,
    #[error("stream error")]
    StreamError,
    #[error("database error")]
    DatabaseError,
    #[error("unknown service error")]
    Unknown,
}

impl<'a> From<&'a ServiceError> for Code {
    fn from(value: &'a ServiceError) -> Self {
        match value {
            ServiceError::ParseError(_) => Code::InvalidArgument,
            ServiceError::StreamStartError => Code::Internal,
            ServiceError::StreamError => Code::Internal,
            ServiceError::DatabaseError => Code::Internal,
            ServiceError::Unknown => Code::Unknown,
        }
    }
}

impl From<ServiceError> for tonic::Status {
    fn from(value: ServiceError) -> Self {
        tonic::Status::new((&value).into(), value.to_string())
    }
}
