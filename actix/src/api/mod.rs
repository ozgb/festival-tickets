use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::db;

type WebResult<T> = actix_web::Result<T>;

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
struct AddTicketToBasketRequest {
    pub ticket_type_id: String,
    pub duration: i32,
}

#[derive(Serialize)]
struct AddTicketToBasketResponse {
    pub order: Order,
}

#[derive(Serialize)]
struct GetTicketTypesResponse {
    pub ticket_types: Vec<TicketType>,
}

#[derive(Deserialize)]
pub struct AddUserInfoRequest {
    pub name: String,
    pub email: String,
    pub address: String,
}

#[post("/tickets/add-to-basket")]
pub async fn add_ticket_to_basket(
    pool: web::Data<db::DbPool>,
    req: web::Json<AddTicketToBasketRequest>,
) -> WebResult<impl Responder> {
    let res = db::add_ticket_to_basket(&pool, &req.ticket_type_id, req.duration).await?;
    Ok(web::Json(res))
}

#[get("/tickets/types")]
pub async fn get_ticket_types(pool: web::Data<db::DbPool>) -> WebResult<impl Responder> {
    let res = db::get_ticket_types(&pool).await?;
    Ok(web::Json(res))
}

#[get("/tickets/durations/{ticket_type_id}")]
pub async fn get_ticket_durations(
    pool: web::Data<db::DbPool>,
    ticket_type_id: web::Path<String>,
) -> WebResult<impl Responder> {
    let res = db::get_ticket_durations(&pool, &ticket_type_id).await?;
    Ok(web::Json(res))
}

#[post("/orders/{order_id}/purchase")]
pub async fn purchase_order(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::purchase_order(&pool, &order_id).await?;
    Ok(web::Json(res))
}

#[get("/orders/{order_id}")]
pub async fn get_order(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::get_order(&pool, &order_id).await?;
    Ok(web::Json(res))
}

#[get("/users/{user_id}")]
pub async fn get_user(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::get_order(&pool, &user_id).await?;
    Ok(web::Json(res))
}

#[post("/orders/{order_id}/add-user-info")]
pub async fn add_user_info(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
    body: web::Json<AddUserInfoRequest>,
) -> WebResult<impl Responder> {
    let res = db::add_user_to_order(&pool, &order_id, &body).await?;
    Ok(web::Json(res))
}

#[get("/orders/stats")]
pub async fn stream_order_stats() -> impl Responder {
    // TODO: Complete this endpoint
    HttpResponse::Ok().body("Hello world!")
}
