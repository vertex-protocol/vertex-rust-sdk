# Vertex Protocol Rust SDK
[![Crates.io][crates-img]][crates-url]

This is the Rust SDK for the [Vertex Protocol API](https://vertex-protocol.gitbook.io/docs/developer-resources/api).

[Documentation](https://docs.rs/vertex-sdk/latest/vertex_sdk/)
## Installation

Add the following line to your Cargo.toml file:
```toml
[dependencies]
vertex_sdk = "0.1.3"
```

## Usage 
See the [examples](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/examples) and [sanity](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/src/sanity) directories.

## Running locally
### Run sanity checks

- `cargo run -- --execute-sanity`: runs sanity checks for executes.
- `cargo run -- --query-sanity`: runs sanity checks for engine queries.
- `cargo run -- --indexer-sanity`: runs sanity checks for indexer queries.

[crates-img]: https://img.shields.io/crates/v/vertex-sdk
[crates-url]: https://crates.io/crates/vertex-sdk


