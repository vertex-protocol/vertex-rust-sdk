use async_trait::async_trait;
use ethers::prelude::{Address, U256};
use ethers_core::types::{Bytes, TransactionReceipt};
use eyre::{eyre, Result};

use crate::bindings::endpoint::Endpoint;
use crate::bindings::querier::Querier;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::builders::execute::slow_mode::SubmitSlowModeTxParams;
use crate::core::query::VertexQuery;
use crate::eip712_structs::{
    BurnLp, Cancellation, CancellationProducts, LinkSigner, LiquidateSubaccount, MintLp,
    TransferQuote, WithdrawCollateral,
};
use crate::engine::{
    CancelOrdersResponse, Execute, ExecuteResponseData, PlaceOrder, PlaceOrderResponse,
};
use crate::provider::VertexProvider;
use crate::trigger;
use crate::utils::deposit::{erc20_client, provider_with_signer};
use crate::utils::response::match_cancel_orders_response;
use crate::utils::signer::Signer;

macro_rules! map_response_type {
    ($response_data:expr, $enum_variant:path => $output_type:ty) => {
        match $response_data {
            Some(data) => match data {
                $enum_variant(response) => Ok(Some(response as $output_type)),
                _ => Err(eyre!("Unexpected response type")),
            },
            None => Ok(None),
        }
    };
}

#[async_trait]
pub trait VertexExecute<S: Signer>: VertexQuery<S> {
    async fn execute(&self, execute: Execute) -> Result<Option<ExecuteResponseData>>;

    async fn execute_trigger(
        &self,
        execute: trigger::Execute,
    ) -> Result<Option<ExecuteResponseData>>;

    async fn submit_slow_mode_tx(
        &self,
        params: SubmitSlowModeTxParams,
    ) -> Result<Option<TransactionReceipt>>;

