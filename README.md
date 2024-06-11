# Vertex Protocol Rust SDK
[![Crates.io][crates-img]][crates-url]

This is the Rust SDK for the [Vertex Protocol API](https://vertex-protocol.gitbook.io/docs/developer-resources/api).

## Quickstart
After instantiating a client, most requests look like the below example. Generally, we use the builder 
pattern to build and send requests with many parameters easy to write and keep track of. Simple queries like `get_market_price`,
with only one parameter can be made directly from the client.


See [`basic_usage.rs`](examples/basic_usage.rs) for an E2E example including depositing into Vertex. 

```rust
use vertex_sdk::prelude::*;

async fn main() {
    let client = VertexClient::new(ClientMode::Prod)
        .with_signer(private_key())
        .await
        .unwrap();

    const BTC_PERP: u32 = 2;

    // query market data
    let market_price = client.get_market_price(BTC_PERP).await.unwrap();

    // place orders
    let place_order_response = client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(1))
        .price_x18(market_price.ask_x18)
        .execute()
        .await
        .unwrap();
    
    let digest = place_order_response.unwrap().digest;

    // cancel orders
    client
        .cancellation_builder()
        .digests(vec![digest])
        .product_ids(vec![BTC_PERP])
        .execute()
        .await
        .unwrap();
}
```

[Documentation](https://docs.rs/vertex-sdk/latest/vertex_sdk/)
## Installation

Add the following line to your Cargo.toml file:
```toml
[dependencies]
vertex_sdk = "0.1.4"
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


