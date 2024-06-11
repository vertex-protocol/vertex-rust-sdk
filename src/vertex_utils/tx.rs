use std::fmt::Debug;

use ethers::prelude::*;
use ethers::types::transaction::eip712::Eip712;
use ethers_core::types::transaction::eip712::EIP712Domain;
use ethers_core::utils::keccak256;
use serde::{Deserialize, Serialize};

use crate::bindings::endpoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxType {
    LiquidateSubaccount = 0,
    DepositCollateral = 1,
    WithdrawCollateral = 2,
    SpotTick = 3,
    UpdatePrice = 4,
    SettlePnl = 5,
    MatchOrders = 6,
    DepositInsurance = 7,
    ExecuteSlowMode = 8,
    MintLp = 9,
    BurnLp = 10,
    SwapAMM = 11,
    MatchOrderAMM = 12,
    DumpFees = 13,
    ClaimSequencerFees = 14,
    PerpTick = 15,
    ManualAssert = 16,
    Rebate = 17,
    UpdateProduct = 18,
    LinkSigner = 19,
    UpdateFeeRates = 20,
    BurnLpAndTransfer = 21,
    MatchOrdersRFQ = 22,
    // TODO: transfer quote is missing some more setup, eg: TxType.
    // need to confirm expected behavior, handling, etc
    TransferQuote = 23,
    RebalanceXWithdraw = 24,
}

impl TxType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::LiquidateSubaccount,
            1 => Self::DepositCollateral,
            2 => Self::WithdrawCollateral,
            3 => Self::SpotTick,
            4 => Self::UpdatePrice,
            5 => Self::SettlePnl,
            6 => Self::MatchOrders,
            7 => Self::DepositInsurance,
            8 => Self::ExecuteSlowMode,
            9 => Self::MintLp,
            10 => Self::BurnLp,
            11 => Self::SwapAMM,
            12 => Self::MatchOrderAMM,
            13 => Self::DumpFees,
            14 => Self::ClaimSequencerFees,
            15 => Self::PerpTick,
            16 => Self::ManualAssert,
            17 => Self::Rebate,
            18 => Self::UpdateProduct,
            19 => Self::LinkSigner,
            20 => Self::UpdateFeeRates,
            21 => Self::BurnLpAndTransfer,
            22 => Self::MatchOrdersRFQ,
            23 => Self::TransferQuote,
            24 => Self::RebalanceXWithdraw,
            _ => panic!("Invalid TxType"),
        }
    }
}

pub fn get_eip712_digest<T: Eip712 + Send + Sync + Debug>(
    payload: &T,
    domain: &EIP712Domain,
) -> H256 {
    let domain_separator = domain.separator();
    let struct_hash = payload.struct_hash().unwrap();
    let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();
    H256::from(keccak256(digest_input))
}

pub fn domain(chain_id: U256, verifying_contract: H160) -> EIP712Domain {
    EIP712Domain {
        name: Some("Vertex".to_string()),
        version: Some("0.0.1".to_string()),
        chain_id: Some(chain_id),
        verifying_contract: Some(verifying_contract),
        salt: None,
    }
}

pub fn domain2(chain_id: u64, verifying_contract: [u8; 20]) -> EIP712Domain {
    domain(U256::from(chain_id), verifying_contract.into())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VertexTx {
    LiquidateSubaccount(endpoint::LiquidateSubaccount),
    LinkSigner(endpoint::LinkSigner),

    DepositCollateral(endpoint::DepositCollateral),
    WithdrawCollateral(endpoint::WithdrawCollateral),

    SettlePnl(endpoint::SettlePnl),
    PerpTick(endpoint::PerpTick),
    SpotTick(endpoint::SpotTick),
    UpdatePrice(endpoint::UpdatePrice),
    ManualAssert(endpoint::ManualAssert),

    MatchOrders(endpoint::MatchOrders),
    ExecuteSlowMode,
    MintLp(endpoint::MintLp),
    BurnLp(endpoint::BurnLp),
    SwapAMM(endpoint::SwapAMM),
    BurnLpAndTransfer(endpoint::BurnLpAndTransfer),
    MatchOrdersRFQ(endpoint::MatchOrders),
    TransferQuote(endpoint::TransferQuote),
    RebalanceXWithdraw(endpoint::RebalanceXWithdraw),
    DumpFees,
    ClaimSequencerFees(endpoint::ClaimSequencerFees),
    Other,
}

impl VertexTx {
    pub fn tx_type(&self) -> TxType {
        match self {
            VertexTx::LiquidateSubaccount(_) => TxType::LiquidateSubaccount,
            VertexTx::DepositCollateral(_) => TxType::DepositCollateral,
            VertexTx::WithdrawCollateral(_) => TxType::WithdrawCollateral,
            VertexTx::SpotTick(_) => TxType::SpotTick,
            VertexTx::UpdatePrice(_) => TxType::UpdatePrice,
            VertexTx::SettlePnl(_) => TxType::SettlePnl,
            VertexTx::MatchOrders(_) => TxType::MatchOrders,
            VertexTx::ExecuteSlowMode => TxType::ExecuteSlowMode,
            VertexTx::MintLp(_) => TxType::MintLp,
            VertexTx::BurnLp(_) => TxType::BurnLp,
            VertexTx::ManualAssert(_) => TxType::ManualAssert,
            VertexTx::SwapAMM(_) => TxType::SwapAMM,
            VertexTx::PerpTick(_) => TxType::PerpTick,
            VertexTx::LinkSigner(_) => TxType::LinkSigner,
            VertexTx::BurnLpAndTransfer(_) => TxType::BurnLpAndTransfer,
            VertexTx::MatchOrdersRFQ(_) => TxType::MatchOrdersRFQ,
            VertexTx::TransferQuote(_) => TxType::TransferQuote,
            VertexTx::RebalanceXWithdraw(_) => TxType::RebalanceXWithdraw,
            VertexTx::DumpFees => TxType::DumpFees,
            VertexTx::ClaimSequencerFees(_) => TxType::ClaimSequencerFees,
            VertexTx::Other => panic!("Other is not a valid tx type"),
        }
    }
}