    async fn place_order(&self, place_order: PlaceOrder) -> Result<Option<PlaceOrderResponse>> {
        let execute = Execute::PlaceOrder(place_order);
        let execute_response_data = self.execute(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn place_trigger_order(
        &self,
        place_trigger_order: trigger::PlaceTriggerOrder,
    ) -> Result<Option<PlaceOrderResponse>> {
        let execute = trigger::Execute::PlaceOrder(place_trigger_order);
        let execute_response_data = self.execute_trigger(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn cancel_orders(&self, tx: Cancellation) -> Result<Option<CancelOrdersResponse>> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::CancelOrders { tx, signature };
        let execute_response_data = self.execute(execute).await?;
        match_cancel_orders_response(execute_response_data)
    }

    async fn cancel_trigger_orders(&self, tx: Cancellation) -> Result<()> {
        let signature: Bytes = self.endpoint_signature(&tx)?.into();
        let execute = trigger::Execute::CancelOrders { tx, signature };
        self.execute_trigger(execute).await?;
        Ok(())
    }

    async fn cancel_product_orders(
        &self,
        tx: CancellationProducts,
    ) -> Result<Option<CancelOrdersResponse>> {
        let signature = self.endpoint_signature(&tx)?;
        let digest = Some(self.signer().endpoint_digest(&tx)?);
        let execute = Execute::CancelProductOrders {
            tx,
            signature,
            digest,
        };
        let execute_response_data = self.execute(execute).await?;
        match_cancel_orders_response(execute_response_data)
    }

    async fn cancel_product_trigger_orders(&self, tx: CancellationProducts) -> Result<()> {
        let signature: Bytes = self.endpoint_signature(&tx)?.into();
        let execute = trigger::Execute::CancelProductOrders { tx, signature };
        self.execute_trigger(execute).await?;
        Ok(())
    }

    async fn cancel_and_place(
        &self,
        cancel_tx: Cancellation,
        place_order: PlaceOrder,
    ) -> Result<Option<PlaceOrderResponse>> {
        let cancel_signature = self.endpoint_signature(&cancel_tx)?;
        let execute = Execute::CancelAndPlace {
            cancel_tx,
            cancel_signature,
            place_order,
        };
        let execute_response_data = self.execute(execute).await?;
        map_response_type!(execute_response_data, ExecuteResponseData::PlaceOrder => PlaceOrderResponse)
    }

    async fn liquidate_subaccount(&self, tx: LiquidateSubaccount) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::LiquidateSubaccount { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn withdraw_collateral(
        &self,
        tx: WithdrawCollateral,
        spot_leverage: Option<bool>,
    ) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::WithdrawCollateral {
            tx,
            signature,
            spot_leverage,
            sequencer_risk_check: None,
        };
        self.execute(execute).await?;
        Ok(())
    }

    async fn mint_lp(&self, tx: MintLp, spot_leverage: Option<bool>) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::MintLp {
            tx,
            signature,
            spot_leverage,
        };
        self.execute(execute).await?;
        Ok(())
    }

    async fn burn_lp(&self, tx: BurnLp) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::BurnLp { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn transfer_quote(&self, tx: TransferQuote) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::TransferQuote { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn link_signer(&self, tx: LinkSigner) -> Result<()> {
        let signature = self.endpoint_signature(&tx)?;
        let execute = Execute::LinkSigner { tx, signature };
        self.execute(execute).await?;
        Ok(())
    }

    async fn submit_private_batch(&self, orders: Vec<[PlaceOrder; 2]>) -> Result<()> {
        let execute = Execute::SubmitPrivateBatch { orders };
        self.execute(execute).await?;
        Ok(())
    }

    async fn deposit_collateral(
        &self,
        deposit_collateral_params: DepositCollateralParams,
    ) -> Result<Option<TransactionReceipt>>;

    async fn approve_endpoint_allowance(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve(self.endpoint_addr(), product_id, amount).await
    }

    #[deprecated(
        since = "0.2.2",
        note = "please use `approve_endpoint_allowance` or `approve` instead"
    )]
    async fn approve_allowance(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve(self.endpoint_addr(), product_id, amount).await
    }

    async fn approve(
        &self,
        spender: Address,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        self.approve_with_gas_price(spender, product_id, amount, None)
            .await
    }

    async fn approve_with_gas_price(
        &self,
        spender: Address,
        product_id: u32,
        amount: u128,
        gas_price: Option<u128>,
    ) -> Result<Option<TransactionReceipt>> {
        let erc20_client = erc20_client(self, product_id).await?;
        let mut tx = erc20_client.approve(spender, U256::from(amount));
        if let Some(price) = gas_price {
            tx = tx.gas_price(price)
        }
        let tx_receipt = tx.send().await?.await?;
        println!("approved: {amount} of product id: {product_id}");
        Ok(tx_receipt)
    }

    async fn get_token_allowance(&self, product_id: u32) -> Result<U256> {
        let erc20_client = erc20_client(self, product_id).await?;
        let owner = self.wallet()?.address();
        let allowance = erc20_client
            .allowance(owner, self.endpoint_addr())
            .call()
            .await?;
        Ok(allowance)
    }

    async fn get_token_balance(&self, product_id: u32) -> Result<U256> {
        let erc20_client = erc20_client(self, product_id).await?;
        let balance = erc20_client
            .balance_of(self.wallet()?.address())
            .call()
            .await?;
        Ok(balance)
    }

    async fn mint_mock_erc20(
        &self,
        product_id: u32,
        amount: u128,
    ) -> Result<Option<TransactionReceipt>> {
        let erc20_client = erc20_client(self, product_id).await?;
        let tx = erc20_client.mint(self.wallet()?.address(), U256::from(amount));
        let tx_receipt = tx.send().await?.await?;
        println!("minted: {amount} of product id: {product_id}");
        Ok(tx_receipt)
    }

    fn endpoint(&self) -> Result<Endpoint<VertexProvider<S>>> {
        let provider = provider_with_signer(self)?;
        let endpoint = Endpoint::new(self.endpoint_addr(), provider);
        Ok(endpoint)
    }

    fn querier(&self) -> Result<Querier<VertexProvider<S>>> {
        let provider = provider_with_signer(self)?;
        let endpoint = Querier::new(self.querier_addr(), provider);
        Ok(endpoint)
    }
}
