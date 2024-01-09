# Vertex Protocol Rust SDK

This is the Rust SDK for the [Vertex Protocol API](https://vertex-protocol.gitbook.io/docs/developer-resources/api).

[Documentation](link to docs.rs)
## Installation

Add the following line to your Cargo.toml file:
```toml
[dependencies]
vertex_sdk = "0.1.0"
```

## Usage 
See the [examples](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/examples) and [sanity](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/src/sanity) directories.

## Running locally
### Run sanity checks

- `cargo run -- --execute-sanity`: runs sanity checks for executes.
- `cargo run -- --query-sanity`: runs sanity checks for engine queries.
- `cargo run -- --indexer-sanity`: runs sanity checks for indexer queries.



