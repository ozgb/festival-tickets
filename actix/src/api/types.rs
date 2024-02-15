use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::db::error::DbError;

#[derive(Error, Debug, Serialize, ToSchema)]
pub enum ApiError {
    #[error("database execution error: {0}")]
    DbExecutionError(String),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
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

#[derive(Serialize)]
pub struct OrderStats {
    pub duration_days: i32,
    pub order_limit: i32,
    pub order_count: i32,
}

#[derive(Serialize, ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub ticket_type_id: String,
    pub user_id: Option<String>,
    pub duration: i32,
    pub price: f32,
    pub reserved_until: chrono::NaiveDateTime,
    pub purchased_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub email: String,
    pub order_id: Uuid,
}

#[derive(Serialize)]
pub struct TicketType {
    pub id: Uuid,
    pub display: String,
    pub sold_out: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct AddTicketToBasketRequest {
    pub ticket_type_id: String,
    /// Duration in days
    pub duration: i32,
}

#[derive(Serialize)]
pub struct AddTicketToBasketResponse {
    pub order: Order,
}

#[derive(Serialize)]
pub struct GetTicketTypesResponse {
    pub ticket_types: Vec<TicketType>,
}

#[derive(Deserialize)]
pub struct AddUserInfoRequest {
    pub name: String,
    pub email: String,
    pub address: String,
}
