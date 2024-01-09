use vertex_sdk::eip712_structs::OrderType;
use vertex_sdk::math::to_i128_x18;
use vertex_sdk::prelude::*;
use vertex_sdk::utils::private_key::private_key;
use vertex_sdk::utils::time::timestamp;

#[tokio::main]
async fn main() {
    let client = VertexClient::new(ClientMode::SepoliaTest)
        .with_signer(private_key())
        .await
        .unwrap();

    const BTC_PERP: u32 = 2;

    // use .execute() to build and send order or .build() to just build the order
    // unset optional parameters are set to reasonable defaults.

    // buy 1 BTC_PERP
    client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(1))
        .price_x18(to_i128_x18(35_000))
        .execute()
        .await
        .unwrap();

    // attempting to build an order without the required parameters errors:
    assert!(client
        .place_order_builder()
        .amount(to_i128_x18(1))
        .price_x18(35_000)
        .execute()
        .await
        .is_err());

    // example order with all parameters set (to the default)
    let order = client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(1))
        .price_x18(to_i128_x18(35_000))
        .reduce_only(false)
        .expiration(u32::MAX as u64)
        .order_type(OrderType::Default)
        .recv_time((timestamp() + 90) * 1000)
        .build()
        .unwrap();

    client.place_order(order).await.unwrap();

    // Place fill or kill order
    client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(-1))
        .price_x18(35_000)
        .order_type(OrderType::FillOrKill)
        .execute()
        .await
        .unwrap();
}
