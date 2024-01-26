mod test_client;
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

    assert_eq!(
        res.ticket_durations,
        vec!["3 days".to_string(), "4 days".to_string()]
    );
}
