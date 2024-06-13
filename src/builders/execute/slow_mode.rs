use crate::bindings::endpoint::{endpoint, SwapAMM, SwapAMMReturn, UnsignedMintLpReturn};
use crate::tx::TxType;
use ethers::types::Bytes;
use eyre::Result;
use std::time::Duration;

use crate::core::execute::VertexExecute;
use crate::utils::client_error::none_error;
use crate::utils::constants::DEFAULT_SLOW_MODE_SLEEP_SECS;
use crate::{build_and_call, fields_to_vars, vertex_builder};
use ethers::abi::AbiEncode;
use ethers::types::TransactionReceipt;

vertex_builder!(
    SubmitSlowModeTxBuilder,
    VertexExecute,
    tx: Bytes,
    mints_fee: bool,
    approves_fee: bool,
    sleep_secs: u64,
    erc20_sleep_secs: u64,
    gas_price: u128;

    pub async fn execute_and_sleep(&self) -> Result<Option<TransactionReceipt>> {
        let tx_hash = self.execute().await?;
        self.sleep().await;
        Ok(tx_hash)
    }

    build_and_call!(self, execute, submit_slow_mode_tx => Option<TransactionReceipt>);

    async fn sleep(&self) {
        let sleep_secs = self.sleep_secs.unwrap_or(DEFAULT_SLOW_MODE_SLEEP_SECS);
        if self.vertex.is_rest_client() {
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }


    pub fn swap_amm_tx(&self, swap_amm: SwapAMM) -> Self {
        let tx = swap_amm_bytes(swap_amm);
        self.tx(tx)
    }

    pub fn withdraw_collateral_tx(&self, withdraw_collateral: endpoint::WithdrawCollateral) -> Self {
        let tx = withdraw_collateral_bytes(withdraw_collateral);
        self.tx(tx)
    }

    pub fn mint_lp_tx(&self, mint_lp_tx: endpoint::MintLp) -> Self {
        let tx = mint_lp_bytes(mint_lp_tx);
        self.tx(tx)
    }

    pub fn build(&self) -> Result<SubmitSlowModeTxParams> {
        fields_to_vars!(self, (tx: clone));
        let mints_fee = self.mints_fee.unwrap_or(false);
        let approves_fee = self.approves_fee.unwrap_or(true);

        Ok(SubmitSlowModeTxParams {
            tx,
            mints_fee,
            approves_fee,
            erc20_sleep_secs: self.erc20_sleep_secs,
            gas_price: self.gas_price,
        })
    }

);

pub struct SubmitSlowModeTxParams {
    pub tx: Bytes,
    pub mints_fee: bool,
    pub approves_fee: bool,
    pub erc20_sleep_secs: Option<u64>,
    pub gas_price: Option<u128>,
}

fn swap_amm_bytes(swap_amm: endpoint::SwapAMM) -> Bytes {
    let swap_amm_return = SwapAMMReturn(swap_amm);

    Bytes::from(
        [
            vec![TxType::SwapAMM as u8],
            AbiEncode::encode(swap_amm_return),
        ]
        .concat(),
    )
}

fn withdraw_collateral_bytes(withdraw_collateral: endpoint::WithdrawCollateral) -> Bytes {
    let withdraw_tx_bytes = AbiEncode::encode(endpoint::UnsignedWithdrawCollateralReturn(
        withdraw_collateral,
    ));

    Bytes::from([vec![TxType::WithdrawCollateral as u8], withdraw_tx_bytes].concat())
}

fn mint_lp_bytes(mint_lp: endpoint::MintLp) -> Bytes {
    let mint_lp_return = UnsignedMintLpReturn(mint_lp);

    Bytes::from(
        [
            vec![TxType::MintLp as u8],
            AbiEncode::encode(mint_lp_return),
        ]
        .concat(),
    )
}
