pub use verifier::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod verifier {
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
    #[doc = "Verifier was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"Q\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_P\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"assignPubKey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deletePubkey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"index\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPubkey\",\"outputs\":[{\"internalType\":\"struct Verifier.Point\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getVersion\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Verifier.Point[8]\",\"name\":\"initialSet\",\"type\":\"tuple[8]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"message\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"signerBitmask\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requireValidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasUsed\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"revertGasInfo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static VERIFIER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Verifier<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Verifier<M> {
        fn clone(&self) -> Self {
            Verifier(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Verifier<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Verifier<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Verifier<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VERIFIER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `Q` (0xe493ef8c) function"]
        pub fn q(&self) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([228, 147, 239, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_P` (0x354f622a) function"]
        pub fn p(&self) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([53, 79, 98, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assignPubKey` (0xbe13bac4) function"]
        pub fn assign_pub_key(
            &self,
            i: ethers::core::types::U256,
            x: ethers::core::types::U256,
            y: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 19, 186, 196], (i, x, y))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deletePubkey` (0xbbef84b4) function"]
        pub fn delete_pubkey(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 239, 132, 180], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPubkey` (0x55e7673b) function"]
        pub fn get_pubkey(&self, index: u8) -> ethers::contract::builders::ContractCall<M, Point> {
            self.0
                .method_hash([85, 231, 103, 59], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVersion` (0x0d8e6e2c) function"]
        pub fn get_version(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x5fcb7d58) function"]
        pub fn initialize(
            &self,
            initial_set: [Point; 8usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 203, 125, 88], initial_set)
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
        #[doc = "Calls the contract's `requireValidSignature` (0x2ac478b6) function"]
        pub fn require_valid_signature(
            &self,
            message: [u8; 32],
            e: [u8; 32],
            s: [u8; 32],
            signer_bitmask: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 196, 120, 182], (message, e, s, signer_bitmask))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revertGasInfo` (0x3c64c215) function"]
        pub fn revert_gas_info(
            &self,
            i: ethers::core::types::U256,
            gas_used: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 100, 194, 21], (i, gas_used))
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, VerifierEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Verifier<M> {
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
    pub enum VerifierEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for VerifierEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VerifierEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VerifierEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VerifierEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VerifierEvents::InitializedFilter(element) => element.fmt(f),
                VerifierEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `Q` function with signature `Q()` and selector `[228, 147, 239, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "Q", abi = "Q()")]
    pub struct QCall;
    #[doc = "Container type for all input parameters for the `_P` function with signature `_P()` and selector `[53, 79, 98, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "_P", abi = "_P()")]
    pub struct PCall;
    #[doc = "Container type for all input parameters for the `assignPubKey` function with signature `assignPubKey(uint256,uint256,uint256)` and selector `[190, 19, 186, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "assignPubKey", abi = "assignPubKey(uint256,uint256,uint256)")]
    pub struct AssignPubKeyCall {
        pub i: ethers::core::types::U256,
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deletePubkey` function with signature `deletePubkey(uint256)` and selector `[187, 239, 132, 180]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deletePubkey", abi = "deletePubkey(uint256)")]
    pub struct DeletePubkeyCall {
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPubkey` function with signature `getPubkey(uint8)` and selector `[85, 231, 103, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPubkey", abi = "getPubkey(uint8)")]
    pub struct GetPubkeyCall {
        pub index: u8,
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize((uint256,uint256)[8])` and selector `[95, 203, 125, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize((uint256,uint256)[8])")]
    pub struct InitializeCall {
        pub initial_set: [Point; 8usize],
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
    #[doc = "Container type for all input parameters for the `requireValidSignature` function with signature `requireValidSignature(bytes32,bytes32,bytes32,uint8)` and selector `[42, 196, 120, 182]`"]
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
        name = "requireValidSignature",
        abi = "requireValidSignature(bytes32,bytes32,bytes32,uint8)"
    )]
    pub struct RequireValidSignatureCall {
        pub message: [u8; 32],
        pub e: [u8; 32],
        pub s: [u8; 32],
        pub signer_bitmask: u8,
    }
    #[doc = "Container type for all input parameters for the `revertGasInfo` function with signature `revertGasInfo(uint256,uint256)` and selector `[60, 100, 194, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "revertGasInfo", abi = "revertGasInfo(uint256,uint256)")]
    pub struct RevertGasInfoCall {
        pub i: ethers::core::types::U256,
        pub gas_used: ethers::core::types::U256,
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
    pub enum VerifierCalls {
        Q(QCall),
        P(PCall),
        AssignPubKey(AssignPubKeyCall),
        DeletePubkey(DeletePubkeyCall),
        GetPubkey(GetPubkeyCall),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequireValidSignature(RequireValidSignatureCall),
        RevertGasInfo(RevertGasInfoCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for VerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <QCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VerifierCalls::Q(decoded));
            }
            if let Ok(decoded) = <PCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VerifierCalls::P(decoded));
            }
            if let Ok(decoded) =
                <AssignPubKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <DeletePubkeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::DeletePubkey(decoded));
            }
            if let Ok(decoded) =
                <GetPubkeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::GetPubkey(decoded));
            }
            if let Ok(decoded) =
                <GetVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequireValidSignatureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::RequireValidSignature(decoded));
            }
            if let Ok(decoded) =
                <RevertGasInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::RevertGasInfo(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VerifierCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VerifierCalls::Q(element) => element.encode(),
                VerifierCalls::P(element) => element.encode(),
                VerifierCalls::AssignPubKey(element) => element.encode(),
                VerifierCalls::DeletePubkey(element) => element.encode(),
                VerifierCalls::GetPubkey(element) => element.encode(),
                VerifierCalls::GetVersion(element) => element.encode(),
                VerifierCalls::Initialize(element) => element.encode(),
                VerifierCalls::Owner(element) => element.encode(),
                VerifierCalls::RenounceOwnership(element) => element.encode(),
                VerifierCalls::RequireValidSignature(element) => element.encode(),
                VerifierCalls::RevertGasInfo(element) => element.encode(),
                VerifierCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VerifierCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VerifierCalls::Q(element) => element.fmt(f),
                VerifierCalls::P(element) => element.fmt(f),
                VerifierCalls::AssignPubKey(element) => element.fmt(f),
                VerifierCalls::DeletePubkey(element) => element.fmt(f),
                VerifierCalls::GetPubkey(element) => element.fmt(f),
                VerifierCalls::GetVersion(element) => element.fmt(f),
                VerifierCalls::Initialize(element) => element.fmt(f),
                VerifierCalls::Owner(element) => element.fmt(f),
                VerifierCalls::RenounceOwnership(element) => element.fmt(f),
                VerifierCalls::RequireValidSignature(element) => element.fmt(f),
                VerifierCalls::RevertGasInfo(element) => element.fmt(f),
                VerifierCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<QCall> for VerifierCalls {
        fn from(var: QCall) -> Self {
            VerifierCalls::Q(var)
        }
    }
    impl ::std::convert::From<PCall> for VerifierCalls {
        fn from(var: PCall) -> Self {
            VerifierCalls::P(var)
        }
    }
    impl ::std::convert::From<AssignPubKeyCall> for VerifierCalls {
        fn from(var: AssignPubKeyCall) -> Self {
            VerifierCalls::AssignPubKey(var)
        }
    }
    impl ::std::convert::From<DeletePubkeyCall> for VerifierCalls {
        fn from(var: DeletePubkeyCall) -> Self {
            VerifierCalls::DeletePubkey(var)
        }
    }
    impl ::std::convert::From<GetPubkeyCall> for VerifierCalls {
        fn from(var: GetPubkeyCall) -> Self {
            VerifierCalls::GetPubkey(var)
        }
    }
    impl ::std::convert::From<GetVersionCall> for VerifierCalls {
        fn from(var: GetVersionCall) -> Self {
            VerifierCalls::GetVersion(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VerifierCalls {
        fn from(var: InitializeCall) -> Self {
            VerifierCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for VerifierCalls {
        fn from(var: OwnerCall) -> Self {
            VerifierCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for VerifierCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            VerifierCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RequireValidSignatureCall> for VerifierCalls {
        fn from(var: RequireValidSignatureCall) -> Self {
            VerifierCalls::RequireValidSignature(var)
        }
    }
    impl ::std::convert::From<RevertGasInfoCall> for VerifierCalls {
        fn from(var: RevertGasInfoCall) -> Self {
            VerifierCalls::RevertGasInfo(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for VerifierCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            VerifierCalls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `Q` function with signature `Q()` and selector `[228, 147, 239, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_P` function with signature `_P()` and selector `[53, 79, 98, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPubkey` function with signature `getPubkey(uint8)` and selector `[85, 231, 103, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPubkeyReturn(pub Point);
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
    #[doc = "`Point(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Point {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
}
