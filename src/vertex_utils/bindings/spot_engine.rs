pub use spot_engine::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod spot_engine {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

    use super::super::super::serialize_utils::{
        deserialize_bytes32, deserialize_i128, deserialize_i256, deserialize_nested_vec_i128,
        deserialize_u128, deserialize_u256, deserialize_vec_i128, serialize_bytes32,
        serialize_i128, serialize_i256, serialize_nested_vec_i128, serialize_u128, serialize_u256,
        serialize_vec_i128,
    };
    use serde::{Deserialize, Serialize};

    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "SpotEngine was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddProduct\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BalanceUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProductUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"isoGroup\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"QuoteProductUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"book\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct RiskHelper.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addProduct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assertUtilization\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountLp\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnLp\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"amountBase\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountQuote\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"isoGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidator\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decomposeLps\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"liquidationFees\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalance\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClearinghouse\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfig\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"enum IProductEngine.HealthType\",\"name\":\"healthType\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCoreRisk\",\"outputs\":[{\"internalType\":\"struct IProductEngine.CoreRisk\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"price\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeight\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getEngineType\",\"outputs\":[{\"internalType\":\"enum IProductEngine.EngineType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"enum IProductEngine.HealthType\",\"name\":\"healthType\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHealthContribution\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"health\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolState\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProductIds\",\"outputs\":[{\"internalType\":\"uint32[]\",\"name\":\"\",\"type\":\"uint32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"isoGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProductIds\",\"outputs\":[{\"internalType\":\"uint32[]\",\"name\":\"\",\"type\":\"uint32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRawBalance\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.Balances\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ISpotEngine.BalanceNormalized\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amountNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRawLpState\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRawState\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRisk\",\"outputs\":[{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStateAndBalance\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStatesAndBalances\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenBalance\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"balance\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalBalances\",\"outputs\":[{\"internalType\":\"int128[]\",\"name\":\"deposits\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"borrows\",\"type\":\"int128[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getWithdrawFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_offchainExchange\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_quote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128[]\",\"name\":\"totalDeposits\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"totalBorrows\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manualAssert\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountBase\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteAmountLow\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteAmountHigh\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintLp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLpBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLpState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"socializeSubaccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapLp\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ISpotEngine.UpdateProductTx\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct RiskHelper.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedUpdateProductTx\",\"outputs\":[{\"internalType\":\"struct ISpotEngine.UpdateProductTx\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct RiskHelper.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"rawTxn\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateProduct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"insurance\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateQuoteFromInsurance\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct RiskHelper.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateRisk\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"dt\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"globalUtilRatiosX18\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateStates\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static SPOTENGINE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct SpotEngine<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SpotEngine<M> {
        fn clone(&self) -> Self {
            SpotEngine(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SpotEngine<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for SpotEngine<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SpotEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SpotEngine<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SPOTENGINE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addProduct` (0x2d243454) function"]
        pub fn add_product(
            &self,
            product_id: u32,
            book: ethers::core::types::Address,
            size_increment: i128,
            min_size: i128,
            lp_spread_x18: i128,
            config: Config,
            risk_store: RiskStore,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 36, 52, 84],
                    (
                        product_id,
                        book,
                        size_increment,
                        min_size,
                        lp_spread_x18,
                        config,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assertUtilization` (0x4ac8d8c1) function"]
        pub fn assert_utilization(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 200, 216, 193], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnLp` (0xd98752ec) function"]
        pub fn burn_lp(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_lp: i128,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([217, 135, 82, 236], (product_id, subaccount, amount_lp))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decomposeLps` (0xb15b8256) function"]
        pub fn decompose_lps(
            &self,
            iso_group: u32,
            liquidatee: [u8; 32],
            liquidator: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([177, 91, 130, 86], (iso_group, liquidatee, liquidator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBalance` (0x7c1e1487) function"]
        pub fn get_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([124, 30, 20, 135], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClearinghouse` (0xb1cb0f42) function"]
        pub fn get_clearinghouse(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([177, 203, 15, 66], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfig` (0xe343738c) function"]
        pub fn get_config(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, Config> {
            self.0
                .method_hash([227, 67, 115, 140], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCoreRisk` (0x8a1d43c9) function"]
        pub fn get_core_risk(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            health_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, CoreRisk> {
            self.0
                .method_hash([138, 29, 67, 201], (subaccount, product_id, health_type))
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
        #[doc = "Calls the contract's `getEngineType` (0x4604d19b) function"]
        pub fn get_engine_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([70, 4, 209, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHealthContribution` (0x871d0912) function"]
        pub fn get_health_contribution(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([135, 29, 9, 18], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolState` (0x8af6426a) function"]
        pub fn get_pool_state(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([138, 246, 66, 106], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProductIds` (0x47428e7b) function"]
        pub fn get_product_ids(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([71, 66, 142, 123], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProductIds` (0xf4c8c58d) function"]
        pub fn get_product_ids_with_iso_group(
            &self,
            iso_group: u32,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([244, 200, 197, 141], iso_group)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRawBalance` (0xedf02653) function"]
        pub fn get_raw_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, Balances> {
            self.0
                .method_hash([237, 240, 38, 83], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRawLpState` (0xc721bd65) function"]
        pub fn get_raw_lp_state(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, LpState> {
            self.0
                .method_hash([199, 33, 189, 101], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRawState` (0xec7a79c9) function"]
        pub fn get_raw_state(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, State> {
            self.0
                .method_hash([236, 122, 121, 201], product_id)
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
        #[doc = "Calls the contract's `getStateAndBalance` (0xe334be33) function"]
        pub fn get_state_and_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (State, Balance)> {
            self.0
                .method_hash([227, 52, 190, 51], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStatesAndBalances` (0x3d5cc9dc) function"]
        pub fn get_states_and_balances(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (LpState, LpBalance, State, Balance)>
        {
            self.0
                .method_hash([61, 92, 201, 220], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getToken` (0x45be7ed6) function"]
        pub fn get_token(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([69, 190, 126, 214], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenBalance` (0xa67ac322) function"]
        pub fn get_token_balance(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([166, 122, 195, 34], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalBalances` (0x2baf57d3) function"]
        pub fn get_total_balances(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i128>, ::std::vec::Vec<i128>),
        > {
            self.0
                .method_hash([43, 175, 87, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWithdrawFee` (0xfdf4a0c0) function"]
        pub fn get_withdraw_fee(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([253, 244, 160, 192], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x1459457a) function"]
        pub fn initialize(
            &self,
            clearinghouse: ethers::core::types::Address,
            offchain_exchange: ethers::core::types::Address,
            quote: ethers::core::types::Address,
            endpoint: ethers::core::types::Address,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, offchain_exchange, quote, endpoint, admin),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manualAssert` (0x30972b50) function"]
        pub fn manual_assert(
            &self,
            total_deposits: ::std::vec::Vec<i128>,
            total_borrows: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 151, 43, 80], (total_deposits, total_borrows))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintLp` (0x98de72fe) function"]
        pub fn mint_lp(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_base: i128,
            quote_amount_low: i128,
            quote_amount_high: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 222, 114, 254],
                    (
                        product_id,
                        subaccount,
                        amount_base,
                        quote_amount_low,
                        quote_amount_high,
                    ),
                )
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setConfig` (0x10ca9388) function"]
        pub fn set_config(
            &self,
            product_id: u32,
            config: Config,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 202, 147, 136], (product_id, config))
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
        #[doc = "Calls the contract's `setLpBalance` (0xf8661884) function"]
        pub fn set_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            lp_balance: LpBalance,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 102, 24, 132], (product_id, subaccount, lp_balance))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLpState` (0x9bb91f6a) function"]
        pub fn set_lp_state(
            &self,
            product_id: u32,
            lp_state: LpState,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 185, 31, 106], (product_id, lp_state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setState` (0x7fa29d49) function"]
        pub fn set_state(
            &self,
            product_id: u32,
            state: State,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 162, 157, 73], (product_id, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `socializeSubaccount` (0x8936f7cd) function"]
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 54, 247, 205], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapLp` (0xc7167cf5) function"]
        pub fn swap_lp(
            &self,
            product_id: u32,
            base_delta: i128,
            quote_delta: i128,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([199, 22, 124, 245], (product_id, base_delta, quote_delta))
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
        #[doc = "Calls the contract's `unsignedUpdateProductTx` (0x49f73568) function"]
        pub fn unsigned_update_product_tx(
            &self,
            p: UpdateProductTx,
        ) -> ethers::contract::builders::ContractCall<M, UpdateProductTx> {
            self.0
                .method_hash([73, 247, 53, 104], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBalance` (0xe0b0621f) function"]
        pub fn update_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 176, 98, 31], (product_id, subaccount, amount_delta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBalance` (0xf8a42e51) function"]
        pub fn update_balance_with_product_id_and_subaccount_and_quote_delta(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
            quote_delta: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 164, 46, 81],
                    (product_id, subaccount, amount_delta, quote_delta),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePrice` (0x153ca6c0) function"]
        pub fn update_price(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 60, 166, 192], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateProduct` (0xc9fe9ac3) function"]
        pub fn update_product(
            &self,
            raw_txn: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 254, 154, 195], raw_txn)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateQuoteFromInsurance` (0xf39eeb10) function"]
        pub fn update_quote_from_insurance(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([243, 158, 235, 16], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateRisk` (0xc55607b5) function"]
        pub fn update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 86, 7, 181], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateStates` (0x6736f5da) function"]
        pub fn update_states(
            &self,
            dt: u128,
            global_util_ratios_x18: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 54, 245, 218], (dt, global_util_ratios_x18))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddProduct` event"]
        pub fn add_product_filter(&self) -> ethers::contract::builders::Event<M, AddProductFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BalanceUpdate` event"]
        pub fn balance_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BalanceUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProductUpdate` event"]
        pub fn product_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProductUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `QuoteProductUpdate` event"]
        pub fn quote_product_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, QuoteProductUpdateFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, SpotEngineEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for SpotEngine<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "AddProduct", abi = "AddProduct(uint32)")]
    pub struct AddProductFilter {
        pub product_id: u32,
    }
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "BalanceUpdate", abi = "BalanceUpdate(uint32,bytes32)")]
    pub struct BalanceUpdateFilter {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[derive(
        Serialize,
        Deserialize,
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
        Serialize,
        Deserialize,
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
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ProductUpdate", abi = "ProductUpdate(uint32)")]
    pub struct ProductUpdateFilter {
        pub product_id: u32,
    }
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "QuoteProductUpdate", abi = "QuoteProductUpdate(uint32)")]
    pub struct QuoteProductUpdateFilter {
        pub iso_group: u32,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum SpotEngineEvents {
        AddProductFilter(AddProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProductUpdateFilter(ProductUpdateFilter),
        QuoteProductUpdateFilter(QuoteProductUpdateFilter),
    }
    impl ethers::contract::EthLogDecode for SpotEngineEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddProductFilter::decode_log(log) {
                return Ok(SpotEngineEvents::AddProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SpotEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SpotEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::ProductUpdateFilter(decoded));
            }
            if let Ok(decoded) = QuoteProductUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::QuoteProductUpdateFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for SpotEngineEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SpotEngineEvents::AddProductFilter(element) => element.fmt(f),
                SpotEngineEvents::BalanceUpdateFilter(element) => element.fmt(f),
                SpotEngineEvents::InitializedFilter(element) => element.fmt(f),
                SpotEngineEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                SpotEngineEvents::ProductUpdateFilter(element) => element.fmt(f),
                SpotEngineEvents::QuoteProductUpdateFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addProduct` function with signature `addProduct(uint32,address,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `[45, 36, 52, 84]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "addProduct",
        abi = "addProduct(uint32,address,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct AddProductCall {
        pub product_id: u32,
        pub book: ethers::core::types::Address,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub lp_spread_x18: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    #[doc = "Container type for all input parameters for the `assertUtilization` function with signature `assertUtilization(uint32)` and selector `[74, 200, 216, 193]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assertUtilization", abi = "assertUtilization(uint32)")]
    pub struct AssertUtilizationCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `[217, 135, 82, 236]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnLp", abi = "burnLp(uint32,bytes32,int128)")]
    pub struct BurnLpCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_lp: i128,
    }
    #[doc = "Container type for all input parameters for the `decomposeLps` function with signature `decomposeLps(uint32,bytes32,bytes32)` and selector `[177, 91, 130, 86]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decomposeLps", abi = "decomposeLps(uint32,bytes32,bytes32)")]
    pub struct DecomposeLpsCall {
        pub iso_group: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub liquidator: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `[124, 30, 20, 135]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBalance", abi = "getBalance(uint32,bytes32)")]
    pub struct GetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `[177, 203, 15, 66]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getClearinghouse", abi = "getClearinghouse()")]
    pub struct GetClearinghouseCall;
    #[doc = "Container type for all input parameters for the `getConfig` function with signature `getConfig(uint32)` and selector `[227, 67, 115, 140]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getConfig", abi = "getConfig(uint32)")]
    pub struct GetConfigCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `[138, 29, 67, 201]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCoreRisk", abi = "getCoreRisk(bytes32,uint32,uint8)")]
    pub struct GetCoreRiskCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub health_type: u8,
    }
    #[doc = "Container type for all input parameters for the `getEndpoint` function with signature `getEndpoint()` and selector `[174, 216, 233, 103]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `getEngineType` function with signature `getEngineType()` and selector `[70, 4, 209, 155]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEngineType", abi = "getEngineType()")]
    pub struct GetEngineTypeCall;
    #[doc = "Container type for all input parameters for the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `[135, 29, 9, 18]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getHealthContribution",
        abi = "getHealthContribution(bytes32,uint8)"
    )]
    pub struct GetHealthContributionCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    #[doc = "Container type for all input parameters for the `getPoolState` function with signature `getPoolState(uint32)` and selector `[138, 246, 66, 106]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPoolState", abi = "getPoolState(uint32)")]
    pub struct GetPoolStateCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getProductIds` function with signature `getProductIds()` and selector `[71, 66, 142, 123]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getProductIds", abi = "getProductIds()")]
    pub struct GetProductIdsCall;
    #[doc = "Container type for all input parameters for the `getProductIds` function with signature `getProductIds(uint32)` and selector `[244, 200, 197, 141]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getProductIds", abi = "getProductIds(uint32)")]
    pub struct GetProductIdsWithIsoGroupCall {
        pub iso_group: u32,
    }
    #[doc = "Container type for all input parameters for the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `[237, 240, 38, 83]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRawBalance", abi = "getRawBalance(uint32,bytes32)")]
    pub struct GetRawBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `[199, 33, 189, 101]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRawLpState", abi = "getRawLpState(uint32)")]
    pub struct GetRawLpStateCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getRawState` function with signature `getRawState(uint32)` and selector `[236, 122, 121, 201]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRawState", abi = "getRawState(uint32)")]
    pub struct GetRawStateCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getRisk` function with signature `getRisk(uint32)` and selector `[236, 217, 203, 168]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `[227, 52, 190, 51]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getStateAndBalance",
        abi = "getStateAndBalance(uint32,bytes32)"
    )]
    pub struct GetStateAndBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getStatesAndBalances` function with signature `getStatesAndBalances(uint32,bytes32)` and selector `[61, 92, 201, 220]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getStatesAndBalances",
        abi = "getStatesAndBalances(uint32,bytes32)"
    )]
    pub struct GetStatesAndBalancesCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getToken` function with signature `getToken(uint32)` and selector `[69, 190, 126, 214]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getToken", abi = "getToken(uint32)")]
    pub struct GetTokenCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `[166, 122, 195, 34]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTokenBalance", abi = "getTokenBalance(uint32)")]
    pub struct GetTokenBalanceCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getTotalBalances` function with signature `getTotalBalances()` and selector `[43, 175, 87, 211]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTotalBalances", abi = "getTotalBalances()")]
    pub struct GetTotalBalancesCall;
    #[doc = "Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `[13, 142, 110, 44]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `getWithdrawFee` function with signature `getWithdrawFee(uint32)` and selector `[253, 244, 160, 192]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getWithdrawFee", abi = "getWithdrawFee(uint32)")]
    pub struct GetWithdrawFeeCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address)` and selector `[20, 89, 69, 122]`"]
    #[derive(
        Serialize,
        Deserialize,
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
        abi = "initialize(address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ethers::core::types::Address,
        pub offchain_exchange: ethers::core::types::Address,
        pub quote: ethers::core::types::Address,
        pub endpoint: ethers::core::types::Address,
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `manualAssert` function with signature `manualAssert(int128[],int128[])` and selector `[48, 151, 43, 80]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "manualAssert", abi = "manualAssert(int128[],int128[])")]
    pub struct ManualAssertCall {
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub total_deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub total_borrows: ::std::vec::Vec<i128>,
    }
    #[doc = "Container type for all input parameters for the `mintLp` function with signature `mintLp(uint32,bytes32,int128,int128,int128)` and selector `[152, 222, 114, 254]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintLp", abi = "mintLp(uint32,bytes32,int128,int128,int128)")]
    pub struct MintLpCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_base: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote_amount_low: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote_amount_high: i128,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `setConfig` function with signature `setConfig(uint32,(address,int128,int128,int128,int128))` and selector `[16, 202, 147, 136]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setConfig",
        abi = "setConfig(uint32,(address,int128,int128,int128,int128))"
    )]
    pub struct SetConfigCall {
        pub product_id: u32,
        pub config: Config,
    }
    #[doc = "Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(address)` and selector `[219, 187, 65, 85]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `setLpBalance` function with signature `setLpBalance(uint32,bytes32,(int128))` and selector `[248, 102, 24, 132]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLpBalance", abi = "setLpBalance(uint32,bytes32,(int128))")]
    pub struct SetLpBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub lp_balance: LpBalance,
    }
    #[doc = "Container type for all input parameters for the `setLpState` function with signature `setLpState(uint32,(int128,(int128,int128),(int128,int128)))` and selector `[155, 185, 31, 106]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setLpState",
        abi = "setLpState(uint32,(int128,(int128,int128),(int128,int128)))"
    )]
    pub struct SetLpStateCall {
        pub product_id: u32,
        pub lp_state: LpState,
    }
    #[doc = "Container type for all input parameters for the `setState` function with signature `setState(uint32,(int128,int128,int128,int128))` and selector `[127, 162, 157, 73]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setState",
        abi = "setState(uint32,(int128,int128,int128,int128))"
    )]
    pub struct SetStateCall {
        pub product_id: u32,
        pub state: State,
    }
    #[doc = "Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32)` and selector `[137, 54, 247, 205]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "socializeSubaccount", abi = "socializeSubaccount(bytes32)")]
    pub struct SocializeSubaccountCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `[199, 22, 124, 245]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swapLp", abi = "swapLp(uint32,int128,int128)")]
    pub struct SwapLpCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub base_delta: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote_delta: i128,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Serialize,
        Deserialize,
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
    #[doc = "Container type for all input parameters for the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `[73, 247, 53, 104]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "unsignedUpdateProductTx",
        abi = "unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))"
    )]
    pub struct UnsignedUpdateProductTxCall {
        pub p: UpdateProductTx,
    }
    #[doc = "Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128)` and selector `[224, 176, 98, 31]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateBalance", abi = "updateBalance(uint32,bytes32,int128)")]
    pub struct UpdateBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_delta: i128,
    }
    #[doc = "Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128,int128)` and selector `[248, 164, 46, 81]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "updateBalance",
        abi = "updateBalance(uint32,bytes32,int128,int128)"
    )]
    pub struct UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_delta: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote_delta: i128,
    }
    #[doc = "Container type for all input parameters for the `updatePrice` function with signature `updatePrice(uint32,int128)` and selector `[21, 60, 166, 192]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updatePrice", abi = "updatePrice(uint32,int128)")]
    pub struct UpdatePriceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
    #[doc = "Container type for all input parameters for the `updateProduct` function with signature `updateProduct(bytes)` and selector `[201, 254, 154, 195]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateProduct", abi = "updateProduct(bytes)")]
    pub struct UpdateProductCall {
        pub raw_txn: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `[243, 158, 235, 16]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "updateQuoteFromInsurance",
        abi = "updateQuoteFromInsurance(bytes32,int128)"
    )]
    pub struct UpdateQuoteFromInsuranceCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub insurance: i128,
    }
    #[doc = "Container type for all input parameters for the `updateRisk` function with signature `updateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `[197, 86, 7, 181]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "updateRisk",
        abi = "updateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct UpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    #[doc = "Container type for all input parameters for the `updateStates` function with signature `updateStates(uint128,int128[])` and selector `[103, 54, 245, 218]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateStates", abi = "updateStates(uint128,int128[])")]
    pub struct UpdateStatesCall {
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub dt: u128,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub global_util_ratios_x18: ::std::vec::Vec<i128>,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum SpotEngineCalls {
        AddProduct(AddProductCall),
        AssertUtilization(AssertUtilizationCall),
        BurnLp(BurnLpCall),
        DecomposeLps(DecomposeLpsCall),
        GetBalance(GetBalanceCall),
        GetClearinghouse(GetClearinghouseCall),
        GetConfig(GetConfigCall),
        GetCoreRisk(GetCoreRiskCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetHealthContribution(GetHealthContributionCall),
        GetPoolState(GetPoolStateCall),
        GetProductIds(GetProductIdsCall),
        GetProductIdsWithIsoGroup(GetProductIdsWithIsoGroupCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawLpState(GetRawLpStateCall),
        GetRawState(GetRawStateCall),
        GetRisk(GetRiskCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        GetStatesAndBalances(GetStatesAndBalancesCall),
        GetToken(GetTokenCall),
        GetTokenBalance(GetTokenBalanceCall),
        GetTotalBalances(GetTotalBalancesCall),
        GetVersion(GetVersionCall),
        GetWithdrawFee(GetWithdrawFeeCall),
        Initialize(InitializeCall),
        ManualAssert(ManualAssertCall),
        MintLp(MintLpCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetConfig(SetConfigCall),
        SetEndpoint(SetEndpointCall),
        SetLpBalance(SetLpBalanceCall),
        SetLpState(SetLpStateCall),
        SetState(SetStateCall),
        SocializeSubaccount(SocializeSubaccountCall),
        SwapLp(SwapLpCall),
        TransferOwnership(TransferOwnershipCall),
        UnsignedUpdateProductTx(UnsignedUpdateProductTxCall),
        UpdateBalance(UpdateBalanceCall),
        UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(
            UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall,
        ),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
        UpdateQuoteFromInsurance(UpdateQuoteFromInsuranceCall),
        UpdateRisk(UpdateRiskCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ethers::core::abi::AbiDecode for SpotEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::AddProduct(decoded));
            }
            if let Ok(decoded) =
                <AssertUtilizationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::AssertUtilization(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::BurnLp(decoded));
            }
            if let Ok(decoded) =
                <DecomposeLpsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::DecomposeLps(decoded));
            }
            if let Ok(decoded) =
                <GetBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetClearinghouse(decoded));
            }
            if let Ok(decoded) =
                <GetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetConfig(decoded));
            }
            if let Ok(decoded) =
                <GetCoreRiskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetCoreRisk(decoded));
            }
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetEngineTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetEngineType(decoded));
            }
            if let Ok(decoded) =
                <GetHealthContributionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetHealthContribution(decoded));
            }
            if let Ok(decoded) =
                <GetPoolStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetPoolState(decoded));
            }
            if let Ok(decoded) =
                <GetProductIdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetProductIds(decoded));
            }
            if let Ok(decoded) =
                <GetProductIdsWithIsoGroupCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SpotEngineCalls::GetProductIdsWithIsoGroup(decoded));
            }
            if let Ok(decoded) =
                <GetRawBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetRawBalance(decoded));
            }
            if let Ok(decoded) =
                <GetRawLpStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetRawLpState(decoded));
            }
            if let Ok(decoded) =
                <GetRawStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetRawState(decoded));
            }
            if let Ok(decoded) =
                <GetRiskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetRisk(decoded));
            }
            if let Ok(decoded) =
                <GetStateAndBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetStateAndBalance(decoded));
            }
            if let Ok(decoded) =
                <GetStatesAndBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetStatesAndBalances(decoded));
            }
            if let Ok(decoded) =
                <GetTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetToken(decoded));
            }
            if let Ok(decoded) =
                <GetTokenBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetTokenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetTotalBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetTotalBalances(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <GetWithdrawFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::GetWithdrawFee(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <ManualAssertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::MintLp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SetConfig(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetLpBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SetLpBalance(decoded));
            }
            if let Ok(decoded) =
                <SetLpStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SetLpState(decoded));
            }
            if let Ok(decoded) =
                <SetStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SetState(decoded));
            }
            if let Ok(decoded) =
                <SocializeSubaccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) = <SwapLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::SwapLp(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnsignedUpdateProductTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UnsignedUpdateProductTx(decoded));
            }
            if let Ok(decoded) =
                <UpdateBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UpdateBalance(decoded));
            }
            if let Ok (decoded) = < UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SpotEngineCalls :: UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta (decoded)) }
            if let Ok(decoded) =
                <UpdatePriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UpdatePrice(decoded));
            }
            if let Ok(decoded) =
                <UpdateProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UpdateProduct(decoded));
            }
            if let Ok(decoded) =
                <UpdateQuoteFromInsuranceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SpotEngineCalls::UpdateQuoteFromInsurance(decoded));
            }
            if let Ok(decoded) =
                <UpdateRiskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UpdateRisk(decoded));
            }
            if let Ok(decoded) =
                <UpdateStatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SpotEngineCalls::UpdateStates(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SpotEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SpotEngineCalls::AddProduct(element) => element.encode(),
                SpotEngineCalls::AssertUtilization(element) => element.encode(),
                SpotEngineCalls::BurnLp(element) => element.encode(),
                SpotEngineCalls::DecomposeLps(element) => element.encode(),
                SpotEngineCalls::GetBalance(element) => element.encode(),
                SpotEngineCalls::GetClearinghouse(element) => element.encode(),
                SpotEngineCalls::GetConfig(element) => element.encode(),
                SpotEngineCalls::GetCoreRisk(element) => element.encode(),
                SpotEngineCalls::GetEndpoint(element) => element.encode(),
                SpotEngineCalls::GetEngineType(element) => element.encode(),
                SpotEngineCalls::GetHealthContribution(element) => element.encode(),
                SpotEngineCalls::GetPoolState(element) => element.encode(),
                SpotEngineCalls::GetProductIds(element) => element.encode(),
                SpotEngineCalls::GetProductIdsWithIsoGroup(element) => element.encode(),
                SpotEngineCalls::GetRawBalance(element) => element.encode(),
                SpotEngineCalls::GetRawLpState(element) => element.encode(),
                SpotEngineCalls::GetRawState(element) => element.encode(),
                SpotEngineCalls::GetRisk(element) => element.encode(),
                SpotEngineCalls::GetStateAndBalance(element) => element.encode(),
                SpotEngineCalls::GetStatesAndBalances(element) => element.encode(),
                SpotEngineCalls::GetToken(element) => element.encode(),
                SpotEngineCalls::GetTokenBalance(element) => element.encode(),
                SpotEngineCalls::GetTotalBalances(element) => element.encode(),
                SpotEngineCalls::GetVersion(element) => element.encode(),
                SpotEngineCalls::GetWithdrawFee(element) => element.encode(),
                SpotEngineCalls::Initialize(element) => element.encode(),
                SpotEngineCalls::ManualAssert(element) => element.encode(),
                SpotEngineCalls::MintLp(element) => element.encode(),
                SpotEngineCalls::Owner(element) => element.encode(),
                SpotEngineCalls::RenounceOwnership(element) => element.encode(),
                SpotEngineCalls::SetConfig(element) => element.encode(),
                SpotEngineCalls::SetEndpoint(element) => element.encode(),
                SpotEngineCalls::SetLpBalance(element) => element.encode(),
                SpotEngineCalls::SetLpState(element) => element.encode(),
                SpotEngineCalls::SetState(element) => element.encode(),
                SpotEngineCalls::SocializeSubaccount(element) => element.encode(),
                SpotEngineCalls::SwapLp(element) => element.encode(),
                SpotEngineCalls::TransferOwnership(element) => element.encode(),
                SpotEngineCalls::UnsignedUpdateProductTx(element) => element.encode(),
                SpotEngineCalls::UpdateBalance(element) => element.encode(),
                SpotEngineCalls::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    element.encode()
                }
                SpotEngineCalls::UpdatePrice(element) => element.encode(),
                SpotEngineCalls::UpdateProduct(element) => element.encode(),
                SpotEngineCalls::UpdateQuoteFromInsurance(element) => element.encode(),
                SpotEngineCalls::UpdateRisk(element) => element.encode(),
                SpotEngineCalls::UpdateStates(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SpotEngineCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SpotEngineCalls::AddProduct(element) => element.fmt(f),
                SpotEngineCalls::AssertUtilization(element) => element.fmt(f),
                SpotEngineCalls::BurnLp(element) => element.fmt(f),
                SpotEngineCalls::DecomposeLps(element) => element.fmt(f),
                SpotEngineCalls::GetBalance(element) => element.fmt(f),
                SpotEngineCalls::GetClearinghouse(element) => element.fmt(f),
                SpotEngineCalls::GetConfig(element) => element.fmt(f),
                SpotEngineCalls::GetCoreRisk(element) => element.fmt(f),
                SpotEngineCalls::GetEndpoint(element) => element.fmt(f),
                SpotEngineCalls::GetEngineType(element) => element.fmt(f),
                SpotEngineCalls::GetHealthContribution(element) => element.fmt(f),
                SpotEngineCalls::GetPoolState(element) => element.fmt(f),
                SpotEngineCalls::GetProductIds(element) => element.fmt(f),
                SpotEngineCalls::GetProductIdsWithIsoGroup(element) => element.fmt(f),
                SpotEngineCalls::GetRawBalance(element) => element.fmt(f),
                SpotEngineCalls::GetRawLpState(element) => element.fmt(f),
                SpotEngineCalls::GetRawState(element) => element.fmt(f),
                SpotEngineCalls::GetRisk(element) => element.fmt(f),
                SpotEngineCalls::GetStateAndBalance(element) => element.fmt(f),
                SpotEngineCalls::GetStatesAndBalances(element) => element.fmt(f),
                SpotEngineCalls::GetToken(element) => element.fmt(f),
                SpotEngineCalls::GetTokenBalance(element) => element.fmt(f),
                SpotEngineCalls::GetTotalBalances(element) => element.fmt(f),
                SpotEngineCalls::GetVersion(element) => element.fmt(f),
                SpotEngineCalls::GetWithdrawFee(element) => element.fmt(f),
                SpotEngineCalls::Initialize(element) => element.fmt(f),
                SpotEngineCalls::ManualAssert(element) => element.fmt(f),
                SpotEngineCalls::MintLp(element) => element.fmt(f),
                SpotEngineCalls::Owner(element) => element.fmt(f),
                SpotEngineCalls::RenounceOwnership(element) => element.fmt(f),
                SpotEngineCalls::SetConfig(element) => element.fmt(f),
                SpotEngineCalls::SetEndpoint(element) => element.fmt(f),
                SpotEngineCalls::SetLpBalance(element) => element.fmt(f),
                SpotEngineCalls::SetLpState(element) => element.fmt(f),
                SpotEngineCalls::SetState(element) => element.fmt(f),
                SpotEngineCalls::SocializeSubaccount(element) => element.fmt(f),
                SpotEngineCalls::SwapLp(element) => element.fmt(f),
                SpotEngineCalls::TransferOwnership(element) => element.fmt(f),
                SpotEngineCalls::UnsignedUpdateProductTx(element) => element.fmt(f),
                SpotEngineCalls::UpdateBalance(element) => element.fmt(f),
                SpotEngineCalls::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    element.fmt(f)
                }
                SpotEngineCalls::UpdatePrice(element) => element.fmt(f),
                SpotEngineCalls::UpdateProduct(element) => element.fmt(f),
                SpotEngineCalls::UpdateQuoteFromInsurance(element) => element.fmt(f),
                SpotEngineCalls::UpdateRisk(element) => element.fmt(f),
                SpotEngineCalls::UpdateStates(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddProductCall> for SpotEngineCalls {
        fn from(var: AddProductCall) -> Self {
            SpotEngineCalls::AddProduct(var)
        }
    }
    impl ::std::convert::From<AssertUtilizationCall> for SpotEngineCalls {
        fn from(var: AssertUtilizationCall) -> Self {
            SpotEngineCalls::AssertUtilization(var)
        }
    }
    impl ::std::convert::From<BurnLpCall> for SpotEngineCalls {
        fn from(var: BurnLpCall) -> Self {
            SpotEngineCalls::BurnLp(var)
        }
    }
    impl ::std::convert::From<DecomposeLpsCall> for SpotEngineCalls {
        fn from(var: DecomposeLpsCall) -> Self {
            SpotEngineCalls::DecomposeLps(var)
        }
    }
    impl ::std::convert::From<GetBalanceCall> for SpotEngineCalls {
        fn from(var: GetBalanceCall) -> Self {
            SpotEngineCalls::GetBalance(var)
        }
    }
    impl ::std::convert::From<GetClearinghouseCall> for SpotEngineCalls {
        fn from(var: GetClearinghouseCall) -> Self {
            SpotEngineCalls::GetClearinghouse(var)
        }
    }
    impl ::std::convert::From<GetConfigCall> for SpotEngineCalls {
        fn from(var: GetConfigCall) -> Self {
            SpotEngineCalls::GetConfig(var)
        }
    }
    impl ::std::convert::From<GetCoreRiskCall> for SpotEngineCalls {
        fn from(var: GetCoreRiskCall) -> Self {
            SpotEngineCalls::GetCoreRisk(var)
        }
    }
    impl ::std::convert::From<GetEndpointCall> for SpotEngineCalls {
        fn from(var: GetEndpointCall) -> Self {
            SpotEngineCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetEngineTypeCall> for SpotEngineCalls {
        fn from(var: GetEngineTypeCall) -> Self {
            SpotEngineCalls::GetEngineType(var)
        }
    }
    impl ::std::convert::From<GetHealthContributionCall> for SpotEngineCalls {
        fn from(var: GetHealthContributionCall) -> Self {
            SpotEngineCalls::GetHealthContribution(var)
        }
    }
    impl ::std::convert::From<GetPoolStateCall> for SpotEngineCalls {
        fn from(var: GetPoolStateCall) -> Self {
            SpotEngineCalls::GetPoolState(var)
        }
    }
    impl ::std::convert::From<GetProductIdsCall> for SpotEngineCalls {
        fn from(var: GetProductIdsCall) -> Self {
            SpotEngineCalls::GetProductIds(var)
        }
    }
    impl ::std::convert::From<GetProductIdsWithIsoGroupCall> for SpotEngineCalls {
        fn from(var: GetProductIdsWithIsoGroupCall) -> Self {
            SpotEngineCalls::GetProductIdsWithIsoGroup(var)
        }
    }
    impl ::std::convert::From<GetRawBalanceCall> for SpotEngineCalls {
        fn from(var: GetRawBalanceCall) -> Self {
            SpotEngineCalls::GetRawBalance(var)
        }
    }
    impl ::std::convert::From<GetRawLpStateCall> for SpotEngineCalls {
        fn from(var: GetRawLpStateCall) -> Self {
            SpotEngineCalls::GetRawLpState(var)
        }
    }
    impl ::std::convert::From<GetRawStateCall> for SpotEngineCalls {
        fn from(var: GetRawStateCall) -> Self {
            SpotEngineCalls::GetRawState(var)
        }
    }
    impl ::std::convert::From<GetRiskCall> for SpotEngineCalls {
        fn from(var: GetRiskCall) -> Self {
            SpotEngineCalls::GetRisk(var)
        }
    }
    impl ::std::convert::From<GetStateAndBalanceCall> for SpotEngineCalls {
        fn from(var: GetStateAndBalanceCall) -> Self {
            SpotEngineCalls::GetStateAndBalance(var)
        }
    }
    impl ::std::convert::From<GetStatesAndBalancesCall> for SpotEngineCalls {
        fn from(var: GetStatesAndBalancesCall) -> Self {
            SpotEngineCalls::GetStatesAndBalances(var)
        }
    }
    impl ::std::convert::From<GetTokenCall> for SpotEngineCalls {
        fn from(var: GetTokenCall) -> Self {
            SpotEngineCalls::GetToken(var)
        }
    }
    impl ::std::convert::From<GetTokenBalanceCall> for SpotEngineCalls {
        fn from(var: GetTokenBalanceCall) -> Self {
            SpotEngineCalls::GetTokenBalance(var)
        }
    }
    impl ::std::convert::From<GetTotalBalancesCall> for SpotEngineCalls {
        fn from(var: GetTotalBalancesCall) -> Self {
            SpotEngineCalls::GetTotalBalances(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for SpotEngineCalls {
        fn from(var: GetVersionCall) -> Self {
            SpotEngineCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<GetWithdrawFeeCall> for SpotEngineCalls {
        fn from(var: GetWithdrawFeeCall) -> Self {
            SpotEngineCalls::GetWithdrawFee(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for SpotEngineCalls {
        fn from(var: InitializeCall) -> Self {
            SpotEngineCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<ManualAssertCall> for SpotEngineCalls {
        fn from(var: ManualAssertCall) -> Self {
            SpotEngineCalls::ManualAssert(var)
        }
    }
    impl ::std::convert::From<MintLpCall> for SpotEngineCalls {
        fn from(var: MintLpCall) -> Self {
            SpotEngineCalls::MintLp(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for SpotEngineCalls {
        fn from(var: OwnerCall) -> Self {
            SpotEngineCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for SpotEngineCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            SpotEngineCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetConfigCall> for SpotEngineCalls {
        fn from(var: SetConfigCall) -> Self {
            SpotEngineCalls::SetConfig(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for SpotEngineCalls {
        fn from(var: SetEndpointCall) -> Self {
            SpotEngineCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetLpBalanceCall> for SpotEngineCalls {
        fn from(var: SetLpBalanceCall) -> Self {
            SpotEngineCalls::SetLpBalance(var)
        }
    }
    impl ::std::convert::From<SetLpStateCall> for SpotEngineCalls {
        fn from(var: SetLpStateCall) -> Self {
            SpotEngineCalls::SetLpState(var)
        }
    }
    impl ::std::convert::From<SetStateCall> for SpotEngineCalls {
        fn from(var: SetStateCall) -> Self {
            SpotEngineCalls::SetState(var)
        }
    }
    impl ::std::convert::From<SocializeSubaccountCall> for SpotEngineCalls {
        fn from(var: SocializeSubaccountCall) -> Self {
            SpotEngineCalls::SocializeSubaccount(var)
        }
    }
    impl ::std::convert::From<SwapLpCall> for SpotEngineCalls {
        fn from(var: SwapLpCall) -> Self {
            SpotEngineCalls::SwapLp(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for SpotEngineCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            SpotEngineCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnsignedUpdateProductTxCall> for SpotEngineCalls {
        fn from(var: UnsignedUpdateProductTxCall) -> Self {
            SpotEngineCalls::UnsignedUpdateProductTx(var)
        }
    }
    impl ::std::convert::From<UpdateBalanceCall> for SpotEngineCalls {
        fn from(var: UpdateBalanceCall) -> Self {
            SpotEngineCalls::UpdateBalance(var)
        }
    }
    impl ::std::convert::From<UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall>
        for SpotEngineCalls
    {
        fn from(var: UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall) -> Self {
            SpotEngineCalls::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(var)
        }
    }
    impl ::std::convert::From<UpdatePriceCall> for SpotEngineCalls {
        fn from(var: UpdatePriceCall) -> Self {
            SpotEngineCalls::UpdatePrice(var)
        }
    }
    impl ::std::convert::From<UpdateProductCall> for SpotEngineCalls {
        fn from(var: UpdateProductCall) -> Self {
            SpotEngineCalls::UpdateProduct(var)
        }
    }
    impl ::std::convert::From<UpdateQuoteFromInsuranceCall> for SpotEngineCalls {
        fn from(var: UpdateQuoteFromInsuranceCall) -> Self {
            SpotEngineCalls::UpdateQuoteFromInsurance(var)
        }
    }
    impl ::std::convert::From<UpdateRiskCall> for SpotEngineCalls {
        fn from(var: UpdateRiskCall) -> Self {
            SpotEngineCalls::UpdateRisk(var)
        }
    }
    impl ::std::convert::From<UpdateStatesCall> for SpotEngineCalls {
        fn from(var: UpdateStatesCall) -> Self {
            SpotEngineCalls::UpdateStates(var)
        }
    }
    #[doc = "Container type for all return fields from the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `[217, 135, 82, 236]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BurnLpReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_base: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_quote: i128,
    }
    #[doc = "Container type for all return fields from the `decomposeLps` function with signature `decomposeLps(uint32,bytes32,bytes32)` and selector `[177, 91, 130, 86]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecomposeLpsReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub liquidation_fees: i128,
    }
    #[doc = "Container type for all return fields from the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `[124, 30, 20, 135]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBalanceReturn(pub Balance);
    #[doc = "Container type for all return fields from the `getClearinghouse` function with signature `getClearinghouse()` and selector `[177, 203, 15, 66]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetClearinghouseReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getConfig` function with signature `getConfig(uint32)` and selector `[227, 67, 115, 140]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetConfigReturn(pub Config);
    #[doc = "Container type for all return fields from the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `[138, 29, 67, 201]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCoreRiskReturn(pub CoreRisk);
    #[doc = "Container type for all return fields from the `getEndpoint` function with signature `getEndpoint()` and selector `[174, 216, 233, 103]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEndpointReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getEngineType` function with signature `getEngineType()` and selector `[70, 4, 209, 155]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEngineTypeReturn(pub u8);
    #[doc = "Container type for all return fields from the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `[135, 29, 9, 18]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetHealthContributionReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub health: i128,
    }
    #[doc = "Container type for all return fields from the `getPoolState` function with signature `getPoolState(uint32)` and selector `[138, 246, 66, 106]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPoolStateReturn(pub i128, pub i128);
    #[doc = "Container type for all return fields from the `getProductIds` function with signature `getProductIds()` and selector `[71, 66, 142, 123]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetProductIdsReturn(pub ::std::vec::Vec<u32>);
    #[doc = "Container type for all return fields from the `getProductIds` function with signature `getProductIds(uint32)` and selector `[244, 200, 197, 141]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetProductIdsWithIsoGroupReturn(pub ::std::vec::Vec<u32>);
    #[doc = "Container type for all return fields from the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `[237, 240, 38, 83]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRawBalanceReturn(pub Balances);
    #[doc = "Container type for all return fields from the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `[199, 33, 189, 101]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRawLpStateReturn(pub LpState);
    #[doc = "Container type for all return fields from the `getRawState` function with signature `getRawState(uint32)` and selector `[236, 122, 121, 201]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRawStateReturn(pub State);
    #[doc = "Container type for all return fields from the `getRisk` function with signature `getRisk(uint32)` and selector `[236, 217, 203, 168]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRiskReturn(pub Risk);
    #[doc = "Container type for all return fields from the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `[227, 52, 190, 51]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetStateAndBalanceReturn(pub State, pub Balance);
    #[doc = "Container type for all return fields from the `getStatesAndBalances` function with signature `getStatesAndBalances(uint32,bytes32)` and selector `[61, 92, 201, 220]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetStatesAndBalancesReturn(pub LpState, pub LpBalance, pub State, pub Balance);
    #[doc = "Container type for all return fields from the `getToken` function with signature `getToken(uint32)` and selector `[69, 190, 126, 214]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `[166, 122, 195, 34]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenBalanceReturn {
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub balance: u128,
    }
    #[doc = "Container type for all return fields from the `getTotalBalances` function with signature `getTotalBalances()` and selector `[43, 175, 87, 211]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTotalBalancesReturn {
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub borrows: ::std::vec::Vec<i128>,
    }
    #[doc = "Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `[13, 142, 110, 44]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetVersionReturn(pub u64);
    #[doc = "Container type for all return fields from the `getWithdrawFee` function with signature `getWithdrawFee(uint32)` and selector `[253, 244, 160, 192]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetWithdrawFeeReturn(pub i128);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `[199, 22, 124, 245]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapLpReturn(pub i128, pub i128);
    #[doc = "Container type for all return fields from the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `[73, 247, 53, 104]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UnsignedUpdateProductTxReturn(pub UpdateProductTx);
    #[doc = "Container type for all return fields from the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `[243, 158, 235, 16]`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UpdateQuoteFromInsuranceReturn(pub i128);
    #[doc = "`CoreRisk(int128,int128,int128)`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CoreRisk {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub long_weight: i128,
    }
    #[doc = "`Balance(int128,int128)`"]
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    #[archive(check_bytes)]
    pub struct Balance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_multiplier_x18: i128,
    }
    #[doc = "`BalanceNormalized(int128)`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceNormalized {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount_normalized: i128,
    }
    #[doc = "`Balances((int128),(int128))`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Balances {
        pub balance: BalanceNormalized,
        pub lp_balance: LpBalance,
    }
    #[doc = "`Config(address,int128,int128,int128,int128)`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Config {
        pub token: ethers::core::types::Address,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_inflection_util_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_floor_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_small_cap_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_large_cap_x18: i128,
    }
    #[doc = "`LpBalance(int128)`"]
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    #[archive(check_bytes)]
    pub struct LpBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
    }
    #[doc = "`LpState(int128,(int128,int128),(int128,int128))`"]
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    #[archive(check_bytes)]
    pub struct LpState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub supply: i128,
        pub quote: Balance,
        pub base: Balance,
    }
    #[doc = "`State(int128,int128,int128,int128)`"]
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    #[archive(check_bytes)]
    pub struct State {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_deposits_multiplier_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_borrows_multiplier_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub total_deposits_normalized: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub total_borrows_normalized: i128,
    }
    #[doc = "`UpdateProductTx(uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))`"]
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UpdateProductTx {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub lp_spread_x18: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    #[doc = "`Risk(int128,int128,int128,int128,int128)`"]
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    #[archive(check_bytes)]
    pub struct Risk {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub long_weight_initial_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub short_weight_initial_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub long_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub short_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
    #[doc = "`RiskStore(int32,int32,int32,int32,int128)`"]
    #[derive(
        Serialize,
        Deserialize,
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
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
}
