# Festival ticket ordering system

# Work in Progress!

Festival booking systems are known to crash on ticket sale launch (Green Man, End of the Road, Bangface). This is an implementation that can survive high load!

Requirements:

- Runs on a single machine
- Can handle thousands of requests per second
- Launch countdown
- Stores state in database
- Multiple room types
- Multiple durations (3 or 4 days)
- Payment portal
- Add to cart -> Checkout -> Payment
- Simple front-end

## Tech stack

- Tonic/gRPC
- Postgres

## Dev setup

Install sqlx cli;

```bash
$ cargo install sqlx-cli
```

In one terminal:

```bash
$ docker compose up
```

In another:

```bash
$ cp template.env .env
$ cargo run
```

### Frontend

Install `protoc-gen-js`:

```bash
$ npm install -g protoc-gen-js
```

Download `protoc-gen-grpc-web`, make it executable and available on your `PATH`: [Releases](https://github.com/grpc/grpc-web/releases)

To generate a new client after updating the gRPC spec, run:

```bash
$ just gen-client
```

To serve the frontend, run:
```bash
$ just serve-client
```
