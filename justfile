set dotenv-load

# Clear and seed the database, then run all tests
test:
    sqlx migrate revert --target-version 0
    sqlx migrate run
    psql $DATABASE_URL -f seed_db.sql
    cargo test -- --show-output

# Generate client code from proto file
gen-client:
    protoc -I=proto/ proto/purchase.proto \
        --js_out=import_style=commonjs:frontend/ \
        --grpc-web_out=import_style=commonjs,mode=grpcwebtext:frontend/
    cd frontend; npm run build

# Run Envoy to route HTTP/1.1 to HTTP2 tonic endpoint
envoy:
    docker run -v "$(pwd)"/envoy.yaml:/etc/envoy/envoy.yaml:ro \
    --network=host envoyproxy/envoy:v1.22.0
