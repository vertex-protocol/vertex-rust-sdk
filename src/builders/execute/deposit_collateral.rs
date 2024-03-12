use crate::core::execute::VertexExecute;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use ethers::types::TransactionReceipt;
use eyre::Result;
use std::time::Duration;

use crate::utils::client_error::none_error;
use crate::utils::constants::DEFAULT_RISK_CHECK_SLEEP_SECS;

vertex_builder!(
    DepositCollateralBuilder,
    VertexExecute,
    product_id: u32,
    amount: u128,
    referral_code: String,
    mints_tokens: bool,
    approves_allowance: bool,
    on_behalf_of: [u8; 32],
    risk_check_sleep_secs: u64;

    build_and_call!(self, execute, deposit_collateral => Option<TransactionReceipt>);

    pub async fn deposit_and_await_balance(&self) -> Result<Option<TransactionReceipt>> {
        let params = self.build()?;
        let product_id = params.product_id;
        let expected_balance = self.calculate_expected_balance(&params).await?;

        let receipt = self.vertex.deposit_collateral(params).await?;

        self.await_expected_balance(expected_balance, product_id).await?;
        self.sleep_for_risk_check().await;
        Ok(receipt)
    }

    async fn sleep_for_risk_check(&self) {
        let sleep_secs = self.risk_check_sleep_secs.unwrap_or(DEFAULT_RISK_CHECK_SLEEP_SECS);
        if self.vertex.is_rest_client() {
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    async fn calculate_expected_balance(&self, params: &DepositCollateralParams) -> Result<i128> {
        let pre_balance = self.spot_balance(params.product_id).await?;
        Ok(pre_balance + params.amount as i128)
    }

    async fn await_expected_balance(&self, expected_balance: i128, product_id: u32) -> Result<()> {
        loop {
            println!("waiting for deposit...");
            let product_balance = self.spot_balance(product_id).await?;
            println!("product id {product_id} balance: {product_balance}");
            if product_balance >= expected_balance {
                return Ok(())
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn spot_balance(&self, product_id: u32) -> Result<i128> {
        let subaccount_info = self.vertex.get_subaccount_info(self.get_subaccount()?).await?;
        let spot_balance = subaccount_info.get_spot_balance(product_id)?;
        Ok(spot_balance.balance.amount)
    }

    pub fn build(&self) -> Result<DepositCollateralParams> {
        let mints_tokens = self.mints_tokens.unwrap_or(false);
        let approves_allowance = self.approves_allowance.unwrap_or(true);
        let subaccount = self.get_subaccount()?;
        let referral_code = if let Some(_) = self.on_behalf_of {
            Some("-1".to_string())
        } else {
            self.referral_code.clone()
        };
        fields_to_vars!(self, product_id, amount);
        Ok(DepositCollateralParams {
            product_id,
            amount,
            subaccount,
            referral_code,
            approves_allowance,
            mints_tokens,
        })
    }

    fn get_subaccount(&self) -> Result<[u8; 32]> {
        let subaccount = if let Some(on_behalf_of) = self.on_behalf_of {
            on_behalf_of
        } else {
            self.vertex.subaccount()?
        };
        Ok(subaccount)
    }
);

pub struct DepositCollateralParams {
    pub product_id: u32,
    pub subaccount: [u8; 32],
    pub amount: u128,
    pub referral_code: Option<String>,
    pub approves_allowance: bool,
    pub mints_tokens: bool,
}
