pub use endpoint::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod endpoint {
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
    #[doc = "Endpoint was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"SubmitTransactions\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct IEndpoint.BurnLpAndTransfer\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"recipient\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"burnLpAndTransfer\",\"outputs\":[{\"internalType\":\"struct IEndpoint.BurnLpAndTransfer\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"recipient\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkLpAction\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkLpActionInner\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"checkSlowModeTxInner\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkSlowModeTxLinkSigner\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.ClaimSequencerFees\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"claimSequencerFees\",\"outputs\":[{\"internalType\":\"struct IEndpoint.ClaimSequencerFees\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"clearinghouse\",\"outputs\":[{\"internalType\":\"contract IClearinghouse\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"subaccountName\",\"type\":\"bytes12\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"referralCode\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositCollateralWithReferral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"subaccountName\",\"type\":\"bytes12\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"referralCode\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositCollateralWithReferral\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeSlowModeTransaction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBook\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getHealthCheckFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLinkedSigner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLiquidationFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNumSubaccounts\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPricesX18\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Prices\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"spotPriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"perpPriceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSequencer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"subaccountId\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubaccountById\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubaccountId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getTakerSequencerFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTime\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"incrementSubmissions\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sanctions\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_sequencer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IClearinghouse\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"slowModeTimeout\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_time\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"_prices\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidationStart\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.ManualAssert\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128[]\",\"name\":\"openInterests\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"totalDeposits\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"totalBorrows\",\"type\":\"int128[]\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"manualAssert\",\"outputs\":[{\"internalType\":\"struct IEndpoint.ManualAssert\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128[]\",\"name\":\"openInterests\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"totalDeposits\",\"type\":\"int128[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"totalBorrows\",\"type\":\"int128[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrderAMM\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"matchOrderAMM\",\"outputs\":[{\"internalType\":\"struct IEndpoint.MatchOrderAMM\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrders\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"amm\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"maker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"matchOrders\",\"outputs\":[{\"internalType\":\"struct IEndpoint.MatchOrders\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"amm\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"maker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nSubmissions\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.PerpTick\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"time\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"avgPriceDiffs\",\"type\":\"int128[]\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"perpTick\",\"outputs\":[{\"internalType\":\"struct IEndpoint.PerpTick\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"time\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"avgPriceDiffs\",\"type\":\"int128[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"processSlowModeTransaction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.Rebate\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"subaccounts\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"amounts\",\"type\":\"int128[]\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"rebate\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Rebate\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"subaccounts\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"amounts\",\"type\":\"int128[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"referralCodes\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"wallet\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"transferable\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerTransferableWallet\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requireSubaccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerFees\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"book\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBook\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceX18\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sequencer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SlowModeConfig\",\"name\":\"_slowModeConfig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"timeout\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"txCount\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"txUpTo\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSlowModeConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"idx\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SlowModeTx\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"executableAt\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"tx\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSlowModeTx\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SettlePnl\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"subaccounts\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"productIds\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"settlePnl\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SettlePnl\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"subaccounts\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"productIds\",\"type\":\"uint256[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedBurnLp\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.BurnLp\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedBurnLp\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedBurnLp\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.BurnLp\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct FEndpoint.SignedCancellation\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct FEndpoint.Cancellation\",\"name\":\"cancellation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"digests\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedCancellation\",\"outputs\":[{\"internalType\":\"struct FEndpoint.SignedCancellation\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct FEndpoint.Cancellation\",\"name\":\"cancellation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"digests\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct FEndpoint.SignedCancellationProducts\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct FEndpoint.CancellationProducts\",\"name\":\"cancellationProducts\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedCancellationProducts\",\"outputs\":[{\"internalType\":\"struct FEndpoint.SignedCancellationProducts\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct FEndpoint.CancellationProducts\",\"name\":\"cancellationProducts\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedLinkSigner\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.LinkSigner\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"signer\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedLinkSigner\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedLinkSigner\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.LinkSigner\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"signer\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedLiquidateSubaccount\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedLiquidateSubaccount\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedLiquidateSubaccount\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedMintLp\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedMintLp\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedMintLp\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedOrder\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedWithdrawCollateral\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.WithdrawCollateral\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"signedWithdrawCollateral\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SignedWithdrawCollateral\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.WithdrawCollateral\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"slowModeConfig\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"timeout\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"txCount\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"txUpTo\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"slowModeTxs\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"executableAt\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"tx\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SpotTick\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"time\",\"type\":\"uint128\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"spotTick\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SpotTick\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"time\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transaction\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitSlowModeTransaction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"transactions\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTransactions\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"idx\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"transactions\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTransactionsChecked\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"idx\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"transactions\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTransactionsCheckedWithGasLimit\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SwapAMM\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"swapAMM\",\"outputs\":[{\"internalType\":\"struct IEndpoint.SwapAMM\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.BurnLp\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedBurnLp\",\"outputs\":[{\"internalType\":\"struct IEndpoint.BurnLp\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.DepositCollateral\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedDepositCollateral\",\"outputs\":[{\"internalType\":\"struct IEndpoint.DepositCollateral\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.DepositInsurance\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedDepositInsurance\",\"outputs\":[{\"internalType\":\"struct IEndpoint.DepositInsurance\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LinkSigner\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"signer\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedLinkSigner\",\"outputs\":[{\"internalType\":\"struct IEndpoint.LinkSigner\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"signer\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedLiquidateSubaccount\",\"outputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedMintLp\",\"outputs\":[{\"internalType\":\"struct IEndpoint.MintLp\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amountBase\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountLow\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteAmountHigh\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.WithdrawCollateral\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unsignedWithdrawCollateral\",\"outputs\":[{\"internalType\":\"struct IEndpoint.WithdrawCollateral\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"amount\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.UpdateFeeRates\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"updateFeeRates\",\"outputs\":[{\"internalType\":\"struct IEndpoint.UpdateFeeRates\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.UpdatePrice\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"updatePrice\",\"outputs\":[{\"internalType\":\"struct IEndpoint.UpdatePrice\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.UpdateProduct\",\"name\":\"p\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"tx\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"updateProduct\",\"outputs\":[{\"internalType\":\"struct IEndpoint.UpdateProduct\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"tx\",\"type\":\"bytes\",\"components\":[]}]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ENDPOINT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Endpoint<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Endpoint<M> {
        fn clone(&self) -> Self {
            Endpoint(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Endpoint<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Endpoint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Endpoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Endpoint<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ENDPOINT_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `burnLpAndTransfer` (0x0748a219) function"]
        pub fn burn_lp_and_transfer(
            &self,
            p: BurnLpAndTransfer,
        ) -> ethers::contract::builders::ContractCall<M, BurnLpAndTransfer> {
            self.0
                .method_hash([7, 72, 162, 25], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkLpAction` (0x8c3d2f74) function"]
        pub fn check_lp_action(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([140, 61, 47, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkLpActionInner` (0xc345530b) function"]
        pub fn check_lp_action_inner(
            &self,
            sender: ethers::core::types::Address,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([195, 69, 83, 11], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkSlowModeTxInner` (0xb70eb263) function"]
        pub fn check_slow_mode_tx_inner(
            &self,
            sender: ethers::core::types::Address,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([183, 14, 178, 99], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkSlowModeTxLinkSigner` (0x5bb4c126) function"]
        pub fn check_slow_mode_tx_link_signer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 180, 193, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimSequencerFees` (0x3842e75e) function"]
        pub fn claim_sequencer_fees(
            &self,
            p: ClaimSequencerFees,
        ) -> ethers::contract::builders::ContractCall<M, ClaimSequencerFees> {
            self.0
                .method_hash([56, 66, 231, 94], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `clearinghouse` (0x5d4f5f97) function"]
        pub fn clearinghouse(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([93, 79, 95, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCollateral` (0x8e5d588c) function"]
        pub fn deposit_collateral(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 93, 88, 140], (subaccount_name, product_id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCollateralWithReferral` (0x221f0939) function"]
        pub fn deposit_collateral_with_referral(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            amount: u128,
            referral_code: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 31, 9, 57],
                    (subaccount, product_id, amount, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositCollateralWithReferral` (0xe9bc7462) function"]
        pub fn deposit_collateral_with_referral_with_subaccount_name_and_product_id_and_amount(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
            referral_code: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 188, 116, 98],
                    (subaccount_name, product_id, amount, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeSlowModeTransaction` (0x65dd1366) function"]
        pub fn execute_slow_mode_transaction(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 221, 19, 102], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBook` (0xcf987c9b) function"]
        pub fn get_book(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 152, 124, 155], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHealthCheckFee` (0xd4de8d9d) function"]
        pub fn get_health_check_fee(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([212, 222, 141, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLinkedSigner` (0x91c1e3d7) function"]
        pub fn get_linked_signer(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([145, 193, 227, 215], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidationFee` (0xfbf41984) function"]
        pub fn get_liquidation_fee(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([251, 244, 25, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNonce` (0x2d0335ab) function"]
        pub fn get_nonce(
            &self,
            sender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], sender)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNumSubaccounts` (0xc4f9b25f) function"]
        pub fn get_num_subaccounts(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([196, 249, 178, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceX18` (0x368e4686) function"]
        pub fn get_price_x18(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([54, 142, 70, 134], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPricesX18` (0xafa55dee) function"]
        pub fn get_prices_x18(
            &self,
            health_group: u32,
        ) -> ethers::contract::builders::ContractCall<M, Prices> {
            self.0
                .method_hash([175, 165, 93, 238], health_group)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSequencer` (0x4d96a90a) function"]
        pub fn get_sequencer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([77, 150, 169, 10], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSubaccountById` (0xef64ed0e) function"]
        pub fn get_subaccount_by_id(
            &self,
            subaccount_id: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([239, 100, 237, 14], subaccount_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSubaccountId` (0x22d4a82d) function"]
        pub fn get_subaccount_id(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([34, 212, 168, 45], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTakerSequencerFee` (0xc510359f) function"]
        pub fn get_taker_sequencer_fee(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([197, 16, 53, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTime` (0x557ed1ba) function"]
        pub fn get_time(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([85, 126, 209, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementSubmissions` (0x22006046) function"]
        pub fn increment_submissions(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 0, 96, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xa53051bc) function"]
        pub fn initialize(
            &self,
            sanctions: ethers::core::types::Address,
            sequencer: ethers::core::types::Address,
            clearinghouse: ethers::core::types::Address,
            slow_mode_timeout: u64,
            time: u128,
            prices: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [165, 48, 81, 188],
                    (
                        sanctions,
                        sequencer,
                        clearinghouse,
                        slow_mode_timeout,
                        time,
                        prices,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationStart` (0x8d0acc9b) function"]
        pub fn liquidation_start(
            &self,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 10, 204, 155], transaction)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manualAssert` (0x2c8c6ffb) function"]
        pub fn manual_assert(
            &self,
            p: ManualAssert,
        ) -> ethers::contract::builders::ContractCall<M, ManualAssert> {
            self.0
                .method_hash([44, 140, 111, 251], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrderAMM` (0x36b90c51) function"]
        pub fn match_order_amm(
            &self,
            p: MatchOrderAMM,
        ) -> ethers::contract::builders::ContractCall<M, MatchOrderAMM> {
            self.0
                .method_hash([54, 185, 12, 81], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrders` (0x45160e52) function"]
        pub fn match_orders(
            &self,
            p: MatchOrders,
        ) -> ethers::contract::builders::ContractCall<M, MatchOrders> {
            self.0
                .method_hash([69, 22, 14, 82], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrate` (0x8fd3ab80) function"]
        pub fn migrate(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 211, 171, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nSubmissions` (0x18ed16eb) function"]
        pub fn n_submissions(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([24, 237, 22, 235], ())
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
        #[doc = "Calls the contract's `perpTick` (0x5a00923b) function"]
        pub fn perp_tick(
            &self,
            p: PerpTick,
        ) -> ethers::contract::builders::ContractCall<M, PerpTick> {
            self.0
                .method_hash([90, 0, 146, 59], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `processSlowModeTransaction` (0x87324338) function"]
        pub fn process_slow_mode_transaction(
            &self,
            sender: ethers::core::types::Address,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 50, 67, 56], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebate` (0x42c74d1d) function"]
        pub fn rebate(&self, p: Rebate) -> ethers::contract::builders::ContractCall<M, Rebate> {
            self.0
                .method_hash([66, 199, 77, 29], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `referralCodes` (0x9534dd3e) function"]
        pub fn referral_codes(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 52, 221, 62], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerTransferableWallet` (0x6cfe5fe4) function"]
        pub fn register_transferable_wallet(
            &self,
            wallet: ethers::core::types::Address,
            transferable: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 254, 95, 228], (wallet, transferable))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requireSubaccount` (0xd6473aaf) function"]
        pub fn require_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 71, 58, 175], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequencerFee` (0xae3a8652) function"]
        pub fn sequencer_fee(&self, p0: u32) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([174, 58, 134, 82], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequencerFees` (0x0d3ad039) function"]
        pub fn sequencer_fees(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([13, 58, 208, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBook` (0x39a135e7) function"]
        pub fn set_book(
            &self,
            product_id: u32,
            book: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 161, 53, 231], (product_id, book))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceX18` (0x8c58e10a) function"]
        pub fn set_price_x18(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 88, 225, 10], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSequencer` (0x2547fa3e) function"]
        pub fn set_sequencer(
            &self,
            sequencer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 71, 250, 62], sequencer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSlowModeConfig` (0x3216c062) function"]
        pub fn set_slow_mode_config(
            &self,
            slow_mode_config: SlowModeConfig,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 22, 192, 98], (slow_mode_config,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSlowModeTx` (0x98cd32fe) function"]
        pub fn set_slow_mode_tx(
            &self,
            idx: u64,
            txn: SlowModeTx,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 205, 50, 254], (idx, txn))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `settlePnl` (0xb2bb6367) function"]
        pub fn settle_pnl(
            &self,
            p: SettlePnl,
        ) -> ethers::contract::builders::ContractCall<M, SettlePnl> {
            self.0
                .method_hash([178, 187, 99, 103], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedBurnLp` (0x610b2e5e) function"]
        pub fn signed_burn_lp(
            &self,
            p: SignedBurnLp,
        ) -> ethers::contract::builders::ContractCall<M, SignedBurnLp> {
            self.0
                .method_hash([97, 11, 46, 94], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedCancellation` (0x3edf2c5b) function"]
        pub fn signed_cancellation(
            &self,
            p: SignedCancellation,
        ) -> ethers::contract::builders::ContractCall<M, SignedCancellation> {
            self.0
                .method_hash([62, 223, 44, 91], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedCancellationProducts` (0xa082c5aa) function"]
        pub fn signed_cancellation_products(
            &self,
            p: SignedCancellationProducts,
        ) -> ethers::contract::builders::ContractCall<M, SignedCancellationProducts> {
            self.0
                .method_hash([160, 130, 197, 170], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedLinkSigner` (0x85c83e9d) function"]
        pub fn signed_link_signer(
            &self,
            p: SignedLinkSigner,
        ) -> ethers::contract::builders::ContractCall<M, SignedLinkSigner> {
            self.0
                .method_hash([133, 200, 62, 157], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedLiquidateSubaccount` (0xbf832e77) function"]
        pub fn signed_liquidate_subaccount(
            &self,
            p: SignedLiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, SignedLiquidateSubaccount> {
            self.0
                .method_hash([191, 131, 46, 119], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedMintLp` (0xd38c3b9c) function"]
        pub fn signed_mint_lp(
            &self,
            p: SignedMintLp,
        ) -> ethers::contract::builders::ContractCall<M, SignedMintLp> {
            self.0
                .method_hash([211, 140, 59, 156], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedOrder` (0x6c457570) function"]
        pub fn signed_order(
            &self,
            p: SignedOrder,
        ) -> ethers::contract::builders::ContractCall<M, SignedOrder> {
            self.0
                .method_hash([108, 69, 117, 112], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedWithdrawCollateral` (0x0d55e26b) function"]
        pub fn signed_withdraw_collateral(
            &self,
            p: SignedWithdrawCollateral,
        ) -> ethers::contract::builders::ContractCall<M, SignedWithdrawCollateral> {
            self.0
                .method_hash([13, 85, 226, 107], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slowModeConfig` (0xb8043ede) function"]
        pub fn slow_mode_config(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64, u64)> {
            self.0
                .method_hash([184, 4, 62, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slowModeTxs` (0xcf8fdb09) function"]
        pub fn slow_mode_txs(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                ethers::core::types::Address,
                ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([207, 143, 219, 9], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `spotTick` (0xd9768695) function"]
        pub fn spot_tick(
            &self,
            p: SpotTick,
        ) -> ethers::contract::builders::ContractCall<M, SpotTick> {
            self.0
                .method_hash([217, 118, 134, 149], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitSlowModeTransaction` (0xe604ed9e) function"]
        pub fn submit_slow_mode_transaction(
            &self,
            transaction: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 4, 237, 158], transaction)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTransactions` (0x1f186b27) function"]
        pub fn submit_transactions(
            &self,
            transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 24, 107, 39], transactions)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTransactionsChecked` (0x2715e15c) function"]
        pub fn submit_transactions_checked(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 21, 225, 92], (idx, transactions))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTransactionsCheckedWithGasLimit` (0x2f9a2744) function"]
        pub fn submit_transactions_checked_with_gas_limit(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
            gas_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u64, ethers::core::types::U256)> {
            self.0
                .method_hash([47, 154, 39, 68], (idx, transactions, gas_limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAMM` (0x0f4b509d) function"]
        pub fn swap_amm(&self, p: SwapAMM) -> ethers::contract::builders::ContractCall<M, SwapAMM> {
            self.0
                .method_hash([15, 75, 80, 157], (p,))
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
        #[doc = "Calls the contract's `unsignedBurnLp` (0x06c0bafd) function"]
        pub fn unsigned_burn_lp(
            &self,
            p: BurnLp,
        ) -> ethers::contract::builders::ContractCall<M, BurnLp> {
            self.0
                .method_hash([6, 192, 186, 253], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedDepositCollateral` (0x3cec4b93) function"]
        pub fn unsigned_deposit_collateral(
            &self,
            p: DepositCollateral,
        ) -> ethers::contract::builders::ContractCall<M, DepositCollateral> {
            self.0
                .method_hash([60, 236, 75, 147], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedDepositInsurance` (0x94faefe5) function"]
        pub fn unsigned_deposit_insurance(
            &self,
            p: DepositInsurance,
        ) -> ethers::contract::builders::ContractCall<M, DepositInsurance> {
            self.0
                .method_hash([148, 250, 239, 229], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedLinkSigner` (0x05e42dc7) function"]
        pub fn unsigned_link_signer(
            &self,
            p: LinkSigner,
        ) -> ethers::contract::builders::ContractCall<M, LinkSigner> {
            self.0
                .method_hash([5, 228, 45, 199], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedLiquidateSubaccount` (0xf9e473fc) function"]
        pub fn unsigned_liquidate_subaccount(
            &self,
            p: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, LiquidateSubaccount> {
            self.0
                .method_hash([249, 228, 115, 252], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedMintLp` (0x910e606a) function"]
        pub fn unsigned_mint_lp(
            &self,
            p: MintLp,
        ) -> ethers::contract::builders::ContractCall<M, MintLp> {
            self.0
                .method_hash([145, 14, 96, 106], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unsignedWithdrawCollateral` (0xbc85ca86) function"]
        pub fn unsigned_withdraw_collateral(
            &self,
            p: WithdrawCollateral,
        ) -> ethers::contract::builders::ContractCall<M, WithdrawCollateral> {
            self.0
                .method_hash([188, 133, 202, 134], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeRates` (0x35639a4f) function"]
        pub fn update_fee_rates(
            &self,
            p: UpdateFeeRates,
        ) -> ethers::contract::builders::ContractCall<M, UpdateFeeRates> {
            self.0
                .method_hash([53, 99, 154, 79], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePrice` (0x1d9eeda5) function"]
        pub fn update_price(
            &self,
            p: UpdatePrice,
        ) -> ethers::contract::builders::ContractCall<M, UpdatePrice> {
            self.0
                .method_hash([29, 158, 237, 165], (p,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateProduct` (0x2cd71b16) function"]
        pub fn update_product(
            &self,
            p: UpdateProduct,
        ) -> ethers::contract::builders::ContractCall<M, UpdateProduct> {
            self.0
                .method_hash([44, 215, 27, 22], (p,))
                .expect("method not found (this should never happen)")
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
        #[doc = "Gets the contract's `SubmitTransactions` event"]
        pub fn submit_transactions_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SubmitTransactionsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, EndpointEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Endpoint<M> {
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
    #[ethevent(name = "SubmitTransactions", abi = "SubmitTransactions()")]
    pub struct SubmitTransactionsFilter();
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum EndpointEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SubmitTransactionsFilter(SubmitTransactionsFilter),
    }
    impl ethers::contract::EthLogDecode for EndpointEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EndpointEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(EndpointEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SubmitTransactionsFilter::decode_log(log) {
                return Ok(EndpointEvents::SubmitTransactionsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for EndpointEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                EndpointEvents::InitializedFilter(element) => element.fmt(f),
                EndpointEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                EndpointEvents::SubmitTransactionsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `[7, 72, 162, 25]`"]
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
        name = "burnLpAndTransfer",
        abi = "burnLpAndTransfer((bytes32,uint32,uint128,bytes32))"
    )]
    pub struct BurnLpAndTransferCall {
        pub p: BurnLpAndTransfer,
    }
    #[doc = "Container type for all input parameters for the `checkLpAction` function with signature `checkLpAction()` and selector `[140, 61, 47, 116]`"]
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
    #[ethcall(name = "checkLpAction", abi = "checkLpAction()")]
    pub struct CheckLpActionCall;
    #[doc = "Container type for all input parameters for the `checkLpActionInner` function with signature `checkLpActionInner(address,bytes)` and selector `[195, 69, 83, 11]`"]
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
    #[ethcall(name = "checkLpActionInner", abi = "checkLpActionInner(address,bytes)")]
    pub struct CheckLpActionInnerCall {
        pub sender: ethers::core::types::Address,
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `[183, 14, 178, 99]`"]
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
        name = "checkSlowModeTxInner",
        abi = "checkSlowModeTxInner(address,bytes)"
    )]
    pub struct CheckSlowModeTxInnerCall {
        pub sender: ethers::core::types::Address,
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `[91, 180, 193, 38]`"]
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
        name = "checkSlowModeTxLinkSigner",
        abi = "checkSlowModeTxLinkSigner()"
    )]
    pub struct CheckSlowModeTxLinkSignerCall;
    #[doc = "Container type for all input parameters for the `claimSequencerFees` function with signature `claimSequencerFees((bytes32))` and selector `[56, 66, 231, 94]`"]
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
    #[ethcall(name = "claimSequencerFees", abi = "claimSequencerFees((bytes32))")]
    pub struct ClaimSequencerFeesCall {
        pub p: ClaimSequencerFees,
    }
    #[doc = "Container type for all input parameters for the `clearinghouse` function with signature `clearinghouse()` and selector `[93, 79, 95, 151]`"]
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
    #[ethcall(name = "clearinghouse", abi = "clearinghouse()")]
    pub struct ClearinghouseCall;
    #[doc = "Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral(bytes12,uint32,uint128)` and selector `[142, 93, 88, 140]`"]
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
        name = "depositCollateral",
        abi = "depositCollateral(bytes12,uint32,uint128)"
    )]
    pub struct DepositCollateralCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
    }
    #[doc = "Container type for all input parameters for the `depositCollateralWithReferral` function with signature `depositCollateralWithReferral(bytes32,uint32,uint128,string)` and selector `[34, 31, 9, 57]`"]
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
        name = "depositCollateralWithReferral",
        abi = "depositCollateralWithReferral(bytes32,uint32,uint128,string)"
    )]
    pub struct DepositCollateralWithReferralCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
        pub referral_code: String,
    }
    #[doc = "Container type for all input parameters for the `depositCollateralWithReferral` function with signature `depositCollateralWithReferral(bytes12,uint32,uint128,string)` and selector `[233, 188, 116, 98]`"]
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
        name = "depositCollateralWithReferral",
        abi = "depositCollateralWithReferral(bytes12,uint32,uint128,string)"
    )]
    pub struct DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
        pub referral_code: String,
    }
    #[doc = "Container type for all input parameters for the `executeSlowModeTransaction` function with signature `executeSlowModeTransaction()` and selector `[101, 221, 19, 102]`"]
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
        name = "executeSlowModeTransaction",
        abi = "executeSlowModeTransaction()"
    )]
    pub struct ExecuteSlowModeTransactionCall;
    #[doc = "Container type for all input parameters for the `getBook` function with signature `getBook(uint32)` and selector `[207, 152, 124, 155]`"]
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
    #[ethcall(name = "getBook", abi = "getBook(uint32)")]
    pub struct GetBookCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `[212, 222, 141, 157]`"]
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
    #[ethcall(name = "getHealthCheckFee", abi = "getHealthCheckFee()")]
    pub struct GetHealthCheckFeeCall;
    #[doc = "Container type for all input parameters for the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `[145, 193, 227, 215]`"]
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
    #[ethcall(name = "getLinkedSigner", abi = "getLinkedSigner(bytes32)")]
    pub struct GetLinkedSignerCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `[251, 244, 25, 132]`"]
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
    #[ethcall(name = "getLiquidationFee", abi = "getLiquidationFee()")]
    pub struct GetLiquidationFeeCall;
    #[doc = "Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `[45, 3, 53, 171]`"]
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
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub sender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `[196, 249, 178, 95]`"]
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
    #[ethcall(name = "getNumSubaccounts", abi = "getNumSubaccounts()")]
    pub struct GetNumSubaccountsCall;
    #[doc = "Container type for all input parameters for the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `[54, 142, 70, 134]`"]
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
    #[ethcall(name = "getPriceX18", abi = "getPriceX18(uint32)")]
    pub struct GetPriceX18Call {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getPricesX18` function with signature `getPricesX18(uint32)` and selector `[175, 165, 93, 238]`"]
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
    #[ethcall(name = "getPricesX18", abi = "getPricesX18(uint32)")]
    pub struct GetPricesX18Call {
        pub health_group: u32,
    }
    #[doc = "Container type for all input parameters for the `getSequencer` function with signature `getSequencer()` and selector `[77, 150, 169, 10]`"]
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
    #[ethcall(name = "getSequencer", abi = "getSequencer()")]
    pub struct GetSequencerCall;
    #[doc = "Container type for all input parameters for the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `[239, 100, 237, 14]`"]
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
    #[ethcall(name = "getSubaccountById", abi = "getSubaccountById(uint64)")]
    pub struct GetSubaccountByIdCall {
        pub subaccount_id: u64,
    }
    #[doc = "Container type for all input parameters for the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `[34, 212, 168, 45]`"]
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
    #[ethcall(name = "getSubaccountId", abi = "getSubaccountId(bytes32)")]
    pub struct GetSubaccountIdCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `[197, 16, 53, 159]`"]
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
    #[ethcall(name = "getTakerSequencerFee", abi = "getTakerSequencerFee()")]
    pub struct GetTakerSequencerFeeCall;
    #[doc = "Container type for all input parameters for the `getTime` function with signature `getTime()` and selector `[85, 126, 209, 186]`"]
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
    #[ethcall(name = "getTime", abi = "getTime()")]
    pub struct GetTimeCall;
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
    #[doc = "Container type for all input parameters for the `incrementSubmissions` function with signature `incrementSubmissions()` and selector `[34, 0, 96, 70]`"]
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
    #[ethcall(name = "incrementSubmissions", abi = "incrementSubmissions()")]
    pub struct IncrementSubmissionsCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint64,uint128,int128[])` and selector `[165, 48, 81, 188]`"]
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
        abi = "initialize(address,address,address,uint64,uint128,int128[])"
    )]
    pub struct InitializeCall {
        pub sanctions: ethers::core::types::Address,
        pub sequencer: ethers::core::types::Address,
        pub clearinghouse: ethers::core::types::Address,
        pub slow_mode_timeout: u64,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub time: u128,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub prices: ::std::vec::Vec<i128>,
    }
    #[doc = "Container type for all input parameters for the `liquidationStart` function with signature `liquidationStart(bytes)` and selector `[141, 10, 204, 155]`"]
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
    #[ethcall(name = "liquidationStart", abi = "liquidationStart(bytes)")]
    pub struct LiquidationStartCall {
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `manualAssert` function with signature `manualAssert((int128[],int128[],int128[]))` and selector `[44, 140, 111, 251]`"]
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
        name = "manualAssert",
        abi = "manualAssert((int128[],int128[],int128[]))"
    )]
    pub struct ManualAssertCall {
        pub p: ManualAssert,
    }
    #[doc = "Container type for all input parameters for the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `[54, 185, 12, 81]`"]
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
        name = "matchOrderAMM",
        abi = "matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))"
    )]
    pub struct MatchOrderAMMCall {
        pub p: MatchOrderAMM,
    }
    #[doc = "Container type for all input parameters for the `matchOrders` function with signature `matchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `[69, 22, 14, 82]`"]
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
        name = "matchOrders",
        abi = "matchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))"
    )]
    pub struct MatchOrdersCall {
        pub p: MatchOrders,
    }
    #[doc = "Container type for all input parameters for the `migrate` function with signature `migrate()` and selector `[143, 211, 171, 128]`"]
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
    #[ethcall(name = "migrate", abi = "migrate()")]
    pub struct MigrateCall;
    #[doc = "Container type for all input parameters for the `nSubmissions` function with signature `nSubmissions()` and selector `[24, 237, 22, 235]`"]
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
    #[ethcall(name = "nSubmissions", abi = "nSubmissions()")]
    pub struct NsubmissionsCall;
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
    #[doc = "Container type for all input parameters for the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `[90, 0, 146, 59]`"]
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
    #[ethcall(name = "perpTick", abi = "perpTick((uint128,int128[]))")]
    pub struct PerpTickCall {
        pub p: PerpTick,
    }
    #[doc = "Container type for all input parameters for the `processSlowModeTransaction` function with signature `processSlowModeTransaction(address,bytes)` and selector `[135, 50, 67, 56]`"]
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
        name = "processSlowModeTransaction",
        abi = "processSlowModeTransaction(address,bytes)"
    )]
    pub struct ProcessSlowModeTransactionCall {
        pub sender: ethers::core::types::Address,
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `[66, 199, 77, 29]`"]
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
    #[ethcall(name = "rebate", abi = "rebate((bytes32[],int128[]))")]
    pub struct RebateCall {
        pub p: Rebate,
    }
    #[doc = "Container type for all input parameters for the `referralCodes` function with signature `referralCodes(address)` and selector `[149, 52, 221, 62]`"]
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
    #[ethcall(name = "referralCodes", abi = "referralCodes(address)")]
    pub struct ReferralCodesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `registerTransferableWallet` function with signature `registerTransferableWallet(address,bool)` and selector `[108, 254, 95, 228]`"]
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
        name = "registerTransferableWallet",
        abi = "registerTransferableWallet(address,bool)"
    )]
    pub struct RegisterTransferableWalletCall {
        pub wallet: ethers::core::types::Address,
        pub transferable: bool,
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
    #[doc = "Container type for all input parameters for the `requireSubaccount` function with signature `requireSubaccount(bytes32)` and selector `[214, 71, 58, 175]`"]
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
    #[ethcall(name = "requireSubaccount", abi = "requireSubaccount(bytes32)")]
    pub struct RequireSubaccountCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `sequencerFee` function with signature `sequencerFee(uint32)` and selector `[174, 58, 134, 82]`"]
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
    #[ethcall(name = "sequencerFee", abi = "sequencerFee(uint32)")]
    pub struct SequencerFeeCall(pub u32);
    #[doc = "Container type for all input parameters for the `sequencerFees` function with signature `sequencerFees()` and selector `[13, 58, 208, 57]`"]
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
    #[ethcall(name = "sequencerFees", abi = "sequencerFees()")]
    pub struct SequencerFeesCall;
    #[doc = "Container type for all input parameters for the `setBook` function with signature `setBook(uint32,address)` and selector `[57, 161, 53, 231]`"]
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
    #[ethcall(name = "setBook", abi = "setBook(uint32,address)")]
    pub struct SetBookCall {
        pub product_id: u32,
        pub book: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceX18` function with signature `setPriceX18(uint32,int128)` and selector `[140, 88, 225, 10]`"]
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
    #[ethcall(name = "setPriceX18", abi = "setPriceX18(uint32,int128)")]
    pub struct SetPriceX18Call {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
    #[doc = "Container type for all input parameters for the `setSequencer` function with signature `setSequencer(address)` and selector `[37, 71, 250, 62]`"]
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
    #[ethcall(name = "setSequencer", abi = "setSequencer(address)")]
    pub struct SetSequencerCall {
        pub sequencer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setSlowModeConfig` function with signature `setSlowModeConfig((uint64,uint64,uint64))` and selector `[50, 22, 192, 98]`"]
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
        name = "setSlowModeConfig",
        abi = "setSlowModeConfig((uint64,uint64,uint64))"
    )]
    pub struct SetSlowModeConfigCall {
        pub slow_mode_config: SlowModeConfig,
    }
    #[doc = "Container type for all input parameters for the `setSlowModeTx` function with signature `setSlowModeTx(uint64,(uint64,address,bytes))` and selector `[152, 205, 50, 254]`"]
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
        name = "setSlowModeTx",
        abi = "setSlowModeTx(uint64,(uint64,address,bytes))"
    )]
    pub struct SetSlowModeTxCall {
        pub idx: u64,
        pub txn: SlowModeTx,
    }
    #[doc = "Container type for all input parameters for the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `[178, 187, 99, 103]`"]
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
    #[ethcall(name = "settlePnl", abi = "settlePnl((bytes32[],uint256[]))")]
    pub struct SettlePnlCall {
        pub p: SettlePnl,
    }
    #[doc = "Container type for all input parameters for the `signedBurnLp` function with signature `signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))` and selector `[97, 11, 46, 94]`"]
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
        name = "signedBurnLp",
        abi = "signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))"
    )]
    pub struct SignedBurnLpCall {
        pub p: SignedBurnLp,
    }
    #[doc = "Container type for all input parameters for the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `[62, 223, 44, 91]`"]
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
        name = "signedCancellation",
        abi = "signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))"
    )]
    pub struct SignedCancellationCall {
        pub p: SignedCancellation,
    }
    #[doc = "Container type for all input parameters for the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `[160, 130, 197, 170]`"]
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
        name = "signedCancellationProducts",
        abi = "signedCancellationProducts(((bytes32,uint32[],uint64),bytes))"
    )]
    pub struct SignedCancellationProductsCall {
        pub p: SignedCancellationProducts,
    }
    #[doc = "Container type for all input parameters for the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `[133, 200, 62, 157]`"]
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
        name = "signedLinkSigner",
        abi = "signedLinkSigner(((bytes32,bytes32,uint64),bytes))"
    )]
    pub struct SignedLinkSignerCall {
        pub p: SignedLinkSigner,
    }
    #[doc = "Container type for all input parameters for the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))` and selector `[191, 131, 46, 119]`"]
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
        name = "signedLiquidateSubaccount",
        abi = "signedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))"
    )]
    pub struct SignedLiquidateSubaccountCall {
        pub p: SignedLiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `signedMintLp` function with signature `signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))` and selector `[211, 140, 59, 156]`"]
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
        name = "signedMintLp",
        abi = "signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))"
    )]
    pub struct SignedMintLpCall {
        pub p: SignedMintLp,
    }
    #[doc = "Container type for all input parameters for the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `[108, 69, 117, 112]`"]
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
        name = "signedOrder",
        abi = "signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))"
    )]
    pub struct SignedOrderCall {
        pub p: SignedOrder,
    }
    #[doc = "Container type for all input parameters for the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `[13, 85, 226, 107]`"]
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
        name = "signedWithdrawCollateral",
        abi = "signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))"
    )]
    pub struct SignedWithdrawCollateralCall {
        pub p: SignedWithdrawCollateral,
    }
    #[doc = "Container type for all input parameters for the `slowModeConfig` function with signature `slowModeConfig()` and selector `[184, 4, 62, 222]`"]
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
    #[ethcall(name = "slowModeConfig", abi = "slowModeConfig()")]
    pub struct SlowModeConfigCall;
    #[doc = "Container type for all input parameters for the `slowModeTxs` function with signature `slowModeTxs(uint64)` and selector `[207, 143, 219, 9]`"]
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
    #[ethcall(name = "slowModeTxs", abi = "slowModeTxs(uint64)")]
    pub struct SlowModeTxsCall(pub u64);
    #[doc = "Container type for all input parameters for the `spotTick` function with signature `spotTick((uint128))` and selector `[217, 118, 134, 149]`"]
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
    #[ethcall(name = "spotTick", abi = "spotTick((uint128))")]
    pub struct SpotTickCall {
        pub p: SpotTick,
    }
    #[doc = "Container type for all input parameters for the `submitSlowModeTransaction` function with signature `submitSlowModeTransaction(bytes)` and selector `[230, 4, 237, 158]`"]
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
        name = "submitSlowModeTransaction",
        abi = "submitSlowModeTransaction(bytes)"
    )]
    pub struct SubmitSlowModeTransactionCall {
        pub transaction: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `submitTransactions` function with signature `submitTransactions(bytes[])` and selector `[31, 24, 107, 39]`"]
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
    #[ethcall(name = "submitTransactions", abi = "submitTransactions(bytes[])")]
    pub struct SubmitTransactionsCall {
        pub transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `submitTransactionsChecked` function with signature `submitTransactionsChecked(uint64,bytes[])` and selector `[39, 21, 225, 92]`"]
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
        name = "submitTransactionsChecked",
        abi = "submitTransactionsChecked(uint64,bytes[])"
    )]
    pub struct SubmitTransactionsCheckedCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `submitTransactionsCheckedWithGasLimit` function with signature `submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)` and selector `[47, 154, 39, 68]`"]
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
        name = "submitTransactionsCheckedWithGasLimit",
        abi = "submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)"
    )]
    pub struct SubmitTransactionsCheckedWithGasLimitCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<ethers::core::types::Bytes>,
        #[serde(
            serialize_with = "serialize_u256",
            deserialize_with = "deserialize_u256"
        )]
        pub gas_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `[15, 75, 80, 157]`"]
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
    #[ethcall(name = "swapAMM", abi = "swapAMM((bytes32,uint32,int128,int128))")]
    pub struct SwapAMMCall {
        pub p: SwapAMM,
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
    #[doc = "Container type for all input parameters for the `unsignedBurnLp` function with signature `unsignedBurnLp((bytes32,uint32,uint128,uint64))` and selector `[6, 192, 186, 253]`"]
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
        name = "unsignedBurnLp",
        abi = "unsignedBurnLp((bytes32,uint32,uint128,uint64))"
    )]
    pub struct UnsignedBurnLpCall {
        pub p: BurnLp,
    }
    #[doc = "Container type for all input parameters for the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `[60, 236, 75, 147]`"]
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
        name = "unsignedDepositCollateral",
        abi = "unsignedDepositCollateral((bytes32,uint32,uint128))"
    )]
    pub struct UnsignedDepositCollateralCall {
        pub p: DepositCollateral,
    }
    #[doc = "Container type for all input parameters for the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `[148, 250, 239, 229]`"]
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
        name = "unsignedDepositInsurance",
        abi = "unsignedDepositInsurance((uint128))"
    )]
    pub struct UnsignedDepositInsuranceCall {
        pub p: DepositInsurance,
    }
    #[doc = "Container type for all input parameters for the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `[5, 228, 45, 199]`"]
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
        name = "unsignedLinkSigner",
        abi = "unsignedLinkSigner((bytes32,bytes32,uint64))"
    )]
    pub struct UnsignedLinkSignerCall {
        pub p: LinkSigner,
    }
    #[doc = "Container type for all input parameters for the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[249, 228, 115, 252]`"]
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
        name = "unsignedLiquidateSubaccount",
        abi = "unsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct UnsignedLiquidateSubaccountCall {
        pub p: LiquidateSubaccount,
    }
    #[doc = "Container type for all input parameters for the `unsignedMintLp` function with signature `unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `[145, 14, 96, 106]`"]
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
        name = "unsignedMintLp",
        abi = "unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct UnsignedMintLpCall {
        pub p: MintLp,
    }
    #[doc = "Container type for all input parameters for the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `[188, 133, 202, 134]`"]
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
        name = "unsignedWithdrawCollateral",
        abi = "unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))"
    )]
    pub struct UnsignedWithdrawCollateralCall {
        pub p: WithdrawCollateral,
    }
    #[doc = "Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `[53, 99, 154, 79]`"]
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
        name = "updateFeeRates",
        abi = "updateFeeRates((address,uint32,int64,int64))"
    )]
    pub struct UpdateFeeRatesCall {
        pub p: UpdateFeeRates,
    }
    #[doc = "Container type for all input parameters for the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `[29, 158, 237, 165]`"]
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
    #[ethcall(name = "updatePrice", abi = "updatePrice((uint32,int128))")]
    pub struct UpdatePriceCall {
        pub p: UpdatePrice,
    }
    #[doc = "Container type for all input parameters for the `updateProduct` function with signature `updateProduct((address,bytes))` and selector `[44, 215, 27, 22]`"]
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
    #[ethcall(name = "updateProduct", abi = "updateProduct((address,bytes))")]
    pub struct UpdateProductCall {
        pub p: UpdateProduct,
    }
    #[derive(
        Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType,
    )]
    pub enum EndpointCalls {
        BurnLpAndTransfer(BurnLpAndTransferCall),
        CheckLpAction(CheckLpActionCall),
        CheckLpActionInner(CheckLpActionInnerCall),
        CheckSlowModeTxInner(CheckSlowModeTxInnerCall),
        CheckSlowModeTxLinkSigner(CheckSlowModeTxLinkSignerCall),
        ClaimSequencerFees(ClaimSequencerFeesCall),
        Clearinghouse(ClearinghouseCall),
        DepositCollateral(DepositCollateralCall),
        DepositCollateralWithReferral(DepositCollateralWithReferralCall),
        DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(
            DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall,
        ),
        ExecuteSlowModeTransaction(ExecuteSlowModeTransactionCall),
        GetBook(GetBookCall),
        GetHealthCheckFee(GetHealthCheckFeeCall),
        GetLinkedSigner(GetLinkedSignerCall),
        GetLiquidationFee(GetLiquidationFeeCall),
        GetNonce(GetNonceCall),
        GetNumSubaccounts(GetNumSubaccountsCall),
        GetPriceX18(GetPriceX18Call),
        GetPricesX18(GetPricesX18Call),
        GetSequencer(GetSequencerCall),
        GetSubaccountById(GetSubaccountByIdCall),
        GetSubaccountId(GetSubaccountIdCall),
        GetTakerSequencerFee(GetTakerSequencerFeeCall),
        GetTime(GetTimeCall),
        GetVersion(GetVersionCall),
        IncrementSubmissions(IncrementSubmissionsCall),
        Initialize(InitializeCall),
        LiquidationStart(LiquidationStartCall),
        ManualAssert(ManualAssertCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        Migrate(MigrateCall),
        Nsubmissions(NsubmissionsCall),
        Owner(OwnerCall),
        PerpTick(PerpTickCall),
        ProcessSlowModeTransaction(ProcessSlowModeTransactionCall),
        Rebate(RebateCall),
        ReferralCodes(ReferralCodesCall),
        RegisterTransferableWallet(RegisterTransferableWalletCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequireSubaccount(RequireSubaccountCall),
        SequencerFee(SequencerFeeCall),
        SequencerFees(SequencerFeesCall),
        SetBook(SetBookCall),
        SetPriceX18(SetPriceX18Call),
        SetSequencer(SetSequencerCall),
        SetSlowModeConfig(SetSlowModeConfigCall),
        SetSlowModeTx(SetSlowModeTxCall),
        SettlePnl(SettlePnlCall),
        SignedBurnLp(SignedBurnLpCall),
        SignedCancellation(SignedCancellationCall),
        SignedCancellationProducts(SignedCancellationProductsCall),
        SignedLinkSigner(SignedLinkSignerCall),
        SignedLiquidateSubaccount(SignedLiquidateSubaccountCall),
        SignedMintLp(SignedMintLpCall),
        SignedOrder(SignedOrderCall),
        SignedWithdrawCollateral(SignedWithdrawCollateralCall),
        SlowModeConfig(SlowModeConfigCall),
        SlowModeTxs(SlowModeTxsCall),
        SpotTick(SpotTickCall),
        SubmitSlowModeTransaction(SubmitSlowModeTransactionCall),
        SubmitTransactions(SubmitTransactionsCall),
        SubmitTransactionsChecked(SubmitTransactionsCheckedCall),
        SubmitTransactionsCheckedWithGasLimit(SubmitTransactionsCheckedWithGasLimitCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        UnsignedBurnLp(UnsignedBurnLpCall),
        UnsignedDepositCollateral(UnsignedDepositCollateralCall),
        UnsignedDepositInsurance(UnsignedDepositInsuranceCall),
        UnsignedLinkSigner(UnsignedLinkSignerCall),
        UnsignedLiquidateSubaccount(UnsignedLiquidateSubaccountCall),
        UnsignedMintLp(UnsignedMintLpCall),
        UnsignedWithdrawCollateral(UnsignedWithdrawCollateralCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
    }
    impl ethers::core::abi::AbiDecode for EndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BurnLpAndTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::BurnLpAndTransfer(decoded));
            }
            if let Ok(decoded) =
                <CheckLpActionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::CheckLpAction(decoded));
            }
            if let Ok(decoded) =
                <CheckLpActionInnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::CheckLpActionInner(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxInnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::CheckSlowModeTxInner(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxLinkSignerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::CheckSlowModeTxLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <ClaimSequencerFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::ClaimSequencerFees(decoded));
            }
            if let Ok(decoded) =
                <ClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Clearinghouse(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::DepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralWithReferralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::DepositCollateralWithReferral(decoded));
            }
            if let Ok (decoded) = < DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (EndpointCalls :: DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount (decoded)) }
            if let Ok(decoded) =
                <ExecuteSlowModeTransactionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::ExecuteSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetBookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetBook(decoded));
            }
            if let Ok(decoded) =
                <GetHealthCheckFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetHealthCheckFee(decoded));
            }
            if let Ok(decoded) =
                <GetLinkedSignerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetLinkedSigner(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetLiquidationFee(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetNumSubaccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetNumSubaccounts(decoded));
            }
            if let Ok(decoded) =
                <GetPriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetPriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetPricesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetPricesX18(decoded));
            }
            if let Ok(decoded) =
                <GetSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetSequencer(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountByIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetSubaccountById(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetSubaccountId(decoded));
            }
            if let Ok(decoded) =
                <GetTakerSequencerFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetTakerSequencerFee(decoded));
            }
            if let Ok(decoded) =
                <GetTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetTime(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <IncrementSubmissionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::IncrementSubmissions(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LiquidationStartCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::LiquidationStart(decoded));
            }
            if let Ok(decoded) =
                <ManualAssertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::ManualAssert(decoded));
            }
            if let Ok(decoded) =
                <MatchOrderAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <NsubmissionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Nsubmissions(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerpTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::PerpTick(decoded));
            }
            if let Ok(decoded) =
                <ProcessSlowModeTransactionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::ProcessSlowModeTransaction(decoded));
            }
            if let Ok(decoded) = <RebateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::Rebate(decoded));
            }
            if let Ok(decoded) =
                <ReferralCodesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::ReferralCodes(decoded));
            }
            if let Ok(decoded) =
                <RegisterTransferableWalletCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::RegisterTransferableWallet(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequireSubaccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::RequireSubaccount(decoded));
            }
            if let Ok(decoded) =
                <SequencerFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SequencerFee(decoded));
            }
            if let Ok(decoded) =
                <SequencerFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SequencerFees(decoded));
            }
            if let Ok(decoded) =
                <SetBookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SetBook(decoded));
            }
            if let Ok(decoded) =
                <SetPriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SetPriceX18(decoded));
            }
            if let Ok(decoded) =
                <SetSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SetSequencer(decoded));
            }
            if let Ok(decoded) =
                <SetSlowModeConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SetSlowModeConfig(decoded));
            }
            if let Ok(decoded) =
                <SetSlowModeTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SetSlowModeTx(decoded));
            }
            if let Ok(decoded) =
                <SettlePnlCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SettlePnl(decoded));
            }
            if let Ok(decoded) =
                <SignedBurnLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SignedBurnLp(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SignedCancellation(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationProductsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SignedCancellationProducts(decoded));
            }
            if let Ok(decoded) =
                <SignedLinkSignerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <SignedLiquidateSubaccountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <SignedMintLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SignedMintLp(decoded));
            }
            if let Ok(decoded) =
                <SignedOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SignedOrder(decoded));
            }
            if let Ok(decoded) =
                <SignedWithdrawCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) =
                <SlowModeConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SlowModeConfig(decoded));
            }
            if let Ok(decoded) =
                <SlowModeTxsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SlowModeTxs(decoded));
            }
            if let Ok(decoded) =
                <SpotTickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SpotTick(decoded));
            }
            if let Ok(decoded) =
                <SubmitSlowModeTransactionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SubmitSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SubmitTransactions(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCheckedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SubmitTransactionsChecked(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCheckedWithGasLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::SubmitTransactionsCheckedWithGasLimit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SwapAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::SwapAMM(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnsignedBurnLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UnsignedBurnLp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::UnsignedDepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositInsuranceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::UnsignedDepositInsurance(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLinkSignerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UnsignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLiquidateSubaccountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::UnsignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <UnsignedMintLpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UnsignedMintLp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedWithdrawCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EndpointCalls::UnsignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) =
                <UpdatePriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UpdatePrice(decoded));
            }
            if let Ok(decoded) =
                <UpdateProductCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EndpointCalls::UpdateProduct(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for EndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self { EndpointCalls :: BurnLpAndTransfer (element) => element . encode () , EndpointCalls :: CheckLpAction (element) => element . encode () , EndpointCalls :: CheckLpActionInner (element) => element . encode () , EndpointCalls :: CheckSlowModeTxInner (element) => element . encode () , EndpointCalls :: CheckSlowModeTxLinkSigner (element) => element . encode () , EndpointCalls :: ClaimSequencerFees (element) => element . encode () , EndpointCalls :: Clearinghouse (element) => element . encode () , EndpointCalls :: DepositCollateral (element) => element . encode () , EndpointCalls :: DepositCollateralWithReferral (element) => element . encode () , EndpointCalls :: DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount (element) => element . encode () , EndpointCalls :: ExecuteSlowModeTransaction (element) => element . encode () , EndpointCalls :: GetBook (element) => element . encode () , EndpointCalls :: GetHealthCheckFee (element) => element . encode () , EndpointCalls :: GetLinkedSigner (element) => element . encode () , EndpointCalls :: GetLiquidationFee (element) => element . encode () , EndpointCalls :: GetNonce (element) => element . encode () , EndpointCalls :: GetNumSubaccounts (element) => element . encode () , EndpointCalls :: GetPriceX18 (element) => element . encode () , EndpointCalls :: GetPricesX18 (element) => element . encode () , EndpointCalls :: GetSequencer (element) => element . encode () , EndpointCalls :: GetSubaccountById (element) => element . encode () , EndpointCalls :: GetSubaccountId (element) => element . encode () , EndpointCalls :: GetTakerSequencerFee (element) => element . encode () , EndpointCalls :: GetTime (element) => element . encode () , EndpointCalls :: GetVersion (element) => element . encode () , EndpointCalls :: IncrementSubmissions (element) => element . encode () , EndpointCalls :: Initialize (element) => element . encode () , EndpointCalls :: LiquidationStart (element) => element . encode () , EndpointCalls :: ManualAssert (element) => element . encode () , EndpointCalls :: MatchOrderAMM (element) => element . encode () , EndpointCalls :: MatchOrders (element) => element . encode () , EndpointCalls :: Migrate (element) => element . encode () , EndpointCalls :: Nsubmissions (element) => element . encode () , EndpointCalls :: Owner (element) => element . encode () , EndpointCalls :: PerpTick (element) => element . encode () , EndpointCalls :: ProcessSlowModeTransaction (element) => element . encode () , EndpointCalls :: Rebate (element) => element . encode () , EndpointCalls :: ReferralCodes (element) => element . encode () , EndpointCalls :: RegisterTransferableWallet (element) => element . encode () , EndpointCalls :: RenounceOwnership (element) => element . encode () , EndpointCalls :: RequireSubaccount (element) => element . encode () , EndpointCalls :: SequencerFee (element) => element . encode () , EndpointCalls :: SequencerFees (element) => element . encode () , EndpointCalls :: SetBook (element) => element . encode () , EndpointCalls :: SetPriceX18 (element) => element . encode () , EndpointCalls :: SetSequencer (element) => element . encode () , EndpointCalls :: SetSlowModeConfig (element) => element . encode () , EndpointCalls :: SetSlowModeTx (element) => element . encode () , EndpointCalls :: SettlePnl (element) => element . encode () , EndpointCalls :: SignedBurnLp (element) => element . encode () , EndpointCalls :: SignedCancellation (element) => element . encode () , EndpointCalls :: SignedCancellationProducts (element) => element . encode () , EndpointCalls :: SignedLinkSigner (element) => element . encode () , EndpointCalls :: SignedLiquidateSubaccount (element) => element . encode () , EndpointCalls :: SignedMintLp (element) => element . encode () , EndpointCalls :: SignedOrder (element) => element . encode () , EndpointCalls :: SignedWithdrawCollateral (element) => element . encode () , EndpointCalls :: SlowModeConfig (element) => element . encode () , EndpointCalls :: SlowModeTxs (element) => element . encode () , EndpointCalls :: SpotTick (element) => element . encode () , EndpointCalls :: SubmitSlowModeTransaction (element) => element . encode () , EndpointCalls :: SubmitTransactions (element) => element . encode () , EndpointCalls :: SubmitTransactionsChecked (element) => element . encode () , EndpointCalls :: SubmitTransactionsCheckedWithGasLimit (element) => element . encode () , EndpointCalls :: SwapAMM (element) => element . encode () , EndpointCalls :: TransferOwnership (element) => element . encode () , EndpointCalls :: UnsignedBurnLp (element) => element . encode () , EndpointCalls :: UnsignedDepositCollateral (element) => element . encode () , EndpointCalls :: UnsignedDepositInsurance (element) => element . encode () , EndpointCalls :: UnsignedLinkSigner (element) => element . encode () , EndpointCalls :: UnsignedLiquidateSubaccount (element) => element . encode () , EndpointCalls :: UnsignedMintLp (element) => element . encode () , EndpointCalls :: UnsignedWithdrawCollateral (element) => element . encode () , EndpointCalls :: UpdateFeeRates (element) => element . encode () , EndpointCalls :: UpdatePrice (element) => element . encode () , EndpointCalls :: UpdateProduct (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for EndpointCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { EndpointCalls :: BurnLpAndTransfer (element) => element . fmt (f) , EndpointCalls :: CheckLpAction (element) => element . fmt (f) , EndpointCalls :: CheckLpActionInner (element) => element . fmt (f) , EndpointCalls :: CheckSlowModeTxInner (element) => element . fmt (f) , EndpointCalls :: CheckSlowModeTxLinkSigner (element) => element . fmt (f) , EndpointCalls :: ClaimSequencerFees (element) => element . fmt (f) , EndpointCalls :: Clearinghouse (element) => element . fmt (f) , EndpointCalls :: DepositCollateral (element) => element . fmt (f) , EndpointCalls :: DepositCollateralWithReferral (element) => element . fmt (f) , EndpointCalls :: DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount (element) => element . fmt (f) , EndpointCalls :: ExecuteSlowModeTransaction (element) => element . fmt (f) , EndpointCalls :: GetBook (element) => element . fmt (f) , EndpointCalls :: GetHealthCheckFee (element) => element . fmt (f) , EndpointCalls :: GetLinkedSigner (element) => element . fmt (f) , EndpointCalls :: GetLiquidationFee (element) => element . fmt (f) , EndpointCalls :: GetNonce (element) => element . fmt (f) , EndpointCalls :: GetNumSubaccounts (element) => element . fmt (f) , EndpointCalls :: GetPriceX18 (element) => element . fmt (f) , EndpointCalls :: GetPricesX18 (element) => element . fmt (f) , EndpointCalls :: GetSequencer (element) => element . fmt (f) , EndpointCalls :: GetSubaccountById (element) => element . fmt (f) , EndpointCalls :: GetSubaccountId (element) => element . fmt (f) , EndpointCalls :: GetTakerSequencerFee (element) => element . fmt (f) , EndpointCalls :: GetTime (element) => element . fmt (f) , EndpointCalls :: GetVersion (element) => element . fmt (f) , EndpointCalls :: IncrementSubmissions (element) => element . fmt (f) , EndpointCalls :: Initialize (element) => element . fmt (f) , EndpointCalls :: LiquidationStart (element) => element . fmt (f) , EndpointCalls :: ManualAssert (element) => element . fmt (f) , EndpointCalls :: MatchOrderAMM (element) => element . fmt (f) , EndpointCalls :: MatchOrders (element) => element . fmt (f) , EndpointCalls :: Migrate (element) => element . fmt (f) , EndpointCalls :: Nsubmissions (element) => element . fmt (f) , EndpointCalls :: Owner (element) => element . fmt (f) , EndpointCalls :: PerpTick (element) => element . fmt (f) , EndpointCalls :: ProcessSlowModeTransaction (element) => element . fmt (f) , EndpointCalls :: Rebate (element) => element . fmt (f) , EndpointCalls :: ReferralCodes (element) => element . fmt (f) , EndpointCalls :: RegisterTransferableWallet (element) => element . fmt (f) , EndpointCalls :: RenounceOwnership (element) => element . fmt (f) , EndpointCalls :: RequireSubaccount (element) => element . fmt (f) , EndpointCalls :: SequencerFee (element) => element . fmt (f) , EndpointCalls :: SequencerFees (element) => element . fmt (f) , EndpointCalls :: SetBook (element) => element . fmt (f) , EndpointCalls :: SetPriceX18 (element) => element . fmt (f) , EndpointCalls :: SetSequencer (element) => element . fmt (f) , EndpointCalls :: SetSlowModeConfig (element) => element . fmt (f) , EndpointCalls :: SetSlowModeTx (element) => element . fmt (f) , EndpointCalls :: SettlePnl (element) => element . fmt (f) , EndpointCalls :: SignedBurnLp (element) => element . fmt (f) , EndpointCalls :: SignedCancellation (element) => element . fmt (f) , EndpointCalls :: SignedCancellationProducts (element) => element . fmt (f) , EndpointCalls :: SignedLinkSigner (element) => element . fmt (f) , EndpointCalls :: SignedLiquidateSubaccount (element) => element . fmt (f) , EndpointCalls :: SignedMintLp (element) => element . fmt (f) , EndpointCalls :: SignedOrder (element) => element . fmt (f) , EndpointCalls :: SignedWithdrawCollateral (element) => element . fmt (f) , EndpointCalls :: SlowModeConfig (element) => element . fmt (f) , EndpointCalls :: SlowModeTxs (element) => element . fmt (f) , EndpointCalls :: SpotTick (element) => element . fmt (f) , EndpointCalls :: SubmitSlowModeTransaction (element) => element . fmt (f) , EndpointCalls :: SubmitTransactions (element) => element . fmt (f) , EndpointCalls :: SubmitTransactionsChecked (element) => element . fmt (f) , EndpointCalls :: SubmitTransactionsCheckedWithGasLimit (element) => element . fmt (f) , EndpointCalls :: SwapAMM (element) => element . fmt (f) , EndpointCalls :: TransferOwnership (element) => element . fmt (f) , EndpointCalls :: UnsignedBurnLp (element) => element . fmt (f) , EndpointCalls :: UnsignedDepositCollateral (element) => element . fmt (f) , EndpointCalls :: UnsignedDepositInsurance (element) => element . fmt (f) , EndpointCalls :: UnsignedLinkSigner (element) => element . fmt (f) , EndpointCalls :: UnsignedLiquidateSubaccount (element) => element . fmt (f) , EndpointCalls :: UnsignedMintLp (element) => element . fmt (f) , EndpointCalls :: UnsignedWithdrawCollateral (element) => element . fmt (f) , EndpointCalls :: UpdateFeeRates (element) => element . fmt (f) , EndpointCalls :: UpdatePrice (element) => element . fmt (f) , EndpointCalls :: UpdateProduct (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<BurnLpAndTransferCall> for EndpointCalls {
        fn from(var: BurnLpAndTransferCall) -> Self {
            EndpointCalls::BurnLpAndTransfer(var)
        }
    }
    impl ::std::convert::From<CheckLpActionCall> for EndpointCalls {
        fn from(var: CheckLpActionCall) -> Self {
            EndpointCalls::CheckLpAction(var)
        }
    }
    impl ::std::convert::From<CheckLpActionInnerCall> for EndpointCalls {
        fn from(var: CheckLpActionInnerCall) -> Self {
            EndpointCalls::CheckLpActionInner(var)
        }
    }
    impl ::std::convert::From<CheckSlowModeTxInnerCall> for EndpointCalls {
        fn from(var: CheckSlowModeTxInnerCall) -> Self {
            EndpointCalls::CheckSlowModeTxInner(var)
        }
    }
    impl ::std::convert::From<CheckSlowModeTxLinkSignerCall> for EndpointCalls {
        fn from(var: CheckSlowModeTxLinkSignerCall) -> Self {
            EndpointCalls::CheckSlowModeTxLinkSigner(var)
        }
    }
    impl ::std::convert::From<ClaimSequencerFeesCall> for EndpointCalls {
        fn from(var: ClaimSequencerFeesCall) -> Self {
            EndpointCalls::ClaimSequencerFees(var)
        }
    }
    impl ::std::convert::From<ClearinghouseCall> for EndpointCalls {
        fn from(var: ClearinghouseCall) -> Self {
            EndpointCalls::Clearinghouse(var)
        }
    }
    impl ::std::convert::From<DepositCollateralCall> for EndpointCalls {
        fn from(var: DepositCollateralCall) -> Self {
            EndpointCalls::DepositCollateral(var)
        }
    }
    impl ::std::convert::From<DepositCollateralWithReferralCall> for EndpointCalls {
        fn from(var: DepositCollateralWithReferralCall) -> Self {
            EndpointCalls::DepositCollateralWithReferral(var)
        }
    }
    impl
        ::std::convert::From<
            DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall,
        > for EndpointCalls
    {
        fn from(
            var: DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall,
        ) -> Self {
            EndpointCalls::DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(var)
        }
    }
    impl ::std::convert::From<ExecuteSlowModeTransactionCall> for EndpointCalls {
        fn from(var: ExecuteSlowModeTransactionCall) -> Self {
            EndpointCalls::ExecuteSlowModeTransaction(var)
        }
    }
    impl ::std::convert::From<GetBookCall> for EndpointCalls {
        fn from(var: GetBookCall) -> Self {
            EndpointCalls::GetBook(var)
        }
    }
    impl ::std::convert::From<GetHealthCheckFeeCall> for EndpointCalls {
        fn from(var: GetHealthCheckFeeCall) -> Self {
            EndpointCalls::GetHealthCheckFee(var)
        }
    }
    impl ::std::convert::From<GetLinkedSignerCall> for EndpointCalls {
        fn from(var: GetLinkedSignerCall) -> Self {
            EndpointCalls::GetLinkedSigner(var)
        }
    }
    impl ::std::convert::From<GetLiquidationFeeCall> for EndpointCalls {
        fn from(var: GetLiquidationFeeCall) -> Self {
            EndpointCalls::GetLiquidationFee(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for EndpointCalls {
        fn from(var: GetNonceCall) -> Self {
            EndpointCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<GetNumSubaccountsCall> for EndpointCalls {
        fn from(var: GetNumSubaccountsCall) -> Self {
            EndpointCalls::GetNumSubaccounts(var)
        }
    }
    impl ::std::convert::From<GetPriceX18Call> for EndpointCalls {
        fn from(var: GetPriceX18Call) -> Self {
            EndpointCalls::GetPriceX18(var)
        }
    }
    impl ::std::convert::From<GetPricesX18Call> for EndpointCalls {
        fn from(var: GetPricesX18Call) -> Self {
            EndpointCalls::GetPricesX18(var)
        }
    }
    impl ::std::convert::From<GetSequencerCall> for EndpointCalls {
        fn from(var: GetSequencerCall) -> Self {
            EndpointCalls::GetSequencer(var)
        }
    }
    impl ::std::convert::From<GetSubaccountByIdCall> for EndpointCalls {
        fn from(var: GetSubaccountByIdCall) -> Self {
            EndpointCalls::GetSubaccountById(var)
        }
    }
    impl ::std::convert::From<GetSubaccountIdCall> for EndpointCalls {
        fn from(var: GetSubaccountIdCall) -> Self {
            EndpointCalls::GetSubaccountId(var)
        }
    }
    impl ::std::convert::From<GetTakerSequencerFeeCall> for EndpointCalls {
        fn from(var: GetTakerSequencerFeeCall) -> Self {
            EndpointCalls::GetTakerSequencerFee(var)
        }
    }
    impl ::std::convert::From<GetTimeCall> for EndpointCalls {
        fn from(var: GetTimeCall) -> Self {
            EndpointCalls::GetTime(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for EndpointCalls {
        fn from(var: GetVersionCall) -> Self {
            EndpointCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<IncrementSubmissionsCall> for EndpointCalls {
        fn from(var: IncrementSubmissionsCall) -> Self {
            EndpointCalls::IncrementSubmissions(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for EndpointCalls {
        fn from(var: InitializeCall) -> Self {
            EndpointCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LiquidationStartCall> for EndpointCalls {
        fn from(var: LiquidationStartCall) -> Self {
            EndpointCalls::LiquidationStart(var)
        }
    }
    impl ::std::convert::From<ManualAssertCall> for EndpointCalls {
        fn from(var: ManualAssertCall) -> Self {
            EndpointCalls::ManualAssert(var)
        }
    }
    impl ::std::convert::From<MatchOrderAMMCall> for EndpointCalls {
        fn from(var: MatchOrderAMMCall) -> Self {
            EndpointCalls::MatchOrderAMM(var)
        }
    }
    impl ::std::convert::From<MatchOrdersCall> for EndpointCalls {
        fn from(var: MatchOrdersCall) -> Self {
            EndpointCalls::MatchOrders(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for EndpointCalls {
        fn from(var: MigrateCall) -> Self {
            EndpointCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<NsubmissionsCall> for EndpointCalls {
        fn from(var: NsubmissionsCall) -> Self {
            EndpointCalls::Nsubmissions(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for EndpointCalls {
        fn from(var: OwnerCall) -> Self {
            EndpointCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PerpTickCall> for EndpointCalls {
        fn from(var: PerpTickCall) -> Self {
            EndpointCalls::PerpTick(var)
        }
    }
    impl ::std::convert::From<ProcessSlowModeTransactionCall> for EndpointCalls {
        fn from(var: ProcessSlowModeTransactionCall) -> Self {
            EndpointCalls::ProcessSlowModeTransaction(var)
        }
    }
    impl ::std::convert::From<RebateCall> for EndpointCalls {
        fn from(var: RebateCall) -> Self {
            EndpointCalls::Rebate(var)
        }
    }
    impl ::std::convert::From<ReferralCodesCall> for EndpointCalls {
        fn from(var: ReferralCodesCall) -> Self {
            EndpointCalls::ReferralCodes(var)
        }
    }
    impl ::std::convert::From<RegisterTransferableWalletCall> for EndpointCalls {
        fn from(var: RegisterTransferableWalletCall) -> Self {
            EndpointCalls::RegisterTransferableWallet(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for EndpointCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            EndpointCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RequireSubaccountCall> for EndpointCalls {
        fn from(var: RequireSubaccountCall) -> Self {
            EndpointCalls::RequireSubaccount(var)
        }
    }
    impl ::std::convert::From<SequencerFeeCall> for EndpointCalls {
        fn from(var: SequencerFeeCall) -> Self {
            EndpointCalls::SequencerFee(var)
        }
    }
    impl ::std::convert::From<SequencerFeesCall> for EndpointCalls {
        fn from(var: SequencerFeesCall) -> Self {
            EndpointCalls::SequencerFees(var)
        }
    }
    impl ::std::convert::From<SetBookCall> for EndpointCalls {
        fn from(var: SetBookCall) -> Self {
            EndpointCalls::SetBook(var)
        }
    }
    impl ::std::convert::From<SetPriceX18Call> for EndpointCalls {
        fn from(var: SetPriceX18Call) -> Self {
            EndpointCalls::SetPriceX18(var)
        }
    }
    impl ::std::convert::From<SetSequencerCall> for EndpointCalls {
        fn from(var: SetSequencerCall) -> Self {
            EndpointCalls::SetSequencer(var)
        }
    }
    impl ::std::convert::From<SetSlowModeConfigCall> for EndpointCalls {
        fn from(var: SetSlowModeConfigCall) -> Self {
            EndpointCalls::SetSlowModeConfig(var)
        }
    }
    impl ::std::convert::From<SetSlowModeTxCall> for EndpointCalls {
        fn from(var: SetSlowModeTxCall) -> Self {
            EndpointCalls::SetSlowModeTx(var)
        }
    }
    impl ::std::convert::From<SettlePnlCall> for EndpointCalls {
        fn from(var: SettlePnlCall) -> Self {
            EndpointCalls::SettlePnl(var)
        }
    }
    impl ::std::convert::From<SignedBurnLpCall> for EndpointCalls {
        fn from(var: SignedBurnLpCall) -> Self {
            EndpointCalls::SignedBurnLp(var)
        }
    }
    impl ::std::convert::From<SignedCancellationCall> for EndpointCalls {
        fn from(var: SignedCancellationCall) -> Self {
            EndpointCalls::SignedCancellation(var)
        }
    }
    impl ::std::convert::From<SignedCancellationProductsCall> for EndpointCalls {
        fn from(var: SignedCancellationProductsCall) -> Self {
            EndpointCalls::SignedCancellationProducts(var)
        }
    }
    impl ::std::convert::From<SignedLinkSignerCall> for EndpointCalls {
        fn from(var: SignedLinkSignerCall) -> Self {
            EndpointCalls::SignedLinkSigner(var)
        }
    }
    impl ::std::convert::From<SignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(var: SignedLiquidateSubaccountCall) -> Self {
            EndpointCalls::SignedLiquidateSubaccount(var)
        }
    }
    impl ::std::convert::From<SignedMintLpCall> for EndpointCalls {
        fn from(var: SignedMintLpCall) -> Self {
            EndpointCalls::SignedMintLp(var)
        }
    }
    impl ::std::convert::From<SignedOrderCall> for EndpointCalls {
        fn from(var: SignedOrderCall) -> Self {
            EndpointCalls::SignedOrder(var)
        }
    }
    impl ::std::convert::From<SignedWithdrawCollateralCall> for EndpointCalls {
        fn from(var: SignedWithdrawCollateralCall) -> Self {
            EndpointCalls::SignedWithdrawCollateral(var)
        }
    }
    impl ::std::convert::From<SlowModeConfigCall> for EndpointCalls {
        fn from(var: SlowModeConfigCall) -> Self {
            EndpointCalls::SlowModeConfig(var)
        }
    }
    impl ::std::convert::From<SlowModeTxsCall> for EndpointCalls {
        fn from(var: SlowModeTxsCall) -> Self {
            EndpointCalls::SlowModeTxs(var)
        }
    }
    impl ::std::convert::From<SpotTickCall> for EndpointCalls {
        fn from(var: SpotTickCall) -> Self {
            EndpointCalls::SpotTick(var)
        }
    }
    impl ::std::convert::From<SubmitSlowModeTransactionCall> for EndpointCalls {
        fn from(var: SubmitSlowModeTransactionCall) -> Self {
            EndpointCalls::SubmitSlowModeTransaction(var)
        }
    }
    impl ::std::convert::From<SubmitTransactionsCall> for EndpointCalls {
        fn from(var: SubmitTransactionsCall) -> Self {
            EndpointCalls::SubmitTransactions(var)
        }
    }
    impl ::std::convert::From<SubmitTransactionsCheckedCall> for EndpointCalls {
        fn from(var: SubmitTransactionsCheckedCall) -> Self {
            EndpointCalls::SubmitTransactionsChecked(var)
        }
    }
    impl ::std::convert::From<SubmitTransactionsCheckedWithGasLimitCall> for EndpointCalls {
        fn from(var: SubmitTransactionsCheckedWithGasLimitCall) -> Self {
            EndpointCalls::SubmitTransactionsCheckedWithGasLimit(var)
        }
    }
    impl ::std::convert::From<SwapAMMCall> for EndpointCalls {
        fn from(var: SwapAMMCall) -> Self {
            EndpointCalls::SwapAMM(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for EndpointCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            EndpointCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnsignedBurnLpCall> for EndpointCalls {
        fn from(var: UnsignedBurnLpCall) -> Self {
            EndpointCalls::UnsignedBurnLp(var)
        }
    }
    impl ::std::convert::From<UnsignedDepositCollateralCall> for EndpointCalls {
        fn from(var: UnsignedDepositCollateralCall) -> Self {
            EndpointCalls::UnsignedDepositCollateral(var)
        }
    }
    impl ::std::convert::From<UnsignedDepositInsuranceCall> for EndpointCalls {
        fn from(var: UnsignedDepositInsuranceCall) -> Self {
            EndpointCalls::UnsignedDepositInsurance(var)
        }
    }
    impl ::std::convert::From<UnsignedLinkSignerCall> for EndpointCalls {
        fn from(var: UnsignedLinkSignerCall) -> Self {
            EndpointCalls::UnsignedLinkSigner(var)
        }
    }
    impl ::std::convert::From<UnsignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(var: UnsignedLiquidateSubaccountCall) -> Self {
            EndpointCalls::UnsignedLiquidateSubaccount(var)
        }
    }
    impl ::std::convert::From<UnsignedMintLpCall> for EndpointCalls {
        fn from(var: UnsignedMintLpCall) -> Self {
            EndpointCalls::UnsignedMintLp(var)
        }
    }
    impl ::std::convert::From<UnsignedWithdrawCollateralCall> for EndpointCalls {
        fn from(var: UnsignedWithdrawCollateralCall) -> Self {
            EndpointCalls::UnsignedWithdrawCollateral(var)
        }
    }
    impl ::std::convert::From<UpdateFeeRatesCall> for EndpointCalls {
        fn from(var: UpdateFeeRatesCall) -> Self {
            EndpointCalls::UpdateFeeRates(var)
        }
    }
    impl ::std::convert::From<UpdatePriceCall> for EndpointCalls {
        fn from(var: UpdatePriceCall) -> Self {
            EndpointCalls::UpdatePrice(var)
        }
    }
    impl ::std::convert::From<UpdateProductCall> for EndpointCalls {
        fn from(var: UpdateProductCall) -> Self {
            EndpointCalls::UpdateProduct(var)
        }
    }
    #[doc = "Container type for all return fields from the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `[7, 72, 162, 25]`"]
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
    pub struct BurnLpAndTransferReturn(pub BurnLpAndTransfer);
    #[doc = "Container type for all return fields from the `checkLpAction` function with signature `checkLpAction()` and selector `[140, 61, 47, 116]`"]
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
    pub struct CheckLpActionReturn(pub u32);
    #[doc = "Container type for all return fields from the `checkLpActionInner` function with signature `checkLpActionInner(address,bytes)` and selector `[195, 69, 83, 11]`"]
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
    pub struct CheckLpActionInnerReturn(pub u32);
    #[doc = "Container type for all return fields from the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `[183, 14, 178, 99]`"]
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
    pub struct CheckSlowModeTxInnerReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `[91, 180, 193, 38]`"]
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
    pub struct CheckSlowModeTxLinkSignerReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `claimSequencerFees` function with signature `claimSequencerFees((bytes32))` and selector `[56, 66, 231, 94]`"]
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
    pub struct ClaimSequencerFeesReturn(pub ClaimSequencerFees);
    #[doc = "Container type for all return fields from the `clearinghouse` function with signature `clearinghouse()` and selector `[93, 79, 95, 151]`"]
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
    pub struct ClearinghouseReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getBook` function with signature `getBook(uint32)` and selector `[207, 152, 124, 155]`"]
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
    pub struct GetBookReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `[212, 222, 141, 157]`"]
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
    pub struct GetHealthCheckFeeReturn(pub i128);
    #[doc = "Container type for all return fields from the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `[145, 193, 227, 215]`"]
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
    pub struct GetLinkedSignerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `[251, 244, 25, 132]`"]
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
    pub struct GetLiquidationFeeReturn(pub i128);
    #[doc = "Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `[45, 3, 53, 171]`"]
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
    pub struct GetNonceReturn(pub u64);
    #[doc = "Container type for all return fields from the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `[196, 249, 178, 95]`"]
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
    pub struct GetNumSubaccountsReturn(pub u64);
    #[doc = "Container type for all return fields from the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `[54, 142, 70, 134]`"]
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
    pub struct GetPriceX18Return {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
    #[doc = "Container type for all return fields from the `getPricesX18` function with signature `getPricesX18(uint32)` and selector `[175, 165, 93, 238]`"]
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
    pub struct GetPricesX18Return(pub Prices);
    #[doc = "Container type for all return fields from the `getSequencer` function with signature `getSequencer()` and selector `[77, 150, 169, 10]`"]
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
    pub struct GetSequencerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `[239, 100, 237, 14]`"]
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
    pub struct GetSubaccountByIdReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `[34, 212, 168, 45]`"]
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
    pub struct GetSubaccountIdReturn(pub u64);
    #[doc = "Container type for all return fields from the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `[197, 16, 53, 159]`"]
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
    pub struct GetTakerSequencerFeeReturn(pub i128);
    #[doc = "Container type for all return fields from the `getTime` function with signature `getTime()` and selector `[85, 126, 209, 186]`"]
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
    pub struct GetTimeReturn(pub u128);
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
    #[doc = "Container type for all return fields from the `manualAssert` function with signature `manualAssert((int128[],int128[],int128[]))` and selector `[44, 140, 111, 251]`"]
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
    pub struct ManualAssertReturn(pub ManualAssert);
    #[doc = "Container type for all return fields from the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `[54, 185, 12, 81]`"]
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
    pub struct MatchOrderAMMReturn(pub MatchOrderAMM);
    #[doc = "Container type for all return fields from the `matchOrders` function with signature `matchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `[69, 22, 14, 82]`"]
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
    pub struct MatchOrdersReturn(pub MatchOrders);
    #[doc = "Container type for all return fields from the `nSubmissions` function with signature `nSubmissions()` and selector `[24, 237, 22, 235]`"]
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
    pub struct NsubmissionsReturn(pub u64);
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
    #[doc = "Container type for all return fields from the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `[90, 0, 146, 59]`"]
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
    pub struct PerpTickReturn(pub PerpTick);
    #[doc = "Container type for all return fields from the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `[66, 199, 77, 29]`"]
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
    pub struct RebateReturn(pub Rebate);
    #[doc = "Container type for all return fields from the `referralCodes` function with signature `referralCodes(address)` and selector `[149, 52, 221, 62]`"]
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
    pub struct ReferralCodesReturn(pub String);
    #[doc = "Container type for all return fields from the `sequencerFee` function with signature `sequencerFee(uint32)` and selector `[174, 58, 134, 82]`"]
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
    pub struct SequencerFeeReturn(pub i128);
    #[doc = "Container type for all return fields from the `sequencerFees` function with signature `sequencerFees()` and selector `[13, 58, 208, 57]`"]
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
    pub struct SequencerFeesReturn(pub i128);
    #[doc = "Container type for all return fields from the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `[178, 187, 99, 103]`"]
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
    pub struct SettlePnlReturn(pub SettlePnl);
    #[doc = "Container type for all return fields from the `signedBurnLp` function with signature `signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))` and selector `[97, 11, 46, 94]`"]
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
    pub struct SignedBurnLpReturn(pub SignedBurnLp);
    #[doc = "Container type for all return fields from the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `[62, 223, 44, 91]`"]
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
    pub struct SignedCancellationReturn(pub SignedCancellation);
    #[doc = "Container type for all return fields from the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `[160, 130, 197, 170]`"]
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
    pub struct SignedCancellationProductsReturn(pub SignedCancellationProducts);
    #[doc = "Container type for all return fields from the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `[133, 200, 62, 157]`"]
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
    pub struct SignedLinkSignerReturn(pub SignedLinkSigner);
    #[doc = "Container type for all return fields from the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))` and selector `[191, 131, 46, 119]`"]
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
    pub struct SignedLiquidateSubaccountReturn(pub SignedLiquidateSubaccount);
    #[doc = "Container type for all return fields from the `signedMintLp` function with signature `signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))` and selector `[211, 140, 59, 156]`"]
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
    pub struct SignedMintLpReturn(pub SignedMintLp);
    #[doc = "Container type for all return fields from the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `[108, 69, 117, 112]`"]
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
    pub struct SignedOrderReturn(pub SignedOrder);
    #[doc = "Container type for all return fields from the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `[13, 85, 226, 107]`"]
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
    pub struct SignedWithdrawCollateralReturn(pub SignedWithdrawCollateral);
    #[doc = "Container type for all return fields from the `slowModeConfig` function with signature `slowModeConfig()` and selector `[184, 4, 62, 222]`"]
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
    pub struct SlowModeConfigReturn {
        pub timeout: u64,
        pub tx_count: u64,
        pub tx_up_to: u64,
    }
    #[doc = "Container type for all return fields from the `slowModeTxs` function with signature `slowModeTxs(uint64)` and selector `[207, 143, 219, 9]`"]
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
    pub struct SlowModeTxsReturn {
        pub executable_at: u64,
        pub sender: ethers::core::types::Address,
        pub tx: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `spotTick` function with signature `spotTick((uint128))` and selector `[217, 118, 134, 149]`"]
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
    pub struct SpotTickReturn(pub SpotTick);
    #[doc = "Container type for all return fields from the `submitTransactionsCheckedWithGasLimit` function with signature `submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)` and selector `[47, 154, 39, 68]`"]
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
    pub struct SubmitTransactionsCheckedWithGasLimitReturn(pub u64, pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `[15, 75, 80, 157]`"]
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
    pub struct SwapAMMReturn(pub SwapAMM);
    #[doc = "Container type for all return fields from the `unsignedBurnLp` function with signature `unsignedBurnLp((bytes32,uint32,uint128,uint64))` and selector `[6, 192, 186, 253]`"]
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
    pub struct UnsignedBurnLpReturn(pub BurnLp);
    #[doc = "Container type for all return fields from the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `[60, 236, 75, 147]`"]
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
    pub struct UnsignedDepositCollateralReturn(pub DepositCollateral);
    #[doc = "Container type for all return fields from the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `[148, 250, 239, 229]`"]
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
    pub struct UnsignedDepositInsuranceReturn(pub DepositInsurance);
    #[doc = "Container type for all return fields from the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `[5, 228, 45, 199]`"]
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
    pub struct UnsignedLinkSignerReturn(pub LinkSigner);
    #[doc = "Container type for all return fields from the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `[249, 228, 115, 252]`"]
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
    pub struct UnsignedLiquidateSubaccountReturn(pub LiquidateSubaccount);
    #[doc = "Container type for all return fields from the `unsignedMintLp` function with signature `unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `[145, 14, 96, 106]`"]
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
    pub struct UnsignedMintLpReturn(pub MintLp);
    #[doc = "Container type for all return fields from the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `[188, 133, 202, 134]`"]
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
    pub struct UnsignedWithdrawCollateralReturn(pub WithdrawCollateral);
    #[doc = "Container type for all return fields from the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `[53, 99, 154, 79]`"]
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
    pub struct UpdateFeeRatesReturn(pub UpdateFeeRates);
    #[doc = "Container type for all return fields from the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `[29, 158, 237, 165]`"]
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
    pub struct UpdatePriceReturn(pub UpdatePrice);
    #[doc = "Container type for all return fields from the `updateProduct` function with signature `updateProduct((address,bytes))` and selector `[44, 215, 27, 22]`"]
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
    pub struct UpdateProductReturn(pub UpdateProduct);
    #[doc = "`Cancellation(bytes32,uint32[],bytes32[],uint64)`"]
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
    pub struct Cancellation {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: Vec<u32>,
        pub digests: Vec<[u8; 32]>,
        pub nonce: u64,
    }
    #[doc = "`CancellationProducts(bytes32,uint32[],uint64)`"]
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
    pub struct CancellationProducts {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: Vec<u32>,
        pub nonce: u64,
    }
    #[doc = "`SignedCancellation((bytes32,uint32[],bytes32[],uint64),bytes)`"]
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
    pub struct SignedCancellation {
        pub cancellation: Cancellation,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedCancellationProducts((bytes32,uint32[],uint64),bytes)`"]
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
    pub struct SignedCancellationProducts {
        pub cancellation_products: CancellationProducts,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`BurnLp(bytes32,uint32,uint128,uint64)`"]
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
    pub struct BurnLp {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
    #[doc = "`BurnLpAndTransfer(bytes32,uint32,uint128,bytes32)`"]
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
    pub struct BurnLpAndTransfer {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub recipient: [u8; 32],
    }
    #[doc = "`ClaimSequencerFees(bytes32)`"]
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
    pub struct ClaimSequencerFees {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[doc = "`DepositCollateral(bytes32,uint32,uint128)`"]
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
    pub struct DepositCollateral {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
    }
    #[doc = "`DepositInsurance(uint128)`"]
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
    pub struct DepositInsurance {
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
    }
    #[doc = "`LinkSigner(bytes32,bytes32,uint64)`"]
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
    pub struct LinkSigner {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub signer: [u8; 32],
        pub nonce: u64,
    }
    #[doc = "`LiquidateSubaccount(bytes32,bytes32,uint8,uint32,int128,uint64)`"]
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
    pub struct LiquidateSubaccount {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        pub mode: u8,
        pub health_group: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        pub nonce: u64,
    }
    #[doc = "`ManualAssert(int128[],int128[],int128[])`"]
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
    pub struct ManualAssert {
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub open_interests: Vec<i128>,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub total_deposits: Vec<i128>,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub total_borrows: Vec<i128>,
    }
    #[doc = "`MatchOrderAMM(uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes))`"]
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
    pub struct MatchOrderAMM {
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
        pub taker: SignedOrder,
    }
    #[doc = "`MatchOrders(uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`"]
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
    pub struct MatchOrders {
        pub product_id: u32,
        pub amm: bool,
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    #[doc = "`MintLp(bytes32,uint32,uint128,uint128,uint128,uint64)`"]
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
    pub struct MintLp {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount_base: u128,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub quote_amount_low: u128,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub quote_amount_high: u128,
        pub nonce: u64,
    }
    #[doc = "`Order(bytes32,int128,int128,uint64,uint64)`"]
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
    pub struct Order {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
    }
    #[doc = "`PerpTick(uint128,int128[])`"]
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
    pub struct PerpTick {
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub time: u128,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub avg_price_diffs: Vec<i128>,
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
    #[doc = "`Rebate(bytes32[],int128[])`"]
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
    pub struct Rebate {
        pub subaccounts: Vec<[u8; 32]>,
        #[serde(
            serialize_with = "serialize_vec_i128",
            deserialize_with = "deserialize_vec_i128"
        )]
        pub amounts: Vec<i128>,
    }
    #[doc = "`SettlePnl(bytes32[],uint256[])`"]
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
    pub struct SettlePnl {
        pub subaccounts: Vec<[u8; 32]>,
        pub product_ids: Vec<ethers::core::types::U256>,
    }
    #[doc = "`SignedBurnLp((bytes32,uint32,uint128,uint64),bytes)`"]
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
    pub struct SignedBurnLp {
        pub tx: BurnLp,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedLinkSigner((bytes32,bytes32,uint64),bytes)`"]
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
    pub struct SignedLinkSigner {
        pub tx: LinkSigner,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64),bytes)`"]
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
    pub struct SignedLiquidateSubaccount {
        pub tx: LiquidateSubaccount,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64),bytes)`"]
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
    pub struct SignedMintLp {
        pub tx: MintLp,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedOrder((bytes32,int128,int128,uint64,uint64),bytes)`"]
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
    pub struct SignedOrder {
        pub order: Order,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SignedWithdrawCollateral((bytes32,uint32,uint128,uint64),bytes)`"]
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
    pub struct SignedWithdrawCollateral {
        pub tx: WithdrawCollateral,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`SlowModeConfig(uint64,uint64,uint64)`"]
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
    pub struct SlowModeConfig {
        pub timeout: u64,
        pub tx_count: u64,
        pub tx_up_to: u64,
    }
    #[doc = "`SlowModeTx(uint64,address,bytes)`"]
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
    pub struct SlowModeTx {
        pub executable_at: u64,
        pub sender: ethers::core::types::Address,
        pub tx: ethers::core::types::Bytes,
    }
    #[doc = "`SpotTick(uint128)`"]
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
    pub struct SpotTick {
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub time: u128,
    }
    #[doc = "`SwapAMM(bytes32,uint32,int128,int128)`"]
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
    pub struct SwapAMM {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
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
    }
    #[doc = "`UpdateFeeRates(address,uint32,int64,int64)`"]
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
    pub struct UpdateFeeRates {
        pub user: ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    #[doc = "`UpdatePrice(uint32,int128)`"]
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
    pub struct UpdatePrice {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_x18: i128,
    }
    #[doc = "`UpdateProduct(address,bytes)`"]
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
    pub struct UpdateProduct {
        pub engine: ethers::core::types::Address,
        pub tx: ethers::core::types::Bytes,
    }
    #[doc = "`WithdrawCollateral(bytes32,uint32,uint128,uint64)`"]
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
    pub struct WithdrawCollateral {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_u128",
            deserialize_with = "deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
}
