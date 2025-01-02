use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use ethers::types::H160;
use eyre::Result;

use crate::eip712_structs;
use crate::utils::client_error::none_error;

vertex_builder!(
    LiquidateSubaccountBuilder,
    VertexExecute,
    liquidatee: [u8; 32],
    product_id: u32,
    is_encoded_spread: bool,
    amount: i128,
    nonce: u64,
    linked_sender: [u8; 32];

    build_and_call!(self, execute, liquidate_subaccount => (), async_build);

    pub async fn build(&self) -> Result<eip712_structs::LiquidateSubaccount> {
        let default_sender = self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);

        fields_to_vars!(self, liquidatee, product_id, is_encoded_spread, amount);

        Ok(eip712_structs::LiquidateSubaccount {
            sender,
            liquidatee,
            productId: product_id,
            isEncodedSpread: is_encoded_spread,
            amount,
            nonce,
        })
    }
);
