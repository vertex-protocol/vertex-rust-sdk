use crate::bindings::endpoint::Endpoint;
use crate::bindings::mock_erc20::MockERC20;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::core::execute::VertexExecute;
use crate::provider::VertexProvider;
use ethers::prelude::TxHash;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Provider;
use eyre::Result;
use std::sync::Arc;
use std::time::Duration;

pub async fn deposit_collateral<V: VertexExecute + Sync>(
    vertex: &V,
    deposit_collateral_params: DepositCollateralParams,
) -> Result<TxHash> {
    let amount = deposit_collateral_params.deposit_collateral_call.amount;
    let product_id = deposit_collateral_params.deposit_collateral_call.product_id;

    if deposit_collateral_params.mints_tokens {
        vertex.mint_mock_erc20(product_id, amount).await?;
    }

    if deposit_collateral_params.approves_allowance {
        vertex.approve_allowance(product_id, amount).await?;
    }

    endpoint_deposit_call(vertex, &deposit_collateral_params).await
}

pub fn provider_with_signer<V: VertexExecute>(vertex: &V) -> Result<Arc<VertexProvider>> {
    println!("node url: {}", vertex.node_url());
    let provider = Provider::new_client(&vertex.node_url(), 15, 500)?;
    let wallet = vertex.wallet()?.clone();
    Ok(Arc::new(SignerMiddleware::new(
        provider.interval(Duration::from_millis(500)),
        wallet,
    )))
}

pub async fn erc20_client<V: VertexExecute + Sync>(
    vertex: &V,
    product_id: u32,
) -> Result<MockERC20<VertexProvider>> {
    let token_address = vertex.get_token_address(product_id).await?;
    let client = provider_with_signer(vertex)?;
    let remote_quote = MockERC20::new(token_address, client.clone());
    Ok(remote_quote)
}

pub async fn endpoint_deposit_call<V: VertexExecute>(
    vertex: &V,
    deposit_collateral_params: &DepositCollateralParams,
) -> Result<TxHash> {
    let product_id = deposit_collateral_params.deposit_collateral_call.product_id;
    let amount = deposit_collateral_params.deposit_collateral_call.amount;
    let subaccount_name = deposit_collateral_params
        .deposit_collateral_call
        .subaccount_name;

    let endpoint: Endpoint<VertexProvider> = vertex.endpoint()?;
    let tx = if let Some(referral_code) = deposit_collateral_params.referral_code.clone() {
        endpoint.deposit_collateral_with_referral_with_subaccount_name_and_product_id_and_amount(
            subaccount_name,
            product_id,
            amount,
            referral_code,
        )
    } else {
        endpoint.deposit_collateral(subaccount_name, product_id, amount)
    };
    let tx_hash = tx.send().await?.tx_hash();
    Ok(tx_hash)
}
