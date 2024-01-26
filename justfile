set dotenv-load

# Clear and seed the database, then run all tests
test:
    sqlx migrate revert --target-version 0
    sqlx migrate run
    psql $DATABASE_URL -f seed_db.sql
    cargo test -- --show-output
