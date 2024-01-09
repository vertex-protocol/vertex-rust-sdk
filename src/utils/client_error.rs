use eyre::{eyre, Report};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ClientError {
    #[error("Access from your IP is blocked. Please check your location and try again.")]
    IPBlocked(String),

    #[error("Subaccount name byte size, {0}, must be less than 12 bytes")]
    SubaccountNameSize(usize),

    #[error("Product id {0} is out of bounds")]
    ProductIdOutOfBounds(u32),

    #[error("Subaccount does not match wallet address")]
    SubaccountWalletMismatch,

    #[error("Feature not implemented for engine testing environment")]
    NotImplementedEngineTest,

    #[error("Admin request failed {0}")]
    AdminFailed(String),

    #[error("Balance not found for product_id: {0}")]
    BalanceNotFound(u32),

    #[error("Internal error: {0:?}")]
    InternalError(String),
}

pub fn none_error(name: &str) -> Report {
    eyre!("{} is None!", name)
}
