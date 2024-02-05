mod test_client;
use std::ops::Add;

use test_client::pb::product_service_client::ProductServiceClient;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

async fn get_client() -> ProductServiceClient<Channel> {
    ProductServiceClient::connect("http://localhost:50051")
        .await
        .unwrap()
}

#[tokio::test]
async fn reserve_ticket() {
    let mut client = get_client().await;
    let res = client
        .get_ticket_types(test_client::pb::GetTicketTypesRequest {})
        .await
        .unwrap();
    let res = res.into_inner();
    assert_eq!(res.ticket_types.len(), 4);

    let res = client
        .get_ticket_durations(test_client::pb::GetTicketDurationsRequest {
            ticket_type_id: "chalet3".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(res.ticket_durations.len(), 2);
    assert!(res.ticket_durations.contains(&3),);
    assert!(res.ticket_durations.contains(&4),);

    let res = client
        .add_ticket_to_basket(test_client::pb::AddTicketToBasketRequest {
            ticket_type_id: "chalet3".to_string(),
            duration: 3,
        })
        .await
        .unwrap()
        .into_inner();

    let ticket = res.order.unwrap();

    assert_eq!(ticket.ticket_type_id, "chalet3".to_string());

    let expected_reservation_time = chrono::Utc::now()
        .add(chrono::Duration::minutes(9))
        .naive_utc();

    println!("{}", ticket.reserved_until);

    let ticket_reserved_time = chrono::NaiveDateTime::parse_from_str(
        &ticket.reserved_until,
        festival_tickets::CHRONO_TIMESTAMP_FMT,
    )
    .unwrap();

    println!("{:#?}", ticket);

    assert!(
        ticket_reserved_time >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );
}

#[tokio::test]
async fn purchase_ticket() {
    let mut client = get_client().await;

    let res = client
        .add_ticket_to_basket(test_client::pb::AddTicketToBasketRequest {
            ticket_type_id: "chalet3".to_string(),
            duration: 3,
        })
        .await
        .unwrap()
        .into_inner();

    let order = res.order.unwrap();

    assert_eq!(order.ticket_type_id, "chalet3".to_string());

    let expected_reservation_time = chrono::Utc::now()
        .add(chrono::Duration::minutes(9))
        .naive_utc();

    let ticket_reserved_time = chrono::NaiveDateTime::parse_from_str(
        &order.reserved_until,
        festival_tickets::CHRONO_TIMESTAMP_FMT,
    )
    .unwrap();

    println!("{:#?}", order);

    assert!(
        ticket_reserved_time >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );

    assert!(order.purchased_at.is_none());

    let res = client
        .purchase_order(test_client::pb::PurchaseOrderRequest { id: order.id })
        .await
        .unwrap()
        .into_inner();

    assert!(res.order.unwrap().purchased_at.is_some());
}

#[tokio::test]
async fn stream_order_stats() {
    let mut client = get_client().await;

    let stream = client
        .get_order_stats(test_client::pb::GetOrderStatsRequest {})
        .await
        .unwrap()
        .into_inner();

    let mut stream = stream.take(6);
    while let Some(item) = stream.next().await {
        println!("\treceived: {:#?}", item.unwrap());
    }
}
