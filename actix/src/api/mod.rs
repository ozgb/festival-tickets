use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::db;
pub mod types;

use types::{AddTicketToBasketRequest, AddUserInfoRequest, ApiError};

type WebResult<T> = actix_web::Result<T>;

pub(super) fn configure(pool: web::Data<db::DbPool>) -> impl FnOnce(&mut web::ServiceConfig) {
    |config: &mut web::ServiceConfig| {
        config
            .app_data(pool)
            .service(add_ticket_to_basket)
            .service(get_ticket_types)
            .service(get_ticket_durations)
            .service(purchase_order)
            .service(get_order)
            .service(get_user)
            .service(add_user_info)
            .service(stream_order_stats);
    }
}

/// Add Ticket of type and duration in days to basket
#[utoipa::path(
    responses(
        (
            status = 200,
            description = "Ticket successfully added to basket and reserved for 10 mins",
            body = Order
        ),
        (
            status = 400,
            description = "Ticket type/duration pair sold-out",
            body = ApiError,
            example = json!(ApiError::FailedPrecondition(String::from("ticket chalet3/3 sold out")))
        )
    )
)]
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
