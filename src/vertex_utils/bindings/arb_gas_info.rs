pub use arb_gas_info::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod arb_gas_info {
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
    #[doc = "ArbGasInfo was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getL1BaseFeeEstimate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getPricesInWei\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ARBGASINFO_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ArbGasInfo<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ArbGasInfo<M> {
        fn clone(&self) -> Self {
            ArbGasInfo(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ArbGasInfo<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ArbGasInfo<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ArbGasInfo))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ArbGasInfo<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ARBGASINFO_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getL1BaseFeeEstimate` (0xf5d6ded7) function"]
        pub fn get_l1_base_fee_estimate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 214, 222, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPricesInWei` (0x41b247a8) function"]
        pub fn get_prices_in_wei(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([65, 178, 71, 168], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ArbGasInfo<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getL1BaseFeeEstimate` function with signature `getL1BaseFeeEstimate()` and selector `[245, 214, 222, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getL1BaseFeeEstimate", abi = "getL1BaseFeeEstimate()")]
    pub struct GetL1BaseFeeEstimateCall;
    #[doc = "Container type for all input parameters for the `getPricesInWei` function with signature `getPricesInWei()` and selector `[65, 178, 71, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPricesInWei", abi = "getPricesInWei()")]
    pub struct GetPricesInWeiCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ArbGasInfoCalls {
        GetL1BaseFeeEstimate(GetL1BaseFeeEstimateCall),
        GetPricesInWei(GetPricesInWeiCall),
    }
    impl ethers::core::abi::AbiDecode for ArbGasInfoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetL1BaseFeeEstimateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ArbGasInfoCalls::GetL1BaseFeeEstimate(decoded));
            }
            if let Ok(decoded) =
                <GetPricesInWeiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ArbGasInfoCalls::GetPricesInWei(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ArbGasInfoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ArbGasInfoCalls::GetL1BaseFeeEstimate(element) => element.encode(),
                ArbGasInfoCalls::GetPricesInWei(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ArbGasInfoCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ArbGasInfoCalls::GetL1BaseFeeEstimate(element) => element.fmt(f),
                ArbGasInfoCalls::GetPricesInWei(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetL1BaseFeeEstimateCall> for ArbGasInfoCalls {
        fn from(var: GetL1BaseFeeEstimateCall) -> Self {
            ArbGasInfoCalls::GetL1BaseFeeEstimate(var)
        }
    }
    impl ::std::convert::From<GetPricesInWeiCall> for ArbGasInfoCalls {
        fn from(var: GetPricesInWeiCall) -> Self {
            ArbGasInfoCalls::GetPricesInWei(var)
        }
    }
    #[doc = "Container type for all return fields from the `getL1BaseFeeEstimate` function with signature `getL1BaseFeeEstimate()` and selector `[245, 214, 222, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetL1BaseFeeEstimateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPricesInWei` function with signature `getPricesInWei()` and selector `[65, 178, 71, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPricesInWeiReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
}
