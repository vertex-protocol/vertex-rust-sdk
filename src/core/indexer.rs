use std::collections::HashMap;

use async_trait::async_trait;
use ethers::prelude::H160;
use eyre::Result;
use serde::de::DeserializeOwned;

use crate::core::base::VertexBase;
use crate::indexer::Query::FastWithdrawalSignature;
use crate::indexer::{
    AccountSnapshotsResponse, ArbRewardsResponse, CandlesticksResponse, EventsResponse,
    FoundationTakerRewardsResponse, FundingRateResponse, InterestAndFundingTicksResponse,
    LeaderboardResponse, LinkedSignerRateLimitResponse, LinkedSignerResponse,
    MakerStatisticsResponse, MarketSnapshotsResponse, MatchesResponse, MerkleProofsResponse,
    OraclePriceResponse, OrdersResponse, PerpContractResponse, PerpPriceResponse, ProductSnapshot,
    ProductsResponse, Query, QueryV2, ReferralCodeResponse, RewardsResponse, SubaccountsResponse,
    SummaryResponse, TickerResponse, TickersParams, TimestampOrTimestamps, TradesResponse,
    UsdcPriceResponse,
};
use crate::indexer::{FastWithdrawalSignatureResponse, LiquidatableAccount};
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::signer::Signer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;

#[async_trait]
pub trait VertexIndexer<S: Signer>: VertexBase<S> {
    async fn query<R: DeserializeOwned + Send>(&self, query: Query) -> Result<R>;

    async fn query_v2<R: DeserializeOwned + Send>(&self, path: &str, query: QueryV2) -> Result<R>;

    async fn get_liquidation_feed(&self) -> Result<Vec<LiquidatableAccount>> {
        let query = Query::LiquidationFeed {};
        self.query(query).await
    }

    async fn get_funding_rate(&self, product_id: u32) -> Result<FundingRateResponse> {
        let query = Query::FundingRate {
            product_id: WrappedU32(product_id),
        };
        self.query(query).await
    }

    async fn get_funding_rates(
        &self,
        product_ids: Vec<u32>,
    ) -> Result<HashMap<u32, FundingRateResponse>> {
        let product_ids = product_ids.into_iter().map(WrappedU32).collect();
        let query = Query::FundingRates { product_ids };
        self.query(query).await
    }

    async fn get_candlesticks(&self, candlesticks_query: Query) -> Result<CandlesticksResponse> {
        self.query(candlesticks_query).await
    }

    async fn get_product_snapshots(
        &self,
        product_snapshots_query: Query,
    ) -> Result<ProductsResponse> {
        self.query(product_snapshots_query).await
    }

    async fn get_multi_product_snapshots(
        &self,
        product_snapshots_query: Query,
    ) -> Result<HashMap<u32, ProductSnapshot>> {
        self.query(product_snapshots_query).await
    }

    async fn get_multi_timestamp_product_snapshots(
        &self,
        product_snapshots_query: Query,
    ) -> Result<HashMap<WrappedU64, HashMap<u32, ProductSnapshot>>> {
        self.query(product_snapshots_query).await
    }

    async fn get_account_snapshots(
        &self,
        account_snapshots_query: Query,
    ) -> Result<AccountSnapshotsResponse> {
        self.query(account_snapshots_query).await
    }

    async fn get_events(&self, events_query: Query) -> Result<EventsResponse> {
        self.query(events_query).await
    }

    async fn get_historical_orders(&self, orders_query: Query) -> Result<OrdersResponse> {
        self.query(orders_query).await
    }

    async fn get_subaccount_summary(
        &self,
        subaccount: [u8; 32],
        timestamp: Option<TimestampOrTimestamps>,
    ) -> Result<SummaryResponse> {
        let query = Query::Summary {
            subaccount,
            timestamp,
        };
        self.query(query).await
    }

    async fn get_matches(&self, matches_query: Query) -> Result<MatchesResponse> {
        self.query(matches_query).await
    }

