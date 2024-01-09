use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::eip712_structs;
use crate::utils::client_error::none_error;

vertex_builder!(
    LiquidateSubaccountBuilder,
    VertexExecute,
    liquidatee: [u8; 32],
    mode: u8,
    health_group: u32,
    amount: i128,
    nonce: u64;

    build_and_call!(self, execute, liquidate_subaccount => (), async_build);

    pub async fn build(&self) -> Result<eip712_structs::LiquidateSubaccount> {
        let sender = self.vertex.subaccount()?;
        let address = self.vertex.address()?;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);

        fields_to_vars!(self, liquidatee, mode, health_group, amount);

        Ok(eip712_structs::LiquidateSubaccount {
            sender,
            liquidatee,
            mode,
            healthGroup: health_group,
            amount,
            nonce,
        })
    }
);
