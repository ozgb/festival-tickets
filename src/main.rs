use tonic::{transport::Server, Request, Response, Status};

use pb::product_service_server::{ProductService, ProductServiceServer};
use pb::{
    AddTicketToBasketRequest, AddTicketToBasketResponse, GetTicketDurationsRequest,
    GetTicketDurationsResponse, GetTicketTypesRequest, GetTicketTypesResponse,
};

pub mod pb {
    tonic::include_proto!("purchase");
}

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl ProductService for Service {
    async fn add_ticket_to_basket(
        &self,
        request: Request<AddTicketToBasketRequest>,
    ) -> Result<Response<AddTicketToBasketResponse>, Status> {
        let req = request.into_inner();
        Ok(Response::new(pb::AddTicketToBasketResponse {
            ticket: Some(pb::Ticket {
                r#type: req.r#type,
                duration: req.duration,
                price: 5f32,
                reserved_until: 23492304234,
            }),
        }))
    }

    async fn get_ticket_types(
        &self,
        _request: Request<GetTicketTypesRequest>,
    ) -> Result<Response<GetTicketTypesResponse>, Status> {
        let reply = pb::GetTicketTypesResponse {
            ticket_types: vec![
                pb::TicketType {
                    id: "chalet3".into(),
                    display: "Chalet, 3 People".into(),
                    sold_out: false,
                },
                pb::TicketType {
                    id: "chalet4".into(),
                    display: "Chalet, 4 People".into(),
                    sold_out: false,
                },
            ],
        };
        Ok(Response::new(reply))
    }

    async fn get_ticket_durations(
        &self,
        _request: Request<GetTicketDurationsRequest>,
    ) -> Result<Response<GetTicketDurationsResponse>, Status> {
        let reply = pb::GetTicketDurationsResponse {
            ticket_durations: vec!["3 days".into(), "4 days".into()],
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = Service::default();

    println!("server listening on {}", addr);

    Server::builder()
        .add_service(ProductServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
