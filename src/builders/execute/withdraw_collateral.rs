use eyre::Result;

use crate::bindings::endpoint;
use crate::eip712_structs;

use crate::core::execute::VertexExecute;
use crate::utils::client_error::none_error;
use crate::{fields_to_vars, vertex_builder};

vertex_builder!(
    WithdrawCollateralBuilder,
    VertexExecute,
    amount: u128,
    product_id: u32,
    nonce: u64,
    spot_leverage: bool;

    // we do not use macro here because of extra required argument
    pub async fn execute(&self) -> Result<()> {
        self.vertex
            .withdraw_collateral(self.build().await?, self.spot_leverage)
            .await
    }

    pub async fn build_endpoint_tx(&self) -> Result<endpoint::WithdrawCollateral> {
        let tx = self.build().await?;
        Ok(
            endpoint::WithdrawCollateral {
                sender: tx.sender,
                amount: tx.amount,
                nonce: tx.nonce,
                product_id: tx.productId,
            }
        )
    }

    pub async fn build(&self) -> Result<eip712_structs::WithdrawCollateral> {
        let sender = self.vertex.subaccount()?;
        let address = self.vertex.address()?;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);
        fields_to_vars!(self, amount, product_id);

        Ok(eip712_structs::WithdrawCollateral {
            sender,
            amount,
            nonce,
            productId: product_id,
        })
    }
);
