use async_trait::async_trait;
use eyre::{eyre, Result};
use serde::de::DeserializeOwned;

use crate::engine::Query::Order;
use crate::engine::{
    AllProductsResponse, AssetsResponse, ContractsResponse, EngineStatus, FeeRatesResponse,
    HealthGroupsResponse, InsuranceResponse, IsolatedPositionsResponse, LinkedSignerResponse,
    MarketLiquidityResponse, MarketPairsParams, MarketPairsResponse, MarketPriceResponse,
    MarketPricesResponse, MaxLpMintableResponse, MaxOrderSizeResponse, MaxWithdrawableResponse,
    NoncesResponse, OrderResponse, OrderbookParams, OrderbookResponse, Query, QueryResponseData,
    QueryV2, SubaccountInfoResponse, SubaccountOrdersResponse, SymbolsResponse, Txn,
};
use crate::trigger;
use crate::trigger::ListTriggerOrdersResponse;

use crate::core::base::VertexBase;
use crate::map_response;

#[async_trait]
pub trait VertexQuery: VertexBase + Sync {
    async fn query(&self, query: Query) -> Result<QueryResponseData>;

    async fn query_trigger(&self, query: trigger::Query) -> Result<trigger::QueryResponseData>;

    async fn query_v2<R: DeserializeOwned + Send>(&self, path: &str, query: QueryV2) -> Result<R>;

    async fn get_status(&self) -> Result<EngineStatus> {
        let query_response = self.query(Query::Status {}).await?;
        map_response!(query_response, QueryResponseData::StatusResponse)
    }

    async fn get_contracts(&self) -> Result<ContractsResponse> {
        let query_response = self.query(Query::Contracts {}).await?;
        map_response!(query_response, QueryResponseData::Contracts)
    }

    async fn get_nonces(&self, address: [u8; 20]) -> Result<NoncesResponse> {
        let query_response = self.query(Query::Nonces { address }).await?;
        map_response!(query_response, QueryResponseData::Nonces)
    }

    async fn next_tx_nonce(&self, address: [u8; 20]) -> Result<u64> {
        let nonces = self.get_nonces(address).await?;
        let tx_nonce = nonces.tx_nonce;
        Ok(tx_nonce)
    }

    async fn get_linked_signer(&self, subaccount: [u8; 32]) -> Result<LinkedSignerResponse> {
        let query_response = self.query(Query::LinkedSigner { subaccount }).await?;
        map_response!(query_response, QueryResponseData::LinkedSigner)
    }

    async fn get_subaccount_info(&self, subaccount: [u8; 32]) -> Result<SubaccountInfoResponse> {
        self.get_subaccount_info_with_txns(subaccount, vec![]).await
    }

