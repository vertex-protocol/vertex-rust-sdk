use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
// #![allow(dead_code, clippy::blacklisted_name)]
use crate::bindings::querier::{
    BookInfo, HealthInfo, LegacyRisk, PerpBalance, PerpProduct, ProductInfo, SpotBalance,
    SubaccountInfo,
};
use crate::bindings::spot_engine;
use crate::eip712_structs::{
    BurnLp, BurnVlp, Cancellation, CancellationProducts, IsolatedOrder, LinkSigner,
    LiquidateSubaccount, MintLp, MintVlp, Order, TransferQuote, WithdrawCollateral,
};
use crate::math::f64_to_x18;
use crate::product::Product;
use crate::serialize_utils::{
    deserialize_bytes20, deserialize_bytes32, deserialize_i128, deserialize_nested_vec_i128,
    deserialize_option_bytes32, deserialize_option_vec_u8, deserialize_u64,
    deserialize_vec_bytes20, deserialize_vec_i128, deserialize_vec_u8,
    serialize_bytes20, serialize_bytes32, serialize_i128, serialize_nested_vec_i128,
    serialize_option_bytes32, serialize_option_vec_u8, serialize_u64, serialize_vec_bytes20, serialize_vec_i128, serialize_vec_u8, str_or_u32, WrappedI128,
};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
// use ts_rs::TS;

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[archive(check_bytes)]
pub enum Direction {
    Long,
    Short,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductDelta {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub amount_delta: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub v_quote_delta: i128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Txn {
    MintLp(spot_engine::MintLpCall),
    BurnLp(spot_engine::BurnLpCall),
    ApplyDelta(ProductDelta),
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msg/")]
pub enum Query {
    Status {},

    Contracts {},

    Nonces {
        #[serde(
            serialize_with = "serialize_bytes20",
            deserialize_with = "deserialize_bytes20"
        )]
        address: [u8; 20],
    },

    LinkedSigner {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        subaccount: [u8; 32],
    },

    SubaccountInfo {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        subaccount: [u8; 32],
        txns: Option<String>,
    },

    AllProducts {},

    EdgeAllProducts {},

    MarketPrice {
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
    },

    MarketPrices {
        product_ids: Vec<u32>,
    },

    Order {
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        digest: [u8; 32],
    },

    Orders {
        // #[ts(type = "string")]
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        product_ids: Vec<u32>,
    },

    ValidateOrder {
        product_id: u32,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        order: Vec<u8>,
    },

    FeeRates {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
    },

    SubaccountOrders {
        // #[ts(type = "string")]
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
    },

    MarketLiquidity {
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
        #[serde(deserialize_with = "str_or_u32")]
        depth: u32,
    },

    MaxOrderSize {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        price_x18: i128,
        direction: Direction,
        spot_leverage: Option<String>,
    },

    MaxWithdrawable {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
        spot_leverage: Option<String>,
    },

    MaxLpMintable {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        #[serde(deserialize_with = "str_or_u32")]
        product_id: u32,
        spot_leverage: Option<String>,
    },

    MaxVlpMintable {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        sender: [u8; 32],
        spot_leverage: Option<String>,
    },

    IsolatedPositions {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        subaccount: [u8; 32],
    },

    HealthGroups {},

    Insurance {},

    Versions {},

    Symbols {
        product_ids: Option<Vec<u32>>,
        product_type: Option<String>,
    },

    MinDepositRates {},
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
pub struct PlaceOrder {
    pub order: Order,
    #[serde(
        serialize_with = "serialize_vec_u8",
        deserialize_with = "deserialize_vec_u8"
    )]
    pub signature: Vec<u8>,
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_option_bytes32",
        deserialize_with = "deserialize_option_bytes32"
    )]
    #[serde(default)]
    pub digest: Option<[u8; 32]>,
    // serde ignore if none
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<u64>,
    pub spot_leverage: Option<bool>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
pub struct PlaceIsolatedOrder {
    pub isolated_order: IsolatedOrder,
    #[serde(
        serialize_with = "serialize_vec_u8",
        deserialize_with = "deserialize_vec_u8"
    )]
    pub signature: Vec<u8>,
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_option_bytes32",
        deserialize_with = "deserialize_option_bytes32"
    )]
    // note that `digest` here should be obtained from `isolated_order.to_order()`!!!
    #[serde(default)]
    pub digest: Option<[u8; 32]>,
    // serde ignore if none
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<u64>,
    pub borrow_margin: Option<bool>,
}

