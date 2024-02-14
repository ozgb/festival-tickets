use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
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

impl actix_web::error::ResponseError for DbError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            DbError::ExecutionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::FailedPrecondition(_) => StatusCode::BAD_REQUEST,
            DbError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
