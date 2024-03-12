use vertex_sdk::math::{to_i128_x18, to_u128_x18, to_u128_x6};
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

    // product_id for USDC
    const USDC: u32 = 0;

    // deposit 1_000 USDC
    client
        .deposit_collateral_builder()
        .product_id(USDC)
        .amount(to_u128_x6(1000)) // scale by token decimals (USDC has 6 decimals)
        .mints_tokens(true) // testnet only (mints specified amount of token before depositing)
        .deposit_and_await_balance() // waits for change in balance
        .await
        .unwrap();

    const BTC: u32 = 0;

    // deposit 1 BTC
    let deposit_collateral_params = client
        .deposit_collateral_builder()
        .product_id(BTC)
        .amount(to_u128_x18(1)) // scale by token decimals (BTC has 18, etc)
        .mints_tokens(true) // testnet only (mints specified amount of token before depositing)
        .build()
        .unwrap();

    // every builder can build a struct which can be used by the client, or it can execute directly
    // with .execute()
    client
        .deposit_collateral(deposit_collateral_params)
        .await
        .unwrap();

    // product_id for BTC_PERP
    const BTC_PERP: u32 = 2;

    // buy 1 BTC_PERP
    client
        .place_order_builder()
        .product_id(BTC_PERP)
        .amount(to_i128_x18(1))
        .price_x18(to_i128_x18(35_000))
        .execute()
        .await
        .unwrap();

    // withdraw 100 USDC
    client
        .withdraw_collateral_builder()
        .product_id(USDC)
        .amount(to_u128_x6(100))
        .execute()
        .await
        .unwrap();
}