impl PlaceIsolatedOrder {
    pub fn to_place_order(&self) -> PlaceOrder {
        PlaceOrder {
            order: self.isolated_order.to_order(),
            signature: self.signature.clone(),
            product_id: self.product_id,
            digest: self.digest,
            id: self.id,
            spot_leverage: None,
        }
    }
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub bytecode: String,
    pub deployed_bytecode: String,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msg/")]
pub enum Execute {
    LiquidateSubaccount {
        tx: LiquidateSubaccount,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },
    WithdrawCollateral {
        tx: WithdrawCollateral,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
        spot_leverage: Option<bool>,
        sequencer_risk_check: Option<bool>,
    },
    MintLp {
        tx: MintLp,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
        spot_leverage: Option<bool>,
    },
    BurnLp {
        tx: BurnLp,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },
    PlaceOrder(PlaceOrder),
    PlaceIsolatedOrder(PlaceIsolatedOrder),
    CancelOrders {
        tx: Cancellation,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },
    CancelProductOrders {
        tx: CancellationProducts,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
        #[serde(
            serialize_with = "serialize_option_bytes32",
            deserialize_with = "deserialize_option_bytes32"
        )]
        #[serde(default)]
        digest: Option<[u8; 32]>,
    },
    LinkSigner {
        tx: LinkSigner,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },
    CancelAndPlace {
        cancel_tx: Cancellation,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        cancel_signature: Vec<u8>,
        place_order: PlaceOrder,
    },
    SubmitPrivateBatch {
        orders: Vec<[PlaceOrder; 2]>,
    },

    TransferQuote {
        tx: TransferQuote,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },

    MintVlp {
        tx: MintVlp,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
        spot_leverage: Option<bool>,
    },
    BurnVlp {
        tx: BurnVlp,
        #[serde(
            serialize_with = "serialize_vec_u8",
            deserialize_with = "deserialize_vec_u8"
        )]
        signature: Vec<u8>,
    },
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[archive(check_bytes)]
pub enum EngineMessage {
    Query(Query),
    Execute(Execute),
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
pub struct LabeledEngineMessage {
    pub chain_id: u64,
    pub msg: EngineMessage,
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct Config {
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub token: [u8; 20],
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub interest_inflection_util_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub interest_floor_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub interest_small_cap_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub interest_large_cap_x18: i128,
}

impl From<crate::bindings::querier::Config> for Config {
    fn from(config: crate::bindings::querier::Config) -> Self {
        Config {
            token: config.token.into(),
            interest_inflection_util_x18: config.interest_inflection_util_x18,
            interest_floor_x18: config.interest_floor_x18,
            interest_small_cap_x18: config.interest_small_cap_x18,
            interest_large_cap_x18: config.interest_large_cap_x18,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct SpotProduct {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub oracle_price_x18: i128,
    pub risk: LegacyRisk,
    pub config: Config,
    pub state: crate::bindings::spot_engine::State,
    pub lp_state: crate::bindings::spot_engine::LpState,
    pub book_info: BookInfo,
}

impl From<crate::bindings::querier::SpotProduct> for SpotProduct {
    fn from(spot_product: crate::bindings::querier::SpotProduct) -> Self {
        SpotProduct {
            product_id: spot_product.product_id,
            oracle_price_x18: spot_product.oracle_price_x18,
            risk: spot_product.risk,
            config: Config::from(spot_product.config),
            state: spot_product.state,
            lp_state: spot_product.lp_state,
            book_info: spot_product.book_info,
        }
    }
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct AllProductsResponse {
    pub spot_products: ::std::vec::Vec<SpotProduct>,
    pub perp_products: ::std::vec::Vec<PerpProduct>,
}

impl From<ProductInfo> for AllProductsResponse {
    fn from(product_info: ProductInfo) -> Self {
        AllProductsResponse {
            spot_products: product_info
                .spot_products
                .into_iter()
                .map(SpotProduct::from)
                .collect(),
            perp_products: product_info.perp_products,
        }
    }
}
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct EdgeAllProductsResponse {
    pub edge_all_products: HashMap<u64, AllProductsResponse>,
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct SubaccountInfoResponse {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub exists: bool,
    pub healths: ::std::vec::Vec<HealthInfo>,
    #[serde(
        serialize_with = "serialize_nested_vec_i128",
        deserialize_with = "deserialize_nested_vec_i128"
    )]
    pub health_contributions: Vec<Vec<i128>>,
    pub spot_count: u32,
    pub perp_count: u32,
    pub spot_balances: ::std::vec::Vec<SpotBalance>,
    pub perp_balances: ::std::vec::Vec<PerpBalance>,
    pub spot_products: ::std::vec::Vec<SpotProduct>,
    pub perp_products: ::std::vec::Vec<PerpProduct>,
}

impl From<SubaccountInfo> for SubaccountInfoResponse {
    fn from(subaccount_info: SubaccountInfo) -> Self {
        SubaccountInfoResponse {
            subaccount: subaccount_info.subaccount,
            exists: subaccount_info.exists,
            healths: subaccount_info.healths,
            health_contributions: subaccount_info.health_contributions,
            spot_count: subaccount_info.spot_count,
            perp_count: subaccount_info.perp_count,
            spot_balances: subaccount_info.spot_balances,
            perp_balances: subaccount_info.perp_balances,
            spot_products: subaccount_info
                .spot_products
                .into_iter()
                .map(SpotProduct::from)
                .collect(),
            perp_products: subaccount_info.perp_products,
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct ContractsResponse {
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    // Should be fine to use u128 here.
    // see https://gist.github.com/rekmarks/a47bd5f2525936c4b8eee31a16345553
    pub chain_id: u64,
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub endpoint_addr: [u8; 20],
    #[serde(
        serialize_with = "serialize_vec_bytes20",
        deserialize_with = "deserialize_vec_bytes20"
    )]
    pub book_addrs: Vec<[u8; 20]>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct NoncesResponse {
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub tx_nonce: u64,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub order_nonce: u64,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct LinkedSignerResponse {
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub linked_signer: [u8; 20],
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MarketPriceResponse {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub bid_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub ask_x18: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MarketPricesResponse {
    pub market_prices: Vec<MarketPriceResponse>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct OrderResponse {
    pub product_id: u32,
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub price_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub amount: i128,
    pub expiration: String,
    pub order_type: String,
    pub nonce: String,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub unfilled_amount: i128,
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub digest: [u8; 32],
    pub placed_at: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<WrappedI128>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct ValidateOrderResponse {
    pub product_id: u32,
    #[serde(
        serialize_with = "serialize_vec_u8",
        deserialize_with = "deserialize_vec_u8"
    )]
    pub order: Vec<u8>,
    pub valid: bool,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct FeeRatesResponse {
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub taker_fee_rates_x18: Vec<i128>,
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub maker_fee_rates_x18: Vec<i128>,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub liquidation_sequencer_fee: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub health_check_sequencer_fee: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub taker_sequencer_fee: i128,
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub withdraw_sequencer_fees: Vec<i128>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct SubaccountOrdersResponse {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    pub product_id: u32,
    pub orders: Vec<OrderResponse>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct ProductOrdersResponse {
    pub product_id: u32,
    pub orders: Vec<OrderResponse>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct OrdersResponse {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub sender: [u8; 32],
    pub product_orders: Vec<ProductOrdersResponse>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MaxOrderSizeResponse {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_order_size: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MaxWithdrawableResponse {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_withdrawable: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MaxLpMintableResponse {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_base_amount: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_quote_amount: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MaxVlpMintableResponse {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_quote_amount: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct HealthGroupsResponse {
    // #[ts(type = "string")]
    pub health_groups: Vec<(u32, u32)>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct InsuranceResponse {
    // #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub insurance: i128,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct SymbolsResponse {
    pub symbols: HashMap<String, SymbolsResponseData>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct SymbolsResponseData {
    #[serde(rename = "type")]
    pub product_type: String,
    pub product_id: u32,
    pub symbol: String,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub price_increment_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub size_increment: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub min_size: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub min_depth_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub max_spread_rate_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub maker_fee_rate_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub taker_fee_rate_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub long_weight_initial_x18: i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub long_weight_maintenance_x18: i128,
    pub max_open_interest_x18: Option<WrappedI128>,
}

impl SymbolsResponseData {
    pub fn placeholder() -> Self {
        Self {
            product_type: "placeholder".to_string(),
            ..Self::default()
        }
    }
}

impl From<&Product> for SymbolsResponseData {
    fn from(product: &Product) -> Self {
        let product_type = match product {
            Product::Spot { .. } => "spot",
            Product::Perp { .. } => "perp",
        }
        .to_string();
        let max_open_interest = product.max_open_interest().and_then(|oi| {
            if oi == 0.0 {
                None
            } else {
                Some(WrappedI128(f64_to_x18(oi)))
            }
        });

        match product {
            Product::Spot {
                symbol,
                long_weight_initial,
                long_weight_maintenance,
                size_increment,
                price_increment,
                min_size,
                product_id,
                ..
            }
            | Product::Perp {
                symbol,
                long_weight_initial,
                long_weight_maintenance,
                size_increment,
                price_increment,
                min_size,
                product_id,
                ..
            } => SymbolsResponseData {
                product_type,
                product_id: *product_id,
                symbol: symbol.clone(),
                price_increment_x18: f64_to_x18(*price_increment),
                min_size: f64_to_x18(*min_size),
                size_increment: f64_to_x18(*size_increment),
                long_weight_initial_x18: f64_to_x18(*long_weight_initial),
                long_weight_maintenance_x18: f64_to_x18(*long_weight_maintenance),
                max_open_interest_x18: max_open_interest,
                ..Self::default()
            },
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct MinDepositRate {
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub min_deposit_rate_x18: i128,
    pub product_id: u32,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct MinDepositRatesResponse {
    // product_id -> MinDepositRate
    pub min_deposit_rates: HashMap<String, MinDepositRate>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct IsolatedPosition {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub subaccount: [u8; 32],
    pub quote_balance: SpotBalance,
    pub base_balance: PerpBalance,
    pub quote_product: SpotProduct,
    pub base_product: PerpProduct,
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub quote_healths: Vec<i128>,
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub base_healths: Vec<i128>,
    pub healths: Vec<HealthInfo>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct IsolatedPositionsResponse {
    pub isolated_positions: Vec<IsolatedPosition>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct PriceLevel(
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub  i128,
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    // #[ts(type = "BigNumberish")]
    pub  i128,
);

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MarketLiquidityResponse {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub product_id: u32,
    #[serde(serialize_with = "serialize_u64", deserialize_with = "deserialize_u64")]
    pub timestamp: u64,
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Failure,
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
#[serde(rename_all = "snake_case")]
pub enum EngineStatus {
    Started,
    // just started, not syncing yet
    Active,
    // accepting incoming executes
    Stopping,
    // active engine is winding down
    Syncing,
    // currently syncing historical txs from the chain
    LiveSyncing,
    // done syncing historical txs; now
    // syncing live writes from primary engine
    Failed, // sequencer is in a failed state
}

impl From<EngineStatus> for u8 {
    fn from(status: EngineStatus) -> Self {
        match status {
            EngineStatus::Started => 0,
            EngineStatus::Active => 1,
            EngineStatus::Stopping => 2,
            EngineStatus::Syncing => 3,
            EngineStatus::LiveSyncing => 4,
            EngineStatus::Failed => 5,
        }
    }
}

impl From<u8> for EngineStatus {
    fn from(status: u8) -> Self {
        match status {
            0 => EngineStatus::Started,
            1 => EngineStatus::Active,
            2 => EngineStatus::Stopping,
            3 => EngineStatus::Syncing,
            4 => EngineStatus::LiveSyncing,
            5 => EngineStatus::Failed,
            _ => panic!("Invalid EngineStatus"),
        }
    }
}

impl From<EngineStatus> for AtomicU8 {
    fn from(status: EngineStatus) -> Self {
        AtomicU8::new(status.into())
    }
}

impl From<AtomicU8> for EngineStatus {
    fn from(status: AtomicU8) -> Self {
        EngineStatus::from(status.load(Ordering::Relaxed))
    }
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum QueryResponseData {
    StatusResponse(EngineStatus),
    Contracts(ContractsResponse),
    FeeRates(FeeRatesResponse),
    Nonces(NoncesResponse),
    LinkedSigner(LinkedSignerResponse),
    SubaccountInfo(SubaccountInfoResponse),
    MarketPrice(MarketPriceResponse),
    MarketPrices(MarketPricesResponse),
    Order(OrderResponse),
    Orders(OrdersResponse),
    ValidateOrder(ValidateOrderResponse),
    SubaccountOrders(SubaccountOrdersResponse),
    MarketLiquidity(MarketLiquidityResponse),
    AllProducts(AllProductsResponse),
    EdgeAllProducts(EdgeAllProductsResponse),
    MaxOrderSize(MaxOrderSizeResponse),
    MaxWithdrawable(MaxWithdrawableResponse),
    MaxLpMintable(MaxLpMintableResponse),
    MaxVlpMintable(MaxVlpMintableResponse),
    HealthGroups(HealthGroupsResponse),
    Insurance(InsuranceResponse),
    Symbols(SymbolsResponse),
    MinDepositRates(MinDepositRatesResponse),
    IsolatedPositions(IsolatedPositionsResponse),
    Error(String),
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
#[serde(rename_all = "snake_case")]
pub struct QueryResponse {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<QueryResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct ExecuteResponse {
    pub status: Status,
    #[serde(
        serialize_with = "serialize_option_vec_u8",
        deserialize_with = "deserialize_option_vec_u8"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub signature: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ExecuteResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[archive(check_bytes)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteResponseData {
    PlaceOrder(PlaceOrderResponse),
    CancelOrders(CancelOrdersResponse),
    CancelProductOrders(CancelOrdersResponse),
    PlaceIsolatedOrder(PlaceOrderResponse),
}

#[derive(
    Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct CancelOrdersResponse {
    pub cancelled_orders: Vec<OrderResponse>,
}

#[derive(
    Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct PlaceOrderResponse {
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub digest: [u8; 32],
}

#[derive(
    Archive, RkyvDeserialize, RkyvSerialize, Clone, Debug, Eq, PartialEq, Serialize, Deserialize,
)]
#[serde(untagged)]
#[archive(check_bytes)]
pub enum EngineResponse {
    Query(QueryResponse),
    Execute(ExecuteResponse),
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msg/")]
pub enum QueryV2 {
    Orderbook(OrderbookParams),

    Pairs(MarketPairsParams),

    Assets {},

    Apr {},
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
pub struct OrderbookParams {
    pub ticker_id: String,
    #[serde(deserialize_with = "str_or_u32")]
    pub depth: u32,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Clone, Serialize, Deserialize, Debug)]
#[archive(check_bytes)]
#[serde(rename_all = "snake_case")]
pub struct MarketPairsParams {
    pub market: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct OrderbookPriceLevel(pub f64, pub f64);

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct OrderbookResponse {
    pub ticker_id: String,
    pub bids: Vec<OrderbookPriceLevel>,
    pub asks: Vec<OrderbookPriceLevel>,
    pub timestamp: u64,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct MarketPair {
    pub ticker_id: String,
    pub base: String,
    pub quote: String,
}

pub type MarketPairsResponse = Vec<MarketPair>;

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize,
)]
#[archive(check_bytes)]
pub struct MarketApr {
    pub name: String,
    pub symbol: String,
    pub product_id: u32,
    pub deposit_apr: f64,
    pub borrow_apr: f64,
    pub tvl: f64,
}

pub type MarketsAprResponse = Vec<MarketApr>;

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive(check_bytes)]
// #[ts(export)]
// #[ts(export_to = "tsBindings/msgResponses/")]
pub struct Asset {
    pub product_id: u32,
    pub ticker_id: Option<String>,
    pub market_type: Option<String>,
    pub name: String,
    pub symbol: String,
    pub maker_fee: Option<f64>,
    pub taker_fee: Option<f64>,
    pub can_withdraw: bool,
    pub can_deposit: bool,
}

pub type AssetsResponse = Vec<Asset>;
