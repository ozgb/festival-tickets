use super::env;
use super::pb;
use futures::TryStreamExt;
use sqlx::postgres::Postgres;
use sqlx::Row;

pub type DbPool = sqlx::Pool<Postgres>;

pub async fn connect_to_pool() -> DbPool {
    let db_url = env::Cfg::DatabaseUrl.load().expect("Failed to load db url");
    let pool = DbPool::connect(&db_url)
        .await
        .expect("Failed to connect to pool");
    pool
}

pub async fn get_ticket_types(pool: &DbPool) -> Result<Vec<pb::TicketType>, sqlx::Error> {
    let mut rows = sqlx::query("SELECT * FROM ticket_types").fetch(pool);

    let mut ticket_types = Vec::new();
    while let Some(row) = rows.try_next().await? {
        ticket_types.push(pb::TicketType {
            id: row.try_get::<String, _>("id")?,
            display: row.try_get::<String, _>("display")?,
            sold_out: false,
        });
    }

    Ok(ticket_types)
}

pub fn get_ticket_durations(_type_id: &str) -> Vec<String> {
    // TODO: Sqlx select
    vec!["3 days".into(), "4 days".into()]
}
