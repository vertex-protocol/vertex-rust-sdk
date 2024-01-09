pub use querier::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod querier {
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
    #[doc = "Querier was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtNegativeInput\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtOverflow\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllBooks\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllProducts\",\"outputs\":[{\"internalType\":\"struct FQuerier.ProductInfo\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct FQuerier.SpotProduct[]\",\"name\":\"spotProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.PerpProduct[]\",\"name\":\"perpProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IProductEngine\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBookInfo\",\"outputs\":[{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClearinghouse\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_fees\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_clearinghouseLiq\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_spotEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_perpEngine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_querier\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getContractVersions\",\"outputs\":[{\"internalType\":\"struct Versions\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"endpoint\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"feeCalculator\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"clearinghouse\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"clearinghouseLiq\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"spotEngine\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"perpEngine\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"querier\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64[]\",\"name\":\"books\",\"type\":\"uint64[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPerpBalance\",\"outputs\":[{\"internalType\":\"struct FQuerier.PerpBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPerpBalances\",\"outputs\":[{\"internalType\":\"struct FQuerier.PerpBalance[]\",\"name\":\"perpBalances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPerpProduct\",\"outputs\":[{\"internalType\":\"struct FQuerier.PerpProduct\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPerpProducts\",\"outputs\":[{\"internalType\":\"struct FQuerier.PerpProduct[]\",\"name\":\"perpProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSpotBalance\",\"outputs\":[{\"internalType\":\"struct FQuerier.SpotBalance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSpotBalances\",\"outputs\":[{\"internalType\":\"struct FQuerier.SpotBalance[]\",\"name\":\"spotBalances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSpotProduct\",\"outputs\":[{\"internalType\":\"struct FQuerier.SpotProduct\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSpotProducts\",\"outputs\":[{\"internalType\":\"struct FQuerier.SpotProduct[]\",\"name\":\"spotProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubaccountInfo\",\"outputs\":[{\"internalType\":\"struct FQuerier.SubaccountInfo\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"exists\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct FQuerier.HealthInfo[]\",\"name\":\"healths\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"int128\",\"name\":\"assets\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"liabilities\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"health\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"int128[][]\",\"name\":\"healthContributions\",\"type\":\"int128[][]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"spotCount\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"perpCount\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct FQuerier.SpotBalance[]\",\"name\":\"spotBalances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.PerpBalance[]\",\"name\":\"perpBalances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IPerpEngine.LpBalance\",\"name\":\"lpBalance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.Balance\",\"name\":\"balance\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"vQuoteBalance\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.SpotProduct[]\",\"name\":\"spotProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Config\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestInflectionUtilX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestFloorX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestSmallCapX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"interestLargeCapX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeDepositsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeBorrowsMultiplierX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalDepositsNormalized\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"totalBorrowsNormalized\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"quote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct ISpotEngine.Balance\",\"name\":\"base\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeMultiplierX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"internalType\":\"struct FQuerier.PerpProduct[]\",\"name\":\"perpProducts\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"oraclePriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"risk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.State\",\"name\":\"state\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"cumulativeFundingLongX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingShortX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"availableSettle\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"openInterest\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct IPerpEngine.LpState\",\"name\":\"lpState\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"supply\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lastCumulativeFundingX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"cumulativeFundingPerLpX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"base\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quote\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"struct FQuerier.BookInfo\",\"name\":\"bookInfo\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static QUERIER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Querier<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Querier<M> {
        fn clone(&self) -> Self {
            Querier(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Querier<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Querier<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Querier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Querier<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), QUERIER_ABI.clone(), client).into()
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
        #[doc = "Calls the contract's `getAllProducts` (0x02ee3a52) function"]
        pub fn get_all_products(&self) -> ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([2, 238, 58, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBookInfo` (0x6124e5ff) function"]
        pub fn get_book_info(
            &self,
            product_id: u32,
            engine: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, BookInfo> {
            self.0
                .method_hash([97, 36, 229, 255], (product_id, engine))
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
        #[doc = "Calls the contract's `getContractVersions` (0xe4717e72) function"]
        pub fn get_contract_versions(
            &self,
            endpoint: ethers::core::types::Address,
            fees: ethers::core::types::Address,
            clearinghouse: ethers::core::types::Address,
            clearinghouse_liq: ethers::core::types::Address,
            spot_engine: ethers::core::types::Address,
            perp_engine: ethers::core::types::Address,
            querier: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, Versions> {
            self.0
                .method_hash(
                    [228, 113, 126, 114],
                    (
                        endpoint,
                        fees,
                        clearinghouse,
                        clearinghouse_liq,
                        spot_engine,
                        perp_engine,
                        querier,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPerpBalance` (0xd7b229b6) function"]
        pub fn get_perp_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, PerpBalance> {
            self.0
                .method_hash([215, 178, 41, 182], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPerpBalances` (0x2593eb5f) function"]
        pub fn get_perp_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpBalance>> {
            self.0
                .method_hash([37, 147, 235, 95], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPerpProduct` (0x1ae10bc5) function"]
        pub fn get_perp_product(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, PerpProduct> {
            self.0
                .method_hash([26, 225, 11, 197], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPerpProducts` (0xee9928c9) function"]
        pub fn get_perp_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpProduct>> {
            self.0
                .method_hash([238, 153, 40, 201], product_ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSpotBalance` (0x74173404) function"]
        pub fn get_spot_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, SpotBalance> {
            self.0
                .method_hash([116, 23, 52, 4], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSpotBalances` (0x31546d51) function"]
        pub fn get_spot_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotBalance>> {
            self.0
                .method_hash([49, 84, 109, 81], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSpotProduct` (0x5723653f) function"]
        pub fn get_spot_product(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, SpotProduct> {
            self.0
                .method_hash([87, 35, 101, 63], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSpotProducts` (0x75a5ab3c) function"]
        pub fn get_spot_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotProduct>> {
            self.0
                .method_hash([117, 165, 171, 60], product_ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSubaccountInfo` (0x5d702e1a) function"]
        pub fn get_subaccount_info(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, SubaccountInfo> {
            self.0
                .method_hash([93, 112, 46, 26], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            clearinghouse: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], clearinghouse)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Querier<M> {
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
    pub enum QuerierErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
    }
    impl ethers::core::abi::AbiDecode for QuerierErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(QuerierErrors::PRBMathSD59x18__SqrtNegativeInput(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(QuerierErrors::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for QuerierErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                QuerierErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.encode(),
                QuerierErrors::PRBMathSD59x18__SqrtOverflow(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for QuerierErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                QuerierErrors::PRBMathSD59x18__SqrtNegativeInput(element) => element.fmt(f),
                QuerierErrors::PRBMathSD59x18__SqrtOverflow(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtNegativeInput> for QuerierErrors {
        fn from(var: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            QuerierErrors::PRBMathSD59x18__SqrtNegativeInput(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtOverflow> for QuerierErrors {
        fn from(var: PRBMathSD59x18__SqrtOverflow) -> Self {
            QuerierErrors::PRBMathSD59x18__SqrtOverflow(var)
        }
    }
    #[doc = "Container type for all input parameters for the `getAllBooks` function with signature `getAllBooks()` and selector `[53, 69, 40, 232]`"]
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
    #[ethcall(name = "getAllBooks", abi = "getAllBooks()")]
    pub struct GetAllBooksCall;
    #[doc = "Container type for all input parameters for the `getAllProducts` function with signature `getAllProducts()` and selector `[2, 238, 58, 82]`"]
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
    #[ethcall(name = "getAllProducts", abi = "getAllProducts()")]
    pub struct GetAllProductsCall;
    #[doc = "Container type for all input parameters for the `getBookInfo` function with signature `getBookInfo(uint32,address)` and selector `[97, 36, 229, 255]`"]
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
    #[ethcall(name = "getBookInfo", abi = "getBookInfo(uint32,address)")]
    pub struct GetBookInfoCall {
        pub product_id: u32,
        pub engine: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `getContractVersions` function with signature `getContractVersions(address,address,address,address,address,address,address)` and selector `[228, 113, 126, 114]`"]
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
        name = "getContractVersions",
        abi = "getContractVersions(address,address,address,address,address,address,address)"
    )]
    pub struct GetContractVersionsCall {
        pub endpoint: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
        pub clearinghouse: ethers::core::types::Address,
        pub clearinghouse_liq: ethers::core::types::Address,
        pub spot_engine: ethers::core::types::Address,
        pub perp_engine: ethers::core::types::Address,
        pub querier: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `[215, 178, 41, 182]`"]
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
    #[ethcall(name = "getPerpBalance", abi = "getPerpBalance(bytes32,uint32)")]
    pub struct GetPerpBalanceCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `[37, 147, 235, 95]`"]
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
    #[ethcall(name = "getPerpBalances", abi = "getPerpBalances(bytes32,uint32[])")]
    pub struct GetPerpBalancesCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `[26, 225, 11, 197]`"]
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
    #[ethcall(name = "getPerpProduct", abi = "getPerpProduct(uint32)")]
    pub struct GetPerpProductCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `[238, 153, 40, 201]`"]
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
    #[ethcall(name = "getPerpProducts", abi = "getPerpProducts(uint32[])")]
    pub struct GetPerpProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `[116, 23, 52, 4]`"]
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
    #[ethcall(name = "getSpotBalance", abi = "getSpotBalance(bytes32,uint32)")]
    pub struct GetSpotBalanceCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `[49, 84, 109, 81]`"]
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
    #[ethcall(name = "getSpotBalances", abi = "getSpotBalances(bytes32,uint32[])")]
    pub struct GetSpotBalancesCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `[87, 35, 101, 63]`"]
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
    #[ethcall(name = "getSpotProduct", abi = "getSpotProduct(uint32)")]
    pub struct GetSpotProductCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `[117, 165, 171, 60]`"]
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
    #[ethcall(name = "getSpotProducts", abi = "getSpotProducts(uint32[])")]
    pub struct GetSpotProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `[93, 112, 46, 26]`"]
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
    #[ethcall(name = "getSubaccountInfo", abi = "getSubaccountInfo(bytes32)")]
    pub struct GetSubaccountInfoCall {
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `[196, 214, 109, 232]`"]
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub clearinghouse: ethers::core::types::Address,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum QuerierCalls {
        GetAllBooks(GetAllBooksCall),
        GetAllProducts(GetAllProductsCall),
        GetBookInfo(GetBookInfoCall),
        GetClearinghouse(GetClearinghouseCall),
        GetContractVersions(GetContractVersionsCall),
        GetPerpBalance(GetPerpBalanceCall),
        GetPerpBalances(GetPerpBalancesCall),
        GetPerpProduct(GetPerpProductCall),
        GetPerpProducts(GetPerpProductsCall),
        GetSpotBalance(GetSpotBalanceCall),
        GetSpotBalances(GetSpotBalancesCall),
        GetSpotProduct(GetSpotProductCall),
        GetSpotProducts(GetSpotProductsCall),
        GetSubaccountInfo(GetSubaccountInfoCall),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
    }
    impl ethers::core::abi::AbiDecode for QuerierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAllBooksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetAllBooks(decoded));
            }
            if let Ok(decoded) =
                <GetAllProductsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetAllProducts(decoded));
            }
            if let Ok(decoded) =
                <GetBookInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetBookInfo(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetClearinghouse(decoded));
            }
            if let Ok(decoded) =
                <GetContractVersionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetContractVersions(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetPerpBalance(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetPerpBalances(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetPerpProduct(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetPerpProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetSpotBalance(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetSpotBalances(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetSpotProduct(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetSpotProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetSubaccountInfo(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuerierCalls::Initialize(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for QuerierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                QuerierCalls::GetAllBooks(element) => element.encode(),
                QuerierCalls::GetAllProducts(element) => element.encode(),
                QuerierCalls::GetBookInfo(element) => element.encode(),
                QuerierCalls::GetClearinghouse(element) => element.encode(),
                QuerierCalls::GetContractVersions(element) => element.encode(),
                QuerierCalls::GetPerpBalance(element) => element.encode(),
                QuerierCalls::GetPerpBalances(element) => element.encode(),
                QuerierCalls::GetPerpProduct(element) => element.encode(),
                QuerierCalls::GetPerpProducts(element) => element.encode(),
                QuerierCalls::GetSpotBalance(element) => element.encode(),
                QuerierCalls::GetSpotBalances(element) => element.encode(),
                QuerierCalls::GetSpotProduct(element) => element.encode(),
                QuerierCalls::GetSpotProducts(element) => element.encode(),
                QuerierCalls::GetSubaccountInfo(element) => element.encode(),
                QuerierCalls::GetVersion(element) => element.encode(),
                QuerierCalls::Initialize(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for QuerierCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                QuerierCalls::GetAllBooks(element) => element.fmt(f),
                QuerierCalls::GetAllProducts(element) => element.fmt(f),
                QuerierCalls::GetBookInfo(element) => element.fmt(f),
                QuerierCalls::GetClearinghouse(element) => element.fmt(f),
                QuerierCalls::GetContractVersions(element) => element.fmt(f),
                QuerierCalls::GetPerpBalance(element) => element.fmt(f),
                QuerierCalls::GetPerpBalances(element) => element.fmt(f),
                QuerierCalls::GetPerpProduct(element) => element.fmt(f),
                QuerierCalls::GetPerpProducts(element) => element.fmt(f),
                QuerierCalls::GetSpotBalance(element) => element.fmt(f),
                QuerierCalls::GetSpotBalances(element) => element.fmt(f),
                QuerierCalls::GetSpotProduct(element) => element.fmt(f),
                QuerierCalls::GetSpotProducts(element) => element.fmt(f),
                QuerierCalls::GetSubaccountInfo(element) => element.fmt(f),
                QuerierCalls::GetVersion(element) => element.fmt(f),
                QuerierCalls::Initialize(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAllBooksCall> for QuerierCalls {
        fn from(var: GetAllBooksCall) -> Self {
            QuerierCalls::GetAllBooks(var)
        }
    }
    impl ::std::convert::From<GetAllProductsCall> for QuerierCalls {
        fn from(var: GetAllProductsCall) -> Self {
            QuerierCalls::GetAllProducts(var)
        }
    }
    impl ::std::convert::From<GetBookInfoCall> for QuerierCalls {
        fn from(var: GetBookInfoCall) -> Self {
            QuerierCalls::GetBookInfo(var)
        }
    }
    impl ::std::convert::From<GetClearinghouseCall> for QuerierCalls {
        fn from(var: GetClearinghouseCall) -> Self {
            QuerierCalls::GetClearinghouse(var)
        }
    }
    impl ::std::convert::From<GetContractVersionsCall> for QuerierCalls {
        fn from(var: GetContractVersionsCall) -> Self {
            QuerierCalls::GetContractVersions(var)
        }
    }
    impl ::std::convert::From<GetPerpBalanceCall> for QuerierCalls {
        fn from(var: GetPerpBalanceCall) -> Self {
            QuerierCalls::GetPerpBalance(var)
        }
    }
    impl ::std::convert::From<GetPerpBalancesCall> for QuerierCalls {
        fn from(var: GetPerpBalancesCall) -> Self {
            QuerierCalls::GetPerpBalances(var)
        }
    }
    impl ::std::convert::From<GetPerpProductCall> for QuerierCalls {
        fn from(var: GetPerpProductCall) -> Self {
            QuerierCalls::GetPerpProduct(var)
        }
    }
    impl ::std::convert::From<GetPerpProductsCall> for QuerierCalls {
        fn from(var: GetPerpProductsCall) -> Self {
            QuerierCalls::GetPerpProducts(var)
        }
    }
    impl ::std::convert::From<GetSpotBalanceCall> for QuerierCalls {
        fn from(var: GetSpotBalanceCall) -> Self {
            QuerierCalls::GetSpotBalance(var)
        }
    }
    impl ::std::convert::From<GetSpotBalancesCall> for QuerierCalls {
        fn from(var: GetSpotBalancesCall) -> Self {
            QuerierCalls::GetSpotBalances(var)
        }
    }
    impl ::std::convert::From<GetSpotProductCall> for QuerierCalls {
        fn from(var: GetSpotProductCall) -> Self {
            QuerierCalls::GetSpotProduct(var)
        }
    }
    impl ::std::convert::From<GetSpotProductsCall> for QuerierCalls {
        fn from(var: GetSpotProductsCall) -> Self {
            QuerierCalls::GetSpotProducts(var)
        }
    }
    impl ::std::convert::From<GetSubaccountInfoCall> for QuerierCalls {
        fn from(var: GetSubaccountInfoCall) -> Self {
            QuerierCalls::GetSubaccountInfo(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for QuerierCalls {
        fn from(var: GetVersionCall) -> Self {
            QuerierCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for QuerierCalls {
        fn from(var: InitializeCall) -> Self {
            QuerierCalls::Initialize(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAllBooks` function with signature `getAllBooks()` and selector `[53, 69, 40, 232]`"]
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
    pub struct GetAllBooksReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getAllProducts` function with signature `getAllProducts()` and selector `[2, 238, 58, 82]`"]
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
    pub struct GetAllProductsReturn(pub ProductInfo);
    #[doc = "Container type for all return fields from the `getBookInfo` function with signature `getBookInfo(uint32,address)` and selector `[97, 36, 229, 255]`"]
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
    pub struct GetBookInfoReturn {
        pub book_info: BookInfo,
    }
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
    #[doc = "Container type for all return fields from the `getContractVersions` function with signature `getContractVersions(address,address,address,address,address,address,address)` and selector `[228, 113, 126, 114]`"]
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
    pub struct GetContractVersionsReturn(pub Versions);
    #[doc = "Container type for all return fields from the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `[215, 178, 41, 182]`"]
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
    pub struct GetPerpBalanceReturn(pub PerpBalance);
    #[doc = "Container type for all return fields from the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `[37, 147, 235, 95]`"]
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
    pub struct GetPerpBalancesReturn {
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
    }
    #[doc = "Container type for all return fields from the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `[26, 225, 11, 197]`"]
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
    pub struct GetPerpProductReturn(pub PerpProduct);
    #[doc = "Container type for all return fields from the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `[238, 153, 40, 201]`"]
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
    pub struct GetPerpProductsReturn {
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    #[doc = "Container type for all return fields from the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `[116, 23, 52, 4]`"]
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
    pub struct GetSpotBalanceReturn(pub SpotBalance);
    #[doc = "Container type for all return fields from the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `[49, 84, 109, 81]`"]
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
    pub struct GetSpotBalancesReturn {
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
    }
    #[doc = "Container type for all return fields from the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `[87, 35, 101, 63]`"]
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
    pub struct GetSpotProductReturn(pub SpotProduct);
    #[doc = "Container type for all return fields from the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `[117, 165, 171, 60]`"]
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
    pub struct GetSpotProductsReturn {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
    }
    #[doc = "Container type for all return fields from the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `[93, 112, 46, 26]`"]
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
    pub struct GetSubaccountInfoReturn(pub SubaccountInfo);
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
    #[doc = "`BookInfo(int128,int128,int128,int128,int128)`"]
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
    pub struct BookInfo {
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
        pub collected_fees: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub lp_spread_x18: i128,
    }
    #[doc = "`HealthInfo(int128,int128,int128)`"]
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
    pub struct HealthInfo {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub assets: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub liabilities: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub health: i128,
    }
    #[doc = "`PerpBalance(uint32,(int128,int128),(int128,int128,int128))`"]
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
    pub struct PerpBalance {
        pub product_id: u32,
        pub lp_balance: crate::bindings::perp_engine::LpBalance,
        pub balance: crate::bindings::perp_engine::Balance,
    }
    #[doc = "`PerpProduct(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))`"]
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
    pub struct PerpProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub state: crate::bindings::perp_engine::State,
        pub lp_state: crate::bindings::perp_engine::LpState,
        pub book_info: BookInfo,
    }
    #[doc = "`ProductInfo((uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`"]
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
    pub struct ProductInfo {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    #[doc = "`SpotBalance(uint32,(int128),(int128,int128))`"]
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
    pub struct SpotBalance {
        pub product_id: u32,
        pub lp_balance: crate::bindings::spot_engine::LpBalance,
        pub balance: crate::bindings::spot_engine::Balance,
    }
    #[doc = "`SpotProduct(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))`"]
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
    pub struct SpotProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub config: Config,
        pub state: crate::bindings::spot_engine::State,
        pub lp_state: crate::bindings::spot_engine::LpState,
        pub book_info: BookInfo,
    }
    #[doc = "`SubaccountInfo(bytes32,bool,(int128,int128,int128)[],int128[][],uint32,uint32,(uint32,(int128),(int128,int128))[],(uint32,(int128,int128),(int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`"]
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
    pub struct SubaccountInfo {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub exists: bool,
        pub healths: ::std::vec::Vec<HealthInfo>,
        #[serde(
            serialize_with = "serialize_nested_vec_i128",
            deserialize_with = "deserialize_nested_vec_i128"
        )]
        pub health_contributions: Vec<Vec<i128>>,
        pub spot_count: u32,
        pub perp_count: u32,
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    #[doc = "`IperpEngineBalance(int128,int128,int128)`"]
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
    pub struct IperpEngineBalance {
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
    #[doc = "`IperpEngineLpBalance(int128,int128)`"]
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
    pub struct IperpEngineLpBalance {
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
    #[doc = "`IperpEngineLpState(int128,int128,int128,int128,int128)`"]
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
    pub struct IperpEngineLpState {
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
    #[doc = "`IperpEngineState(int128,int128,int128,int128)`"]
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
    pub struct IperpEngineState {
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
    #[doc = "`IspotEngineBalance(int128,int128)`"]
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
    pub struct IspotEngineBalance {
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
    #[doc = "`IspotEngineLpBalance(int128)`"]
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
    pub struct IspotEngineLpBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
    }
    #[doc = "`IspotEngineLpState(int128,(int128,int128),(int128,int128))`"]
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
    pub struct IspotEngineLpState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub supply: i128,
        pub quote: crate::bindings::spot_engine::Balance,
        pub base: crate::bindings::spot_engine::Balance,
    }
    #[doc = "`IspotEngineState(int128,int128,int128,int128)`"]
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
    pub struct IspotEngineState {
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
        pub large_position_penalty_x18: i128,
    }
    #[doc = "`Versions(uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64[])`"]
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
    pub struct Versions {
        pub endpoint: u64,
        pub fee_calculator: u64,
        pub clearinghouse: u64,
        pub clearinghouse_liq: u64,
        pub spot_engine: u64,
        pub perp_engine: u64,
        pub querier: u64,
        pub books: Vec<u64>,
    }
}
