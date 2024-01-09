use crate::core::indexer::VertexIndexer;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::MakerStatisticsResponse;
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::client_error::none_error;

vertex_builder!(
    MakerStatisticsBuilder,
    VertexIndexer,
    epoch: u32,
    product_id: u32,
    interval: u64;

    build_and_call!(self, query, get_maker_statistics => MakerStatisticsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, epoch, product_id, interval);
        Ok(indexer::Query::MakerStatistics {
            epoch: WrappedU32(epoch),
            product_id: WrappedU32(product_id),
            interval: WrappedU64(interval),
        })
    }
);
