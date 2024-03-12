use crate::eip712_structs::{Cancellation, ListTriggerOrders};
use crate::eip712_structs::{CancellationProducts, Order};
use crate::engine::{ExecuteResponse, PlaceOrder, Status};
use crate::serialize_utils::{deserialize_i128, serialize_i128, WrappedBytes32};
use ethers::types::{Bytes, H256};
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelReason {
    UserRequested,
    LinkedSignerChanged,
    Expired,
    AccountHealth,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderStatus {
    Pending,
    Triggering,
    Triggered(ExecuteResponse),
    Cancelled(CancelReason),
    InternalError(String),
}

impl TriggerOrderStatus {
    pub fn pending(&self) -> bool {
        match self {
            TriggerOrderStatus::Pending => true,
            _ => false,
        }
    }

    pub fn byte(&self) -> i32 {
        match self {
            TriggerOrderStatus::Pending => 0,
            TriggerOrderStatus::Triggering => 1,
            TriggerOrderStatus::Triggered(_) => 2,
            TriggerOrderStatus::Cancelled(_) => 3,
            TriggerOrderStatus::InternalError(_) => 4,
        }
    }

    pub fn byte_from_str(s: &str) -> Result<i32> {
        match s {
            "pending" => Ok(0),
            "triggering" => Ok(1),
            "triggered" => Ok(2),
            "cancelled" => Ok(3),
            "internal_error" => Ok(4),
            _ => Err(eyre::eyre!("Invalid status string")),
        }
    }

    pub fn data(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_status_data(data: &[u8]) -> Self {
        serde_json::from_slice(data).unwrap()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TriggerCriteria {
    // on the oracle price
    PriceAbove(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
    PriceBelow(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),

    LastPriceAbove(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
    LastPriceBelow(
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        i128,
    ),
}

impl TriggerCriteria {
    pub fn byte(&self) -> i32 {
        match self {
            TriggerCriteria::PriceAbove(_) => 0,
            TriggerCriteria::PriceBelow(_) => 1,
            TriggerCriteria::LastPriceAbove(_) => 2,
            TriggerCriteria::LastPriceBelow(_) => 3,
        }
    }

    pub fn price(&self) -> String {
        let p = match self {
            TriggerCriteria::PriceAbove(price) => *price,
            TriggerCriteria::PriceBelow(price) => *price,
            TriggerCriteria::LastPriceAbove(price) => *price,
            TriggerCriteria::LastPriceBelow(price) => *price,
        };
        format!("{:0>40}", p)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PlaceTriggerOrder {
    pub order: Order,
    pub signature: Bytes,
    pub product_id: u32,
    pub spot_leverage: Option<bool>,
    pub trigger: TriggerCriteria,
    pub digest: Option<H256>,
    pub id: Option<u64>,
}

impl PlaceTriggerOrder {
    pub fn to_place_order(&self) -> PlaceOrder {
        PlaceOrder {
            order: self.order.clone(),
            signature: self.signature.to_vec(),
            product_id: self.product_id,
            digest: None,
            id: self.id,
            spot_leverage: self.spot_leverage,
        }
    }

    pub fn order_data(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_order_data(data: &[u8]) -> Self {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub struct PlaceTriggerOrderOld {
            pub order: Order,
            pub signature: Bytes,
            pub product_id: u32,
            pub spot_leverage: Option<bool>,
            pub trigger: TriggerCriteria,
            pub digest: Option<H256>,
        }

        match bincode::deserialize(data) {
            Ok(p) => p,
            Err(_) => {
                let p_old = bincode::deserialize::<PlaceTriggerOrderOld>(data).unwrap();
                PlaceTriggerOrder {
                    order: p_old.order,
                    signature: p_old.signature,
                    product_id: p_old.product_id,
                    spot_leverage: p_old.spot_leverage,
                    trigger: p_old.trigger,
                    digest: p_old.digest,
                    id: None,
                }
            }
        }
    }

    pub fn expiration(&self) -> u64 {
        self.order.expiration()
    }

    pub fn is_bid(&self) -> bool {
        self.order.amount.is_positive()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
// #[ts(export)]·
// #[ts(export_to = "tsBindings/msg/")]
pub enum Execute {
    PlaceOrder(PlaceTriggerOrder),
    CancelOrders {
        tx: Cancellation,
        signature: Bytes,
    },
    CancelProductOrders {
        tx: CancellationProducts,
        signature: Bytes,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Query {
    ListTriggerOrders {
        tx: ListTriggerOrders,
        signature: Bytes,
        product_id: Option<u32>,
        pending: bool,
        max_update_time: Option<u64>,
        max_digest: Option<WrappedBytes32>,
        limit: Option<u32>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TriggerOrderInfo {
    pub order: PlaceTriggerOrder,
    pub status: TriggerOrderStatus,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTriggerOrdersResponse {
    pub orders: Vec<TriggerOrderInfo>,
}

impl ListTriggerOrdersResponse {
    pub fn contains_digest(&self, digest: H256) -> bool {
        self.orders
            .iter()
            .any(|order| order.order.digest == Some(digest))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum QueryResponseData {
    ListTriggerOrders(ListTriggerOrdersResponse),
    Error(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
