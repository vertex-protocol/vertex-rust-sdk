use eyre::Result;

use crate::engine::Direction;
use crate::math::to_i128_x18;

use crate::prelude::*;
use crate::print_json;
use crate::utils::private_key::private_key;
use crate::vertex_client::VertexClient;

pub async fn query_sanity_check() -> Result<()> {
    let client = VertexClient::new(ClientMode::Local)
        .with_signer(private_key())
        .await?;

    let all_products = client.get_all_products().await?;
    print_json!(all_products);

    let subaccount_info = client
        .get_subaccount_info(client.subaccount().unwrap())
        .await?;
    print_json!(subaccount_info);

    let order_book = client.get_orderbook("BTC-PERP_USDC".to_string(), 2).await?;
    print_json!(order_book);

    let pairs = client.get_pairs(None).await?;
    print_json!(pairs);

    let assets = client.get_assets().await?;
    print_json!(assets);

    let versions = client.get_versions().await?;
    print_json!(versions);

    let symbols = client.get_symbols(None, None).await?;
    print_json!(symbols);

    let health_groups = client.get_health_groups().await?;
    print_json!(health_groups);

    let fee_rates = client.get_fee_rates([0; 32]).await?;
    print_json!(fee_rates);

    let market_price = client.get_market_price(1).await?;
    print_json!(market_price);

    let market_prices = client.get_market_prices(vec![1]).await?;
    print_json!(market_prices);

    let linked_signer = client.get_linked_signer([0; 32]).await?;
    print_json!(linked_signer);

    let nonces = client.get_nonces([0; 20]).await?;
    print_json!(nonces);

    let status = client.get_status().await?;
    print_json!(status);

    let market_liquidity = client.get_market_liquidity(1, 1).await?;
    print_json!(market_liquidity);

    let max_order_size = client
        .get_max_order_size_builder()
        .subaccount(client.subaccount().unwrap())
        .product_id(1)
        .price_x18(to_i128_x18(27000))
        .direction(Direction::Short)
        .query()
        .await?;

    print_json!(max_order_size);

    let max_withdrawable = client
        .get_max_withdrawable_builder()
        .product_id(1)
        .subaccount(client.subaccount().unwrap())
        .query()
        .await?;
    print_json!(max_withdrawable);

    let max_lp_mintable = client
        .get_max_lp_mintable_builder()
        .subaccount(client.subaccount().unwrap())
        .product_id(1)
        .query()
        .await?;
    print_json!(max_lp_mintable);

    Ok(())
}
