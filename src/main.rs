use festival_tickets::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = Service::default();

    println!("server listening on {}", addr);

    Server::builder()
        .add_service(service.into_service())
        .serve(addr)
        .await?;

    Ok(())
}
