pub use offchain_book::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod offchain_book {
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
    #[doc = "OffchainBook was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isTaker\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"feeAmount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FillOrder\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimSequencerFee\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"feesAmount\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"clearinghouse\",\"outputs\":[{\"internalType\":\"contract IClearinghouse\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropDigest\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropOrderChecked\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dumpFees\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"engine\",\"outputs\":[{\"internalType\":\"contract IProductEngine\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fees\",\"outputs\":[{\"internalType\":\"contract IFeeCalculator\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAmounts\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getAllStates\",\"outputs\":[{\"internalType\":\"struct FOffchainBook.AllStates\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"fees\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IOffchainBook.Market\",\"name\":\"market\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencerFees\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getCollectedFees\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"collectedMakerFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedTakerFees\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDigest\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeRatesX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"takerFeeRateX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"makerFeeRateX18\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMarket\",\"outputs\":[{\"internalType\":\"struct IOffchainBook.Market\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencerFees\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinSize\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePriceX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePricesX18\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Prices\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"spotPriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"perpPriceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order1\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order2\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderFilledAmounts\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"makerFee\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"takerFee\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencer\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"incrementFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IClearinghouse\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IProductEngine\",\"name\":\"_engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IFeeCalculator\",\"name\":\"_fees\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_lpSpreadX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"market\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencerFees\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrderAMM\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"address\",\"name\":\"takerLinkedSigner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"matchOrderAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrdersWithSigner\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.MatchOrders\",\"name\":\"matchOrders\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"amm\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"maker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"address\",\"name\":\"takerLinkedSigner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"makerLinkedSigner\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"matchOrders\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"_sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"_lpSpreadX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"modifyConfig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"takerDigest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"makerDigest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"takerDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"modifyFilledAmount\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct FOffchainBook.AllStates\",\"name\":\"allStates\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"endpoint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"engine\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"fees\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IOffchainBook.Market\",\"name\":\"market\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencerFees\",\"type\":\"int128\",\"components\":[]}]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAllStates\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"filledAmount\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFilledAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IOffchainBook.Market\",\"name\":\"_market\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceIncrementX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sequencerFees\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMarket\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SwapAMM\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"signedOrder\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validateOrder\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static OFFCHAINBOOK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct OffchainBook<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OffchainBook<M> {
        fn clone(&self) -> Self {
            OffchainBook(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OffchainBook<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for OffchainBook<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OffchainBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OffchainBook<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), OFFCHAINBOOK_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `claimSequencerFee` (0x1bcb460a) function"]
        pub fn claim_sequencer_fee(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([27, 203, 70, 10], ())
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
        #[doc = "Calls the contract's `dropDigest` (0xe1e7188d) function"]
        pub fn drop_digest(
            &self,
            digest: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 231, 24, 141], digest)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dropOrder` (0xee442697) function"]
        pub fn drop_order(&self, order: Order) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 68, 38, 151], (order,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dropOrderChecked` (0xd43c77d6) function"]
        pub fn drop_order_checked(
            &self,
            order: Order,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 60, 119, 214], (order,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dumpFees` (0x707c8b58) function"]
        pub fn dump_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `engine` (0xc9d4623f) function"]
        pub fn engine(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([201, 212, 98, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fees` (0x9af1d35a) function"]
        pub fn fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([154, 241, 211, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledAmounts` (0x40f1a34d) function"]
        pub fn filled_amounts(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([64, 241, 163, 77], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAllStates` (0xa4ddb071) function"]
        pub fn get_all_states(&self) -> ethers::contract::builders::ContractCall<M, AllStates> {
            self.0
                .method_hash([164, 221, 176, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollectedFees` (0xd54a8d18) function"]
        pub fn get_collected_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([213, 74, 141, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDigest` (0xe874b4f3) function"]
        pub fn get_digest(
            &self,
            order: Order,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 116, 180, 243], (order,))
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
        #[doc = "Calls the contract's `getFeeRatesX18` (0x5e4b6063) function"]
        pub fn get_fee_rates_x18(
            &self,
            subaccount: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([94, 75, 96, 99], subaccount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMarket` (0xf1be1679) function"]
        pub fn get_market(&self) -> ethers::contract::builders::ContractCall<M, Market> {
            self.0
                .method_hash([241, 190, 22, 121], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinSize` (0xd07041e0) function"]
        pub fn get_min_size(&self) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([208, 112, 65, 224], ())
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
        #[doc = "Calls the contract's `getOrderFilledAmounts` (0x3455d913) function"]
        pub fn get_order_filled_amounts(
            &self,
            order_1: Order,
            order_2: Order,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([52, 85, 217, 19], (order_1, order_2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementFees` (0xa54b7cf3) function"]
        pub fn increment_fees(
            &self,
            maker_fee: i128,
            taker_fee: i128,
            sequencer: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 75, 124, 243], (maker_fee, taker_fee, sequencer))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x5a174b8e) function"]
        pub fn initialize(
            &self,
            clearinghouse: ethers::core::types::Address,
            engine: ethers::core::types::Address,
            endpoint: ethers::core::types::Address,
            admin: ethers::core::types::Address,
            fees: ethers::core::types::Address,
            product_id: u32,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 23, 75, 142],
                    (
                        clearinghouse,
                        engine,
                        endpoint,
                        admin,
                        fees,
                        product_id,
                        size_increment,
                        price_increment_x18,
                        min_size,
                        lp_spread_x18,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `market` (0x80f55605) function"]
        pub fn market(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (u32, i128, i128, i128, i128, i128)>
        {
            self.0
                .method_hash([128, 245, 86, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrderAMM` (0x78f0d3ce) function"]
        pub fn match_order_amm(
            &self,
            txn: MatchOrderAMM,
            taker_linked_signer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 240, 211, 206], (txn, taker_linked_signer))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrders` (0x665dc77e) function"]
        pub fn match_orders(
            &self,
            txn: MatchOrdersWithSigner,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 93, 199, 126], (txn,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `modifyConfig` (0xc40f02bd) function"]
        pub fn modify_config(
            &self,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 15, 2, 189],
                    (size_increment, price_increment_x18, min_size, lp_spread_x18),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `modifyFilledAmount` (0x35ed4e6d) function"]
        pub fn modify_filled_amount(
            &self,
            taker_digest: [u8; 32],
            maker_digest: [u8; 32],
            taker_delta: i128,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash(
                    [53, 237, 78, 109],
                    (taker_digest, maker_digest, taker_delta),
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
        #[doc = "Calls the contract's `setAllStates` (0x81f9d62e) function"]
        pub fn set_all_states(
            &self,
            all_states: AllStates,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 249, 214, 46], (all_states,))
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
        #[doc = "Calls the contract's `setFilledAmount` (0xde1078bd) function"]
        pub fn set_filled_amount(
            &self,
            digest: [u8; 32],
            filled_amount: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 16, 120, 189], (digest, filled_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMarket` (0x945d8ef6) function"]
        pub fn set_market(
            &self,
            market: Market,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 93, 142, 246], (market,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAMM` (0x0f4b509d) function"]
        pub fn swap_amm(&self, txn: SwapAMM) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 75, 80, 157], (txn,))
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
        #[doc = "Calls the contract's `validateOrder` (0xba3691ad) function"]
        pub fn validate_order(
            &self,
            signed_order: SignedOrder,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 54, 145, 173], (signed_order,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FillOrder` event"]
        pub fn fill_order_filter(&self) -> ethers::contract::builders::Event<M, FillOrderFilter> {
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
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OffchainBookEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OffchainBook<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
        name = "FillOrder",
        abi = "FillOrder(bytes32,bytes32,int128,int128,uint64,uint64,bool,int128,int128,int128)"
    )]
    pub struct FillOrderFilter {
        #[ethevent(indexed)]
        pub digest: [u8; 32],
        #[ethevent(indexed)]
        pub subaccount: [u8; 32],
        pub price_x18: i128,
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
        pub is_taker: bool,
        pub fee_amount: i128,
        pub base_delta: i128,
        pub quote_delta: i128,
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
    pub enum OffchainBookEvents {
        FillOrderFilter(FillOrderFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for OffchainBookEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FillOrderFilter::decode_log(log) {
                return Ok(OffchainBookEvents::FillOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OffchainBookEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OffchainBookEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OffchainBookEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OffchainBookEvents::FillOrderFilter(element) => element.fmt(f),
                OffchainBookEvents::InitializedFilter(element) => element.fmt(f),
                OffchainBookEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `claimSequencerFee` function with signature `claimSequencerFee()` and selector `[27, 203, 70, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimSequencerFee", abi = "claimSequencerFee()")]
    pub struct ClaimSequencerFeeCall;
    #[doc = "Container type for all input parameters for the `clearinghouse` function with signature `clearinghouse()` and selector `[93, 79, 95, 151]`"]
    #[derive(
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
    #[doc = "Container type for all input parameters for the `dropDigest` function with signature `dropDigest(bytes32)` and selector `[225, 231, 24, 141]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "dropDigest", abi = "dropDigest(bytes32)")]
    pub struct DropDigestCall {
        pub digest: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `dropOrder` function with signature `dropOrder((bytes32,int128,int128,uint64,uint64))` and selector `[238, 68, 38, 151]`"]
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
        name = "dropOrder",
        abi = "dropOrder((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCall {
        pub order: Order,
    }
    #[doc = "Container type for all input parameters for the `dropOrderChecked` function with signature `dropOrderChecked((bytes32,int128,int128,uint64,uint64))` and selector `[212, 60, 119, 214]`"]
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
        name = "dropOrderChecked",
        abi = "dropOrderChecked((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCheckedCall {
        pub order: Order,
    }
    #[doc = "Container type for all input parameters for the `dumpFees` function with signature `dumpFees()` and selector `[112, 124, 139, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    #[doc = "Container type for all input parameters for the `engine` function with signature `engine()` and selector `[201, 212, 98, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "engine", abi = "engine()")]
    pub struct EngineCall;
    #[doc = "Container type for all input parameters for the `fees` function with signature `fees()` and selector `[154, 241, 211, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "fees", abi = "fees()")]
    pub struct FeesCall;
    #[doc = "Container type for all input parameters for the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `[64, 241, 163, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "filledAmounts", abi = "filledAmounts(bytes32)")]
    pub struct FilledAmountsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `getAllStates` function with signature `getAllStates()` and selector `[164, 221, 176, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAllStates", abi = "getAllStates()")]
    pub struct GetAllStatesCall;
    #[doc = "Container type for all input parameters for the `getCollectedFees` function with signature `getCollectedFees()` and selector `[213, 74, 141, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCollectedFees", abi = "getCollectedFees()")]
    pub struct GetCollectedFeesCall;
    #[doc = "Container type for all input parameters for the `getDigest` function with signature `getDigest((bytes32,int128,int128,uint64,uint64))` and selector `[232, 116, 180, 243]`"]
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
        name = "getDigest",
        abi = "getDigest((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetDigestCall {
        pub order: Order,
    }
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
    #[doc = "Container type for all input parameters for the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32)` and selector `[94, 75, 96, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFeeRatesX18", abi = "getFeeRatesX18(bytes32)")]
    pub struct GetFeeRatesX18Call {
        pub subaccount: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getMarket` function with signature `getMarket()` and selector `[241, 190, 22, 121]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMarket", abi = "getMarket()")]
    pub struct GetMarketCall;
    #[doc = "Container type for all input parameters for the `getMinSize` function with signature `getMinSize()` and selector `[208, 112, 65, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinSize", abi = "getMinSize()")]
    pub struct GetMinSizeCall;
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
    #[doc = "Container type for all input parameters for the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `[52, 85, 217, 19]`"]
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
        name = "getOrderFilledAmounts",
        abi = "getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetOrderFilledAmountsCall {
        pub order_1: Order,
        pub order_2: Order,
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
    #[doc = "Container type for all input parameters for the `incrementFees` function with signature `incrementFees(int128,int128,int128)` and selector `[165, 75, 124, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "incrementFees", abi = "incrementFees(int128,int128,int128)")]
    pub struct IncrementFeesCall {
        pub maker_fee: i128,
        pub taker_fee: i128,
        pub sequencer: i128,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address,uint32,int128,int128,int128,int128)` and selector `[90, 23, 75, 142]`"]
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
        abi = "initialize(address,address,address,address,address,uint32,int128,int128,int128,int128)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ethers::core::types::Address,
        pub engine: ethers::core::types::Address,
        pub endpoint: ethers::core::types::Address,
        pub admin: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
    }
    #[doc = "Container type for all input parameters for the `market` function with signature `market()` and selector `[128, 245, 86, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "market", abi = "market()")]
    pub struct MarketCall;
    #[doc = "Container type for all input parameters for the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)` and selector `[120, 240, 211, 206]`"]
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
        name = "matchOrderAMM",
        abi = "matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)"
    )]
    pub struct MatchOrderAMMCall {
        pub txn: MatchOrderAMM,
        pub taker_linked_signer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))` and selector `[102, 93, 199, 126]`"]
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
        name = "matchOrders",
        abi = "matchOrders(((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))"
    )]
    pub struct MatchOrdersCall {
        pub txn: MatchOrdersWithSigner,
    }
    #[doc = "Container type for all input parameters for the `modifyConfig` function with signature `modifyConfig(int128,int128,int128,int128)` and selector `[196, 15, 2, 189]`"]
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
        name = "modifyConfig",
        abi = "modifyConfig(int128,int128,int128,int128)"
    )]
    pub struct ModifyConfigCall {
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
    }
    #[doc = "Container type for all input parameters for the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `[53, 237, 78, 109]`"]
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
        name = "modifyFilledAmount",
        abi = "modifyFilledAmount(bytes32,bytes32,int128)"
    )]
    pub struct ModifyFilledAmountCall {
        pub taker_digest: [u8; 32],
        pub maker_digest: [u8; 32],
        pub taker_delta: i128,
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
    #[doc = "Container type for all input parameters for the `setAllStates` function with signature `setAllStates((address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128))` and selector `[129, 249, 214, 46]`"]
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
        name = "setAllStates",
        abi = "setAllStates((address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128))"
    )]
    pub struct SetAllStatesCall {
        pub all_states: AllStates,
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
    #[doc = "Container type for all input parameters for the `setFilledAmount` function with signature `setFilledAmount(bytes32,int128)` and selector `[222, 16, 120, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFilledAmount", abi = "setFilledAmount(bytes32,int128)")]
    pub struct SetFilledAmountCall {
        pub digest: [u8; 32],
        pub filled_amount: i128,
    }
    #[doc = "Container type for all input parameters for the `setMarket` function with signature `setMarket((uint32,int128,int128,int128,int128,int128))` and selector `[148, 93, 142, 246]`"]
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
        name = "setMarket",
        abi = "setMarket((uint32,int128,int128,int128,int128,int128))"
    )]
    pub struct SetMarketCall {
        pub market: Market,
    }
    #[doc = "Container type for all input parameters for the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `[15, 75, 80, 157]`"]
    #[derive(
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
        pub txn: SwapAMM,
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
    #[doc = "Container type for all input parameters for the `validateOrder` function with signature `validateOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `[186, 54, 145, 173]`"]
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
        name = "validateOrder",
        abi = "validateOrder(((bytes32,int128,int128,uint64,uint64),bytes))"
    )]
    pub struct ValidateOrderCall {
        pub signed_order: SignedOrder,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OffchainBookCalls {
        ClaimSequencerFee(ClaimSequencerFeeCall),
        Clearinghouse(ClearinghouseCall),
        DropDigest(DropDigestCall),
        DropOrder(DropOrderCall),
        DropOrderChecked(DropOrderCheckedCall),
        DumpFees(DumpFeesCall),
        Engine(EngineCall),
        Fees(FeesCall),
        FilledAmounts(FilledAmountsCall),
        GetAllStates(GetAllStatesCall),
        GetCollectedFees(GetCollectedFeesCall),
        GetDigest(GetDigestCall),
        GetEndpoint(GetEndpointCall),
        GetFeeRatesX18(GetFeeRatesX18Call),
        GetMarket(GetMarketCall),
        GetMinSize(GetMinSizeCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderFilledAmounts(GetOrderFilledAmountsCall),
        GetVersion(GetVersionCall),
        IncrementFees(IncrementFeesCall),
        Initialize(InitializeCall),
        Market(MarketCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        ModifyConfig(ModifyConfigCall),
        ModifyFilledAmount(ModifyFilledAmountCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAllStates(SetAllStatesCall),
        SetEndpoint(SetEndpointCall),
        SetFilledAmount(SetFilledAmountCall),
        SetMarket(SetMarketCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        ValidateOrder(ValidateOrderCall),
    }
    impl ethers::core::abi::AbiDecode for OffchainBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ClaimSequencerFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::ClaimSequencerFee(decoded));
            }
            if let Ok(decoded) =
                <ClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::Clearinghouse(decoded));
            }
            if let Ok(decoded) =
                <DropDigestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::DropDigest(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::DropOrder(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCheckedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::DropOrderChecked(decoded));
            }
            if let Ok(decoded) =
                <DumpFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::DumpFees(decoded));
            }
            if let Ok(decoded) = <EngineCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::Engine(decoded));
            }
            if let Ok(decoded) = <FeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(OffchainBookCalls::Fees(decoded));
            }
            if let Ok(decoded) =
                <FilledAmountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::FilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetAllStatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetAllStates(decoded));
            }
            if let Ok(decoded) =
                <GetCollectedFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <GetDigestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetDigest(decoded));
            }
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetFeeRatesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetFeeRatesX18(decoded));
            }
            if let Ok(decoded) =
                <GetMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetMarket(decoded));
            }
            if let Ok(decoded) =
                <GetMinSizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetMinSize(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePricesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) =
                <GetOrderFilledAmountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetOrderFilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <IncrementFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::IncrementFees(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::Market(decoded));
            }
            if let Ok(decoded) =
                <MatchOrderAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <ModifyConfigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::ModifyConfig(decoded));
            }
            if let Ok(decoded) =
                <ModifyFilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::ModifyFilledAmount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetAllStatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::SetAllStates(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetFilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::SetFilledAmount(decoded));
            }
            if let Ok(decoded) =
                <SetMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::SetMarket(decoded));
            }
            if let Ok(decoded) =
                <SwapAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::SwapAMM(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ValidateOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainBookCalls::ValidateOrder(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OffchainBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OffchainBookCalls::ClaimSequencerFee(element) => element.encode(),
                OffchainBookCalls::Clearinghouse(element) => element.encode(),
                OffchainBookCalls::DropDigest(element) => element.encode(),
                OffchainBookCalls::DropOrder(element) => element.encode(),
                OffchainBookCalls::DropOrderChecked(element) => element.encode(),
                OffchainBookCalls::DumpFees(element) => element.encode(),
                OffchainBookCalls::Engine(element) => element.encode(),
                OffchainBookCalls::Fees(element) => element.encode(),
                OffchainBookCalls::FilledAmounts(element) => element.encode(),
                OffchainBookCalls::GetAllStates(element) => element.encode(),
                OffchainBookCalls::GetCollectedFees(element) => element.encode(),
                OffchainBookCalls::GetDigest(element) => element.encode(),
                OffchainBookCalls::GetEndpoint(element) => element.encode(),
                OffchainBookCalls::GetFeeRatesX18(element) => element.encode(),
                OffchainBookCalls::GetMarket(element) => element.encode(),
                OffchainBookCalls::GetMinSize(element) => element.encode(),
                OffchainBookCalls::GetOraclePriceX18(element) => element.encode(),
                OffchainBookCalls::GetOraclePricesX18(element) => element.encode(),
                OffchainBookCalls::GetOrderFilledAmounts(element) => element.encode(),
                OffchainBookCalls::GetVersion(element) => element.encode(),
                OffchainBookCalls::IncrementFees(element) => element.encode(),
                OffchainBookCalls::Initialize(element) => element.encode(),
                OffchainBookCalls::Market(element) => element.encode(),
                OffchainBookCalls::MatchOrderAMM(element) => element.encode(),
                OffchainBookCalls::MatchOrders(element) => element.encode(),
                OffchainBookCalls::ModifyConfig(element) => element.encode(),
                OffchainBookCalls::ModifyFilledAmount(element) => element.encode(),
                OffchainBookCalls::Owner(element) => element.encode(),
                OffchainBookCalls::RenounceOwnership(element) => element.encode(),
                OffchainBookCalls::SetAllStates(element) => element.encode(),
                OffchainBookCalls::SetEndpoint(element) => element.encode(),
                OffchainBookCalls::SetFilledAmount(element) => element.encode(),
                OffchainBookCalls::SetMarket(element) => element.encode(),
                OffchainBookCalls::SwapAMM(element) => element.encode(),
                OffchainBookCalls::TransferOwnership(element) => element.encode(),
                OffchainBookCalls::ValidateOrder(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for OffchainBookCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OffchainBookCalls::ClaimSequencerFee(element) => element.fmt(f),
                OffchainBookCalls::Clearinghouse(element) => element.fmt(f),
                OffchainBookCalls::DropDigest(element) => element.fmt(f),
                OffchainBookCalls::DropOrder(element) => element.fmt(f),
                OffchainBookCalls::DropOrderChecked(element) => element.fmt(f),
                OffchainBookCalls::DumpFees(element) => element.fmt(f),
                OffchainBookCalls::Engine(element) => element.fmt(f),
                OffchainBookCalls::Fees(element) => element.fmt(f),
                OffchainBookCalls::FilledAmounts(element) => element.fmt(f),
                OffchainBookCalls::GetAllStates(element) => element.fmt(f),
                OffchainBookCalls::GetCollectedFees(element) => element.fmt(f),
                OffchainBookCalls::GetDigest(element) => element.fmt(f),
                OffchainBookCalls::GetEndpoint(element) => element.fmt(f),
                OffchainBookCalls::GetFeeRatesX18(element) => element.fmt(f),
                OffchainBookCalls::GetMarket(element) => element.fmt(f),
                OffchainBookCalls::GetMinSize(element) => element.fmt(f),
                OffchainBookCalls::GetOraclePriceX18(element) => element.fmt(f),
                OffchainBookCalls::GetOraclePricesX18(element) => element.fmt(f),
                OffchainBookCalls::GetOrderFilledAmounts(element) => element.fmt(f),
                OffchainBookCalls::GetVersion(element) => element.fmt(f),
                OffchainBookCalls::IncrementFees(element) => element.fmt(f),
                OffchainBookCalls::Initialize(element) => element.fmt(f),
                OffchainBookCalls::Market(element) => element.fmt(f),
                OffchainBookCalls::MatchOrderAMM(element) => element.fmt(f),
                OffchainBookCalls::MatchOrders(element) => element.fmt(f),
                OffchainBookCalls::ModifyConfig(element) => element.fmt(f),
                OffchainBookCalls::ModifyFilledAmount(element) => element.fmt(f),
                OffchainBookCalls::Owner(element) => element.fmt(f),
                OffchainBookCalls::RenounceOwnership(element) => element.fmt(f),
                OffchainBookCalls::SetAllStates(element) => element.fmt(f),
                OffchainBookCalls::SetEndpoint(element) => element.fmt(f),
                OffchainBookCalls::SetFilledAmount(element) => element.fmt(f),
                OffchainBookCalls::SetMarket(element) => element.fmt(f),
                OffchainBookCalls::SwapAMM(element) => element.fmt(f),
                OffchainBookCalls::TransferOwnership(element) => element.fmt(f),
                OffchainBookCalls::ValidateOrder(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ClaimSequencerFeeCall> for OffchainBookCalls {
        fn from(var: ClaimSequencerFeeCall) -> Self {
            OffchainBookCalls::ClaimSequencerFee(var)
        }
    }
    impl ::std::convert::From<ClearinghouseCall> for OffchainBookCalls {
        fn from(var: ClearinghouseCall) -> Self {
            OffchainBookCalls::Clearinghouse(var)
        }
    }
    impl ::std::convert::From<DropDigestCall> for OffchainBookCalls {
        fn from(var: DropDigestCall) -> Self {
            OffchainBookCalls::DropDigest(var)
        }
    }
    impl ::std::convert::From<DropOrderCall> for OffchainBookCalls {
        fn from(var: DropOrderCall) -> Self {
            OffchainBookCalls::DropOrder(var)
        }
    }
    impl ::std::convert::From<DropOrderCheckedCall> for OffchainBookCalls {
        fn from(var: DropOrderCheckedCall) -> Self {
            OffchainBookCalls::DropOrderChecked(var)
        }
    }
    impl ::std::convert::From<DumpFeesCall> for OffchainBookCalls {
        fn from(var: DumpFeesCall) -> Self {
            OffchainBookCalls::DumpFees(var)
        }
    }
    impl ::std::convert::From<EngineCall> for OffchainBookCalls {
        fn from(var: EngineCall) -> Self {
            OffchainBookCalls::Engine(var)
        }
    }
    impl ::std::convert::From<FeesCall> for OffchainBookCalls {
        fn from(var: FeesCall) -> Self {
            OffchainBookCalls::Fees(var)
        }
    }
    impl ::std::convert::From<FilledAmountsCall> for OffchainBookCalls {
        fn from(var: FilledAmountsCall) -> Self {
            OffchainBookCalls::FilledAmounts(var)
        }
    }
    impl ::std::convert::From<GetAllStatesCall> for OffchainBookCalls {
        fn from(var: GetAllStatesCall) -> Self {
            OffchainBookCalls::GetAllStates(var)
        }
    }
    impl ::std::convert::From<GetCollectedFeesCall> for OffchainBookCalls {
        fn from(var: GetCollectedFeesCall) -> Self {
            OffchainBookCalls::GetCollectedFees(var)
        }
    }
    impl ::std::convert::From<GetDigestCall> for OffchainBookCalls {
        fn from(var: GetDigestCall) -> Self {
            OffchainBookCalls::GetDigest(var)
        }
    }
    impl ::std::convert::From<GetEndpointCall> for OffchainBookCalls {
        fn from(var: GetEndpointCall) -> Self {
            OffchainBookCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetFeeRatesX18Call> for OffchainBookCalls {
        fn from(var: GetFeeRatesX18Call) -> Self {
            OffchainBookCalls::GetFeeRatesX18(var)
        }
    }
    impl ::std::convert::From<GetMarketCall> for OffchainBookCalls {
        fn from(var: GetMarketCall) -> Self {
            OffchainBookCalls::GetMarket(var)
        }
    }
    impl ::std::convert::From<GetMinSizeCall> for OffchainBookCalls {
        fn from(var: GetMinSizeCall) -> Self {
            OffchainBookCalls::GetMinSize(var)
        }
    }
    impl ::std::convert::From<GetOraclePriceX18Call> for OffchainBookCalls {
        fn from(var: GetOraclePriceX18Call) -> Self {
            OffchainBookCalls::GetOraclePriceX18(var)
        }
    }
    impl ::std::convert::From<GetOraclePricesX18Call> for OffchainBookCalls {
        fn from(var: GetOraclePricesX18Call) -> Self {
            OffchainBookCalls::GetOraclePricesX18(var)
        }
    }
    impl ::std::convert::From<GetOrderFilledAmountsCall> for OffchainBookCalls {
        fn from(var: GetOrderFilledAmountsCall) -> Self {
            OffchainBookCalls::GetOrderFilledAmounts(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for OffchainBookCalls {
        fn from(var: GetVersionCall) -> Self {
            OffchainBookCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<IncrementFeesCall> for OffchainBookCalls {
        fn from(var: IncrementFeesCall) -> Self {
            OffchainBookCalls::IncrementFees(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for OffchainBookCalls {
        fn from(var: InitializeCall) -> Self {
            OffchainBookCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MarketCall> for OffchainBookCalls {
        fn from(var: MarketCall) -> Self {
            OffchainBookCalls::Market(var)
        }
    }
    impl ::std::convert::From<MatchOrderAMMCall> for OffchainBookCalls {
        fn from(var: MatchOrderAMMCall) -> Self {
            OffchainBookCalls::MatchOrderAMM(var)
        }
    }
    impl ::std::convert::From<MatchOrdersCall> for OffchainBookCalls {
        fn from(var: MatchOrdersCall) -> Self {
            OffchainBookCalls::MatchOrders(var)
        }
    }
    impl ::std::convert::From<ModifyConfigCall> for OffchainBookCalls {
        fn from(var: ModifyConfigCall) -> Self {
            OffchainBookCalls::ModifyConfig(var)
        }
    }
    impl ::std::convert::From<ModifyFilledAmountCall> for OffchainBookCalls {
        fn from(var: ModifyFilledAmountCall) -> Self {
            OffchainBookCalls::ModifyFilledAmount(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for OffchainBookCalls {
        fn from(var: OwnerCall) -> Self {
            OffchainBookCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for OffchainBookCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            OffchainBookCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetAllStatesCall> for OffchainBookCalls {
        fn from(var: SetAllStatesCall) -> Self {
            OffchainBookCalls::SetAllStates(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for OffchainBookCalls {
        fn from(var: SetEndpointCall) -> Self {
            OffchainBookCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetFilledAmountCall> for OffchainBookCalls {
        fn from(var: SetFilledAmountCall) -> Self {
            OffchainBookCalls::SetFilledAmount(var)
        }
    }
    impl ::std::convert::From<SetMarketCall> for OffchainBookCalls {
        fn from(var: SetMarketCall) -> Self {
            OffchainBookCalls::SetMarket(var)
        }
    }
    impl ::std::convert::From<SwapAMMCall> for OffchainBookCalls {
        fn from(var: SwapAMMCall) -> Self {
            OffchainBookCalls::SwapAMM(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for OffchainBookCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            OffchainBookCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<ValidateOrderCall> for OffchainBookCalls {
        fn from(var: ValidateOrderCall) -> Self {
            OffchainBookCalls::ValidateOrder(var)
        }
    }
    #[doc = "Container type for all return fields from the `claimSequencerFee` function with signature `claimSequencerFee()` and selector `[27, 203, 70, 10]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ClaimSequencerFeeReturn {
        pub fees_amount: i128,
    }
    #[doc = "Container type for all return fields from the `clearinghouse` function with signature `clearinghouse()` and selector `[93, 79, 95, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ClearinghouseReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `engine` function with signature `engine()` and selector `[201, 212, 98, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct EngineReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `fees` function with signature `fees()` and selector `[154, 241, 211, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `[64, 241, 163, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FilledAmountsReturn(pub i128);
    #[doc = "Container type for all return fields from the `getAllStates` function with signature `getAllStates()` and selector `[164, 221, 176, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAllStatesReturn(pub AllStates);
    #[doc = "Container type for all return fields from the `getCollectedFees` function with signature `getCollectedFees()` and selector `[213, 74, 141, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCollectedFeesReturn {
        pub collected_maker_fees: i128,
        pub collected_taker_fees: i128,
    }
    #[doc = "Container type for all return fields from the `getDigest` function with signature `getDigest((bytes32,int128,int128,uint64,uint64))` and selector `[232, 116, 180, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDigestReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32)` and selector `[94, 75, 96, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFeeRatesX18Return {
        pub taker_fee_rate_x18: i128,
        pub maker_fee_rate_x18: i128,
    }
    #[doc = "Container type for all return fields from the `getMarket` function with signature `getMarket()` and selector `[241, 190, 22, 121]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMarketReturn(pub Market);
    #[doc = "Container type for all return fields from the `getMinSize` function with signature `getMinSize()` and selector `[208, 112, 65, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMinSizeReturn(pub i128);
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
    #[doc = "Container type for all return fields from the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `[52, 85, 217, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOrderFilledAmountsReturn(pub i128, pub i128);
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
    #[doc = "Container type for all return fields from the `market` function with signature `market()` and selector `[128, 245, 86, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MarketReturn {
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub lp_spread_x18: i128,
        pub collected_fees: i128,
        pub sequencer_fees: i128,
    }
    #[doc = "Container type for all return fields from the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `[53, 237, 78, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ModifyFilledAmountReturn(pub i128, pub i128);
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
    #[doc = "`AllStates(address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllStates {
        pub endpoint: ethers::core::types::Address,
        pub clearinghouse: ethers::core::types::Address,
        pub engine: ethers::core::types::Address,
        pub fees: ethers::core::types::Address,
        pub market: Market,
        pub min_size: i128,
    }
    #[doc = "`MatchOrderAMM(uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes))`"]
    #[derive(
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
        pub base_delta: i128,
        pub quote_delta: i128,
        pub taker: SignedOrder,
    }
    #[doc = "`MatchOrders(uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`"]
    #[derive(
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
    #[doc = "`MatchOrdersWithSigner((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MatchOrdersWithSigner {
        pub match_orders: MatchOrders,
        pub taker_linked_signer: ethers::core::types::Address,
        pub maker_linked_signer: ethers::core::types::Address,
    }
    #[doc = "`Order(bytes32,int128,int128,uint64,uint64)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Order {
        pub sender: [u8; 32],
        pub price_x18: i128,
        pub amount: i128,
        pub expiration: u64,
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
    #[doc = "`SignedOrder((bytes32,int128,int128,uint64,uint64),bytes)`"]
    #[derive(
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
    #[doc = "`SwapAMM(bytes32,uint32,int128,int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SwapAMM {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: i128,
        pub price_x18: i128,
    }
    #[doc = "`Market(uint32,int128,int128,int128,int128,int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Market {
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub lp_spread_x18: i128,
        pub collected_fees: i128,
        pub sequencer_fees: i128,
    }
}
