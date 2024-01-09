use vertex_sdk::math::to_i128_x18;
use vertex_sdk::prelude::*;
use vertex_sdk::utils::private_key::private_key;

#[tokio::main]
async fn main() {
    // paste private key or set RUST_SDK_PRIVATE_KEY in .env file
    let private_key = private_key();

    let client = VertexClient::new(ClientMode::SepoliaTest)
        .with_signer(private_key)
        .await
        .unwrap();

    const BTC_PERP: u32 = 2;

    // sell 1 BTC_PERP
    let response = client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(-1))
        .price_x18(to_i128_x18(35_000))
        .execute()
        .await
        .unwrap();

    let digest = response.unwrap().digest;

    // check subaccount open orders
    let open_orders = client
        .get_subaccount_orders(client.subaccount().unwrap(), BTC_PERP)
        .await
        .unwrap();
    println!("open orders: {:?}", open_orders);

    // cancel order
    let cancellation_response = client
        .cancellation_builder()
        .digests(vec![digest])
        .product_ids(vec![BTC_PERP])
        .execute()
        .await
        .unwrap();

    let canceled_digest = cancellation_response.unwrap().cancelled_orders[0].digest;
    assert_eq!(digest, canceled_digest)
}
