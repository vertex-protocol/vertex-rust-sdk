use eyre::Result;

use crate::eip712_structs::BurnLp;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    BurnLpBuilder,
    VertexExecute,
    product_id: u32,
    amount: u128,
    nonce: u64;

    build_and_call!(self, execute, burn_lp => (), async_build);

    pub async fn build(&self) -> Result<BurnLp> {
        let sender = self.vertex.subaccount()?;
        let address = self.vertex.address()?;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);
        fields_to_vars!(self, product_id, amount);

        Ok(BurnLp {
            sender,
            productId: product_id,
            amount,
            nonce,
        })
    }
);
