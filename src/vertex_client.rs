use async_trait::async_trait;
use ethers::types::{H160, U256};
use ethers_core::types::TransactionReceipt;
use eyre::eyre;
use eyre::Result;
use serde::de::DeserializeOwned;
use std::time::Duration;

use engine::Status;

use crate::bindings::clearinghouse::Clearinghouse;
use crate::bindings::withdraw_pool::WithdrawPool;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::builders::execute::slow_mode::SubmitSlowModeTxParams;
use crate::eip712_structs::to_bytes12;
use crate::engine::{ExecuteResponse, QueryV2};
use crate::prelude::*;
use crate::provider::VertexProvider;
use crate::utils::client_error::{none_error, ClientError};
use crate::utils::constants::SLOW_MODE_FEE;
use crate::utils::deployment::Deployment;
use crate::utils::deposit::{deposit_collateral, provider_with_signer};
use crate::utils::rest::RestClient;
use crate::{engine, indexer, trigger};
use crate::{extract_response_data, fields_to_vars};
use crate::utils::signer::Signer;

#[derive(Clone)]
pub struct VertexClient<S: Signer> {
    pub client_mode: ClientMode,
    pub deployment: Deployment,
    pub client: RestClient,
    pub gateway_url: String,
    pub archive_url: String,
    pub trigger_url: String,
    pub subaccount_name_bytes: Option<[u8; 12]>,
    pub book_addrs: Option<Vec<H160>>,
    pub wallet: Option<S>,
    pub chain_id: Option<U256>,
}

impl <S: Signer> VertexClient<S> {
    pub fn new(client_mode: ClientMode) -> Self {
        Self {
            gateway_url: client_mode.default_gateway_url(),
            archive_url: client_mode.default_archive_url(),
            trigger_url: client_mode.default_trigger_url(),
            client: RestClient::new(),
            deployment: client_mode.deployment(),
            book_addrs: None,
            chain_id: None,
            wallet: None,
            subaccount_name_bytes: None,
            client_mode,
        }
    }

    pub fn with_gateway_url(&self, gateway_url: String) -> Self {
        Self {
            gateway_url,
            ..self.clone()
        }
    }

    pub fn with_archive_url(&self, archive_url: String) -> Self {
        Self {
            archive_url,
            ..self.clone()
        }
    }

    pub fn with_trigger_url(&self, trigger_url: String) -> Self {
        Self {
            trigger_url,
            ..self.clone()
        }
    }

    pub async fn withdraw_pool(&self) -> Result<WithdrawPool<VertexProvider<S>>> {
        let provider = provider_with_signer(self)?;
        let clearinghouse = Clearinghouse::new(self.deployment.clearinghouse, provider.clone());
        let withdraw_pool_address = clearinghouse.get_withdraw_pool().call().await?;
        let withdraw_pool = WithdrawPool::new(withdraw_pool_address, provider);
        Ok(withdraw_pool)
    }
}

#[async_trait]
impl <S: Signer> VertexBase<S> for VertexClient<S> {
    async fn with_signer(&self, signer: S) -> Result<Self> {
        let contracts_response = self.get_contracts().await?;
        let chain_id = U256::from(contracts_response.chain_id);
        let signer = signer.with_chain_id(chain_id.as_u64());
        let book_addrs: Vec<H160> = contracts_response
            .book_addrs
            .iter()
            .map(H160::from)
            .collect();
        Ok(Self {
            chain_id: Some(chain_id),
            wallet: Some(signer),
            book_addrs: Some(book_addrs),
            ..self.clone()
        })
    }

    fn with_subaccount_name_bytes(&self, subaccount_name: [u8; 12]) -> Self {
        Self {
            subaccount_name_bytes: Some(subaccount_name),
            ..self.clone()
        }
    }
    fn wallet(&self) -> Result<&S> {
        self.wallet.as_ref().ok_or(none_error("wallet"))
    }

    fn subaccount_name_bytes(&self) -> [u8; 12] {
        self.subaccount_name_bytes.unwrap_or(to_bytes12("default"))
    }

    fn node_url(&self) -> String {
        self.deployment.node_url.clone()
    }

    fn endpoint_addr(&self) -> H160 {
        self.deployment.endpoint
    }

    fn querier_addr(&self) -> H160 {
        self.deployment.querier
    }

    fn chain_id(&self) -> Result<U256> {
        fields_to_vars!(self, chain_id);
        Ok(chain_id)
    }

