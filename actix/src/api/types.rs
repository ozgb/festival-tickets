use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    pub reserved_until: chrono::DateTime<chrono::Utc>,
    pub purchased_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub email: String,
    pub order_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct TicketType {
    pub id: String,
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

#[derive(Deserialize, ToSchema)]
pub struct AddUserInfoRequest {
    pub name: String,
    pub email: String,
    pub address: String,
}