    async fn get_subaccount_info_with_txns(
        &self,
        subaccount: [u8; 32],
        txns: Vec<Txn>,
    ) -> Result<SubaccountInfoResponse> {
        let txns = if txns.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&txns)?)
        };
        let query_response = self
            .query(Query::SubaccountInfo { subaccount, txns })
            .await?;
        map_response!(query_response, QueryResponseData::SubaccountInfo)
    }

    async fn get_all_products(&self) -> Result<AllProductsResponse> {
        let query = Query::AllProducts {};
        let query_response = self.query(query).await?;
        map_response!(query_response, QueryResponseData::AllProducts)
    }

    async fn get_token_address(&self, product_id: u32) -> Result<[u8; 20]> {
        let products = self.get_all_products().await?;
        for p in products.spot_products {
            if p.product_id == product_id {
                return Ok(p.config.token);
            }
        }
        Err(eyre!("get_token_address: product id not found"))
    }

    async fn get_market_price(&self, product_id: u32) -> Result<MarketPriceResponse> {
        let query_response = self.query(Query::MarketPrice { product_id }).await?;
        map_response!(query_response, QueryResponseData::MarketPrice)
    }

    async fn get_market_prices(&self, product_ids: Vec<u32>) -> Result<MarketPricesResponse> {
        let query_response = self.query(Query::MarketPrices { product_ids }).await?;
        map_response!(query_response, QueryResponseData::MarketPrices)
    }

    async fn get_order(&self, product_id: u32, digest: [u8; 32]) -> Result<OrderResponse> {
        let query = Order { product_id, digest };
        let query_response = self.query(query).await?;
        map_response!(query_response, QueryResponseData::Order)
    }

    async fn get_fee_rates(&self, sender: [u8; 32]) -> Result<FeeRatesResponse> {
        let query_response = self.query(Query::FeeRates { sender }).await?;
        map_response!(query_response, QueryResponseData::FeeRates)
    }

    async fn get_subaccount_orders(
        &self,
        sender: [u8; 32],
        product_id: u32,
    ) -> Result<SubaccountOrdersResponse> {
        let query = Query::SubaccountOrders { sender, product_id };
        let query_response = self.query(query).await?;
        map_response!(query_response, QueryResponseData::SubaccountOrders)
    }

    async fn get_market_liquidity(
        &self,
        product_id: u32,
        depth: u32,
    ) -> Result<MarketLiquidityResponse> {
        let query_response = self
            .query(Query::MarketLiquidity { product_id, depth })
            .await?;
        map_response!(query_response, QueryResponseData::MarketLiquidity)
    }

    async fn get_max_order_size(
        &self,
        max_order_size_query: Query,
    ) -> Result<MaxOrderSizeResponse> {
        let query_response = self.query(max_order_size_query).await?;
        map_response!(query_response, QueryResponseData::MaxOrderSize)
    }

    async fn get_max_withdrawable(
        &self,
        max_withdrawable_query: Query,
    ) -> Result<MaxWithdrawableResponse> {
        let query_response = self.query(max_withdrawable_query).await?;
        map_response!(query_response, QueryResponseData::MaxWithdrawable)
    }

    async fn get_max_lp_mintable(
        &self,
        max_lp_mintable_query: Query,
    ) -> Result<MaxLpMintableResponse> {
        let query_response = self.query(max_lp_mintable_query).await?;
        map_response!(query_response, QueryResponseData::MaxLpMintable)
    }

    async fn get_health_groups(&self) -> Result<HealthGroupsResponse> {
        let query_response = self.query(Query::HealthGroups {}).await?;
        map_response!(query_response, QueryResponseData::HealthGroups)
    }

    async fn get_insurance(&self) -> Result<InsuranceResponse> {
        let query_response = self.query(Query::Insurance {}).await?;
        map_response!(query_response, QueryResponseData::Insurance)
    }

    async fn get_symbols(
        &self,
        product_ids: Option<Vec<u32>>,
        product_type: Option<String>,
    ) -> Result<SymbolsResponse> {
        let query_response = self
            .query(Query::Symbols {
                product_ids,
                product_type,
            })
            .await?;
        map_response!(query_response, QueryResponseData::Symbols)
    }

    async fn list_trigger_orders(
        &self,
        list_trigger_orders: trigger::Query,
    ) -> Result<ListTriggerOrdersResponse> {
        let query_response = self.query_trigger(list_trigger_orders).await?;
        map_response!(
            query_response,
            trigger::QueryResponseData::ListTriggerOrders
        )
    }

    async fn get_orderbook(&self, ticker_id: String, depth: u32) -> Result<OrderbookResponse> {
        let query = QueryV2::Orderbook(OrderbookParams { ticker_id, depth });
        self.query_v2("/orderbook", query).await
    }

    async fn get_pairs(&self, market: Option<String>) -> Result<MarketPairsResponse> {
        let query = QueryV2::Pairs(MarketPairsParams { market });
        self.query_v2("/pairs", query).await
    }

    async fn get_assets(&self) -> Result<AssetsResponse> {
        let query = QueryV2::Assets {};
        self.query_v2("/assets", query).await
    }

    async fn get_isolated_positions(
        &self,
        subaccount: [u8; 32],
    ) -> Result<IsolatedPositionsResponse> {
        let query_response = self.query(Query::IsolatedPositions { subaccount }).await?;
        map_response!(query_response, QueryResponseData::IsolatedPositions)
    }
}
