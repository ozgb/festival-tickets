use std::ops::Add;

use crate::pb::OrderStats;

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

pub async fn get_ticket_durations(pool: &DbPool, _type_id: &str) -> Result<Vec<i32>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM order_stats")
        .fetch_all(pool)
        .await?;

    let mut durations = vec![];
    for row in rows {
        if row.order_limit > row.order_count.unwrap_or(0) {
            durations.push(row.duration_days);
        }
    }

    Ok(durations)
}

pub async fn add_ticket_to_basket(
    pool: &DbPool,
    type_id: &str,
    duration: i32,
) -> Result<pb::Order, sqlx::Error> {
    let order = sqlx::query_as!(
        pb::Order,
        r#"
WITH ord as (
        INSERT INTO orders (ticket_type, reserved_until, duration_days)
        VALUES ( $1, $2, $3::integer )
        RETURNING *
    )
SELECT 
    ord.id::text as "id!",
    tt.id as "ticket_type_id!",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until::text as "reserved_until!",
    ord.purchased_at::text as purchased_at
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

    Ok(order)
}

pub async fn purchase_order(pool: &DbPool, order_id: &Uuid) -> Result<pb::Order, sqlx::Error> {
    let order = sqlx::query_as!(
        pb::Order,
        r#"
with ord as (
    UPDATE orders
    SET purchased_at = $1
    WHERE id = $2 AND purchased_at IS NULL
    RETURNING *
    )
SELECT
    ord.id::text as "id!",
    tt.id as "ticket_type_id!",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until::text as "reserved_until!",
    ord.purchased_at::text as purchased_at
FROM ticket_types as tt
JOIN ord ON tt.id = ord.ticket_type
        "#,
        chrono::Utc::now().naive_utc(),
        order_id
    )
    .fetch_one(pool)
    .await?;

    Ok(order)
}

pub async fn get_order(pool: &DbPool, order_id: &Uuid) -> Result<pb::Order, sqlx::Error> {
    let order = sqlx::query_as!(
        pb::Order,
        r#"
SELECT
    ord.id::text as "id!",
    tt.id as "ticket_type_id!",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until::text as "reserved_until!",
    ord.purchased_at::text as purchased_at
FROM orders as ord
JOIN ticket_types as tt ON tt.id = ord.ticket_type
WHERE ord.id = $1
        "#,
        order_id
    )
    .fetch_one(pool)
    .await?;

    Ok(order)
}

pub async fn get_order_stats(pool: &DbPool) -> Result<Vec<OrderStats>, sqlx::Error> {
    let order_stats = sqlx::query_as!(
        pb::OrderStats,
        r#"
SELECT
    duration_days::integer as "duration_days!",
    order_limit::integer as "order_limit!",
    order_count::integer as "order_count!"
FROM order_stats"#
    )
    .fetch_all(pool)
    .await?;

    Ok(order_stats)
}

pub async fn remove_expired_orders(pool: &DbPool) -> Result<(), sqlx::Error> {
    let cur_time = chrono::Utc::now();
    sqlx::query!(
        r#"
DELETE FROM orders
WHERE reserved_until < $1 AND purchased_at IS NULL
        "#,
        cur_time.naive_utc()
    )
    .execute(pool)
    .await?;

    Ok(())
}
