use ethers::types::{H160, H256, I256, U256};
use ethers_core::types::{Bytes, H512};
use eyre::Result;
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::{from_utf8, FromStr};

const MAP: &[u8] = "0123456789abcdef".as_bytes();

pub fn str_or_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU32<'a> {
        Str(&'a str),
        U32(u32),
    }

    Ok(match StrOrU32::deserialize(deserializer)? {
        StrOrU32::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrU32::U32(v) => v,
    })
}

pub fn u32_within_i32_range<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u32::deserialize(deserializer)?;
    if u32_in_i32_range(v) {
        Ok(v)
    } else {
        Err(D::Error::custom("u32 value too large"))
    }
}

fn u32_in_i32_range(v: u32) -> bool {
    v <= i32::MAX as u32
}

pub fn str_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        String(String),
        U64(u64),
    }

    let v = match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrU64::String(v) => v.parse().unwrap_or(0),
        StrOrU64::U64(v) => v,
    };
    if u64_in_i64_range(v) {
        Ok(v)
    } else {
        Err(D::Error::custom("u64 value too large"))
    }
}

fn u64_in_i64_range(v: u64) -> bool {
    v <= i64::MAX as u64
}

// why do we have this when there is str_or_u64 above?
// what's above allows us to take query inputs as u64s or strings; it works great for json
// because json is self-describing, but it won't work with bincode because bincode is not
// so on structs that are serialized in both json and bincode, we need to use this
// and just serialize all u64s as strings
pub fn serialize_u64<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u64::from_str(&s).map_err(|_| D::Error::custom("invalid u64 value"))
}

pub fn serialize_u128<S>(value: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse()
        .map_err(|_| D::Error::custom("invalid u128 value"))
}

pub fn serialize_i128<S>(value: &i128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_i128<'de, D>(deserializer: D) -> Result<i128, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse()
        .map_err(|_| D::Error::custom("invalid i128 value"))
}

pub fn serialize_i64<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(|_| D::Error::custom("invalid i64 value"))
}

#[derive(
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
)]
#[archive(check_bytes)]
pub struct WrappedI128(
    #[serde(
        serialize_with = "serialize_i128",
        deserialize_with = "deserialize_i128"
    )]
    pub i128,
);

pub fn serialize_vec_i128<S>(value: &Vec<i128>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        seq.serialize_element(&WrappedI128(*e))?;
    }
    seq.end()
}

struct VecI128Deserializer;

impl<'de> Visitor<'de> for VecI128Deserializer {
    type Value = Vec<i128>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("vector sequence.")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut ret = Vec::<i128>::new();
        while let Some(v) = seq.next_element::<WrappedI128>()? {
            ret.push(v.0);
        }
        Ok(ret)
    }
}

pub fn deserialize_vec_i128<'de, D>(deserializer: D) -> Result<Vec<i128>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(VecI128Deserializer)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WrappedVecI128(
    #[serde(
        serialize_with = "serialize_vec_i128",
        deserialize_with = "deserialize_vec_i128"
    )]
    pub Vec<i128>,
);

pub fn serialize_nested_vec_i128<S>(
    value: &Vec<Vec<i128>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        seq.serialize_element(&WrappedVecI128(e.clone()))?;
    }
    seq.end()
}

struct NestedVecI128Deserializer;

impl<'de> Visitor<'de> for NestedVecI128Deserializer {
    type Value = Vec<Vec<i128>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("nested vector sequence.")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut ret = Vec::<Vec<i128>>::new();
        while let Some(v) = seq.next_element::<WrappedVecI128>()? {
            ret.push(v.0);
        }
        Ok(ret)
    }
}

pub fn deserialize_nested_vec_i128<'de, D>(deserializer: D) -> Result<Vec<Vec<i128>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(NestedVecI128Deserializer)
}

pub fn serialize_u256<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(|_| D::Error::custom("invalid u256 value"))
}

pub fn serialize_i256<S>(value: &I256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_i256<'de, D>(deserializer: D) -> Result<I256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    I256::from_dec_str(&s).map_err(|_| D::Error::custom("invalid i256 value"))
}

pub fn serialize_bytes64<S>(value: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex = [0u8; 130];
    hex[0] = b'0';
    hex[1] = b'x';
    for i in 0..64 {
        hex[i * 2 + 2] = MAP[(value[i] >> 4) as usize];
        hex[i * 2 + 1 + 2] = MAP[(value[i] & 0xf) as usize];
    }
    serializer.serialize_str(from_utf8(&hex).unwrap())
}

pub fn deserialize_bytes64<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(H512::from_str(&s)
        .map_err(|_| D::Error::custom("invalid H256 value"))?
        .to_fixed_bytes())
}

pub fn serialize_bytes32<S>(value: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex = [0u8; 66];
    hex[0] = b'0';
    hex[1] = b'x';
    for i in 0..32 {
        hex[i * 2 + 2] = MAP[(value[i] >> 4) as usize];
        hex[i * 2 + 1 + 2] = MAP[(value[i] & 0xf) as usize];
    }
    serializer.serialize_str(from_utf8(&hex).unwrap())
}

pub fn deserialize_bytes32<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(H256::from_str(&s)
        .map_err(|_| D::Error::custom("invalid H256 value"))?
        .to_fixed_bytes())
}

pub fn serialize_bytes20<S>(value: &[u8; 20], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex = [0u8; 42];
    hex[0] = b'0';
    hex[1] = b'x';
    for i in 0..20 {
        hex[i * 2 + 2] = MAP[(value[i] >> 4) as usize];
        hex[i * 2 + 1 + 2] = MAP[(value[i] & 0xf) as usize];
    }
    serializer.serialize_str(from_utf8(&hex).unwrap())
}

