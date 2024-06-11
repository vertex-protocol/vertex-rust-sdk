use ethers::prelude::TxHash;
use ethers_core::types::Bytes;
use eyre::{eyre, Result};

use crate::bindings::endpoint;
use crate::eip712_structs;
use crate::engine::{PlaceOrder, PlaceOrderResponse};
use crate::trigger::{PlaceTriggerOrder, TriggerCriteria};
use crate::tx::get_eip712_digest;
use crate::utils::client_error::none_error;

use crate::core::execute::VertexExecute;
use crate::utils::nonce::order_nonce;
use crate::{build_and_call, fields_to_vars, vertex_builder};

vertex_builder!(
    PlaceOrderBuilder,
    VertexExecute,
    product_id: u32,
    price_x18: i128,
    linked_sender: [u8; 32],
    amount: i128,
    expiration: u64,
    order_type: eip712_structs::OrderType,
    reduce_only: bool,
    trigger_criteria: TriggerCriteria,
    nonce: u64,
    recv_time: u64,
    spot_leverage: bool,
    mock_digest_and_signature: bool,
    id: u64;


    build_and_call!(self, execute, place_order => Option<PlaceOrderResponse>);

    pub async fn execute_trigger(&self) -> Result<Option<PlaceOrderResponse>> {
        self.vertex.place_trigger_order(self.build_trigger()?).await
    }

    pub fn build(&self) -> Result<PlaceOrder> {
        self.assert_trigger_unset()?;
        let order = self.order()?;
        let id = self.id;
        let signature = self.get_signature(self.get_product_id()?, &order)?;
        let digest = self.get_digest(&order)?;

        Ok(PlaceOrder {
            order,
            signature,
            product_id: self.get_product_id()?,
            digest: Some(digest),
            id,
            spot_leverage: self.spot_leverage,
        })
    }

    pub fn build_trigger(&self) -> Result<PlaceTriggerOrder> {
        let order = self.order()?;
        let product_id = self.get_product_id()?;

        let trigger = self
            .trigger_criteria
            .clone()
            .ok_or(none_error("trigger_criteria"))?;
        let signature = self.get_signature(product_id, &order)?;
        let digest = self.get_digest(&order)?;
        Ok(PlaceTriggerOrder {
            order,
            signature: Bytes::from(signature),
            product_id,
            digest: Some(TxHash(digest)),
            spot_leverage: self.spot_leverage,
            trigger,
            id: self.id
        })
    }

    fn get_signature(&self, product_id: u32, order: &eip712_structs::Order) -> Result<Vec<u8>> {
        if self.should_mock_digest_and_signature() {
            let mut signature = vec![0; 65];
            signature.extend(self.vertex.address()?);
            Ok(signature)
        } else {
            self.vertex.signer().order_signature(product_id, order)
        }
    }

    fn get_digest(&self, order: &eip712_structs::Order) -> Result<[u8; 32]> {
        if self.should_mock_digest_and_signature() {
            Ok(random_digest())
        } else {
            self.encode_digest(order)
        }
    }

    fn encode_digest(&self, order: &eip712_structs::Order) -> Result<[u8; 32]> {
        let domain = self.vertex.signer().order_domain(self.get_product_id()?)?;
        let encoded = get_eip712_digest(order, &domain);
        Ok(encoded.to_fixed_bytes())
    }

    fn assert_trigger_unset(&self) -> Result<()> {
        if self.trigger_criteria.is_some() {
            Err(eyre!("trigger_criteria set, use .build_triger to build trigger orders or clear trigger criteria"))
        } else {
            Ok(())
        }
    }

    pub fn order(&self) -> Result<eip712_structs::Order> {
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

        builder.order_inner()
    }

    fn order_inner(&self) -> Result<eip712_structs::Order> {
        fields_to_vars!(self, amount, price_x18, reduce_only, (order_type: clone));

        let mut expiration = self.expiration.ok_or(none_error("expiration"))?;

        expiration = order_type.apply_to_expiration(expiration);
        if reduce_only {
            expiration |= 1 << 61;
        }
        let mut nonce = self.nonce.unwrap_or(order_nonce(self.recv_time));
        if self.trigger_criteria.is_some() {
            nonce |= 1 << 63;
        }

        let default_sender = self.vertex.subaccount()?;
        let sender = self.linked_sender.unwrap_or(default_sender);

        Ok(eip712_structs::Order::from_binding(&endpoint::Order {
            sender,
            price_x18,
            amount,
            expiration,
            nonce,
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
