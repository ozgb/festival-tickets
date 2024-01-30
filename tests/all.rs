mod test_client;
use std::ops::Add;

use test_client::pb::product_service_client::ProductServiceClient;
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

    let ticket = res.ticket.unwrap();

    assert_eq!(
        ticket.r#type.as_ref().unwrap().display,
        "Chalet, 3 people".to_string()
    );

    let expected_reservation_time = chrono::Utc::now()
        .add(chrono::Duration::minutes(9))
        .naive_utc();

    let ticket_reserved_time =
        chrono::NaiveDateTime::from_timestamp_opt(ticket.reserved_until as i64, 0).unwrap();

    println!("{:#?}", ticket);

    assert!(
        ticket_reserved_time >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );
}
