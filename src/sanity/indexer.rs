use crate::indexer::Limit;
use crate::prelude::*;
use crate::print_json;
use crate::serialize_utils::WrappedU32;
use crate::utils::private_key::private_key;
use crate::utils::time::timestamp;
use crate::vertex_client::VertexClient;
use eyre::Result;

pub async fn indexer_sanity_check() -> Result<()> {
    let client = VertexClient::new(ClientMode::SepoliaTest)
        .with_signer(private_key())
        .await?;
    let funding_rate = client.get_funding_rate(1).await?;
    print_json!(funding_rate);

    let funding_rates = client.get_funding_rates(vec![1]).await?;
    print_json!(funding_rates);

    let liquidation_feed = client.get_liquidation_feed().await?;
    print_json!(liquidation_feed);

    let oracle_price = client.get_oracle_price(vec![1, 2]).await?;
    print_json!(oracle_price);

    let rewards = client
        .get_rewards_builder()
        .address([0; 20])
        .query()
        .await?;
    print_json!(rewards);

    let subaccount_summary = client.get_subaccount_summary([0; 32], None).await?;
    print_json!(subaccount_summary);

    let linked_signer_rate_limit = client.get_linked_signer_rate_limit([0; 32]).await?;
    print_json!(linked_signer_rate_limit);

    let linked_signers = client.get_linked_signers(Some(100), Some(10)).await?;
    print_json!(linked_signers);

    let referral_code = client.get_referral_code([0; 32]).await?;
    print_json!(referral_code);

    let perp_prices = client.get_perp_prices(vec![2]).await?;
    print_json!(perp_prices);

    let trades = client
        .get_trades_builder()
        .ticker_id("BTC-PERP_USDC".to_string())
        .max_trade_id(0)
        .limit(10)
        .query()
        .await?;
    print_json!(trades);

    let tickers = client.get_tickers(None, None).await?;
    print_json!(tickers);

    let contracts = client.get_contracts_v2(None).await?;
    print_json!(contracts);

    let candlesticks = client
        .get_candlesticks_builder()
        .product_id(1)
        .granularity(60)
        .max_time(timestamp())
        .limit(3000)
        .query()
        .await?;
    print_json!(candlesticks);

    let product_snapshots = client
        .get_product_snapshots_builder()
        .product_id(1)
        .max_time(timestamp())
        .limit(10)
        .idx(1000)
        .query()
        .await?;
    print_json!(product_snapshots);

    let events = client
        .get_events_builder()
        .limit(Limit::Txs(WrappedU32(10)))
        .product_ids(vec![1, 4])
        .max_time(timestamp())
        .idx(10000)
        .desc(true)
        .query()
        .await?;
    print_json!(events);

    let historical_orders = client
        .get_historical_orders_builder()
        .subaccount([0; 32])
        .limit(20)
        .product_ids(vec![1, 4])
        .max_time(timestamp())
        .query()
        .await?;
    print_json!(historical_orders);

    let matches = client
        .get_matches_builder()
        .subaccount([0; 32])
        .product_ids(vec![1, 2, 3])
        .limit(100)
        .max_time(timestamp())
        .idx(1000)
        .query()
        .await?;
    print_json!(matches);

    if client.client_mode != ClientMode::Local {
        let maker_statistics = client
            .get_maker_statistics_builder()
            .epoch(0)
            .product_id(1)
            .interval(3600)
            .query()
            .await?;
        print_json!(maker_statistics);
    }

    let subaccounts = client
        .get_subaccounts_builder()
        .address([0; 20])
        .limit(10)
        .start(100)
        .query()
        .await?;
    print_json!(subaccounts);

    let interest_and_funding = client
        .get_interest_and_funding_builder()
        .subaccount(client.subaccount().unwrap())
        .product_ids(vec![1, 4])
        .limit(100)
        .query()
        .await?;
    print_json!(interest_and_funding);

    let market_snapshots = client
        .get_market_snapshots_builder()
        .granularity(84600)
        .count(2)
        .max_time(timestamp())
        .query()
        .await?;
    print_json!(market_snapshots);

    Ok(())
}
