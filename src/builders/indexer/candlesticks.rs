use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::CandlesticksResponse;
use crate::serialize_utils::WrappedU32;
use crate::utils::client_error::none_error;

vertex_builder!(
    CandlesticksBuilder ,
    VertexIndexer,
    product_id: u32,
    granularity: u32,
    max_time: u64,
    limit: u32;

    build_and_call!(self, query, get_candlesticks => CandlesticksResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, product_id, granularity);

        Ok(indexer::Query::Candlesticks {
            product_id: WrappedU32(product_id),
            granularity: WrappedU32(granularity),
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
        })
    }

);
