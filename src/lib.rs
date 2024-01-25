use std::sync::Arc;
use tonic::{Request, Response, Status};

use pb::product_service_server::{ProductService, ProductServiceServer};
use pb::{
    AddTicketToBasketRequest, AddTicketToBasketResponse, GetTicketDurationsRequest,
    GetTicketDurationsResponse, GetTicketTypesRequest, GetTicketTypesResponse,
};

pub mod db;
mod env;

pub mod pb {
    tonic::include_proto!("purchase");
}

pub struct Service {
    dbpool: Arc<db::DbPool>,
}

impl Service {
    pub fn new(dbpool: Arc<db::DbPool>) -> Self {
        Self { dbpool }
    }

    pub fn into_service(self) -> ProductServiceServer<Service> {
        ProductServiceServer::new(self)
    }
}

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
            ticket_types: db::get_ticket_types(&self.dbpool)
                .await
                .map_err(|_e| Status::new(tonic::Code::Internal, "failed"))?,
        };
        Ok(Response::new(reply))
    }

    async fn get_ticket_durations(
        &self,
        request: Request<GetTicketDurationsRequest>,
    ) -> Result<Response<GetTicketDurationsResponse>, Status> {
        let req = request.into_inner();
        let reply = pb::GetTicketDurationsResponse {
            ticket_durations: db::get_ticket_durations(req.ticket_type),
        };
        Ok(Response::new(reply))
    }
}
