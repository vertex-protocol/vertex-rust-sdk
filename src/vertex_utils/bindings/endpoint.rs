pub use endpoint::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod endpoint {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("burnLpAndTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("burnLpAndTransfer"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.BurnLpAndTransfer",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.BurnLpAndTransfer",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainlinkFullReport"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainlinkFullReport",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                    3usize,
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkFullReport",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                    3usize,
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkFullReport",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainlinkReportBlob"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainlinkReportBlob",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkReportBlob",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkReportBlob",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkLpAction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkLpAction"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkLpActionInner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkLpActionInner"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSlowModeTxInner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSlowModeTxInner",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSlowModeTxLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSlowModeTxLinkSigner",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimSequencerFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("claimSequencerFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.ClaimSequencerFees",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.ClaimSequencerFees",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearinghouse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearinghouse"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccountName"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(12usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes12"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateralWithReferral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositCollateralWithReferral",),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositCollateralWithReferral",),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subaccountName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        12usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes12"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeSlowModeTransaction",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHealthCheckFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHealthCheckFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLinkedSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLinkedSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidationFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLiquidationFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumSubaccounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNumSubaccounts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOffchainExchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOffchainExchange",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPriceX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceX18"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_priceX18"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSequencer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSequencer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSlowModeTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlowModeTx"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("idx"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeTx",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubaccountById"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubaccountById"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccountId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubaccountId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubaccountId"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTakerSequencerFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTakerSequencerFee",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTime"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTime"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTimes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTimes"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Endpoint.Times"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVersion"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementSubmissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("incrementSubmissions",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sanctions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sequencer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_offchainExchange"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_verifier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialPrices"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("legacyMatchOrders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("legacyMatchOrders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacyMatchOrders",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacyMatchOrders",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("legacySignedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("legacySignedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacySignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacySignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("legacySpotTick"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("legacySpotTick"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LegacySpotTick",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LegacySpotTick",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("legacyUnsignedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("legacyUnsignedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacyLiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LegacyLiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidationStart"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidationStart"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transaction"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("manualAssert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("manualAssert"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.ManualAssert",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.ManualAssert",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrderAMM",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrderAMM",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrders",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrders",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nSubmissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nSubmissions"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("perpTick"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpTick"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.PerpTick",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.PerpTick",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processSlowModeTransaction",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.RebalanceXWithdraw",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.RebalanceXWithdraw",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.Rebate"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.Rebate"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("referralCodes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("referralCodes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerTransferableWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerTransferableWallet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_transferable"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resyncSlowModeTxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resyncSlowModeTxs"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPriceX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPriceX18"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_priceX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSlowModeConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSlowModeConfig"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_slowModeConfig"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeConfig",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSlowModeTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSlowModeTx"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeTx",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("settlePnl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("settlePnl"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SettlePnl",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SettlePnl",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedBurnLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedBurnLp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnLp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnLp",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedCancellation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedCancellation"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellation",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellation",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedCancellationProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedCancellationProducts",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellationProducts",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellationProducts",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedLinkSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLinkSigner",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLinkSigner",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedMintLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedMintLp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintLp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintLp",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedOrder"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedOrder",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedOrder",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedTransferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedTransferQuote",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedTransferQuote",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedTransferQuote",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedWithdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedWithdrawCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedWithdrawCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedWithdrawCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("spotTick"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("spotTick"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SpotTick",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SpotTick",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSlowModeTransaction",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transaction"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitTransactions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transactions"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactionsChecked"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitTransactionsChecked",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transactions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("e"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signerBitmask"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactionsCheckedWithGasLimit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "submitTransactionsCheckedWithGasLimit",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transactions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapAMM"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapAMM"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SwapAMM"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SwapAMM"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferQuote"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedBurnLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedBurnLp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnLp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnLp"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDepositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDepositCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDepositInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDepositInsurance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositInsurance",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositInsurance",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedLinkSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LinkSigner",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LinkSigner",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedMintLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedMintLp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintLp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintLp"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedTransferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedTransferQuote",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedWithdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedWithdrawCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateFeeRates",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateFeeRates",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdatePrice",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdatePrice",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateProduct",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateProduct",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateSanctions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateSanctions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_sanctions"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ENDPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pa\x87\xF2\x80b\0\0\"`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\x8BW`\x005`\xE0\x1C\x80c\x85\xC8>\x9D\x11a\x02\xE2W\x80c\xB1\xFB\xD6\x0B\x11a\x01\x91W\x80c\xDBZP!\x11a\0\xEEW\x80c\xEERU&\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\x0F\xA8W\x80c\xF8\x0F|\xE5\x14a\x0C9W\x80c\xFB\xF4\x19\x84\x14a\x0E\x83W`\0\x80\xFD[\x80c\xEERU&\x14a\x0FJW\x80c\xEFd\xED\x0E\x14a\x0FlW\x80c\xF2c9'\x14a\x0F\x95W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x0E\xD1W\x80c\xE9\xABw\xE5\x14a\x0E\xE4W\x80c\xE9\xBCtb\x14a\x0F7W`\0\x80\xFD[\x80c\xDBZP!\x14a\x0E\x91W\x80c\xDCB\xE6\x1B\x14a\x0E\xB1W`\0\x80\xFD[\x80c\xC3ES\x0B\x11a\x01EW\x80c\xC5\x105\x9F\x11a\x01*W\x80c\xC5\x105\x9F\x14a\x0E\\W\x80c\xD3\x8C;\x9C\x14a\x0EcW\x80c\xD4\xDE\x8D\x9D\x14a\x0E\x83W`\0\x80\xFD[\x80c\xC3ES\x0B\x14a\x0E8W\x80c\xC4\xF9\xB2_\x14a\x0EKW`\0\x80\xFD[\x80c\xB3d\x88\xB8\x11a\x01vW\x80c\xB3d\x88\xB8\x14a\x0E\x05W\x80c\xB7\x0E\xB2c\x14a\x0E%W\x80c\xBC\x85\xCA\x86\x14a\x05\xB9W`\0\x80\xFD[\x80c\xB1\xFB\xD6\x0B\x14a\r\xC5W\x80c\xB2\xBBcg\x14a\r\xE5W`\0\x80\xFD[\x80c\x91q\xD0\x8B\x11a\x02?W\x80c\x98\xCD2\xFE\x11a\x01\xF3W\x80c\x9E\x85\x14$\x11a\x01\xD8W\x80c\x9E\x85\x14$\x14a\r\x85W\x80c\xA0\x82\xC5\xAA\x14a\r\xA5W\x80c\xB1\xC8\xEC+\x14a\n\x15W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x14a\r%W\x80c\x9A\x08\xE55\x14a\r8W`\0\x80\xFD[\x80c\x94\xFA\xEF\xE5\x11a\x02$W\x80c\x94\xFA\xEF\xE5\x14a\x0C9W\x80c\x954\xDD>\x14a\x0CdW\x80c\x96\xC4|o\x14a\x0C\x84W`\0\x80\xFD[\x80c\x91q\xD0\x8B\x14a\x0B\xF0W\x80c\x91\xC1\xE3\xD7\x14a\x0C\x10W`\0\x80\xFD[\x80c\x8D< \xB1\x11a\x02\x96W\x80c\x8E]X\x8C\x11a\x02{W\x80c\x8E]X\x8C\x14a\x0B\xACW\x80c\x8FO\x8E\xCC\x14a\x0B\xBFW\x80c\x91\x0E`j\x14a\x0B\xD0W`\0\x80\xFD[\x80c\x8D< \xB1\x14a\x0B{W\x80c\x8D\xA5\xCB[\x14a\x0B\x9BW`\0\x80\xFD[\x80c\x8C=/t\x11a\x02\xC7W\x80c\x8C=/t\x14a\x0B\x07W\x80c\x8CX\xE1\n\x14a\x0B$W\x80c\x8D\n\xCC\x9B\x14a\x0BhW`\0\x80\xFD[\x80c\x85\xC8>\x9D\x14a\n\xD4W\x80c\x872C8\x14a\n\xF4W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04>W\x80cZ\0\x92;\x11a\x03\x9BW\x80clEup\x11a\x03OW\x80co;\nr\x11a\x034W\x80co;\nr\x14a\n\x99W\x80cqP\x18\xA6\x14a\n\xB9W\x80c}\xB6\xA2[\x14a\n\xC1W`\0\x80\xFD[\x80clEup\x14a\nfW\x80cl\xFE_\xE4\x14a\n\x86W`\0\x80\xFD[\x80c]O_\x97\x11a\x03\x80W\x80c]O_\x97\x14a\nKW\x80ca\x0B.^\x14a\x06.W\x80ce\xDD\x13f\x14a\n^W`\0\x80\xFD[\x80cZ\0\x92;\x14a\n\x15W\x80c[\xB4\xC1&\x14a\n5W`\0\x80\xFD[\x80c>\xDF,[\x11a\x03\xF2W\x80cM\x96\xA9\n\x11a\x03\xD7W\x80cM\x96\xA9\n\x14a\t\xBDW\x80cTDV\x9D\x14a\t\xE2W\x80cU~\xD1\xBA\x14a\t\xF5W`\0\x80\xFD[\x80c>\xDF,[\x14a\t}W\x80cB\xC7M\x1D\x14a\t\x9DW`\0\x80\xFD[\x80c6\xB9\x0CQ\x11a\x04#W\x80c6\xB9\x0CQ\x14a\x08\xF7W\x80c8B\xE7^\x14a\t\x17W\x80c<\xECK\x93\x14a\t9W`\0\x80\xFD[\x80c5c\x9AO\x14a\x08qW\x80c6\x8EF\x86\x14a\x08\xD1W`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x04\xECW\x80c,\x8Co\xFB\x11a\x04\xA0W\x80c-\x035\xAB\x11a\x04\x85W\x80c-\x035\xAB\x14a\x07\xCBW\x80c/\x9A'D\x14a\x07\xFDW\x80c2\x16\xC0b\x14a\x08\x10W`\0\x80\xFD[\x80c,\x8Co\xFB\x14a\x07\x8BW\x80c,\xD7\x1B\x16\x14a\x07\xABW`\0\x80\xFD[\x80c\"\0`F\x11a\x04\xD1W\x80c\"\0`F\x14a\x07GW\x80c\"\x1F\t9\x14a\x07OW\x80c\"\xD4\xA8-\x14a\x07bW`\0\x80\xFD[\x80c\x1F\x18k'\x14a\x07*W\x80c!\x04u\x89\x14a\x07?W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05CW\x80c\x18\xED\x16\xEB\x11a\x05(W\x80c\x18\xED\x16\xEB\x14a\x06\xE0W\x80c\x1D\x97\xD2/\x14a\x06iW\x80c\x1D\x9E\xED\xA5\x14a\x06\xF3W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06iW\x80c\x0FKP\x9D\x14a\x06\x89W`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05tW\x80c\x07H\xA2\x19\x14a\x05\xD9W\x80c\rU\xE2k\x14a\x06.W\x80c\r\x8En,\x14a\x06NW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\x90W\x80c\x06\xC0\xBA\xFD\x14a\x05\xB9W[`\0\x80\xFD[a\x05\xA3a\x05\x9E6`\x04aY\xD0V[a\x0F\xBBV[`@Qa\x05\xB0\x91\x90aY\xECV[`@Q\x80\x91\x03\x90\xF3[a\x05\xCCa\x05\xC76`\x04aZ(V[a\x0F\xEDV[`@Qa\x05\xB0\x91\x90aZDV[a\x05\xECa\x05\xE76`\x04aZ(V[a\x10 V[`@Qa\x05\xB0\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06Aa\x06<6`\x04aZ\x99V[a\x10SV[`@Qa\x05\xB0\x91\x90a[\x8CV[`\x1B[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x06|a\x06w6`\x04aZ(V[a\x10dV[`@Qa\x05\xB0\x91\x90a[\x9FV[a\x06\x9Ca\x06\x976`\x04aZ(V[a\x10\x97V[`@Qa\x05\xB0\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[`\xA6Ta\x06Q\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x07\x06a\x07\x016`\x04a[\xEEV[a\x10\xCAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x0F\x0B\x92\x81\x01\x92\x90\x92R\x01a\x05\xB0V[a\x07=a\x0786`\x04a\\UV[a\x10\xEDV[\0[a\x07=a\x11\x89V[a\x07=a\x129V[a\x07=a\x07]6`\x04a^3V[a\x12xV[a\x06Qa\x07p6`\x04a^\xB0V[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x07\x9Ea\x07\x996`\x04a^\xC9V[a\x15\xB7V[`@Qa\x05\xB0\x91\x90a_;V[a\x07\xBEa\x07\xB96`\x04a_\x9CV[a\x15\xE4V[`@Qa\x05\xB0\x91\x90a_\xD0V[a\x06Qa\x07\xD96`\x04a`\x13V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x07=a\x08\x0B6`\x04a`GV[a\x16\x02V[a\x07=a\x08\x1E6`\x04a`\xA0V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\x08\x84a\x08\x7F6`\x04aZ(V[a\x17\x94V[`@Qa\x05\xB0\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\x08\xE4a\x08\xDF6`\x04a`\xEEV[a\x17\xC7V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x05\xB0V[a\t\na\t\x056`\x04aa\x0BV[a\x18\"V[`@Qa\x05\xB0\x91\x90aa\xDDV[a\t*a\t%6`\x04ab\x02V[a\x183V[`@Q\x90Q\x81R` \x01a\x05\xB0V[a\tLa\tG6`\x04aY\xD0V[a\x18QV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xB0V[a\t\x90a\t\x8B6`\x04a_\x9CV[a\x18}V[`@Qa\x05\xB0\x91\x90ab\x84V[a\t\xB0a\t\xAB6`\x04a_\x9CV[a\x18\x8EV[`@Qa\x05\xB0\x91\x90ac\x02V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x07=a\t\xF06`\x04ac\xE1V[a\x18\xABV[a\t\xFDa\x1C\xE5V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\n(a\n#6`\x04a_\x9CV[a\x1DsV[`@Qa\x05\xB0\x91\x90ad\xA1V[a\n=a\x1D\x91V[`@Q\x90\x81R` \x01a\x05\xB0V[`\x9ATa\t\xCA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x07=a\x1F\x84V[a\nya\nt6`\x04ad\xC6V[a\x1F\xC7V[`@Qa\x05\xB0\x91\x90ad\xFAV[a\x07=a\n\x946`\x04ae\x1DV[a\x1F\xD8V[a\n\xACa\n\xA76`\x04aZ\x99V[a \x05V[`@Qa\x05\xB0\x91\x90aeRV[a\x07=a \x16V[a\x07=a\n\xCF6`\x04ae\xC2V[a *V[a\n\xE7a\n\xE26`\x04aa\x0BV[a\";V[`@Qa\x05\xB0\x91\x90af7V[a\x07=a\x0B\x026`\x04af\xC1V[a\"LV[a\x0B\x0Fa)\xC9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x07=a\x0B26`\x04ag\x15V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x07=a\x0Bv6`\x04agNV[a+\xB5V[a\x0B\x8Ea\x0B\x896`\x04a^\xC9V[a,\xE4V[`@Qa\x05\xB0\x91\x90ag\xC1V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\t\xCAV[a\x07=a\x0B\xBA6`\x04ag\xECV[a,\xF5V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\t\xCAV[a\x0B\xE3a\x0B\xDE6`\x04ah1V[a-eV[`@Qa\x05\xB0\x91\x90ahMV[a\x0C\x03a\x0B\xFE6`\x04ah\xC2V[a-\xA6V[`@Qa\x05\xB0\x91\x90ah\xF6V[a\t\xCAa\x0C\x1E6`\x04a^\xB0V[`\0\x90\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0CLa\x0CG6`\x04ab\x02V[a-\xB7V[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x05\xB0V[a\x0Cwa\x0Cr6`\x04a`\x13V[a-\xD5V[`@Qa\x05\xB0\x91\x90ailV[a\x0C\x97a\x0C\x926`\x04ai\x7FV[a.oV[`@Qa\x05\xB0\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x07=a\r36`\x04ai\xBBV[a.\xB7V[a\rKa\rF6`\x04aY\xD0V[a0\x1FV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xB0V[a\r\x98a\r\x936`\x04ah1V[a0KV[`@Qa\x05\xB0\x91\x90aj^V[a\r\xB8a\r\xB36`\x04a_\x9CV[a0\x8CV[`@Qa\x05\xB0\x91\x90aj\xB5V[a\r\xD8a\r\xD36`\x04ah\xC2V[a0\x9DV[`@Qa\x05\xB0\x91\x90ak\x16V[a\r\xF8a\r\xF36`\x04a_\x9CV[a0\xAEV[`@Qa\x05\xB0\x91\x90akrV[a\x0E\x18a\x0E\x136`\x04aa\x0BV[a0\xCBV[`@Qa\x05\xB0\x91\x90ak\xCFV[a\n=a\x0E36`\x04af\xC1V[a0\xDCV[a\x0B\x0Fa\x0EF6`\x04af\xC1V[a1yV[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x06QV[`\0a\x08\xE4V[a\x0Eva\x0Eq6`\x04ah\xC2V[a2YV[`@Qa\x05\xB0\x91\x90al$V[g\r\xE0\xB6\xB3\xA7d\0\0a\x08\xE4V[a\x0E\xA4a\x0E\x9F6`\x04ah\xC2V[a2jV[`@Qa\x05\xB0\x91\x90al\x8BV[a\x0E\xC4a\x0E\xBF6`\x04ah1V[a2{V[`@Qa\x05\xB0\x91\x90am(V[a\x07=a\x0E\xDF6`\x04agNV[a2\xBCV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x05\xB0V[a\x07=a\x0FE6`\x04am\x80V[a6PV[a\x0F]a\x0FX6`\x04am\xF7V[a6\xDEV[`@Qa\x05\xB0\x93\x92\x91\x90an\x12V[a\n=a\x0Fz6`\x04am\xF7V[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x07=a\x0F\xA36`\x04a`\x13V[a7\xF9V[a\x07=a\x0F\xB66`\x04a`\x13V[a8#V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83an\xAFV[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ao&V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83aoBV[a\x10[aV\xC0V[a\x0F\xE7\x82ao\xF1V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ap3V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83apOV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ap\xADV[`\0[\x81\x81\x10\x15a\x11BW6`\0\x84\x84\x84\x81\x81\x10a\x11\rWa\x11\raq\x0BV[\x90P` \x02\x81\x01\x90a\x11\x1F\x91\x90aq!V[\x91P\x91Pa\x11-\x82\x82a8\xB3V[PP\x80\x80a\x11:\x90aq}V[\x91PPa\x10\xF0V[P`\xA6\x80T\x82\x91\x90`\0\x90a\x11a\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aq\x96V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x08\x1EW`\xA8`\0\x82` \x01\x80Qa\x11\xF7\x90aq\xC1V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x122`\x01\x83\x01\x82aV\xF9V[PPa\x11\xC2V[`\xA6\x80T`\0\x90a\x12R\x90`\x01`\x01`@\x1B\x03\x16aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPV[\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbIRC`\xE8\x1B` \x82\x01R\x90a\x12\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01R`]c\xFF\xFF\xFF\xFF\x85\x16\x03a\x12\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P``\x84\x90\x1Ca\x133\x813\x81\x14a\x13-W`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaJzV[\x83aJzV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA6\x91\x90ar\nV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\xBBW`\0\x80\xFD[a\x13\xCF\x813\x86`\x01`\x01`\x80\x1B\x03\x16aJ\xD3V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R`\0a\x14\x16Bb\x03\xF4\x80aq\x96V[`@\x80Q``\x81\x01\x82R`\x01`\x01`@\x1B\x03\x83\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x91\x92P\x81\x01`\x01`@Q\x80``\x01`@R\x80\x8C\x81R` \x01\x8Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x14\xA7\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14\xC5\x92\x91` \x01ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x84` \x01\x80Q\x80\x91\x90a\x14\xED\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x15Y\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x82Q`\xA7\x80T` \x86\x01Q`@\x90\x96\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x97\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x94\x90\x94\x16\x17\x90\x92UPPPPPPPPV[a\x15\xDB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x0F\xE7\x82ar\xFFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x0F\xE7\x82asMV[`\0Z`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x91\x92P`\x01`\x01`@\x1B\x03\x87\x81\x16\x91\x16\x14a\x16OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0[\x83\x81\x10\x15a\x17\x1BW6`\0\x86\x86\x84\x81\x81\x10a\x16pWa\x16paq\x0BV[\x90P` \x02\x81\x01\x90a\x16\x82\x91\x90aq!V[\x91P\x91Pa\x16\x90\x82\x82a8\xB3V[\x84Za\x16\x9C\x90\x86asYV[\x11\x15a\x17\x06W`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xEDW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x01W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x17\x13\x90aq}V[\x91PPa\x16SV[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x179\x90\x85asYV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17uW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83as\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x18\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x91\x90PV[a\x18*aW\xB7V[a\x0F\xE7\x82at\xF4V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x0F\xE76\x83\x90\x03\x83\x01\x83au\0V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83auAV[a\x18\x85aW\xE5V[a\x0F\xE7\x82av=V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x0F\xE7\x82aw\x0BV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x18\xCBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x18\xE5WP0;\x15\x80\x15a\x18\xE5WP`\0T`\xFF\x16`\x01\x14[a\x19WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12\xB2V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x19zW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x19\x82aJ\xE9V[a\x19\xF6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaK\\V[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1Aq\x90`\0\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xB2\x91\x90ar\nV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1A\xF6\x90`\x01\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B7\x91\x90ar\nV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB9\x91\x90ar\nV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1C\x95W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1CDWa\x1CDaq\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1C\x8D\x81aw\x93V[\x91PPa\x1C\x1DV[P\x80\x15a\x1C\xDCW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1D\"W\x81Qa\x1D(V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1DlW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x0F\xE7\x82aw\xF8V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a\x1E&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a\x1E\x84\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xB0\x90ax\x04V[\x80\x15a\x1E\xFDW\x80`\x1F\x10a\x1E\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a\x1F7\x92\x90\x91`\x04\x01ax8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1FpWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1Fm\x91\x81\x01\x90axZV[`\x01[a\x1F}WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x08\x1E\x81`\0aK\xD1V[a\x1F\xCFaX\x14V[a\x0F\xE7\x82axsV[a\x1F\xE0aN<V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a \raXKV[a\x0F\xE7\x82ax\xA5V[a \x1EaN<V[a (`\0aN\x96V[V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x88\x81\x16\x91\x16\x14a rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a \x8AW`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a!$W\x81\x87\x87\x83\x81\x81\x10a \xD7Wa \xD7aq\x0BV[\x90P` \x02\x81\x01\x90a \xE9\x91\x90aq!V[`@Q` \x01a \xFB\x93\x92\x91\x90ax\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a!\x1D\x90aq}V[\x90Pa \xBCV[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x96W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a!\xEFW6`\0\x88\x88\x84\x81\x81\x10a!\xBAWa!\xBAaq\x0BV[\x90P` \x02\x81\x01\x90a!\xCC\x91\x90aq!V[\x91P\x91Pa!\xDA\x82\x82a8\xB3V[PP\x80\x80a!\xE7\x90aq}V[\x91PPa!\x9DV[P`\xA6\x80T\x86\x91\x90`\0\x90a\"\x0E\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aq\x96V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPPPPV[a\"CaX{V[a\x0F\xE7\x82ay\x0CV[30\x14a\"XW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a\"mWa\"maq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a\"\x88Wa\"\x88ar'V[\x90P`\0\x81`\x18\x81\x11\x15a\"\x9EWa\"\x9Ear'V[\x03a#BW`\0a\"\xB2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a\"\xBF\x91\x90ay\xBAV[\x90Pa\"\xCF\x81`\0\x01Q\x86aN\xE8V[\x80Qa\"\xDA\x90aOFV[`\x9AT`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cR\xEF\xAD\xF1\x90a#\n\x90\x84\x90`\x04\x01aj^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#8W=`\0\x80>=`\0\xFD[PPPPPa)\xC3V[`\x01\x81`\x18\x81\x11\x15a#VWa#Var'V[\x03a#\xE6W`\0a#j\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a#w\x91\x90auAV[\x90Pa#\x87\x81`\0\x01Q\x86aN\xE8V[\x80Qa#\x92\x90aO\xA4V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01a#\nV[`\x02\x81`\x18\x81\x11\x15a#\xFAWa#\xFAar'V[\x03a$\x8BW`\0a$\x0E\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a$\x1B\x91\x90ao&V[\x90Pa$+\x81`\0\x01Q\x86aN\xE8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x82A\x8Ck`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a#\nV[`\x05\x81`\x18\x81\x11\x15a$\x9FWa$\x9Far'V[\x03a$\xF1W`\0a$\xB3\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a$\xC0\x91\x90az\x9AV[`\x9AT`@Qc\xB2\xBBcg`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xBBcg\x90a#\n\x90\x84\x90`\x04\x01akrV[`\x07\x81`\x18\x81\x11\x15a%\x05Wa%\x05ar'V[\x03a%aW`\0a%\x19\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a%&\x91\x90a{\x19V[`\x9AT`@Qc:\x91\xC5\x8B`\xE0\x1B\x81R\x82Q`\x01`\x01`\x80\x1B\x03\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c:\x91\xC5\x8B\x90`$\x01a#\nV[`\t\x81`\x18\x81\x11\x15a%uWa%uar'V[\x03a'\x05W`\0a%\x89\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a%\x96\x91\x90a{\x96V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a%\xC8\x90`\0\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\t\x91\x90ar\nV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x85\x91\x90ar\nV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a&\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x80Qa&\xD5\x90\x86aN\xE8V[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a#\n\x90\x84\x90`\x04\x01ahMV[`\n\x81`\x18\x81\x11\x15a'\x19Wa'\x19ar'V[\x03a'zW`\0a'-\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a':\x91\x90ao&V[\x90Pa'J\x81`\0\x01Q\x86aN\xE8V[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a#\n\x90\x84\x90`\x04\x01aZDV[`\x0B\x81`\x18\x81\x11\x15a'\x8EWa'\x8Ear'V[\x03a(&W`\0a'\xA2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a'\xAF\x91\x90apOV[\x90Pa'\xBF\x81`\0\x01Q\x86aN\xE8V[\x80Qa'\xCA\x90aOFV[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a#\nV[`\x12\x81`\x18\x81\x11\x15a(:Wa(:ar'V[\x03a(\x90W`\0a(N\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a([\x91\x90a{\xB2V[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a#\n\x91`\x04\x01ailV[`\x13\x81`\x18\x81\x11\x15a(\xA4Wa(\xA4ar'V[\x03a)\x13W`\0a(\xB8\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a(\xC5\x91\x90an\xAFV[\x90Pa(\xD5\x81`\0\x01Q\x86aN\xE8V[\x80Qa(\xE0\x90aOFV[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua)\xC3V[`\x15\x81`\x18\x81\x11\x15a)'Wa)'ar'V[\x03a\x05\x8BW`\0a);\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a)H\x91\x90aoBV[\x90Pa)X\x81`\0\x01Q\x86aN\xE8V[a)e\x81``\x01QaO\xA4V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a#\nV[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a*^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a*\xBC\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xE8\x90ax\x04V[\x80\x15a+5W\x80`\x1F\x10a+\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\x18W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a+o\x92\x90\x91`\x04\x01ax8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a+\xA8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+\xA5\x91\x81\x01\x90a{\xE6V[`\x01[a\x1F}W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a+\xCAWa+\xCAaq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a+\xE5Wa+\xE5ar'V[\x90P`\0\x81`\x18\x81\x11\x15a+\xFBWa+\xFBar'V[\x03a,MW`\0a,\x0F\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a,\x1C\x91\x90a|DV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa,2\x91aP3V[\x80QQa,G\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[Pa,\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a,\xBA\x83aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a,\xECaX\xA4V[a\x0F\xE7\x82a|\xEDV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra-`\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra->\x90a|\xF9V[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x12xV[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a{\x96V[a-\xAEaX\xD1V[a\x0F\xE7\x82a}\x1DV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a{\x19V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta-\xEE\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.\x1A\x90ax\x04V[\x80\x15a.gW\x80`\x1F\x10a.<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a}QV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a/CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\xB2V[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a/Z\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a/\xC6\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a}\xE8V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ay\xBAV[a0\x94aY\x10V[a\x0F\xE7\x82a~.V[a0\xA5aX\xD1V[a\x0F\xE7\x82a\x7F\x14V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x0F\xE7\x82a\x7F_V[a0\xD3aY8V[a\x0F\xE7\x82a\x7FkV[`\0\x80\x83\x83`\0\x81\x81\x10a0\xF2Wa0\xF2aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a1\rWa1\rar'V[\x90P`\x13\x81`\x18\x81\x11\x15a1#Wa1#ar'V[\x03a1nW`\0a17\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1D\x91\x90an\xAFV[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a1bW`\0a1eV[\x80Q[\x92PPPa\x1F}V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a1\x8FWa1\x8Faq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a1\xAAWa1\xAAar'V[\x90P`\t\x81`\x18\x81\x11\x15a1\xC0Wa1\xC0ar'V[\x03a1\xEFW`\0a1\xD4\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90a{\x96V[` \x01Q\x92Pa\x1F}\x91PPV[`\n\x81`\x18\x81\x11\x15a2\x03Wa2\x03ar'V[\x03a2$W`\0a2\x17\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90ao&V[`\x0B\x81`\x18\x81\x11\x15a28Wa28ar'V[\x03a1nW`\0a2L\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90apOV[a2aaX\xD1V[a\x0F\xE7\x82a\x80\x15V[a2raYWV[a\x0F\xE7\x82a\x80qV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a\x81\x1EV[`\0\x82\x82`\0\x81\x81\x10a2\xD1Wa2\xD1aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a2\xECWa2\xECar'V[\x90P3`\x01\x82`\x18\x81\x11\x15a3\x03Wa3\x03ar'V[\x03a3\rW`\0\x80\xFD[`\x07\x82`\x18\x81\x11\x15a3!Wa3!ar'V[\x03a3\x82W`\0a35\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a3B\x91\x90a{\x19V[\x90P`\0a3NaP\xEFV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3cW`\0\x80\xFD[a3{\x81\x84\x84`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16aJ\xD3V[PPa4\xCAV[`\x12\x82`\x18\x81\x11\x15a3\x96Wa3\x96ar'V[\x03a3\xBAW`gT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a3\xB5W`\0\x80\xFD[a4\xCAV[`\x15\x82`\x18\x81\x11\x15a3\xCEWa3\xCEar'V[\x03a4zW`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a4*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0a4:\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a4G\x91\x90aoBV[\x90Pa4t\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaJzV[Pa4\xCAV[`\0a4\x84aP\xEFV[\x90P`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a4\xA1\x90\x84\x90`\x0F\x0Ba\x81:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPP[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R`\0a5\x11Bb\x03\xF4\x80aq\x96V[\x90P`@Q\x80``\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x85\x01\x80Q`\xA8\x93P\x90a5\x88\x82aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a5\xF4\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x82Q`\xA7\x80T` \x86\x01Q`@\x90\x96\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x97\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x94\x90\x94\x16\x17\x90\x92UPPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra6\xD7\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra6\x99\x90a|\xF9V[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12x\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a7h\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\x94\x90ax\x04V[\x80\x15a7\xE1W\x80`\x1F\x10a7\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a8\x01aN<V[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a8+aN<V[`\x01`\x01`\xA0\x1B\x03\x81\x16a8\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12\xB2V[a8\xB0\x81aN\x96V[PV[`\0\x82\x82`\0\x81\x81\x10a8\xC8Wa8\xC8aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a8\xE3Wa8\xE3ar'V[\x90P`\0\x81`\x18\x81\x11\x15a8\xF9Wa8\xF9ar'V[\x03a:ZW`\0a9\r\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a9\x1A\x91\x90a|DV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa90\x91aP3V[`\0a9\xD9`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a\x86^`w\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aQaV[\x90P\x81QQa9\xE7\x90aOFV[\x81QQa9\xFC\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a:,\x91`\x04\x01aj^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[`\x02\x81`\x18\x81\x11\x15a:nWa:nar'V[\x03a<\x0EW`\0a:\x82\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a:\x8F\x91\x90a\x81\x9BV[\x80Q\x80Q``\x90\x91\x01Q\x91\x92Pa:\xA5\x91aP3V[`\0a; `@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a\x87n`O\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90P\x81Q\x80Q`\x9BT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ra;\xAC\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xA1\x91\x90a\x81\xCFV[\x84Q` \x01QaQ\xAFV[`\x9AT\x82Q\x80Q` \x82\x01Q`@\x92\x83\x01Q\x92Qc\x82A\x8Ck`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16`$\x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a:,V[`\x03\x81`\x18\x81\x11\x15a<\"Wa<\"ar'V[\x03a=*W`\0a<6\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a<C\x91\x90a\x81\xECV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15a<\x8FW` \x82\x01Q\x83Qa<\x8A\x91\x90a\x82 V[a<\x92V[`\0[`\x9BT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a<\xCB\x91\x85\x91\x90`\x04\x01a\x82HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xF9W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa-`\x91PPV[`\x0F\x81`\x18\x81\x11\x15a=>Wa=>ar'V[\x03a>?W`\0a=R\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a=_\x91\x90a\x81\xECV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15a=\xA7W\x81Q\x83Qa=\xA2\x91\x90a\x82 V[a=\xAAV[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a=\xE3\x91\x85\x91\x90`\x04\x01a\x82HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\x11W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa-`\x91PPV[`\x04\x81`\x18\x81\x11\x15a>SWa>Sar'V[\x03a>\x88W`\0a>g\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a>t\x91\x90ap\xADV[\x90Pa)\xC3\x81`\0\x01Q\x82` \x01QaR\x86V[`\x05\x81`\x18\x81\x11\x15a>\x9CWa>\x9Car'V[\x03a?&W`\0a>\xB0\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a>\xBD\x91\x90az\x9AV[`\x9AT`@Qc\xB2\xBBcg`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xBBcg\x90a>\xEE\x90\x84\x90`\x04\x01akrV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\x1CW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x06\x81`\x18\x81\x11\x15a?:Wa?:ar'V[\x14\x80a?WWP`\x16\x81`\x18\x81\x11\x15a?UWa?Uar'V[\x14[\x15a@\x08W`\0a?k\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a?x\x91\x90a\x82jV[` \x81\x01QQQ\x90\x91Pa?\x8B\x90aOFV[`@\x81\x01QQQa?\x9B\x90aOFV[`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x84\x01QQQ`\0\x90\x81R`\xAB\x80\x83R\x84\x82 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84\x86\x01R\x86\x86\x01QQQ\x83R\x92R\x83\x90 T\x81\x16\x82\x84\x01R`\xB0T\x92Qc\x11\x17\x8F-`\xE3\x1B\x81R\x91\x92\x16\x90c\x88\xBCyh\x90a:,\x90\x84\x90`\x04\x01a\x82\x9EV[`\x0C\x81`\x18\x81\x11\x15a@\x1CWa@\x1Car'V[\x03a@\x9BW`\0a@0\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a@=\x91\x90a\x82\xE9V[``\x81\x01QQQ\x90\x91Pa@P\x90aOFV[`\xB0T``\x82\x01QQQ`\0\x90\x81R`\xAB` R`@\x90\x81\x90 T\x90Qc<xi\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92cx\xF0\xD3\xCE\x92a>\xEE\x92\x86\x92\x90\x91\x16\x90`\x04\x01a\x83\x1DV[`\x08\x81`\x18\x81\x11\x15a@\xAFWa@\xAFar'V[\x03aAMW`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra@\xF7\x81`\x01aK\xD1V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81`\x18\x81\x11\x15aAaWaAaar'V[\x03aBzW`\0aAu\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aA\x82\x91\x90a\x83HV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaA\x98\x91aP3V[`\0aB3`@Q\x80`\xA0\x01`@R\x80`v\x81R` \x01a\x86\xF8`v\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93R\x90\x82\x16\x92\x85\x01\x92\x90\x92R\x16`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01a9\xBEV[\x90P\x81QQaBJ\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91a:,\x91`\x04\x01ahMV[`\n\x81`\x18\x81\x11\x15aB\x8EWaB\x8Ear'V[\x03aC\x87W`\0aB\xA2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aB\xAF\x91\x90a\x81\x9BV[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaB\xC5\x91aP3V[`\0aC@`@Q\x80`\x80\x01`@R\x80`C\x81R` \x01a\x86\x1B`C\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90P\x81QQaCW\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91a:,\x91`\x04\x01aZDV[`\r\x81`\x18\x81\x11\x15aC\x9BWaC\x9Bar'V[\x03aD\x04W`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xDCW=`\0\x80>=`\0\xFD[`\x0E\x81`\x18\x81\x11\x15aD\x18WaD\x18ar'V[\x03aFYW`\0aD,\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aD9\x91\x90au\0V[\x90P`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaD\xB8\x91\x90\x81\x01\x90a\x83|V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xD5WaD\xD5a\\\xC4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\xFEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15aE\xE1W`\xAA`\0\x84\x83\x81Q\x81\x10aE#WaE#aq\x0BV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10aEeWaEeaq\x0BV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10aE\x91WaE\x91aq\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aE\xD9\x81aq}V[\x91PPaE\x04V[P\x82QaE\xED\x90aOFV[`\x9AT`@Qcx\x1C\x85\x7F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF09\n\xFE\x90aF\x1F\x90\x86\x90\x85\x90`\x04\x01a\x84\x15V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15aFMW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\x10\x81`\x18\x81\x11\x15aFmWaFmar'V[\x03aGUW`\0aF\x81\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aF\x8E\x91\x90a\x84/V[`\x9CT\x81Q`@Qc\x9Bov+`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x9Bov+\x91aF\xC1\x91`\x04\x01a\x84cV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aF\xD9W`\0\x80\xFD[PZ\xFA\x15\x80\x15aF\xEDW=`\0\x80>=`\0\xFD[PP`\x9BT` \x84\x01Q`@\x80\x86\x01Q\x90Qc\x03\tr\xB5`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94Pc0\x97+P\x93PaG)\x92`\x04\x01a\x84vV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aGAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a?\x1CW=`\0\x80>=`\0\xFD[`\x13\x81`\x18\x81\x11\x15aGiWaGiar'V[\x03aH3W`\0aG}\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aG\x8A\x91\x90a\x84\x9BV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaG\xA0\x91aP3V[`\0aG\xFC`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x85\xE5`6\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a9\xBEV[\x90PPQ` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81`\x18\x81\x11\x15aHGWaHGar'V[\x03aH\xC9W`\0aH[\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aHh\x91\x90as\x82V[`\xB0T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xB7mx\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x93\x16`$\x84\x01R`\x07\x90\x81\x0B`D\x84\x01R\x0B`d\x82\x01R\x92\x93P\x16\x90c\xB7mx\xE3\x90`\x84\x01a>\xEEV[`\x17\x81`\x18\x81\x11\x15aH\xDDWaH\xDDar'V[\x03aI\xE1W`\0aH\xF1\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aH\xFE\x91\x90a\x84\xCFV[\x90P`\0aIu`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a\x85\x9A`K\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90PaI\x88\x82`\0\x01Q` \x01QaOFV[\x81Q\x80Q``\x90\x91\x01QaI\x9C\x91\x90aP3V[\x81QQaI\xB1\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a:,\x91`\x04\x01a[\x9FV[`\x18\x81`\x18\x81\x11\x15aI\xF5WaI\xF5ar'V[\x03a\x05\x8BW`\0aJ\t\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aJ\x16\x91\x90a}\xE8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x82A\x8Ck`\xE0\x1B\x81R`\x01`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x93\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`d\x83\x01R\x92\x93P\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a>\xEEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaJ\x9D\x90ax\x04V[\x90P`\0\x03aJ\xCFW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa-`\x92\x84\x01\x90aW3V[PPV[`\x9ATa-`\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\0Ta\x01\0\x90\x04`\xFF\x16aKTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[a (aS\xC6V[`\0Ta\x01\0\x90\x04`\xFF\x16aK\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[aJ\xCF\x82\x82aT:V[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x86\xD5`#\x919\x90aL%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aL\x84\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaL\xB0\x90ax\x04V[\x80\x15aL\xFDW\x80`\x1F\x10aL\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aL\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aL\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aM\x1E\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aM[`\x01\x83\x01\x82aV\xF9V[PP\x81\x80aMvWPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aM\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aM\xE5\x92\x90\x91`\x04\x01ax8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aM\xFFW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aN\x10WP`\x01[a)\xC3Wb\x03\xD0\x90Z\x11\x15\x80aN0WPaN,`\x02\x82a\x85\x03V[Z\x11\x15[\x15aN7W\xFE[a)\xC3V[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a (W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\xB2V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aO\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a-`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`\x01\x81\x14\x80aOkWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90aJ\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a8\xB0W`\xA2\x80T`\0\x90aO\xDB\x90`\x01`\x01`@\x1B\x03\x16aq\xE4V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aP\\\x83aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a-`WaP\xAD\x81`\x01`\x01`@\x1B\x03\x16aT\xBFV[`@Q` \x01aP\xBD\x91\x90a\x85%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x12\xB2\x91`\x04\x01ailV[aJ\xCF\x82\x82`\0aQ\xAFV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE7\x91\x90ar\nV[`\0a\x0F\xE7aQnaU^V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aQ\xCB\x86a\x85jV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aR\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15aR.W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aRZ\x90\x84\x90`\x0F\x0Ba\x81:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x81`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b$\xA8)`\xE9\x1B\x81RP\x90aR\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS;\x91\x90ar\nV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a-`Wc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`\xAF` R`@\x90\x81\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x86\x16\x17\x90UQbT\xF2\x9B`\xE6\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x0F\x83\x90\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x15<\xA6\xC0\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xF0W`\0\x80\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16aT1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[a (3aN\x96V[`\0Ta\x01\0\x90\x04`\xFF\x16aT\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0aT\xCC\x83aU\xDEV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xEBWaT\xEBa\\\xC4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aU\x15W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aU\x1FWP\x93\x92PPPV[`\0aU\xD9\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaU\x8D`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[\x90P\x90V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aV'Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aVSWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10aVqWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10aV\x89Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10aV\x9DWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10aV\xAFW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0F\xE7W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80TaW\x05\x90ax\x04V[`\0\x82U\x80`\x1F\x10aW\x15WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a8\xB0\x91\x90aY\x8BV[\x82\x80TaW?\x90ax\x04V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aWaW`\0\x85UaW\xA7V[\x82`\x1F\x10aWzW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85UaW\xA7V[\x82\x80\x01`\x01\x01\x85U\x82\x15aW\xA7W\x91\x82\x01[\x82\x81\x11\x15aW\xA7W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aW\x8CV[PaW\xB3\x92\x91PaY\x8BV[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01aW\xE0aX\x14V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01aX\xC4aX\x14V[\x81R` \x01aW\xE0aX\x14V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01aX\xC4aX\x14V[`@Q\x80`\xA0\x01`@R\x80aYjaY\xA0V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15aW\xB3W`\0\x81U`\x01\x01aY\x8CV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aY\xE2W`\0\x80\xFD[a\x1F}\x83\x83aY\xBEV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0F\xE7V[`\0`\x80\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aZ:W`\0\x80\xFD[a\x1F}\x83\x83aZ\x16V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x0F\xE7V[`\0`\xA0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ\xABW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xC1W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aZ\x87V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15aZ\xF0W\x81\x81\x01Q\x83\x82\x01R` \x01aZ\xD8V[\x83\x81\x11\x15a)\xC3WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra[\x19\x81` \x86\x01` \x86\x01aZ\xD5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a[q\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01RaZ\xCD`\xA0\x85\x01\x82a[\x01V[` \x81R`\0a\x1F}` \x83\x01\x84a[-V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x0F\xE7V[`\0`@\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\\\0W`\0\x80\xFD[a\x1F}\x83\x83a[\xDCV[`\0\x80\x83`\x1F\x84\x01\x12a\\\x1CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\\3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\\NW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\\hW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\\~W`\0\x80\xFD[a\\\x8A\x85\x82\x86\x01a\\\nV[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8\xB0W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a]\xD4Wa]\xD4a\\\xC4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a]\xF5Wa]\xF5a\\\xC4V[a^\x08`\x1F\x84\x01`\x1F\x19\x16` \x01a]\xACV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a^\x1CW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a^IW`\0\x80\xFD[\x845\x93P` \x85\x015a^[\x81a\\\x96V[\x92Pa^i`@\x86\x01a\\\xA8V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a^\x84W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a^\x95W`\0\x80\xFD[a^\xA4\x87\x825` \x84\x01a]\xDCV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a^\xC2W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a^\xDBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a^\xF1W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aY\xBEV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a_\x11V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q``` \x84\x01Ra_W`\x80\x84\x01\x82a^\xFDV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra_u\x83\x83a^\xFDV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa_\x93\x82\x82a^\xFDV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a_\xAEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xC4W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a[\xDCV[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01RaZ\xCD``\x84\x01\x82a[\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a8\xB0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a`%W`\0\x80\xFD[\x815a\x1F}\x81a_\xFEV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a`]W`\0\x80\xFD[a`f\x85a`0V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a`\x81W`\0\x80\xFD[a`\x8D\x87\x82\x88\x01a\\\nV[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a`\xB2W`\0\x80\xFD[a`\xBAa\\\xDAV[a`\xC3\x83a`0V[\x81Ra`\xD1` \x84\x01a`0V[` \x82\x01Ra`\xE2`@\x84\x01a`0V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aa\0W`\0\x80\xFD[\x815a\x1F}\x81a\\\x96V[`\0` \x82\x84\x03\x12\x15aa\x1DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aa3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aZ\x16V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01RaZ\xCD`\xC0\x85\x01\x82a[\x01V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01RaZ\xCD`\x80\x85\x01\x82aa?V[` \x81R`\0a\x1F}` \x83\x01\x84aa\x9EV[`\0` \x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ab\x14W`\0\x80\xFD[a\x1F}\x83\x83aa\xF0V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ab2V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01abhV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rab\xB2`\xE0\x85\x01\x82ab\x1EV[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rab\xCF\x82\x82abTV[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90a_\x93\x81\x83a[\x01V[` \x81R`\0\x82Q`@` \x84\x01Rac\x1E``\x84\x01\x82abTV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra_\x93\x82\x82a^\xFDV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15acTWacTa\\\xC4V[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a8\xB0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ac~W`\0\x80\xFD[\x815` ac\x93ac\x8E\x83ac;V[a]\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ac\xB2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805ac\xC9\x81ac^V[\x83R\x91\x83\x01\x91\x83\x01ac\xB6V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15ac\xFAW`\0\x80\xFD[\x865ad\x05\x81a_\xFEV[\x95P` \x87\x015ad\x15\x81a_\xFEV[\x94P`@\x87\x015ad%\x81a_\xFEV[\x93P``\x87\x015ad5\x81a_\xFEV[\x92P`\x80\x87\x015adE\x81a_\xFEV[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ad`W`\0\x80\xFD[adl\x89\x82\x8A\x01acmV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01RaZ\xCD`@\x85\x01\x82a^\xFDV[` \x81R`\0a\x1F}` \x83\x01\x84adyV[`\0`\xC0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ad\xD8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xEEW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ad\xB4V[` \x81R`\0a\x1F}` \x83\x01\x84aa?V[\x805\x80\x15\x15\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ae0W`\0\x80\xFD[\x825ae;\x81a_\xFEV[\x91PaeI` \x84\x01ae\rV[\x90P\x92P\x92\x90PV[` \x81Rae\x97` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01RaZ\xCD`\xC0\x84\x01\x82a[\x01V[\x805`\xFF\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15ae\xDBW`\0\x80\xFD[ae\xE4\x87a`0V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xFFW`\0\x80\xFD[af\x0B\x89\x82\x8A\x01a\\\nV[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Paf+`\x80\x88\x01ae\xB1V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Raff` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01RaZ\xCD`\xA0\x84\x01\x82a[\x01V[`\0\x80\x83`\x1F\x84\x01\x12af\x92W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\xA9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\\NW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15af\xD6W`\0\x80\xFD[\x835af\xE1\x81a_\xFEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15af\xFCW`\0\x80\xFD[ag\x08\x86\x82\x87\x01af\x80V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15ag(W`\0\x80\xFD[\x825ag3\x81a\\\x96V[\x91P` \x83\x015agC\x81ac^V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15agaW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15agwW`\0\x80\xFD[a\\\x8A\x85\x82\x86\x01af\x80V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rag\xA8``\x85\x01\x82aa?V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra_\x93\x82\x82aa?V[` \x81R`\0a\x1F}` \x83\x01\x84ag\x83V[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15ah\x01W`\0\x80\xFD[ah\n\x84ag\xD4V[\x92P` \x84\x015ah\x1A\x81a\\\x96V[\x91Pah(`@\x85\x01a\\\xA8V[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ahCW`\0\x80\xFD[a\x1F}\x83\x83ad\xB4V[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xEAW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ah\xB0V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01RaZ\xCDa\x01\0\x84\x01\x82a[\x01V[` \x81R`\0a\x1F}` \x83\x01\x84a[\x01V[`\0`\xE0\x82\x84\x03\x12\x15ai\x91W`\0\x80\xFD[a\x1F}\x83\x83ah\xB0V[`\0\x82`\x1F\x83\x01\x12ai\xACW`\0\x80\xFD[a\x1F}\x83\x835` \x85\x01a]\xDCV[`\0\x80`@\x83\x85\x03\x12\x15ai\xCEW`\0\x80\xFD[ai\xD7\x83a`0V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ai\xF3W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15aj\x07W`\0\x80\xFD[aj\x0Fa\\\xDAV[aj\x18\x83a`0V[\x81R` \x83\x015aj(\x81a_\xFEV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15aj?W`\0\x80\xFD[ajK\x88\x82\x86\x01ai\x9BV[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Raj\xE4`\xC0\x85\x01\x82ab\x1EV[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Ra_\x93\x81\x83a[\x01V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Rak\x8E``\x85\x01\x82abTV[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15ac\xD6W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90ak\xAFV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Ral\x07`\xA0\x84\x01\x82aa?V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Ra_\x93\x82\x82aa?V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15al\xBAW\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01al\x9BV[PPP\x83\x01Q`\xE0`\x80\x84\x01Ral\xD5a\x01\0\x84\x01\x82a[\x01V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Ral\xF3\x83\x83abTV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPam\x11\x82\x82abTV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15am\x98W`\0\x80\xFD[am\xA1\x86ag\xD4V[\x94P` \x86\x015am\xB1\x81a\\\x96V[\x93Pam\xBF`@\x87\x01a\\\xA8V[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am\xDAW`\0\x80\xFD[am\xE6\x88\x82\x89\x01af\x80V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15an\tW`\0\x80\xFD[a\x1F}\x82a`0V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01RanU`\xC0\x85\x01\x82a[\x01V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15an\x80W`\0\x80\xFD[an\x88a\\\xDAV[\x90P\x815\x81R` \x82\x015` \x82\x01Ran\xA4`@\x83\x01a`0V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15an\xC1W`\0\x80\xFD[a\x1F}\x83\x83annV[`\0`\x80\x82\x84\x03\x12\x15an\xDDW`\0\x80\xFD[an\xE5a]\x02V[\x90P\x815\x81R` \x82\x015an\xF9\x81a\\\x96V[` \x82\x01Rao\n`@\x83\x01a\\\xA8V[`@\x82\x01Rao\x1B``\x83\x01a`0V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15ao8W`\0\x80\xFD[a\x1F}\x83\x83an\xCBV[`\0`\x80\x82\x84\x03\x12\x15aoTW`\0\x80\xFD[ao\\a]\x02V[\x825\x81R` \x83\x015aon\x81a\\\x96V[` \x82\x01Rao\x7F`@\x84\x01a\\\xA8V[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15ao\xAAW`\0\x80\xFD[ao\xB2a]$V[\x90Pao\xBE\x83\x83an\xCBV[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[ao\xE5\x84\x82\x85\x01ai\x9BV[` \x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83ao\x98V[`\0`\x80\x82\x84\x03\x12\x15ap\x0FW`\0\x80\xFD[ap\x17a]\x02V[\x90P\x815\x81R` \x82\x015` \x82\x01Rao\n`@\x83\x01a\\\xA8V[`\0`\x80\x82\x84\x03\x12\x15apEW`\0\x80\xFD[a\x1F}\x83\x83ao\xFDV[`\0`\x80\x82\x84\x03\x12\x15apaW`\0\x80\xFD[apia]\x02V[\x825\x81R` \x83\x015ap{\x81a\\\x96V[` \x82\x01R`@\x83\x015ap\x8E\x81ac^V[`@\x82\x01R``\x83\x015ap\xA1\x81ac^V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ap\xBFW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15ap\xE1Wap\xE1a\\\xC4V[`@R\x825ap\xEF\x81a\\\x96V[\x81R` \x83\x015ap\xFF\x81ac^V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aq8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aqRW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\\NW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aq\x8FWaq\x8FaqgV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aq\xB8Waq\xB8aqgV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80aq\xDAWaq\xDAaqgV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03ar\0War\0aqgV[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ar\x1CW`\0\x80\xFD[\x81Qa\x1F}\x81a_\xFEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qar^\x81`\x01\x85\x01` \x87\x01aZ\xD5V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15ar~W`\0\x80\xFD[ar\x86a\\\xDAV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ar\x9FW`\0\x80\xFD[ar\xAB\x85\x83\x86\x01acmV[\x83R` \x84\x015\x91P\x80\x82\x11\x15ar\xC1W`\0\x80\xFD[ar\xCD\x85\x83\x86\x01acmV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15ar\xE6W`\0\x80\xFD[Par\xF3\x84\x82\x85\x01acmV[`@\x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83arlV[`\0`@\x82\x84\x03\x12\x15as\x1DW`\0\x80\xFD[as%a]$V[\x90P\x815as2\x81a_\xFEV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0a\x0F\xE76\x83as\x0BV[`\0\x82\x82\x10\x15askWaskaqgV[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14a\\\xBFW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15as\x94W`\0\x80\xFD[as\x9Ca]\x02V[\x825as\xA7\x81a_\xFEV[\x81R` \x83\x015as\xB7\x81a\\\x96V[` \x82\x01Ras\xC8`@\x84\x01aspV[`@\x82\x01Rap\xA1``\x84\x01aspV[`\0\x81\x83\x03`\xC0\x81\x12\x15as\xECW`\0\x80\xFD[as\xF4a]$V[\x91P`\xA0\x81\x12\x15at\x04W`\0\x80\xFD[Pat\ra]FV[\x825\x81R` \x83\x015at\x1F\x81ac^V[` \x82\x01R`@\x83\x015at2\x81ac^V[`@\x82\x01RatC``\x84\x01a`0V[``\x82\x01RatT`\x80\x84\x01a`0V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15at\x86W`\0\x80\xFD[at\x8Ea]\x02V[\x90P\x815at\x9B\x81a\\\x96V[\x81R` \x82\x015at\xAB\x81ac^V[` \x82\x01R`@\x82\x015at\xBE\x81ac^V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15at\xDCW`\0\x80\xFD[at\xE8\x84\x82\x85\x01as\xD9V[``\x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83attV[`\0` \x82\x84\x03\x12\x15au\x12W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15au4Wau4a\\\xC4V[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15auSW`\0\x80\xFD[au[a\\\xDAV[\x825\x81R` \x83\x015aum\x81a\\\x96V[` \x82\x01Ra`\xE2`@\x84\x01a\\\xA8V[`\0\x82`\x1F\x83\x01\x12au\x8FW`\0\x80\xFD[\x815` au\x9Fac\x8E\x83ac;V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15au\xBEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805au\xD5\x81a\\\x96V[\x83R\x91\x83\x01\x91\x83\x01au\xC2V[`\0\x82`\x1F\x83\x01\x12au\xF3W`\0\x80\xFD[\x815` av\x03ac\x8E\x83ac;V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805\x83R\x91\x83\x01\x91\x83\x01av&V[`\0`@\x826\x03\x12\x15avOW`\0\x80\xFD[avWa]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15avnW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15av\x83W`\0\x80\xFD[av\x8Ba]\x02V[\x825\x81R` \x83\x015\x82\x81\x11\x15av\xA1W`\0\x80\xFD[av\xAD6\x82\x86\x01au~V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15av\xC5W`\0\x80\xFD[av\xD16\x82\x86\x01au\xE2V[`@\x83\x01RPav\xE3``\x84\x01a`0V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15av\xFEW`\0\x80\xFD[Pao\xE56\x82\x86\x01ai\x9BV[`\0`@\x826\x03\x12\x15aw\x1DW`\0\x80\xFD[aw%a]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aw<W`\0\x80\xFD[awH6\x83\x87\x01au\xE2V[\x83R` \x85\x015\x91P\x80\x82\x11\x15aw^W`\0\x80\xFD[Pao\xE56\x82\x86\x01acmV[` \x81\x01`\x02\x83\x10aw\x8DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03ar\0War\0aqgV[`\0`@\x82\x84\x03\x12\x15aw\xBEW`\0\x80\xFD[aw\xC6a]$V[\x90Paw\xD1\x82a\\\xA8V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw\xECW`\0\x80\xFD[ao\xE5\x84\x82\x85\x01acmV[`\0a\x0F\xE76\x83aw\xACV[`\x01\x81\x81\x1C\x90\x82\x16\x80ax\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18\x1CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a[\x01V[`\0` \x82\x84\x03\x12\x15axlW`\0\x80\xFD[PQ\x91\x90PV[`\0a\x0F\xE76\x83as\xD9V[`\0`\xA0\x82\x84\x03\x12\x15ax\x91W`\0\x80\xFD[ax\x99a]$V[\x90Pao\xBE\x83\x83ao\xFDV[`\0a\x0F\xE76\x83ax\x7FV[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15ax\xDDW`\0\x80\xFD[ax\xE5a]$V[\x90Pax\xF1\x83\x83annV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0a\x0F\xE76\x83ax\xCBV[`\0\x80\x85\x85\x11\x15ay(W`\0\x80\xFD[\x83\x86\x11\x15ay5W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x84\x03\x12\x15ayTW`\0\x80\xFD[ay\\a]hV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015ayz\x81a\\\x96V[`@\x82\x01Ray\x8B``\x83\x01ae\rV[``\x82\x01R`\x80\x82\x015ay\x9E\x81ac^V[`\x80\x82\x01Ray\xAF`\xA0\x83\x01a`0V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15ay\xCCW`\0\x80\xFD[a\x1F}\x83\x83ayBV[`\0`@\x82\x84\x03\x12\x15ay\xE8W`\0\x80\xFD[ay\xF0a]$V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15az\tW`\0\x80\xFD[az\x15\x85\x83\x86\x01au\xE2V[\x83R` \x91P\x81\x84\x015\x81\x81\x11\x15az,W`\0\x80\xFD[\x84\x01\x90P`\x1F\x81\x01\x85\x13az?W`\0\x80\xFD[\x805azMac\x8E\x82ac;V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15azlW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15az\x8AW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90azqV[\x80\x85\x87\x01RPPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15az\xACW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15az\xC2W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ay\xD6V[`\0` \x82\x84\x03\x12\x15az\xE0W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a{\x02Wa{\x02a\\\xC4V[`@R\x90P\x80a{\x11\x83a\\\xA8V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a{+W`\0\x80\xFD[a\x1F}\x83\x83az\xCEV[`\0`\xC0\x82\x84\x03\x12\x15a{GW`\0\x80\xFD[a{Oa]hV[\x90P\x815\x81R` \x82\x015a{c\x81a\\\x96V[` \x82\x01Ra{t`@\x83\x01a\\\xA8V[`@\x82\x01Ra{\x85``\x83\x01a\\\xA8V[``\x82\x01Ray\x9E`\x80\x83\x01a\\\xA8V[`\0`\xC0\x82\x84\x03\x12\x15a{\xA8W`\0\x80\xFD[a\x1F}\x83\x83a{5V[`\0` \x82\x84\x03\x12\x15a{\xC4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a{\xDAW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01as\x0BV[`\0` \x82\x84\x03\x12\x15a{\xF8W`\0\x80\xFD[\x81Qa\x1F}\x81a\\\x96V[`\0`\xE0\x82\x84\x03\x12\x15a|\x15W`\0\x80\xFD[a|\x1Da]$V[\x90Pa|)\x83\x83ayBV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a|VW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a|lW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a|\x03V[`\0``\x82\x84\x03\x12\x15a|\x8AW`\0\x80\xFD[a|\x92a\\\xDAV[\x90P\x815a|\x9F\x81a\\\x96V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a|\xBBW`\0\x80\xFD[a|\xC7\x85\x83\x86\x01as\xD9V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a|\xE0W`\0\x80\xFD[Par\xF3\x84\x82\x85\x01as\xD9V[`\0a\x0F\xE76\x83a|xV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x18\x1CW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x0F\xE76\x83a|\x03V[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a}cW`\0\x80\xFD[a}ka]\x8AV[\x825\x81R` \x83\x015a}}\x81a\\\x96V[` \x82\x01R`@\x83\x015a}\x90\x81a\\\x96V[`@\x82\x01Ra}\xA1``\x84\x01a})V[``\x82\x01Ra}\xB2`\x80\x84\x01a})V[`\x80\x82\x01Ra}\xC3`\xA0\x84\x01a`0V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a}\xDCW`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a}\xFAW`\0\x80\xFD[a~\x02a\\\xDAV[\x825a~\r\x81a\\\x96V[\x81Ra~\x1B` \x84\x01a\\\xA8V[` \x82\x01R`@\x83\x015a`\xE2\x81a_\xFEV[`\0`@\x826\x03\x12\x15a~@W`\0\x80\xFD[a~Ha]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a~_W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a~tW`\0\x80\xFD[a~|a\\\xDAV[\x825\x81R` \x83\x015\x82\x81\x11\x15a~\x92W`\0\x80\xFD[a~\x9E6\x82\x86\x01au~V[` \x83\x01RPa~\xB0`@\x84\x01a`0V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15av\xFEW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a~\xDDW`\0\x80\xFD[a~\xE5a]hV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x7F\x01`@\x83\x01ae\xB1V[`@\x82\x01R``\x82\x015ay\x8B\x81a\\\x96V[`\0`\xE0\x826\x03\x12\x15a\x7F&W`\0\x80\xFD[a\x7F.a]$V[a\x7F86\x84a~\xCBV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x7FSW`\0\x80\xFD[ao\xE56\x82\x86\x01ai\x9BV[`\0a\x0F\xE76\x83ay\xD6V[`\0`\x80\x826\x03\x12\x15a\x7F}W`\0\x80\xFD[a\x7F\x85a]\x02V[\x825a\x7F\x90\x81a\\\x96V[\x81Ra\x7F\x9E` \x84\x01ae\rV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x7F\xBDW`\0\x80\xFD[a\x7F\xC96\x83\x87\x01as\xD9V[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x7F\xE2W`\0\x80\xFD[Pat\xE86\x82\x86\x01as\xD9V[`\0`\xE0\x82\x84\x03\x12\x15a\x80\x01W`\0\x80\xFD[a\x80\ta]$V[\x90Pa|)\x83\x83a{5V[`\0a\x0F\xE76\x83a\x7F\xEFV[`\0\x82`\x1F\x83\x01\x12a\x802W`\0\x80\xFD[a\x80:a\\\xDAV[\x80``\x84\x01\x85\x81\x11\x15a\x80LW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x80fW\x805\x84R` \x93\x84\x01\x93\x01a\x80NV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x80\x83W`\0\x80\xFD[a\x80\x8Ba]FV[a\x80\x956\x84a\x80!V[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x80\xB1W`\0\x80\xFD[a\x80\xBD6\x83\x87\x01ai\x9BV[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x80\xD6W`\0\x80\xFD[a\x80\xE26\x83\x87\x01au\xE2V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x80\xFBW`\0\x80\xFD[Pa\x81\x086\x82\x86\x01au\xE2V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x810W`\0\x80\xFD[a\x1F}\x83\x83a~\xCBV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x81mWa\x81maqgV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x81\x92Wa\x81\x92aqgV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x81\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x81\xC3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ao\x98V[`\0` \x82\x84\x03\x12\x15a\x81\xE1W`\0\x80\xFD[\x81Qa\x1F}\x81ac^V[`\0` \x82\x84\x03\x12\x15a\x81\xFEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x82\x14W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aw\xACV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x82@Wa\x82@aqgV[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x82|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x82\x92W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a|xV[` \x81R`\0\x82Q``` \x84\x01Ra\x82\xBA`\x80\x84\x01\x82ag\x83V[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x82\xFBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x11W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01attV[`@\x81R`\0a\x830`@\x83\x01\x85aa\x9EV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x83ZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83pW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a\x7F\xEFV[`\0` \x80\x83\x85\x03\x12\x15a\x83\x8FW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\xA5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x83\xB6W`\0\x80\xFD[\x80Qa\x83\xC4ac\x8E\x82ac;V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x83\xE3W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x84\nW\x83Qa\x83\xFB\x81a\\\x96V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x83\xE8V[\x97\x96PPPPPPPV[\x82Q\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x84AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84WW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01arlV[` \x81R`\0a\x1F}` \x83\x01\x84a^\xFDV[`@\x81R`\0a\x84\x89`@\x83\x01\x85a^\xFDV[\x82\x81\x03` \x84\x01Ra_\x93\x81\x85a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x84\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xC3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ax\xCBV[`\0` \x82\x84\x03\x12\x15a\x84\xE1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xF7W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ax\x7FV[`\0\x82a\x85 WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x85]\x81`\x19\x85\x01` \x87\x01aZ\xD5V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x85\x90Wa\x85\x90aqgV[`\0\x03\x92\x91PPV\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)BurnLp(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)no slow mode transactions remainingMintLp(bytes32 sender,uint32 productId,uint128 amountBase,uint128 quoteAmountLow,uint128 quoteAmountHigh,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x88g\xF2\xB0\x8FLnd}n\xBE\x88\x14\x13\xDF\x9A\x12\xF0\x0Bg\xBB\xE5\xDB\xE8\xEEp+\xBD\xDFq}\x9FdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static ENDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\x8BW`\x005`\xE0\x1C\x80c\x85\xC8>\x9D\x11a\x02\xE2W\x80c\xB1\xFB\xD6\x0B\x11a\x01\x91W\x80c\xDBZP!\x11a\0\xEEW\x80c\xEERU&\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\x0F\xA8W\x80c\xF8\x0F|\xE5\x14a\x0C9W\x80c\xFB\xF4\x19\x84\x14a\x0E\x83W`\0\x80\xFD[\x80c\xEERU&\x14a\x0FJW\x80c\xEFd\xED\x0E\x14a\x0FlW\x80c\xF2c9'\x14a\x0F\x95W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x0E\xD1W\x80c\xE9\xABw\xE5\x14a\x0E\xE4W\x80c\xE9\xBCtb\x14a\x0F7W`\0\x80\xFD[\x80c\xDBZP!\x14a\x0E\x91W\x80c\xDCB\xE6\x1B\x14a\x0E\xB1W`\0\x80\xFD[\x80c\xC3ES\x0B\x11a\x01EW\x80c\xC5\x105\x9F\x11a\x01*W\x80c\xC5\x105\x9F\x14a\x0E\\W\x80c\xD3\x8C;\x9C\x14a\x0EcW\x80c\xD4\xDE\x8D\x9D\x14a\x0E\x83W`\0\x80\xFD[\x80c\xC3ES\x0B\x14a\x0E8W\x80c\xC4\xF9\xB2_\x14a\x0EKW`\0\x80\xFD[\x80c\xB3d\x88\xB8\x11a\x01vW\x80c\xB3d\x88\xB8\x14a\x0E\x05W\x80c\xB7\x0E\xB2c\x14a\x0E%W\x80c\xBC\x85\xCA\x86\x14a\x05\xB9W`\0\x80\xFD[\x80c\xB1\xFB\xD6\x0B\x14a\r\xC5W\x80c\xB2\xBBcg\x14a\r\xE5W`\0\x80\xFD[\x80c\x91q\xD0\x8B\x11a\x02?W\x80c\x98\xCD2\xFE\x11a\x01\xF3W\x80c\x9E\x85\x14$\x11a\x01\xD8W\x80c\x9E\x85\x14$\x14a\r\x85W\x80c\xA0\x82\xC5\xAA\x14a\r\xA5W\x80c\xB1\xC8\xEC+\x14a\n\x15W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x14a\r%W\x80c\x9A\x08\xE55\x14a\r8W`\0\x80\xFD[\x80c\x94\xFA\xEF\xE5\x11a\x02$W\x80c\x94\xFA\xEF\xE5\x14a\x0C9W\x80c\x954\xDD>\x14a\x0CdW\x80c\x96\xC4|o\x14a\x0C\x84W`\0\x80\xFD[\x80c\x91q\xD0\x8B\x14a\x0B\xF0W\x80c\x91\xC1\xE3\xD7\x14a\x0C\x10W`\0\x80\xFD[\x80c\x8D< \xB1\x11a\x02\x96W\x80c\x8E]X\x8C\x11a\x02{W\x80c\x8E]X\x8C\x14a\x0B\xACW\x80c\x8FO\x8E\xCC\x14a\x0B\xBFW\x80c\x91\x0E`j\x14a\x0B\xD0W`\0\x80\xFD[\x80c\x8D< \xB1\x14a\x0B{W\x80c\x8D\xA5\xCB[\x14a\x0B\x9BW`\0\x80\xFD[\x80c\x8C=/t\x11a\x02\xC7W\x80c\x8C=/t\x14a\x0B\x07W\x80c\x8CX\xE1\n\x14a\x0B$W\x80c\x8D\n\xCC\x9B\x14a\x0BhW`\0\x80\xFD[\x80c\x85\xC8>\x9D\x14a\n\xD4W\x80c\x872C8\x14a\n\xF4W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04>W\x80cZ\0\x92;\x11a\x03\x9BW\x80clEup\x11a\x03OW\x80co;\nr\x11a\x034W\x80co;\nr\x14a\n\x99W\x80cqP\x18\xA6\x14a\n\xB9W\x80c}\xB6\xA2[\x14a\n\xC1W`\0\x80\xFD[\x80clEup\x14a\nfW\x80cl\xFE_\xE4\x14a\n\x86W`\0\x80\xFD[\x80c]O_\x97\x11a\x03\x80W\x80c]O_\x97\x14a\nKW\x80ca\x0B.^\x14a\x06.W\x80ce\xDD\x13f\x14a\n^W`\0\x80\xFD[\x80cZ\0\x92;\x14a\n\x15W\x80c[\xB4\xC1&\x14a\n5W`\0\x80\xFD[\x80c>\xDF,[\x11a\x03\xF2W\x80cM\x96\xA9\n\x11a\x03\xD7W\x80cM\x96\xA9\n\x14a\t\xBDW\x80cTDV\x9D\x14a\t\xE2W\x80cU~\xD1\xBA\x14a\t\xF5W`\0\x80\xFD[\x80c>\xDF,[\x14a\t}W\x80cB\xC7M\x1D\x14a\t\x9DW`\0\x80\xFD[\x80c6\xB9\x0CQ\x11a\x04#W\x80c6\xB9\x0CQ\x14a\x08\xF7W\x80c8B\xE7^\x14a\t\x17W\x80c<\xECK\x93\x14a\t9W`\0\x80\xFD[\x80c5c\x9AO\x14a\x08qW\x80c6\x8EF\x86\x14a\x08\xD1W`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x04\xECW\x80c,\x8Co\xFB\x11a\x04\xA0W\x80c-\x035\xAB\x11a\x04\x85W\x80c-\x035\xAB\x14a\x07\xCBW\x80c/\x9A'D\x14a\x07\xFDW\x80c2\x16\xC0b\x14a\x08\x10W`\0\x80\xFD[\x80c,\x8Co\xFB\x14a\x07\x8BW\x80c,\xD7\x1B\x16\x14a\x07\xABW`\0\x80\xFD[\x80c\"\0`F\x11a\x04\xD1W\x80c\"\0`F\x14a\x07GW\x80c\"\x1F\t9\x14a\x07OW\x80c\"\xD4\xA8-\x14a\x07bW`\0\x80\xFD[\x80c\x1F\x18k'\x14a\x07*W\x80c!\x04u\x89\x14a\x07?W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05CW\x80c\x18\xED\x16\xEB\x11a\x05(W\x80c\x18\xED\x16\xEB\x14a\x06\xE0W\x80c\x1D\x97\xD2/\x14a\x06iW\x80c\x1D\x9E\xED\xA5\x14a\x06\xF3W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06iW\x80c\x0FKP\x9D\x14a\x06\x89W`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05tW\x80c\x07H\xA2\x19\x14a\x05\xD9W\x80c\rU\xE2k\x14a\x06.W\x80c\r\x8En,\x14a\x06NW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\x90W\x80c\x06\xC0\xBA\xFD\x14a\x05\xB9W[`\0\x80\xFD[a\x05\xA3a\x05\x9E6`\x04aY\xD0V[a\x0F\xBBV[`@Qa\x05\xB0\x91\x90aY\xECV[`@Q\x80\x91\x03\x90\xF3[a\x05\xCCa\x05\xC76`\x04aZ(V[a\x0F\xEDV[`@Qa\x05\xB0\x91\x90aZDV[a\x05\xECa\x05\xE76`\x04aZ(V[a\x10 V[`@Qa\x05\xB0\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06Aa\x06<6`\x04aZ\x99V[a\x10SV[`@Qa\x05\xB0\x91\x90a[\x8CV[`\x1B[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x06|a\x06w6`\x04aZ(V[a\x10dV[`@Qa\x05\xB0\x91\x90a[\x9FV[a\x06\x9Ca\x06\x976`\x04aZ(V[a\x10\x97V[`@Qa\x05\xB0\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[`\xA6Ta\x06Q\x90`\x01`\x01`@\x1B\x03\x16\x81V[a\x07\x06a\x07\x016`\x04a[\xEEV[a\x10\xCAV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x0F\x0B\x92\x81\x01\x92\x90\x92R\x01a\x05\xB0V[a\x07=a\x0786`\x04a\\UV[a\x10\xEDV[\0[a\x07=a\x11\x89V[a\x07=a\x129V[a\x07=a\x07]6`\x04a^3V[a\x12xV[a\x06Qa\x07p6`\x04a^\xB0V[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x07\x9Ea\x07\x996`\x04a^\xC9V[a\x15\xB7V[`@Qa\x05\xB0\x91\x90a_;V[a\x07\xBEa\x07\xB96`\x04a_\x9CV[a\x15\xE4V[`@Qa\x05\xB0\x91\x90a_\xD0V[a\x06Qa\x07\xD96`\x04a`\x13V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x07=a\x08\x0B6`\x04a`GV[a\x16\x02V[a\x07=a\x08\x1E6`\x04a`\xA0V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\x08\x84a\x08\x7F6`\x04aZ(V[a\x17\x94V[`@Qa\x05\xB0\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\x08\xE4a\x08\xDF6`\x04a`\xEEV[a\x17\xC7V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x05\xB0V[a\t\na\t\x056`\x04aa\x0BV[a\x18\"V[`@Qa\x05\xB0\x91\x90aa\xDDV[a\t*a\t%6`\x04ab\x02V[a\x183V[`@Q\x90Q\x81R` \x01a\x05\xB0V[a\tLa\tG6`\x04aY\xD0V[a\x18QV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xB0V[a\t\x90a\t\x8B6`\x04a_\x9CV[a\x18}V[`@Qa\x05\xB0\x91\x90ab\x84V[a\t\xB0a\t\xAB6`\x04a_\x9CV[a\x18\x8EV[`@Qa\x05\xB0\x91\x90ac\x02V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x07=a\t\xF06`\x04ac\xE1V[a\x18\xABV[a\t\xFDa\x1C\xE5V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xB0V[a\n(a\n#6`\x04a_\x9CV[a\x1DsV[`@Qa\x05\xB0\x91\x90ad\xA1V[a\n=a\x1D\x91V[`@Q\x90\x81R` \x01a\x05\xB0V[`\x9ATa\t\xCA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x07=a\x1F\x84V[a\nya\nt6`\x04ad\xC6V[a\x1F\xC7V[`@Qa\x05\xB0\x91\x90ad\xFAV[a\x07=a\n\x946`\x04ae\x1DV[a\x1F\xD8V[a\n\xACa\n\xA76`\x04aZ\x99V[a \x05V[`@Qa\x05\xB0\x91\x90aeRV[a\x07=a \x16V[a\x07=a\n\xCF6`\x04ae\xC2V[a *V[a\n\xE7a\n\xE26`\x04aa\x0BV[a\";V[`@Qa\x05\xB0\x91\x90af7V[a\x07=a\x0B\x026`\x04af\xC1V[a\"LV[a\x0B\x0Fa)\xC9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x05\xB0V[a\x07=a\x0B26`\x04ag\x15V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x07=a\x0Bv6`\x04agNV[a+\xB5V[a\x0B\x8Ea\x0B\x896`\x04a^\xC9V[a,\xE4V[`@Qa\x05\xB0\x91\x90ag\xC1V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\t\xCAV[a\x07=a\x0B\xBA6`\x04ag\xECV[a,\xF5V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\t\xCAV[a\x0B\xE3a\x0B\xDE6`\x04ah1V[a-eV[`@Qa\x05\xB0\x91\x90ahMV[a\x0C\x03a\x0B\xFE6`\x04ah\xC2V[a-\xA6V[`@Qa\x05\xB0\x91\x90ah\xF6V[a\t\xCAa\x0C\x1E6`\x04a^\xB0V[`\0\x90\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x0CLa\x0CG6`\x04ab\x02V[a-\xB7V[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x05\xB0V[a\x0Cwa\x0Cr6`\x04a`\x13V[a-\xD5V[`@Qa\x05\xB0\x91\x90ailV[a\x0C\x97a\x0C\x926`\x04ai\x7FV[a.oV[`@Qa\x05\xB0\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x07=a\r36`\x04ai\xBBV[a.\xB7V[a\rKa\rF6`\x04aY\xD0V[a0\x1FV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xB0V[a\r\x98a\r\x936`\x04ah1V[a0KV[`@Qa\x05\xB0\x91\x90aj^V[a\r\xB8a\r\xB36`\x04a_\x9CV[a0\x8CV[`@Qa\x05\xB0\x91\x90aj\xB5V[a\r\xD8a\r\xD36`\x04ah\xC2V[a0\x9DV[`@Qa\x05\xB0\x91\x90ak\x16V[a\r\xF8a\r\xF36`\x04a_\x9CV[a0\xAEV[`@Qa\x05\xB0\x91\x90akrV[a\x0E\x18a\x0E\x136`\x04aa\x0BV[a0\xCBV[`@Qa\x05\xB0\x91\x90ak\xCFV[a\n=a\x0E36`\x04af\xC1V[a0\xDCV[a\x0B\x0Fa\x0EF6`\x04af\xC1V[a1yV[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x06QV[`\0a\x08\xE4V[a\x0Eva\x0Eq6`\x04ah\xC2V[a2YV[`@Qa\x05\xB0\x91\x90al$V[g\r\xE0\xB6\xB3\xA7d\0\0a\x08\xE4V[a\x0E\xA4a\x0E\x9F6`\x04ah\xC2V[a2jV[`@Qa\x05\xB0\x91\x90al\x8BV[a\x0E\xC4a\x0E\xBF6`\x04ah1V[a2{V[`@Qa\x05\xB0\x91\x90am(V[a\x07=a\x0E\xDF6`\x04agNV[a2\xBCV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x05\xB0V[a\x07=a\x0FE6`\x04am\x80V[a6PV[a\x0F]a\x0FX6`\x04am\xF7V[a6\xDEV[`@Qa\x05\xB0\x93\x92\x91\x90an\x12V[a\n=a\x0Fz6`\x04am\xF7V[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x07=a\x0F\xA36`\x04a`\x13V[a7\xF9V[a\x07=a\x0F\xB66`\x04a`\x13V[a8#V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83an\xAFV[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ao&V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83aoBV[a\x10[aV\xC0V[a\x0F\xE7\x82ao\xF1V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ap3V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83apOV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ap\xADV[`\0[\x81\x81\x10\x15a\x11BW6`\0\x84\x84\x84\x81\x81\x10a\x11\rWa\x11\raq\x0BV[\x90P` \x02\x81\x01\x90a\x11\x1F\x91\x90aq!V[\x91P\x91Pa\x11-\x82\x82a8\xB3V[PP\x80\x80a\x11:\x90aq}V[\x91PPa\x10\xF0V[P`\xA6\x80T\x82\x91\x90`\0\x90a\x11a\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aq\x96V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x08\x1EW`\xA8`\0\x82` \x01\x80Qa\x11\xF7\x90aq\xC1V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x122`\x01\x83\x01\x82aV\xF9V[PPa\x11\xC2V[`\xA6\x80T`\0\x90a\x12R\x90`\x01`\x01`@\x1B\x03\x16aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPV[\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbIRC`\xE8\x1B` \x82\x01R\x90a\x12\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01R`]c\xFF\xFF\xFF\xFF\x85\x16\x03a\x12\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P``\x84\x90\x1Ca\x133\x813\x81\x14a\x13-W`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaJzV[\x83aJzV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA6\x91\x90ar\nV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\xBBW`\0\x80\xFD[a\x13\xCF\x813\x86`\x01`\x01`\x80\x1B\x03\x16aJ\xD3V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R`\0a\x14\x16Bb\x03\xF4\x80aq\x96V[`@\x80Q``\x81\x01\x82R`\x01`\x01`@\x1B\x03\x83\x16\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x91\x92P\x81\x01`\x01`@Q\x80``\x01`@R\x80\x8C\x81R` \x01\x8Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x14\xA7\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14\xC5\x92\x91` \x01ar=V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x84` \x01\x80Q\x80\x91\x90a\x14\xED\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x15Y\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x82Q`\xA7\x80T` \x86\x01Q`@\x90\x96\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x97\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x94\x90\x94\x16\x17\x90\x92UPPPPPPPPV[a\x15\xDB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x0F\xE7\x82ar\xFFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x0F\xE7\x82asMV[`\0Z`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x91\x92P`\x01`\x01`@\x1B\x03\x87\x81\x16\x91\x16\x14a\x16OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0[\x83\x81\x10\x15a\x17\x1BW6`\0\x86\x86\x84\x81\x81\x10a\x16pWa\x16paq\x0BV[\x90P` \x02\x81\x01\x90a\x16\x82\x91\x90aq!V[\x91P\x91Pa\x16\x90\x82\x82a8\xB3V[\x84Za\x16\x9C\x90\x86asYV[\x11\x15a\x17\x06W`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xEDW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x01W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x17\x13\x90aq}V[\x91PPa\x16SV[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x179\x90\x85asYV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17uW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83as\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x18\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x91\x90PV[a\x18*aW\xB7V[a\x0F\xE7\x82at\xF4V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x0F\xE76\x83\x90\x03\x83\x01\x83au\0V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83auAV[a\x18\x85aW\xE5V[a\x0F\xE7\x82av=V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x0F\xE7\x82aw\x0BV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x18\xCBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x18\xE5WP0;\x15\x80\x15a\x18\xE5WP`\0T`\xFF\x16`\x01\x14[a\x19WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12\xB2V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x19zW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x19\x82aJ\xE9V[a\x19\xF6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaK\\V[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1Aq\x90`\0\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xB2\x91\x90ar\nV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1A\xF6\x90`\x01\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B7\x91\x90ar\nV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB9\x91\x90ar\nV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1C\x95W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1CDWa\x1CDaq\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1C\x8D\x81aw\x93V[\x91PPa\x1C\x1DV[P\x80\x15a\x1C\xDCW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1D\"W\x81Qa\x1D(V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1DlW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x0F\xE7\x82aw\xF8V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a\x1E&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a\x1E\x84\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xB0\x90ax\x04V[\x80\x15a\x1E\xFDW\x80`\x1F\x10a\x1E\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a\x1F7\x92\x90\x91`\x04\x01ax8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1FpWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1Fm\x91\x81\x01\x90axZV[`\x01[a\x1F}WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x08\x1E\x81`\0aK\xD1V[a\x1F\xCFaX\x14V[a\x0F\xE7\x82axsV[a\x1F\xE0aN<V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a \raXKV[a\x0F\xE7\x82ax\xA5V[a \x1EaN<V[a (`\0aN\x96V[V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x88\x81\x16\x91\x16\x14a rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a \x8AW`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a!$W\x81\x87\x87\x83\x81\x81\x10a \xD7Wa \xD7aq\x0BV[\x90P` \x02\x81\x01\x90a \xE9\x91\x90aq!V[`@Q` \x01a \xFB\x93\x92\x91\x90ax\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a!\x1D\x90aq}V[\x90Pa \xBCV[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\x96W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a!\xEFW6`\0\x88\x88\x84\x81\x81\x10a!\xBAWa!\xBAaq\x0BV[\x90P` \x02\x81\x01\x90a!\xCC\x91\x90aq!V[\x91P\x91Pa!\xDA\x82\x82a8\xB3V[PP\x80\x80a!\xE7\x90aq}V[\x91PPa!\x9DV[P`\xA6\x80T\x86\x91\x90`\0\x90a\"\x0E\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aq\x96V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPPPPV[a\"CaX{V[a\x0F\xE7\x82ay\x0CV[30\x14a\"XW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a\"mWa\"maq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a\"\x88Wa\"\x88ar'V[\x90P`\0\x81`\x18\x81\x11\x15a\"\x9EWa\"\x9Ear'V[\x03a#BW`\0a\"\xB2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a\"\xBF\x91\x90ay\xBAV[\x90Pa\"\xCF\x81`\0\x01Q\x86aN\xE8V[\x80Qa\"\xDA\x90aOFV[`\x9AT`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cR\xEF\xAD\xF1\x90a#\n\x90\x84\x90`\x04\x01aj^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#8W=`\0\x80>=`\0\xFD[PPPPPa)\xC3V[`\x01\x81`\x18\x81\x11\x15a#VWa#Var'V[\x03a#\xE6W`\0a#j\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a#w\x91\x90auAV[\x90Pa#\x87\x81`\0\x01Q\x86aN\xE8V[\x80Qa#\x92\x90aO\xA4V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01a#\nV[`\x02\x81`\x18\x81\x11\x15a#\xFAWa#\xFAar'V[\x03a$\x8BW`\0a$\x0E\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a$\x1B\x91\x90ao&V[\x90Pa$+\x81`\0\x01Q\x86aN\xE8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x82A\x8Ck`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a#\nV[`\x05\x81`\x18\x81\x11\x15a$\x9FWa$\x9Far'V[\x03a$\xF1W`\0a$\xB3\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a$\xC0\x91\x90az\x9AV[`\x9AT`@Qc\xB2\xBBcg`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xBBcg\x90a#\n\x90\x84\x90`\x04\x01akrV[`\x07\x81`\x18\x81\x11\x15a%\x05Wa%\x05ar'V[\x03a%aW`\0a%\x19\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a%&\x91\x90a{\x19V[`\x9AT`@Qc:\x91\xC5\x8B`\xE0\x1B\x81R\x82Q`\x01`\x01`\x80\x1B\x03\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c:\x91\xC5\x8B\x90`$\x01a#\nV[`\t\x81`\x18\x81\x11\x15a%uWa%uar'V[\x03a'\x05W`\0a%\x89\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a%\x96\x91\x90a{\x96V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a%\xC8\x90`\0\x90`\x04\x01awkV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\t\x91\x90ar\nV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x85\x91\x90ar\nV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a&\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P\x80Qa&\xD5\x90\x86aN\xE8V[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a#\n\x90\x84\x90`\x04\x01ahMV[`\n\x81`\x18\x81\x11\x15a'\x19Wa'\x19ar'V[\x03a'zW`\0a'-\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a':\x91\x90ao&V[\x90Pa'J\x81`\0\x01Q\x86aN\xE8V[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a#\n\x90\x84\x90`\x04\x01aZDV[`\x0B\x81`\x18\x81\x11\x15a'\x8EWa'\x8Ear'V[\x03a(&W`\0a'\xA2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a'\xAF\x91\x90apOV[\x90Pa'\xBF\x81`\0\x01Q\x86aN\xE8V[\x80Qa'\xCA\x90aOFV[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a#\nV[`\x12\x81`\x18\x81\x11\x15a(:Wa(:ar'V[\x03a(\x90W`\0a(N\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a([\x91\x90a{\xB2V[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a#\n\x91`\x04\x01ailV[`\x13\x81`\x18\x81\x11\x15a(\xA4Wa(\xA4ar'V[\x03a)\x13W`\0a(\xB8\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a(\xC5\x91\x90an\xAFV[\x90Pa(\xD5\x81`\0\x01Q\x86aN\xE8V[\x80Qa(\xE0\x90aOFV[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua)\xC3V[`\x15\x81`\x18\x81\x11\x15a)'Wa)'ar'V[\x03a\x05\x8BW`\0a);\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a)H\x91\x90aoBV[\x90Pa)X\x81`\0\x01Q\x86aN\xE8V[a)e\x81``\x01QaO\xA4V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a#\nV[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a*^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a*\xBC\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xE8\x90ax\x04V[\x80\x15a+5W\x80`\x1F\x10a+\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\x18W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a+o\x92\x90\x91`\x04\x01ax8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a+\xA8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+\xA5\x91\x81\x01\x90a{\xE6V[`\x01[a\x1F}W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a+\xCAWa+\xCAaq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a+\xE5Wa+\xE5ar'V[\x90P`\0\x81`\x18\x81\x11\x15a+\xFBWa+\xFBar'V[\x03a,MW`\0a,\x0F\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a,\x1C\x91\x90a|DV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa,2\x91aP3V[\x80QQa,G\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[Pa,\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a,\xBA\x83aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a,\xECaX\xA4V[a\x0F\xE7\x82a|\xEDV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra-`\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra->\x90a|\xF9V[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x12xV[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a{\x96V[a-\xAEaX\xD1V[a\x0F\xE7\x82a}\x1DV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a{\x19V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta-\xEE\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta.\x1A\x90ax\x04V[\x80\x15a.gW\x80`\x1F\x10a.<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a.gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a.JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a}QV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a/CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12\xB2V[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a/Z\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a/\xC6\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a}\xE8V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83ay\xBAV[a0\x94aY\x10V[a\x0F\xE7\x82a~.V[a0\xA5aX\xD1V[a\x0F\xE7\x82a\x7F\x14V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x0F\xE7\x82a\x7F_V[a0\xD3aY8V[a\x0F\xE7\x82a\x7FkV[`\0\x80\x83\x83`\0\x81\x81\x10a0\xF2Wa0\xF2aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a1\rWa1\rar'V[\x90P`\x13\x81`\x18\x81\x11\x15a1#Wa1#ar'V[\x03a1nW`\0a17\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1D\x91\x90an\xAFV[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a1bW`\0a1eV[\x80Q[\x92PPPa\x1F}V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a1\x8FWa1\x8Faq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a1\xAAWa1\xAAar'V[\x90P`\t\x81`\x18\x81\x11\x15a1\xC0Wa1\xC0ar'V[\x03a1\xEFW`\0a1\xD4\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90a{\x96V[` \x01Q\x92Pa\x1F}\x91PPV[`\n\x81`\x18\x81\x11\x15a2\x03Wa2\x03ar'V[\x03a2$W`\0a2\x17\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90ao&V[`\x0B\x81`\x18\x81\x11\x15a28Wa28ar'V[\x03a1nW`\0a2L\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a1\xE1\x91\x90apOV[a2aaX\xD1V[a\x0F\xE7\x82a\x80\x15V[a2raYWV[a\x0F\xE7\x82a\x80qV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x0F\xE76\x83\x90\x03\x83\x01\x83a\x81\x1EV[`\0\x82\x82`\0\x81\x81\x10a2\xD1Wa2\xD1aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a2\xECWa2\xECar'V[\x90P3`\x01\x82`\x18\x81\x11\x15a3\x03Wa3\x03ar'V[\x03a3\rW`\0\x80\xFD[`\x07\x82`\x18\x81\x11\x15a3!Wa3!ar'V[\x03a3\x82W`\0a35\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a3B\x91\x90a{\x19V[\x90P`\0a3NaP\xEFV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3cW`\0\x80\xFD[a3{\x81\x84\x84`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16aJ\xD3V[PPa4\xCAV[`\x12\x82`\x18\x81\x11\x15a3\x96Wa3\x96ar'V[\x03a3\xBAW`gT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a3\xB5W`\0\x80\xFD[a4\xCAV[`\x15\x82`\x18\x81\x11\x15a3\xCEWa3\xCEar'V[\x03a4zW`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a4*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0a4:\x84`\x01\x81\x88ay\x18V[\x81\x01\x90a4G\x91\x90aoBV[\x90Pa4t\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaJzV[Pa4\xCAV[`\0a4\x84aP\xEFV[\x90P`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a4\xA1\x90\x84\x90`\x0F\x0Ba\x81:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPP[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R`\0a5\x11Bb\x03\xF4\x80aq\x96V[\x90P`@Q\x80``\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x85\x01\x80Q`\xA8\x93P\x90a5\x88\x82aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a5\xF4\x92`\x01\x85\x01\x92\x90\x91\x01\x90aW3V[PP\x82Q`\xA7\x80T` \x86\x01Q`@\x90\x96\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x97\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x94\x90\x94\x16\x17\x90\x92UPPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra6\xD7\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra6\x99\x90a|\xF9V[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x12x\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a7h\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta7\x94\x90ax\x04V[\x80\x15a7\xE1W\x80`\x1F\x10a7\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a7\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a7\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a8\x01aN<V[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a8+aN<V[`\x01`\x01`\xA0\x1B\x03\x81\x16a8\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12\xB2V[a8\xB0\x81aN\x96V[PV[`\0\x82\x82`\0\x81\x81\x10a8\xC8Wa8\xC8aq\x0BV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x18\x81\x11\x15a8\xE3Wa8\xE3ar'V[\x90P`\0\x81`\x18\x81\x11\x15a8\xF9Wa8\xF9ar'V[\x03a:ZW`\0a9\r\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a9\x1A\x91\x90a|DV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa90\x91aP3V[`\0a9\xD9`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a\x86^`w\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aQaV[\x90P\x81QQa9\xE7\x90aOFV[\x81QQa9\xFC\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a:,\x91`\x04\x01aj^V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[`\x02\x81`\x18\x81\x11\x15a:nWa:nar'V[\x03a<\x0EW`\0a:\x82\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a:\x8F\x91\x90a\x81\x9BV[\x80Q\x80Q``\x90\x91\x01Q\x91\x92Pa:\xA5\x91aP3V[`\0a; `@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a\x87n`O\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90P\x81Q\x80Q`\x9BT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ra;\xAC\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xA1\x91\x90a\x81\xCFV[\x84Q` \x01QaQ\xAFV[`\x9AT\x82Q\x80Q` \x82\x01Q`@\x92\x83\x01Q\x92Qc\x82A\x8Ck`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16`$\x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a:,V[`\x03\x81`\x18\x81\x11\x15a<\"Wa<\"ar'V[\x03a=*W`\0a<6\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a<C\x91\x90a\x81\xECV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15a<\x8FW` \x82\x01Q\x83Qa<\x8A\x91\x90a\x82 V[a<\x92V[`\0[`\x9BT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a<\xCB\x91\x85\x91\x90`\x04\x01a\x82HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xF9W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa-`\x91PPV[`\x0F\x81`\x18\x81\x11\x15a=>Wa=>ar'V[\x03a>?W`\0a=R\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a=_\x91\x90a\x81\xECV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15a=\xA7W\x81Q\x83Qa=\xA2\x91\x90a\x82 V[a=\xAAV[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a=\xE3\x91\x85\x91\x90`\x04\x01a\x82HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\x11W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa-`\x91PPV[`\x04\x81`\x18\x81\x11\x15a>SWa>Sar'V[\x03a>\x88W`\0a>g\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a>t\x91\x90ap\xADV[\x90Pa)\xC3\x81`\0\x01Q\x82` \x01QaR\x86V[`\x05\x81`\x18\x81\x11\x15a>\x9CWa>\x9Car'V[\x03a?&W`\0a>\xB0\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a>\xBD\x91\x90az\x9AV[`\x9AT`@Qc\xB2\xBBcg`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB2\xBBcg\x90a>\xEE\x90\x84\x90`\x04\x01akrV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\x1CW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x06\x81`\x18\x81\x11\x15a?:Wa?:ar'V[\x14\x80a?WWP`\x16\x81`\x18\x81\x11\x15a?UWa?Uar'V[\x14[\x15a@\x08W`\0a?k\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a?x\x91\x90a\x82jV[` \x81\x01QQQ\x90\x91Pa?\x8B\x90aOFV[`@\x81\x01QQQa?\x9B\x90aOFV[`@\x80Q``\x81\x01\x82R\x82\x81R` \x80\x84\x01QQQ`\0\x90\x81R`\xAB\x80\x83R\x84\x82 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84\x86\x01R\x86\x86\x01QQQ\x83R\x92R\x83\x90 T\x81\x16\x82\x84\x01R`\xB0T\x92Qc\x11\x17\x8F-`\xE3\x1B\x81R\x91\x92\x16\x90c\x88\xBCyh\x90a:,\x90\x84\x90`\x04\x01a\x82\x9EV[`\x0C\x81`\x18\x81\x11\x15a@\x1CWa@\x1Car'V[\x03a@\x9BW`\0a@0\x83`\x01\x81\x87ay\x18V[\x81\x01\x90a@=\x91\x90a\x82\xE9V[``\x81\x01QQQ\x90\x91Pa@P\x90aOFV[`\xB0T``\x82\x01QQQ`\0\x90\x81R`\xAB` R`@\x90\x81\x90 T\x90Qc<xi\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92cx\xF0\xD3\xCE\x92a>\xEE\x92\x86\x92\x90\x91\x16\x90`\x04\x01a\x83\x1DV[`\x08\x81`\x18\x81\x11\x15a@\xAFWa@\xAFar'V[\x03aAMW`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra@\xF7\x81`\x01aK\xD1V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81`\x18\x81\x11\x15aAaWaAaar'V[\x03aBzW`\0aAu\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aA\x82\x91\x90a\x83HV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaA\x98\x91aP3V[`\0aB3`@Q\x80`\xA0\x01`@R\x80`v\x81R` \x01a\x86\xF8`v\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93R\x90\x82\x16\x92\x85\x01\x92\x90\x92R\x16`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01a9\xBEV[\x90P\x81QQaBJ\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91a:,\x91`\x04\x01ahMV[`\n\x81`\x18\x81\x11\x15aB\x8EWaB\x8Ear'V[\x03aC\x87W`\0aB\xA2\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aB\xAF\x91\x90a\x81\x9BV[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaB\xC5\x91aP3V[`\0aC@`@Q\x80`\x80\x01`@R\x80`C\x81R` \x01a\x86\x1B`C\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90P\x81QQaCW\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91a:,\x91`\x04\x01aZDV[`\r\x81`\x18\x81\x11\x15aC\x9BWaC\x9Bar'V[\x03aD\x04W`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xDCW=`\0\x80>=`\0\xFD[`\x0E\x81`\x18\x81\x11\x15aD\x18WaD\x18ar'V[\x03aFYW`\0aD,\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aD9\x91\x90au\0V[\x90P`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaD\xB8\x91\x90\x81\x01\x90a\x83|V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xD5WaD\xD5a\\\xC4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\xFEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15aE\xE1W`\xAA`\0\x84\x83\x81Q\x81\x10aE#WaE#aq\x0BV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10aEeWaEeaq\x0BV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10aE\x91WaE\x91aq\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aE\xD9\x81aq}V[\x91PPaE\x04V[P\x82QaE\xED\x90aOFV[`\x9AT`@Qcx\x1C\x85\x7F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF09\n\xFE\x90aF\x1F\x90\x86\x90\x85\x90`\x04\x01a\x84\x15V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15aFMW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\x10\x81`\x18\x81\x11\x15aFmWaFmar'V[\x03aGUW`\0aF\x81\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aF\x8E\x91\x90a\x84/V[`\x9CT\x81Q`@Qc\x9Bov+`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x9Bov+\x91aF\xC1\x91`\x04\x01a\x84cV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aF\xD9W`\0\x80\xFD[PZ\xFA\x15\x80\x15aF\xEDW=`\0\x80>=`\0\xFD[PP`\x9BT` \x84\x01Q`@\x80\x86\x01Q\x90Qc\x03\tr\xB5`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94Pc0\x97+P\x93PaG)\x92`\x04\x01a\x84vV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15aGAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a?\x1CW=`\0\x80>=`\0\xFD[`\x13\x81`\x18\x81\x11\x15aGiWaGiar'V[\x03aH3W`\0aG}\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aG\x8A\x91\x90a\x84\x9BV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaG\xA0\x91aP3V[`\0aG\xFC`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x85\xE5`6\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a9\xBEV[\x90PPQ` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81`\x18\x81\x11\x15aHGWaHGar'V[\x03aH\xC9W`\0aH[\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aHh\x91\x90as\x82V[`\xB0T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xB7mx\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x93\x16`$\x84\x01R`\x07\x90\x81\x0B`D\x84\x01R\x0B`d\x82\x01R\x92\x93P\x16\x90c\xB7mx\xE3\x90`\x84\x01a>\xEEV[`\x17\x81`\x18\x81\x11\x15aH\xDDWaH\xDDar'V[\x03aI\xE1W`\0aH\xF1\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aH\xFE\x91\x90a\x84\xCFV[\x90P`\0aIu`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a\x85\x9A`K\x919\x80Q` \x91\x82\x01 \x84Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a9\xBEV[\x90PaI\x88\x82`\0\x01Q` \x01QaOFV[\x81Q\x80Q``\x90\x91\x01QaI\x9C\x91\x90aP3V[\x81QQaI\xB1\x90g\r\xE0\xB6\xB3\xA7d\0\0aP\xE3V[`\x9AT\x82Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a:,\x91`\x04\x01a[\x9FV[`\x18\x81`\x18\x81\x11\x15aI\xF5WaI\xF5ar'V[\x03a\x05\x8BW`\0aJ\t\x83`\x01\x81\x87ay\x18V[\x81\x01\x90aJ\x16\x91\x90a}\xE8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x82A\x8Ck`\xE0\x1B\x81R`\x01`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x93\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`d\x83\x01R\x92\x93P\x91\x16\x90c\x82A\x8Ck\x90`\x84\x01a>\xEEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaJ\x9D\x90ax\x04V[\x90P`\0\x03aJ\xCFW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa-`\x92\x84\x01\x90aW3V[PPV[`\x9ATa-`\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\0Ta\x01\0\x90\x04`\xFF\x16aKTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[a (aS\xC6V[`\0Ta\x01\0\x90\x04`\xFF\x16aK\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[aJ\xCF\x82\x82aT:V[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x86\xD5`#\x919\x90aL%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aL\x84\x90ax\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaL\xB0\x90ax\x04V[\x80\x15aL\xFDW\x80`\x1F\x10aL\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aL\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aL\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aM\x1E\x90aq\xE4V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aM[`\x01\x83\x01\x82aV\xF9V[PP\x81\x80aMvWPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aM\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aM\xE5\x92\x90\x91`\x04\x01ax8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aM\xFFW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aN\x10WP`\x01[a)\xC3Wb\x03\xD0\x90Z\x11\x15\x80aN0WPaN,`\x02\x82a\x85\x03V[Z\x11\x15[\x15aN7W\xFE[a)\xC3V[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a (W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\xB2V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aO\x0BWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a-`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`\x01\x81\x14\x80aOkWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90aJ\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a8\xB0W`\xA2\x80T`\0\x90aO\xDB\x90`\x01`\x01`@\x1B\x03\x16aq\xE4V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aP\\\x83aq\xE4V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a-`WaP\xAD\x81`\x01`\x01`@\x1B\x03\x16aT\xBFV[`@Q` \x01aP\xBD\x91\x90a\x85%V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x12\xB2\x91`\x04\x01ailV[aJ\xCF\x82\x82`\0aQ\xAFV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE7\x91\x90ar\nV[`\0a\x0F\xE7aQnaU^V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aQ\xCB\x86a\x85jV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aR\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15aR.W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aRZ\x90\x84\x90`\x0F\x0Ba\x81:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x81`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b$\xA8)`\xE9\x1B\x81RP\x90aR\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12\xB2\x91\x90ailV[P`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS;\x91\x90ar\nV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a-`Wc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`\xAF` R`@\x90\x81\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x86\x16\x17\x90UQbT\xF2\x9B`\xE6\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x0F\x83\x90\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x15<\xA6\xC0\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xF0W`\0\x80\xFD[`\0Ta\x01\0\x90\x04`\xFF\x16aT1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[a (3aN\x96V[`\0Ta\x01\0\x90\x04`\xFF\x16aT\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12\xB2V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0aT\xCC\x83aU\xDEV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xEBWaT\xEBa\\\xC4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aU\x15W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aU\x1FWP\x93\x92PPPV[`\0aU\xD9\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaU\x8D`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[\x90P\x90V[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aV'Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aVSWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10aVqWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10aV\x89Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10aV\x9DWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10aV\xAFW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0F\xE7W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80TaW\x05\x90ax\x04V[`\0\x82U\x80`\x1F\x10aW\x15WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a8\xB0\x91\x90aY\x8BV[\x82\x80TaW?\x90ax\x04V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aWaW`\0\x85UaW\xA7V[\x82`\x1F\x10aWzW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85UaW\xA7V[\x82\x80\x01`\x01\x01\x85U\x82\x15aW\xA7W\x91\x82\x01[\x82\x81\x11\x15aW\xA7W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aW\x8CV[PaW\xB3\x92\x91PaY\x8BV[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01aW\xE0aX\x14V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01aX\xC4aX\x14V[\x81R` \x01aW\xE0aX\x14V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aV\xECV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01aX\xC4aX\x14V[`@Q\x80`\xA0\x01`@R\x80aYjaY\xA0V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15aW\xB3W`\0\x81U`\x01\x01aY\x8CV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aY\xE2W`\0\x80\xFD[a\x1F}\x83\x83aY\xBEV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0F\xE7V[`\0`\x80\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aZ:W`\0\x80\xFD[a\x1F}\x83\x83aZ\x16V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x0F\xE7V[`\0`\xA0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ\xABW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xC1W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aZ\x87V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15aZ\xF0W\x81\x81\x01Q\x83\x82\x01R` \x01aZ\xD8V[\x83\x81\x11\x15a)\xC3WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra[\x19\x81` \x86\x01` \x86\x01aZ\xD5V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a[q\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01RaZ\xCD`\xA0\x85\x01\x82a[\x01V[` \x81R`\0a\x1F}` \x83\x01\x84a[-V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x0F\xE7V[`\0`@\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\\\0W`\0\x80\xFD[a\x1F}\x83\x83a[\xDCV[`\0\x80\x83`\x1F\x84\x01\x12a\\\x1CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\\3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\\NW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\\hW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\\~W`\0\x80\xFD[a\\\x8A\x85\x82\x86\x01a\\\nV[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8\xB0W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\\\xFCWa\\\xFCa\\\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a]\xD4Wa]\xD4a\\\xC4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a]\xF5Wa]\xF5a\\\xC4V[a^\x08`\x1F\x84\x01`\x1F\x19\x16` \x01a]\xACV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a^\x1CW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a^IW`\0\x80\xFD[\x845\x93P` \x85\x015a^[\x81a\\\x96V[\x92Pa^i`@\x86\x01a\\\xA8V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a^\x84W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a^\x95W`\0\x80\xFD[a^\xA4\x87\x825` \x84\x01a]\xDCV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a^\xC2W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a^\xDBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a^\xF1W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aY\xBEV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a_\x11V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q``` \x84\x01Ra_W`\x80\x84\x01\x82a^\xFDV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra_u\x83\x83a^\xFDV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa_\x93\x82\x82a^\xFDV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a_\xAEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xC4W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a[\xDCV[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01RaZ\xCD``\x84\x01\x82a[\x01V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a8\xB0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a`%W`\0\x80\xFD[\x815a\x1F}\x81a_\xFEV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a`]W`\0\x80\xFD[a`f\x85a`0V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a`\x81W`\0\x80\xFD[a`\x8D\x87\x82\x88\x01a\\\nV[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a`\xB2W`\0\x80\xFD[a`\xBAa\\\xDAV[a`\xC3\x83a`0V[\x81Ra`\xD1` \x84\x01a`0V[` \x82\x01Ra`\xE2`@\x84\x01a`0V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aa\0W`\0\x80\xFD[\x815a\x1F}\x81a\\\x96V[`\0` \x82\x84\x03\x12\x15aa\x1DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aa3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aZ\x16V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01RaZ\xCD`\xC0\x85\x01\x82a[\x01V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01RaZ\xCD`\x80\x85\x01\x82aa?V[` \x81R`\0a\x1F}` \x83\x01\x84aa\x9EV[`\0` \x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ab\x14W`\0\x80\xFD[a\x1F}\x83\x83aa\xF0V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ab2V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a_0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01abhV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rab\xB2`\xE0\x85\x01\x82ab\x1EV[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rab\xCF\x82\x82abTV[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90a_\x93\x81\x83a[\x01V[` \x81R`\0\x82Q`@` \x84\x01Rac\x1E``\x84\x01\x82abTV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra_\x93\x82\x82a^\xFDV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15acTWacTa\\\xC4V[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a8\xB0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ac~W`\0\x80\xFD[\x815` ac\x93ac\x8E\x83ac;V[a]\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ac\xB2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805ac\xC9\x81ac^V[\x83R\x91\x83\x01\x91\x83\x01ac\xB6V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15ac\xFAW`\0\x80\xFD[\x865ad\x05\x81a_\xFEV[\x95P` \x87\x015ad\x15\x81a_\xFEV[\x94P`@\x87\x015ad%\x81a_\xFEV[\x93P``\x87\x015ad5\x81a_\xFEV[\x92P`\x80\x87\x015adE\x81a_\xFEV[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ad`W`\0\x80\xFD[adl\x89\x82\x8A\x01acmV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01RaZ\xCD`@\x85\x01\x82a^\xFDV[` \x81R`\0a\x1F}` \x83\x01\x84adyV[`\0`\xC0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ad\xD8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xEEW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ad\xB4V[` \x81R`\0a\x1F}` \x83\x01\x84aa?V[\x805\x80\x15\x15\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ae0W`\0\x80\xFD[\x825ae;\x81a_\xFEV[\x91PaeI` \x84\x01ae\rV[\x90P\x92P\x92\x90PV[` \x81Rae\x97` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01RaZ\xCD`\xC0\x84\x01\x82a[\x01V[\x805`\xFF\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15ae\xDBW`\0\x80\xFD[ae\xE4\x87a`0V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xFFW`\0\x80\xFD[af\x0B\x89\x82\x8A\x01a\\\nV[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Paf+`\x80\x88\x01ae\xB1V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Raff` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01RaZ\xCD`\xA0\x84\x01\x82a[\x01V[`\0\x80\x83`\x1F\x84\x01\x12af\x92W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\xA9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\\NW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15af\xD6W`\0\x80\xFD[\x835af\xE1\x81a_\xFEV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15af\xFCW`\0\x80\xFD[ag\x08\x86\x82\x87\x01af\x80V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15ag(W`\0\x80\xFD[\x825ag3\x81a\\\x96V[\x91P` \x83\x015agC\x81ac^V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15agaW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15agwW`\0\x80\xFD[a\\\x8A\x85\x82\x86\x01af\x80V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rag\xA8``\x85\x01\x82aa?V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra_\x93\x82\x82aa?V[` \x81R`\0a\x1F}` \x83\x01\x84ag\x83V[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15ah\x01W`\0\x80\xFD[ah\n\x84ag\xD4V[\x92P` \x84\x015ah\x1A\x81a\\\x96V[\x91Pah(`@\x85\x01a\\\xA8V[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ahCW`\0\x80\xFD[a\x1F}\x83\x83ad\xB4V[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x18\x1CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xEAW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ah\xB0V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01RaZ\xCDa\x01\0\x84\x01\x82a[\x01V[` \x81R`\0a\x1F}` \x83\x01\x84a[\x01V[`\0`\xE0\x82\x84\x03\x12\x15ai\x91W`\0\x80\xFD[a\x1F}\x83\x83ah\xB0V[`\0\x82`\x1F\x83\x01\x12ai\xACW`\0\x80\xFD[a\x1F}\x83\x835` \x85\x01a]\xDCV[`\0\x80`@\x83\x85\x03\x12\x15ai\xCEW`\0\x80\xFD[ai\xD7\x83a`0V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ai\xF3W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15aj\x07W`\0\x80\xFD[aj\x0Fa\\\xDAV[aj\x18\x83a`0V[\x81R` \x83\x015aj(\x81a_\xFEV[` \x82\x01R`@\x83\x015\x82\x81\x11\x15aj?W`\0\x80\xFD[ajK\x88\x82\x86\x01ai\x9BV[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Raj\xE4`\xC0\x85\x01\x82ab\x1EV[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Ra_\x93\x81\x83a[\x01V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Rak\x8E``\x85\x01\x82abTV[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15ac\xD6W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90ak\xAFV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Ral\x07`\xA0\x84\x01\x82aa?V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Ra_\x93\x82\x82aa?V[` \x81RaiQ` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15al\xBAW\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01al\x9BV[PPP\x83\x01Q`\xE0`\x80\x84\x01Ral\xD5a\x01\0\x84\x01\x82a[\x01V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Ral\xF3\x83\x83abTV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPam\x11\x82\x82abTV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x0F\xE7\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15am\x98W`\0\x80\xFD[am\xA1\x86ag\xD4V[\x94P` \x86\x015am\xB1\x81a\\\x96V[\x93Pam\xBF`@\x87\x01a\\\xA8V[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am\xDAW`\0\x80\xFD[am\xE6\x88\x82\x89\x01af\x80V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15an\tW`\0\x80\xFD[a\x1F}\x82a`0V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01RanU`\xC0\x85\x01\x82a[\x01V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15an\x80W`\0\x80\xFD[an\x88a\\\xDAV[\x90P\x815\x81R` \x82\x015` \x82\x01Ran\xA4`@\x83\x01a`0V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15an\xC1W`\0\x80\xFD[a\x1F}\x83\x83annV[`\0`\x80\x82\x84\x03\x12\x15an\xDDW`\0\x80\xFD[an\xE5a]\x02V[\x90P\x815\x81R` \x82\x015an\xF9\x81a\\\x96V[` \x82\x01Rao\n`@\x83\x01a\\\xA8V[`@\x82\x01Rao\x1B``\x83\x01a`0V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15ao8W`\0\x80\xFD[a\x1F}\x83\x83an\xCBV[`\0`\x80\x82\x84\x03\x12\x15aoTW`\0\x80\xFD[ao\\a]\x02V[\x825\x81R` \x83\x015aon\x81a\\\x96V[` \x82\x01Rao\x7F`@\x84\x01a\\\xA8V[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15ao\xAAW`\0\x80\xFD[ao\xB2a]$V[\x90Pao\xBE\x83\x83an\xCBV[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[ao\xE5\x84\x82\x85\x01ai\x9BV[` \x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83ao\x98V[`\0`\x80\x82\x84\x03\x12\x15ap\x0FW`\0\x80\xFD[ap\x17a]\x02V[\x90P\x815\x81R` \x82\x015` \x82\x01Rao\n`@\x83\x01a\\\xA8V[`\0`\x80\x82\x84\x03\x12\x15apEW`\0\x80\xFD[a\x1F}\x83\x83ao\xFDV[`\0`\x80\x82\x84\x03\x12\x15apaW`\0\x80\xFD[apia]\x02V[\x825\x81R` \x83\x015ap{\x81a\\\x96V[` \x82\x01R`@\x83\x015ap\x8E\x81ac^V[`@\x82\x01R``\x83\x015ap\xA1\x81ac^V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ap\xBFW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15ap\xE1Wap\xE1a\\\xC4V[`@R\x825ap\xEF\x81a\\\x96V[\x81R` \x83\x015ap\xFF\x81ac^V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aq8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aqRW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\\NW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aq\x8FWaq\x8FaqgV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aq\xB8Waq\xB8aqgV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80aq\xDAWaq\xDAaqgV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03ar\0War\0aqgV[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ar\x1CW`\0\x80\xFD[\x81Qa\x1F}\x81a_\xFEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qar^\x81`\x01\x85\x01` \x87\x01aZ\xD5V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15ar~W`\0\x80\xFD[ar\x86a\\\xDAV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ar\x9FW`\0\x80\xFD[ar\xAB\x85\x83\x86\x01acmV[\x83R` \x84\x015\x91P\x80\x82\x11\x15ar\xC1W`\0\x80\xFD[ar\xCD\x85\x83\x86\x01acmV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15ar\xE6W`\0\x80\xFD[Par\xF3\x84\x82\x85\x01acmV[`@\x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83arlV[`\0`@\x82\x84\x03\x12\x15as\x1DW`\0\x80\xFD[as%a]$V[\x90P\x815as2\x81a_\xFEV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0a\x0F\xE76\x83as\x0BV[`\0\x82\x82\x10\x15askWaskaqgV[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14a\\\xBFW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15as\x94W`\0\x80\xFD[as\x9Ca]\x02V[\x825as\xA7\x81a_\xFEV[\x81R` \x83\x015as\xB7\x81a\\\x96V[` \x82\x01Ras\xC8`@\x84\x01aspV[`@\x82\x01Rap\xA1``\x84\x01aspV[`\0\x81\x83\x03`\xC0\x81\x12\x15as\xECW`\0\x80\xFD[as\xF4a]$V[\x91P`\xA0\x81\x12\x15at\x04W`\0\x80\xFD[Pat\ra]FV[\x825\x81R` \x83\x015at\x1F\x81ac^V[` \x82\x01R`@\x83\x015at2\x81ac^V[`@\x82\x01RatC``\x84\x01a`0V[``\x82\x01RatT`\x80\x84\x01a`0V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15at\x86W`\0\x80\xFD[at\x8Ea]\x02V[\x90P\x815at\x9B\x81a\\\x96V[\x81R` \x82\x015at\xAB\x81ac^V[` \x82\x01R`@\x82\x015at\xBE\x81ac^V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15at\xDCW`\0\x80\xFD[at\xE8\x84\x82\x85\x01as\xD9V[``\x83\x01RP\x92\x91PPV[`\0a\x0F\xE76\x83attV[`\0` \x82\x84\x03\x12\x15au\x12W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15au4Wau4a\\\xC4V[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15auSW`\0\x80\xFD[au[a\\\xDAV[\x825\x81R` \x83\x015aum\x81a\\\x96V[` \x82\x01Ra`\xE2`@\x84\x01a\\\xA8V[`\0\x82`\x1F\x83\x01\x12au\x8FW`\0\x80\xFD[\x815` au\x9Fac\x8E\x83ac;V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15au\xBEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805au\xD5\x81a\\\x96V[\x83R\x91\x83\x01\x91\x83\x01au\xC2V[`\0\x82`\x1F\x83\x01\x12au\xF3W`\0\x80\xFD[\x815` av\x03ac\x8E\x83ac;V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ac\xD6W\x805\x83R\x91\x83\x01\x91\x83\x01av&V[`\0`@\x826\x03\x12\x15avOW`\0\x80\xFD[avWa]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15avnW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15av\x83W`\0\x80\xFD[av\x8Ba]\x02V[\x825\x81R` \x83\x015\x82\x81\x11\x15av\xA1W`\0\x80\xFD[av\xAD6\x82\x86\x01au~V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15av\xC5W`\0\x80\xFD[av\xD16\x82\x86\x01au\xE2V[`@\x83\x01RPav\xE3``\x84\x01a`0V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15av\xFEW`\0\x80\xFD[Pao\xE56\x82\x86\x01ai\x9BV[`\0`@\x826\x03\x12\x15aw\x1DW`\0\x80\xFD[aw%a]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aw<W`\0\x80\xFD[awH6\x83\x87\x01au\xE2V[\x83R` \x85\x015\x91P\x80\x82\x11\x15aw^W`\0\x80\xFD[Pao\xE56\x82\x86\x01acmV[` \x81\x01`\x02\x83\x10aw\x8DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03ar\0War\0aqgV[`\0`@\x82\x84\x03\x12\x15aw\xBEW`\0\x80\xFD[aw\xC6a]$V[\x90Paw\xD1\x82a\\\xA8V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw\xECW`\0\x80\xFD[ao\xE5\x84\x82\x85\x01acmV[`\0a\x0F\xE76\x83aw\xACV[`\x01\x81\x81\x1C\x90\x82\x16\x80ax\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x18\x1CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a[\x01V[`\0` \x82\x84\x03\x12\x15axlW`\0\x80\xFD[PQ\x91\x90PV[`\0a\x0F\xE76\x83as\xD9V[`\0`\xA0\x82\x84\x03\x12\x15ax\x91W`\0\x80\xFD[ax\x99a]$V[\x90Pao\xBE\x83\x83ao\xFDV[`\0a\x0F\xE76\x83ax\x7FV[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15ax\xDDW`\0\x80\xFD[ax\xE5a]$V[\x90Pax\xF1\x83\x83annV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0a\x0F\xE76\x83ax\xCBV[`\0\x80\x85\x85\x11\x15ay(W`\0\x80\xFD[\x83\x86\x11\x15ay5W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x84\x03\x12\x15ayTW`\0\x80\xFD[ay\\a]hV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015ayz\x81a\\\x96V[`@\x82\x01Ray\x8B``\x83\x01ae\rV[``\x82\x01R`\x80\x82\x015ay\x9E\x81ac^V[`\x80\x82\x01Ray\xAF`\xA0\x83\x01a`0V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15ay\xCCW`\0\x80\xFD[a\x1F}\x83\x83ayBV[`\0`@\x82\x84\x03\x12\x15ay\xE8W`\0\x80\xFD[ay\xF0a]$V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15az\tW`\0\x80\xFD[az\x15\x85\x83\x86\x01au\xE2V[\x83R` \x91P\x81\x84\x015\x81\x81\x11\x15az,W`\0\x80\xFD[\x84\x01\x90P`\x1F\x81\x01\x85\x13az?W`\0\x80\xFD[\x805azMac\x8E\x82ac;V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15azlW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15az\x8AW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90azqV[\x80\x85\x87\x01RPPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15az\xACW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15az\xC2W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ay\xD6V[`\0` \x82\x84\x03\x12\x15az\xE0W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a{\x02Wa{\x02a\\\xC4V[`@R\x90P\x80a{\x11\x83a\\\xA8V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a{+W`\0\x80\xFD[a\x1F}\x83\x83az\xCEV[`\0`\xC0\x82\x84\x03\x12\x15a{GW`\0\x80\xFD[a{Oa]hV[\x90P\x815\x81R` \x82\x015a{c\x81a\\\x96V[` \x82\x01Ra{t`@\x83\x01a\\\xA8V[`@\x82\x01Ra{\x85``\x83\x01a\\\xA8V[``\x82\x01Ray\x9E`\x80\x83\x01a\\\xA8V[`\0`\xC0\x82\x84\x03\x12\x15a{\xA8W`\0\x80\xFD[a\x1F}\x83\x83a{5V[`\0` \x82\x84\x03\x12\x15a{\xC4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a{\xDAW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01as\x0BV[`\0` \x82\x84\x03\x12\x15a{\xF8W`\0\x80\xFD[\x81Qa\x1F}\x81a\\\x96V[`\0`\xE0\x82\x84\x03\x12\x15a|\x15W`\0\x80\xFD[a|\x1Da]$V[\x90Pa|)\x83\x83ayBV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a|VW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a|lW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a|\x03V[`\0``\x82\x84\x03\x12\x15a|\x8AW`\0\x80\xFD[a|\x92a\\\xDAV[\x90P\x815a|\x9F\x81a\\\x96V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a|\xBBW`\0\x80\xFD[a|\xC7\x85\x83\x86\x01as\xD9V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a|\xE0W`\0\x80\xFD[Par\xF3\x84\x82\x85\x01as\xD9V[`\0a\x0F\xE76\x83a|xV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x18\x1CW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x0F\xE76\x83a|\x03V[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\\\xBFW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a}cW`\0\x80\xFD[a}ka]\x8AV[\x825\x81R` \x83\x015a}}\x81a\\\x96V[` \x82\x01R`@\x83\x015a}\x90\x81a\\\x96V[`@\x82\x01Ra}\xA1``\x84\x01a})V[``\x82\x01Ra}\xB2`\x80\x84\x01a})V[`\x80\x82\x01Ra}\xC3`\xA0\x84\x01a`0V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a}\xDCW`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a}\xFAW`\0\x80\xFD[a~\x02a\\\xDAV[\x825a~\r\x81a\\\x96V[\x81Ra~\x1B` \x84\x01a\\\xA8V[` \x82\x01R`@\x83\x015a`\xE2\x81a_\xFEV[`\0`@\x826\x03\x12\x15a~@W`\0\x80\xFD[a~Ha]$V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a~_W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a~tW`\0\x80\xFD[a~|a\\\xDAV[\x825\x81R` \x83\x015\x82\x81\x11\x15a~\x92W`\0\x80\xFD[a~\x9E6\x82\x86\x01au~V[` \x83\x01RPa~\xB0`@\x84\x01a`0V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15av\xFEW`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a~\xDDW`\0\x80\xFD[a~\xE5a]hV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x7F\x01`@\x83\x01ae\xB1V[`@\x82\x01R``\x82\x015ay\x8B\x81a\\\x96V[`\0`\xE0\x826\x03\x12\x15a\x7F&W`\0\x80\xFD[a\x7F.a]$V[a\x7F86\x84a~\xCBV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x7FSW`\0\x80\xFD[ao\xE56\x82\x86\x01ai\x9BV[`\0a\x0F\xE76\x83ay\xD6V[`\0`\x80\x826\x03\x12\x15a\x7F}W`\0\x80\xFD[a\x7F\x85a]\x02V[\x825a\x7F\x90\x81a\\\x96V[\x81Ra\x7F\x9E` \x84\x01ae\rV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x7F\xBDW`\0\x80\xFD[a\x7F\xC96\x83\x87\x01as\xD9V[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x7F\xE2W`\0\x80\xFD[Pat\xE86\x82\x86\x01as\xD9V[`\0`\xE0\x82\x84\x03\x12\x15a\x80\x01W`\0\x80\xFD[a\x80\ta]$V[\x90Pa|)\x83\x83a{5V[`\0a\x0F\xE76\x83a\x7F\xEFV[`\0\x82`\x1F\x83\x01\x12a\x802W`\0\x80\xFD[a\x80:a\\\xDAV[\x80``\x84\x01\x85\x81\x11\x15a\x80LW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x80fW\x805\x84R` \x93\x84\x01\x93\x01a\x80NV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x80\x83W`\0\x80\xFD[a\x80\x8Ba]FV[a\x80\x956\x84a\x80!V[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x80\xB1W`\0\x80\xFD[a\x80\xBD6\x83\x87\x01ai\x9BV[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x80\xD6W`\0\x80\xFD[a\x80\xE26\x83\x87\x01au\xE2V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x80\xFBW`\0\x80\xFD[Pa\x81\x086\x82\x86\x01au\xE2V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x810W`\0\x80\xFD[a\x1F}\x83\x83a~\xCBV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x81mWa\x81maqgV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x81\x92Wa\x81\x92aqgV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x81\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x81\xC3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ao\x98V[`\0` \x82\x84\x03\x12\x15a\x81\xE1W`\0\x80\xFD[\x81Qa\x1F}\x81ac^V[`\0` \x82\x84\x03\x12\x15a\x81\xFEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x82\x14W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01aw\xACV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x82@Wa\x82@aqgV[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x82|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x82\x92W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a|xV[` \x81R`\0\x82Q``` \x84\x01Ra\x82\xBA`\x80\x84\x01\x82ag\x83V[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x82\xFBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x11W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01attV[`@\x81R`\0a\x830`@\x83\x01\x85aa\x9EV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x83ZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83pW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01a\x7F\xEFV[`\0` \x80\x83\x85\x03\x12\x15a\x83\x8FW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\xA5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x83\xB6W`\0\x80\xFD[\x80Qa\x83\xC4ac\x8E\x82ac;V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x83\xE3W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x84\nW\x83Qa\x83\xFB\x81a\\\x96V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x83\xE8V[\x97\x96PPPPPPPV[\x82Q\x81R`@` \x82\x01R`\0aZ\xCD`@\x83\x01\x84a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x84AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84WW`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01arlV[` \x81R`\0a\x1F}` \x83\x01\x84a^\xFDV[`@\x81R`\0a\x84\x89`@\x83\x01\x85a^\xFDV[\x82\x81\x03` \x84\x01Ra_\x93\x81\x85a^\xFDV[`\0` \x82\x84\x03\x12\x15a\x84\xADW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xC3W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ax\xCBV[`\0` \x82\x84\x03\x12\x15a\x84\xE1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xF7W`\0\x80\xFD[aZ\xCD\x84\x82\x85\x01ax\x7FV[`\0\x82a\x85 WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x85]\x81`\x19\x85\x01` \x87\x01aZ\xD5V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x85\x90Wa\x85\x90aqgV[`\0\x03\x92\x91PPV\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)BurnLp(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)no slow mode transactions remainingMintLp(bytes32 sender,uint32 productId,uint128 amountBase,uint128 quoteAmountLow,uint128 quoteAmountHigh,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x88g\xF2\xB0\x8FLnd}n\xBE\x88\x14\x13\xDF\x9A\x12\xF0\x0Bg\xBB\xE5\xDB\xE8\xEEp+\xBD\xDFq}\x9FdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static ENDPOINT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Endpoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Endpoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Endpoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Endpoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Endpoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Endpoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Endpoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ENDPOINT_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ENDPOINT_ABI.clone(),
                ENDPOINT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `burnLpAndTransfer` (0x0748a219) function
        pub fn burn_lp_and_transfer(
            &self,
            p: BurnLpAndTransfer,
        ) -> ::ethers::contract::builders::ContractCall<M, BurnLpAndTransfer> {
            self.0
                .method_hash([7, 72, 162, 25], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainlinkFullReport` (0xdb5a5021) function
        pub fn chainlink_full_report(
            &self,
            p: ChainlinkFullReport,
        ) -> ::ethers::contract::builders::ContractCall<M, ChainlinkFullReport> {
            self.0
                .method_hash([219, 90, 80, 33], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainlinkReportBlob` (0x96c47c6f) function
        pub fn chainlink_report_blob(
            &self,
            p: ChainlinkReportBlob,
        ) -> ::ethers::contract::builders::ContractCall<M, ChainlinkReportBlob> {
            self.0
                .method_hash([150, 196, 124, 111], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkLpAction` (0x8c3d2f74) function
        pub fn check_lp_action(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([140, 61, 47, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkLpActionInner` (0xc345530b) function
        pub fn check_lp_action_inner(
            &self,
            p0: ::ethers::core::types::Address,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([195, 69, 83, 11], (p0, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSlowModeTxInner` (0xb70eb263) function
        pub fn check_slow_mode_tx_inner(
            &self,
            sender: ::ethers::core::types::Address,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([183, 14, 178, 99], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSlowModeTxLinkSigner` (0x5bb4c126) function
        pub fn check_slow_mode_tx_link_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 180, 193, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimSequencerFees` (0x3842e75e) function
        pub fn claim_sequencer_fees(
            &self,
            p: ClaimSequencerFees,
        ) -> ::ethers::contract::builders::ContractCall<M, ClaimSequencerFees> {
            self.0
                .method_hash([56, 66, 231, 94], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearinghouse` (0x5d4f5f97) function
        pub fn clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 79, 95, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateral` (0x8e5d588c) function
        pub fn deposit_collateral(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 93, 88, 140], (subaccount_name, product_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateralWithReferral` (0x221f0939) function
        pub fn deposit_collateral_with_referral(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            amount: u128,
            referral_code: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 31, 9, 57],
                    (subaccount, product_id, amount, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateralWithReferral` (0xe9bc7462) function
        pub fn deposit_collateral_with_referral_with_subaccount_name_and_product_id_and_amount(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
            referral_code: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 188, 116, 98],
                    (subaccount_name, product_id, amount, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSlowModeTransaction` (0x65dd1366) function
        pub fn execute_slow_mode_transaction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 221, 19, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthCheckFee` (0xd4de8d9d) function
        pub fn get_health_check_fee(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([212, 222, 141, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLinkedSigner` (0x91c1e3d7) function
        pub fn get_linked_signer(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([145, 193, 227, 215], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationFee` (0xfbf41984) function
        pub fn get_liquidation_fee(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([251, 244, 25, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumSubaccounts` (0xc4f9b25f) function
        pub fn get_num_subaccounts(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([196, 249, 178, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOffchainExchange` (0x8f4f8ecc) function
        pub fn get_offchain_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([143, 79, 142, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceX18` (0x368e4686) function
        pub fn get_price_x18(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([54, 142, 70, 134], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencer` (0x4d96a90a) function
        pub fn get_sequencer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([77, 150, 169, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlowModeTx` (0xee525526) function
        pub fn get_slow_mode_tx(
            &self,
            idx: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (SlowModeTx, u64, u64)> {
            self.0
                .method_hash([238, 82, 85, 38], idx)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountById` (0xef64ed0e) function
        pub fn get_subaccount_by_id(
            &self,
            subaccount_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([239, 100, 237, 14], subaccount_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountId` (0x22d4a82d) function
        pub fn get_subaccount_id(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([34, 212, 168, 45], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTakerSequencerFee` (0xc510359f) function
        pub fn get_taker_sequencer_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([197, 16, 53, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTime` (0x557ed1ba) function
        pub fn get_time(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([85, 126, 209, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimes` (0xe9ab77e5) function
        pub fn get_times(&self) -> ::ethers::contract::builders::ContractCall<M, Times> {
            self.0
                .method_hash([233, 171, 119, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementSubmissions` (0x22006046) function
        pub fn increment_submissions(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 0, 96, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x5444569d) function
        pub fn initialize(
            &self,
            sanctions: ::ethers::core::types::Address,
            sequencer: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            clearinghouse: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
            initial_prices: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [84, 68, 86, 157],
                    (
                        sanctions,
                        sequencer,
                        offchain_exchange,
                        clearinghouse,
                        verifier,
                        initial_prices,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `legacyMatchOrders` (0xb36488b8) function
        pub fn legacy_match_orders(
            &self,
            p: LegacyMatchOrders,
        ) -> ::ethers::contract::builders::ContractCall<M, LegacyMatchOrders> {
            self.0
                .method_hash([179, 100, 136, 184], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `legacySignedLiquidateSubaccount` (0xb1fbd60b) function
        pub fn legacy_signed_liquidate_subaccount(
            &self,
            p: LegacySignedLiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, LegacySignedLiquidateSubaccount>
        {
            self.0
                .method_hash([177, 251, 214, 11], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `legacySpotTick` (0xf80f7ce5) function
        pub fn legacy_spot_tick(
            &self,
            p: LegacySpotTick,
        ) -> ::ethers::contract::builders::ContractCall<M, LegacySpotTick> {
            self.0
                .method_hash([248, 15, 124, 229], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `legacyUnsignedLiquidateSubaccount` (0xdc42e61b) function
        pub fn legacy_unsigned_liquidate_subaccount(
            &self,
            p: LegacyLiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, LegacyLiquidateSubaccount> {
            self.0
                .method_hash([220, 66, 230, 27], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationStart` (0x8d0acc9b) function
        pub fn liquidation_start(
            &self,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 10, 204, 155], transaction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0x2c8c6ffb) function
        pub fn manual_assert(
            &self,
            p: ManualAssert,
        ) -> ::ethers::contract::builders::ContractCall<M, ManualAssert> {
            self.0
                .method_hash([44, 140, 111, 251], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrderAMM` (0x36b90c51) function
        pub fn match_order_amm(
            &self,
            p: MatchOrderAMM,
        ) -> ::ethers::contract::builders::ContractCall<M, MatchOrderAMM> {
            self.0
                .method_hash([54, 185, 12, 81], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrders` (0x8d3c20b1) function
        pub fn match_orders(
            &self,
            p: MatchOrders,
        ) -> ::ethers::contract::builders::ContractCall<M, MatchOrders> {
            self.0
                .method_hash([141, 60, 32, 177], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nSubmissions` (0x18ed16eb) function
        pub fn n_submissions(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([24, 237, 22, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `perpTick` (0x5a00923b) function
        pub fn perp_tick(
            &self,
            p: PerpTick,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpTick> {
            self.0
                .method_hash([90, 0, 146, 59], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processSlowModeTransaction` (0x87324338) function
        pub fn process_slow_mode_transaction(
            &self,
            sender: ::ethers::core::types::Address,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 50, 67, 56], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceXWithdraw` (0x9a08e535) function
        pub fn rebalance_x_withdraw(
            &self,
            p: RebalanceXWithdraw,
        ) -> ::ethers::contract::builders::ContractCall<M, RebalanceXWithdraw> {
            self.0
                .method_hash([154, 8, 229, 53], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebate` (0x42c74d1d) function
        pub fn rebate(&self, p: Rebate) -> ::ethers::contract::builders::ContractCall<M, Rebate> {
            self.0
                .method_hash([66, 199, 77, 29], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `referralCodes` (0x9534dd3e) function
        pub fn referral_codes(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 52, 221, 62], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerTransferableWallet` (0x6cfe5fe4) function
        pub fn register_transferable_wallet(
            &self,
            wallet: ::ethers::core::types::Address,
            transferable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 254, 95, 228], (wallet, transferable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resyncSlowModeTxs` (0x21047589) function
        pub fn resync_slow_mode_txs(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 4, 117, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPriceX18` (0x8c58e10a) function
        pub fn set_price_x18(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 88, 225, 10], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSlowModeConfig` (0x3216c062) function
        pub fn set_slow_mode_config(
            &self,
            slow_mode_config: SlowModeConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 22, 192, 98], (slow_mode_config,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSlowModeTx` (0x98cd32fe) function
        pub fn set_slow_mode_tx(
            &self,
            idx: u64,
            txn: SlowModeTx,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 205, 50, 254], (idx, txn))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settlePnl` (0xb2bb6367) function
        pub fn settle_pnl(
            &self,
            p: SettlePnl,
        ) -> ::ethers::contract::builders::ContractCall<M, SettlePnl> {
            self.0
                .method_hash([178, 187, 99, 103], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedBurnLp` (0x610b2e5e) function
        pub fn signed_burn_lp(
            &self,
            p: SignedBurnLp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedBurnLp> {
            self.0
                .method_hash([97, 11, 46, 94], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedCancellation` (0x3edf2c5b) function
        pub fn signed_cancellation(
            &self,
            p: SignedCancellation,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedCancellation> {
            self.0
                .method_hash([62, 223, 44, 91], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedCancellationProducts` (0xa082c5aa) function
        pub fn signed_cancellation_products(
            &self,
            p: SignedCancellationProducts,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedCancellationProducts> {
            self.0
                .method_hash([160, 130, 197, 170], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedLinkSigner` (0x85c83e9d) function
        pub fn signed_link_signer(
            &self,
            p: SignedLinkSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedLinkSigner> {
            self.0
                .method_hash([133, 200, 62, 157], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedLiquidateSubaccount` (0x9171d08b) function
        pub fn signed_liquidate_subaccount(
            &self,
            p: SignedLiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedLiquidateSubaccount> {
            self.0
                .method_hash([145, 113, 208, 139], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMintLp` (0xd38c3b9c) function
        pub fn signed_mint_lp(
            &self,
            p: SignedMintLp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedMintLp> {
            self.0
                .method_hash([211, 140, 59, 156], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedOrder` (0x6c457570) function
        pub fn signed_order(
            &self,
            p: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedOrder> {
            self.0
                .method_hash([108, 69, 117, 112], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedTransferQuote` (0x6f3b0a72) function
        pub fn signed_transfer_quote(
            &self,
            p: SignedTransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedTransferQuote> {
            self.0
                .method_hash([111, 59, 10, 114], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedWithdrawCollateral` (0x0d55e26b) function
        pub fn signed_withdraw_collateral(
            &self,
            p: SignedWithdrawCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedWithdrawCollateral> {
            self.0
                .method_hash([13, 85, 226, 107], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spotTick` (0xb1c8ec2b) function
        pub fn spot_tick(
            &self,
            p: SpotTick,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotTick> {
            self.0
                .method_hash([177, 200, 236, 43], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSlowModeTransaction` (0xe604ed9e) function
        pub fn submit_slow_mode_transaction(
            &self,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 4, 237, 158], transaction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactions` (0x1f186b27) function
        pub fn submit_transactions(
            &self,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 24, 107, 39], transactions)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactionsChecked` (0x7db6a25b) function
        pub fn submit_transactions_checked(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
            e: [u8; 32],
            s: [u8; 32],
            signer_bitmask: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [125, 182, 162, 91],
                    (idx, transactions, e, s, signer_bitmask),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactionsCheckedWithGasLimit` (0x2f9a2744) function
        pub fn submit_transactions_checked_with_gas_limit(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
            gas_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 154, 39, 68], (idx, transactions, gas_limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapAMM` (0x0f4b509d) function
        pub fn swap_amm(
            &self,
            p: SwapAMM,
        ) -> ::ethers::contract::builders::ContractCall<M, SwapAMM> {
            self.0
                .method_hash([15, 75, 80, 157], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferQuote` (0x1d97d22f) function
        pub fn transfer_quote(
            &self,
            p: TransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, TransferQuote> {
            self.0
                .method_hash([29, 151, 210, 47], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedBurnLp` (0x06c0bafd) function
        pub fn unsigned_burn_lp(
            &self,
            p: BurnLp,
        ) -> ::ethers::contract::builders::ContractCall<M, BurnLp> {
            self.0
                .method_hash([6, 192, 186, 253], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDepositCollateral` (0x3cec4b93) function
        pub fn unsigned_deposit_collateral(
            &self,
            p: DepositCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositCollateral> {
            self.0
                .method_hash([60, 236, 75, 147], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDepositInsurance` (0x94faefe5) function
        pub fn unsigned_deposit_insurance(
            &self,
            p: DepositInsurance,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositInsurance> {
            self.0
                .method_hash([148, 250, 239, 229], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedLinkSigner` (0x05e42dc7) function
        pub fn unsigned_link_signer(
            &self,
            p: LinkSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, LinkSigner> {
            self.0
                .method_hash([5, 228, 45, 199], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedLiquidateSubaccount` (0x9e851424) function
        pub fn unsigned_liquidate_subaccount(
            &self,
            p: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, LiquidateSubaccount> {
            self.0
                .method_hash([158, 133, 20, 36], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedMintLp` (0x910e606a) function
        pub fn unsigned_mint_lp(
            &self,
            p: MintLp,
        ) -> ::ethers::contract::builders::ContractCall<M, MintLp> {
            self.0
                .method_hash([145, 14, 96, 106], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedTransferQuote` (0x0edaacce) function
        pub fn unsigned_transfer_quote(
            &self,
            p: TransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, TransferQuote> {
            self.0
                .method_hash([14, 218, 172, 206], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedWithdrawCollateral` (0xbc85ca86) function
        pub fn unsigned_withdraw_collateral(
            &self,
            p: WithdrawCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, WithdrawCollateral> {
            self.0
                .method_hash([188, 133, 202, 134], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeRates` (0x35639a4f) function
        pub fn update_fee_rates(
            &self,
            p: UpdateFeeRates,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateFeeRates> {
            self.0
                .method_hash([53, 99, 154, 79], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePrice` (0x1d9eeda5) function
        pub fn update_price(
            &self,
            p: UpdatePrice,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdatePrice> {
            self.0
                .method_hash([29, 158, 237, 165], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateProduct` (0x2cd71b16) function
        pub fn update_product(
            &self,
            p: UpdateProduct,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateProduct> {
            self.0
                .method_hash([44, 215, 27, 22], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSanctions` (0xf2633927) function
        pub fn update_sanctions(
            &self,
            sanctions: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 99, 57, 39], sanctions)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SubmitTransactions` event
        pub fn submit_transactions_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SubmitTransactionsFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EndpointEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Endpoint<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SubmitTransactions", abi = "SubmitTransactions()")]
    pub struct SubmitTransactionsFilter;
    ///Container type for all of the contract's events
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum EndpointEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SubmitTransactionsFilter(SubmitTransactionsFilter),
    }
    impl ::ethers::contract::EthLogDecode for EndpointEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EndpointEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(EndpointEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SubmitTransactionsFilter::decode_log(log) {
                return Ok(EndpointEvents::SubmitTransactionsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EndpointEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for EndpointEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for EndpointEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsFilter> for EndpointEvents {
        fn from(value: SubmitTransactionsFilter) -> Self {
            Self::SubmitTransactionsFilter(value)
        }
    }
    ///Container type for all input parameters for the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `0x0748a219`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "burnLpAndTransfer",
        abi = "burnLpAndTransfer((bytes32,uint32,uint128,bytes32))"
    )]
    pub struct BurnLpAndTransferCall {
        pub p: BurnLpAndTransfer,
    }
    ///Container type for all input parameters for the `chainlinkFullReport` function with signature `chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))` and selector `0xdb5a5021`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "chainlinkFullReport",
        abi = "chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))"
    )]
    pub struct ChainlinkFullReportCall {
        pub p: ChainlinkFullReport,
    }
    ///Container type for all input parameters for the `chainlinkReportBlob` function with signature `chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))` and selector `0x96c47c6f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "chainlinkReportBlob",
        abi = "chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))"
    )]
    pub struct ChainlinkReportBlobCall {
        pub p: ChainlinkReportBlob,
    }
    ///Container type for all input parameters for the `checkLpAction` function with signature `checkLpAction()` and selector `0x8c3d2f74`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "checkLpAction", abi = "checkLpAction()")]
    pub struct CheckLpActionCall;
    ///Container type for all input parameters for the `checkLpActionInner` function with signature `checkLpActionInner(address,bytes)` and selector `0xc345530b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "checkLpActionInner", abi = "checkLpActionInner(address,bytes)")]
    pub struct CheckLpActionInnerCall {
        pub p0: ::ethers::core::types::Address,
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `0xb70eb263`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "checkSlowModeTxInner",
        abi = "checkSlowModeTxInner(address,bytes)"
    )]
    pub struct CheckSlowModeTxInnerCall {
        pub sender: ::ethers::core::types::Address,
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `0x5bb4c126`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "checkSlowModeTxLinkSigner",
        abi = "checkSlowModeTxLinkSigner()"
    )]
    pub struct CheckSlowModeTxLinkSignerCall;
    ///Container type for all input parameters for the `claimSequencerFees` function with signature `claimSequencerFees((bytes32))` and selector `0x3842e75e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "claimSequencerFees", abi = "claimSequencerFees((bytes32))")]
    pub struct ClaimSequencerFeesCall {
        pub p: ClaimSequencerFees,
    }
    ///Container type for all input parameters for the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "clearinghouse", abi = "clearinghouse()")]
    pub struct ClearinghouseCall;
    ///Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral(bytes12,uint32,uint128)` and selector `0x8e5d588c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositCollateral",
        abi = "depositCollateral(bytes12,uint32,uint128)"
    )]
    pub struct DepositCollateralCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///Container type for all input parameters for the `depositCollateralWithReferral` function with signature `depositCollateralWithReferral(bytes32,uint32,uint128,string)` and selector `0x221f0939`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositCollateralWithReferral",
        abi = "depositCollateralWithReferral(bytes32,uint32,uint128,string)"
    )]
    pub struct DepositCollateralWithReferralCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub referral_code: ::std::string::String,
    }
    ///Container type for all input parameters for the `depositCollateralWithReferral` function with signature `depositCollateralWithReferral(bytes12,uint32,uint128,string)` and selector `0xe9bc7462`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositCollateralWithReferral",
        abi = "depositCollateralWithReferral(bytes12,uint32,uint128,string)"
    )]
    pub struct DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub referral_code: ::std::string::String,
    }
    ///Container type for all input parameters for the `executeSlowModeTransaction` function with signature `executeSlowModeTransaction()` and selector `0x65dd1366`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeSlowModeTransaction",
        abi = "executeSlowModeTransaction()"
    )]
    pub struct ExecuteSlowModeTransactionCall;
    ///Container type for all input parameters for the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `0xd4de8d9d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getHealthCheckFee", abi = "getHealthCheckFee()")]
    pub struct GetHealthCheckFeeCall;
    ///Container type for all input parameters for the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `0x91c1e3d7`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLinkedSigner", abi = "getLinkedSigner(bytes32)")]
    pub struct GetLinkedSignerCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `0xfbf41984`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLiquidationFee", abi = "getLiquidationFee()")]
    pub struct GetLiquidationFeeCall;
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `0xc4f9b25f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNumSubaccounts", abi = "getNumSubaccounts()")]
    pub struct GetNumSubaccountsCall;
    ///Container type for all input parameters for the `getOffchainExchange` function with signature `getOffchainExchange()` and selector `0x8f4f8ecc`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getOffchainExchange", abi = "getOffchainExchange()")]
    pub struct GetOffchainExchangeCall;
    ///Container type for all input parameters for the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `0x368e4686`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPriceX18", abi = "getPriceX18(uint32)")]
    pub struct GetPriceX18Call {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSequencer` function with signature `getSequencer()` and selector `0x4d96a90a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSequencer", abi = "getSequencer()")]
    pub struct GetSequencerCall;
    ///Container type for all input parameters for the `getSlowModeTx` function with signature `getSlowModeTx(uint64)` and selector `0xee525526`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSlowModeTx", abi = "getSlowModeTx(uint64)")]
    pub struct GetSlowModeTxCall {
        pub idx: u64,
    }
    ///Container type for all input parameters for the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `0xef64ed0e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSubaccountById", abi = "getSubaccountById(uint64)")]
    pub struct GetSubaccountByIdCall {
        pub subaccount_id: u64,
    }
    ///Container type for all input parameters for the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `0x22d4a82d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSubaccountId", abi = "getSubaccountId(bytes32)")]
    pub struct GetSubaccountIdCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `0xc510359f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTakerSequencerFee", abi = "getTakerSequencerFee()")]
    pub struct GetTakerSequencerFeeCall;
    ///Container type for all input parameters for the `getTime` function with signature `getTime()` and selector `0x557ed1ba`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTime", abi = "getTime()")]
    pub struct GetTimeCall;
    ///Container type for all input parameters for the `getTimes` function with signature `getTimes()` and selector `0xe9ab77e5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTimes", abi = "getTimes()")]
    pub struct GetTimesCall;
    ///Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getVersion", abi = "getVersion()")]
    pub struct GetVersionCall;
    ///Container type for all input parameters for the `incrementSubmissions` function with signature `incrementSubmissions()` and selector `0x22006046`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "incrementSubmissions", abi = "incrementSubmissions()")]
    pub struct IncrementSubmissionsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address,int128[])` and selector `0x5444569d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,address,int128[])"
    )]
    pub struct InitializeCall {
        pub sanctions: ::ethers::core::types::Address,
        pub sequencer: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub verifier: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub initial_prices: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `legacyMatchOrders` function with signature `legacyMatchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0xb36488b8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "legacyMatchOrders",
        abi = "legacyMatchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))"
    )]
    pub struct LegacyMatchOrdersCall {
        pub p: LegacyMatchOrders,
    }
    ///Container type for all input parameters for the `legacySignedLiquidateSubaccount` function with signature `legacySignedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))` and selector `0xb1fbd60b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "legacySignedLiquidateSubaccount",
        abi = "legacySignedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))"
    )]
    pub struct LegacySignedLiquidateSubaccountCall {
        pub p: LegacySignedLiquidateSubaccount,
    }
    ///Container type for all input parameters for the `legacySpotTick` function with signature `legacySpotTick((uint128))` and selector `0xf80f7ce5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "legacySpotTick", abi = "legacySpotTick((uint128))")]
    pub struct LegacySpotTickCall {
        pub p: LegacySpotTick,
    }
    ///Container type for all input parameters for the `legacyUnsignedLiquidateSubaccount` function with signature `legacyUnsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0xdc42e61b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "legacyUnsignedLiquidateSubaccount",
        abi = "legacyUnsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LegacyUnsignedLiquidateSubaccountCall {
        pub p: LegacyLiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liquidationStart` function with signature `liquidationStart(bytes)` and selector `0x8d0acc9b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "liquidationStart", abi = "liquidationStart(bytes)")]
    pub struct LiquidationStartCall {
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert((int128[],int128[],int128[]))` and selector `0x2c8c6ffb`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "manualAssert",
        abi = "manualAssert((int128[],int128[],int128[]))"
    )]
    pub struct ManualAssertCall {
        pub p: ManualAssert,
    }
    ///Container type for all input parameters for the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0x36b90c51`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "matchOrderAMM",
        abi = "matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))"
    )]
    pub struct MatchOrderAMMCall {
        pub p: MatchOrderAMM,
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0x8d3c20b1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "matchOrders",
        abi = "matchOrders((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))"
    )]
    pub struct MatchOrdersCall {
        pub p: MatchOrders,
    }
    ///Container type for all input parameters for the `nSubmissions` function with signature `nSubmissions()` and selector `0x18ed16eb`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nSubmissions", abi = "nSubmissions()")]
    pub struct NsubmissionsCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `0x5a00923b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "perpTick", abi = "perpTick((uint128,int128[]))")]
    pub struct PerpTickCall {
        pub p: PerpTick,
    }
    ///Container type for all input parameters for the `processSlowModeTransaction` function with signature `processSlowModeTransaction(address,bytes)` and selector `0x87324338`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "processSlowModeTransaction",
        abi = "processSlowModeTransaction(address,bytes)"
    )]
    pub struct ProcessSlowModeTransactionCall {
        pub sender: ::ethers::core::types::Address,
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw((uint32,uint128,address))` and selector `0x9a08e535`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "rebalanceXWithdraw",
        abi = "rebalanceXWithdraw((uint32,uint128,address))"
    )]
    pub struct RebalanceXWithdrawCall {
        pub p: RebalanceXWithdraw,
    }
    ///Container type for all input parameters for the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `0x42c74d1d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "rebate", abi = "rebate((bytes32[],int128[]))")]
    pub struct RebateCall {
        pub p: Rebate,
    }
    ///Container type for all input parameters for the `referralCodes` function with signature `referralCodes(address)` and selector `0x9534dd3e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "referralCodes", abi = "referralCodes(address)")]
    pub struct ReferralCodesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `registerTransferableWallet` function with signature `registerTransferableWallet(address,bool)` and selector `0x6cfe5fe4`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "registerTransferableWallet",
        abi = "registerTransferableWallet(address,bool)"
    )]
    pub struct RegisterTransferableWalletCall {
        pub wallet: ::ethers::core::types::Address,
        pub transferable: bool,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `resyncSlowModeTxs` function with signature `resyncSlowModeTxs()` and selector `0x21047589`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "resyncSlowModeTxs", abi = "resyncSlowModeTxs()")]
    pub struct ResyncSlowModeTxsCall;
    ///Container type for all input parameters for the `setPriceX18` function with signature `setPriceX18(uint32,int128)` and selector `0x8c58e10a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setPriceX18", abi = "setPriceX18(uint32,int128)")]
    pub struct SetPriceX18Call {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all input parameters for the `setSlowModeConfig` function with signature `setSlowModeConfig((uint64,uint64,uint64))` and selector `0x3216c062`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setSlowModeConfig",
        abi = "setSlowModeConfig((uint64,uint64,uint64))"
    )]
    pub struct SetSlowModeConfigCall {
        pub slow_mode_config: SlowModeConfig,
    }
    ///Container type for all input parameters for the `setSlowModeTx` function with signature `setSlowModeTx(uint64,(uint64,address,bytes))` and selector `0x98cd32fe`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setSlowModeTx",
        abi = "setSlowModeTx(uint64,(uint64,address,bytes))"
    )]
    pub struct SetSlowModeTxCall {
        pub idx: u64,
        pub txn: SlowModeTx,
    }
    ///Container type for all input parameters for the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `0xb2bb6367`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "settlePnl", abi = "settlePnl((bytes32[],uint256[]))")]
    pub struct SettlePnlCall {
        pub p: SettlePnl,
    }
    ///Container type for all input parameters for the `signedBurnLp` function with signature `signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x610b2e5e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedBurnLp",
        abi = "signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))"
    )]
    pub struct SignedBurnLpCall {
        pub p: SignedBurnLp,
    }
    ///Container type for all input parameters for the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `0x3edf2c5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedCancellation",
        abi = "signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))"
    )]
    pub struct SignedCancellationCall {
        pub p: SignedCancellation,
    }
    ///Container type for all input parameters for the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `0xa082c5aa`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedCancellationProducts",
        abi = "signedCancellationProducts(((bytes32,uint32[],uint64),bytes))"
    )]
    pub struct SignedCancellationProductsCall {
        pub p: SignedCancellationProducts,
    }
    ///Container type for all input parameters for the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `0x85c83e9d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedLinkSigner",
        abi = "signedLinkSigner(((bytes32,bytes32,uint64),bytes))"
    )]
    pub struct SignedLinkSignerCall {
        pub p: SignedLinkSigner,
    }
    ///Container type for all input parameters for the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))` and selector `0x9171d08b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedLiquidateSubaccount",
        abi = "signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))"
    )]
    pub struct SignedLiquidateSubaccountCall {
        pub p: SignedLiquidateSubaccount,
    }
    ///Container type for all input parameters for the `signedMintLp` function with signature `signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))` and selector `0xd38c3b9c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedMintLp",
        abi = "signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))"
    )]
    pub struct SignedMintLpCall {
        pub p: SignedMintLp,
    }
    ///Container type for all input parameters for the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `0x6c457570`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedOrder",
        abi = "signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))"
    )]
    pub struct SignedOrderCall {
        pub p: SignedOrder,
    }
    ///Container type for all input parameters for the `signedTransferQuote` function with signature `signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))` and selector `0x6f3b0a72`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedTransferQuote",
        abi = "signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))"
    )]
    pub struct SignedTransferQuoteCall {
        pub p: SignedTransferQuote,
    }
    ///Container type for all input parameters for the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x0d55e26b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "signedWithdrawCollateral",
        abi = "signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))"
    )]
    pub struct SignedWithdrawCollateralCall {
        pub p: SignedWithdrawCollateral,
    }
    ///Container type for all input parameters for the `spotTick` function with signature `spotTick((uint128,int128[]))` and selector `0xb1c8ec2b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "spotTick", abi = "spotTick((uint128,int128[]))")]
    pub struct SpotTickCall {
        pub p: SpotTick,
    }
    ///Container type for all input parameters for the `submitSlowModeTransaction` function with signature `submitSlowModeTransaction(bytes)` and selector `0xe604ed9e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitSlowModeTransaction",
        abi = "submitSlowModeTransaction(bytes)"
    )]
    pub struct SubmitSlowModeTransactionCall {
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitTransactions` function with signature `submitTransactions(bytes[])` and selector `0x1f186b27`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "submitTransactions", abi = "submitTransactions(bytes[])")]
    pub struct SubmitTransactionsCall {
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `submitTransactionsChecked` function with signature `submitTransactionsChecked(uint64,bytes[],bytes32,bytes32,uint8)` and selector `0x7db6a25b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitTransactionsChecked",
        abi = "submitTransactionsChecked(uint64,bytes[],bytes32,bytes32,uint8)"
    )]
    pub struct SubmitTransactionsCheckedCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub e: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub s: [u8; 32],
        pub signer_bitmask: u8,
    }
    ///Container type for all input parameters for the `submitTransactionsCheckedWithGasLimit` function with signature `submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)` and selector `0x2f9a2744`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitTransactionsCheckedWithGasLimit",
        abi = "submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)"
    )]
    pub struct SubmitTransactionsCheckedWithGasLimitCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub gas_limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `0x0f4b509d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "swapAMM", abi = "swapAMM((bytes32,uint32,int128,int128))")]
    pub struct SwapAMMCall {
        pub p: SwapAMM,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferQuote` function with signature `transferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x1d97d22f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "transferQuote",
        abi = "transferQuote((bytes32,bytes32,uint128,uint64))"
    )]
    pub struct TransferQuoteCall {
        pub p: TransferQuote,
    }
    ///Container type for all input parameters for the `unsignedBurnLp` function with signature `unsignedBurnLp((bytes32,uint32,uint128,uint64))` and selector `0x06c0bafd`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedBurnLp",
        abi = "unsignedBurnLp((bytes32,uint32,uint128,uint64))"
    )]
    pub struct UnsignedBurnLpCall {
        pub p: BurnLp,
    }
    ///Container type for all input parameters for the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `0x3cec4b93`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedDepositCollateral",
        abi = "unsignedDepositCollateral((bytes32,uint32,uint128))"
    )]
    pub struct UnsignedDepositCollateralCall {
        pub p: DepositCollateral,
    }
    ///Container type for all input parameters for the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `0x94faefe5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedDepositInsurance",
        abi = "unsignedDepositInsurance((uint128))"
    )]
    pub struct UnsignedDepositInsuranceCall {
        pub p: DepositInsurance,
    }
    ///Container type for all input parameters for the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `0x05e42dc7`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedLinkSigner",
        abi = "unsignedLinkSigner((bytes32,bytes32,uint64))"
    )]
    pub struct UnsignedLinkSignerCall {
        pub p: LinkSigner,
    }
    ///Container type for all input parameters for the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))` and selector `0x9e851424`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedLiquidateSubaccount",
        abi = "unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))"
    )]
    pub struct UnsignedLiquidateSubaccountCall {
        pub p: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `unsignedMintLp` function with signature `unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `0x910e606a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedMintLp",
        abi = "unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct UnsignedMintLpCall {
        pub p: MintLp,
    }
    ///Container type for all input parameters for the `unsignedTransferQuote` function with signature `unsignedTransferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x0edaacce`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedTransferQuote",
        abi = "unsignedTransferQuote((bytes32,bytes32,uint128,uint64))"
    )]
    pub struct UnsignedTransferQuoteCall {
        pub p: TransferQuote,
    }
    ///Container type for all input parameters for the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `0xbc85ca86`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedWithdrawCollateral",
        abi = "unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))"
    )]
    pub struct UnsignedWithdrawCollateralCall {
        pub p: WithdrawCollateral,
    }
    ///Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `0x35639a4f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateFeeRates",
        abi = "updateFeeRates((address,uint32,int64,int64))"
    )]
    pub struct UpdateFeeRatesCall {
        pub p: UpdateFeeRates,
    }
    ///Container type for all input parameters for the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `0x1d9eeda5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updatePrice", abi = "updatePrice((uint32,int128))")]
    pub struct UpdatePriceCall {
        pub p: UpdatePrice,
    }
    ///Container type for all input parameters for the `updateProduct` function with signature `updateProduct((address,bytes))` and selector `0x2cd71b16`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateProduct", abi = "updateProduct((address,bytes))")]
    pub struct UpdateProductCall {
        pub p: UpdateProduct,
    }
    ///Container type for all input parameters for the `updateSanctions` function with signature `updateSanctions(address)` and selector `0xf2633927`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateSanctions", abi = "updateSanctions(address)")]
    pub struct UpdateSanctionsCall {
        pub sanctions: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum EndpointCalls {
        BurnLpAndTransfer(BurnLpAndTransferCall),
        ChainlinkFullReport(ChainlinkFullReportCall),
        ChainlinkReportBlob(ChainlinkReportBlobCall),
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
        GetHealthCheckFee(GetHealthCheckFeeCall),
        GetLinkedSigner(GetLinkedSignerCall),
        GetLiquidationFee(GetLiquidationFeeCall),
        GetNonce(GetNonceCall),
        GetNumSubaccounts(GetNumSubaccountsCall),
        GetOffchainExchange(GetOffchainExchangeCall),
        GetPriceX18(GetPriceX18Call),
        GetSequencer(GetSequencerCall),
        GetSlowModeTx(GetSlowModeTxCall),
        GetSubaccountById(GetSubaccountByIdCall),
        GetSubaccountId(GetSubaccountIdCall),
        GetTakerSequencerFee(GetTakerSequencerFeeCall),
        GetTime(GetTimeCall),
        GetTimes(GetTimesCall),
        GetVersion(GetVersionCall),
        IncrementSubmissions(IncrementSubmissionsCall),
        Initialize(InitializeCall),
        LegacyMatchOrders(LegacyMatchOrdersCall),
        LegacySignedLiquidateSubaccount(LegacySignedLiquidateSubaccountCall),
        LegacySpotTick(LegacySpotTickCall),
        LegacyUnsignedLiquidateSubaccount(LegacyUnsignedLiquidateSubaccountCall),
        LiquidationStart(LiquidationStartCall),
        ManualAssert(ManualAssertCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        Nsubmissions(NsubmissionsCall),
        Owner(OwnerCall),
        PerpTick(PerpTickCall),
        ProcessSlowModeTransaction(ProcessSlowModeTransactionCall),
        RebalanceXWithdraw(RebalanceXWithdrawCall),
        Rebate(RebateCall),
        ReferralCodes(ReferralCodesCall),
        RegisterTransferableWallet(RegisterTransferableWalletCall),
        RenounceOwnership(RenounceOwnershipCall),
        ResyncSlowModeTxs(ResyncSlowModeTxsCall),
        SetPriceX18(SetPriceX18Call),
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
        SignedTransferQuote(SignedTransferQuoteCall),
        SignedWithdrawCollateral(SignedWithdrawCollateralCall),
        SpotTick(SpotTickCall),
        SubmitSlowModeTransaction(SubmitSlowModeTransactionCall),
        SubmitTransactions(SubmitTransactionsCall),
        SubmitTransactionsChecked(SubmitTransactionsCheckedCall),
        SubmitTransactionsCheckedWithGasLimit(SubmitTransactionsCheckedWithGasLimitCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        TransferQuote(TransferQuoteCall),
        UnsignedBurnLp(UnsignedBurnLpCall),
        UnsignedDepositCollateral(UnsignedDepositCollateralCall),
        UnsignedDepositInsurance(UnsignedDepositInsuranceCall),
        UnsignedLinkSigner(UnsignedLinkSignerCall),
        UnsignedLiquidateSubaccount(UnsignedLiquidateSubaccountCall),
        UnsignedMintLp(UnsignedMintLpCall),
        UnsignedTransferQuote(UnsignedTransferQuoteCall),
        UnsignedWithdrawCollateral(UnsignedWithdrawCollateralCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
        UpdateSanctions(UpdateSanctionsCall),
    }
    impl ::ethers::core::abi::AbiDecode for EndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BurnLpAndTransferCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnLpAndTransfer(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkFullReportCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainlinkFullReport(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkReportBlobCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainlinkReportBlob(decoded));
            }
            if let Ok(decoded) = <CheckLpActionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckLpAction(decoded));
            }
            if let Ok(decoded) =
                <CheckLpActionInnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckLpActionInner(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxInnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSlowModeTxInner(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSlowModeTxLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <ClaimSequencerFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClaimSequencerFees(decoded));
            }
            if let Ok(decoded) = <ClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Clearinghouse(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralWithReferralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositCollateralWithReferral(decoded));
            }
            if let Ok(decoded) = <DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <ExecuteSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetHealthCheckFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHealthCheckFee(decoded));
            }
            if let Ok(decoded) =
                <GetLinkedSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLinkedSigner(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidationFee(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetNumSubaccountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNumSubaccounts(decoded));
            }
            if let Ok(decoded) =
                <GetOffchainExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOffchainExchange(decoded));
            }
            if let Ok(decoded) = <GetPriceX18Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPriceX18(decoded));
            }
            if let Ok(decoded) = <GetSequencerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSequencer(decoded));
            }
            if let Ok(decoded) = <GetSlowModeTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSlowModeTx(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountByIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountById(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountId(decoded));
            }
            if let Ok(decoded) =
                <GetTakerSequencerFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTakerSequencerFee(decoded));
            }
            if let Ok(decoded) = <GetTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTime(decoded));
            }
            if let Ok(decoded) = <GetTimesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTimes(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) =
                <IncrementSubmissionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementSubmissions(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LegacyMatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LegacyMatchOrders(decoded));
            }
            if let Ok(decoded) =
                <LegacySignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LegacySignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <LegacySpotTickCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LegacySpotTick(decoded));
            }
            if let Ok(decoded) =
                <LegacyUnsignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LegacyUnsignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <LiquidationStartCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidationStart(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MatchOrderAMMCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded) = <NsubmissionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Nsubmissions(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PerpTickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PerpTick(decoded));
            }
            if let Ok(decoded) =
                <ProcessSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <RebalanceXWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceXWithdraw(decoded));
            }
            if let Ok(decoded) = <RebateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rebate(decoded));
            }
            if let Ok(decoded) = <ReferralCodesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReferralCodes(decoded));
            }
            if let Ok(decoded) =
                <RegisterTransferableWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterTransferableWallet(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ResyncSlowModeTxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ResyncSlowModeTxs(decoded));
            }
            if let Ok(decoded) = <SetPriceX18Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPriceX18(decoded));
            }
            if let Ok(decoded) =
                <SetSlowModeConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSlowModeConfig(decoded));
            }
            if let Ok(decoded) = <SetSlowModeTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSlowModeTx(decoded));
            }
            if let Ok(decoded) = <SettlePnlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SettlePnl(decoded));
            }
            if let Ok(decoded) = <SignedBurnLpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedBurnLp(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedCancellation(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedCancellationProducts(decoded));
            }
            if let Ok(decoded) =
                <SignedLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <SignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) = <SignedMintLpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedMintLp(decoded));
            }
            if let Ok(decoded) = <SignedOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignedOrder(decoded));
            }
            if let Ok(decoded) =
                <SignedTransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedTransferQuote(decoded));
            }
            if let Ok(decoded) =
                <SignedWithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) = <SpotTickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SpotTick(decoded));
            }
            if let Ok(decoded) =
                <SubmitSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitTransactions(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCheckedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitTransactionsChecked(decoded));
            }
            if let Ok(decoded) = <SubmitTransactionsCheckedWithGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitTransactionsCheckedWithGasLimit(decoded));
            }
            if let Ok(decoded) = <SwapAMMCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapAMM(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferQuote(decoded));
            }
            if let Ok(decoded) =
                <UnsignedBurnLpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedBurnLp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDepositInsurance(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <UnsignedMintLpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedMintLp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedTransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedTransferQuote(decoded));
            }
            if let Ok(decoded) =
                <UnsignedWithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) = <UpdatePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePrice(decoded));
            }
            if let Ok(decoded) = <UpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateProduct(decoded));
            }
            if let Ok(decoded) =
                <UpdateSanctionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateSanctions(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BurnLpAndTransfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainlinkFullReport(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainlinkReportBlob(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckLpAction(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckLpActionInner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSlowModeTxInner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSlowModeTxLinkSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimSequencerFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositCollateral(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositCollateralWithReferral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthCheckFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLinkedSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidationFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNumSubaccounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOffchainExchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSequencer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlowModeTx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountById(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTakerSequencerFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTimes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementSubmissions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LegacyMatchOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LegacySignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LegacySpotTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LegacyUnsignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationStart(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrderAMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nsubmissions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceXWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rebate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReferralCodes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterTransferableWallet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResyncSlowModeTxs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPriceX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSlowModeConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSlowModeTx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettlePnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedBurnLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedCancellation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedCancellationProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedLinkSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedMintLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedTransferQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedWithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpotTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactionsChecked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactionsCheckedWithGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapAMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferQuote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedBurnLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedDepositCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedDepositInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedLinkSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedMintLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedTransferQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedWithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeRates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateSanctions(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EndpointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnLpAndTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkFullReport(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkReportBlob(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckLpAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckLpActionInner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxInner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimSequencerFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositCollateralWithReferral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthCheckFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLinkedSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidationFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNumSubaccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOffchainExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlowModeTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountById(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTakerSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementSubmissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LegacyMatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::LegacySignedLiquidateSubaccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LegacySpotTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::LegacyUnsignedLiquidateSubaccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationStart(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrderAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nsubmissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceXWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rebate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferralCodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterTransferableWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResyncSlowModeTxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSlowModeConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSlowModeTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlePnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedBurnLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellationProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedTransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedWithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsChecked(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsCheckedWithGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedBurnLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedMintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedTransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedWithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSanctions(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnLpAndTransferCall> for EndpointCalls {
        fn from(value: BurnLpAndTransferCall) -> Self {
            Self::BurnLpAndTransfer(value)
        }
    }
    impl ::core::convert::From<ChainlinkFullReportCall> for EndpointCalls {
        fn from(value: ChainlinkFullReportCall) -> Self {
            Self::ChainlinkFullReport(value)
        }
    }
    impl ::core::convert::From<ChainlinkReportBlobCall> for EndpointCalls {
        fn from(value: ChainlinkReportBlobCall) -> Self {
            Self::ChainlinkReportBlob(value)
        }
    }
    impl ::core::convert::From<CheckLpActionCall> for EndpointCalls {
        fn from(value: CheckLpActionCall) -> Self {
            Self::CheckLpAction(value)
        }
    }
    impl ::core::convert::From<CheckLpActionInnerCall> for EndpointCalls {
        fn from(value: CheckLpActionInnerCall) -> Self {
            Self::CheckLpActionInner(value)
        }
    }
    impl ::core::convert::From<CheckSlowModeTxInnerCall> for EndpointCalls {
        fn from(value: CheckSlowModeTxInnerCall) -> Self {
            Self::CheckSlowModeTxInner(value)
        }
    }
    impl ::core::convert::From<CheckSlowModeTxLinkSignerCall> for EndpointCalls {
        fn from(value: CheckSlowModeTxLinkSignerCall) -> Self {
            Self::CheckSlowModeTxLinkSigner(value)
        }
    }
    impl ::core::convert::From<ClaimSequencerFeesCall> for EndpointCalls {
        fn from(value: ClaimSequencerFeesCall) -> Self {
            Self::ClaimSequencerFees(value)
        }
    }
    impl ::core::convert::From<ClearinghouseCall> for EndpointCalls {
        fn from(value: ClearinghouseCall) -> Self {
            Self::Clearinghouse(value)
        }
    }
    impl ::core::convert::From<DepositCollateralCall> for EndpointCalls {
        fn from(value: DepositCollateralCall) -> Self {
            Self::DepositCollateral(value)
        }
    }
    impl ::core::convert::From<DepositCollateralWithReferralCall> for EndpointCalls {
        fn from(value: DepositCollateralWithReferralCall) -> Self {
            Self::DepositCollateralWithReferral(value)
        }
    }
    impl
        ::core::convert::From<
            DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall,
        > for EndpointCalls
    {
        fn from(
            value: DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmountCall,
        ) -> Self {
            Self::DepositCollateralWithReferralWithSubaccountNameAndProductIdAndAmount(value)
        }
    }
    impl ::core::convert::From<ExecuteSlowModeTransactionCall> for EndpointCalls {
        fn from(value: ExecuteSlowModeTransactionCall) -> Self {
            Self::ExecuteSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<GetHealthCheckFeeCall> for EndpointCalls {
        fn from(value: GetHealthCheckFeeCall) -> Self {
            Self::GetHealthCheckFee(value)
        }
    }
    impl ::core::convert::From<GetLinkedSignerCall> for EndpointCalls {
        fn from(value: GetLinkedSignerCall) -> Self {
            Self::GetLinkedSigner(value)
        }
    }
    impl ::core::convert::From<GetLiquidationFeeCall> for EndpointCalls {
        fn from(value: GetLiquidationFeeCall) -> Self {
            Self::GetLiquidationFee(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for EndpointCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetNumSubaccountsCall> for EndpointCalls {
        fn from(value: GetNumSubaccountsCall) -> Self {
            Self::GetNumSubaccounts(value)
        }
    }
    impl ::core::convert::From<GetOffchainExchangeCall> for EndpointCalls {
        fn from(value: GetOffchainExchangeCall) -> Self {
            Self::GetOffchainExchange(value)
        }
    }
    impl ::core::convert::From<GetPriceX18Call> for EndpointCalls {
        fn from(value: GetPriceX18Call) -> Self {
            Self::GetPriceX18(value)
        }
    }
    impl ::core::convert::From<GetSequencerCall> for EndpointCalls {
        fn from(value: GetSequencerCall) -> Self {
            Self::GetSequencer(value)
        }
    }
    impl ::core::convert::From<GetSlowModeTxCall> for EndpointCalls {
        fn from(value: GetSlowModeTxCall) -> Self {
            Self::GetSlowModeTx(value)
        }
    }
    impl ::core::convert::From<GetSubaccountByIdCall> for EndpointCalls {
        fn from(value: GetSubaccountByIdCall) -> Self {
            Self::GetSubaccountById(value)
        }
    }
    impl ::core::convert::From<GetSubaccountIdCall> for EndpointCalls {
        fn from(value: GetSubaccountIdCall) -> Self {
            Self::GetSubaccountId(value)
        }
    }
    impl ::core::convert::From<GetTakerSequencerFeeCall> for EndpointCalls {
        fn from(value: GetTakerSequencerFeeCall) -> Self {
            Self::GetTakerSequencerFee(value)
        }
    }
    impl ::core::convert::From<GetTimeCall> for EndpointCalls {
        fn from(value: GetTimeCall) -> Self {
            Self::GetTime(value)
        }
    }
    impl ::core::convert::From<GetTimesCall> for EndpointCalls {
        fn from(value: GetTimesCall) -> Self {
            Self::GetTimes(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for EndpointCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<IncrementSubmissionsCall> for EndpointCalls {
        fn from(value: IncrementSubmissionsCall) -> Self {
            Self::IncrementSubmissions(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EndpointCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LegacyMatchOrdersCall> for EndpointCalls {
        fn from(value: LegacyMatchOrdersCall) -> Self {
            Self::LegacyMatchOrders(value)
        }
    }
    impl ::core::convert::From<LegacySignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: LegacySignedLiquidateSubaccountCall) -> Self {
            Self::LegacySignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<LegacySpotTickCall> for EndpointCalls {
        fn from(value: LegacySpotTickCall) -> Self {
            Self::LegacySpotTick(value)
        }
    }
    impl ::core::convert::From<LegacyUnsignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: LegacyUnsignedLiquidateSubaccountCall) -> Self {
            Self::LegacyUnsignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<LiquidationStartCall> for EndpointCalls {
        fn from(value: LiquidationStartCall) -> Self {
            Self::LiquidationStart(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for EndpointCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<MatchOrderAMMCall> for EndpointCalls {
        fn from(value: MatchOrderAMMCall) -> Self {
            Self::MatchOrderAMM(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for EndpointCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<NsubmissionsCall> for EndpointCalls {
        fn from(value: NsubmissionsCall) -> Self {
            Self::Nsubmissions(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for EndpointCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpTickCall> for EndpointCalls {
        fn from(value: PerpTickCall) -> Self {
            Self::PerpTick(value)
        }
    }
    impl ::core::convert::From<ProcessSlowModeTransactionCall> for EndpointCalls {
        fn from(value: ProcessSlowModeTransactionCall) -> Self {
            Self::ProcessSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<RebalanceXWithdrawCall> for EndpointCalls {
        fn from(value: RebalanceXWithdrawCall) -> Self {
            Self::RebalanceXWithdraw(value)
        }
    }
    impl ::core::convert::From<RebateCall> for EndpointCalls {
        fn from(value: RebateCall) -> Self {
            Self::Rebate(value)
        }
    }
    impl ::core::convert::From<ReferralCodesCall> for EndpointCalls {
        fn from(value: ReferralCodesCall) -> Self {
            Self::ReferralCodes(value)
        }
    }
    impl ::core::convert::From<RegisterTransferableWalletCall> for EndpointCalls {
        fn from(value: RegisterTransferableWalletCall) -> Self {
            Self::RegisterTransferableWallet(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for EndpointCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ResyncSlowModeTxsCall> for EndpointCalls {
        fn from(value: ResyncSlowModeTxsCall) -> Self {
            Self::ResyncSlowModeTxs(value)
        }
    }
    impl ::core::convert::From<SetPriceX18Call> for EndpointCalls {
        fn from(value: SetPriceX18Call) -> Self {
            Self::SetPriceX18(value)
        }
    }
    impl ::core::convert::From<SetSlowModeConfigCall> for EndpointCalls {
        fn from(value: SetSlowModeConfigCall) -> Self {
            Self::SetSlowModeConfig(value)
        }
    }
    impl ::core::convert::From<SetSlowModeTxCall> for EndpointCalls {
        fn from(value: SetSlowModeTxCall) -> Self {
            Self::SetSlowModeTx(value)
        }
    }
    impl ::core::convert::From<SettlePnlCall> for EndpointCalls {
        fn from(value: SettlePnlCall) -> Self {
            Self::SettlePnl(value)
        }
    }
    impl ::core::convert::From<SignedBurnLpCall> for EndpointCalls {
        fn from(value: SignedBurnLpCall) -> Self {
            Self::SignedBurnLp(value)
        }
    }
    impl ::core::convert::From<SignedCancellationCall> for EndpointCalls {
        fn from(value: SignedCancellationCall) -> Self {
            Self::SignedCancellation(value)
        }
    }
    impl ::core::convert::From<SignedCancellationProductsCall> for EndpointCalls {
        fn from(value: SignedCancellationProductsCall) -> Self {
            Self::SignedCancellationProducts(value)
        }
    }
    impl ::core::convert::From<SignedLinkSignerCall> for EndpointCalls {
        fn from(value: SignedLinkSignerCall) -> Self {
            Self::SignedLinkSigner(value)
        }
    }
    impl ::core::convert::From<SignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: SignedLiquidateSubaccountCall) -> Self {
            Self::SignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<SignedMintLpCall> for EndpointCalls {
        fn from(value: SignedMintLpCall) -> Self {
            Self::SignedMintLp(value)
        }
    }
    impl ::core::convert::From<SignedOrderCall> for EndpointCalls {
        fn from(value: SignedOrderCall) -> Self {
            Self::SignedOrder(value)
        }
    }
    impl ::core::convert::From<SignedTransferQuoteCall> for EndpointCalls {
        fn from(value: SignedTransferQuoteCall) -> Self {
            Self::SignedTransferQuote(value)
        }
    }
    impl ::core::convert::From<SignedWithdrawCollateralCall> for EndpointCalls {
        fn from(value: SignedWithdrawCollateralCall) -> Self {
            Self::SignedWithdrawCollateral(value)
        }
    }
    impl ::core::convert::From<SpotTickCall> for EndpointCalls {
        fn from(value: SpotTickCall) -> Self {
            Self::SpotTick(value)
        }
    }
    impl ::core::convert::From<SubmitSlowModeTransactionCall> for EndpointCalls {
        fn from(value: SubmitSlowModeTransactionCall) -> Self {
            Self::SubmitSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCall) -> Self {
            Self::SubmitTransactions(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCheckedCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCheckedCall) -> Self {
            Self::SubmitTransactionsChecked(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCheckedWithGasLimitCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCheckedWithGasLimitCall) -> Self {
            Self::SubmitTransactionsCheckedWithGasLimit(value)
        }
    }
    impl ::core::convert::From<SwapAMMCall> for EndpointCalls {
        fn from(value: SwapAMMCall) -> Self {
            Self::SwapAMM(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for EndpointCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferQuoteCall> for EndpointCalls {
        fn from(value: TransferQuoteCall) -> Self {
            Self::TransferQuote(value)
        }
    }
    impl ::core::convert::From<UnsignedBurnLpCall> for EndpointCalls {
        fn from(value: UnsignedBurnLpCall) -> Self {
            Self::UnsignedBurnLp(value)
        }
    }
    impl ::core::convert::From<UnsignedDepositCollateralCall> for EndpointCalls {
        fn from(value: UnsignedDepositCollateralCall) -> Self {
            Self::UnsignedDepositCollateral(value)
        }
    }
    impl ::core::convert::From<UnsignedDepositInsuranceCall> for EndpointCalls {
        fn from(value: UnsignedDepositInsuranceCall) -> Self {
            Self::UnsignedDepositInsurance(value)
        }
    }
    impl ::core::convert::From<UnsignedLinkSignerCall> for EndpointCalls {
        fn from(value: UnsignedLinkSignerCall) -> Self {
            Self::UnsignedLinkSigner(value)
        }
    }
    impl ::core::convert::From<UnsignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: UnsignedLiquidateSubaccountCall) -> Self {
            Self::UnsignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<UnsignedMintLpCall> for EndpointCalls {
        fn from(value: UnsignedMintLpCall) -> Self {
            Self::UnsignedMintLp(value)
        }
    }
    impl ::core::convert::From<UnsignedTransferQuoteCall> for EndpointCalls {
        fn from(value: UnsignedTransferQuoteCall) -> Self {
            Self::UnsignedTransferQuote(value)
        }
    }
    impl ::core::convert::From<UnsignedWithdrawCollateralCall> for EndpointCalls {
        fn from(value: UnsignedWithdrawCollateralCall) -> Self {
            Self::UnsignedWithdrawCollateral(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRatesCall> for EndpointCalls {
        fn from(value: UpdateFeeRatesCall) -> Self {
            Self::UpdateFeeRates(value)
        }
    }
    impl ::core::convert::From<UpdatePriceCall> for EndpointCalls {
        fn from(value: UpdatePriceCall) -> Self {
            Self::UpdatePrice(value)
        }
    }
    impl ::core::convert::From<UpdateProductCall> for EndpointCalls {
        fn from(value: UpdateProductCall) -> Self {
            Self::UpdateProduct(value)
        }
    }
    impl ::core::convert::From<UpdateSanctionsCall> for EndpointCalls {
        fn from(value: UpdateSanctionsCall) -> Self {
            Self::UpdateSanctions(value)
        }
    }
    ///Container type for all return fields from the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `0x0748a219`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLpAndTransferReturn(pub BurnLpAndTransfer);
    ///Container type for all return fields from the `chainlinkFullReport` function with signature `chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))` and selector `0xdb5a5021`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ChainlinkFullReportReturn(pub ChainlinkFullReport);
    ///Container type for all return fields from the `chainlinkReportBlob` function with signature `chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))` and selector `0x96c47c6f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ChainlinkReportBlobReturn(pub ChainlinkReportBlob);
    ///Container type for all return fields from the `checkLpAction` function with signature `checkLpAction()` and selector `0x8c3d2f74`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckLpActionReturn(pub u32);
    ///Container type for all return fields from the `checkLpActionInner` function with signature `checkLpActionInner(address,bytes)` and selector `0xc345530b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckLpActionInnerReturn(pub u32);
    ///Container type for all return fields from the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `0xb70eb263`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckSlowModeTxInnerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `0x5bb4c126`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckSlowModeTxLinkSignerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `claimSequencerFees` function with signature `claimSequencerFees((bytes32))` and selector `0x3842e75e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ClaimSequencerFeesReturn(pub ClaimSequencerFees);
    ///Container type for all return fields from the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ClearinghouseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `0xd4de8d9d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHealthCheckFeeReturn(pub i128);
    ///Container type for all return fields from the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `0x91c1e3d7`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLinkedSignerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `0xfbf41984`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidationFeeReturn(pub i128);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNonceReturn(pub u64);
    ///Container type for all return fields from the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `0xc4f9b25f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNumSubaccountsReturn(pub u64);
    ///Container type for all return fields from the `getOffchainExchange` function with signature `getOffchainExchange()` and selector `0x8f4f8ecc`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetOffchainExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `0x368e4686`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPriceX18Return {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all return fields from the `getSequencer` function with signature `getSequencer()` and selector `0x4d96a90a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSequencerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSlowModeTx` function with signature `getSlowModeTx(uint64)` and selector `0xee525526`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSlowModeTxReturn(pub SlowModeTx, pub u64, pub u64);
    ///Container type for all return fields from the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `0xef64ed0e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSubaccountByIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `0x22d4a82d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSubaccountIdReturn(pub u64);
    ///Container type for all return fields from the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `0xc510359f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTakerSequencerFeeReturn(pub i128);
    ///Container type for all return fields from the `getTime` function with signature `getTime()` and selector `0x557ed1ba`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTimeReturn(pub u128);
    ///Container type for all return fields from the `getTimes` function with signature `getTimes()` and selector `0xe9ab77e5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTimesReturn(pub Times);
    ///Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetVersionReturn(pub u64);
    ///Container type for all return fields from the `legacyMatchOrders` function with signature `legacyMatchOrders((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0xb36488b8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacyMatchOrdersReturn(pub LegacyMatchOrders);
    ///Container type for all return fields from the `legacySignedLiquidateSubaccount` function with signature `legacySignedLiquidateSubaccount(((bytes32,bytes32,uint8,uint32,int128,uint64),bytes))` and selector `0xb1fbd60b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacySignedLiquidateSubaccountReturn(pub LegacySignedLiquidateSubaccount);
    ///Container type for all return fields from the `legacySpotTick` function with signature `legacySpotTick((uint128))` and selector `0xf80f7ce5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacySpotTickReturn(pub LegacySpotTick);
    ///Container type for all return fields from the `legacyUnsignedLiquidateSubaccount` function with signature `legacyUnsignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0xdc42e61b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacyUnsignedLiquidateSubaccountReturn(pub LegacyLiquidateSubaccount);
    ///Container type for all return fields from the `manualAssert` function with signature `manualAssert((int128[],int128[],int128[]))` and selector `0x2c8c6ffb`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ManualAssertReturn(pub ManualAssert);
    ///Container type for all return fields from the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0x36b90c51`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MatchOrderAMMReturn(pub MatchOrderAMM);
    ///Container type for all return fields from the `matchOrders` function with signature `matchOrders((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)))` and selector `0x8d3c20b1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MatchOrdersReturn(pub MatchOrders);
    ///Container type for all return fields from the `nSubmissions` function with signature `nSubmissions()` and selector `0x18ed16eb`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NsubmissionsReturn(pub u64);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `0x5a00923b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PerpTickReturn(pub PerpTick);
    ///Container type for all return fields from the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw((uint32,uint128,address))` and selector `0x9a08e535`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RebalanceXWithdrawReturn(pub RebalanceXWithdraw);
    ///Container type for all return fields from the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `0x42c74d1d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RebateReturn(pub Rebate);
    ///Container type for all return fields from the `referralCodes` function with signature `referralCodes(address)` and selector `0x9534dd3e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ReferralCodesReturn(pub ::std::string::String);
    ///Container type for all return fields from the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `0xb2bb6367`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SettlePnlReturn(pub SettlePnl);
    ///Container type for all return fields from the `signedBurnLp` function with signature `signedBurnLp(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x610b2e5e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedBurnLpReturn(pub SignedBurnLp);
    ///Container type for all return fields from the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `0x3edf2c5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedCancellationReturn(pub SignedCancellation);
    ///Container type for all return fields from the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `0xa082c5aa`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedCancellationProductsReturn(pub SignedCancellationProducts);
    ///Container type for all return fields from the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `0x85c83e9d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedLinkSignerReturn(pub SignedLinkSigner);
    ///Container type for all return fields from the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))` and selector `0x9171d08b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedLiquidateSubaccountReturn(pub SignedLiquidateSubaccount);
    ///Container type for all return fields from the `signedMintLp` function with signature `signedMintLp(((bytes32,uint32,uint128,uint128,uint128,uint64),bytes))` and selector `0xd38c3b9c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedMintLpReturn(pub SignedMintLp);
    ///Container type for all return fields from the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `0x6c457570`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedOrderReturn(pub SignedOrder);
    ///Container type for all return fields from the `signedTransferQuote` function with signature `signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))` and selector `0x6f3b0a72`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedTransferQuoteReturn(pub SignedTransferQuote);
    ///Container type for all return fields from the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x0d55e26b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedWithdrawCollateralReturn(pub SignedWithdrawCollateral);
    ///Container type for all return fields from the `spotTick` function with signature `spotTick((uint128,int128[]))` and selector `0xb1c8ec2b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SpotTickReturn(pub SpotTick);
    ///Container type for all return fields from the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `0x0f4b509d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapAMMReturn(pub SwapAMM);
    ///Container type for all return fields from the `transferQuote` function with signature `transferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x1d97d22f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferQuoteReturn(pub TransferQuote);
    ///Container type for all return fields from the `unsignedBurnLp` function with signature `unsignedBurnLp((bytes32,uint32,uint128,uint64))` and selector `0x06c0bafd`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedBurnLpReturn(pub BurnLp);
    ///Container type for all return fields from the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `0x3cec4b93`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedDepositCollateralReturn(pub DepositCollateral);
    ///Container type for all return fields from the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `0x94faefe5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedDepositInsuranceReturn(pub DepositInsurance);
    ///Container type for all return fields from the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `0x05e42dc7`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedLinkSignerReturn(pub LinkSigner);
    ///Container type for all return fields from the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))` and selector `0x9e851424`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedLiquidateSubaccountReturn(pub LiquidateSubaccount);
    ///Container type for all return fields from the `unsignedMintLp` function with signature `unsignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `0x910e606a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedMintLpReturn(pub MintLp);
    ///Container type for all return fields from the `unsignedTransferQuote` function with signature `unsignedTransferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x0edaacce`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedTransferQuoteReturn(pub TransferQuote);
    ///Container type for all return fields from the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `0xbc85ca86`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedWithdrawCollateralReturn(pub WithdrawCollateral);
    ///Container type for all return fields from the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `0x35639a4f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateFeeRatesReturn(pub UpdateFeeRates);
    ///Container type for all return fields from the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `0x1d9eeda5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdatePriceReturn(pub UpdatePrice);
    ///Container type for all return fields from the `updateProduct` function with signature `updateProduct((address,bytes))` and selector `0x2cd71b16`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateProductReturn(pub UpdateProduct);
    ///`Times(uint128,uint128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Times {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub perp_time: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub spot_time: u128,
    }
    ///`Cancellation(bytes32,uint32[],bytes32[],uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Cancellation {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
        pub digests: ::std::vec::Vec<[u8; 32]>,
        pub nonce: u64,
    }
    ///`CancellationProducts(bytes32,uint32[],uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CancellationProducts {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
        pub nonce: u64,
    }
    ///`ChainlinkFullReport(bytes32[3],bytes,bytes32[],bytes32[],bytes32)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ChainlinkFullReport {
        pub report_context: [[u8; 32]; 3],
        pub report_blob: ::ethers::core::types::Bytes,
        pub raw_rs: ::std::vec::Vec<[u8; 32]>,
        pub raw_ss: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub raw_vs: [u8; 32],
    }
    ///`ChainlinkReportBlob(bytes32,uint32,uint32,uint192,uint192,uint64,int192)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ChainlinkReportBlob {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub feed_id: [u8; 32],
        pub valid_from_timestamp: u32,
        pub observations_timestamp: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub native_fee: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub link_fee: ::ethers::core::types::U256,
        pub expires_at: u64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i256",
            deserialize_with = "crate::serialize_utils::deserialize_i256"
        )]
        pub price: ::ethers::core::types::I256,
    }
    ///`SignedCancellation((bytes32,uint32[],bytes32[],uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedCancellation {
        pub cancellation: Cancellation,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedCancellationProducts((bytes32,uint32[],uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedCancellationProducts {
        pub cancellation_products: CancellationProducts,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`BurnLp(bytes32,uint32,uint128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
    ///`BurnLpAndTransfer(bytes32,uint32,uint128,bytes32)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLpAndTransfer {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub recipient: [u8; 32],
    }
    ///`ClaimSequencerFees(bytes32)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ClaimSequencerFees {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///`DepositCollateral(bytes32,uint32,uint128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositCollateral {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///`DepositInsurance(uint128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositInsurance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///`LegacyLiquidateSubaccount(bytes32,bytes32,uint8,uint32,int128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacyLiquidateSubaccount {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        pub mode: u8,
        pub health_group: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub nonce: u64,
    }
    ///`LegacyMatchOrders(uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacyMatchOrders {
        pub product_id: u32,
        pub amm: bool,
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    ///`LegacySignedLiquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacySignedLiquidateSubaccount {
        pub tx: LegacyLiquidateSubaccount,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`LegacySpotTick(uint128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacySpotTick {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub time: u128,
    }
    ///`LinkSigner(bytes32,bytes32,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LinkSigner {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub signer: [u8; 32],
        pub nonce: u64,
    }
    ///`LiquidateSubaccount(bytes32,bytes32,uint32,bool,int128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LiquidateSubaccount {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        pub product_id: u32,
        pub is_encoded_spread: bool,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub nonce: u64,
    }
    ///`ManualAssert(int128[],int128[],int128[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ManualAssert {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub open_interests: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub total_deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub total_borrows: ::std::vec::Vec<i128>,
    }
    ///`MatchOrderAMM(uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MatchOrderAMM {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub base_delta: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_delta: i128,
        pub taker: SignedOrder,
    }
    ///`MatchOrders(uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MatchOrders {
        pub product_id: u32,
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    ///`MintLp(bytes32,uint32,uint128,uint128,uint128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MintLp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount_base: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub quote_amount_low: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub quote_amount_high: u128,
        pub nonce: u64,
    }
    ///`Order(bytes32,int128,int128,uint64,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Order {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
    }
    ///`PerpTick(uint128,int128[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PerpTick {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub time: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub avg_price_diffs: ::std::vec::Vec<i128>,
    }
    ///`RebalanceXWithdraw(uint32,uint128,address)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RebalanceXWithdraw {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///`Rebate(bytes32[],int128[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Rebate {
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub amounts: ::std::vec::Vec<i128>,
    }
    ///`SettlePnl(bytes32[],uint256[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SettlePnl {
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
        pub product_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`SignedBurnLp((bytes32,uint32,uint128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedBurnLp {
        pub tx: BurnLp,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedLinkSigner((bytes32,bytes32,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedLinkSigner {
        pub tx: LinkSigner,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedLiquidateSubaccount {
        pub tx: LiquidateSubaccount,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedMintLp((bytes32,uint32,uint128,uint128,uint128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedMintLp {
        pub tx: MintLp,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedOrder((bytes32,int128,int128,uint64,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedOrder {
        pub order: Order,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedTransferQuote((bytes32,bytes32,uint128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedTransferQuote {
        pub tx: TransferQuote,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedWithdrawCollateral((bytes32,uint32,uint128,uint64),bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SignedWithdrawCollateral {
        pub tx: WithdrawCollateral,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SlowModeConfig(uint64,uint64,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SlowModeConfig {
        pub timeout: u64,
        pub tx_count: u64,
        pub tx_up_to: u64,
    }
    ///`SlowModeTx(uint64,address,bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SlowModeTx {
        pub executable_at: u64,
        pub sender: ::ethers::core::types::Address,
        pub tx: ::ethers::core::types::Bytes,
    }
    ///`SpotTick(uint128,int128[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SpotTick {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub time: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub utilization_ratios_x18: ::std::vec::Vec<i128>,
    }
    ///`SwapAMM(bytes32,uint32,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapAMM {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///`TransferQuote(bytes32,bytes32,uint128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TransferQuote {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub recipient: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
    ///`UpdateFeeRates(address,uint32,int64,int64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateFeeRates {
        pub user: ::ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    ///`UpdatePrice(uint32,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdatePrice {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///`UpdateProduct(address,bytes)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateProduct {
        pub engine: ::ethers::core::types::Address,
        pub tx: ::ethers::core::types::Bytes,
    }
    ///`WithdrawCollateral(bytes32,uint32,uint128,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawCollateral {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
}
