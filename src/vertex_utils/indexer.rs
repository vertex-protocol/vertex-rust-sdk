use crate::bindings::endpoint::WithdrawCollateral;
use crate::bindings::querier::{PerpBalance, PerpProduct, SpotBalance, SpotProduct};
use crate::eip712_structs;
use crate::eip712_structs::LeaderboardAuthentication;
use crate::serialize_utils::{
    deserialize_bytes20, deserialize_bytes32, deserialize_f64, deserialize_i128, deserialize_i64,
    deserialize_option_f64, deserialize_u64, deserialize_vec_bytes20, deserialize_vec_u8,
    serialize_bytes20, serialize_bytes32, serialize_f64, serialize_i128, serialize_i64,
    serialize_option_f64, serialize_u64, serialize_vec_bytes20, serialize_vec_u8, WrappedBytes32,
    WrappedI128, WrappedU32, WrappedU64,
};
use crate::tx::{TxType, VertexTx};
use ethers::types::H160;
use ethers_core::types::Bytes;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TimestampOrTimestamps {
    Timestamp(WrappedU64),
    Timestamps(Vec<WrappedU64>),
}

impl TimestampOrTimestamps {
    pub fn timestamps(&self) -> Vec<u64> {
        match self {
            TimestampOrTimestamps::Timestamp(ts) => vec![ts.0],
            TimestampOrTimestamps::Timestamps(ts) => ts.iter().map(|t| t.0).collect(),
        }
    }

    pub fn is_single(&self) -> bool {
        matches!(self, Self::Timestamp(_))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Query {
    LiquidationFeed {},

    FundingRate {
        product_id: WrappedU32,
    },

    FundingRates {
        product_ids: Vec<WrappedU32>,
    },

    Candlesticks {
        product_id: WrappedU32,
        granularity: WrappedU32,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
    },

    Products {
        product_id: WrappedU32,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
        idx: Option<WrappedU64>,
    },

    ProductSnapshots {
        product_ids: Vec<WrappedU32>,
        max_time: Option<TimestampOrTimestamps>,
    },

    Events {
        subaccount: Option<WrappedBytes32>,
        product_ids: Option<Vec<WrappedU32>>,
        event_types: Option<Vec<TxType>>,
        max_time: Option<WrappedU64>,
        limit: Option<Limit>,
        idx: Option<WrappedU64>,
        desc: Option<bool>,
        isolated: Option<bool>,
    },

    Orders {
        subaccount: Option<WrappedBytes32>,
        product_ids: Option<Vec<WrappedU32>>,
        digests: Option<Vec<WrappedBytes32>>,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
        idx: Option<WrappedU64>,
        isolated: Option<bool>,
    },

    Summary {
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
        timestamp: Option<TimestampOrTimestamps>,
    },

    AccountSnapshots {
        subaccounts: Vec<WrappedBytes32>,
        timestamps: Vec<WrappedU64>,
        isolated: Option<bool>,
    },

    Matches {
        subaccount: Option<WrappedBytes32>,
        product_ids: Option<Vec<WrappedU32>>,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
        idx: Option<WrappedU64>,
        isolated: Option<bool>,
    },

    Rewards {
        address: H160,
        start: Option<WrappedU32>,
        limit: Option<WrappedU32>,
    },

    MakerStatistics {
        epoch: WrappedU32,
        product_id: WrappedU32,
        interval: WrappedU64,
    },

    Subaccounts {
        address: Option<H160>,
        start: Option<WrappedU64>,
        limit: Option<WrappedU64>,
    },

    Price {
        product_id: WrappedU32,
    },

    PerpPrices {
        product_ids: Vec<WrappedU32>,
    },

    OraclePrice {
        product_ids: Vec<WrappedU32>,
    },

    UsdcPrice {},

    LinkedSignerRateLimit {
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
    },

    LinkedSigners {
        start_idx: Option<WrappedU64>,
        limit: Option<WrappedU32>,
    },

    IsolatedSubaccounts {
        subaccount: Option<[u8; 32]>,
        start_idx: Option<WrappedU64>,
        limit: Option<WrappedU32>,
    },

    MarketSnapshots {
        interval: Interval,
        product_ids: Option<Vec<WrappedU32>>,
    },

    EdgeMarketSnapshots {
        interval: Interval,
    },

    EdgeCandlesticks {
        product_id: WrappedU32,
        granularity: WrappedU32,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
    },

    ReferralCode {
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
    },

    InterestAndFunding {
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
        product_ids: Vec<WrappedU32>,
        max_idx: Option<WrappedU64>,
        max_time: Option<WrappedU64>,
        limit: WrappedU32,
    },

    VrtxMerkleProofs {
        address: H160,
    },

    // TODO: remove this after FE uses the new one.
    ArbMerkleProofs {
        address: H160,
    },

    // TODO: remove this after FE uses the new one.
    ArbRewards {
        address: H160,
    },

    FoundationRewardsMerkleProofs {
        address: H160,
    },

    FoundationTakerRewards {
        address: H160,
        start: Option<WrappedU32>,
        limit: Option<WrappedU32>,
    },

    Signatures {
        digests: Vec<WrappedBytes32>,
    },

    TakerRewards {
        address: H160,
        start: Option<WrappedU32>,
        limit: Option<WrappedU32>,
    },

    InitialDropConditions {
        address: H160,
    },

    BlitzPoints {
        address: H160,
    },

    BlastPoints {
        address: H160,
    },

    BlitzPointsLeaderboard {
        epoch: u32,
        start: Option<u32>,
        limit: Option<u32>,
    },

    Leaderboard {
        contest_id: WrappedU32,
        rank_type: LeaderboardType,
        start: Option<WrappedU64>,
        limit: Option<WrappedU64>,
    },

    LeaderboardRank {
        contest_ids: Vec<WrappedU32>,
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
    },

    LeaderboardContests {
        contest_ids: Vec<WrappedU32>,
    },

    LeaderboardRegistration {
        #[serde(
            deserialize_with = "deserialize_bytes32",
            serialize_with = "serialize_bytes32"
        )]
        subaccount: [u8; 32],
        contest_id: WrappedU32,
        update_registration: Option<UpdateLeaderboardRegistration>,
    },

