use std::ops::Add;

use festival_tickets_client::types::{AddTicketToBasketRequest, AddUserInfoRequest};

#[actix_web::test]
async fn reserve_ticket() {
    let client = festival_tickets_client::Client::new("http://localhost:50051");

    let res = client.get_ticket_types().await.unwrap().into_inner();
    assert_eq!(res.len(), 4);

    let res = client
        .get_ticket_durations("chalet3")
        .await
        .unwrap()
        .into_inner();
    assert_eq!(res.len(), 2);
    assert!(res.contains(&3),);
    assert!(res.contains(&4),);

    let order = client
        .add_ticket_to_basket(&AddTicketToBasketRequest {
            ticket_type_id: "chalet3".to_owned(),
            duration: 3,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(order.ticket_type_id, "chalet3".to_string());

    let expected_reservation_time = chrono::Utc::now().add(chrono::Duration::minutes(9));

    println!("{}", order.reserved_until);

    assert!(
        order.reserved_until >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );
}

#[actix_web::test]
async fn purchase_ticket() {
    let client = festival_tickets_client::Client::new("http://localhost:50051");

    let order = client
        .add_ticket_to_basket(&AddTicketToBasketRequest {
            ticket_type_id: "chalet3".to_owned(),
            duration: 3,
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(order.ticket_type_id, "chalet3".to_string());

    let expected_reservation_time = chrono::Utc::now().add(chrono::Duration::minutes(9));

    println!("{:#?}", order);

    assert!(
        order.reserved_until >= expected_reservation_time,
        "Check ticket is reserved for at least 9 minutes"
    );

    assert!(order.purchased_at.is_none());

    let res = client.purchase_order(&order.id).await;

    // Assert that attempting to purchase order before adding user info fails pre-condition
    match res {
        Err(e) => match e {
            festival_tickets_client::Error::ErrorResponse(e) => match e.into_inner() {
                festival_tickets_client::types::ApiError::FailedPrecondition(p) => {
                    println!("error: {}", p);
                }
                _ => assert!(false, "expected res to be pre-condition err!"),
            },
            _ => assert!(false, "expected res to be ApiError err!"),
        },
        _ => assert!(false, "expected res to be err!"),
    }

    // Add user to order
    let order = client
        .add_user_info(
            &order.id,
            &AddUserInfoRequest {
                address: "22 Oscar St, Dorset, UK".to_string(),
                email: "oscar@oscar.com".to_string(),
                name: "Oscar".to_string(),
            },
        )
        .await
        .unwrap()
        .into_inner();

    assert!(order.user_id.is_some());

    // Get order
    let order = client.get_order(&order.id).await.unwrap().into_inner();
    assert!(order.user_id.is_some());

    // Purchase order
    let order = client.purchase_order(&order.id).await.unwrap().into_inner();
    assert!(order.purchased_at.is_some());
}
