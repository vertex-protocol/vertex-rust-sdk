use eyre::Result;

use crate::eip712_structs;
use crate::engine::CancelOrdersResponse;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::utils::nonce::order_nonce;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    CancellationProductsBuilder,
    VertexExecute,
    product_ids: Vec<u32>,
    nonce: u64,
    linked_sender: [u8; 32],
    recv_time: u64;

    build_and_call!(self, execute, cancel_product_orders => Option<CancelOrdersResponse>);
    build_and_call!(self, execute_trigger, cancel_product_trigger_orders => ());

    pub fn build(&self) -> Result<eip712_structs::CancellationProducts> {
        let default_sender =  self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let nonce = self.nonce.unwrap_or(order_nonce(self.recv_time));
        fields_to_vars!(self, (product_ids: clone));

        Ok(eip712_structs::CancellationProducts {
            sender,
            productIds: product_ids,
            nonce,
        })
    }
);
