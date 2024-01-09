use eyre::Result;

use crate::indexer;
use crate::indexer::{TradesParams, TradesResponse};
use crate::serialize_utils::{WrappedU32, WrappedU64};
use crate::utils::client_error::none_error;

use crate::core::indexer::VertexIndexer;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    TradesParamsBuilder,
    VertexIndexer,
    ticker_id: String,
    max_trade_id: u64,
    limit: u32;

    build_and_call!(self, query, get_trades => TradesResponse);

    pub fn build(&self) -> Result<indexer::QueryV2> {
        fields_to_vars!(self, (ticker_id: clone));
        Ok(indexer::QueryV2::Trades(TradesParams{
            ticker_id,
            max_trade_id: self.max_trade_id.map(WrappedU64),
            limit: self.limit.map(WrappedU32),
        }))
    }
);
