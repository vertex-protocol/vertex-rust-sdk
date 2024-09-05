use crate::core::indexer::VertexIndexer;
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use ethers::prelude::H160;

use eyre::Result;

use crate::indexer;
use crate::indexer::RewardsResponse;
use crate::serialize_utils::WrappedU32;

vertex_builder!(
    RewardsBuilder,
    VertexIndexer,
    address: [u8; 20],
    start: u32,
    limit: u32;

    build_and_call!(self, query, get_rewards => RewardsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, address);

        Ok(indexer::Query::Rewards {
            address: H160::from(address),
            start: self.start.map(WrappedU32),
            limit: self.limit.map(WrappedU32),
        })
    }
);
