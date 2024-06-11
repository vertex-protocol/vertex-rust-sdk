use std::collections::HashMap;

use eyre::Result;

use crate::core::indexer::VertexIndexer;
use crate::indexer;
use crate::indexer::{ProductSnapshot, TimestampOrTimestamps};
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    MultiProductSnapshotsBuilder,
    VertexIndexer,
    product_ids: Vec<u32>,
    max_time: u64,
    timestamps: Vec<u64>;

    build_and_call!(self, query, get_multi_product_snapshots => HashMap<u32, ProductSnapshot>);

    build_and_call!(self, query_multi_timestamp, get_multi_timestamp_product_snapshots => HashMap<WrappedU64, HashMap<u32, ProductSnapshot>>);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, (product_ids: clone));
        let mut max_time = self.max_time.map(|ts| TimestampOrTimestamps::Timestamp(WrappedU64(ts)));
        if max_time.is_none() {
            max_time = self.timestamps.clone().map(|ts| {
                assert!(self.max_time.is_none(), "cannot set max_time and timestamps");
                TimestampOrTimestamps::Timestamps(ts.into_iter().map(WrappedU64).collect())
            });
        }

        let product_ids = product_ids.into_iter().map(WrappedU32).collect::<Vec<WrappedU32>>();

        Ok(indexer::Query::ProductSnapshots {
            product_ids,
            max_time,
        })
    }

);
