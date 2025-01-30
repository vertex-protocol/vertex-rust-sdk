use eyre::Result;

use crate::core::indexer::VertexIndexer;
use crate::indexer;
use crate::indexer::AccountSnapshotsResponse;
use crate::serialize_utils::{WrappedBytes32, WrappedU64};
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    AccountSnapshotsBuilder,
    VertexIndexer,
    subaccounts: Vec<[u8; 32]>,
    timestamps: Vec<u64>,
    isolated: bool;

    build_and_call!(self, query, get_account_snapshots => AccountSnapshotsResponse);

    pub fn build(&self) -> Result<indexer::Query> {
        fields_to_vars!(self, (subaccounts: clone), (timestamps: clone));
        let subaccounts = subaccounts.into_iter().map(WrappedBytes32).collect();
        let timestamps = timestamps.into_iter().map(WrappedU64).collect();
        Ok(indexer::Query::AccountSnapshots{
            subaccounts,
            timestamps,
            isolated: self.isolated,
        })
    }

);
