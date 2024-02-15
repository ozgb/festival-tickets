use actix_web::{middleware::Logger, web, App, HttpServer};

pub mod api;
pub mod db;
pub mod env;

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
            .service(api::add_ticket_to_basket)
            .service(api::get_ticket_types)
            .service(api::get_ticket_durations)
            .service(api::purchase_order)
            .service(api::get_order)
            .service(api::get_user)
            .service(api::add_user_info)
            .service(api::stream_order_stats)
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
