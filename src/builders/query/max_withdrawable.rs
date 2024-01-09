use crate::core::query::VertexQuery;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::engine;
use crate::engine::MaxWithdrawableResponse;
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::optional_bool_to_string;

vertex_builder!(
    MaxWithdrawableBuilder,
    VertexQuery,
    subaccount: [u8; 32],
    product_id: u32,
    spot_leverage: bool;

    build_and_call!(self, query, get_max_withdrawable =>MaxWithdrawableResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, product_id, subaccount);
        Ok(engine::Query::MaxWithdrawable {
            sender: subaccount,
            product_id,
            spot_leverage: optional_bool_to_string(self.spot_leverage),
        })
    }
);
