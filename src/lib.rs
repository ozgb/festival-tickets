use sqlx::types::Uuid;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use pb::product_service_server::{ProductService, ProductServiceServer};
use pb::{
    AddTicketToBasketRequest, AddTicketToBasketResponse, GetOrderRequest, GetOrderResponse,
    GetTicketDurationsRequest, GetTicketDurationsResponse, GetTicketTypesRequest,
    GetTicketTypesResponse, PurchaseOrderRequest, PurchaseOrderResponse,
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
        let order = db::add_ticket_to_basket(&self.dbpool, &req.ticket_type_id, req.duration)
            .await
            .map_err(|_e| Status::new(tonic::Code::Internal, "failed"))?;

        Ok(Response::new(pb::AddTicketToBasketResponse {
            order: Some(order),
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
            ticket_durations: db::get_ticket_durations(&self.dbpool, &req.ticket_type_id)
                .await
                .map_err(|_e| Status::new(tonic::Code::Internal, "failed"))?,
        };
        Ok(Response::new(reply))
    }

    async fn purchase_order(
        &self,
        request: Request<PurchaseOrderRequest>,
    ) -> Result<Response<PurchaseOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = Uuid::parse_str(&req.id).map_err(|e| {
            Status::new(
                tonic::Code::InvalidArgument,
                format!(
                    "failed to parse order id as uuid: {}, {}",
                    &req.id,
                    e.to_string()
                ),
            )
        })?;

        let order = db::purchase_order(&self.dbpool, &order_id)
            .await
            .map_err(|_e| Status::new(tonic::Code::Internal, "failed"))?;

        Ok(Response::new(pb::PurchaseOrderResponse {
            order: Some(order),
        }))
    }

    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = Uuid::parse_str(&req.id).map_err(|e| {
            Status::new(
                tonic::Code::InvalidArgument,
                format!(
                    "failed to parse order id as uuid: {}, {}",
                    &req.id,
                    e.to_string()
                ),
            )
        })?;

        let order = db::get_order(&self.dbpool, &order_id)
            .await
            .map_err(|_e| Status::new(tonic::Code::Internal, "failed"))?;

        Ok(Response::new(pb::GetOrderResponse { order: Some(order) }))
    }
}
