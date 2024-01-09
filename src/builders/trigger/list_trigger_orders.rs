use crate::core::query::VertexQuery;
use crate::utils::nonce::default_recv_time;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use ethers::types::Bytes;
use eyre::Result;

use crate::eip712_structs;
use crate::trigger;
use crate::trigger::ListTriggerOrdersResponse;
use crate::utils::client_error::none_error;
use crate::utils::wrapped_option_utils::wrapped_option_bytes32;

vertex_builder!(
    ListTriggerOrdersBuilder ,
    VertexQuery,
    recv_time: u64,
    product_id: u32,
    linked_sender: [u8; 32],
    pending: bool,
    max_digest: [u8; 32],
    max_update_time: u64,
    limit: u32;

    build_and_call!(self, query, list_trigger_orders => ListTriggerOrdersResponse);

    pub fn build(&self) -> Result<trigger::Query> {
        let tx = self.list_trigger_orders()?;
        fields_to_vars!(self, pending);
        let signature = Bytes::from(self.vertex.endpoint_signature(&tx)?);

        Ok(trigger::Query::ListTriggerOrders {
            tx,
            signature,
            product_id: self.product_id,
            pending,
            max_update_time: self.max_update_time,
            max_digest: wrapped_option_bytes32(self.max_digest),
            limit: self.limit,
        })
    }


    fn list_trigger_orders(&self) -> Result<eip712_structs::ListTriggerOrders> {
        let default_sender =  self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);
        let recv_time = self.recv_time.unwrap_or(default_recv_time());
        Ok(eip712_structs::ListTriggerOrders {
            sender,
            recvTime: recv_time,
        })
    }
);
