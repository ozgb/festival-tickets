set dotenv-load

# Default recipe
default:
    just -l

# Clear and seed the database, then run all tests
test:
    sqlx migrate revert --target-version 0
    sqlx migrate run
    psql $DATABASE_URL -f seed_db.sql
    cargo test -- --show-output

# Generate client code from proto file
gen-client:
    protoc -I=proto/ proto/purchase.proto \
        --js_out=import_style=commonjs:frontend/grpc/ \
        --grpc-web_out=import_style=typescript,mode=grpcwebtext:frontend/grpc/
    cd frontend; npm run build

# Serve web client
serve-client:
    cd frontend; npm run serve

# Run Envoy to route HTTP/1.1 to HTTP2 tonic endpoint
envoy:
    docker run -v "$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro \
    --network=host envoyproxy/envoy:v1.22.0
