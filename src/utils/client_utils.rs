use eyre::{eyre, Result};

use crate::prelude::VertexBase;
use crate::utils::client_error::ClientError;

pub fn subaccount_matches_wallet<V>(client: &V, subaccount: [u8; 32]) -> Result<()>
where
    V: VertexBase,
{
    let wallet_address = &client.address()?[..20];
    if wallet_address == &subaccount[..20] {
        Ok(())
    } else {
        Err(eyre!(ClientError::SubaccountWalletMismatch))
    }
}

pub fn validate_subaccount_name(name: &str) -> Result<()> {
    let byte_len = name.as_bytes().len();
    if byte_len > 12 {
        Err(eyre!(ClientError::SubaccountNameSize(byte_len)))
    } else {
        Ok(())
    }
}
