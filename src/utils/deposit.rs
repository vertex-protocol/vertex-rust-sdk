use crate::bindings::endpoint::Endpoint;
use crate::bindings::mock_erc20::MockERC20;
use crate::builders::execute::deposit_collateral::DepositCollateralParams;
use crate::core::execute::VertexExecute;
use crate::eip712_structs::{from_bytes32, to_bytes12};
use crate::provider::VertexProvider;
use crate::revert::parse_provider_error;
use crate::utils::constants::DEFAULT_PENDING_TX_RETIRES;
use ethers_contract::ContractError;
use ethers_core::types::TransactionReceipt;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Provider;
use eyre::{eyre, Result};
use std::sync::Arc;
use std::time::Duration;
use crate::utils::signer::Signer;

pub async fn deposit_collateral<S: Signer, V: VertexExecute<S> + Sync>(
    vertex: &V,
    deposit_collateral_params: DepositCollateralParams,
) -> Result<Option<TransactionReceipt>> {
    let amount = deposit_collateral_params.amount;
    let product_id = deposit_collateral_params.product_id;

    if deposit_collateral_params.mints_tokens {
        vertex.mint_mock_erc20(product_id, amount).await?;
        if let Some(sleep_secs) = deposit_collateral_params.erc20_sleep_secs {
            println!("sleeping for {}s (erc20)", sleep_secs);
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    if deposit_collateral_params.approves_allowance {
        vertex
            .approve_with_gas_price(
                vertex.endpoint_addr(),
                product_id,
                amount,
                deposit_collateral_params.gas_price,
            )
            .await?;
        if let Some(sleep_secs) = deposit_collateral_params.erc20_sleep_secs {
            println!("sleeping for {}s (erc20)", sleep_secs);
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    endpoint_deposit_call(vertex, &deposit_collateral_params).await
}

pub fn provider_with_signer<S: Signer, V: VertexExecute<S>>(vertex: &V) -> Result<Arc<VertexProvider<S>>> {
    let provider = Provider::new_client(&vertex.node_url(), 15, 500)?;
    let wallet = vertex.wallet()?.clone();
    Ok(Arc::new(SignerMiddleware::new(
        provider.interval(Duration::from_millis(500)),
        wallet,
    )))
}

pub async fn erc20_client<S: Signer, V: VertexExecute<S> + Sync>(
    vertex: &V,
    product_id: u32,
) -> Result<MockERC20<VertexProvider<S>>> {
    let token_address = vertex.get_token_address(product_id).await?;
    let client = provider_with_signer(vertex)?;
    let remote_quote = MockERC20::new(token_address, client.clone());
    Ok(remote_quote)
}

pub async fn endpoint_deposit_call<S: Signer, V: VertexExecute<S>>(
    vertex: &V,
    deposit_collateral_params: &DepositCollateralParams,
) -> Result<Option<TransactionReceipt>> {
    let product_id = deposit_collateral_params.product_id;
    let amount = deposit_collateral_params.amount;
    let subaccount = deposit_collateral_params.subaccount;

    let endpoint: Endpoint<VertexProvider<S>> = vertex.endpoint()?;
    let mut tx = if let Some(referral_code) = deposit_collateral_params.referral_code.clone() {
        endpoint.deposit_collateral_with_referral(subaccount, product_id, amount, referral_code)
    } else {
        let (_, subaccount_name) = from_bytes32(subaccount);
        let subaccount_name = to_bytes12(subaccount_name.as_str());
        endpoint.deposit_collateral(subaccount_name, product_id, amount)
    };

    if let Some(gas_price) = deposit_collateral_params.gas_price {
        tx = tx.gas_price(gas_price);
    }

    let pending_tx = tx.send().await;
    let pending_tx = match pending_tx {
        Ok(tx) => tx,
        Err(e) => return Err(eyre!(parse_provider_error(e))),
    };
    let tx_receipt = pending_tx
        .retries(DEFAULT_PENDING_TX_RETIRES)
        .log_msg("pending tx")
        .await;
    match tx_receipt {
        Ok(receipt) => Ok(receipt),
        Err(e) => Err(eyre!(parse_provider_error(
            ContractError::<VertexProvider<S>>::ProviderError { e }
        ))),
    }
}
