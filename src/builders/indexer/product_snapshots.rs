use crate::core::indexer::VertexIndexer;
use crate::serialize_utils::WrappedU32;
use crate::utils::wrapped_option_utils::wrapped_option_u64;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::ProductsResponse;
use crate::utils::client_error::none_error;

vertex_builder!(
    ProductSnapshotsBuilder ,
    VertexIndexer,
    product_id: u32,
    max_time: u64,
    limit: u32,
    idx: u64;

    build_and_call!(self, query, get_product_snapshots => ProductsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, product_id);
        Ok(indexer::Query::Products {
            product_id: WrappedU32(product_id),
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
            idx: wrapped_option_u64(self.idx),
        })
    }

);
