pub use clearinghouse_liq::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod clearinghouse_liq {
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
    #[doc = "ClearinghouseLiq was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtNegativeInput\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"x\",\"type\":\"int256\",\"components\":[]}],\"type\":\"error\",\"name\":\"PRBMathSD59x18__SqrtOverflow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"endpoint\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"quote\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"fees\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ClearinghouseInitialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"liquidatorSubaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"liquidateeSubaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"amountQuote\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"int128\",\"name\":\"insuranceCover\",\"type\":\"int128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Liquidation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ModifyCollateral\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEndpoint\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxHealthGroup\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePriceX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclePricesX18\",\"outputs\":[{\"internalType\":\"struct IEndpoint.Prices\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"spotPriceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"perpPriceX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRisk\",\"outputs\":[{\"internalType\":\"struct RiskHelper.Risk\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int128\",\"name\":\"longWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightInitialX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"longWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"shortWeightMaintenanceX18\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"largePositionPenaltyX18\",\"type\":\"int128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"insurance\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAboveInitial\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isUnderInitial\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IEndpoint.LiquidateSubaccount\",\"name\":\"txn\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"sender\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"liquidatee\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"healthGroup\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"amount\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateSubaccountImpl\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_endpoint\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CLEARINGHOUSELIQ_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ClearinghouseLiq<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ClearinghouseLiq<M> {
        fn clone(&self) -> Self {
            ClearinghouseLiq(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ClearinghouseLiq<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ClearinghouseLiq<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ClearinghouseLiq))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ClearinghouseLiq<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CLEARINGHOUSELIQ_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getEndpoint` (0xaed8e967) function"]
        pub fn get_endpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([174, 216, 233, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMaxHealthGroup` (0xf8fac037) function"]
        pub fn get_max_health_group(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([248, 250, 192, 55], ())
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
        #[doc = "Calls the contract's `liquidateSubaccountImpl` (0xf8f0722d) function"]
        pub fn liquidate_subaccount_impl(
            &self,
            txn: LiquidateSubaccount,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 240, 114, 45], (txn,))
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ClearinghouseLiqEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ClearinghouseLiq<M> {
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
    pub enum ClearinghouseLiqErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
    }
    impl ethers::core::abi::AbiDecode for ClearinghouseLiqErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ClearinghouseLiqErrors::PRBMathSD59x18__SqrtNegativeInput(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ClearinghouseLiqErrors::PRBMathSD59x18__SqrtOverflow(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ClearinghouseLiqErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ClearinghouseLiqErrors::PRBMathSD59x18__SqrtNegativeInput(element) => {
                    element.encode()
                }
                ClearinghouseLiqErrors::PRBMathSD59x18__SqrtOverflow(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ClearinghouseLiqErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseLiqErrors::PRBMathSD59x18__SqrtNegativeInput(element) => {
                    element.fmt(f)
                }
                ClearinghouseLiqErrors::PRBMathSD59x18__SqrtOverflow(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtNegativeInput> for ClearinghouseLiqErrors {
        fn from(var: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            ClearinghouseLiqErrors::PRBMathSD59x18__SqrtNegativeInput(var)
        }
    }
    impl ::std::convert::From<PRBMathSD59x18__SqrtOverflow> for ClearinghouseLiqErrors {
        fn from(var: PRBMathSD59x18__SqrtOverflow) -> Self {
            ClearinghouseLiqErrors::PRBMathSD59x18__SqrtOverflow(var)
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
    pub enum ClearinghouseLiqEvents {
        ClearinghouseInitializedFilter(ClearinghouseInitializedFilter),
        InitializedFilter(InitializedFilter),
        LiquidationFilter(LiquidationFilter),
        ModifyCollateralFilter(ModifyCollateralFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for ClearinghouseLiqEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ClearinghouseInitializedFilter::decode_log(log) {
                return Ok(ClearinghouseLiqEvents::ClearinghouseInitializedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ClearinghouseLiqEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationFilter::decode_log(log) {
                return Ok(ClearinghouseLiqEvents::LiquidationFilter(decoded));
            }
            if let Ok(decoded) = ModifyCollateralFilter::decode_log(log) {
                return Ok(ClearinghouseLiqEvents::ModifyCollateralFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ClearinghouseLiqEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ClearinghouseLiqEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseLiqEvents::ClearinghouseInitializedFilter(element) => element.fmt(f),
                ClearinghouseLiqEvents::InitializedFilter(element) => element.fmt(f),
                ClearinghouseLiqEvents::LiquidationFilter(element) => element.fmt(f),
                ClearinghouseLiqEvents::ModifyCollateralFilter(element) => element.fmt(f),
                ClearinghouseLiqEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ClearinghouseLiqCalls {
        GetEndpoint(GetEndpointCall),
        GetMaxHealthGroup(GetMaxHealthGroupCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetRisk(GetRiskCall),
        GetVersion(GetVersionCall),
        Insurance(InsuranceCall),
        IsAboveInitial(IsAboveInitialCall),
        IsUnderInitial(IsUnderInitialCall),
        LiquidateSubaccountImpl(LiquidateSubaccountImplCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetEndpoint(SetEndpointCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ClearinghouseLiqCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetMaxHealthGroupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetMaxHealthGroup(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePriceX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePricesX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) =
                <GetRiskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetRisk(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <InsuranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::Insurance(decoded));
            }
            if let Ok(decoded) =
                <IsAboveInitialCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::IsAboveInitial(decoded));
            }
            if let Ok(decoded) =
                <IsUnderInitialCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::IsUnderInitial(decoded));
            }
            if let Ok(decoded) =
                <LiquidateSubaccountImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::LiquidateSubaccountImpl(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetEndpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::SetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ClearinghouseLiqCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ClearinghouseLiqCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ClearinghouseLiqCalls::GetEndpoint(element) => element.encode(),
                ClearinghouseLiqCalls::GetMaxHealthGroup(element) => element.encode(),
                ClearinghouseLiqCalls::GetOraclePriceX18(element) => element.encode(),
                ClearinghouseLiqCalls::GetOraclePricesX18(element) => element.encode(),
                ClearinghouseLiqCalls::GetRisk(element) => element.encode(),
                ClearinghouseLiqCalls::GetVersion(element) => element.encode(),
                ClearinghouseLiqCalls::Insurance(element) => element.encode(),
                ClearinghouseLiqCalls::IsAboveInitial(element) => element.encode(),
                ClearinghouseLiqCalls::IsUnderInitial(element) => element.encode(),
                ClearinghouseLiqCalls::LiquidateSubaccountImpl(element) => element.encode(),
                ClearinghouseLiqCalls::Owner(element) => element.encode(),
                ClearinghouseLiqCalls::RenounceOwnership(element) => element.encode(),
                ClearinghouseLiqCalls::SetEndpoint(element) => element.encode(),
                ClearinghouseLiqCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ClearinghouseLiqCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ClearinghouseLiqCalls::GetEndpoint(element) => element.fmt(f),
                ClearinghouseLiqCalls::GetMaxHealthGroup(element) => element.fmt(f),
                ClearinghouseLiqCalls::GetOraclePriceX18(element) => element.fmt(f),
                ClearinghouseLiqCalls::GetOraclePricesX18(element) => element.fmt(f),
                ClearinghouseLiqCalls::GetRisk(element) => element.fmt(f),
                ClearinghouseLiqCalls::GetVersion(element) => element.fmt(f),
                ClearinghouseLiqCalls::Insurance(element) => element.fmt(f),
                ClearinghouseLiqCalls::IsAboveInitial(element) => element.fmt(f),
                ClearinghouseLiqCalls::IsUnderInitial(element) => element.fmt(f),
                ClearinghouseLiqCalls::LiquidateSubaccountImpl(element) => element.fmt(f),
                ClearinghouseLiqCalls::Owner(element) => element.fmt(f),
                ClearinghouseLiqCalls::RenounceOwnership(element) => element.fmt(f),
                ClearinghouseLiqCalls::SetEndpoint(element) => element.fmt(f),
                ClearinghouseLiqCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetEndpointCall> for ClearinghouseLiqCalls {
        fn from(var: GetEndpointCall) -> Self {
            ClearinghouseLiqCalls::GetEndpoint(var)
        }
    }
    impl ::std::convert::From<GetMaxHealthGroupCall> for ClearinghouseLiqCalls {
        fn from(var: GetMaxHealthGroupCall) -> Self {
            ClearinghouseLiqCalls::GetMaxHealthGroup(var)
        }
    }
    impl ::std::convert::From<GetOraclePriceX18Call> for ClearinghouseLiqCalls {
        fn from(var: GetOraclePriceX18Call) -> Self {
            ClearinghouseLiqCalls::GetOraclePriceX18(var)
        }
    }
    impl ::std::convert::From<GetOraclePricesX18Call> for ClearinghouseLiqCalls {
        fn from(var: GetOraclePricesX18Call) -> Self {
            ClearinghouseLiqCalls::GetOraclePricesX18(var)
        }
    }
    impl ::std::convert::From<GetRiskCall> for ClearinghouseLiqCalls {
        fn from(var: GetRiskCall) -> Self {
            ClearinghouseLiqCalls::GetRisk(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for ClearinghouseLiqCalls {
        fn from(var: GetVersionCall) -> Self {
            ClearinghouseLiqCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<InsuranceCall> for ClearinghouseLiqCalls {
        fn from(var: InsuranceCall) -> Self {
            ClearinghouseLiqCalls::Insurance(var)
        }
    }
    impl ::std::convert::From<IsAboveInitialCall> for ClearinghouseLiqCalls {
        fn from(var: IsAboveInitialCall) -> Self {
            ClearinghouseLiqCalls::IsAboveInitial(var)
        }
    }
    impl ::std::convert::From<IsUnderInitialCall> for ClearinghouseLiqCalls {
        fn from(var: IsUnderInitialCall) -> Self {
            ClearinghouseLiqCalls::IsUnderInitial(var)
        }
    }
    impl ::std::convert::From<LiquidateSubaccountImplCall> for ClearinghouseLiqCalls {
        fn from(var: LiquidateSubaccountImplCall) -> Self {
            ClearinghouseLiqCalls::LiquidateSubaccountImpl(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ClearinghouseLiqCalls {
        fn from(var: OwnerCall) -> Self {
            ClearinghouseLiqCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ClearinghouseLiqCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ClearinghouseLiqCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetEndpointCall> for ClearinghouseLiqCalls {
        fn from(var: SetEndpointCall) -> Self {
            ClearinghouseLiqCalls::SetEndpoint(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ClearinghouseLiqCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ClearinghouseLiqCalls::TransferOwnership(var)
        }
    }
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
