use crate::core::indexer::VertexIndexer;
use crate::utils::wrapped_option_utils::{
    wrapped_option_bytes32, wrapped_option_u64, wrapped_option_vec_bytes32, wrapped_option_vec_u32,
};
use crate::{build_and_call, vertex_builder};
use eyre::{eyre, Result};

use crate::indexer;
use crate::indexer::OrdersResponse;
use crate::serialize_utils::WrappedU32;

vertex_builder!(
    HistoricalOrdersBuilder,
    VertexIndexer,
    subaccount: [u8; 32],
    product_ids: Vec<u32>,
    max_time: u64,
    limit: u32,
    idx: u64,
    digests: Vec<[u8; 32]>,
    isolated: bool;

    build_and_call!(self, query, get_historical_orders => OrdersResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        self.validate_build_conditions()?;
        let product_ids = wrapped_option_vec_u32(self.product_ids.clone());

        Ok(indexer::Query::Orders {
            subaccount: wrapped_option_bytes32(self.subaccount),
            product_ids,
            digests: wrapped_option_vec_bytes32(self.digests.clone()),
            max_time: wrapped_option_u64(self.max_time),
            limit: self.limit.map(WrappedU32),
            idx: wrapped_option_u64(self.idx),
            isolated: self.isolated,
        })
    }

    fn validate_build_conditions(&self) -> Result<()> {
        if self.subaccount.is_none() && self.digests.is_none() {
            return Err(eyre!(
                "historical orders: subaccount or digests must be set"
            ));
        }
        Ok(())
    }
);
