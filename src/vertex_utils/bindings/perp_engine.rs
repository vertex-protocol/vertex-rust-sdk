pub use perp_engine::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod perp_engine {
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
    #[doc = "PerpEngine was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtNegativeInput\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddProduct\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BalanceUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProductUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"book\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IClearinghouseState.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"largePositionPenalty\",\"type\":\"int32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addProduct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IProductEngine.ProductDelta[]\",\"name\":\"deltas\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteDelta\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"applyDeltas\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balances\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountLp\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnLp\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"amountBase\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountQuote\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidator\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feeCalculator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decomposeLps\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"liquidationFees\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAvailableSettle\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalance\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalanceAmount\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBalances\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClearinghouse\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getEngineType\",\"outputs\":[{\"internalType\":\"enum IProductEngine.EngineType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLpState\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePriceX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePricesX18\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Prices\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"spotPriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"perpPriceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderbook\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPoolState\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPositionPnl\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProductIds\",\"outputs\":[{\"internalType\":\"uint32[]\",\"name\":\"\",\"type\":\"uint32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getRawBalance\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getRawLpBalance\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getRawLpState\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getRawState\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementState\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStateAndBalance\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStatesAndBalances\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasBalance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_quote\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_fees\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpBalances\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lpStates\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128[]\",\"name\":\"openInterests\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manualAssert\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"contract IOffchainBook\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amountBase\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteAmountLow\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteAmountHigh\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintLp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"perpPositionClosed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLpBalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLpState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"productIds\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settlePnl\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"insurance\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"socializeSubaccount\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"states\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapLp\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapLp\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"baseSwapped\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteSwapped\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IPerpEngine.UpdateProductTx\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IClearinghouseState.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"largePositionPenalty\",\"type\":\"int32\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedUpdateProductTx\",\"outputs\":[{\"internalType\":\"struct IPerpEngine.UpdateProductTx\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IClearinghouseState.RiskStore\",\"name\":\"riskStore\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int32\",\"name\":\"longWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightInitial\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"longWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"shortWeightMaintenance\",\"type\":\"int32\",\"components\":[]},{\"internalType\":\"int32\",\"name\":\"largePositionPenalty\",\"type\":\"int32\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"tx\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateProduct\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"dt\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"avgPriceDiffs\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateStates\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PERPENGINE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PerpEngine<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PerpEngine<M> {
        fn clone(&self) -> Self {
            PerpEngine(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PerpEngine<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PerpEngine<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PerpEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PerpEngine<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PERPENGINE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addProduct` (0x367bc290) function"]
        pub fn add_product(
            &self,
            health_group: u32,
            book: ethers::core::types::Address,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
            risk_store: RiskStore,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [54, 123, 194, 144],
                    (
                        health_group,
                        book,
                        size_increment,
                        price_increment_x18,
                        min_size,
                        lp_spread_x18,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `applyDeltas` (0x6c9a2ce3) function"]
        pub fn apply_deltas(
            &self,
            deltas: ::std::vec::Vec<ProductDelta>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 154, 44, 227], deltas)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balances` (0xbf4c8f5f) function"]
        pub fn balances(
            &self,
            p0: u32,
            p1: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128, i128)> {
            self.0
                .method_hash([191, 76, 143, 95], (p0, p1))
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
        #[doc = "Calls the contract's `decomposeLps` (0x011a447a) function"]
        pub fn decompose_lps(
            &self,
            liquidatee: [u8; 32],
            liquidator: [u8; 32],
            fee_calculator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([1, 26, 68, 122], (liquidatee, liquidator, fee_calculator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAvailableSettle` (0x3056f78f) function"]
        pub fn get_available_settle(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([48, 86, 247, 143], product_id)
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
        #[doc = "Calls the contract's `getBalanceAmount` (0xe039bfa9) function"]
        pub fn get_balance_amount(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([224, 57, 191, 169], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBalances` (0x6aca31a3) function"]
        pub fn get_balances(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (LpBalance, Balance)> {
            self.0
                .method_hash([106, 202, 49, 163], (product_id, subaccount))
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
        #[doc = "Calls the contract's `getLpState` (0x3f857d5a) function"]
        pub fn get_lp_state(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, LpState> {
            self.0
                .method_hash([63, 133, 125, 90], product_id)
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
        #[doc = "Calls the contract's `getPoolState` (0x8af6426a) function"]
        pub fn get_pool_state(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([138, 246, 66, 106], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionPnl` (0x1769225f) function"]
        pub fn get_position_pnl(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([23, 105, 34, 95], (product_id, subaccount))
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
        #[doc = "Calls the contract's `getRawBalance` (0xedf02653) function"]
        pub fn get_raw_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([237, 240, 38, 83], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRawLpBalance` (0x6f27be34) function"]
        pub fn get_raw_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, LpBalance> {
            self.0
                .method_hash([111, 39, 190, 52], (product_id, subaccount))
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
        #[doc = "Calls the contract's `getSettlementState` (0x388927b8) function"]
        pub fn get_settlement_state(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (i128, LpState, LpBalance, State, Balance)>
        {
            self.0
                .method_hash([56, 137, 39, 184], (product_id, subaccount))
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
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasBalance` (0xde7ebc91) function"]
        pub fn has_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([222, 126, 188, 145], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x1459457a) function"]
        pub fn initialize(
            &self,
            clearinghouse: ethers::core::types::Address,
            quote: ethers::core::types::Address,
            endpoint: ethers::core::types::Address,
            admin: ethers::core::types::Address,
            fees: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, quote, endpoint, admin, fees),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lpBalances` (0x08ed83c1) function"]
        pub fn lp_balances(
            &self,
            p0: u32,
            p1: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([8, 237, 131, 193], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lpStates` (0x042508e6) function"]
        pub fn lp_states(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128, i128, i128, i128)> {
            self.0
                .method_hash([4, 37, 8, 230], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manualAssert` (0x9b6f762b) function"]
        pub fn manual_assert(
            &self,
            open_interests: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 111, 118, 43], open_interests)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `markets` (0xece91e35) function"]
        pub fn markets(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([236, 233, 30, 53], p0)
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
        #[doc = "Calls the contract's `perpPositionClosed` (0x64c42cc2) function"]
        pub fn perp_position_closed(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 196, 44, 194], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBalance` (0x385de9c3) function"]
        pub fn set_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            balance: Balance,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 93, 233, 195], (product_id, subaccount, balance))
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
        #[doc = "Calls the contract's `setLpBalance` (0x3025586a) function"]
        pub fn set_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            lp_balance: LpBalance,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 37, 88, 106], (product_id, subaccount, lp_balance))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLpState` (0x26fa00ac) function"]
        pub fn set_lp_state(
            &self,
            product_id: u32,
            lp_state: LpState,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 250, 0, 172], (product_id, lp_state))
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
        #[doc = "Calls the contract's `settlePnl` (0xd6b0e0b5) function"]
        pub fn settle_pnl(
            &self,
            subaccount: [u8; 32],
            product_ids: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([214, 176, 224, 181], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `socializeSubaccount` (0xb1cd4b8f) function"]
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([177, 205, 75, 143], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `states` (0x7f17baad) function"]
        pub fn states(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128, i128, i128)> {
            self.0
                .method_hash([127, 23, 186, 173], p0)
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
        #[doc = "Calls the contract's `swapLp` (0xd952f2a3) function"]
        pub fn swap_lp_with_product_id_and_amount_and_price_x_18(
            &self,
            product_id: u32,
            amount: i128,
            price_x18: i128,
            size_increment: i128,
            lp_spread_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash(
                    [217, 82, 242, 163],
                    (product_id, amount, price_x18, size_increment, lp_spread_x18),
                )
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
        #[doc = "Calls the contract's `unsignedUpdateProductTx` (0x3ec8c7da) function"]
        pub fn unsigned_update_product_tx(
            &self,
            p: UpdateProductTx,
        ) -> ethers::contract::builders::ContractCall<M, UpdateProductTx> {
            self.0
                .method_hash([62, 200, 199, 218], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateProduct` (0xc9fe9ac3) function"]
        pub fn update_product(
            &self,
            tx: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 254, 154, 195], tx)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateStates` (0x6736f5da) function"]
        pub fn update_states(
            &self,
            dt: u128,
            avg_price_diffs: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 54, 245, 218], (dt, avg_price_diffs))
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
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PerpEngineEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PerpEngine<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtNegativeInput` with signature `PRBMathSD59x18__SqrtNegativeInput(int256)` and selector `[193, 25, 7, 254]`"]
    #[derive(
        Serialize,
        Deserialize,
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
        #[serde(
            serialize_with = "serialize_i256",
            deserialize_with = "deserialize_i256"
        )]
        pub x: I256,
    }
    #[doc = "Custom Error type `PRBMathSD59x18__SqrtOverflow` with signature `PRBMathSD59x18__SqrtOverflow(int256)` and selector `[44, 72, 44, 57]`"]
    #[derive(
        Serialize,
        Deserialize,
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
        #[serde(
            serialize_with = "serialize_i256",
            deserialize_with = "deserialize_i256"
        )]
        pub x: I256,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum PerpEngineErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
    }
    impl ethers::core::abi::AbiDecode for PerpEngineErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PerpEngineErrors::PRBMathSD59x18__SqrtNegativeInput(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PerpEngineErrors::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PerpEngineErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                PerpEngineErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.encode(),
                PerpEngineErrors::PRBMathSD59x18__SqrtOverflow(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PerpEngineErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PerpEngineErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.fmt(f),
                PerpEngineErrors::PRBMathSD59x18__SqrtOverflow(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtNegativeInput> for PerpEngineErrors {
        fn from(var: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            PerpEngineErrors::PRBMathSD59x18__SqrtNegativeInput(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtOverflow> for PerpEngineErrors {
        fn from(var: PRBMathSD59x18__SqrtOverflow) -> Self {
            PerpEngineErrors::PRBMathSD59x18__SqrtOverflow(var)
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
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum PerpEngineEvents {
        AddProductFilter(AddProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProductUpdateFilter(ProductUpdateFilter),
    }
    impl ethers::contract::EthLogDecode for PerpEngineEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddProductFilter::decode_log(log) {
                return Ok(PerpEngineEvents::AddProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PerpEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PerpEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::ProductUpdateFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PerpEngineEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PerpEngineEvents::AddProductFilter(element) => element.fmt(f),
                PerpEngineEvents::BalanceUpdateFilter(element) => element.fmt(f),
                PerpEngineEvents::InitializedFilter(element) => element.fmt(f),
                PerpEngineEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                PerpEngineEvents::ProductUpdateFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addProduct` function with signature `addProduct(uint32,address,int128,int128,int128,int128,(int32,int32,int32,int32,int32))` and selector `[54, 123, 194, 144]`"]
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
        abi = "addProduct(uint32,address,int128,int128,int128,int128,(int32,int32,int32,int32,int32))"
    )]
    pub struct AddProductCall {
        pub health_group: u32,
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
        pub price_increment_x18: i128,
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
        pub risk_store: RiskStore,
    }
    #[doc = "Container type for all input parameters for the `applyDeltas` function with signature `applyDeltas((uint32,bytes32,int128,int128)[])` and selector `[108, 154, 44, 227]`"]
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
        name = "applyDeltas",
        abi = "applyDeltas((uint32,bytes32,int128,int128)[])"
    )]
    pub struct ApplyDeltasCall {
        pub deltas: ::std::vec::Vec<ProductDelta>,
    }
    #[doc = "Container type for all input parameters for the `balances` function with signature `balances(uint32,bytes32)` and selector `[191, 76, 143, 95]`"]
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
    #[ethcall(name = "balances", abi = "balances(uint32,bytes32)")]
    pub struct BalancesCall(pub u32, pub [u8; 32]);
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
    #[doc = "Container type for all input parameters for the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32,address)` and selector `[1, 26, 68, 122]`"]
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
    #[ethcall(name = "decomposeLps", abi = "decomposeLps(bytes32,bytes32,address)")]
    pub struct DecomposeLpsCall {
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
        pub fee_calculator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `[48, 86, 247, 143]`"]
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
    #[ethcall(name = "getAvailableSettle", abi = "getAvailableSettle(uint32)")]
    pub struct GetAvailableSettleCall {
        pub product_id: u32,
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
    #[doc = "Container type for all input parameters for the `getBalanceAmount` function with signature `getBalanceAmount(uint32,bytes32)` and selector `[224, 57, 191, 169]`"]
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
    #[ethcall(name = "getBalanceAmount", abi = "getBalanceAmount(uint32,bytes32)")]
    pub struct GetBalanceAmountCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getBalances` function with signature `getBalances(uint32,bytes32)` and selector `[106, 202, 49, 163]`"]
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
    #[ethcall(name = "getBalances", abi = "getBalances(uint32,bytes32)")]
    pub struct GetBalancesCall {
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
    #[doc = "Container type for all input parameters for the `getLpState` function with signature `getLpState(uint32)` and selector `[63, 133, 125, 90]`"]
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
    #[ethcall(name = "getLpState", abi = "getLpState(uint32)")]
    pub struct GetLpStateCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `[47, 143, 31, 176]`"]
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
    #[ethcall(name = "getOraclePriceX18", abi = "getOraclePriceX18(uint32)")]
    pub struct GetOraclePriceX18Call {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `[211, 214, 96, 203]`"]
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
    #[ethcall(name = "getOraclePricesX18", abi = "getOraclePricesX18(uint32)")]
    pub struct GetOraclePricesX18Call {
        pub health_group: u32,
    }
    #[doc = "Container type for all input parameters for the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `[68, 39, 149, 45]`"]
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
    #[ethcall(name = "getOrderbook", abi = "getOrderbook(uint32)")]
    pub struct GetOrderbookCall {
        pub product_id: u32,
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
    #[doc = "Container type for all input parameters for the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `[23, 105, 34, 95]`"]
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
    #[ethcall(name = "getPositionPnl", abi = "getPositionPnl(uint32,bytes32)")]
    pub struct GetPositionPnlCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `getRawLpBalance` function with signature `getRawLpBalance(uint32,bytes32)` and selector `[111, 39, 190, 52]`"]
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
    #[ethcall(name = "getRawLpBalance", abi = "getRawLpBalance(uint32,bytes32)")]
    pub struct GetRawLpBalanceCall {
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
    #[doc = "Container type for all input parameters for the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `[56, 137, 39, 184]`"]
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
        name = "getSettlementState",
        abi = "getSettlementState(uint32,bytes32)"
    )]
    pub struct GetSettlementStateCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `hasBalance` function with signature `hasBalance(uint32,bytes32)` and selector `[222, 126, 188, 145]`"]
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
    #[ethcall(name = "hasBalance", abi = "hasBalance(uint32,bytes32)")]
    pub struct HasBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
        pub quote: ethers::core::types::Address,
        pub endpoint: ethers::core::types::Address,
        pub admin: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lpBalances` function with signature `lpBalances(uint32,bytes32)` and selector `[8, 237, 131, 193]`"]
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
    #[ethcall(name = "lpBalances", abi = "lpBalances(uint32,bytes32)")]
    pub struct LpBalancesCall(pub u32, pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `lpStates` function with signature `lpStates(uint32)` and selector `[4, 37, 8, 230]`"]
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
    #[ethcall(name = "lpStates", abi = "lpStates(uint32)")]
    pub struct LpStatesCall(pub u32);
    #[doc = "Container type for all input parameters for the `manualAssert` function with signature `manualAssert(int128[])` and selector `[155, 111, 118, 43]`"]
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
    #[ethcall(name = "manualAssert", abi = "manualAssert(int128[])")]
    pub struct ManualAssertCall {
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub open_interests: ::std::vec::Vec<i128>,
    }
    #[doc = "Container type for all input parameters for the `markets` function with signature `markets(uint32)` and selector `[236, 233, 30, 53]`"]
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
    #[ethcall(name = "markets", abi = "markets(uint32)")]
    pub struct MarketsCall(pub u32);
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
    #[doc = "Container type for all input parameters for the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `[100, 196, 44, 194]`"]
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
        name = "perpPositionClosed",
        abi = "perpPositionClosed(uint32,bytes32)"
    )]
    pub struct PerpPositionClosedCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
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
    #[doc = "Container type for all input parameters for the `setBalance` function with signature `setBalance(uint32,bytes32,(int128,int128,int128))` and selector `[56, 93, 233, 195]`"]
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
        name = "setBalance",
        abi = "setBalance(uint32,bytes32,(int128,int128,int128))"
    )]
    pub struct SetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub balance: Balance,
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
    #[doc = "Container type for all input parameters for the `setLpBalance` function with signature `setLpBalance(uint32,bytes32,(int128,int128))` and selector `[48, 37, 88, 106]`"]
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
        name = "setLpBalance",
        abi = "setLpBalance(uint32,bytes32,(int128,int128))"
    )]
    pub struct SetLpBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub lp_balance: LpBalance,
    }
    #[doc = "Container type for all input parameters for the `setLpState` function with signature `setLpState(uint32,(int128,int128,int128,int128,int128))` and selector `[38, 250, 0, 172]`"]
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
        abi = "setLpState(uint32,(int128,int128,int128,int128,int128))"
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
    #[doc = "Container type for all input parameters for the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `[214, 176, 224, 181]`"]
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
    #[ethcall(name = "settlePnl", abi = "settlePnl(bytes32,uint256)")]
    pub struct SettlePnlCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "serialize_u256",
            deserialize_with = "deserialize_u256"
        )]
        pub product_ids: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `[177, 205, 75, 143]`"]
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
        name = "socializeSubaccount",
        abi = "socializeSubaccount(bytes32,int128)"
    )]
    pub struct SocializeSubaccountCall {
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
    #[doc = "Container type for all input parameters for the `states` function with signature `states(uint32)` and selector `[127, 23, 186, 173]`"]
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
    #[ethcall(name = "states", abi = "states(uint32)")]
    pub struct StatesCall(pub u32);
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
    #[doc = "Container type for all input parameters for the `swapLp` function with signature `swapLp(uint32,int128,int128,int128,int128)` and selector `[217, 82, 242, 163]`"]
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
    #[ethcall(name = "swapLp", abi = "swapLp(uint32,int128,int128,int128,int128)")]
    pub struct SwapLpWithProductIdAndAmountAndPriceX18Call {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub lp_spread_x18: i128,
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
    #[doc = "Container type for all input parameters for the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))` and selector `[62, 200, 199, 218]`"]
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
        abi = "unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))"
    )]
    pub struct UnsignedUpdateProductTxCall {
        pub p: UpdateProductTx,
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
        pub tx: ethers::core::types::Bytes,
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
        pub avg_price_diffs: ::std::vec::Vec<i128>,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum PerpEngineCalls {
        AddProduct(AddProductCall),
        ApplyDeltas(ApplyDeltasCall),
        Balances(BalancesCall),
        BurnLp(BurnLpCall),
        DecomposeLps(DecomposeLpsCall),
        GetAvailableSettle(GetAvailableSettleCall),
        GetBalance(GetBalanceCall),
        GetBalanceAmount(GetBalanceAmountCall),
        GetBalances(GetBalancesCall),
        GetClearinghouse(GetClearinghouseCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetLpState(GetLpStateCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderbook(GetOrderbookCall),
        GetPoolState(GetPoolStateCall),
        GetPositionPnl(GetPositionPnlCall),
        GetProductIds(GetProductIdsCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawLpBalance(GetRawLpBalanceCall),
        GetRawLpState(GetRawLpStateCall),
        GetRawState(GetRawStateCall),
        GetSettlementState(GetSettlementStateCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        GetStatesAndBalances(GetStatesAndBalancesCall),
        GetVersion(GetVersionCall),
        HasBalance(HasBalanceCall),
        Initialize(InitializeCall),
        LpBalances(LpBalancesCall),
        LpStates(LpStatesCall),
        ManualAssert(ManualAssertCall),
        Markets(MarketsCall),
        MintLp(MintLpCall),
        Owner(OwnerCall),
        PerpPositionClosed(PerpPositionClosedCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBalance(SetBalanceCall),
        SetEndpoint(SetEndpointCall),
        SetLpBalance(SetLpBalanceCall),
        SetLpState(SetLpStateCall),
        SetState(SetStateCall),
        SettlePnl(SettlePnlCall),
        SocializeSubaccount(SocializeSubaccountCall),
        States(StatesCall),
        SwapLp(SwapLpCall),
        SwapLpWithProductIdAndAmountAndPriceX18(SwapLpWithProductIdAndAmountAndPriceX18Call),
        TransferOwnership(TransferOwnershipCall),
        UnsignedUpdateProductTx(UnsignedUpdateProductTxCall),
        UpdateProduct(UpdateProductCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ethers::core::abi::AbiDecode for PerpEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::AddProduct(decoded));
            }
            if let Ok(decoded) =
                <ApplyDeltasCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::ApplyDeltas(decoded));
            }
            if let Ok(decoded) =
                <BalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::Balances(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::BurnLp(decoded));
            }
            if let Ok(decoded) =
                <DecomposeLpsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::DecomposeLps(decoded));
            }
            if let Ok(decoded) =
                <GetAvailableSettleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetAvailableSettle(decoded));
            }
            if let Ok(decoded) =
                <GetBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetBalanceAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetBalanceAmount(decoded));
            }
            if let Ok(decoded) =
                <GetBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetBalances(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetClearinghouse(decoded));
            }
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetEngineTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetEngineType(decoded));
            }
            if let Ok(decoded) =
                <GetLpStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetLpState(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePricesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) =
                <GetOrderbookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetOrderbook(decoded));
            }
            if let Ok(decoded) =
                <GetPoolStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetPoolState(decoded));
            }
            if let Ok(decoded) =
                <GetPositionPnlCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetPositionPnl(decoded));
            }
            if let Ok(decoded) =
                <GetProductIdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetProductIds(decoded));
            }
            if let Ok(decoded) =
                <GetRawBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetRawBalance(decoded));
            }
            if let Ok(decoded) =
                <GetRawLpBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetRawLpBalance(decoded));
            }
            if let Ok(decoded) =
                <GetRawLpStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetRawLpState(decoded));
            }
            if let Ok(decoded) =
                <GetRawStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetRawState(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetSettlementState(decoded));
            }
            if let Ok(decoded) =
                <GetStateAndBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetStateAndBalance(decoded));
            }
            if let Ok(decoded) =
                <GetStatesAndBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetStatesAndBalances(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <HasBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::HasBalance(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LpBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::LpBalances(decoded));
            }
            if let Ok(decoded) =
                <LpStatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::LpStates(decoded));
            }
            if let Ok(decoded) =
                <ManualAssertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::ManualAssert(decoded));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::Markets(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::MintLp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerpPositionClosedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::PerpPositionClosed(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SetBalance(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetLpBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SetLpBalance(decoded));
            }
            if let Ok(decoded) =
                <SetLpStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SetLpState(decoded));
            }
            if let Ok(decoded) =
                <SetStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SetState(decoded));
            }
            if let Ok(decoded) =
                <SettlePnlCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SettlePnl(decoded));
            }
            if let Ok(decoded) =
                <SocializeSubaccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) = <StatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::States(decoded));
            }
            if let Ok(decoded) = <SwapLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::SwapLp(decoded));
            }
            if let Ok (decoded) = < SwapLpWithProductIdAndAmountAndPriceX18Call as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PerpEngineCalls :: SwapLpWithProductIdAndAmountAndPriceX18 (decoded)) }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnsignedUpdateProductTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::UnsignedUpdateProductTx(decoded));
            }
            if let Ok(decoded) =
                <UpdateProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::UpdateProduct(decoded));
            }
            if let Ok(decoded) =
                <UpdateStatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PerpEngineCalls::UpdateStates(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PerpEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PerpEngineCalls::AddProduct(element) => element.encode(),
                PerpEngineCalls::ApplyDeltas(element) => element.encode(),
                PerpEngineCalls::Balances(element) => element.encode(),
                PerpEngineCalls::BurnLp(element) => element.encode(),
                PerpEngineCalls::DecomposeLps(element) => element.encode(),
                PerpEngineCalls::GetAvailableSettle(element) => element.encode(),
                PerpEngineCalls::GetBalance(element) => element.encode(),
                PerpEngineCalls::GetBalanceAmount(element) => element.encode(),
                PerpEngineCalls::GetBalances(element) => element.encode(),
                PerpEngineCalls::GetClearinghouse(element) => element.encode(),
                PerpEngineCalls::GetEndpoint(element) => element.encode(),
                PerpEngineCalls::GetEngineType(element) => element.encode(),
                PerpEngineCalls::GetLpState(element) => element.encode(),
                PerpEngineCalls::GetOraclePriceX18(element) => element.encode(),
                PerpEngineCalls::GetOraclePricesX18(element) => element.encode(),
                PerpEngineCalls::GetOrderbook(element) => element.encode(),
                PerpEngineCalls::GetPoolState(element) => element.encode(),
                PerpEngineCalls::GetPositionPnl(element) => element.encode(),
                PerpEngineCalls::GetProductIds(element) => element.encode(),
                PerpEngineCalls::GetRawBalance(element) => element.encode(),
                PerpEngineCalls::GetRawLpBalance(element) => element.encode(),
                PerpEngineCalls::GetRawLpState(element) => element.encode(),
                PerpEngineCalls::GetRawState(element) => element.encode(),
                PerpEngineCalls::GetSettlementState(element) => element.encode(),
                PerpEngineCalls::GetStateAndBalance(element) => element.encode(),
                PerpEngineCalls::GetStatesAndBalances(element) => element.encode(),
                PerpEngineCalls::GetVersion(element) => element.encode(),
                PerpEngineCalls::HasBalance(element) => element.encode(),
                PerpEngineCalls::Initialize(element) => element.encode(),
                PerpEngineCalls::LpBalances(element) => element.encode(),
                PerpEngineCalls::LpStates(element) => element.encode(),
                PerpEngineCalls::ManualAssert(element) => element.encode(),
                PerpEngineCalls::Markets(element) => element.encode(),
                PerpEngineCalls::MintLp(element) => element.encode(),
                PerpEngineCalls::Owner(element) => element.encode(),
                PerpEngineCalls::PerpPositionClosed(element) => element.encode(),
                PerpEngineCalls::RenounceOwnership(element) => element.encode(),
                PerpEngineCalls::SetBalance(element) => element.encode(),
                PerpEngineCalls::SetEndpoint(element) => element.encode(),
                PerpEngineCalls::SetLpBalance(element) => element.encode(),
                PerpEngineCalls::SetLpState(element) => element.encode(),
                PerpEngineCalls::SetState(element) => element.encode(),
                PerpEngineCalls::SettlePnl(element) => element.encode(),
                PerpEngineCalls::SocializeSubaccount(element) => element.encode(),
                PerpEngineCalls::States(element) => element.encode(),
                PerpEngineCalls::SwapLp(element) => element.encode(),
                PerpEngineCalls::SwapLpWithProductIdAndAmountAndPriceX18(element) => {
                    element.encode()
                }
                PerpEngineCalls::TransferOwnership(element) => element.encode(),
                PerpEngineCalls::UnsignedUpdateProductTx(element) => element.encode(),
                PerpEngineCalls::UpdateProduct(element) => element.encode(),
                PerpEngineCalls::UpdateStates(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PerpEngineCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PerpEngineCalls::AddProduct(element) => element.fmt(f),
                PerpEngineCalls::ApplyDeltas(element) => element.fmt(f),
                PerpEngineCalls::Balances(element) => element.fmt(f),
                PerpEngineCalls::BurnLp(element) => element.fmt(f),
                PerpEngineCalls::DecomposeLps(element) => element.fmt(f),
                PerpEngineCalls::GetAvailableSettle(element) => element.fmt(f),
                PerpEngineCalls::GetBalance(element) => element.fmt(f),
                PerpEngineCalls::GetBalanceAmount(element) => element.fmt(f),
                PerpEngineCalls::GetBalances(element) => element.fmt(f),
                PerpEngineCalls::GetClearinghouse(element) => element.fmt(f),
                PerpEngineCalls::GetEndpoint(element) => element.fmt(f),
                PerpEngineCalls::GetEngineType(element) => element.fmt(f),
                PerpEngineCalls::GetLpState(element) => element.fmt(f),
                PerpEngineCalls::GetOraclePriceX18(element) => element.fmt(f),
                PerpEngineCalls::GetOraclePricesX18(element) => element.fmt(f),
                PerpEngineCalls::GetOrderbook(element) => element.fmt(f),
                PerpEngineCalls::GetPoolState(element) => element.fmt(f),
                PerpEngineCalls::GetPositionPnl(element) => element.fmt(f),
                PerpEngineCalls::GetProductIds(element) => element.fmt(f),
                PerpEngineCalls::GetRawBalance(element) => element.fmt(f),
                PerpEngineCalls::GetRawLpBalance(element) => element.fmt(f),
                PerpEngineCalls::GetRawLpState(element) => element.fmt(f),
                PerpEngineCalls::GetRawState(element) => element.fmt(f),
                PerpEngineCalls::GetSettlementState(element) => element.fmt(f),
                PerpEngineCalls::GetStateAndBalance(element) => element.fmt(f),
                PerpEngineCalls::GetStatesAndBalances(element) => element.fmt(f),
                PerpEngineCalls::GetVersion(element) => element.fmt(f),
                PerpEngineCalls::HasBalance(element) => element.fmt(f),
                PerpEngineCalls::Initialize(element) => element.fmt(f),
                PerpEngineCalls::LpBalances(element) => element.fmt(f),
                PerpEngineCalls::LpStates(element) => element.fmt(f),
                PerpEngineCalls::ManualAssert(element) => element.fmt(f),
                PerpEngineCalls::Markets(element) => element.fmt(f),
                PerpEngineCalls::MintLp(element) => element.fmt(f),
                PerpEngineCalls::Owner(element) => element.fmt(f),
                PerpEngineCalls::PerpPositionClosed(element) => element.fmt(f),
                PerpEngineCalls::RenounceOwnership(element) => element.fmt(f),
                PerpEngineCalls::SetBalance(element) => element.fmt(f),
                PerpEngineCalls::SetEndpoint(element) => element.fmt(f),
                PerpEngineCalls::SetLpBalance(element) => element.fmt(f),
                PerpEngineCalls::SetLpState(element) => element.fmt(f),
                PerpEngineCalls::SetState(element) => element.fmt(f),
                PerpEngineCalls::SettlePnl(element) => element.fmt(f),
                PerpEngineCalls::SocializeSubaccount(element) => element.fmt(f),
                PerpEngineCalls::States(element) => element.fmt(f),
                PerpEngineCalls::SwapLp(element) => element.fmt(f),
                PerpEngineCalls::SwapLpWithProductIdAndAmountAndPriceX18(element) => element.fmt(f),
                PerpEngineCalls::TransferOwnership(element) => element.fmt(f),
                PerpEngineCalls::UnsignedUpdateProductTx(element) => element.fmt(f),
                PerpEngineCalls::UpdateProduct(element) => element.fmt(f),
                PerpEngineCalls::UpdateStates(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddProductCall> for PerpEngineCalls {
        fn from(var: AddProductCall) -> Self {
            PerpEngineCalls::AddProduct(var)
        }
    }
    impl ::std::convert::From<ApplyDeltasCall> for PerpEngineCalls {
        fn from(var: ApplyDeltasCall) -> Self {
            PerpEngineCalls::ApplyDeltas(var)
        }
    }
    impl ::std::convert::From<BalancesCall> for PerpEngineCalls {
        fn from(var: BalancesCall) -> Self {
            PerpEngineCalls::Balances(var)
        }
    }
    impl ::std::convert::From<BurnLpCall> for PerpEngineCalls {
        fn from(var: BurnLpCall) -> Self {
            PerpEngineCalls::BurnLp(var)
        }
    }
    impl ::std::convert::From<DecomposeLpsCall> for PerpEngineCalls {
        fn from(var: DecomposeLpsCall) -> Self {
            PerpEngineCalls::DecomposeLps(var)
        }
    }
    impl ::std::convert::From<GetAvailableSettleCall> for PerpEngineCalls {
        fn from(var: GetAvailableSettleCall) -> Self {
            PerpEngineCalls::GetAvailableSettle(var)
        }
    }
    impl ::std::convert::From<GetBalanceCall> for PerpEngineCalls {
        fn from(var: GetBalanceCall) -> Self {
            PerpEngineCalls::GetBalance(var)
        }
    }
    impl ::std::convert::From<GetBalanceAmountCall> for PerpEngineCalls {
        fn from(var: GetBalanceAmountCall) -> Self {
            PerpEngineCalls::GetBalanceAmount(var)
        }
    }
    impl ::std::convert::From<GetBalancesCall> for PerpEngineCalls {
        fn from(var: GetBalancesCall) -> Self {
            PerpEngineCalls::GetBalances(var)
        }
    }
    impl ::std::convert::From<GetClearinghouseCall> for PerpEngineCalls {
        fn from(var: GetClearinghouseCall) -> Self {
            PerpEngineCalls::GetClearinghouse(var)
        }
    }
    impl ::std::convert::From<GetEndpointCall> for PerpEngineCalls {
        fn from(var: GetEndpointCall) -> Self {
            PerpEngineCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetEngineTypeCall> for PerpEngineCalls {
        fn from(var: GetEngineTypeCall) -> Self {
            PerpEngineCalls::GetEngineType(var)
        }
    }
    impl ::std::convert::From<GetLpStateCall> for PerpEngineCalls {
        fn from(var: GetLpStateCall) -> Self {
            PerpEngineCalls::GetLpState(var)
        }
    }
    impl ::std::convert::From<GetOraclePriceX18Call> for PerpEngineCalls {
        fn from(var: GetOraclePriceX18Call) -> Self {
            PerpEngineCalls::GetOraclePriceX18(var)
        }
    }
    impl ::std::convert::From<GetOraclePricesX18Call> for PerpEngineCalls {
        fn from(var: GetOraclePricesX18Call) -> Self {
            PerpEngineCalls::GetOraclePricesX18(var)
        }
    }
    impl ::std::convert::From<GetOrderbookCall> for PerpEngineCalls {
        fn from(var: GetOrderbookCall) -> Self {
            PerpEngineCalls::GetOrderbook(var)
        }
    }
    impl ::std::convert::From<GetPoolStateCall> for PerpEngineCalls {
        fn from(var: GetPoolStateCall) -> Self {
            PerpEngineCalls::GetPoolState(var)
        }
    }
    impl ::std::convert::From<GetPositionPnlCall> for PerpEngineCalls {
        fn from(var: GetPositionPnlCall) -> Self {
            PerpEngineCalls::GetPositionPnl(var)
        }
    }
    impl ::std::convert::From<GetProductIdsCall> for PerpEngineCalls {
        fn from(var: GetProductIdsCall) -> Self {
            PerpEngineCalls::GetProductIds(var)
        }
    }
    impl ::std::convert::From<GetRawBalanceCall> for PerpEngineCalls {
        fn from(var: GetRawBalanceCall) -> Self {
            PerpEngineCalls::GetRawBalance(var)
        }
    }
    impl ::std::convert::From<GetRawLpBalanceCall> for PerpEngineCalls {
        fn from(var: GetRawLpBalanceCall) -> Self {
            PerpEngineCalls::GetRawLpBalance(var)
        }
    }
    impl ::std::convert::From<GetRawLpStateCall> for PerpEngineCalls {
        fn from(var: GetRawLpStateCall) -> Self {
            PerpEngineCalls::GetRawLpState(var)
        }
    }
    impl ::std::convert::From<GetRawStateCall> for PerpEngineCalls {
        fn from(var: GetRawStateCall) -> Self {
            PerpEngineCalls::GetRawState(var)
        }
    }
    impl ::std::convert::From<GetSettlementStateCall> for PerpEngineCalls {
        fn from(var: GetSettlementStateCall) -> Self {
            PerpEngineCalls::GetSettlementState(var)
        }
    }
    impl ::std::convert::From<GetStateAndBalanceCall> for PerpEngineCalls {
        fn from(var: GetStateAndBalanceCall) -> Self {
            PerpEngineCalls::GetStateAndBalance(var)
        }
    }
    impl ::std::convert::From<GetStatesAndBalancesCall> for PerpEngineCalls {
        fn from(var: GetStatesAndBalancesCall) -> Self {
            PerpEngineCalls::GetStatesAndBalances(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for PerpEngineCalls {
        fn from(var: GetVersionCall) -> Self {
            PerpEngineCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<HasBalanceCall> for PerpEngineCalls {
        fn from(var: HasBalanceCall) -> Self {
            PerpEngineCalls::HasBalance(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PerpEngineCalls {
        fn from(var: InitializeCall) -> Self {
            PerpEngineCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LpBalancesCall> for PerpEngineCalls {
        fn from(var: LpBalancesCall) -> Self {
            PerpEngineCalls::LpBalances(var)
        }
    }
    impl ::std::convert::From<LpStatesCall> for PerpEngineCalls {
        fn from(var: LpStatesCall) -> Self {
            PerpEngineCalls::LpStates(var)
        }
    }
    impl ::std::convert::From<ManualAssertCall> for PerpEngineCalls {
        fn from(var: ManualAssertCall) -> Self {
            PerpEngineCalls::ManualAssert(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for PerpEngineCalls {
        fn from(var: MarketsCall) -> Self {
            PerpEngineCalls::Markets(var)
        }
    }
    impl ::std::convert::From<MintLpCall> for PerpEngineCalls {
        fn from(var: MintLpCall) -> Self {
            PerpEngineCalls::MintLp(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PerpEngineCalls {
        fn from(var: OwnerCall) -> Self {
            PerpEngineCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PerpPositionClosedCall> for PerpEngineCalls {
        fn from(var: PerpPositionClosedCall) -> Self {
            PerpEngineCalls::PerpPositionClosed(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PerpEngineCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PerpEngineCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetBalanceCall> for PerpEngineCalls {
        fn from(var: SetBalanceCall) -> Self {
            PerpEngineCalls::SetBalance(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for PerpEngineCalls {
        fn from(var: SetEndpointCall) -> Self {
            PerpEngineCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetLpBalanceCall> for PerpEngineCalls {
        fn from(var: SetLpBalanceCall) -> Self {
            PerpEngineCalls::SetLpBalance(var)
        }
    }
    impl ::std::convert::From<SetLpStateCall> for PerpEngineCalls {
        fn from(var: SetLpStateCall) -> Self {
            PerpEngineCalls::SetLpState(var)
        }
    }
    impl ::std::convert::From<SetStateCall> for PerpEngineCalls {
        fn from(var: SetStateCall) -> Self {
            PerpEngineCalls::SetState(var)
        }
    }
    impl ::std::convert::From<SettlePnlCall> for PerpEngineCalls {
        fn from(var: SettlePnlCall) -> Self {
            PerpEngineCalls::SettlePnl(var)
        }
    }
    impl ::std::convert::From<SocializeSubaccountCall> for PerpEngineCalls {
        fn from(var: SocializeSubaccountCall) -> Self {
            PerpEngineCalls::SocializeSubaccount(var)
        }
    }
    impl ::std::convert::From<StatesCall> for PerpEngineCalls {
        fn from(var: StatesCall) -> Self {
            PerpEngineCalls::States(var)
        }
    }
    impl ::std::convert::From<SwapLpCall> for PerpEngineCalls {
        fn from(var: SwapLpCall) -> Self {
            PerpEngineCalls::SwapLp(var)
        }
    }
    impl ::std::convert::From<SwapLpWithProductIdAndAmountAndPriceX18Call> for PerpEngineCalls {
        fn from(var: SwapLpWithProductIdAndAmountAndPriceX18Call) -> Self {
            PerpEngineCalls::SwapLpWithProductIdAndAmountAndPriceX18(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PerpEngineCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PerpEngineCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnsignedUpdateProductTxCall> for PerpEngineCalls {
        fn from(var: UnsignedUpdateProductTxCall) -> Self {
            PerpEngineCalls::UnsignedUpdateProductTx(var)
        }
    }
    impl ::std::convert::From<UpdateProductCall> for PerpEngineCalls {
        fn from(var: UpdateProductCall) -> Self {
            PerpEngineCalls::UpdateProduct(var)
        }
    }
    impl ::std::convert::From<UpdateStatesCall> for PerpEngineCalls {
        fn from(var: UpdateStatesCall) -> Self {
            PerpEngineCalls::UpdateStates(var)
        }
    }
    #[doc = "Container type for all return fields from the `balances` function with signature `balances(uint32,bytes32)` and selector `[191, 76, 143, 95]`"]
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
    pub struct BalancesReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
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
    #[doc = "Container type for all return fields from the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32,address)` and selector `[1, 26, 68, 122]`"]
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
    #[doc = "Container type for all return fields from the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `[48, 86, 247, 143]`"]
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
    pub struct GetAvailableSettleReturn(pub i128);
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
    #[doc = "Container type for all return fields from the `getBalanceAmount` function with signature `getBalanceAmount(uint32,bytes32)` and selector `[224, 57, 191, 169]`"]
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
    pub struct GetBalanceAmountReturn(pub i128);
    #[doc = "Container type for all return fields from the `getBalances` function with signature `getBalances(uint32,bytes32)` and selector `[106, 202, 49, 163]`"]
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
    pub struct GetBalancesReturn(pub LpBalance, pub Balance);
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
    #[doc = "Container type for all return fields from the `getLpState` function with signature `getLpState(uint32)` and selector `[63, 133, 125, 90]`"]
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
    pub struct GetLpStateReturn(pub LpState);
    #[doc = "Container type for all return fields from the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `[47, 143, 31, 176]`"]
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
    pub struct GetOraclePriceX18Return(pub i128);
    #[doc = "Container type for all return fields from the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `[211, 214, 96, 203]`"]
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
    pub struct GetOraclePricesX18Return(pub Prices);
    #[doc = "Container type for all return fields from the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `[68, 39, 149, 45]`"]
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
    pub struct GetOrderbookReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `[23, 105, 34, 95]`"]
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
    pub struct GetPositionPnlReturn(pub i128);
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
    pub struct GetRawBalanceReturn(pub Balance);
    #[doc = "Container type for all return fields from the `getRawLpBalance` function with signature `getRawLpBalance(uint32,bytes32)` and selector `[111, 39, 190, 52]`"]
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
    pub struct GetRawLpBalanceReturn(pub LpBalance);
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
    #[doc = "Container type for all return fields from the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `[56, 137, 39, 184]`"]
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
    pub struct GetSettlementStateReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub available_settle: i128,
        pub lp_state: LpState,
        pub lp_balance: LpBalance,
        pub state: State,
        pub balance: Balance,
    }
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
    #[doc = "Container type for all return fields from the `hasBalance` function with signature `hasBalance(uint32,bytes32)` and selector `[222, 126, 188, 145]`"]
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
    pub struct HasBalanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `lpBalances` function with signature `lpBalances(uint32,bytes32)` and selector `[8, 237, 131, 193]`"]
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
    pub struct LpBalancesReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    #[doc = "Container type for all return fields from the `lpStates` function with signature `lpStates(uint32)` and selector `[4, 37, 8, 230]`"]
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
    pub struct LpStatesReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub supply: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_per_lp_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub base: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote: i128,
    }
    #[doc = "Container type for all return fields from the `markets` function with signature `markets(uint32)` and selector `[236, 233, 30, 53]`"]
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
    pub struct MarketsReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `[100, 196, 44, 194]`"]
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
    pub struct PerpPositionClosedReturn(pub bool);
    #[doc = "Container type for all return fields from the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `[214, 176, 224, 181]`"]
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
    pub struct SettlePnlReturn(pub i128);
    #[doc = "Container type for all return fields from the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `[177, 205, 75, 143]`"]
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
    pub struct SocializeSubaccountReturn(pub i128);
    #[doc = "Container type for all return fields from the `states` function with signature `states(uint32)` and selector `[127, 23, 186, 173]`"]
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
    pub struct StatesReturn {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub open_interest: i128,
    }
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
    #[doc = "Container type for all return fields from the `swapLp` function with signature `swapLp(uint32,int128,int128,int128,int128)` and selector `[217, 82, 242, 163]`"]
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
    pub struct SwapLpWithProductIdAndAmountAndPriceX18Return {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub base_swapped: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote_swapped: i128,
    }
    #[doc = "Container type for all return fields from the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))` and selector `[62, 200, 199, 218]`"]
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
    #[doc = "`RiskStore(int32,int32,int32,int32,int32)`"]
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
        pub large_position_penalty: i32,
    }
    #[doc = "`Prices(int128,int128)`"]
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
    pub struct Prices {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub spot_price_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub perp_price_x18: i128,
    }
    #[doc = "`Balance(int128,int128,int128)`"]
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
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    #[doc = "`LpBalance(int128,int128)`"]
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
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    #[doc = "`LpState(int128,int128,int128,int128,int128)`"]
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
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_per_lp_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub base: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote: i128,
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
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub open_interest: i128,
    }
    #[doc = "`UpdateProductTx(uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32))`"]
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
        pub price_increment_x18: i128,
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
        pub risk_store: RiskStore,
    }
    #[doc = "`ProductDelta(uint32,bytes32,int128,int128)`"]
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
    pub struct ProductDelta {
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
        pub v_quote_delta: i128,
    }
}
