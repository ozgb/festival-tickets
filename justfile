# Default recipe
default:
    just -l

# Reset database
reset-db:
    sqlx migrate revert --target-version 0
    sqlx migrate run