    FastWithdrawalSignature {
        idx: WrappedU64,
    },

    ActiveUsers {
        start_time: WrappedU64,
        end_time: WrappedU64,
        api_key: String,
    },

    Ratelimit {
        ip: Option<String>,
        wallet: Option<String>,
        api_key: String,
    },

    SonicPoints {
        address: H160,
    },

    SonicPointsLeaderboard {
        start: Option<u32>,
        limit: Option<u32>,
    },

    StakingV2ModifyStake {},

    StakingV2PoolSnapshots {
        interval: Interval,
    },

    StakingV2TopStakers {
        limit: Option<u32>,
    },

    FoundationTokenIncentivesSnapshots {
        interval: Interval,
    },

    VrtxSupplySnapshots {
        interval: Interval,
    },

    VlpSnapshots {
        interval: Option<Interval>,
        idx: Option<WrappedU64>,
        max_time: Option<WrappedU64>,
        limit: Option<WrappedU32>,
    },

    TxHashes {
        idxs: Vec<WrappedU64>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxHash {
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub submission_idx: i64,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub length: i64,
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub tx_hash: [u8; 32],
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TxHashesResponse {
    pub tx_hashes: Vec<Option<TxHash>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VlpSnapshot {
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub submission_idx: i64,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub timestamp: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_volume: i128,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub cumulative_trades: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_mint_usdc: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_burn_usdc: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_pnl: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub oracle_price_x18: i128,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub depositors: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub tvl: i128,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VlpSnapshotsResponse {
    pub snapshots: Vec<VlpSnapshot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VrtxSupplySnapshot {
    pub timestamp: u64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub vrtx_oracle_price: i128,
    pub cumulative_incentives: f64,
    pub cumulative_lba: f64,
    pub cumulative_ecosystem_supply: f64,
    pub cumulative_treasury_supply: f64,
    pub cumulative_investors_supply: f64,
    pub cumulative_team_supply: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VrtxSupplySnapshotsResponse {
    pub snapshots: Vec<VrtxSupplySnapshot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoundationTokenIncentivesSnapshot {
    pub timestamp: u64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_foundation_token_incentives: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub foundation_token_oracle_price: i128,
    pub foundation_token_product_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoundationTokenIncentivesSnapshotsResponse {
    pub snapshots: HashMap<u64, Vec<FoundationTokenIncentivesSnapshot>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StakingV2PoolSnapshot {
    pub timestamp: u64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_staked: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_unstaked: i128,
    pub number_of_stakers: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StakingV2PoolSnapshotsResponse {
    pub snapshots: Vec<StakingV2PoolSnapshot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Staker {
    pub address: H160,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub stake_amount: i128,
    pub pool_share: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StakingV2TopTradersResponse {
    pub stakers: Vec<Staker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidatableAccount {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub update_time: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Balance {
    Spot(SpotBalance),
    Perp(PerpBalance),
    Pending, // just something to fill the struct with
}

impl Balance {
    pub fn amount(&self) -> i128 {
        match self {
            Balance::Spot(spot) => spot.balance.amount,
            Balance::Perp(perp) => perp.balance.amount,
            Balance::Pending => panic!("Pending balance has no amount"),
        }
    }

    pub fn lp_amount(&self) -> i128 {
        match self {
            Balance::Spot(spot) => spot.lp_balance.amount,
            Balance::Perp(perp) => perp.lp_balance.amount,
            Balance::Pending => panic!("Pending balance has no amount"),
        }
    }

    pub fn quote_amount(&self) -> i128 {
        match self {
            Balance::Spot(_) => panic!("Spot balance has no quote amount"),
            Balance::Perp(perp) => perp.balance.v_quote_balance,
            Balance::Pending => panic!("Pending balance has no quote amount"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Limit {
    Raw(WrappedU32),
    Txs(WrappedU32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub product_id: u32,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub submission_idx: i64,
    pub event_type: TxType,
    pub isolated: bool,
    pub isolated_product_id: Option<u32>,
    pub pre_balance: Balance,
    pub post_balance: Balance,
    pub product: Product,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_interest_unrealized: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_interest_cumulative: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_funding_unrealized: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_funding_cumulative: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_unrealized: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_cumulative: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_lp_unrealized: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_lp_cumulative: i128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tx {
    pub tx: VertexTx,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub submission_idx: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsResponse {
    pub events: Vec<Event>,
    pub txs: Vec<Tx>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Product {
    Spot(SpotProduct),
    Perp(PerpProduct),
    Pending,
}

impl Product {
    pub fn spot(&self) -> SpotProduct {
        match self {
            Product::Spot(product) => product.clone(),
            _ => panic!("Product::spot() called on non-spot product"),
        }
    }
    pub fn perp(&self) -> PerpProduct {
        match self {
            Product::Perp(product) => product.clone(),
            _ => panic!("Product::perp() called on non-perp product"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductResponse {
    pub products: Vec<Product>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub digest: [u8; 32],
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub product_id: i32,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub submission_idx: i64,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub last_fill_submission_idx: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub amount: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub price_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub base_filled: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub quote_filled: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub fee: i128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub expiration: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub nonce: u64,
    pub isolated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub digest: [u8; 32],
    pub signature: Bytes,
    pub signer: H160,
    pub is_linked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureResponse {
    pub signatures: Vec<Signature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candlestick {
    pub product_id: i32,
    pub granularity: i32,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub submission_idx: i64,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub timestamp: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub open_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub high_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub low_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub close_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub volume: i128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandlesticksResponse {
    pub candlesticks: Vec<Candlestick>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ProductSnapshot {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub submission_idx: u64,
    pub product: Product,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ProductSnapshotsResponse {
    TsToProductSnapshots(HashMap<WrappedU64, HashMap<u32, ProductSnapshot>>),
    ProductSnapshots(HashMap<u32, ProductSnapshot>),
}

impl ProductSnapshotsResponse {
    pub fn product_snapshots(self) -> Result<HashMap<u32, ProductSnapshot>> {
        match self {
            ProductSnapshotsResponse::ProductSnapshots(ps) => Ok(ps),
            ProductSnapshotsResponse::TsToProductSnapshots(_) => {
                Err(eyre!("expected product_snapshots response"))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductsResponse {
    pub products: Vec<ProductSnapshot>,
    pub txs: Vec<Tx>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingRateResponse {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub funding_rate_x18: i64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalReward {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub reward_coefficient: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub q_scores: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_fees: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_fees: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_tokens: f64,
    pub uptimes: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressReward {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub q_score: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub sum_q_min: f64,
    pub uptime: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volume: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volume: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_fee: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_fee: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_referral_fee: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_referral_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub rebates: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reward {
    pub epoch: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub start_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub period: u64,
    pub address_rewards: Vec<AddressReward>,
    pub global_rewards: Vec<GlobalReward>,
    pub num_eligible_addresses: u32,
}

pub type RewardResponse = Reward;

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardsResponse {
    pub rewards: Vec<RewardResponse>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub total_referrals: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TakerReward {
    pub epoch: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_referral_tokens: f64,
}

pub type TakerRewardResponse = TakerReward;

#[derive(Serialize, Deserialize, Debug)]
pub struct TakerRewardsResponse {
    pub taker_rewards: Vec<TakerRewardResponse>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub total_referrals: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveUsersResponse {
    #[serde(
        serialize_with = "serialize_vec_bytes20",
        deserialize_with = "deserialize_vec_bytes20"
    )]
    pub addresses: Vec<[u8; 20]>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RatelimitResponse {
    pub description: Option<String>,
    pub ip: Option<String>,
    pub wallet: Option<String>,
    pub gateway_query: Option<[u64; 2]>,
    pub gateway_execute: Option<[u64; 2]>,
    pub archive_query: Option<[u64; 2]>,
    pub subscriptions: Option<u64>,
    pub authenticated_subscriptions: Option<u64>,
    pub private_gateway_access: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoundationGlobalReward {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_fees: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_fees: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoundationAddressReward {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volume: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_fee: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_tokens: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_fee: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoundationTakerReward {
    pub week: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub start_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub period: u64,
    pub address_rewards: Vec<FoundationAddressReward>,
    pub global_rewards: Vec<FoundationGlobalReward>,
}

pub type FoundationTakerRewardResponse = FoundationTakerReward;
pub type ArbRewardResponse = FoundationTakerReward;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArbRewardsResponse {
    pub arb_rewards: Vec<ArbRewardResponse>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoundationTakerRewardsResponse {
    pub foundation_taker_rewards: Vec<FoundationTakerRewardResponse>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialDropConditionsResponse {
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub amount: f64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub deadline: u64,
    pub account_value_reached: bool,
    pub perp_trades_done: bool,
    pub tweeted: bool,
    pub claimed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase2BlitzPoints {
    pub epoch: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub start_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub period: u64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub trading_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub referral_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volumes: f64,
    pub rank: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlitzPointsResponse {
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub initial_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub trading_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub referral_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volumes: f64,
    pub users_referred: u64,
    pub phase2_points: Vec<Phase2BlitzPoints>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SonicPointsResponse {
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub trading_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub referral_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volumes: f64,
    pub users_referred: u64,
    pub rank: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlastPointsResponse {
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub gold: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointsLeaderboardPosition {
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub address: [u8; 20],
    pub rank: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub trading_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub referral_points: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub taker_volumes: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_volumes: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointsLeaderboardResponse {
    pub positions: Vec<PointsLeaderboardPosition>,
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EventsOrTsToEvents {
    Events(Vec<Event>),
    TsToEvents(HashMap<WrappedU64, Vec<Event>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryResponse {
    pub events: EventsOrTsToEvents,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Match {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub digest: [u8; 32],
    pub order: eip712_structs::Order,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub base_filled: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub quote_filled: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub fee: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub sequencer_fee: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_fee: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_base_filled: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_quote_filled: i128,
    pub pre_balance: MatchBalances,
    pub post_balance: MatchBalances,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_unrealized: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub net_entry_cumulative: i128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub submission_idx: u64,
    pub isolated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatchBalances {
    pub base: Balance,
    pub quote: Option<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchesResponse {
    pub matches: Vec<Match>,
    pub txs: Vec<Tx>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subaccount {
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub id: u64,
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubaccountsResponse {
    pub subaccounts: Vec<Subaccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerpPriceResponse {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub index_price_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub mark_price_x18: i128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OraclePrice {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub oracle_price_x18: i128,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OraclePriceResponse {
    pub prices: Vec<OraclePrice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsdcPriceResponse {
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub price_x18: i128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MakerStatistic {
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub timestamp: u64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_fee: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub uptime: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub sum_q_min: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub q_score: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub maker_share: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub expected_maker_reward: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MakerInfo {
    pub address: H160,
    pub data: Vec<MakerStatistic>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MakerStatisticsResponse {
    pub reward_coefficient: f64,
    pub makers: Vec<MakerInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedSignerRateLimitResponse {
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub remaining_tx: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub wait_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub total_tx_limit: u64,
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub signer: [u8; 20],
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct LinkedSigner {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub linked: [u8; 20],
    pub submission_idx: i64,
    pub update_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedSignerResponse {
    pub events: Vec<LinkedSigner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsolatedSubaccount {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub product_id: i32,
    pub submission_idx: i64,
    pub update_time: i64,
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub isolated_subaccount: [u8; 32],
    pub status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsolatedSubaccountsResponse {
    pub isolated_subaccounts: Vec<IsolatedSubaccount>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MarketSnapshotsResponse {
    pub snapshots: Vec<MarketSnapshotData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EdgeMarketSnapshotsResponse {
    pub snapshots: HashMap<u64, Vec<MarketSnapshotData>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MarketSnapshotData {
    pub timestamp: i64,
    pub cumulative_users: i64,
    pub daily_active_users: i64,
    pub cumulative_trades: HashMap<i32, i64>,
    pub cumulative_volumes: HashMap<i32, WrappedI128>,
    pub cumulative_trade_sizes: HashMap<i32, WrappedI128>,
    pub cumulative_taker_fees: HashMap<i32, WrappedI128>,
    pub cumulative_sequencer_fees: HashMap<i32, WrappedI128>,
    pub cumulative_maker_fees: HashMap<i32, WrappedI128>,
    pub cumulative_liquidation_amounts: HashMap<i32, WrappedI128>,
    pub open_interests: HashMap<i32, WrappedI128>,
    pub total_deposits: HashMap<i32, WrappedI128>,
    pub total_borrows: HashMap<i32, WrappedI128>,
    pub funding_rates: HashMap<i32, WrappedI128>,
    pub deposit_rates: HashMap<i32, WrappedI128>,
    pub borrow_rates: HashMap<i32, WrappedI128>,
    pub cumulative_inflows: HashMap<i32, WrappedI128>,
    pub cumulative_outflows: HashMap<i32, WrappedI128>,
    pub oracle_prices: HashMap<i32, WrappedI128>,
    pub tvl: WrappedI128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductMarketActivity {
    pub product_id: i32,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_volume_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub volume_24hr_x18: i128,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub cumulative_trades: i64,
    #[serde(serialize_with = "serialize_i64", deserialize_with = "deserialize_i64")]
    pub trades_24hr: i64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_maker_fees_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub maker_fees_24hr_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub cumulative_taker_fees_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub taker_fees_24hr_x18: i128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketActivityResponse {
    pub markets: Vec<ProductMarketActivity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferralCodeResponse {
    pub referral_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub product_id: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub idx: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub timestamp: u64,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub amount: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub balance_amount: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub rate_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub oracle_price_x18: i128,
    pub isolated: bool,
    pub isolated_product_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterestAndFundingTicksResponse {
    pub interest_payments: Vec<Payment>,
    pub funding_payments: Vec<Payment>,
    pub next_idx: Option<WrappedU64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryV2 {
    Tickers(TickersParams),
    Contracts(ContractsParams),
    Trades(TradesParams),
    Vrtx(VrtxParams),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickersParams {
    pub market: Option<String>,
    pub edge: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractsParams {
    pub edge: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradesParams {
    pub ticker_id: String,
    pub max_trade_id: Option<WrappedU64>,
    pub limit: Option<WrappedU32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VrtxParams {
    pub q: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VrtxResponse {
    TotalSupply(f64),
    CirculatingSupply(f64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerResponse {
    pub ticker_id: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub last_price: f64,
    pub base_volume: f64,
    pub quote_volume: f64,
    pub price_change_percent_24h: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerpContractResponse {
    pub ticker_id: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub last_price: f64,
    pub base_volume: f64,
    pub quote_volume: f64,
    pub product_type: String,
    pub contract_price: f64,
    pub contract_price_currency: String,
    pub open_interest: f64,
    pub open_interest_usd: f64,
    pub index_price: f64,
    pub mark_price: f64,
    pub funding_rate: f64,
    pub next_funding_rate_timestamp: u64,
    pub price_change_percent_24h: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub ticker_id: String,
    pub trade_id: u64,
    pub price: f64,
    pub base_filled: f64,
    pub quote_filled: f64,
    pub timestamp: u64,
    pub trade_type: String,
}

pub type TradesResponse = Vec<Trade>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleProof {
    pub total_amount: String,
    pub proof: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerkleProofsResponse {
    pub merkle_proofs: Vec<MerkleProof>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountSnapshotsResponse {
    pub snapshots: HashMap<WrappedBytes32, HashMap<WrappedU64, Vec<Event>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Interval {
    pub count: u64,
    pub granularity: u64,
    pub max_time: Option<WrappedU64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LeaderboardType {
    PNL,
    ROI,
}

impl LeaderboardType {
    pub fn to_str(&self) -> &str {
        match self {
            LeaderboardType::PNL => "pnl",
            LeaderboardType::ROI => "roi",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardPosition {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub contest_id: u32,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub pnl: f64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub pnl_rank: u64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub roi: f64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub roi_rank: u64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub account_value: f64,
    #[serde(
        serialize_with = "serialize_option_f64",
        deserialize_with = "deserialize_option_f64"
    )]
    pub volume: Option<f64>,
    #[serde(
        serialize_with = "serialize_option_f64",
        deserialize_with = "deserialize_option_f64"
    )]
    pub staked_vrtx: Option<f64>,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardContest {
    pub contest_id: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub start_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub end_time: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub timeframe: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub count: u64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub threshold: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub volume_threshold: f64,
    #[serde(serialize_with = "serialize_f64", deserialize_with = "deserialize_f64")]
    pub staking_threshold: f64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub last_updated: u64,
    pub product_ids: Vec<u32>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardResponse {
    pub positions: Vec<LeaderboardPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardRankResponse {
    pub positions: HashMap<String, LeaderboardPosition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardContestsResponse {
    pub contests: Vec<LeaderboardContest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardRegistration {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub contest_id: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardRegistrationResponse {
    pub registration: Option<LeaderboardRegistration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateLeaderboardRegistration {
    pub tx: LeaderboardAuthentication,
    #[serde(
        serialize_with = "serialize_vec_u8",
        deserialize_with = "deserialize_vec_u8"
    )]
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastWithdrawalSignatureResponse {
    pub idx: WrappedU64,
    pub tx: WithdrawCollateral,
    pub tx_bytes: Bytes,
    pub signatures: Vec<Bytes>,
}