    fn book_addr(&self, product_id: u32) -> Result<H160> {
        fields_to_vars!(self, (book_addrs: clone));
        if product_id as usize >= book_addrs.len() {
            return Err(eyre!(ClientError::ProductIdOutOfBounds(product_id)));
        }
        Ok(book_addrs[product_id as usize])
    }

    fn is_rest_client(&self) -> bool {
        true
    }
}

impl <S: Signer> VertexBuilder<S> for VertexClient<S> {}

#[async_trait]
impl <S: Signer> VertexExecute<S> for VertexClient<S> {
    async fn execute(
        &self,
        execute: engine::Execute,
    ) -> Result<Option<engine::ExecuteResponseData>> {
        let url = format!("{}/execute", self.gateway_url);
        let response: ExecuteResponse = self.client.post_request(&url, &execute).await?;
        extract_response_data!(response, engine::ExecuteResponse => engine::ExecuteResponseData)
    }

    async fn execute_trigger(
        &self,
        execute: trigger::Execute,
    ) -> Result<Option<engine::ExecuteResponseData>> {
        let url = format!("{}/execute", self.trigger_url);
        let response: ExecuteResponse = self.client.post_request(&url, &execute).await?;
        extract_response_data!(response, engine::ExecuteResponse => engine::ExecuteResponseData)
    }

    async fn submit_slow_mode_tx(
        &self,
        params: SubmitSlowModeTxParams,
    ) -> Result<Option<TransactionReceipt>> {
        if params.mints_fee {
            self.mint_mock_erc20(0, SLOW_MODE_FEE).await?;
            if let Some(sleep_secs) = params.erc20_sleep_secs {
                println!("sleeping for {}s (erc20)", sleep_secs);
                tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
            }
        }
        if params.approves_fee {
            self.approve_endpoint_allowance(0, SLOW_MODE_FEE).await?;
            if let Some(sleep_secs) = params.erc20_sleep_secs {
                println!("sleeping for {}s (erc20)", sleep_secs);
                tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
            }
        }
        let endpoint = self.endpoint()?;
        let mut tx = endpoint.submit_slow_mode_transaction(params.tx);
        if let Some(gas_price) = params.gas_price {
            tx = tx.gas_price(gas_price)
        }
        let tx_receipt = tx.send().await?.log_msg("pending tx").await?;
        Ok(tx_receipt)
    }

    async fn deposit_collateral(
        &self,
        deposit_collateral_params: DepositCollateralParams,
    ) -> Result<Option<TransactionReceipt>> {
        deposit_collateral(self, deposit_collateral_params).await
    }
}

#[async_trait]
impl <S: Signer> VertexQuery<S> for VertexClient<S> {
    async fn query(&self, query: engine::Query) -> Result<engine::QueryResponseData> {
        let url = format!("{}/query", self.gateway_url);
        let response: engine::QueryResponse = self.client.post_request(&url, &query).await?;
        let data =
            extract_response_data!(response, engine::QueryResponse => engine::QueryResponseData)?;
        Ok(data.unwrap())
    }

    async fn query_trigger(&self, query: trigger::Query) -> Result<trigger::QueryResponseData> {
        let url = format!("{}/query", self.trigger_url);
        let response: trigger::QueryResponse = self.client.post_request(&url, &query).await?;
        let data =
            extract_response_data!(response, trigger::QueryResponse => trigger::QueryResponseData)?;
        Ok(data.unwrap())
    }

    async fn query_v2<R: DeserializeOwned + Send>(&self, path: &str, query: QueryV2) -> Result<R> {
        let params = serde_url_params::to_string(&query)?;
        let url = format!("{}/v2{path}?{params}", self.gateway_url.replace("/v1", ""));
        self.client.get_request(&url).await
    }
}

#[async_trait]
impl <S: Signer> VertexIndexer<S> for VertexClient<S> {
    async fn query<R: DeserializeOwned + Send>(&self, query: indexer::Query) -> Result<R> {
        self.client.post_request(&self.archive_url, &query).await
    }

    async fn query_v2<R: DeserializeOwned + Send>(
        &self,
        path: &str,
        query: indexer::QueryV2,
    ) -> Result<R> {
        let params = serde_url_params::to_string(&query)?;
        let url = format!("{}/v2{path}?{params}", self.archive_url.replace("/v1", ""));
        self.client.get_request(&url).await
    }
}
