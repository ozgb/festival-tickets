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

    let expected_reservation_time = chrono::Utc::now().add(chrono::Duration::minutes(9));

    println!("{}", ticket.reserved_until);

    let ticket_reserved_time =
        chrono::DateTime::parse_from_rfc3339(&ticket.reserved_until).unwrap();

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

    let expected_reservation_time = chrono::Utc::now().add(chrono::Duration::minutes(9));

    let ticket_reserved_time = chrono::DateTime::parse_from_rfc3339(&order.reserved_until).unwrap();

    println!("{:#?}", order);

    assert!(
        ticket_reserved_time >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );

    assert!(order.purchased_at.is_none());

    let res = client
        .purchase_order(test_client::pb::PurchaseOrderRequest {
            id: order.id.clone(),
        })
        .await;

    // Assert that attempting to purchase order before adding user info fails pre-condition
    assert!(res.is_err());

    // Add user to order
    let res = client
        .add_user_info(test_client::pb::AddUserInfoRequest {
            user_name: "Oscar".to_string(),
            user_email: "oscar@oscar.com".to_string(),
            user_address: "22 Oscar St, Dorset, UK".to_string(),
            order_id: order.id.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    assert!(res.order.unwrap().user_id.is_some());

    // Get order
    let res = client
        .get_order(test_client::pb::GetOrderRequest {
            id: order.id.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    assert!(res.order.unwrap().user_id.is_some());

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