pub fn deserialize_bytes20<'de, D>(deserializer: D) -> Result<[u8; 20], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(H160::from_str(&s)
        .map_err(|_| D::Error::custom("invalid H160 value"))?
        .to_fixed_bytes())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WrappedVecU8(
    #[serde(
        serialize_with = "serialize_vec_u8",
        deserialize_with = "deserialize_vec_u8"
    )]
    pub Vec<u8>,
);

pub fn serialize_option_vec_u8<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        serialize_vec_u8(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_option_vec_u8<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedVecU8>::deserialize(deserializer)
        .map(|opt_wrapped: Option<WrappedVecU8>| opt_wrapped.map(|wrapped: WrappedVecU8| wrapped.0))
}

pub fn serialize_vec_u8<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex = vec![0; value.len() * 2 + 2];
    hex[0] = b'0';
    hex[1] = b'x';
    for i in 0..value.len() {
        hex[i * 2 + 2] = MAP[(value[i] >> 4) as usize];
        hex[i * 2 + 1 + 2] = MAP[(value[i] & 0xf) as usize];
    }
    serializer.serialize_str(from_utf8(&hex).unwrap())
}

pub fn deserialize_vec_u8<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(Bytes::from_str(&s)
        .map_err(|_| D::Error::custom("invalid Bytes value"))?
        .to_vec())
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WrappedBytes32(
    #[serde(
        serialize_with = "serialize_bytes32",
        deserialize_with = "deserialize_bytes32"
    )]
    pub [u8; 32],
);

#[derive(
    Hash,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
pub struct WrappedBytes64(
    #[serde(
        serialize_with = "serialize_bytes64",
        deserialize_with = "deserialize_bytes64"
    )]
    pub [u8; 64],
);

#[derive(Hash, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
pub struct WrappedU64(
    #[serde(deserialize_with = "str_or_u64", serialize_with = "serialize_u64")] pub u64,
);

#[derive(Hash, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Copy)]
pub struct WrappedU32(#[serde(deserialize_with = "u32_within_i32_range")] pub u32);

impl WrappedU32 {
    pub fn convert_to_vec_i32(v: &[WrappedU32]) -> Vec<i32> {
        v.iter().map(|x| i32::from(*x)).collect()
    }

    pub fn wrap_vec_u32(v: &[u32]) -> Vec<WrappedU32> {
        v.iter().map(|x| WrappedU32(*x)).collect()
    }
}

impl From<WrappedU32> for i32 {
    fn from(wrapped: WrappedU32) -> Self {
        wrapped.0 as i32
    }
}

pub fn deserialize_option_bytes32<'de, D>(deserializer: D) -> Result<Option<[u8; 32]>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedBytes32>::deserialize(deserializer).map(
        |opt_wrapped: Option<WrappedBytes32>| opt_wrapped.map(|wrapped: WrappedBytes32| wrapped.0),
    )
}

pub fn serialize_option_bytes32<S>(
    value: &Option<[u8; 32]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        serialize_bytes32(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_option_bytes64<'de, D>(deserializer: D) -> Result<Option<[u8; 64]>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedBytes64>::deserialize(deserializer).map(
        |opt_wrapped: Option<WrappedBytes64>| opt_wrapped.map(|wrapped: WrappedBytes64| wrapped.0),
    )
}

pub fn serialize_option_bytes64<S>(
    value: &Option<[u8; 64]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        serialize_bytes64(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

struct VecBytes32Deserializer;

impl<'de> Visitor<'de> for VecBytes32Deserializer {
    type Value = Vec<[u8; 32]>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("vector sequence.")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut ret = Vec::<[u8; 32]>::new();
        while let Some(v) = seq.next_element::<H256>()? {
            ret.push(v.to_fixed_bytes());
        }
        Ok(ret)
    }
}

pub fn serialize_vec_bytes32<S>(value: &Vec<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        seq.serialize_element(&WrappedBytes32(*e))?;
    }
    seq.end()
}

pub fn deserialize_vec_bytes32<'de, D>(deserializer: D) -> Result<Vec<[u8; 32]>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(VecBytes32Deserializer)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WrappedBytes20(
    #[serde(
        serialize_with = "serialize_bytes20",
        deserialize_with = "deserialize_bytes20"
    )]
    pub [u8; 20],
);

struct VecBytes20Deserializer;

impl<'de> Visitor<'de> for VecBytes20Deserializer {
    type Value = Vec<[u8; 20]>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("vector sequence.")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut ret = Vec::<[u8; 20]>::new();
        while let Some(v) = seq.next_element::<H160>()? {
            ret.push(v.to_fixed_bytes());
        }
        Ok(ret)
    }
}

pub fn serialize_vec_bytes20<S>(value: &Vec<[u8; 20]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        seq.serialize_element(&WrappedBytes20(*e))?;
    }
    seq.end()
}

pub fn deserialize_vec_bytes20<'de, D>(deserializer: D) -> Result<Vec<[u8; 20]>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(VecBytes20Deserializer)
}

pub fn serialize_f64<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(|_| D::Error::custom("invalid f64 value"))
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct WrappedF64(
    #[serde(deserialize_with = "deserialize_f64", serialize_with = "serialize_f64")] pub f64,
);

pub fn serialize_option_f64<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        serialize_f64(value, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedF64>::deserialize(deserializer)
        .map(|opt_wrapped: Option<WrappedF64>| opt_wrapped.map(|wrapped: WrappedF64| wrapped.0))
}

pub fn str_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        I64(i64),
    }
    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrU64::I64(v) => v,
    })
}
