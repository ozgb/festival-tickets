use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("execution error")]
    ExecutionError(#[from] sqlx::Error),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    #[error("unknown service error")]
    Unknown,
}
