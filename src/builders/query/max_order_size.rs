use crate::core::query::VertexQuery;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use eyre::Result;

use crate::engine;
use crate::engine::{Direction, MaxOrderSizeResponse};
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::optional_bool_to_string;

vertex_builder!(
    MaxOrderSizeBuilder,
    VertexQuery,
    subaccount: [u8; 32],
    product_id: u32,
    price_x18: i128,
    direction: Direction,
    spot_leverage: bool;

    build_and_call!(self, query, get_max_order_size => MaxOrderSizeResponse);

    pub fn build(&self) -> Result<engine::Query> {
        fields_to_vars!(self, product_id, subaccount, price_x18, (direction: clone));
        Ok(engine::Query::MaxOrderSize {
            sender: subaccount,
            product_id,
            price_x18,
            direction,
            spot_leverage: optional_bool_to_string(self.spot_leverage),
        })
    }
);
