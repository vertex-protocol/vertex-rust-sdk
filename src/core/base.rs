use std::fmt::Debug;

use async_trait::async_trait;
use ethers::prelude::{H160, U256};
use ethers_core::types::transaction::eip712::Eip712;
use eyre::Result;

use crate::eip712_structs::{concat_to_bytes32, to_bytes12};

use crate::utils::client_utils::validate_subaccount_name;
use crate::utils::signer::{Signer, VertexSigner};

#[async_trait]
pub trait VertexBase<S: Signer>: Clone + Sync {
    async fn with_signer(&self, signer: S) -> Result<Self>;

    fn with_subaccount_name(&self, subaccount_name: &str) -> Self {
        validate_subaccount_name(subaccount_name).unwrap();
        self.with_subaccount_name_bytes(to_bytes12(subaccount_name))
    }

    fn with_subaccount_name_bytes(&self, subaccount_name: [u8; 12]) -> Self;

    fn wallet(&self) -> Result<&S>;

    fn address(&self) -> Result<[u8; 20]> {
        Ok(self.wallet()?.address().into())
    }

    fn subaccount(&self) -> Result<[u8; 32]> {
        let address = self.address()?;
        let name = self.subaccount_name_bytes();
        Ok(concat_to_bytes32(address, name))
    }

    fn subaccount_name_bytes(&self) -> [u8; 12];

    fn signer(&self) -> VertexSigner<S, Self> {
        VertexSigner::new(self)
    }

    fn endpoint_signature<T: Eip712 + Send + Sync + Debug>(
        &self,
        endpoint_tx: &T,
    ) -> Result<Vec<u8>> {
        self.signer().endpoint_signature(endpoint_tx)
    }

    fn node_url(&self) -> String;

    fn endpoint_addr(&self) -> H160;

    fn querier_addr(&self) -> H160;

    fn chain_id(&self) -> Result<U256>;

    fn book_addr(&self, product_id: u32) -> Result<H160>;

    fn is_rest_client(&self) -> bool;
}
