use ethers::types::H160;
use eyre::Result;

use crate::eip712_structs::BurnVlp;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    BurnVlpBuilder,
    VertexExecute,
    vlp_amount: u128,
    nonce: u64,
    linked_sender: [u8; 32];

    build_and_call!(self, execute, burn_vlp => (), async_build);

    pub async fn build(&self) -> Result<BurnVlp> {
        let default_sender = self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);
        fields_to_vars!(self, vlp_amount);

        Ok(BurnVlp {
            sender,
            vlpAmount: vlp_amount,
            nonce,
        })
    }
);
