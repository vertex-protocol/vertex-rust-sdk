use ethers::types::H160;
use eyre::Result;

use crate::eip712_structs::TransferQuote;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    TransferQuoteBuilder,
    VertexExecute,
    recipient: [u8; 32],
    linked_sender: [u8; 32],
    amount: u128,
    nonce: u64;

    build_and_call!(self, execute, transfer_quote => (), async_build);

    pub async fn build(&self) -> Result<TransferQuote> {
        let default_sender = self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);

        let address = H160::from_slice(&sender[0..20]).0;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);
        fields_to_vars!(self, recipient, amount);

        Ok(TransferQuote {
            sender,
            recipient,
            amount,
            nonce
        })
    }
);
