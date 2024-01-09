use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::{
    wrapped_option_bytes32, wrapped_option_u64, wrapped_option_vec_u32,
};
use crate::{build_and_call, vertex_builder};
use eyre::Result;

use crate::indexer;
use crate::indexer::{EventsResponse, Limit};
use crate::tx::TxType;

vertex_builder!(
    EventsBuilder,
    VertexIndexer,
    subaccount: [u8; 32],
    product_ids: Vec<u32>,
    event_types: Vec<TxType>,
    max_time: u64,
    limit: Limit,
    idx: u64,
    desc: bool;

    build_and_call!(self, query, get_events => EventsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        let product_ids = wrapped_option_vec_u32(self.product_ids.clone());

        Ok(indexer::Query::Events {
            subaccount: wrapped_option_bytes32(self.subaccount.clone()),
            product_ids,
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.clone(),
            idx: wrapped_option_u64(self.idx.clone()),
            event_types: self.event_types.clone(),
            desc: self.desc,
        })
    }
);
