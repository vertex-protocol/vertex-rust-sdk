pub use offchain_exchange::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod offchain_exchange {
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
    #[doc = "OffchainExchange was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isTaker\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"feeAmount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FillOrder\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropDigest\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropOrderChecked\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dumpFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledAmounts\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"users\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllFeeRates\",\"outputs\":[{\"internalType\":\"struct IOffchainExchange.FeeRates[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"isNonDefault\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllVirtualBooks\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCollectedFees\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"collectedMakerFees\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedTakerFees\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"startAt\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"limit\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCustomFeeAddresses\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDigest\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"taker\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeFractionX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeRatesX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLpParams\",\"outputs\":[{\"internalType\":\"struct IOffchainExchange.LpParams\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMarketInfo\",\"outputs\":[{\"internalType\":\"struct IOffchainExchange.MarketInfo\",\"name\":\"m\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMinSize\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order1\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order2\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderFilledAmounts\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSizeIncrement\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVirtualBook\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"makerFee\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"takerFee\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"incrementFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrderAMM\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"baseDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"quoteDelta\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"address\",\"name\":\"takerLinkedSigner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"matchOrderAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.MatchOrdersWithSigner\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.MatchOrders\",\"name\":\"matchOrders\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"taker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct IEndpoint.SignedOrder\",\"name\":\"maker\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IEndpoint.Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"expiration\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"address\",\"name\":\"takerLinkedSigner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"makerLinkedSigner\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"matchOrders\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"takerDigest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"makerDigest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"takerDelta\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"modifyFilledAmount\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"struct IOffchainExchange.FeeRates\",\"name\":\"feeRate\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"isNonDefault\",\"type\":\"uint8\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"digest\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"filledAmount\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFilledAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.SwapAMM\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"priceX18\",\"type\":\"int128\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapAMM\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"collectedFees\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateCollectedFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeRates\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"virtualBook\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"sizeIncrement\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"minSize\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"lpSpreadX18\",\"type\":\"int128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateMarket\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint32[]\",\"name\":\"productIds\",\"type\":\"uint32[]\",\"components\":[]},{\"internalType\":\"int128[]\",\"name\":\"minSizes\",\"type\":\"int128[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateMinSizes\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static OFFCHAINEXCHANGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct OffchainExchange<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OffchainExchange<M> {
        fn clone(&self) -> Self {
            OffchainExchange(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OffchainExchange<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for OffchainExchange<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OffchainExchange))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OffchainExchange<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), OFFCHAINEXCHANGE_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `dropOrder` (0x9376003e) function"]
        pub fn drop_order(
            &self,
            product_id: u32,
            order: Order,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 118, 0, 62], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dropOrderChecked` (0xfb420c59) function"]
        pub fn drop_order_checked(
            &self,
            product_id: u32,
            order: Order,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 66, 12, 89], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dumpFees` (0x707c8b58) function"]
        pub fn dump_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
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
        #[doc = "Calls the contract's `getAllFeeRates` (0xd895202a) function"]
        pub fn get_all_fee_rates(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FeeRates>> {
            self.0
                .method_hash([216, 149, 32, 42], (users, product_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAllVirtualBooks` (0xce933e59) function"]
        pub fn get_all_virtual_books(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([206, 147, 62, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollectedFees` (0xff0be9ef) function"]
        pub fn get_collected_fees(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([255, 11, 233, 239], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCustomFeeAddresses` (0x3fceea28) function"]
        pub fn get_custom_fee_addresses(
            &self,
            start_at: u32,
            limit: u32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 206, 234, 40], (start_at, limit))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDigest` (0x95ee6071) function"]
        pub fn get_digest(
            &self,
            product_id: u32,
            order: Order,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([149, 238, 96, 113], (product_id, order))
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
        #[doc = "Calls the contract's `getFeeFractionX18` (0xb5cbd70e) function"]
        pub fn get_fee_fraction_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            taker: bool,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([181, 203, 215, 14], (subaccount, product_id, taker))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeRatesX18` (0x0f2c878e) function"]
        pub fn get_fee_rates_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([15, 44, 135, 142], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLpParams` (0x4821c8b5) function"]
        pub fn get_lp_params(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, LpParams> {
            self.0
                .method_hash([72, 33, 200, 181], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMarketInfo` (0x1d029b4d) function"]
        pub fn get_market_info(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, MarketInfo> {
            self.0
                .method_hash([29, 2, 155, 77], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinSize` (0xb60aaa7c) function"]
        pub fn get_min_size(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([182, 10, 170, 124], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderFilledAmounts` (0xa39b9dcd) function"]
        pub fn get_order_filled_amounts(
            &self,
            product_id: u32,
            order_1: Order,
            order_2: Order,
        ) -> ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([163, 155, 157, 205], (product_id, order_1, order_2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSizeIncrement` (0xf2b26331) function"]
        pub fn get_size_increment(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([242, 178, 99, 49], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVirtualBook` (0x66f87bd1) function"]
        pub fn get_virtual_book(
            &self,
            product_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([102, 248, 123, 209], product_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementFees` (0x1f4ce016) function"]
        pub fn increment_fees(
            &self,
            product_id: u32,
            maker_fee: i128,
            taker_fee: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 76, 224, 22], (product_id, maker_fee, taker_fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            clearinghouse: ethers::core::types::Address,
            endpoint: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (clearinghouse, endpoint))
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
        #[doc = "Calls the contract's `matchOrders` (0x88bc7968) function"]
        pub fn match_orders(
            &self,
            txn: MatchOrdersWithSigner,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 188, 121, 104], (txn,))
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
        #[doc = "Calls the contract's `setEndpoint` (0xdbbb4155) function"]
        pub fn set_endpoint(
            &self,
            endpoint: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 187, 65, 85], endpoint)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeRate` (0xceba8953) function"]
        pub fn set_fee_rate(
            &self,
            user: ethers::core::types::Address,
            product_id: u32,
            fee_rate: FeeRates,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 186, 137, 83], (user, product_id, fee_rate))
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
        #[doc = "Calls the contract's `updateCollectedFees` (0x812609f1) function"]
        pub fn update_collected_fees(
            &self,
            product_id: u32,
            collected_fees: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 38, 9, 241], (product_id, collected_fees))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFeeRates` (0xb76d78e3) function"]
        pub fn update_fee_rates(
            &self,
            user: ethers::core::types::Address,
            product_id: u32,
            maker_rate_x18: i64,
            taker_rate_x18: i64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 109, 120, 227],
                    (user, product_id, maker_rate_x18, taker_rate_x18),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateMarket` (0x03a21ae0) function"]
        pub fn update_market(
            &self,
            product_id: u32,
            virtual_book: ethers::core::types::Address,
            size_increment: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [3, 162, 26, 224],
                    (
                        product_id,
                        virtual_book,
                        size_increment,
                        min_size,
                        lp_spread_x18,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateMinSizes` (0xe77bce84) function"]
        pub fn update_min_sizes(
            &self,
            product_ids: ::std::vec::Vec<u32>,
            min_sizes: ::std::vec::Vec<i128>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 123, 206, 132], (product_ids, min_sizes))
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, OffchainExchangeEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OffchainExchange<M> {
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
        abi = "FillOrder(uint32,bytes32,bytes32,int128,int128,uint64,uint64,bool,int128,int128,int128)"
    )]
    pub struct FillOrderFilter {
        #[ethevent(indexed)]
        pub product_id: u32,
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
    pub enum OffchainExchangeEvents {
        FillOrderFilter(FillOrderFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for OffchainExchangeEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FillOrderFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::FillOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OffchainExchangeEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OffchainExchangeEvents::FillOrderFilter(element) => element.fmt(f),
                OffchainExchangeEvents::InitializedFilter(element) => element.fmt(f),
                OffchainExchangeEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `dropOrder` function with signature `dropOrder(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `[147, 118, 0, 62]`"]
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
        abi = "dropOrder(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCall {
        pub product_id: u32,
        pub order: Order,
    }
    #[doc = "Container type for all input parameters for the `dropOrderChecked` function with signature `dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `[251, 66, 12, 89]`"]
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
        abi = "dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCheckedCall {
        pub product_id: u32,
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
    #[doc = "Container type for all input parameters for the `getAllFeeRates` function with signature `getAllFeeRates(address[],uint32[])` and selector `[216, 149, 32, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAllFeeRates", abi = "getAllFeeRates(address[],uint32[])")]
    pub struct GetAllFeeRatesCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
        pub product_ids: ::std::vec::Vec<u32>,
    }
    #[doc = "Container type for all input parameters for the `getAllVirtualBooks` function with signature `getAllVirtualBooks()` and selector `[206, 147, 62, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAllVirtualBooks", abi = "getAllVirtualBooks()")]
    pub struct GetAllVirtualBooksCall;
    #[doc = "Container type for all input parameters for the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `[255, 11, 233, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCollectedFees", abi = "getCollectedFees(uint32)")]
    pub struct GetCollectedFeesCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `[63, 206, 234, 40]`"]
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
        name = "getCustomFeeAddresses",
        abi = "getCustomFeeAddresses(uint32,uint32)"
    )]
    pub struct GetCustomFeeAddressesCall {
        pub start_at: u32,
        pub limit: u32,
    }
    #[doc = "Container type for all input parameters for the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `[149, 238, 96, 113]`"]
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
        abi = "getDigest(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetDigestCall {
        pub product_id: u32,
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
    #[doc = "Container type for all input parameters for the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `[181, 203, 215, 14]`"]
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
        name = "getFeeFractionX18",
        abi = "getFeeFractionX18(bytes32,uint32,bool)"
    )]
    pub struct GetFeeFractionX18Call {
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub taker: bool,
    }
    #[doc = "Container type for all input parameters for the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `[15, 44, 135, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFeeRatesX18", abi = "getFeeRatesX18(bytes32,uint32)")]
    pub struct GetFeeRatesX18Call {
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getLpParams` function with signature `getLpParams(uint32)` and selector `[72, 33, 200, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLpParams", abi = "getLpParams(uint32)")]
    pub struct GetLpParamsCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `[29, 2, 155, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMarketInfo", abi = "getMarketInfo(uint32)")]
    pub struct GetMarketInfoCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getMinSize` function with signature `getMinSize(uint32)` and selector `[182, 10, 170, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMinSize", abi = "getMinSize(uint32)")]
    pub struct GetMinSizeCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `[163, 155, 157, 205]`"]
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
        abi = "getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetOrderFilledAmountsCall {
        pub product_id: u32,
        pub order_1: Order,
        pub order_2: Order,
    }
    #[doc = "Container type for all input parameters for the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `[242, 178, 99, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSizeIncrement", abi = "getSizeIncrement(uint32)")]
    pub struct GetSizeIncrementCall {
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
    #[doc = "Container type for all input parameters for the `getVirtualBook` function with signature `getVirtualBook(uint32)` and selector `[102, 248, 123, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getVirtualBook", abi = "getVirtualBook(uint32)")]
    pub struct GetVirtualBookCall {
        pub product_id: u32,
    }
    #[doc = "Container type for all input parameters for the `incrementFees` function with signature `incrementFees(uint32,int128,int128)` and selector `[31, 76, 224, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "incrementFees", abi = "incrementFees(uint32,int128,int128)")]
    pub struct IncrementFeesCall {
        pub product_id: u32,
        pub maker_fee: i128,
        pub taker_fee: i128,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `[72, 92, 201, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub clearinghouse: ethers::core::types::Address,
        pub endpoint: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))` and selector `[136, 188, 121, 104]`"]
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
        abi = "matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))"
    )]
    pub struct MatchOrdersCall {
        pub txn: MatchOrdersWithSigner,
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
    #[doc = "Container type for all input parameters for the `setFeeRate` function with signature `setFeeRate(address,uint32,(int64,int64,uint8))` and selector `[206, 186, 137, 83]`"]
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
        name = "setFeeRate",
        abi = "setFeeRate(address,uint32,(int64,int64,uint8))"
    )]
    pub struct SetFeeRateCall {
        pub user: ethers::core::types::Address,
        pub product_id: u32,
        pub fee_rate: FeeRates,
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
    #[doc = "Container type for all input parameters for the `updateCollectedFees` function with signature `updateCollectedFees(uint32,int128)` and selector `[129, 38, 9, 241]`"]
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
        name = "updateCollectedFees",
        abi = "updateCollectedFees(uint32,int128)"
    )]
    pub struct UpdateCollectedFeesCall {
        pub product_id: u32,
        pub collected_fees: i128,
    }
    #[doc = "Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates(address,uint32,int64,int64)` and selector `[183, 109, 120, 227]`"]
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
        abi = "updateFeeRates(address,uint32,int64,int64)"
    )]
    pub struct UpdateFeeRatesCall {
        pub user: ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    #[doc = "Container type for all input parameters for the `updateMarket` function with signature `updateMarket(uint32,address,int128,int128,int128)` and selector `[3, 162, 26, 224]`"]
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
        name = "updateMarket",
        abi = "updateMarket(uint32,address,int128,int128,int128)"
    )]
    pub struct UpdateMarketCall {
        pub product_id: u32,
        pub virtual_book: ethers::core::types::Address,
        pub size_increment: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
    }
    #[doc = "Container type for all input parameters for the `updateMinSizes` function with signature `updateMinSizes(uint32[],int128[])` and selector `[231, 123, 206, 132]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateMinSizes", abi = "updateMinSizes(uint32[],int128[])")]
    pub struct UpdateMinSizesCall {
        pub product_ids: ::std::vec::Vec<u32>,
        pub min_sizes: ::std::vec::Vec<i128>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OffchainExchangeCalls {
        DropDigest(DropDigestCall),
        DropOrder(DropOrderCall),
        DropOrderChecked(DropOrderCheckedCall),
        DumpFees(DumpFeesCall),
        FilledAmounts(FilledAmountsCall),
        GetAllFeeRates(GetAllFeeRatesCall),
        GetAllVirtualBooks(GetAllVirtualBooksCall),
        GetCollectedFees(GetCollectedFeesCall),
        GetCustomFeeAddresses(GetCustomFeeAddressesCall),
        GetDigest(GetDigestCall),
        GetEndpoint(GetEndpointCall),
        GetFeeFractionX18(GetFeeFractionX18Call),
        GetFeeRatesX18(GetFeeRatesX18Call),
        GetLpParams(GetLpParamsCall),
        GetMarketInfo(GetMarketInfoCall),
        GetMinSize(GetMinSizeCall),
        GetOrderFilledAmounts(GetOrderFilledAmountsCall),
        GetSizeIncrement(GetSizeIncrementCall),
        GetVersion(GetVersionCall),
        GetVirtualBook(GetVirtualBookCall),
        IncrementFees(IncrementFeesCall),
        Initialize(InitializeCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        ModifyFilledAmount(ModifyFilledAmountCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetEndpoint(SetEndpointCall),
        SetFeeRate(SetFeeRateCall),
        SetFilledAmount(SetFilledAmountCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateCollectedFees(UpdateCollectedFeesCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpdateMarket(UpdateMarketCall),
        UpdateMinSizes(UpdateMinSizesCall),
    }
    impl ethers::core::abi::AbiDecode for OffchainExchangeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DropDigestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::DropDigest(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::DropOrder(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCheckedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::DropOrderChecked(decoded));
            }
            if let Ok(decoded) =
                <DumpFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::DumpFees(decoded));
            }
            if let Ok(decoded) =
                <FilledAmountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::FilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetAllFeeRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetAllFeeRates(decoded));
            }
            if let Ok(decoded) =
                <GetAllVirtualBooksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetAllVirtualBooks(decoded));
            }
            if let Ok(decoded) =
                <GetCollectedFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <GetCustomFeeAddressesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetCustomFeeAddresses(decoded));
            }
            if let Ok(decoded) =
                <GetDigestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetDigest(decoded));
            }
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFractionX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetFeeRatesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetFeeRatesX18(decoded));
            }
            if let Ok(decoded) =
                <GetLpParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetLpParams(decoded));
            }
            if let Ok(decoded) =
                <GetMarketInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetMarketInfo(decoded));
            }
            if let Ok(decoded) =
                <GetMinSizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetMinSize(decoded));
            }
            if let Ok(decoded) =
                <GetOrderFilledAmountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetOrderFilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetSizeIncrementCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetSizeIncrement(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualBookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::GetVirtualBook(decoded));
            }
            if let Ok(decoded) =
                <IncrementFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::IncrementFees(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <MatchOrderAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <ModifyFilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::ModifyFilledAmount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::SetFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetFilledAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::SetFilledAmount(decoded));
            }
            if let Ok(decoded) =
                <SwapAMMCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::SwapAMM(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateCollectedFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::UpdateCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) =
                <UpdateMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::UpdateMarket(decoded));
            }
            if let Ok(decoded) =
                <UpdateMinSizesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OffchainExchangeCalls::UpdateMinSizes(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OffchainExchangeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OffchainExchangeCalls::DropDigest(element) => element.encode(),
                OffchainExchangeCalls::DropOrder(element) => element.encode(),
                OffchainExchangeCalls::DropOrderChecked(element) => element.encode(),
                OffchainExchangeCalls::DumpFees(element) => element.encode(),
                OffchainExchangeCalls::FilledAmounts(element) => element.encode(),
                OffchainExchangeCalls::GetAllFeeRates(element) => element.encode(),
                OffchainExchangeCalls::GetAllVirtualBooks(element) => element.encode(),
                OffchainExchangeCalls::GetCollectedFees(element) => element.encode(),
                OffchainExchangeCalls::GetCustomFeeAddresses(element) => element.encode(),
                OffchainExchangeCalls::GetDigest(element) => element.encode(),
                OffchainExchangeCalls::GetEndpoint(element) => element.encode(),
                OffchainExchangeCalls::GetFeeFractionX18(element) => element.encode(),
                OffchainExchangeCalls::GetFeeRatesX18(element) => element.encode(),
                OffchainExchangeCalls::GetLpParams(element) => element.encode(),
                OffchainExchangeCalls::GetMarketInfo(element) => element.encode(),
                OffchainExchangeCalls::GetMinSize(element) => element.encode(),
                OffchainExchangeCalls::GetOrderFilledAmounts(element) => element.encode(),
                OffchainExchangeCalls::GetSizeIncrement(element) => element.encode(),
                OffchainExchangeCalls::GetVersion(element) => element.encode(),
                OffchainExchangeCalls::GetVirtualBook(element) => element.encode(),
                OffchainExchangeCalls::IncrementFees(element) => element.encode(),
                OffchainExchangeCalls::Initialize(element) => element.encode(),
                OffchainExchangeCalls::MatchOrderAMM(element) => element.encode(),
                OffchainExchangeCalls::MatchOrders(element) => element.encode(),
                OffchainExchangeCalls::ModifyFilledAmount(element) => element.encode(),
                OffchainExchangeCalls::Owner(element) => element.encode(),
                OffchainExchangeCalls::RenounceOwnership(element) => element.encode(),
                OffchainExchangeCalls::SetEndpoint(element) => element.encode(),
                OffchainExchangeCalls::SetFeeRate(element) => element.encode(),
                OffchainExchangeCalls::SetFilledAmount(element) => element.encode(),
                OffchainExchangeCalls::SwapAMM(element) => element.encode(),
                OffchainExchangeCalls::TransferOwnership(element) => element.encode(),
                OffchainExchangeCalls::UpdateCollectedFees(element) => element.encode(),
                OffchainExchangeCalls::UpdateFeeRates(element) => element.encode(),
                OffchainExchangeCalls::UpdateMarket(element) => element.encode(),
                OffchainExchangeCalls::UpdateMinSizes(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for OffchainExchangeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OffchainExchangeCalls::DropDigest(element) => element.fmt(f),
                OffchainExchangeCalls::DropOrder(element) => element.fmt(f),
                OffchainExchangeCalls::DropOrderChecked(element) => element.fmt(f),
                OffchainExchangeCalls::DumpFees(element) => element.fmt(f),
                OffchainExchangeCalls::FilledAmounts(element) => element.fmt(f),
                OffchainExchangeCalls::GetAllFeeRates(element) => element.fmt(f),
                OffchainExchangeCalls::GetAllVirtualBooks(element) => element.fmt(f),
                OffchainExchangeCalls::GetCollectedFees(element) => element.fmt(f),
                OffchainExchangeCalls::GetCustomFeeAddresses(element) => element.fmt(f),
                OffchainExchangeCalls::GetDigest(element) => element.fmt(f),
                OffchainExchangeCalls::GetEndpoint(element) => element.fmt(f),
                OffchainExchangeCalls::GetFeeFractionX18(element) => element.fmt(f),
                OffchainExchangeCalls::GetFeeRatesX18(element) => element.fmt(f),
                OffchainExchangeCalls::GetLpParams(element) => element.fmt(f),
                OffchainExchangeCalls::GetMarketInfo(element) => element.fmt(f),
                OffchainExchangeCalls::GetMinSize(element) => element.fmt(f),
                OffchainExchangeCalls::GetOrderFilledAmounts(element) => element.fmt(f),
                OffchainExchangeCalls::GetSizeIncrement(element) => element.fmt(f),
                OffchainExchangeCalls::GetVersion(element) => element.fmt(f),
                OffchainExchangeCalls::GetVirtualBook(element) => element.fmt(f),
                OffchainExchangeCalls::IncrementFees(element) => element.fmt(f),
                OffchainExchangeCalls::Initialize(element) => element.fmt(f),
                OffchainExchangeCalls::MatchOrderAMM(element) => element.fmt(f),
                OffchainExchangeCalls::MatchOrders(element) => element.fmt(f),
                OffchainExchangeCalls::ModifyFilledAmount(element) => element.fmt(f),
                OffchainExchangeCalls::Owner(element) => element.fmt(f),
                OffchainExchangeCalls::RenounceOwnership(element) => element.fmt(f),
                OffchainExchangeCalls::SetEndpoint(element) => element.fmt(f),
                OffchainExchangeCalls::SetFeeRate(element) => element.fmt(f),
                OffchainExchangeCalls::SetFilledAmount(element) => element.fmt(f),
                OffchainExchangeCalls::SwapAMM(element) => element.fmt(f),
                OffchainExchangeCalls::TransferOwnership(element) => element.fmt(f),
                OffchainExchangeCalls::UpdateCollectedFees(element) => element.fmt(f),
                OffchainExchangeCalls::UpdateFeeRates(element) => element.fmt(f),
                OffchainExchangeCalls::UpdateMarket(element) => element.fmt(f),
                OffchainExchangeCalls::UpdateMinSizes(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DropDigestCall> for OffchainExchangeCalls {
        fn from(var: DropDigestCall) -> Self {
            OffchainExchangeCalls::DropDigest(var)
        }
    }
    impl ::std::convert::From<DropOrderCall> for OffchainExchangeCalls {
        fn from(var: DropOrderCall) -> Self {
            OffchainExchangeCalls::DropOrder(var)
        }
    }
    impl ::std::convert::From<DropOrderCheckedCall> for OffchainExchangeCalls {
        fn from(var: DropOrderCheckedCall) -> Self {
            OffchainExchangeCalls::DropOrderChecked(var)
        }
    }
    impl ::std::convert::From<DumpFeesCall> for OffchainExchangeCalls {
        fn from(var: DumpFeesCall) -> Self {
            OffchainExchangeCalls::DumpFees(var)
        }
    }
    impl ::std::convert::From<FilledAmountsCall> for OffchainExchangeCalls {
        fn from(var: FilledAmountsCall) -> Self {
            OffchainExchangeCalls::FilledAmounts(var)
        }
    }
    impl ::std::convert::From<GetAllFeeRatesCall> for OffchainExchangeCalls {
        fn from(var: GetAllFeeRatesCall) -> Self {
            OffchainExchangeCalls::GetAllFeeRates(var)
        }
    }
    impl ::std::convert::From<GetAllVirtualBooksCall> for OffchainExchangeCalls {
        fn from(var: GetAllVirtualBooksCall) -> Self {
            OffchainExchangeCalls::GetAllVirtualBooks(var)
        }
    }
    impl ::std::convert::From<GetCollectedFeesCall> for OffchainExchangeCalls {
        fn from(var: GetCollectedFeesCall) -> Self {
            OffchainExchangeCalls::GetCollectedFees(var)
        }
    }
    impl ::std::convert::From<GetCustomFeeAddressesCall> for OffchainExchangeCalls {
        fn from(var: GetCustomFeeAddressesCall) -> Self {
            OffchainExchangeCalls::GetCustomFeeAddresses(var)
        }
    }
    impl ::std::convert::From<GetDigestCall> for OffchainExchangeCalls {
        fn from(var: GetDigestCall) -> Self {
            OffchainExchangeCalls::GetDigest(var)
        }
    }
    impl ::std::convert::From<GetEndpointCall> for OffchainExchangeCalls {
        fn from(var: GetEndpointCall) -> Self {
            OffchainExchangeCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetFeeFractionX18Call> for OffchainExchangeCalls {
        fn from(var: GetFeeFractionX18Call) -> Self {
            OffchainExchangeCalls::GetFeeFractionX18(var)
        }
    }
    impl ::std::convert::From<GetFeeRatesX18Call> for OffchainExchangeCalls {
        fn from(var: GetFeeRatesX18Call) -> Self {
            OffchainExchangeCalls::GetFeeRatesX18(var)
        }
    }
    impl ::std::convert::From<GetLpParamsCall> for OffchainExchangeCalls {
        fn from(var: GetLpParamsCall) -> Self {
            OffchainExchangeCalls::GetLpParams(var)
        }
    }
    impl ::std::convert::From<GetMarketInfoCall> for OffchainExchangeCalls {
        fn from(var: GetMarketInfoCall) -> Self {
            OffchainExchangeCalls::GetMarketInfo(var)
        }
    }
    impl ::std::convert::From<GetMinSizeCall> for OffchainExchangeCalls {
        fn from(var: GetMinSizeCall) -> Self {
            OffchainExchangeCalls::GetMinSize(var)
        }
    }
    impl ::std::convert::From<GetOrderFilledAmountsCall> for OffchainExchangeCalls {
        fn from(var: GetOrderFilledAmountsCall) -> Self {
            OffchainExchangeCalls::GetOrderFilledAmounts(var)
        }
    }
    impl ::std::convert::From<GetSizeIncrementCall> for OffchainExchangeCalls {
        fn from(var: GetSizeIncrementCall) -> Self {
            OffchainExchangeCalls::GetSizeIncrement(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for OffchainExchangeCalls {
        fn from(var: GetVersionCall) -> Self {
            OffchainExchangeCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<GetVirtualBookCall> for OffchainExchangeCalls {
        fn from(var: GetVirtualBookCall) -> Self {
            OffchainExchangeCalls::GetVirtualBook(var)
        }
    }
    impl ::std::convert::From<IncrementFeesCall> for OffchainExchangeCalls {
        fn from(var: IncrementFeesCall) -> Self {
            OffchainExchangeCalls::IncrementFees(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for OffchainExchangeCalls {
        fn from(var: InitializeCall) -> Self {
            OffchainExchangeCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MatchOrderAMMCall> for OffchainExchangeCalls {
        fn from(var: MatchOrderAMMCall) -> Self {
            OffchainExchangeCalls::MatchOrderAMM(var)
        }
    }
    impl ::std::convert::From<MatchOrdersCall> for OffchainExchangeCalls {
        fn from(var: MatchOrdersCall) -> Self {
            OffchainExchangeCalls::MatchOrders(var)
        }
    }
    impl ::std::convert::From<ModifyFilledAmountCall> for OffchainExchangeCalls {
        fn from(var: ModifyFilledAmountCall) -> Self {
            OffchainExchangeCalls::ModifyFilledAmount(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for OffchainExchangeCalls {
        fn from(var: OwnerCall) -> Self {
            OffchainExchangeCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for OffchainExchangeCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            OffchainExchangeCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for OffchainExchangeCalls {
        fn from(var: SetEndpointCall) -> Self {
            OffchainExchangeCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<SetFeeRateCall> for OffchainExchangeCalls {
        fn from(var: SetFeeRateCall) -> Self {
            OffchainExchangeCalls::SetFeeRate(var)
        }
    }
    impl ::std::convert::From<SetFilledAmountCall> for OffchainExchangeCalls {
        fn from(var: SetFilledAmountCall) -> Self {
            OffchainExchangeCalls::SetFilledAmount(var)
        }
    }
    impl ::std::convert::From<SwapAMMCall> for OffchainExchangeCalls {
        fn from(var: SwapAMMCall) -> Self {
            OffchainExchangeCalls::SwapAMM(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for OffchainExchangeCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            OffchainExchangeCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateCollectedFeesCall> for OffchainExchangeCalls {
        fn from(var: UpdateCollectedFeesCall) -> Self {
            OffchainExchangeCalls::UpdateCollectedFees(var)
        }
    }
    impl ::std::convert::From<UpdateFeeRatesCall> for OffchainExchangeCalls {
        fn from(var: UpdateFeeRatesCall) -> Self {
            OffchainExchangeCalls::UpdateFeeRates(var)
        }
    }
    impl ::std::convert::From<UpdateMarketCall> for OffchainExchangeCalls {
        fn from(var: UpdateMarketCall) -> Self {
            OffchainExchangeCalls::UpdateMarket(var)
        }
    }
    impl ::std::convert::From<UpdateMinSizesCall> for OffchainExchangeCalls {
        fn from(var: UpdateMinSizesCall) -> Self {
            OffchainExchangeCalls::UpdateMinSizes(var)
        }
    }
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
    #[doc = "Container type for all return fields from the `getAllFeeRates` function with signature `getAllFeeRates(address[],uint32[])` and selector `[216, 149, 32, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAllFeeRatesReturn(pub ::std::vec::Vec<FeeRates>);
    #[doc = "Container type for all return fields from the `getAllVirtualBooks` function with signature `getAllVirtualBooks()` and selector `[206, 147, 62, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAllVirtualBooksReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `[255, 11, 233, 239]`"]
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
    #[doc = "Container type for all return fields from the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `[63, 206, 234, 40]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCustomFeeAddressesReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `[149, 238, 96, 113]`"]
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
    #[doc = "Container type for all return fields from the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `[181, 203, 215, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFeeFractionX18Return(pub i128);
    #[doc = "Container type for all return fields from the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `[15, 44, 135, 142]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFeeRatesX18Return(pub i128, pub i128);
    #[doc = "Container type for all return fields from the `getLpParams` function with signature `getLpParams(uint32)` and selector `[72, 33, 200, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLpParamsReturn(pub LpParams);
    #[doc = "Container type for all return fields from the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `[29, 2, 155, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMarketInfoReturn {
        pub m: MarketInfo,
    }
    #[doc = "Container type for all return fields from the `getMinSize` function with signature `getMinSize(uint32)` and selector `[182, 10, 170, 124]`"]
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
    #[doc = "Container type for all return fields from the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `[163, 155, 157, 205]`"]
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
    #[doc = "Container type for all return fields from the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `[242, 178, 99, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSizeIncrementReturn(pub i128);
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
    #[doc = "Container type for all return fields from the `getVirtualBook` function with signature `getVirtualBook(uint32)` and selector `[102, 248, 123, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetVirtualBookReturn(pub ethers::core::types::Address);
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
    #[doc = "`MatchOrders(uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`"]
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
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    #[doc = "`MatchOrdersWithSigner((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address)`"]
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
    #[doc = "`FeeRates(int64,int64,uint8)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FeeRates {
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
        pub is_non_default: u8,
    }
    #[doc = "`LpParams(int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LpParams {
        pub lp_spread_x18: i128,
    }
    #[doc = "`MarketInfo(int128,int128,int128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MarketInfo {
        pub min_size: i128,
        pub size_increment: i128,
        pub collected_fees: i128,
    }
}
