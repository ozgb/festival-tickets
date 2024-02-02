use std::sync::Arc;

use festival_tickets::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging, level INFO
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();

    let addr = "0.0.0.0:50051".parse().unwrap();
    log::info!("connecting to db...");
    let pool = festival_tickets::db::connect_to_pool().await;
    // Run database migrations
    sqlx::migrate!().run(&pool).await?;

    let service = Service::new(Arc::new(pool)).into_service();

    log::info!("server listening on {}", addr);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
