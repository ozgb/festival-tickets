use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type WebResult<T> = actix_web::Result<T>;

pub mod db;
pub mod env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[derive(Serialize)]
pub struct OrderStats {
    duration_days: i32,
    order_limit: i32,
    order_count: i32,
}

#[derive(Serialize)]
pub struct Order {
    id: Uuid,
    ticket_type_id: String,
    user_id: Option<String>,
    duration: i32,
    price: f32,
    reserved_until: chrono::NaiveDateTime,
    purchased_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    name: String,
    address: String,
    email: String,
    order_id: Uuid,
}

#[derive(Serialize)]
pub struct TicketType {
    id: Uuid,
    display: String,
    sold_out: bool,
}

#[derive(Deserialize)]
struct AddTicketToBasketRequest {
    ticket_type_id: String,
    duration: i32,
}

#[derive(Serialize)]
struct AddTicketToBasketResponse {
    order: Order,
}

#[post("/tickets/add-to-basket")]
async fn echo(
    pool: web::Data<db::DbPool>,
    req: web::Json<AddTicketToBasketRequest>,
) -> WebResult<impl Responder> {
    let res = db::add_ticket_to_basket(&pool, &req.ticket_type_id, req.duration).await?;
    Ok(web::Json(res))
}

#[derive(Serialize)]
struct GetTicketTypesResponse {
    ticket_types: Vec<TicketType>,
}

#[get("/tickets/types")]
async fn get_ticket_types(pool: web::Data<db::DbPool>) -> WebResult<impl Responder> {
    let res = db::get_ticket_types(&pool).await?;
    Ok(web::Json(res))
}

#[get("/tickets/durations/{ticket_type_id}")]
async fn get_ticket_durations(
    pool: web::Data<db::DbPool>,
    ticket_type_id: web::Path<String>,
) -> WebResult<impl Responder> {
    let res = db::get_ticket_durations(&pool, &ticket_type_id).await?;
    Ok(web::Json(res))
}

#[post("/orders/{order_id}/purchase")]
async fn purchase_order(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::purchase_order(&pool, &order_id).await?;
    Ok(web::Json(res))
}

#[get("/orders/{order_id}")]
async fn get_order(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::get_order(&pool, &order_id).await?;
    Ok(web::Json(res))
}

#[get("/users/{user_id}")]
async fn get_user(
    pool: web::Data<db::DbPool>,
    user_id: web::Path<Uuid>,
) -> WebResult<impl Responder> {
    let res = db::get_order(&pool, &user_id).await?;
    Ok(web::Json(res))
}

#[derive(Deserialize)]
pub struct AddUserInfoRequest {
    name: String,
    email: String,
    address: String,
}

#[post("/orders/{order_id}/add-user-info")]
async fn add_user_info(
    pool: web::Data<db::DbPool>,
    order_id: web::Path<Uuid>,
    body: web::Json<AddUserInfoRequest>,
) -> WebResult<impl Responder> {
    let res = db::add_user_to_order(&pool, &order_id, &body).await?;
    Ok(web::Json(res))
}

#[get("/orders/stats")]
async fn stream_order_stats() -> impl Responder {
    // TODO: Complete this endpoint
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging, level INFO
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();
    let addr = ("0.0.0.0", 50051);

    log::info!("connecting to db...");
    let pool = db::connect_to_pool().await;
    // Run database migrations
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("failed to apply database migrations");

    println!("serving on {}:{}", addr.0, addr.1);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
