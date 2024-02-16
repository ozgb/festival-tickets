use std::ops::Add;

use crate::api::types::{AddUserInfoRequest, Order, OrderStats, TicketType, User};

use super::env;
use chrono;
use futures::TryStreamExt;
use sqlx::postgres::Postgres;
use sqlx::types::Uuid;
use sqlx::Row;

pub type DbPool = sqlx::Pool<Postgres>;

pub mod error;
use error::DbError;

pub type DbResult<T> = Result<T, DbError>;

pub async fn connect_to_pool() -> DbPool {
    let db_url = env::Cfg::DatabaseUrl.load().expect("Failed to load db url");

    DbPool::connect(&db_url)
        .await
        .expect("Failed to connect to pool")
}

pub async fn get_ticket_types(pool: &DbPool) -> DbResult<Vec<TicketType>> {
    let mut rows = sqlx::query("SELECT * FROM ticket_types").fetch(pool);

    let mut ticket_types = Vec::new();
    while let Some(row) = rows.try_next().await? {
        ticket_types.push(TicketType {
            id: row.try_get::<Uuid, _>("id")?,
            display: row.try_get::<String, _>("display")?,
            sold_out: false,
        });
    }

    Ok(ticket_types)
}

pub async fn get_ticket_durations(pool: &DbPool, _type_id: &str) -> DbResult<Vec<i32>> {
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

pub async fn add_ticket_to_basket(pool: &DbPool, type_id: &str, duration: i32) -> DbResult<Order> {
    let order = sqlx::query_as!(
        Order,
        r#"
WITH ord as (
        INSERT INTO orders (ticket_type, reserved_until, duration_days)
        VALUES ( $1, $2, $3::integer )
        RETURNING *
    )
SELECT 
    ord.id as "id!",
    tt.id as "ticket_type_id!",
    ord.user_id::text as "user_id",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until as "reserved_until!",
    ord.purchased_at as purchased_at
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

pub async fn purchase_order(pool: &DbPool, order_id: &Uuid) -> DbResult<Order> {
    let precond = sqlx::query!("SELECT user_id FROM orders WHERE id = $1", order_id)
        .fetch_one(pool)
        .await?;

    if precond.user_id.is_none() {
        return Err(DbError::FailedPrecondition(format!(
            "user info missing from order {}",
            order_id
        )));
    }

    let order = sqlx::query_as!(
        Order,
        r#"
with ord as (
    UPDATE orders
    SET purchased_at = $1
    WHERE id = $2 AND purchased_at IS NULL
    RETURNING *
    )
SELECT
    ord.id as "id!",
    tt.id as "ticket_type_id!",
    ord.user_id::text as "user_id",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until as "reserved_until!",
    ord.purchased_at as purchased_at
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

pub async fn get_order(pool: &DbPool, order_id: &Uuid) -> DbResult<Option<Order>> {
    let order = sqlx::query_as!(
        Order,
        r#"
SELECT
    ord.id as "id!",
    tt.id as "ticket_type_id!",
    ord.user_id::text as "user_id",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until as "reserved_until!",
    ord.purchased_at as purchased_at
FROM orders as ord
JOIN ticket_types as tt ON tt.id = ord.ticket_type
WHERE ord.id = $1
        "#,
        order_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(order)
}

pub async fn get_user(pool: &DbPool, user_id: &Uuid) -> DbResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
SELECT
    users.id as "id!",
    users.name as "name!",
    users.address as "address!",
    users.email as "email!",
    ord.id as "order_id!"
FROM users
JOIN orders AS ord ON ord.user_id = users.id
WHERE users.id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn add_user_to_order(
    pool: &DbPool,
    order_id: &Uuid,
    req: &AddUserInfoRequest,
) -> DbResult<Order> {
    // TODO: Check if order already has a user attached

    let mut tx = pool.begin().await?;

    let user = sqlx::query!(
        r#"
INSERT INTO users (name, address, email)
VALUES ($1, $2, $3)
RETURNING *
        "#,
        req.name,
        req.address,
        req.email
    )
    .fetch_one(&mut *tx)
    .await?;

    let _update = sqlx::query!(
        r#"
UPDATE orders
SET user_id = $2
WHERE id = $1
RETURNING *
        "#,
        order_id,
        user.id
    )
    .fetch_one(&mut *tx)
    .await?;

    let order = sqlx::query_as!(
        Order,
        r#"
SELECT 
    ord.id as "id!",
    tt.id as "ticket_type_id!",
    ord.user_id::text as "user_id",
    ord.duration_days::integer as "duration!",
    44.0::real as "price!",
    ord.reserved_until as "reserved_until!",
    ord.purchased_at as purchased_at
FROM orders as ord
JOIN ticket_types as tt ON tt.id = ord.ticket_type
WHERE ord.id = $1
        "#,
        order_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(order)
}

pub async fn get_order_stats(pool: &DbPool) -> DbResult<Vec<OrderStats>> {
    let order_stats = sqlx::query_as!(
        OrderStats,
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

pub async fn remove_expired_orders(pool: &DbPool) -> DbResult<()> {
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
