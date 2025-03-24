use crate::core::query::VertexQuery;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::engine;
use crate::engine::MaxVlpMintableResponse;
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::optional_bool_to_string;

vertex_builder!(
    MaxVlpMintableBuilder,
    VertexQuery,
    subaccount: [u8; 32],
    spot_leverage: bool;

    build_and_call!(self, query, get_max_vlp_mintable => MaxVlpMintableResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, subaccount);
        Ok(engine::Query::MaxVlpMintable {
            sender: subaccount,
            spot_leverage: optional_bool_to_string(self.spot_leverage),
        })
    }
);
