pub use clearinghouse::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod clearinghouse {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Clearinghouse was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtNegativeInput\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"endpoint\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"fees\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClearinghouseInitialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"liquidatorSubaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"liquidateeSubaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amountQuote\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"insuranceCover\",\"type\":\"int128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Liquidation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ModifyCollateral\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum IProductEngine.EngineType\",\"name\":\"engineType\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addEngine\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.BurnLp\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnLp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.BurnLpAndTransfer\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"recipient\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnLpAndTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.ClaimSequencerFees\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"int128[]\",\"name\":\"fees\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimSequencerFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.DepositCollateral\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.DepositInsurance\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositInsurance\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllBooks\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClearinghouseLiq\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEngineByProduct\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"enum IProductEngine.EngineType\",\"name\":\"engineType\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEngineByType\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"enum IProductEngine.HealthType\",\"name\":\"healthType\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHealth\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"health\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInsurance\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxHealthGroup\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNumProducts\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePriceX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePricesX18\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Prices\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"spotPriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"perpPriceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderbook\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getQuote\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRisk\",\"outputs\":[{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_quote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_fees\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_clearinghouseLiq\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"insurance\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAboveInitial\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isUnderInitial\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liqDecomposeLps\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liqFinalizeSubaccount\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liqLiquidationPayment\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liqSettleAgainstLiquidator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateSubaccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateSubaccountImpl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintLp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintLpSlowMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IClearinghouseState.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"largePositionPenalty\",\"type\":\"int32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"modifyProductConfig\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"book\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IClearinghouseState.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"largePositionPenalty\",\"type\":\"int32\",\"components\":[]}]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerProductForId\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDecimals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInsurance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SettlePnl\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"subaccounts\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"productIds\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePnl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.UpdateFeeRates\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeRates\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouseLiq\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeClearinghouseLiq\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.WithdrawCollateral\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawCollateral\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CLEARINGHOUSE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Clearinghouse<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Clearinghouse<M> {
        fn clone(&self) -> Self {
            Clearinghouse(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Clearinghouse<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Clearinghouse<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Clearinghouse))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Clearinghouse<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CLEARINGHOUSE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `addEngine` (0x1750893e) function"]
        pub fn add_engine(
            &self,
            engine: ethers::core::types::Address,
            engine_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 80, 137, 62], (engine, engine_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnLp` (0xbf1fb321) function"]
        pub fn burn_lp(&self, txn: BurnLp) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 31, 179, 33], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnLpAndTransfer` (0x0748a219) function"]
        pub fn burn_lp_and_transfer(
            &self,
            txn: BurnLpAndTransfer,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 72, 162, 25], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimSequencerFees` (0xf0390afe) function"]
        pub fn claim_sequencer_fees(
            &self,
            txn: ClaimSequencerFees,
            fees: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 57, 10, 254], (txn, fees))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCollateral` (0x67271722) function"]
        pub fn deposit_collateral(
            &self,
            txn: DepositCollateral,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 39, 23, 34], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositInsurance` (0x3a91c58b) function"]
        pub fn deposit_insurance(
            &self,
            txn: DepositInsurance,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 145, 197, 139], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAllBooks` (0x354528e8) function"]
        pub fn get_all_books(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([53, 69, 40, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClearinghouseLiq` (0x9b0861c1) function"]
        pub fn get_clearinghouse_liq(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([155, 8, 97, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEndpoint` (0xaed8e967) function"]
        pub fn get_endpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([174, 216, 233, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEngineByProduct` (0xdeb14ec3) function"]
        pub fn get_engine_by_product(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([222, 177, 78, 195], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEngineByType` (0x5d2e9ad1) function"]
        pub fn get_engine_by_type(
            &self,
            engine_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([93, 46, 154, 209], engine_type)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHealth` (0x88b6496f) function"]
        pub fn get_health(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([136, 182, 73, 111], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInsurance` (0x267a8da0) function"]
        pub fn get_insurance(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([38, 122, 141, 160], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMaxHealthGroup` (0xf8fac037) function"]
        pub fn get_max_health_group(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([248, 250, 192, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNumProducts` (0x6bad77e1) function"]
        pub fn get_num_products(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([107, 173, 119, 225], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOraclePriceX18` (0x2f8f1fb0) function"]
        pub fn get_oracle_price_x18(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([47, 143, 31, 176], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOraclePricesX18` (0xd3d660cb) function"]
        pub fn get_oracle_prices_x18(
            &self,
            health_group: u32,
        ) -> ethers::contract::builders::ContractCall<M, Prices> {
            self.0
                .method_hash([211, 214, 96, 203], health_group)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderbook` (0x4427952d) function"]
        pub fn get_orderbook(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([68, 39, 149, 45], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getQuote` (0x171755b1) function"]
        pub fn get_quote(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([23, 23, 85, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRisk` (0xecd9cba8) function"]
        pub fn get_risk(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, Risk> {
            self.0
                .method_hash([236, 217, 203, 168], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf8c8765e) function"]
        pub fn initialize(
            &self,
            endpoint: ethers::core::types::Address,
            quote: ethers::core::types::Address,
            fees: ethers::core::types::Address,
            clearinghouse_liq: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (endpoint, quote, fees, clearinghouse_liq),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `insurance` (0x89cf3204) function"]
        pub fn insurance(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([137, 207, 50, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAboveInitial` (0x56bc3c38) function"]
        pub fn is_above_initial(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([86, 188, 60, 56], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isUnderInitial` (0xb5fc6205) function"]
        pub fn is_under_initial(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 252, 98, 5], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liqDecomposeLps` (0x4dafc1a6) function"]
        pub fn liq_decompose_lps(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([77, 175, 193, 166], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liqFinalizeSubaccount` (0x81210112) function"]
        pub fn liq_finalize_subaccount(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([129, 33, 1, 18], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liqLiquidationPayment` (0x191bdc11) function"]
        pub fn liq_liquidation_payment(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 27, 220, 17], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liqSettleAgainstLiquidator` (0xcb0138c3) function"]
        pub fn liq_settle_against_liquidator(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 1, 56, 195], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateSubaccount` (0x3ee8c44c) function"]
        pub fn liquidate_subaccount(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 232, 196, 76], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateSubaccountImpl` (0xf8f0722d) function"]
        pub fn liquidate_subaccount_impl(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 240, 114, 45], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintLp` (0xe671b16b) function"]
        pub fn mint_lp(&self, txn: MintLp) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 113, 177, 107], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintLpSlowMode` (0xa3d02fa6) function"]
        pub fn mint_lp_slow_mode(
            &self,
            txn: MintLp,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 208, 47, 166], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `modifyProductConfig` (0xccb6f69a) function"]
        pub fn modify_product_config(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 182, 246, 154], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerProductForId` (0x26df2414) function"]
        pub fn register_product_for_id(
            &self,
            book: ethers::core::types::Address,
            risk_store: RiskStore,
            health_group: u32,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([38, 223, 36, 20], (book, risk_store, health_group))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDecimals` (0x6302345c) function"]
        pub fn set_decimals(
            &self,
            product_id: u32,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 2, 52, 92], (product_id, decimals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEndpoint` (0xdbbb4155) function"]
        pub fn set_endpoint(
            &self,
            endpoint: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 187, 65, 85], endpoint)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setInsurance` (0x02a0f0c5) function"]
        pub fn set_insurance(
            &self,
            amount: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 160, 240, 197], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePnl` (0xb2bb6367) function"]
        pub fn settle_pnl(
            &self,
            txn: SettlePnl,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 187, 99, 103], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeRates` (0x35639a4f) function"]
        pub fn update_fee_rates(
            &self,
            txn: UpdateFeeRates,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 99, 154, 79], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeClearinghouseLiq` (0x3c54c2de) function"]
        pub fn upgrade_clearinghouse_liq(
            &self,
            clearinghouse_liq: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 84, 194, 222], clearinghouse_liq)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawCollateral` (0xc354dd90) function"]
        pub fn withdraw_collateral(
            &self,
            txn: WithdrawCollateral,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 84, 221, 144], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ClearinghouseInitialized` event"]
        pub fn clearinghouse_initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ClearinghouseInitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Liquidation` event"]
        pub fn liquidation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ModifyCollateral` event"]
        pub fn modify_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ModifyCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ClearinghouseEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Clearinghouse<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtNegativeInput` with signature `PRBMathSD59x18__SqrtNegativeInput(int256)` and selector `[193, 25, 7, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "PRBMathSD59x18__SqrtNegativeInput",
        abi = "PRBMathSD59x18__SqrtNegativeInput(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtNegativeInput {
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtOverflow` with signature `PRBMathSD59x18__SqrtOverflow(int256)` and selector `[44, 72, 44, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "PRBMathSD59x18__SqrtOverflow",
        abi = "PRBMathSD59x18__SqrtOverflow(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtOverflow {
        pub x: I256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ClearinghouseErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
    }
    impl ethers::core::abi::AbiDecode for ClearinghouseErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ClearinghouseErrors::PRBMathSD59x18__SqrtNegativeInput(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ClearinghouseErrors::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ClearinghouseErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ClearinghouseErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.encode(),
                ClearinghouseErrors::PRBMathSD59x18__SqrtOverflow(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ClearinghouseErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.fmt(f),
                ClearinghouseErrors::PRBMathSD59x18__SqrtOverflow(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtNegativeInput> for ClearinghouseErrors {
        fn from(var: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            ClearinghouseErrors::PRBMathSD59x18__SqrtNegativeInput(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtOverflow> for ClearinghouseErrors {
        fn from(var: PRBMathSD59x18__SqrtOverflow) -> Self {
            ClearinghouseErrors::PRBMathSD59x18__SqrtOverflow(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ClearinghouseInitialized",
        abi = "ClearinghouseInitialized(address,address,address)"
    )]
    pub struct ClearinghouseInitializedFilter {
        pub endpoint: ethers::core::types::Address,
        pub quote: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "Liquidation",
        abi = "Liquidation(bytes32,bytes32,uint8,uint32,int128,int128,int128)"
    )]
    pub struct LiquidationFilter {
        #[ethevent(indexed)]
        pub liquidator_subaccount: [u8; 32],
        #[ethevent(indexed)]
        pub liquidatee_subaccount: [u8; 32],
        #[ethevent(indexed)]
        pub mode: u8,
        pub health_group: u32,
        pub amount: i128,
        pub amount_quote: i128,
        pub insurance_cover: i128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ModifyCollateral",
        abi = "ModifyCollateral(int128,bytes32,uint32)"
    )]
    pub struct ModifyCollateralFilter {
        pub amount: i128,
        #[ethevent(indexed)]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ClearinghouseEvents {
        ClearinghouseInitializedFilter(ClearinghouseInitializedFilter),
        InitializedFilter(InitializedFilter),
        LiquidationFilter(LiquidationFilter),
        ModifyCollateralFilter(ModifyCollateralFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for ClearinghouseEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ClearinghouseInitializedFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::ClearinghouseInitializedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::LiquidationFilter(decoded));
            }
            if let Ok(decoded) = ModifyCollateralFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::ModifyCollateralFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ClearinghouseEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseEvents::ClearinghouseInitializedFilter(element) => element.fmt(f),
                ClearinghouseEvents::InitializedFilter(element) => element.fmt(f),
                ClearinghouseEvents::LiquidationFilter(element) => element.fmt(f),
                ClearinghouseEvents::ModifyCollateralFilter(element) => element.fmt(f),
                ClearinghouseEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addEngine` function with signature `addEngine(address,uint8)` and selector `[23, 80, 137, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addEngine", abi = "addEngine(address,uint8)")]
    pub struct AddEngineCall {
        pub engine: ethers::core::types::Address,
        pub engine_type: u8,
    }
    #[doc = "Container type for all input parameters for the `burnLp` function with signature `burnLp((bytes32,uint32,uint128,uint64))` and selector `[191, 31, 179, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnLp", abi = "burnLp((bytes32,uint32,uint128,uint64))")]
    pub struct BurnLpCall {
        pub txn: BurnLp,
    }
    #[doc = "Container type for all input parameters for the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `[7, 72, 162, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "burnLpAndTransfer",
        abi = "burnLpAndTransfer((bytes32,uint32,uint128,bytes32))"
    )]
    pub struct BurnLpAndTransferCall {
        pub txn: BurnLpAndTransfer,
    }
    #[doc = "Container type for all input parameters for the `claimSequencerFees` function with signature `claimSequencerFees((bytes32),int128[])` and selector `[240, 57, 10, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "claimSequencerFees",
        abi = "claimSequencerFees((bytes32),int128[])"
    )]
    pub struct ClaimSequencerFeesCall {
        pub txn: ClaimSequencerFees,
        pub fees: ::std::vec::Vec<i128>,
    }
    #[doc = "Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral((bytes32,uint32,uint128))` and selector `[103, 39, 23, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "depositCollateral",
        abi = "depositCollateral((bytes32,uint32,uint128))"
    )]
    pub struct DepositCollateralCall {
        pub txn: DepositCollateral,
    }
    #[doc = "Container type for all input parameters for the `depositInsurance` function with signature `depositInsurance((uint128))` and selector `[58, 145, 197, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "depositInsurance", abi = "depositInsurance((uint128))")]
    pub struct DepositInsuranceCall {
        pub txn: DepositInsurance,
    }
    #[doc = "Container type for all input parameters for the `getAllBooks` function with signature `getAllBooks()` and selector `[53, 69, 40, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAllBooks", abi = "getAllBooks()")]
    pub struct GetAllBooksCall;
    #[doc = "Container type for all input parameters for the `getClearinghouseLiq` function with signature `getClearinghouseLiq()` and selector `[155, 8, 97, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getClearinghouseLiq", abi = "getClearinghouseLiq()")]
    pub struct GetClearinghouseLiqCall;
    #[doc = "Container type for all input parameters for the `getEndpoint` function with signature `getEndpoint()` and selector `[174, 216, 233, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEndpoint", abi = "getEndpoint()")]
    pub struct GetEndpointCall;
    #[doc = "Container type for all input parameters for the `getEngineByProduct` function with signature `getEngineByProduct(uint32)` and selector `[222, 177, 78, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEngineByProduct", abi = "getEngineByProduct(uint32)")]
    pub struct GetEngineByProductCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `[93, 46, 154, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEngineByType", abi = "getEngineByType(uint8)")]
    pub struct GetEngineByTypeCall {
        pub engine_type: u8,
    }
    #[doc = "Container type for all input parameters for the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `[136, 182, 73, 111]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getHealth", abi = "getHealth(bytes32,uint8)")]
    pub struct GetHealthCall {
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    #[doc = "Container type for all input parameters for the `getInsurance` function with signature `getInsurance()` and selector `[38, 122, 141, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getInsurance", abi = "getInsurance()")]
    pub struct GetInsuranceCall;
    #[doc = "Container type for all input parameters for the `getMaxHealthGroup` function with signature `getMaxHealthGroup()` and selector `[248, 250, 192, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMaxHealthGroup", abi = "getMaxHealthGroup()")]
    pub struct GetMaxHealthGroupCall;
    #[doc = "Container type for all input parameters for the `getNumProducts` function with signature `getNumProducts()` and selector `[107, 173, 119, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getNumProducts", abi = "getNumProducts()")]
    pub struct GetNumProductsCall;
    #[doc = "Container type for all input parameters for the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `[47, 143, 31, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOraclePriceX18", abi = "getOraclePriceX18(uint32)")]
    pub struct GetOraclePriceX18Call {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `[211, 214, 96, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOraclePricesX18", abi = "getOraclePricesX18(uint32)")]
    pub struct GetOraclePricesX18Call {
        pub health_group: u32,
    }
    #[doc = "Container type for all input parameters for the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `[68, 39, 149, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOrderbook", abi = "getOrderbook(uint32)")]
    pub struct GetOrderbookCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getQuote` function with signature `getQuote()` and selector `[23, 23, 85, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getQuote", abi = "getQuote()")]
    pub struct GetQuoteCall;
    #[doc = "Container type for all input parameters for the `getRisk` function with signature `getRisk(uint32)` and selector `[236, 217, 203, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRisk", abi = "getRisk(uint32)")]
    pub struct GetRiskCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `[13, 142, 110, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getVersion", abi = "getVersion()")]
    pub struct GetVersionCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `[248, 200, 118, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub endpoint: ethers::core::types::Address,
        pub quote: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
        pub clearinghouse_liq: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `insurance` function with signature `insurance()` and selector `[137, 207, 50, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "insurance", abi = "insurance()")]
    pub struct InsuranceCall;
    #[doc = "Container type for all input parameters for the `isAboveInitial` function with signature `isAboveInitial(bytes32)` and selector `[86, 188, 60, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isAboveInitial", abi = "isAboveInitial(bytes32)")]
    pub struct IsAboveInitialCall {
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isUnderInitial` function with signature `isUnderInitial(bytes32)` and selector `[181, 252, 98, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isUnderInitial", abi = "isUnderInitial(bytes32)")]
    pub struct IsUnderInitialCall {
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `liqDecomposeLps` function with signature `liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[77, 175, 193, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liqDecomposeLps",
        abi = "liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqDecomposeLpsCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `liqFinalizeSubaccount` function with signature `liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[129, 33, 1, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liqFinalizeSubaccount",
        abi = "liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqFinalizeSubaccountCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `liqLiquidationPayment` function with signature `liqLiquidationPayment((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[25, 27, 220, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liqLiquidationPayment",
        abi = "liqLiquidationPayment((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqLiquidationPaymentCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `liqSettleAgainstLiquidator` function with signature `liqSettleAgainstLiquidator((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[203, 1, 56, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liqSettleAgainstLiquidator",
        abi = "liqSettleAgainstLiquidator((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqSettleAgainstLiquidatorCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `liquidateSubaccount` function with signature `liquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[62, 232, 196, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liquidateSubaccount",
        abi = "liquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiquidateSubaccountCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `liquidateSubaccountImpl` function with signature `liquidateSubaccountImpl((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[248, 240, 114, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "liquidateSubaccountImpl",
        abi = "liquidateSubaccountImpl((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiquidateSubaccountImplCall {
        pub txn: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `mintLp` function with signature `mintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `[230, 113, 177, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "mintLp",
        abi = "mintLp((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct MintLpCall {
        pub txn: MintLp,
    }
    #[doc = "Container type for all input parameters for the `mintLpSlowMode` function with signature `mintLpSlowMode((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `[163, 208, 47, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "mintLpSlowMode",
        abi = "mintLpSlowMode((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct MintLpSlowModeCall {
        pub txn: MintLp,
    }
    #[doc = "Container type for all input parameters for the `modifyProductConfig` function with signature `modifyProductConfig(uint32,(int32,int32,int32,int32,int32))` and selector `[204, 182, 246, 154]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "modifyProductConfig",
        abi = "modifyProductConfig(uint32,(int32,int32,int32,int32,int32))"
    )]
    pub struct ModifyProductConfigCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `registerProductForId` function with signature `registerProductForId(address,(int32,int32,int32,int32,int32),uint32)` and selector `[38, 223, 36, 20]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "registerProductForId",
        abi = "registerProductForId(address,(int32,int32,int32,int32,int32),uint32)"
    )]
    pub struct RegisterProductForIdCall {
        pub book: ethers::core::types::Address,
        pub risk_store: RiskStore,
        pub health_group: u32,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setDecimals` function with signature `setDecimals(uint32,uint8)` and selector `[99, 2, 52, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDecimals", abi = "setDecimals(uint32,uint8)")]
    pub struct SetDecimalsCall {
        pub product_id: u32,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(address)` and selector `[219, 187, 65, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(address)")]
    pub struct SetEndpointCall {
        pub endpoint: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setInsurance` function with signature `setInsurance(int128)` and selector `[2, 160, 240, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setInsurance", abi = "setInsurance(int128)")]
    pub struct SetInsuranceCall {
        pub amount: i128,
    }
    #[doc = "Container type for all input parameters for the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `[178, 187, 99, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "settlePnl", abi = "settlePnl((bytes32[],uint256[]))")]
    pub struct SettlePnlCall {
        pub txn: SettlePnl,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `[53, 99, 154, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "updateFeeRates",
        abi = "updateFeeRates((address,uint32,int64,int64))"
    )]
    pub struct UpdateFeeRatesCall {
        pub txn: UpdateFeeRates,
    }
    #[doc = "Container type for all input parameters for the `upgradeClearinghouseLiq` function with signature `upgradeClearinghouseLiq(address)` and selector `[60, 84, 194, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "upgradeClearinghouseLiq",
        abi = "upgradeClearinghouseLiq(address)"
    )]
    pub struct UpgradeClearinghouseLiqCall {
        pub clearinghouse_liq: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawCollateral` function with signature `withdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `[195, 84, 221, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "withdrawCollateral",
        abi = "withdrawCollateral((bytes32,uint32,uint128,uint64))"
    )]
    pub struct WithdrawCollateralCall {
        pub txn: WithdrawCollateral,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ClearinghouseCalls {
        AddEngine(AddEngineCall),
        BurnLp(BurnLpCall),
        BurnLpAndTransfer(BurnLpAndTransferCall),
        ClaimSequencerFees(ClaimSequencerFeesCall),
        DepositCollateral(DepositCollateralCall),
        DepositInsurance(DepositInsuranceCall),
        GetAllBooks(GetAllBooksCall),
        GetClearinghouseLiq(GetClearinghouseLiqCall),
        GetEndpoint(GetEndpointCall),
        GetEngineByProduct(GetEngineByProductCall),
        GetEngineByType(GetEngineByTypeCall),
        GetHealth(GetHealthCall),
        GetInsurance(GetInsuranceCall),
        GetMaxHealthGroup(GetMaxHealthGroupCall),
        GetNumProducts(GetNumProductsCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderbook(GetOrderbookCall),
        GetQuote(GetQuoteCall),
        GetRisk(GetRiskCall),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
        Insurance(InsuranceCall),
        IsAboveInitial(IsAboveInitialCall),
        IsUnderInitial(IsUnderInitialCall),
        LiqDecomposeLps(LiqDecomposeLpsCall),
        LiqFinalizeSubaccount(LiqFinalizeSubaccountCall),
        LiqLiquidationPayment(LiqLiquidationPaymentCall),
        LiqSettleAgainstLiquidator(LiqSettleAgainstLiquidatorCall),
        LiquidateSubaccount(LiquidateSubaccountCall),
        LiquidateSubaccountImpl(LiquidateSubaccountImplCall),
        MintLp(MintLpCall),
        MintLpSlowMode(MintLpSlowModeCall),
        ModifyProductConfig(ModifyProductConfigCall),
        Owner(OwnerCall),
        RegisterProductForId(RegisterProductForIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDecimals(SetDecimalsCall),
        SetEndpoint(SetEndpointCall),
        SetInsurance(SetInsuranceCall),
        SettlePnl(SettlePnlCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpgradeClearinghouseLiq(UpgradeClearinghouseLiqCall),
        WithdrawCollateral(WithdrawCollateralCall),
    }
    impl ethers::core::abi::AbiDecode for ClearinghouseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddEngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::AddEngine(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::BurnLp(decoded));
            }
            if let Ok(decoded) =
                <BurnLpAndTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::BurnLpAndTransfer(decoded));
            }
            if let Ok(decoded) =
                <ClaimSequencerFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::ClaimSequencerFees(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::DepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <DepositInsuranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::DepositInsurance(decoded));
            }
            if let Ok(decoded) =
                <GetAllBooksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetAllBooks(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseLiqCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetClearinghouseLiq(decoded));
            }
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetEngineByProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetEngineByProduct(decoded));
            }
            if let Ok(decoded) =
                <GetEngineByTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetEngineByType(decoded));
            }
            if let Ok(decoded) =
                <GetHealthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetHealth(decoded));
            }
            if let Ok(decoded) =
                <GetInsuranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetInsurance(decoded));
            }
            if let Ok(decoded) =
                <GetMaxHealthGroupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetMaxHealthGroup(decoded));
            }
            if let Ok(decoded) =
                <GetNumProductsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetNumProducts(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePricesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) =
                <GetOrderbookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetOrderbook(decoded));
            }
            if let Ok(decoded) =
                <GetQuoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetQuote(decoded));
            }
            if let Ok(decoded) =
                <GetRiskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetRisk(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InsuranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::Insurance(decoded));
            }
            if let Ok(decoded) =
                <IsAboveInitialCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::IsAboveInitial(decoded));
            }
            if let Ok(decoded) =
                <IsUnderInitialCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::IsUnderInitial(decoded));
            }
            if let Ok(decoded) =
                <LiqDecomposeLpsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::LiqDecomposeLps(decoded));
            }
            if let Ok(decoded) =
                <LiqFinalizeSubaccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::LiqFinalizeSubaccount(decoded));
            }
            if let Ok(decoded) =
                <LiqLiquidationPaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::LiqLiquidationPayment(decoded));
            }
            if let Ok(decoded) =
                <LiqSettleAgainstLiquidatorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ClearinghouseCalls::LiqSettleAgainstLiquidator(decoded));
            }
            if let Ok(decoded) =
                <LiquidateSubaccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::LiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <LiquidateSubaccountImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::LiquidateSubaccountImpl(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::MintLp(decoded));
            }
            if let Ok(decoded) =
                <MintLpSlowModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::MintLpSlowMode(decoded));
            }
            if let Ok(decoded) =
                <ModifyProductConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::ModifyProductConfig(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RegisterProductForIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::RegisterProductForId(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::SetDecimals(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetInsuranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::SetInsurance(decoded));
            }
            if let Ok(decoded) =
                <SettlePnlCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::SettlePnl(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) =
                <UpgradeClearinghouseLiqCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::UpgradeClearinghouseLiq(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseCalls::WithdrawCollateral(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ClearinghouseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ClearinghouseCalls::AddEngine(element) => element.encode(),
                ClearinghouseCalls::BurnLp(element) => element.encode(),
                ClearinghouseCalls::BurnLpAndTransfer(element) => element.encode(),
                ClearinghouseCalls::ClaimSequencerFees(element) => element.encode(),
                ClearinghouseCalls::DepositCollateral(element) => element.encode(),
                ClearinghouseCalls::DepositInsurance(element) => element.encode(),
                ClearinghouseCalls::GetAllBooks(element) => element.encode(),
                ClearinghouseCalls::GetClearinghouseLiq(element) => element.encode(),
                ClearinghouseCalls::GetEndpoint(element) => element.encode(),
                ClearinghouseCalls::GetEngineByProduct(element) => element.encode(),
                ClearinghouseCalls::GetEngineByType(element) => element.encode(),
                ClearinghouseCalls::GetHealth(element) => element.encode(),
                ClearinghouseCalls::GetInsurance(element) => element.encode(),
                ClearinghouseCalls::GetMaxHealthGroup(element) => element.encode(),
                ClearinghouseCalls::GetNumProducts(element) => element.encode(),
                ClearinghouseCalls::GetOraclePriceX18(element) => element.encode(),
                ClearinghouseCalls::GetOraclePricesX18(element) => element.encode(),
                ClearinghouseCalls::GetOrderbook(element) => element.encode(),
                ClearinghouseCalls::GetQuote(element) => element.encode(),
                ClearinghouseCalls::GetRisk(element) => element.encode(),
                ClearinghouseCalls::GetVersion(element) => element.encode(),
                ClearinghouseCalls::Initialize(element) => element.encode(),
                ClearinghouseCalls::Insurance(element) => element.encode(),
                ClearinghouseCalls::IsAboveInitial(element) => element.encode(),
                ClearinghouseCalls::IsUnderInitial(element) => element.encode(),
                ClearinghouseCalls::LiqDecomposeLps(element) => element.encode(),
                ClearinghouseCalls::LiqFinalizeSubaccount(element) => element.encode(),
                ClearinghouseCalls::LiqLiquidationPayment(element) => element.encode(),
                ClearinghouseCalls::LiqSettleAgainstLiquidator(element) => element.encode(),
                ClearinghouseCalls::LiquidateSubaccount(element) => element.encode(),
                ClearinghouseCalls::LiquidateSubaccountImpl(element) => element.encode(),
                ClearinghouseCalls::MintLp(element) => element.encode(),
                ClearinghouseCalls::MintLpSlowMode(element) => element.encode(),
                ClearinghouseCalls::ModifyProductConfig(element) => element.encode(),
                ClearinghouseCalls::Owner(element) => element.encode(),
                ClearinghouseCalls::RegisterProductForId(element) => element.encode(),
                ClearinghouseCalls::RenounceOwnership(element) => element.encode(),
                ClearinghouseCalls::SetDecimals(element) => element.encode(),
                ClearinghouseCalls::SetEndpoint(element) => element.encode(),
                ClearinghouseCalls::SetInsurance(element) => element.encode(),
                ClearinghouseCalls::SettlePnl(element) => element.encode(),
                ClearinghouseCalls::TransferOwnership(element) => element.encode(),
                ClearinghouseCalls::UpdateFeeRates(element) => element.encode(),
                ClearinghouseCalls::UpgradeClearinghouseLiq(element) => element.encode(),
                ClearinghouseCalls::WithdrawCollateral(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ClearinghouseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseCalls::AddEngine(element) => element.fmt(f),
                ClearinghouseCalls::BurnLp(element) => element.fmt(f),
                ClearinghouseCalls::BurnLpAndTransfer(element) => element.fmt(f),
                ClearinghouseCalls::ClaimSequencerFees(element) => element.fmt(f),
                ClearinghouseCalls::DepositCollateral(element) => element.fmt(f),
                ClearinghouseCalls::DepositInsurance(element) => element.fmt(f),
                ClearinghouseCalls::GetAllBooks(element) => element.fmt(f),
                ClearinghouseCalls::GetClearinghouseLiq(element) => element.fmt(f),
                ClearinghouseCalls::GetEndpoint(element) => element.fmt(f),
                ClearinghouseCalls::GetEngineByProduct(element) => element.fmt(f),
                ClearinghouseCalls::GetEngineByType(element) => element.fmt(f),
                ClearinghouseCalls::GetHealth(element) => element.fmt(f),
                ClearinghouseCalls::GetInsurance(element) => element.fmt(f),
                ClearinghouseCalls::GetMaxHealthGroup(element) => element.fmt(f),
                ClearinghouseCalls::GetNumProducts(element) => element.fmt(f),
                ClearinghouseCalls::GetOraclePriceX18(element) => element.fmt(f),
                ClearinghouseCalls::GetOraclePricesX18(element) => element.fmt(f),
                ClearinghouseCalls::GetOrderbook(element) => element.fmt(f),
                ClearinghouseCalls::GetQuote(element) => element.fmt(f),
                ClearinghouseCalls::GetRisk(element) => element.fmt(f),
                ClearinghouseCalls::GetVersion(element) => element.fmt(f),
                ClearinghouseCalls::Initialize(element) => element.fmt(f),
                ClearinghouseCalls::Insurance(element) => element.fmt(f),
                ClearinghouseCalls::IsAboveInitial(element) => element.fmt(f),
                ClearinghouseCalls::IsUnderInitial(element) => element.fmt(f),
                ClearinghouseCalls::LiqDecomposeLps(element) => element.fmt(f),
                ClearinghouseCalls::LiqFinalizeSubaccount(element) => element.fmt(f),
                ClearinghouseCalls::LiqLiquidationPayment(element) => element.fmt(f),
                ClearinghouseCalls::LiqSettleAgainstLiquidator(element) => element.fmt(f),
                ClearinghouseCalls::LiquidateSubaccount(element) => element.fmt(f),
                ClearinghouseCalls::LiquidateSubaccountImpl(element) => element.fmt(f),
                ClearinghouseCalls::MintLp(element) => element.fmt(f),
                ClearinghouseCalls::MintLpSlowMode(element) => element.fmt(f),
                ClearinghouseCalls::ModifyProductConfig(element) => element.fmt(f),
                ClearinghouseCalls::Owner(element) => element.fmt(f),
                ClearinghouseCalls::RegisterProductForId(element) => element.fmt(f),
                ClearinghouseCalls::RenounceOwnership(element) => element.fmt(f),
                ClearinghouseCalls::SetDecimals(element) => element.fmt(f),
                ClearinghouseCalls::SetEndpoint(element) => element.fmt(f),
                ClearinghouseCalls::SetInsurance(element) => element.fmt(f),
                ClearinghouseCalls::SettlePnl(element) => element.fmt(f),
                ClearinghouseCalls::TransferOwnership(element) => element.fmt(f),
                ClearinghouseCalls::UpdateFeeRates(element) => element.fmt(f),
                ClearinghouseCalls::UpgradeClearinghouseLiq(element) => element.fmt(f),
                ClearinghouseCalls::WithdrawCollateral(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddEngineCall> for ClearinghouseCalls {
        fn from(var: AddEngineCall) -> Self {
            ClearinghouseCalls::AddEngine(var)
        }
    }
    impl ::std::convert::From<BurnLpCall> for ClearinghouseCalls {
        fn from(var: BurnLpCall) -> Self {
            ClearinghouseCalls::BurnLp(var)
        }
    }
    impl ::std::convert::From<BurnLpAndTransferCall> for ClearinghouseCalls {
        fn from(var: BurnLpAndTransferCall) -> Self {
            ClearinghouseCalls::BurnLpAndTransfer(var)
        }
    }
    impl ::std::convert::From<ClaimSequencerFeesCall> for ClearinghouseCalls {
        fn from(var: ClaimSequencerFeesCall) -> Self {
            ClearinghouseCalls::ClaimSequencerFees(var)
        }
    }
    impl ::std::convert::From<DepositCollateralCall> for ClearinghouseCalls {
        fn from(var: DepositCollateralCall) -> Self {
            ClearinghouseCalls::DepositCollateral(var)
        }
    }
    impl ::std::convert::From<DepositInsuranceCall> for ClearinghouseCalls {
        fn from(var: DepositInsuranceCall) -> Self {
            ClearinghouseCalls::DepositInsurance(var)
        }
    }
    impl ::std::convert::From<GetAllBooksCall> for ClearinghouseCalls {
        fn from(var: GetAllBooksCall) -> Self {
            ClearinghouseCalls::GetAllBooks(var)
        }
    }
    impl ::std::convert::From<GetClearinghouseLiqCall> for ClearinghouseCalls {
        fn from(var: GetClearinghouseLiqCall) -> Self {
            ClearinghouseCalls::GetClearinghouseLiq(var)
        }
    }
    impl ::std::convert::From<GetEndpointCall> for ClearinghouseCalls {
        fn from(var: GetEndpointCall) -> Self {
            ClearinghouseCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetEngineByProductCall> for ClearinghouseCalls {
        fn from(var: GetEngineByProductCall) -> Self {
            ClearinghouseCalls::GetEngineByProduct(var)
        }
    }
    impl ::std::convert::From<GetEngineByTypeCall> for ClearinghouseCalls {
        fn from(var: GetEngineByTypeCall) -> Self {
            ClearinghouseCalls::GetEngineByType(var)
        }
    }
    impl ::std::convert::From<GetHealthCall> for ClearinghouseCalls {
        fn from(var: GetHealthCall) -> Self {
            ClearinghouseCalls::GetHealth(var)
        }
    }
    impl ::std::convert::From<GetInsuranceCall> for ClearinghouseCalls {
        fn from(var: GetInsuranceCall) -> Self {
            ClearinghouseCalls::GetInsurance(var)
        }
    }
    impl ::std::convert::From<GetMaxHealthGroupCall> for ClearinghouseCalls {
        fn from(var: GetMaxHealthGroupCall) -> Self {
            ClearinghouseCalls::GetMaxHealthGroup(var)
        }
    }
    impl ::std::convert::From<GetNumProductsCall> for ClearinghouseCalls {
        fn from(var: GetNumProductsCall) -> Self {
            ClearinghouseCalls::GetNumProducts(var)
        }
    }
    impl ::std::convert::From<GetOraclePriceX18Call> for ClearinghouseCalls {
        fn from(var: GetOraclePriceX18Call) -> Self {
            ClearinghouseCalls::GetOraclePriceX18(var)
        }
    }
    impl ::std::convert::From<GetOraclePricesX18Call> for ClearinghouseCalls {
        fn from(var: GetOraclePricesX18Call) -> Self {
            ClearinghouseCalls::GetOraclePricesX18(var)
        }
    }
    impl ::std::convert::From<GetOrderbookCall> for ClearinghouseCalls {
        fn from(var: GetOrderbookCall) -> Self {
            ClearinghouseCalls::GetOrderbook(var)
        }
    }
    impl ::std::convert::From<GetQuoteCall> for ClearinghouseCalls {
        fn from(var: GetQuoteCall) -> Self {
            ClearinghouseCalls::GetQuote(var)
        }
    }
    impl ::std::convert::From<GetRiskCall> for ClearinghouseCalls {
        fn from(var: GetRiskCall) -> Self {
            ClearinghouseCalls::GetRisk(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for ClearinghouseCalls {
        fn from(var: GetVersionCall) -> Self {
            ClearinghouseCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for ClearinghouseCalls {
        fn from(var: InitializeCall) -> Self {
            ClearinghouseCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InsuranceCall> for ClearinghouseCalls {
        fn from(var: InsuranceCall) -> Self {
            ClearinghouseCalls::Insurance(var)
        }
    }
    impl ::std::convert::From<IsAboveInitialCall> for ClearinghouseCalls {
        fn from(var: IsAboveInitialCall) -> Self {
            ClearinghouseCalls::IsAboveInitial(var)
        }
    }
    impl ::std::convert::From<IsUnderInitialCall> for ClearinghouseCalls {
        fn from(var: IsUnderInitialCall) -> Self {
            ClearinghouseCalls::IsUnderInitial(var)
        }
    }
    impl ::std::convert::From<LiqDecomposeLpsCall> for ClearinghouseCalls {
        fn from(var: LiqDecomposeLpsCall) -> Self {
            ClearinghouseCalls::LiqDecomposeLps(var)
        }
    }
    impl ::std::convert::From<LiqFinalizeSubaccountCall> for ClearinghouseCalls {
        fn from(var: LiqFinalizeSubaccountCall) -> Self {
            ClearinghouseCalls::LiqFinalizeSubaccount(var)
        }
    }
    impl ::std::convert::From<LiqLiquidationPaymentCall> for ClearinghouseCalls {
        fn from(var: LiqLiquidationPaymentCall) -> Self {
            ClearinghouseCalls::LiqLiquidationPayment(var)
        }
    }
    impl ::std::convert::From<LiqSettleAgainstLiquidatorCall> for ClearinghouseCalls {
        fn from(var: LiqSettleAgainstLiquidatorCall) -> Self {
            ClearinghouseCalls::LiqSettleAgainstLiquidator(var)
        }
    }
    impl ::std::convert::From<LiquidateSubaccountCall> for ClearinghouseCalls {
        fn from(var: LiquidateSubaccountCall) -> Self {
            ClearinghouseCalls::LiquidateSubaccount(var)
        }
    }
    impl ::std::convert::From<LiquidateSubaccountImplCall> for ClearinghouseCalls {
        fn from(var: LiquidateSubaccountImplCall) -> Self {
            ClearinghouseCalls::LiquidateSubaccountImpl(var)
        }
    }
    impl ::std::convert::From<MintLpCall> for ClearinghouseCalls {
        fn from(var: MintLpCall) -> Self {
            ClearinghouseCalls::MintLp(var)
        }
    }
    impl ::std::convert::From<MintLpSlowModeCall> for ClearinghouseCalls {
        fn from(var: MintLpSlowModeCall) -> Self {
            ClearinghouseCalls::MintLpSlowMode(var)
        }
    }
    impl ::std::convert::From<ModifyProductConfigCall> for ClearinghouseCalls {
        fn from(var: ModifyProductConfigCall) -> Self {
            ClearinghouseCalls::ModifyProductConfig(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ClearinghouseCalls {
        fn from(var: OwnerCall) -> Self {
            ClearinghouseCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RegisterProductForIdCall> for ClearinghouseCalls {
        fn from(var: RegisterProductForIdCall) -> Self {
            ClearinghouseCalls::RegisterProductForId(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ClearinghouseCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ClearinghouseCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetDecimalsCall> for ClearinghouseCalls {
        fn from(var: SetDecimalsCall) -> Self {
            ClearinghouseCalls::SetDecimals(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for ClearinghouseCalls {
        fn from(var: SetEndpointCall) -> Self {
            ClearinghouseCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetInsuranceCall> for ClearinghouseCalls {
        fn from(var: SetInsuranceCall) -> Self {
            ClearinghouseCalls::SetInsurance(var)
        }
    }
    impl ::std::convert::From<SettlePnlCall> for ClearinghouseCalls {
        fn from(var: SettlePnlCall) -> Self {
            ClearinghouseCalls::SettlePnl(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ClearinghouseCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ClearinghouseCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateFeeRatesCall> for ClearinghouseCalls {
        fn from(var: UpdateFeeRatesCall) -> Self {
            ClearinghouseCalls::UpdateFeeRates(var)
        }
    }
    impl ::std::convert::From<UpgradeClearinghouseLiqCall> for ClearinghouseCalls {
        fn from(var: UpgradeClearinghouseLiqCall) -> Self {
            ClearinghouseCalls::UpgradeClearinghouseLiq(var)
        }
    }
    impl ::std::convert::From<WithdrawCollateralCall> for ClearinghouseCalls {
        fn from(var: WithdrawCollateralCall) -> Self {
            ClearinghouseCalls::WithdrawCollateral(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAllBooks` function with signature `getAllBooks()` and selector `[53, 69, 40, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAllBooksReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getClearinghouseLiq` function with signature `getClearinghouseLiq()` and selector `[155, 8, 97, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetClearinghouseLiqReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getEndpoint` function with signature `getEndpoint()` and selector `[174, 216, 233, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEndpointReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getEngineByProduct` function with signature `getEngineByProduct(uint32)` and selector `[222, 177, 78, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEngineByProductReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `[93, 46, 154, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEngineByTypeReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `[136, 182, 73, 111]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetHealthReturn {
        pub health: i128,
    }
    #[doc = "Container type for all return fields from the `getInsurance` function with signature `getInsurance()` and selector `[38, 122, 141, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInsuranceReturn(pub i128);
    #[doc = "Container type for all return fields from the `getMaxHealthGroup` function with signature `getMaxHealthGroup()` and selector `[248, 250, 192, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMaxHealthGroupReturn(pub u32);
    #[doc = "Container type for all return fields from the `getNumProducts` function with signature `getNumProducts()` and selector `[107, 173, 119, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetNumProductsReturn(pub u32);
    #[doc = "Container type for all return fields from the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `[47, 143, 31, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOraclePriceX18Return(pub i128);
    #[doc = "Container type for all return fields from the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `[211, 214, 96, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOraclePricesX18Return(pub Prices);
    #[doc = "Container type for all return fields from the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `[68, 39, 149, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOrderbookReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getQuote` function with signature `getQuote()` and selector `[23, 23, 85, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetQuoteReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getRisk` function with signature `getRisk(uint32)` and selector `[236, 217, 203, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRiskReturn(pub Risk);
    #[doc = "Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `[13, 142, 110, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetVersionReturn(pub u64);
    #[doc = "Container type for all return fields from the `insurance` function with signature `insurance()` and selector `[137, 207, 50, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InsuranceReturn(pub i128);
    #[doc = "Container type for all return fields from the `isAboveInitial` function with signature `isAboveInitial(bytes32)` and selector `[86, 188, 60, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsAboveInitialReturn(pub bool);
    #[doc = "Container type for all return fields from the `isUnderInitial` function with signature `isUnderInitial(bytes32)` and selector `[181, 252, 98, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsUnderInitialReturn(pub bool);
    #[doc = "Container type for all return fields from the `liqDecomposeLps` function with signature `liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[77, 175, 193, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiqDecomposeLpsReturn(pub bool);
    #[doc = "Container type for all return fields from the `liqFinalizeSubaccount` function with signature `liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[129, 33, 1, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiqFinalizeSubaccountReturn(pub bool);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `registerProductForId` function with signature `registerProductForId(address,(int32,int32,int32,int32,int32),uint32)` and selector `[38, 223, 36, 20]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RegisterProductForIdReturn {
        pub product_id: u32,
    }
    #[doc = "`RiskStore(int32,int32,int32,int32,int32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub large_position_penalty: i32,
    }
    #[doc = "`BurnLp(bytes32,uint32,uint128,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BurnLp {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub nonce: u64,
    }
    #[doc = "`BurnLpAndTransfer(bytes32,uint32,uint128,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BurnLpAndTransfer {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub recipient: [u8; 32],
    }
    #[doc = "`ClaimSequencerFees(bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ClaimSequencerFees {
        pub subaccount: [u8; 32],
    }
    #[doc = "`DepositCollateral(bytes32,uint32,uint128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DepositCollateral {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
    }
    #[doc = "`DepositInsurance(uint128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DepositInsurance {
        pub amount: u128,
    }
    #[doc = "`LiquidateSubaccount(bytes32,bytes32,uint8,uint32,int128,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidateSubaccount {
        pub sender: [u8; 32],
        pub liquidatee: [u8; 32],
        pub mode: u8,
        pub health_group: u32,
        pub amount: i128,
        pub nonce: u64,
    }
    #[doc = "`MintLp(bytes32,uint32,uint128,uint128,uint128,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintLp {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount_base: u128,
        pub quote_amount_low: u128,
        pub quote_amount_high: u128,
        pub nonce: u64,
    }
    #[doc = "`Prices(int128,int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Prices {
        pub spot_price_x18: i128,
        pub perp_price_x18: i128,
    }
    #[doc = "`SettlePnl(bytes32[],uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SettlePnl {
        pub subaccounts: Vec<[u8; 32]>,
        pub product_ids: Vec<ethers::core::types::U256>,
    }
    #[doc = "`UpdateFeeRates(address,uint32,int64,int64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UpdateFeeRates {
        pub user: ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    #[doc = "`WithdrawCollateral(bytes32,uint32,uint128,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WithdrawCollateral {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub nonce: u64,
    }
    #[doc = "`Risk(int128,int128,int128,int128,int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Risk {
        pub long_weight_initial_x18: i128,
        pub short_weight_initial_x18: i128,
        pub long_weight_maintenance_x18: i128,
        pub short_weight_maintenance_x18: i128,
        pub large_position_penalty_x18: i128,
    }
}
