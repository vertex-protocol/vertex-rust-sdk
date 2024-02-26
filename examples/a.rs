use std::fs::read_to_string;

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use vertex_sdk::{
    bindings::querier::{HealthInfo, PerpBalance, PerpProduct, SpotBalance},
    engine::{QueryResponseData, SpotProduct},
    serialize_utils::{
        deserialize_bytes20, deserialize_bytes32, deserialize_i128, deserialize_nested_vec_i128,
        deserialize_option_bytes32, deserialize_option_vec_u8, deserialize_u128, deserialize_u64,
        deserialize_vec_bytes20, deserialize_vec_i128, deserialize_vec_u8, serialize_bytes20,
        serialize_bytes32, serialize_i128, serialize_nested_vec_i128, serialize_option_bytes32,
        serialize_option_vec_u8, serialize_u128, serialize_u64, serialize_vec_bytes20,
        serialize_vec_i128, serialize_vec_u8, str_or_u32,
    },
};

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

#[tokio::main]
async fn main() {
    let data = read_to_string("./examples/a.json").unwrap();
    // serde_json::from_str::<QueryResponseData>(&data).unwrap();
    let a = serde_json::from_str::<SubaccountInfoResponse>(&data).unwrap();
    println!("{:?}", a);
}
