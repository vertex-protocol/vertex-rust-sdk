use eyre::Result;

use crate::bindings::endpoint;
use crate::eip712_structs;
use crate::engine::{PlaceIsolatedOrder, PlaceOrderResponse};
use crate::tx::get_eip712_digest;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::utils::nonce::order_nonce;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    PlaceIsolatedOrderBuilder,
    VertexExecute,
    product_id: u32,
    price_x18: i128,
    linked_sender: [u8; 32],
    amount: i128,
    margin: i128,
    expiration: u64,
    order_type: eip712_structs::OrderType,
    reduce_only: bool,
    nonce: u64,
    recv_time: u64,
    mock_digest_and_signature: bool,
    id: u64,
    borrow_margin: bool;


    build_and_call!(self, execute, place_isolated_order => Option<PlaceOrderResponse>);

    pub fn build(&self) -> Result<PlaceIsolatedOrder> {
        let isolated_order = self.isolated_order()?;
        let id = self.id;
        let signature = self.get_signature(self.get_product_id()?, &isolated_order)?;
        let digest = self.get_digest(&isolated_order)?;
        let borrow_margin = self.borrow_margin;

        Ok(PlaceIsolatedOrder {
            isolated_order,
            signature,
            product_id: self.get_product_id()?,
            digest: Some(digest),
            id,
            borrow_margin,
        })
    }

    fn get_signature(&self, product_id: u32, isolated_order: &eip712_structs::IsolatedOrder) -> Result<Vec<u8>> {
        if self.should_mock_digest_and_signature() {
            let mut signature = vec![0; 65];
            signature.extend(self.vertex.address()?);
            Ok(signature)
        } else {
            self.vertex.signer().order_signature(product_id, isolated_order)
        }
    }

    fn get_digest(&self, isolated_order: &eip712_structs::IsolatedOrder) -> Result<[u8; 32]> {
        if self.should_mock_digest_and_signature() {
            Ok(random_digest())
        } else {
            self.encode_digest(isolated_order)
        }
    }

    fn encode_digest(&self, isolated_order: &eip712_structs::IsolatedOrder) -> Result<[u8; 32]> {
        let domain = self.vertex.signer().order_domain(self.get_product_id()?)?;
        let encoded = get_eip712_digest(&isolated_order.to_order(), &domain);
        Ok(encoded.to_fixed_bytes())
    }

    pub fn isolated_order(&self) -> Result<eip712_structs::IsolatedOrder> {
        let mut builder = self.clone();

        if self.expiration.is_none() {
            builder = builder.expiration(u32::MAX as u64);
        }

        if self.order_type.is_none() {
            builder = builder.order_type(eip712_structs::OrderType::Default);
        }

        if self.reduce_only.is_none() {
            builder = builder.reduce_only(false);
        }

        builder.isolated_order_inner()
    }

    fn isolated_order_inner(&self) -> Result<eip712_structs::IsolatedOrder> {
        fields_to_vars!(self, amount, price_x18, reduce_only, margin, (order_type: clone));

        let mut expiration = self.expiration.ok_or(none_error("expiration"))?;

        expiration = order_type.apply_to_expiration(expiration);
        if reduce_only {
            expiration |= 1 << 61;
        }
        let nonce = self.nonce.unwrap_or(order_nonce(self.recv_time));

        let default_sender = self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);

        Ok(eip712_structs::IsolatedOrder::from_binding(&endpoint::IsolatedOrder {
            sender,
            price_x18,
            amount,
            expiration,
            nonce,
            margin
        }))
    }

    fn get_product_id(&self) -> Result<u32> {
        fields_to_vars!(self, product_id);
        Ok(product_id)
    }

    fn should_mock_digest_and_signature(&self) -> bool {
        self.mock_digest_and_signature.unwrap_or(false)
    }
);

fn random_digest() -> [u8; 32] {
    let mut arr = [0u8; 32];
    rand::Rng::fill(&mut rand::thread_rng(), &mut arr[..]);
    arr
}