    async fn get_rewards(&self, query: Query) -> Result<RewardsResponse> {
        self.query(query).await
    }

    async fn get_subaccounts(&self, subaccounts_query: Query) -> Result<SubaccountsResponse> {
        self.query(subaccounts_query).await
    }

    async fn get_perp_prices(
        &self,
        product_ids: Vec<u32>,
    ) -> Result<HashMap<u32, PerpPriceResponse>> {
        let product_ids = product_ids.into_iter().map(WrappedU32).collect();
        let query = Query::PerpPrices { product_ids };
        self.query(query).await
    }

    async fn get_oracle_price(&self, product_ids: Vec<u32>) -> Result<OraclePriceResponse> {
        let product_ids = product_ids.into_iter().map(WrappedU32).collect();
        let query = Query::OraclePrice { product_ids };
        self.query(query).await
    }

    async fn get_usdc_price(&self) -> Result<UsdcPriceResponse> {
        let query = Query::UsdcPrice {};
        self.query(query).await
    }

    async fn get_maker_statistics(
        &self,
        maker_statistics_query: Query,
    ) -> Result<MakerStatisticsResponse> {
        self.query(maker_statistics_query).await
    }

    async fn get_linked_signer_rate_limit(
        &self,
        subaccount: [u8; 32],
    ) -> Result<LinkedSignerRateLimitResponse> {
        let query = Query::LinkedSignerRateLimit { subaccount };
        self.query(query).await
    }

    async fn get_linked_signers(
        &self,
        start_idx: Option<u64>,
        limit: Option<u32>,
    ) -> Result<LinkedSignerResponse> {
        let query = Query::LinkedSigners {
            start_idx: wrapped_option_u64(start_idx),
            limit: limit.map(WrappedU32),
        };
        self.query(query).await
    }

    async fn get_market_snapshots(
        &self,
        market_snapshots_query: Query,
    ) -> Result<MarketSnapshotsResponse> {
        self.query(market_snapshots_query).await
    }

    async fn get_referral_code(&self, subaccount: [u8; 32]) -> Result<ReferralCodeResponse> {
        let query = Query::ReferralCode { subaccount };
        self.query(query).await
    }

    async fn get_interest_and_funding(
        &self,
        interest_and_funding_query: Query,
    ) -> Result<InterestAndFundingTicksResponse> {
        self.query(interest_and_funding_query).await
    }

    async fn get_vrtx_merkle_proofs(&self, address: H160) -> Result<MerkleProofsResponse> {
        let query = Query::VrtxMerkleProofs { address };
        self.query(query).await
    }

    async fn get_arb_merkle_proofs(&self, address: H160) -> Result<MerkleProofsResponse> {
        let query = Query::ArbMerkleProofs { address };
        self.query(query).await
    }

    async fn get_arb_rewards(&self, address: H160) -> Result<ArbRewardsResponse> {
        let query = Query::ArbRewards { address };
        self.query(query).await
    }

    async fn get_tickers(&self, market: Option<String>) -> Result<HashMap<String, TickerResponse>> {
        self.query_v2("/tickers", QueryV2::Tickers(TickersParams { market }))
            .await
    }

    async fn get_trades(&self, trades_query: QueryV2) -> Result<TradesResponse> {
        self.query_v2("/trades", trades_query).await
    }

    async fn get_contracts_v2(&self) -> Result<HashMap<String, PerpContractResponse>> {
        self.query_v2("/contracts", QueryV2::Contracts {}).await
    }

    async fn get_leaderboard(&self, query: Query) -> Result<LeaderboardResponse> {
        self.query(query).await
    }

    async fn get_fast_withdrawal_signature(
        &self,
        idx: u64,
    ) -> Result<FastWithdrawalSignatureResponse> {
        self.query(FastWithdrawalSignature {
            idx: WrappedU64(idx),
        })
        .await
    }

    async fn get_foundation_taker_rewards(
        &self,
        query: Query,
    ) -> Result<FoundationTakerRewardsResponse> {
        self.query(query).await
    }
}
