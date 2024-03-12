use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::{
    wrapped_option_bytes32, wrapped_option_u64, wrapped_option_vec_u32,
};
use crate::{build_and_call, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::MatchesResponse;
use crate::serialize_utils::WrappedU32;

vertex_builder!(
    MatchesBuilder,
    VertexIndexer,
    subaccount: [u8; 32],
    product_ids: Vec<u32>,
    max_time: u64,
    limit: u32,
    idx: u64;

    build_and_call!(self, query, get_matches => MatchesResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        let product_ids = wrapped_option_vec_u32(self.product_ids.clone());

        Ok(indexer::Query::Matches {
            subaccount: wrapped_option_bytes32(self.subaccount),
            product_ids,
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
            idx: wrapped_option_u64(self.idx),
        })
    }
);
