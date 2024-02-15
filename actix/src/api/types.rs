use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct OrderStats {
    pub duration_days: i32,
    pub order_limit: i32,
    pub order_count: i32,
}

#[derive(Serialize)]
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

#[derive(Deserialize)]
pub struct AddTicketToBasketRequest {
    pub ticket_type_id: String,
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
