use eyre::Result;

use crate::indexer;
use crate::indexer::InterestAndFundingTicksResponse;
use crate::serialize_utils::WrappedU32;
use crate::utils::client_error::none_error;

use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    InterestAndFundingTicksBuilder,
    VertexIndexer,
    subaccount: [u8; 32],
    product_ids: Vec<u32>,
    max_idx: u64,
    max_time: u64,
    limit: u32;

    build_and_call!(self, query, get_interest_and_funding => InterestAndFundingTicksResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, subaccount, (product_ids: clone), limit);
        Ok(indexer::Query::InterestAndFunding {
            subaccount,
            product_ids: WrappedU32::wrap_vec_u32(&product_ids),
            max_idx: wrapped_option_u64(self.max_idx),
            max_time: wrapped_option_u64(self.max_time),
            limit: WrappedU32(limit),
        })
    }
);
