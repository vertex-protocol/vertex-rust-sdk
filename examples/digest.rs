use ethers::prelude::H256;

use vertex_sdk::math::{f64_to_x18, to_i128_x18};
use vertex_sdk::prelude::*;
use vertex_sdk::utils::private_key::private_key;

/// Example showing how to get an order digest before placing an order
#[tokio::main]
async fn main() {
    let client = VertexClient::new(ClientMode::SepoliaTest)
        .with_signer(private_key())
        .await
        .unwrap();

    const BTC_PERP: u32 = 2;

    // place buy order for 1 BTC_PERP
    let place_order = client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(f64_to_x18(0.01))
        .price_x18(to_i128_x18(35_000))
        .build()
        .unwrap();

    let digest = place_order.digest.unwrap();

    println!("digest: {:#x}", H256::from(digest));

    client.place_order(place_order).await.unwrap();
}
