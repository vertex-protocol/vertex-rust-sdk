use std::time::Duration;

use ethers::abi::AbiEncode;
use ethers::types::Bytes;
use ethers::types::TransactionReceipt;
use eyre::Result;

use crate::bindings::endpoint::endpoint;
use crate::math::to_i128_x18;
use crate::tx::TxType;

use crate::builders::execute::slow_mode::SubmitSlowModeTxParams;
use crate::core::execute::VertexExecute;
use crate::utils::client_error::none_error;
use crate::{build_and_call, fields_to_vars, vertex_builder};

const ONE_X6: i128 = 1_000_000;
vertex_builder!(
    DepositInsuranceBuilder,
    VertexExecute,
    amount: u128,
    mints_tokens: bool,
    approves_allowance: bool;

    pub async fn deposit_and_await_balance(&self) -> Result<Option<TransactionReceipt>> {
        let expected_balance = self.calculate_expected_balance().await?;
        if self.vertex.is_rest_client() {
            self.handle_erc20().await?;
        }
        let tx_hash = self.execute().await?;
        self.await_expected_balance(expected_balance).await?;
        Ok(tx_hash)
    }

    async fn handle_erc20(&self) -> Result<()> {
        fields_to_vars!(self, amount);
        if self.mints_tokens.unwrap_or(false) {
            self.vertex.mint_mock_erc20(0, amount).await?;
        }
        if self.approves_allowance.unwrap_or(false) {
            self.vertex.approve_allowance(0, amount).await?;
        }
        Ok(())
    }

    async fn calculate_expected_balance(&self) -> Result<i128> {
        fields_to_vars!(self, amount);
        let mut amount = amount as i128 / ONE_X6;
        amount = to_i128_x18(amount);
        let pre_balance = self.vertex.get_insurance().await?.insurance;

        Ok(pre_balance + amount)
    }

    async fn await_expected_balance(&self, expected_balance: i128) -> Result<()> {
        loop {
            println!("waiting for deposit...");
            let insurance = self.vertex.get_insurance().await?.insurance;
            if insurance >= expected_balance {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    build_and_call!(self, execute, submit_slow_mode_tx => Option<TransactionReceipt>);

    pub fn build(&self) -> Result<SubmitSlowModeTxParams> {
        fields_to_vars!(self, amount);

        let tx = deposit_insurance_bytes(amount);

        Ok(SubmitSlowModeTxParams {
            tx,
            mints_fee: false,
            approves_fee: false,
        })
    }

);

// TODO: this only deposits into cross group
pub fn deposit_insurance_bytes(amount: u128) -> Bytes {
    let tx_bytes = AbiEncode::encode(endpoint::UnsignedDepositInsuranceReturn(
        endpoint::DepositInsurance { amount },
    ));
    Bytes::from([vec![TxType::DepositInsurance as u8], tx_bytes].concat())
}
