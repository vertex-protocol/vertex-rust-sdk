use crate::engine::{CancelOrdersResponse, ExecuteResponseData};
use eyre::{eyre, Result};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::utils::client_error::ClientError;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum VertexRestResponse<R> {
    Success(R),
    IPBlocked(CloudflareIPResponse),
    IndexerError(IndexerError),
    // This variant should be last, as it will match any response shape
    Unknown(Value),
}

impl<R: DeserializeOwned + Send> VertexRestResponse<R> {
    pub fn extract_response(self) -> Result<R> {
        match self {
            VertexRestResponse::Success(response) => Ok(response),
            VertexRestResponse::IndexerError(error) => Err(eyre!(error.error)),
            VertexRestResponse::IPBlocked(response) => {
                Err(eyre!(ClientError::IPBlocked(format!("{:?}", response))))
            }
            VertexRestResponse::Unknown(value) => Ok(serde_json::from_value(value)?),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct IndexerError {
    pub error: String,
    pub error_code: i32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CloudflareIPResponse {
    reason: String,
    blocked: bool,
}

#[doc(hidden)]
#[macro_export]
macro_rules! extract_response_data {
    ($response:expr, $resp_type:ty => $data_type:ty) => {
        if $response.status == Status::Failure {
            Err(eyre!(serde_json::to_string_pretty(&$response)?))
        } else {
            Ok($response.data as Option<$data_type>)
        }
    };
}

// this function exists since the enums can be deserialized interchangeably since they are untagged
// and have the same shape.
pub fn match_cancel_orders_response(
    execute_response_data: Option<ExecuteResponseData>,
) -> Result<Option<CancelOrdersResponse>> {
    match execute_response_data {
        Some(data) => match data {
            ExecuteResponseData::CancelOrders(response) => Ok(Some(response)),
            ExecuteResponseData::CancelProductOrders(response) => Ok(Some(response)),
            _ => Err(eyre!("Unexpected response type")),
        },
        None => Ok(None),
    }
}
