use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, vertex_builder};
use ethers::prelude::H160;
use eyre::Result;

use crate::indexer;
use crate::indexer::SubaccountsResponse;

vertex_builder!(
    SubaccountsBuilder,
    VertexIndexer,
    address: [u8; 20],
    start: u64,
    limit: u64;

    build_and_call!(self, query, get_subaccounts => SubaccountsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        let address = self.get_address();
        Ok(indexer::Query::Subaccounts {
            address,
            start: wrapped_option_u64(self.start),
            limit: wrapped_option_u64(self.limit),
        })
    }

    fn get_address(&self) -> Option<H160> {
        self.address.map(H160::from)
    }
);
