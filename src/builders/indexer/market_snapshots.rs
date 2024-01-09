use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::{wrapped_option_u64, wrapped_option_vec_u32};
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::Interval;
use crate::indexer::MarketSnapshotsResponse;
use crate::utils::client_error::none_error;

vertex_builder!(
    MarketSnapshotsBuilder,
    VertexIndexer,
    max_time: u64,
    granularity: u64,
    product_ids: Vec<u32>,
    count: u64;

    build_and_call!(self, query, get_market_snapshots => MarketSnapshotsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, granularity, count);
        let product_ids = wrapped_option_vec_u32(self.product_ids.clone());
        let interval = Interval {
            count,
            granularity,
            max_time: wrapped_option_u64(self.max_time),
        };
        Ok(indexer::Query::MarketSnapshots {
            interval,
            product_ids
        })
    }
);
