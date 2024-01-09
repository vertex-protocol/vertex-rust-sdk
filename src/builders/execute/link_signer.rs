use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::eip712_structs::LinkSigner;
use crate::utils::client_error::none_error;

vertex_builder!(
    LinkSignerBuilder,
    VertexExecute,
    signer: [u8; 32],
    nonce: u64;

    build_and_call!(self, execute, link_signer => (), async_build);

    pub async fn build(&self) -> Result<LinkSigner> {
        let sender = self.vertex.subaccount()?;
        let address = self.vertex.address()?;
        let nonce = self
            .nonce
            .unwrap_or(self.vertex.next_tx_nonce(address).await?);
        fields_to_vars!(self, signer);
        Ok(LinkSigner {
            sender,
            signer,
            nonce,
        })
    }
);
