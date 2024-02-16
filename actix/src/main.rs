use actix_web::{middleware::Logger, web, App, HttpServer};

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub mod api;
pub mod db;
pub mod env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup Api

    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::add_ticket_to_basket,
            api::get_ticket_types,
            api::get_ticket_durations,
            api::purchase_order,
            api::get_order,
            api::get_user,
            api::add_user_info,
        ),
        components(
            schemas(
                api::types::Order,
                api::error::ApiError,
                api::types::AddTicketToBasketRequest,
                api::types::TicketType,
                api::types::User,
                api::types::AddUserInfoRequest,
            )
        ),
        tags(
            (name = "festival-tickets", description = "Purchase festival tickets")
        ),
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

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
            .configure(api::configure(web::Data::new(pool.clone())))
            // Setup OpenAPI routes.
            // See: https://github.com/juhaku/utoipa/blob/master/examples/todo-actix/src/main.rs
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
