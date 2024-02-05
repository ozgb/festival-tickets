use async_stream::stream;
use db::DbPool;
use futures::{future::FutureExt, pin_mut};
use sqlx::types::Uuid;
use std::sync::Arc;
use tokio::select;
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::{wrappers::BroadcastStream, Stream};
use tonic::{Request, Response, Status};

use pb::product_service_server::{ProductService, ProductServiceServer};
use pb::{
    AddTicketToBasketRequest, AddTicketToBasketResponse, GetOrderRequest, GetOrderResponse,
    GetOrderStatsRequest, GetTicketDurationsRequest, GetTicketDurationsResponse,
    GetTicketTypesRequest, GetTicketTypesResponse, OrderStats, PurchaseOrderRequest,
    PurchaseOrderResponse,
};

pub mod db;
mod env;
pub mod error;

use error::ServiceError;

type ServiceResult<T> = tonic::Result<tonic::Response<T>>;

pub mod pb {
    tonic::include_proto!("purchase");
}

struct OrderStatsSubMsg {
    resp: tokio::sync::oneshot::Sender<tokio::sync::broadcast::Receiver<pb::OrderStats>>,
}

pub struct Service {
    dbpool: Arc<DbPool>,
    order_stats_sub: tokio::sync::mpsc::Sender<OrderStatsSubMsg>,
}

impl Service {
    pub fn new(dbpool: Arc<db::DbPool>) -> Self {
        // Setup broadcast channel for order stats updates
        let (tx, _rx) = broadcast::channel::<pb::OrderStats>(16);
        let (order_stats_sub_tx, order_stats_sub_rx) = tokio::sync::mpsc::channel(32);

        let _order_stats_handle = tokio::spawn(Self::send_order_stats(
            dbpool.clone(),
            tx,
            order_stats_sub_rx,
        ));

        tokio::spawn(Self::remove_old_orders(dbpool.clone()));

        Self {
            dbpool,
            order_stats_sub: order_stats_sub_tx,
        }
    }

    pub fn into_service(self) -> ProductServiceServer<Service> {
        ProductServiceServer::new(self)
    }

    async fn remove_old_orders(pool: Arc<DbPool>) {
        loop {
            match db::remove_expired_orders(&pool).await {
                Ok(_) => (),
                Err(e) => log::error!("error removing old orders: {}", e),
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn send_order_stats(
        pool: Arc<DbPool>,
        tx: tokio::sync::broadcast::Sender<pb::OrderStats>,
        mut order_stats_sub_rx: tokio::sync::mpsc::Receiver<OrderStatsSubMsg>,
    ) {
        loop {
            let sleep_future = sleep(Duration::from_millis(500)).fuse();
            pin_mut!(sleep_future);

            // Wait for 500ms sleep, interrupt to respond to subscribe requests
            loop {
                select! {
                    () = &mut sleep_future => break,
                    msg = order_stats_sub_rx.recv() => {
                        if let Some(v) = msg {
                            let _ = v.resp.send(tx.subscribe())
                                .map_err(|e| log::error!("error responding to order stats sub: {:?}", e));
                        }
                    }
                }
            }

            if tx.receiver_count() > 0 {
                db::get_order_stats(&pool)
                    .await
                    .unwrap_or(vec![])
                    .into_iter()
                    .for_each(|s| {
                        // Ignore errors - this fails if there are no order stats listeners
                        let _ = tx.send(s);
                    })
            }
        }
    }
}

#[tonic::async_trait]
impl ProductService for Service {
    async fn add_ticket_to_basket(
        &self,
        request: Request<AddTicketToBasketRequest>,
    ) -> ServiceResult<AddTicketToBasketResponse> {
        let req = request.into_inner();
        let order = db::add_ticket_to_basket(&self.dbpool, &req.ticket_type_id, req.duration)
            .await
            .map_err(|e| {
                log::error!("{:#?}", e);
                ServiceError::DatabaseError
            })?;

        Ok(Response::new(pb::AddTicketToBasketResponse {
            order: Some(order),
        }))
    }

    async fn get_ticket_types(
        &self,
        _request: Request<GetTicketTypesRequest>,
    ) -> ServiceResult<GetTicketTypesResponse> {
        let reply = pb::GetTicketTypesResponse {
            ticket_types: db::get_ticket_types(&self.dbpool).await.map_err(|e| {
                log::error!("{:#?}", e);
                ServiceError::DatabaseError
            })?,
        };
        Ok(Response::new(reply))
    }

    async fn get_ticket_durations(
        &self,
        request: Request<GetTicketDurationsRequest>,
    ) -> ServiceResult<GetTicketDurationsResponse> {
        let req = request.into_inner();
        let reply = pb::GetTicketDurationsResponse {
            ticket_durations: db::get_ticket_durations(&self.dbpool, &req.ticket_type_id)
                .await
                .map_err(|e| {
                    log::error!("{:#?}", e);
                    ServiceError::DatabaseError
                })?,
        };
        Ok(Response::new(reply))
    }

    async fn purchase_order(
        &self,
        request: Request<PurchaseOrderRequest>,
    ) -> ServiceResult<PurchaseOrderResponse> {
        let req = request.into_inner();
        let order_id = Uuid::parse_str(&req.id)
            .map_err(|e| ServiceError::ParseError(format!("uuid ({})", e)))?;

        let order = db::purchase_order(&self.dbpool, &order_id)
            .await
            .map_err(|e| {
                log::error!("{:#?}", e);
                ServiceError::DatabaseError
            })?;

        Ok(Response::new(pb::PurchaseOrderResponse {
            order: Some(order),
        }))
    }

    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> ServiceResult<GetOrderResponse> {
        let req = request.into_inner();
        let order_id = Uuid::parse_str(&req.id)
            .map_err(|e| ServiceError::ParseError(format!("uuid ({})", e)))?;

        let order = db::get_order(&self.dbpool, &order_id).await.map_err(|e| {
            log::error!("{:#?}", e);
            ServiceError::DatabaseError
        })?;

        Ok(Response::new(pb::GetOrderResponse { order: Some(order) }))
    }

    type GetOrderStatsStream =
        std::pin::Pin<Box<dyn Stream<Item = Result<OrderStats, Status>> + Send>>;

    async fn get_order_stats(
        &self,
        _request: Request<GetOrderStatsRequest>,
    ) -> ServiceResult<Self::GetOrderStatsStream> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.order_stats_sub
            .clone()
            .send(OrderStatsSubMsg { resp: tx })
            .await
            .map_err(|e| {
                log::error!("failed to request new order stats stream: {}", e);
                ServiceError::StreamStartError
            })?;
        let stream = rx.await.map_err(|e| {
            log::error!("failed to receive new order stats stream: {}", e);
            ServiceError::StreamStartError
        })?;

        let stream = BroadcastStream::new(stream);
        return Ok(Response::new(
            Box::pin(map_broadcast_err_to_status(stream)) as Self::GetOrderStatsStream
        ));
    }
}

fn map_broadcast_err_to_status<S: Stream<Item = Result<OrderStats, BroadcastStreamRecvError>>>(
    input: S,
) -> impl Stream<Item = Result<OrderStats, Status>> {
    stream! {
        for await value in input {
            yield value.map_err(|e| {
                log::error!("failed to yield new stream item: {}", e);
                ServiceError::StreamError.into()
            })
        }
    }
}
