pub use mock_feed_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_feed_contract {
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
    #[doc = "MockFeedContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"dec\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"desc\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endedAt\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRound\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"v\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MOCKFEEDCONTRACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MockFeedContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockFeedContract<M> {
        fn clone(&self) -> Self {
            MockFeedContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockFeedContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MockFeedContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockFeedContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockFeedContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKFEEDCONTRACT_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRound` (0x9d250df8) function"]
        pub fn set_round(
            &self,
            round_id: u128,
            answer: I256,
            started_at: ethers::core::types::U256,
            ended_at: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 37, 13, 248], (round_id, answer, started_at, ended_at))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockFeedContract<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `description` function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    #[doc = "Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    #[doc = "Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `setRound` function with signature `setRound(uint80,int256,uint256,uint256)` and selector `[157, 37, 13, 248]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRound", abi = "setRound(uint80,int256,uint256,uint256)")]
    pub struct SetRoundCall {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub ended_at: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockFeedContractCalls {
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        SetRound(SetRoundCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for MockFeedContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <SetRoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::SetRound(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFeedContractCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockFeedContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockFeedContractCalls::Decimals(element) => element.encode(),
                MockFeedContractCalls::Description(element) => element.encode(),
                MockFeedContractCalls::GetRoundData(element) => element.encode(),
                MockFeedContractCalls::LatestRoundData(element) => element.encode(),
                MockFeedContractCalls::SetRound(element) => element.encode(),
                MockFeedContractCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockFeedContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFeedContractCalls::Decimals(element) => element.fmt(f),
                MockFeedContractCalls::Description(element) => element.fmt(f),
                MockFeedContractCalls::GetRoundData(element) => element.fmt(f),
                MockFeedContractCalls::LatestRoundData(element) => element.fmt(f),
                MockFeedContractCalls::SetRound(element) => element.fmt(f),
                MockFeedContractCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DecimalsCall> for MockFeedContractCalls {
        fn from(var: DecimalsCall) -> Self {
            MockFeedContractCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for MockFeedContractCalls {
        fn from(var: DescriptionCall) -> Self {
            MockFeedContractCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for MockFeedContractCalls {
        fn from(var: GetRoundDataCall) -> Self {
            MockFeedContractCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for MockFeedContractCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            MockFeedContractCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<SetRoundCall> for MockFeedContractCalls {
        fn from(var: SetRoundCall) -> Self {
            MockFeedContractCalls::SetRound(var)
        }
    }
    impl ::std::convert::From<VersionCall> for MockFeedContractCalls {
        fn from(var: VersionCall) -> Self {
            MockFeedContractCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn {
        pub dec: u8,
    }
    #[doc = "Container type for all return fields from the `description` function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DescriptionReturn {
        pub desc: String,
    }
    #[doc = "Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn {
        pub v: ethers::core::types::U256,
    }
}
