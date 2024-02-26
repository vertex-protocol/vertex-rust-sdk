use vertex_sdk::math::to_u128_x6;
use vertex_sdk::prelude::*;

#[tokio::main]
async fn main() {
    // a client without a signer can make queries
    let client = VertexClient::new(ClientMode::SepoliaTest);

    const ARB_PERP: u32 = 6;
    let depth = 10;
    let order_book = client.get_market_liquidity(ARB_PERP, depth).await.unwrap();
    println!("orderbook: {:?}", order_book);

    let liquidation_feed = client.get_liquidation_feed().await.unwrap();
    print!("liquidateable accounts: {:?}", liquidation_feed);

    // anything requiring a private key fails
    assert!(client.subaccount().is_err());
    assert!(client
        .deposit_collateral_builder()
        .product_id(0)
        .amount(to_u128_x6(10))
        .execute()
        .await
        .is_err());
}
