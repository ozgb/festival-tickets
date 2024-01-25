use std::sync::Arc;

use festival_tickets::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let pool = festival_tickets::db::connect_to_pool().await;
    let service = Service::new(Arc::new(pool));

    println!("server listening on {}", addr);

    Server::builder()
        .add_service(service.into_service())
        .serve(addr)
        .await?;

    Ok(())
}
