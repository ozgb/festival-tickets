use std::ops::Add;

use super::env;
use super::pb;
use chrono;
use futures::TryStreamExt;
use sqlx::postgres::Postgres;
use sqlx::types::Uuid;
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

pub async fn get_ticket_durations(pool: &DbPool, _type_id: &str) -> Result<Vec<u32>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM order_stats")
        .fetch_all(pool)
        .await?;

    let mut durations = vec![];
    for row in rows {
        if row.order_limit > row.order_count.unwrap_or(0) {
            durations.push(row.duration_days as u32);
        }
    }

    Ok(durations)
}

pub async fn add_ticket_to_basket(
    pool: &DbPool,
    type_id: &str,
    duration: u32,
) -> Result<pb::Order, sqlx::Error> {
    let record = sqlx::query!(
        r#"
WITH ord as (
        INSERT INTO orders (ticket_type, reserved_until, duration_days)
        VALUES ( $1, $2, $3::integer )
        RETURNING *
    )
SELECT 
    tt.id as ticket_type_id,
    tt.display as ticket_type_display,
    ord.id as order_id,
    ord.reserved_until as order_reserved_until,
    ord.purchased_at as order_purchased_at,
    ord.duration_days as duration_days
FROM ticket_types as tt
JOIN ord ON tt.id = ord.ticket_type
        "#,
        type_id,
        chrono::Utc::now()
            .add(chrono::Duration::minutes(10))
            .naive_utc(),
        duration as i32
    )
    .fetch_one(pool)
    .await?;

    Ok(pb::Order {
        id: record.order_id.unwrap().to_string(),
        r#type: Some(pb::TicketType {
            id: record.ticket_type_id.clone(),
            display: record.ticket_type_display.unwrap_or(record.ticket_type_id),
            sold_out: false,
        }),
        duration: record.duration_days.unwrap() as u32,
        price: 0f32,
        reserved_until: record
            .order_reserved_until
            .signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
            .num_seconds() as u64,
        purchased_at: record.order_purchased_at.map(|p| {
            p.signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
                .num_seconds() as u64
        }),
    })
}

pub async fn purchase_order(pool: &DbPool, order_id: &Uuid) -> Result<pb::Order, sqlx::Error> {
    let record = sqlx::query!(
        r#"
with ord as (
    UPDATE orders
    SET purchased_at = $1
    WHERE id = $2 AND purchased_at IS NULL
    RETURNING *
    )
SELECT
    tt.id as ticket_type_id,
    tt.display as ticket_type_display,
    ord.id as order_id,
    ord.reserved_until as order_reserved_until,
    ord.purchased_at as order_purchased_at,
    ord.duration_days as duration_days
FROM ticket_types as tt
JOIN ord ON tt.id = ord.ticket_type
        "#,
        chrono::Utc::now().naive_utc(),
        order_id
    )
    .fetch_one(pool)
    .await?;

    Ok(pb::Order {
        id: record.order_id.unwrap().to_string(),
        r#type: Some(pb::TicketType {
            id: record.ticket_type_id.clone(),
            display: record.ticket_type_display.unwrap_or(record.ticket_type_id),
            sold_out: false,
        }),
        duration: record.duration_days.unwrap() as u32,
        price: 0f32,
        reserved_until: record
            .order_reserved_until
            .signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
            .num_seconds() as u64,
        purchased_at: record.order_purchased_at.map(|p| {
            p.signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
                .num_seconds() as u64
        }),
    })
}

pub async fn get_order(pool: &DbPool, order_id: &Uuid) -> Result<pb::Order, sqlx::Error> {
    let record = sqlx::query!(
        r#"
SELECT
    tt.id as ticket_type_id,
    tt.display as ticket_type_display,
    ord.id as order_id,
    ord.reserved_until as order_reserved_until,
    ord.duration_days as duration_days,
    ord.purchased_at as order_purchased_at
FROM orders as ord
JOIN ticket_types as tt ON tt.id = ord.ticket_type
WHERE ord.id = $1
        "#,
        order_id
    )
    .fetch_one(pool)
    .await?;

    Ok(pb::Order {
        id: record.order_id.unwrap().to_string(),
        r#type: Some(pb::TicketType {
            id: record.ticket_type_id.clone(),
            display: record.ticket_type_display.unwrap_or(record.ticket_type_id),
            sold_out: false,
        }),
        duration: record.duration_days.unwrap() as u32,
        price: 0f32,
        reserved_until: record
            .order_reserved_until
            .signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
            .num_seconds() as u64,
        purchased_at: record.order_purchased_at.map(|p| {
            p.signed_duration_since(chrono::NaiveDateTime::UNIX_EPOCH)
                .num_seconds() as u64
        }),
    })
}
