use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum DbError {
    #[error("execution error")]
    ExecutionError(#[from] sqlx::Error),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    #[error("unknown service error")]
    Unknown,
}
