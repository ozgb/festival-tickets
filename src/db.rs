use super::pb;

// Define database types

struct TicketType {
    id: String,
    display: String,
}

pub fn get_ticket_types() -> Vec<pb::TicketType> {
    // TODO: Sqlx select

    vec![
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
    ]
}

pub fn get_ticket_durations(_type: Option<pb::TicketType>) -> Vec<String> {
    // TODO: Sqlx select
    vec!["3 days".into(), "4 days".into()]
}
