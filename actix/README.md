# Actix Web Implementation

This is a re-implementation of the festival tickets app in Actix Web

This can be used as a comparison to see how the complexity/setup differs between the two implementations.

## Requirements

To re-generate openapi client library for the tests:

Get progenitor using [cargo binstall](https://github.com/cargo-bins/cargo-binstall):
```bash
$ cargo binstall cargo-progenitor
```
or install from source:
```bash
$ cargo install cargo-progenitor
```

Start the server locally and run:
```bash
$ just gen-client
```

## Features

- OpenAPI Spec
- Very fast
