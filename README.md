# Vertex Protocol Rust SDK
[![Crates.io][crates-img]][crates-url]

This is the Rust SDK for the [Vertex Protocol API](https://vertex-protocol.gitbook.io/docs/developer-resources/api).

[Documentation](https://docs.rs/vertex-sdk/latest/vertex_sdk/)

## Quickstart
Instantiate a client on the chain you would like to interact with. 
For example, `ClientMode::Prod` to use Arbitrum and `ClientMode::BlastProd` to use Blast.
A signer (private key) is required for executes. 
A signer is not required for queries.
For requests with many parameters, use the client to build and send requests.
For simple queries (1-2 params) like `get_market_price`, call directly from the client.



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


## Installation

Add the following line to your Cargo.toml file:
```toml
[dependencies]
vertex_sdk = "0.2.5"
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


