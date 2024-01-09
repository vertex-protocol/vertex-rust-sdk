pub use fee_calculator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod fee_calculator {
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
    #[doc = "FeeCalculator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClearinghouse\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"taker\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeFractionX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getInterestFeeFractionX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getLiquidationFeeFractionX18\",\"outputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_clearinghouse\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"subaccount\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"quoteVolume\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"recordVolume\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"productId\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"makerRateX18\",\"type\":\"int64\",\"components\":[]},{\"internalType\":\"int64\",\"name\":\"takerRateX18\",\"type\":\"int64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFeeRates\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static FEECALCULATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct FeeCalculator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FeeCalculator<M> {
        fn clone(&self) -> Self {
            FeeCalculator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FeeCalculator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for FeeCalculator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FeeCalculator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FeeCalculator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), FEECALCULATOR_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getClearinghouse` (0xb1cb0f42) function"]
        pub fn get_clearinghouse(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([177, 203, 15, 66], ())
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
        #[doc = "Calls the contract's `getInterestFeeFractionX18` (0xf82ce57d) function"]
        pub fn get_interest_fee_fraction_x18(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([248, 44, 229, 125], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidationFeeFractionX18` (0x71bb043a) function"]
        pub fn get_liquidation_fee_fraction_x18(
            &self,
            p0: [u8; 32],
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([113, 187, 4, 58], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x8129fc1c) function"]
        pub fn initialize(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrate` (0xce5494bb) function"]
        pub fn migrate(
            &self,
            clearinghouse: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 84, 148, 187], clearinghouse)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recordVolume` (0xe88d7946) function"]
        pub fn record_volume(
            &self,
            subaccount: [u8; 32],
            quote_volume: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 141, 121, 70], (subaccount, quote_volume))
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
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for FeeCalculator<M> {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[doc = "Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `[177, 203, 15, 66]`"]
    #[derive(
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
    #[doc = "Container type for all input parameters for the `getInterestFeeFractionX18` function with signature `getInterestFeeFractionX18(uint32)` and selector `[248, 44, 229, 125]`"]
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
        name = "getInterestFeeFractionX18",
        abi = "getInterestFeeFractionX18(uint32)"
    )]
    pub struct GetInterestFeeFractionX18Call(pub u32);
    #[doc = "Container type for all input parameters for the `getLiquidationFeeFractionX18` function with signature `getLiquidationFeeFractionX18(bytes32,uint32)` and selector `[113, 187, 4, 58]`"]
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
        name = "getLiquidationFeeFractionX18",
        abi = "getLiquidationFeeFractionX18(bytes32,uint32)"
    )]
    pub struct GetLiquidationFeeFractionX18Call(pub [u8; 32], pub u32);
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `[129, 41, 252, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    #[doc = "Container type for all input parameters for the `migrate` function with signature `migrate(address)` and selector `[206, 84, 148, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "migrate", abi = "migrate(address)")]
    pub struct MigrateCall {
        pub clearinghouse: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `recordVolume` function with signature `recordVolume(bytes32,uint128)` and selector `[232, 141, 121, 70]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "recordVolume", abi = "recordVolume(bytes32,uint128)")]
    pub struct RecordVolumeCall {
        pub subaccount: [u8; 32],
        pub quote_volume: u128,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FeeCalculatorCalls {
        GetClearinghouse(GetClearinghouseCall),
        GetFeeFractionX18(GetFeeFractionX18Call),
        GetInterestFeeFractionX18(GetInterestFeeFractionX18Call),
        GetLiquidationFeeFractionX18(GetLiquidationFeeFractionX18Call),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
        Migrate(MigrateCall),
        RecordVolume(RecordVolumeCall),
        UpdateFeeRates(UpdateFeeRatesCall),
    }
    impl ethers::core::abi::AbiDecode for FeeCalculatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetClearinghouseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::GetClearinghouse(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFractionX18Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::GetFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetInterestFeeFractionX18Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FeeCalculatorCalls::GetInterestFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationFeeFractionX18Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FeeCalculatorCalls::GetLiquidationFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <RecordVolumeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::RecordVolume(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FeeCalculatorCalls::UpdateFeeRates(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FeeCalculatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FeeCalculatorCalls::GetClearinghouse(element) => element.encode(),
                FeeCalculatorCalls::GetFeeFractionX18(element) => element.encode(),
                FeeCalculatorCalls::GetInterestFeeFractionX18(element) => element.encode(),
                FeeCalculatorCalls::GetLiquidationFeeFractionX18(element) => element.encode(),
                FeeCalculatorCalls::GetVersion(element) => element.encode(),
                FeeCalculatorCalls::Initialize(element) => element.encode(),
                FeeCalculatorCalls::Migrate(element) => element.encode(),
                FeeCalculatorCalls::RecordVolume(element) => element.encode(),
                FeeCalculatorCalls::UpdateFeeRates(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FeeCalculatorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FeeCalculatorCalls::GetClearinghouse(element) => element.fmt(f),
                FeeCalculatorCalls::GetFeeFractionX18(element) => element.fmt(f),
                FeeCalculatorCalls::GetInterestFeeFractionX18(element) => element.fmt(f),
                FeeCalculatorCalls::GetLiquidationFeeFractionX18(element) => element.fmt(f),
                FeeCalculatorCalls::GetVersion(element) => element.fmt(f),
                FeeCalculatorCalls::Initialize(element) => element.fmt(f),
                FeeCalculatorCalls::Migrate(element) => element.fmt(f),
                FeeCalculatorCalls::RecordVolume(element) => element.fmt(f),
                FeeCalculatorCalls::UpdateFeeRates(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetClearinghouseCall> for FeeCalculatorCalls {
        fn from(var: GetClearinghouseCall) -> Self {
            FeeCalculatorCalls::GetClearinghouse(var)
        }
    }
    impl ::std::convert::From<GetFeeFractionX18Call> for FeeCalculatorCalls {
        fn from(var: GetFeeFractionX18Call) -> Self {
            FeeCalculatorCalls::GetFeeFractionX18(var)
        }
    }
    impl ::std::convert::From<GetInterestFeeFractionX18Call> for FeeCalculatorCalls {
        fn from(var: GetInterestFeeFractionX18Call) -> Self {
            FeeCalculatorCalls::GetInterestFeeFractionX18(var)
        }
    }
    impl ::std::convert::From<GetLiquidationFeeFractionX18Call> for FeeCalculatorCalls {
        fn from(var: GetLiquidationFeeFractionX18Call) -> Self {
            FeeCalculatorCalls::GetLiquidationFeeFractionX18(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for FeeCalculatorCalls {
        fn from(var: GetVersionCall) -> Self {
            FeeCalculatorCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for FeeCalculatorCalls {
        fn from(var: InitializeCall) -> Self {
            FeeCalculatorCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for FeeCalculatorCalls {
        fn from(var: MigrateCall) -> Self {
            FeeCalculatorCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<RecordVolumeCall> for FeeCalculatorCalls {
        fn from(var: RecordVolumeCall) -> Self {
            FeeCalculatorCalls::RecordVolume(var)
        }
    }
    impl ::std::convert::From<UpdateFeeRatesCall> for FeeCalculatorCalls {
        fn from(var: UpdateFeeRatesCall) -> Self {
            FeeCalculatorCalls::UpdateFeeRates(var)
        }
    }
    #[doc = "Container type for all return fields from the `getClearinghouse` function with signature `getClearinghouse()` and selector `[177, 203, 15, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetClearinghouseReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `getInterestFeeFractionX18` function with signature `getInterestFeeFractionX18(uint32)` and selector `[248, 44, 229, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInterestFeeFractionX18Return(pub i128);
    #[doc = "Container type for all return fields from the `getLiquidationFeeFractionX18` function with signature `getLiquidationFeeFractionX18(bytes32,uint32)` and selector `[113, 187, 4, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLiquidationFeeFractionX18Return(pub i128);
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
}
