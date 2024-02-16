use crate::db::error::DbError;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, Serialize, ToSchema)]
pub enum ApiError {
    #[error("database execution error: {0}")]
    DbExecutionError(String),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unknown service error")]
    Unknown,
}

impl From<DbError> for ApiError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::FailedPrecondition(e) => Self::FailedPrecondition(e),
            DbError::ExecutionError(e) => Self::DbExecutionError(e.to_string()),
            DbError::Unknown => Self::Unknown,
        }
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::DbExecutionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::FailedPrecondition(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
