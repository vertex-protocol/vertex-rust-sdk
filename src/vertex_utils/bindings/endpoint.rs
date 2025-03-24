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
                    ::std::borrow::ToOwned::to_owned("assertCode"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertCode",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertCode",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.CreateIsolatedSubaccount",
                                ),
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
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.CreateIsolatedSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("getSequencerFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSequencerFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getSlots"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlots"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("nSubmissionsSlot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("incrementSubmissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("incrementSubmissions",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("rebalanceVlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceVlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.RebalanceVlp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.RebalanceVlp",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("signedBurnVlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedBurnVlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnVlp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnVlp",),
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
                    ::std::borrow::ToOwned::to_owned("signedMintVlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedMintVlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintVlp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintVlp",),
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
                    ::std::borrow::ToOwned::to_owned("unsignedBurnVlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedBurnVlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnVlp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnVlp"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDelistProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDelistProduct",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DelistProduct",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DelistProduct",),
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
                    ::std::borrow::ToOwned::to_owned("unsignedMintVlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedMintVlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintVlp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintVlp"),
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
                    ::std::borrow::ToOwned::to_owned("unsignedWithdrawInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedWithdrawInsurance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawInsurance",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawInsurance",
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
                    ::std::borrow::ToOwned::to_owned("updateMinDepositRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateMinDepositRate",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.UpdateMinDepositRate",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.UpdateMinDepositRate",
                                ),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pa\x95\x14\x80b\0\0\"`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\xF9W`\x005`\xE0\x1C\x80c}\xB6\xA2[\x11a\x03\x19W\x80c\xB1\xC8\xEC+\x11a\x01\xA7W\x80c\xD4\xDE\x8D\x9D\x11a\0\xF9W\x80c\xEERU&\x11a\0\xA2W\x80c\xF8\x0F|\xE5\x11a\0|W\x80c\xF8\x0F|\xE5\x14a\r\x95W\x80c\xFA\xB2\xC4i\x14a\x11DW\x80c\xFB\xF4\x19\x84\x14a\x10\x1FW`\0\x80\xFD[\x80c\xEERU&\x14a\x10\xE6W\x80c\xEFd\xED\x0E\x14a\x11\x08W\x80c\xF2\xFD\xE3\x8B\x14a\x111W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x10mW\x80c\xE9\xABw\xE5\x14a\x10\x80W\x80c\xE9\xBCtb\x14a\x10\xD3W`\0\x80\xFD[\x80c\xD4\xDE\x8D\x9D\x14a\x10\x1FW\x80c\xDBZP!\x14a\x10-W\x80c\xDCB\xE6\x1B\x14a\x10MW`\0\x80\xFD[\x80c\xB7\x0E\xB2c\x11a\x01[W\x80c\xC4\xF9\xB2_\x11a\x015W\x80c\xC4\xF9\xB2_\x14a\x0F\xE7W\x80c\xC5\x105\x9F\x14a\x0F\xF8W\x80c\xD3\x8C;\x9C\x14a\x0F\xFFW`\0\x80\xFD[\x80c\xB7\x0E\xB2c\x14a\x0F\xC1W\x80c\xBC\x85\xCA\x86\x14a\x06'W\x80c\xC3ES\x0B\x14a\x0F\xD4W`\0\x80\xFD[\x80c\xB2\xBBcg\x11a\x01\x8CW\x80c\xB2\xBBcg\x14a\x0FaW\x80c\xB3\x14}\x17\x14a\x0F\x81W\x80c\xB3d\x88\xB8\x14a\x0F\xA1W`\0\x80\xFD[\x80c\xB1\xC8\xEC+\x14a\x0BGW\x80c\xB1\xFB\xD6\x0B\x14a\x0FAW`\0\x80\xFD[\x80c\x91\x0E`j\x11a\x02kW\x80c\x98\xCD2\xFE\x11a\x02\x14W\x80c\xA0\x82\xC5\xAA\x11a\x01\xEEW\x80c\xA0\x82\xC5\xAA\x14a\x0F\x01W\x80c\xA0\xCCc\r\x14a\x0F!W\x80c\xA7\x80\xA4\xBE\x14a\x0F!W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x14a\x0E\x81W\x80c\x9A\x08\xE55\x14a\x0E\x94W\x80c\x9E\x85\x14$\x14a\x0E\xE1W`\0\x80\xFD[\x80c\x94\xFA\xEF\xE5\x11a\x02EW\x80c\x94\xFA\xEF\xE5\x14a\r\x95W\x80c\x954\xDD>\x14a\r\xC0W\x80c\x96\xC4|o\x14a\r\xE0W`\0\x80\xFD[\x80c\x91\x0E`j\x14a\rBW\x80c\x91q\xD0\x8B\x14a\rbW\x80c\x91\xC1\xE3\xD7\x14a\r\x82W`\0\x80\xFD[\x80c\x8CX\xE1\n\x11a\x02\xCDW\x80c\x8D\xA5\xCB[\x11a\x02\xA7W\x80c\x8D\xA5\xCB[\x14a\r\rW\x80c\x8E]X\x8C\x14a\r\x1EW\x80c\x8FO\x8E\xCC\x14a\r1W`\0\x80\xFD[\x80c\x8CX\xE1\n\x14a\x0C\x96W\x80c\x8D\n\xCC\x9B\x14a\x0C\xDAW\x80c\x8D< \xB1\x14a\x0C\xEDW`\0\x80\xFD[\x80c\x85\xC8>\x9D\x11a\x02\xFEW\x80c\x85\xC8>\x9D\x14a\x0CFW\x80c\x872C8\x14a\x0CfW\x80c\x8C=/t\x14a\x0CyW`\0\x80\xFD[\x80c}\xB6\xA2[\x14a\x0C3W\x80c\x82fD\xF7\x14a\x0C\x13W`\0\x80\xFD[\x80c/\x9A'D\x11a\x04\x96W\x80cTDV\x9D\x11a\x03\xE8W\x80ce\xDD\x13f\x11a\x03\x91W\x80co;\nr\x11a\x03kW\x80co;\nr\x14a\x0B\xEBW\x80cqP\x18\xA6\x14a\x0C\x0BW\x80cy\xF1$3\x14a\x0C\x13W`\0\x80\xFD[\x80ce\xDD\x13f\x14a\x0B\x90W\x80clEup\x14a\x0B\x98W\x80cl\xFE_\xE4\x14a\x0B\xB8W`\0\x80\xFD[\x80c[\xB4\xC1&\x11a\x03\xC2W\x80c[\xB4\xC1&\x14a\x0BgW\x80c]O_\x97\x14a\x0B}W\x80ca\x0B.^\x14a\x06\x9CW`\0\x80\xFD[\x80cTDV\x9D\x14a\x0B\x14W\x80cU~\xD1\xBA\x14a\x0B'W\x80cZ\0\x92;\x14a\x0BGW`\0\x80\xFD[\x80c8B\xE7^\x11a\x04JW\x80cB\xC7M\x1D\x11a\x04$W\x80cB\xC7M\x1D\x14a\n\xA6W\x80cM\x96\xA9\n\x14a\n\xC6W\x80cO\xCF\xAEX\x14a\n\xEBW`\0\x80\xFD[\x80c8B\xE7^\x14a\n W\x80c<\xECK\x93\x14a\nBW\x80c>\xDF,[\x14a\n\x86W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04{W\x80c5c\x9AO\x14a\tzW\x80c6\x8EF\x86\x14a\t\xDAW\x80c6\xB9\x0CQ\x14a\n\0W`\0\x80\xFD[\x80c/\x9A'D\x14a\t\x06W\x80c2\x16\xC0b\x14a\t\x19W`\0\x80\xFD[\x80c\x1D\x97\xD2/\x11a\x05OW\x80c\"\x1F\t9\x11a\x05\x03W\x80c,\x8Co\xFB\x11a\x04\xDDW\x80c,\x8Co\xFB\x14a\x08\x94W\x80c,\xD7\x1B\x16\x14a\x08\xB4W\x80c-\x035\xAB\x14a\x08\xD4W`\0\x80\xFD[\x80c\"\x1F\t9\x14a\x08XW\x80c\"\xD4\xA8-\x14a\x08kW\x80c'ay\x97\x14a\x08\x13W`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x054W\x80c\x1F\x18k'\x14a\x083W\x80c!\x04u\x89\x14a\x08HW\x80c\"\0`F\x14a\x08PW`\0\x80\xFD[\x80c\x1D\x97\xD2/\x14a\x06\xDCW\x80c\x1D\x9E\xED\xA5\x14a\x08\x13W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05\xB1W\x80c\x15\xD3\"\xE7\x11a\x05\x8BW\x80c\x15\xD3\"\xE7\x14a\x07\x93W\x80c\x16\xCD\xB6\x90\x14a\x07\xC8W\x80c\x18\xED\x16\xEB\x14a\x07\xE8W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06\xDCW\x80c\x0FKP\x9D\x14a\x06\xFCW\x80c\x14sWU\x14a\x07SW`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05\xE2W\x80c\x07H\xA2\x19\x14a\x06GW\x80c\rU\xE2k\x14a\x06\x9CW\x80c\x0Ef&[\x14a\x06\xBCW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\xFEW\x80c\x06\xC0\xBA\xFD\x14a\x06'W[`\0\x80\xFD[a\x06\x11a\x06\x0C6`\x04a`\xA1V[a\x11KV[`@Qa\x06\x1E\x91\x90a`\xBDV[`@Q\x80\x91\x03\x90\xF3[a\x06:a\x0656`\x04a`\xF9V[a\x11}V[`@Qa\x06\x1E\x91\x90aa\x15V[a\x06Za\x06U6`\x04a`\xF9V[a\x11\xB0V[`@Qa\x06\x1E\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06\xAFa\x06\xAA6`\x04aajV[a\x11\xE3V[`@Qa\x06\x1E\x91\x90ab]V[a\x06\xCFa\x06\xCA6`\x04ab\x82V[a\x11\xF4V[`@Qa\x06\x1E\x91\x90ab\xF1V[a\x06\xEFa\x06\xEA6`\x04a`\xF9V[a\x12\x11V[`@Qa\x06\x1E\x91\x90acyV[a\x07\x0Fa\x07\n6`\x04a`\xF9V[a\x12DV[`@Qa\x06\x1E\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x07fa\x07a6`\x04ac\xB6V[a\x12wV[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x06\x1EV[a\x07\xA6a\x07\xA16`\x04ac\xB6V[a\x12\x9AV[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x93\x84\x01Q\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x06\x1EV[a\x07\xDBa\x07\xD66`\x04ac\xD2V[a\x12\xBDV[`@Qa\x06\x1E\x91\x90ad\x92V[`\xA6Ta\x07\xFB\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x08&a\x08!6`\x04ac\xB6V[a\x13\x10V[`@Qa\x06\x1E\x91\x90ad\xA5V[a\x08Fa\x08A6`\x04ae\x10V[a\x133V[\0[a\x08Fa\x13\xCFV[a\x07\xFBa\x14\x7FV[a\x08Fa\x08f6`\x04ag\x11V[a\x14\xC4V[a\x07\xFBa\x08y6`\x04agzV[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08\xA7a\x08\xA26`\x04ag\x93V[a\x18SV[`@Qa\x06\x1E\x91\x90ag\xFAV[a\x08\xC7a\x08\xC26`\x04ab\x82V[a\x18\x80V[`@Qa\x06\x1E\x91\x90ahRV[a\x07\xFBa\x08\xE26`\x04ah\x95V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08Fa\t\x146`\x04ah\xC9V[a\x18\x9EV[a\x08Fa\t'6`\x04ai\"V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\t\x8Da\t\x886`\x04a`\xF9V[a\x19\xF0V[`@Qa\x06\x1E\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\t\xEDa\t\xE86`\x04aipV[a\x1A#V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x06\x1EV[a\n\x13a\n\x0E6`\x04ai\x8DV[a\x1A~V[`@Qa\x06\x1E\x91\x90aj_V[a\n3a\n.6`\x04aj\x84V[a\x1A\x8FV[`@Q\x90Q\x81R` \x01a\x06\x1EV[a\nUa\nP6`\x04a`\xA1V[a\x1A\xADV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x06\x1EV[a\n\x99a\n\x946`\x04ab\x82V[a\x1A\xD9V[`@Qa\x06\x1E\x91\x90aj\xD6V[a\n\xB9a\n\xB46`\x04ab\x82V[a\x1A\xEAV[`@Qa\x06\x1E\x91\x90akTV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\t\xEDa\n\xF96`\x04aipV[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xAA` R`@\x90 T`\x0F\x0B\x90V[a\x08Fa\x0B\"6`\x04al3V[a\x1B\x07V[a\x0B/a\x1FAV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x0BZa\x0BU6`\x04ab\x82V[a\x1F\xCFV[`@Qa\x06\x1E\x91\x90al\xF3V[a\x0Boa\x1F\xEDV[`@Q\x90\x81R` \x01a\x06\x1EV[`\x9ATa\n\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08Fa!\xE0V[a\x0B\xABa\x0B\xA66`\x04am\x18V[a\"#V[`@Qa\x06\x1E\x91\x90amLV[a\x08Fa\x0B\xC66`\x04amoV[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x0B\xFEa\x0B\xF96`\x04aajV[a\"4V[`@Qa\x06\x1E\x91\x90am\xA4V[a\x08Fa\"EV[a\x0C&a\x0C!6`\x04a`\xA1V[a\"YV[`@Qa\x06\x1E\x91\x90an\x03V[a\x08Fa\x0CA6`\x04anGV[a\"\x85V[a\x0CYa\x0CT6`\x04ai\x8DV[a$WV[`@Qa\x06\x1E\x91\x90an\xBCV[a\x08Fa\x0Ct6`\x04aoFV[a$hV[a\x0C\x81a.\xBAV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x08Fa\x0C\xA46`\x04ao\x9AV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x08Fa\x0C\xE86`\x04ao\xD3V[a0\xA6V[a\r\0a\x0C\xFB6`\x04ag\x93V[a1\xD5V[`@Qa\x06\x1E\x91\x90apFV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\n\xD3V[a\x08Fa\r,6`\x04apqV[a1\xE6V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\n\xD3V[a\rUa\rP6`\x04ap\xB6V[a2VV[`@Qa\x06\x1E\x91\x90ap\xD2V[a\rua\rp6`\x04aqGV[a2\x97V[`@Qa\x06\x1E\x91\x90aq{V[a\n\xD3a\r\x906`\x04agzV[a2\xA8V[a\r\xA8a\r\xA36`\x04aj\x84V[a3iV[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x06\x1EV[a\r\xD3a\r\xCE6`\x04ah\x95V[a3\x87V[`@Qa\x06\x1E\x91\x90aq\xF1V[a\r\xF3a\r\xEE6`\x04ar\x04V[a4!V[`@Qa\x06\x1E\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x08Fa\x0E\x8F6`\x04ar V[a4iV[a\x0E\xA7a\x0E\xA26`\x04a`\xA1V[a5\xD1V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x06\x1EV[a\x0E\xF4a\x0E\xEF6`\x04ap\xB6V[a5\xFDV[`@Qa\x06\x1E\x91\x90ar\xC3V[a\x0F\x14a\x0F\x0F6`\x04ab\x82V[a6>V[`@Qa\x06\x1E\x91\x90as\x1AV[a\x0F4a\x0F/6`\x04aajV[a6OV[`@Qa\x06\x1E\x91\x90as\xE0V[a\x0FTa\x0FO6`\x04aqGV[a6\x88V[`@Qa\x06\x1E\x91\x90as\xF3V[a\x0Fta\x0Fo6`\x04ab\x82V[a6\x99V[`@Qa\x06\x1E\x91\x90atOV[a\x0F\x94a\x0F\x8F6`\x04ag\x93V[a6\xB6V[`@Qa\x06\x1E\x91\x90at\xACV[a\x0F\xB4a\x0F\xAF6`\x04ai\x8DV[a6\xDBV[`@Qa\x06\x1E\x91\x90at\xE4V[a\x0Boa\x0F\xCF6`\x04aoFV[a6\xECV[a\x0C\x81a\x0F\xE26`\x04aoFV[a7\x89V[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x07\xFBV[`\0a\t\xEDV[a\x10\x12a\x10\r6`\x04aqGV[a8iV[`@Qa\x06\x1E\x91\x90au9V[g\r\xE0\xB6\xB3\xA7d\0\0a\t\xEDV[a\x10@a\x10;6`\x04aqGV[a8zV[`@Qa\x06\x1E\x91\x90au\xA0V[a\x10`a\x10[6`\x04ap\xB6V[a8\x8BV[`@Qa\x06\x1E\x91\x90av=V[a\x08Fa\x10{6`\x04ao\xD3V[a8\xCCV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x06\x1EV[a\x08Fa\x10\xE16`\x04av\x95V[a<\xEFV[a\x10\xF9a\x10\xF46`\x04aw\x0CV[a=}V[`@Qa\x06\x1E\x93\x92\x91\x90aw'V[a\x0Boa\x11\x166`\x04aw\x0CV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x08Fa\x11?6`\x04ah\x95V[a>\x98V[`\xA6a\x0BoV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83aw\xC4V[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83ax;V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83axWV[a\x11\xEBa]\x91V[a\x11w\x82ay\x06V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82aymV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83az\x7FV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83az\x9BV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83az\xF9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83a{8V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a|EV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83a|\x93V[`\0[\x81\x81\x10\x15a\x13\x88W6`\0\x84\x84\x84\x81\x81\x10a\x13SWa\x13Sa|\xAFV[\x90P` \x02\x81\x01\x90a\x13e\x91\x90a|\xC5V[\x91P\x91Pa\x13s\x82\x82a?%V[PP\x80\x80a\x13\x80\x90a}!V[\x91PPa\x136V[P`\xA6\x80T\x82\x91\x90`\0\x90a\x13\xA7\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a}:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\t'W`\xA8`\0\x82` \x01\x80Qa\x14=\x90a}eV[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x14x`\x01\x83\x01\x82a]\xCAV[PPa\x14\x08V[`\xA6\x80T`\0\x91\x90\x82\x90a\x14\x9B\x90`\x01`\x01`@\x1B\x03\x16a}\x88V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA6T\x16\x91\x90PV[\x80Q`\0\x03a\x14\xD2W`\0\x80\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x15\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`@Q\x80\x91\x03\x90\xFD[P``\x84\x90\x1Ca\x15R\x813\x81\x14a\x15LW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaR\x04V[\x83aR\x04V[`\x01\x85\x14\x80\x15\x90a\x15xWP`\0\x85\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x15\xF0W`\x9AT`@Qc\xD6\x93\xC5\xF1`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD6\x93\xC5\xF1\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xEBW=`\0\x80>=`\0\xFD[PPPP[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ra\x16t\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16d\x91\x90a}\xAEV[3\x85`\x01`\x01`\x80\x1B\x03\x16aRYV[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x16\xC1Bb\x03\xF4\x80a}:V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x17E\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17c\x92\x91` \x01a}\xE1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a\x17\x8B\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x17\xF7\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPV[a\x18w`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x11w\x82a~\x10V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11w\x82a~\xD7V[`\0Z\x90Pa\x18\xAC\x85aR\x82V[`\0[\x83\x81\x10\x15a\x19wW6`\0\x86\x86\x84\x81\x81\x10a\x18\xCCWa\x18\xCCa|\xAFV[\x90P` \x02\x81\x01\x90a\x18\xDE\x91\x90a|\xC5V[\x91P\x91Pa\x18\xEC\x82\x82a?%V[\x84Za\x18\xF8\x90\x86a~\xE3V[\x11\x15a\x19bW`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19IW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19]W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x19o\x90a}!V[\x91PPa\x18\xAFV[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x19\x95\x90\x85a~\xE3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xD1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19\xE5W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x7F\x0CV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1AxW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x91\x90PV[a\x1A\x86a^\x88V[a\x11w\x82a\x80~V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11w6\x83\x90\x03\x83\x01\x83a\x80\x8AV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x80\xCBV[a\x1A\xE1a^\xB6V[a\x11w\x82a\x81lV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82a\x82:V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1B'WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1BAWP0;\x15\x80\x15a\x1BAWP`\0T`\xFF\x16`\x01\x14[a\x1B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x12V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B\xD6W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1B\xDEaR\xCAV[a\x1CR`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaS=V[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1C\xCD\x90`\0\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x0E\x91\x90a}\xAEV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1DR\x90`\x01\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x93\x91\x90a}\xAEV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x15\x91\x90a}\xAEV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\xF1W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\xA0Wa\x1E\xA0a|\xAFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1E\xE9\x81a\x82\xC2V[\x91PPa\x1EyV[P\x80\x15a\x1F8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F~W\x81Qa\x1F\x84V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1F\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11w\x82a\x83'V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a \xE0\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x0C\x90a\x833V[\x80\x15a!YW\x80`\x1F\x10a!.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!YV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!<W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\x93\x92\x90\x91`\x04\x01a\x83gV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a!\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra!\xC9\x91\x81\x01\x90a\x83\x89V[`\x01[a!\xD9WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\t'\x81`\0aS\xB2V[a\"+a^\xE5V[a\x11w\x82a\x83\xA2V[a\"<a_\x1CV[a\x11w\x82a\x83\xD4V[a\"MaVQV[a\"W`\0aV\xABV[V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x84\x1DV[a\"\x8E\x86aR\x82V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA5W`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a#?W\x81\x87\x87\x83\x81\x81\x10a\"\xF2Wa\"\xF2a|\xAFV[\x90P` \x02\x81\x01\x90a#\x04\x91\x90a|\xC5V[`@Q` \x01a#\x16\x93\x92\x91\x90a\x849V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a#8\x90a}!V[\x90Pa\"\xD7V[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xB1W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a$MW6`\0\x88\x88\x84\x81\x81\x10a#\xD5Wa#\xD5a|\xAFV[\x90P` \x02\x81\x01\x90a#\xE7\x91\x90a|\xC5V[\x91P\x91Pa#\xF5\x82\x82a?%V[`\xA6\x80T`\x01\x91\x90`\0\x90a$\x14\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a}:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a$E\x90a}!V[\x91PPa#\xB8V[PPPPPPPPV[a$_a_LV[a\x11w\x82a\x84\x94V[30\x14a$tW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a$\x89Wa$\x89a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a$\xA4Wa$\xA4a}\xCBV[\x90P`\x01\x81` \x81\x11\x15a$\xBAWa$\xBAa}\xCBV[\x03a%~W`\0a$\xCE\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a$\xDB\x91\x90a\x80\xCBV[\x90Pa$\xEB\x81`\0\x01Q\x86aV\xFDV[\x80Qa$\xF6\x90aW[V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%`W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%tW=`\0\x80>=`\0\xFD[PPPPPa.\xB4V[`\x02\x81` \x81\x11\x15a%\x92Wa%\x92a}\xCBV[\x03a&6W`\0a%\xA6\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a%\xB3\x91\x90ax;V[\x90Pa%\xC3\x81`\0\x01Q\x86aV\xFDV[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA6T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a%FV[`\x07\x81` \x81\x11\x15a&JWa&Ja}\xCBV[\x03a&\xB8W`\x9AT`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a&\x81\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&\xAFW=`\0\x80>=`\0\xFD[PPPPa.\xB4V[`\t\x81` \x81\x11\x15a&\xCCWa&\xCCa}\xCBV[\x03a(\\W`\0a&\xE0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a&\xED\x91\x90a\x85\x84V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a'\x1F\x90`\0\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'`\x91\x90a}\xAEV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xDC\x91\x90a}\xAEV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a(\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x80Qa(,\x90\x86aV\xFDV[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a%F\x90\x84\x90`\x04\x01ap\xD2V[`\n\x81` \x81\x11\x15a(pWa(pa}\xCBV[\x03a(\xD1W`\0a(\x84\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a(\x91\x91\x90ax;V[\x90Pa(\xA1\x81`\0\x01Q\x86aV\xFDV[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a%F\x90\x84\x90`\x04\x01aa\x15V[`\x0B\x81` \x81\x11\x15a(\xE5Wa(\xE5a}\xCBV[\x03a)}W`\0a(\xF9\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a)\x06\x91\x90az\x9BV[\x90Pa)\x16\x81`\0\x01Q\x86aV\xFDV[\x80Qa)!\x90aW\xEAV[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a%FV[`\x12\x81` \x81\x11\x15a)\x91Wa)\x91a}\xCBV[\x03a)\xE7W`\0a)\xA5\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a)\xB2\x91\x90a\x85\xA0V[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a%F\x91`\x04\x01aq\xF1V[`\x13\x81` \x81\x11\x15a)\xFBWa)\xFBa}\xCBV[\x03a*jW`\0a*\x0F\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a*\x1C\x91\x90aw\xC4V[\x90Pa*,\x81`\0\x01Q\x86aV\xFDV[\x80Qa*7\x90aW\xEAV[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua.\xB4V[`\x15\x81` \x81\x11\x15a*~Wa*~a}\xCBV[\x03a+\x1AW`\0a*\x92\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a*\x9F\x91\x90axWV[\x90Pa*\xAF\x81`\0\x01Q\x86aV\xFDV[a*\xBC\x81``\x01QaW[V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a%FV[`\x1B\x81` \x81\x11\x15a+.Wa+.a}\xCBV[\x03a+tW`\x9AT`\xA6T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a&\x81\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x85\xD4V[`\x1D\x81` \x81\x11\x15a+\x88Wa+\x88a}\xCBV[\x03a+\xBFW`\x9AT`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a&\x81\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\r\x81` \x81\x11\x15a+\xD3Wa+\xD3a}\xCBV[\x03a.ZW`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,<W=`\0\x80>=`\0\xFD[PPPP`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xBD\x91\x90\x81\x01\x90a\x86\x01V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xDAWa,\xDAae\x8AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a-\xE6W`\xAA`\0\x84\x83\x81Q\x81\x10a-(Wa-(a|\xAFV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a-jWa-ja|\xAFV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10a-\x96Wa-\x96a|\xAFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a-\xDE\x81a}!V[\x91PPa-\tV[Pa-\xF1`\x01aW\xEAV[`\x9AT`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a.!\x90\x84\x90`\x04\x01a\x86\x9AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.;W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.OW=`\0\x80>=`\0\xFD[PPPPPPa.\xB4V[`\x18\x81` \x81\x11\x15a.nWa.na}\xCBV[\x03a\x05\xF9W`\x9AT`\xA6T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a&\x81\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x85\xD4V[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a/OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a/\xAD\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xD9\x90a\x833V[\x80\x15a0&W\x80`\x1F\x10a/\xFBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0&V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a0`\x92\x90\x91`\x04\x01a\x83gV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a0\x99WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra0\x96\x91\x81\x01\x90a\x86\xADV[`\x01[a!\xD9W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a0\xBBWa0\xBBa|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a0\xD6Wa0\xD6a}\xCBV[\x90P`\0\x81` \x81\x11\x15a0\xECWa0\xECa}\xCBV[\x03a1>W`\0a1\0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a1\r\x91\x90a\x87gV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa1#\x91aXHV[\x80QQa18\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[Pa1\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a1\xAB\x83a}\x88V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a1\xDDa_uV[a\x11w\x82a\x88\x10V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra2Q\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra2/\x90a\x88\x1CV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x14\xC4V[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x85\x84V[a2\x9Fa_\xA2V[a\x11w\x82a\x88@V[`\0b\xFF\xFF\xFF\x82\x16biso\x14a2\xD6W`\0\x82\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x11wV[`\xB0T`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xAB\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3I\x91\x90a\x83\x89V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11w6\x83\x90\x03\x83\x01\x83a\x88\x97V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta3\xA0\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3\xCC\x90a\x833V[\x80\x15a4\x19W\x80`\x1F\x10a3\xEEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\x19V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x88\xDBV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a4\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x12V[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a5\x0C\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a5x\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x89rV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x89\xB8V[a6Fa_\xE1V[a\x11w\x82a\x89\xD4V[`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a\x8A\xD2V[a6\x90a_\xA2V[a\x11w\x82a\x8B'V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82a\x8BrV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a\x8C5V[a6\xE3a`\tV[a\x11w\x82a\x8C\x94V[`\0\x80\x83\x83`\0\x81\x81\x10a7\x02Wa7\x02a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\x1DWa7\x1Da}\xCBV[\x90P`\x13\x81` \x81\x11\x15a73Wa73a}\xCBV[\x03a7~W`\0a7G\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7T\x91\x90aw\xC4V[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a7rW`\0a7uV[\x80Q[\x92PPPa!\xD9V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a7\x9FWa7\x9Fa|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\xBAWa7\xBAa}\xCBV[\x90P`\t\x81` \x81\x11\x15a7\xD0Wa7\xD0a}\xCBV[\x03a7\xFFW`\0a7\xE4\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90a\x85\x84V[` \x01Q\x92Pa!\xD9\x91PPV[`\n\x81` \x81\x11\x15a8\x13Wa8\x13a}\xCBV[\x03a84W`\0a8'\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90ax;V[`\x0B\x81` \x81\x11\x15a8HWa8Ha}\xCBV[\x03a7~W`\0a8\\\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90az\x9BV[a8qa_\xA2V[a\x11w\x82a\x8D>V[a8\x82a`(V[a\x11w\x82a\x8D\x9AV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x8EGV[`\0\x82\x82`\0\x81\x81\x10a8\xE1Wa8\xE1a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a8\xFCWa8\xFCa}\xCBV[\x90P3`\x01\x82` \x81\x11\x15a9\x13Wa9\x13a}\xCBV[\x03a9\x1DW`\0\x80\xFD[`\x07\x82` \x81\x11\x15a91Wa91a}\xCBV[\x03a9wW`\0a9E\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a9R\x91\x90a\x88\x97V[\x90Pa9qa9_aY\x04V[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aRYV[Pa;rV[`\x12\x82` \x81\x11\x15a9\x8BWa9\x8Ba}\xCBV[\x03a9\xBFW`gT`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a9\xBAW`\0\x80\xFD[a;rV[`\x15\x82` \x81\x11\x15a9\xD3Wa9\xD3a}\xCBV[\x03a:yW`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a:/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P`\0a:?\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a:L\x91\x90axWV[\x90Pa9q\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaR\x04V[`\x1B\x82` \x81\x11\x15a:\x8DWa:\x8Da}\xCBV[\x03a:\xA3W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\x1D\x82` \x81\x11\x15a:\xB7Wa:\xB7a}\xCBV[\x03a:\xCDW`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\r\x82` \x81\x11\x15a:\xE1Wa:\xE1a}\xCBV[\x03a:\xF7W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\x18\x82` \x81\x11\x15a;\x0BWa;\x0Ba}\xCBV[\x03a;!W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[a;/a;,aY\x04V[PV[`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a;J\x90\x84\x90`\x0F\x0Ba\x8EcV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a;\xBFBb\x03\xF4\x80a}:V[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA8\x93P\x90a<(\x82a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a<\x94\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra=v\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra=8\x90a\x88\x1CV[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14\xC4\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a>\x07\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta>3\x90a\x833V[\x80\x15a>\x80W\x80`\x1F\x10a>UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a>\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a>cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a>\xA0aVQV[`\x01`\x01`\xA0\x1B\x03\x81\x16a?\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x12V[a;,\x81aV\xABV[`\0\x82\x82`\0\x81\x81\x10a?:Wa?:a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a?UWa?Ua}\xCBV[\x90P`\0\x81` \x81\x11\x15a?kWa?ka}\xCBV[\x03a@\xD8W`\0a?\x7F\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a?\x8C\x91\x90a\x87gV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa?\xA2\x91aXHV[a@Y\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a\x93\x80`w\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aYwV[PPV[\x80QQa@e\x90aW\xEAV[\x80QQa@z\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a@\xAA\x91`\x04\x01ar\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$MW=`\0\x80>=`\0\xFD[`\x02\x81` \x81\x11\x15a@\xECWa@\xECa}\xCBV[\x03aB\xA7W`\0aA\0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aA\r\x91\x90a\x8E\xC4V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaA#\x91aXHV[aA\xA8\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a\x94\x90`O\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01RaB2\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB'\x91\x90a\x8E\xF8V[\x83Q` \x01QaY\xC5V[`\x9AT\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA6T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a@\xAAV[`\x03\x81` \x81\x11\x15aB\xBBWaB\xBBa}\xCBV[\x03aC\xBFW`\0aB\xCF\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aB\xDC\x91\x90a\x8F\x15V[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15aC(W` \x82\x01Q\x83QaC#\x91\x90a\x8FIV[aC+V[`\0[`\x9BT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCzW`\0\x80\xFD[PZ\xF1\x15\x80\x15aC\x8EW=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa2Q\x91PPV[`\x0F\x81` \x81\x11\x15aC\xD3WaC\xD3a}\xCBV[\x03aD\xD4W`\0aC\xE7\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aC\xF4\x91\x90a\x8F\x15V[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15aD<W\x81Q\x83QaD7\x91\x90a\x8FIV[aD?V[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91aDx\x91\x85\x91\x90`\x04\x01a\x8FqV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15aD\xA6W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa2Q\x91PPV[`\x04\x81` \x81\x11\x15aD\xE8WaD\xE8a}\xCBV[\x03aE\xADW`\x9AT`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90aE$\x90\x88\x90\x88\x90`\x04\x01a\x84\xF3V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEf\x91\x90a\x8F\x93V[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a=vWc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90UPPPPPV[`\x05\x81` \x81\x11\x15aE\xC1WaE\xC1a}\xCBV[\x03aF&W`\x9AT`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F8W=`\0\x80>=`\0\xFD[`\x06\x81` \x81\x11\x15aF:WaF:a}\xCBV[\x14\x80aFWWP`\x16\x81` \x81\x11\x15aFUWaFUa}\xCBV[\x14[\x15aGKW`\0aFk\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aFx\x91\x90a\x8F\xC2V[` \x81\x01QQQ\x90\x91PaF\x8B\x90aW\xEAV[`@\x81\x01QQQaF\x9B\x90aW\xEAV[`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01aF\xC3\x84` \x01Q`\0\x01Q`\0\x01Qa2\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aF\xE6\x84`@\x01Q`\0\x01Q`\0\x01Qa2\xA8V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R`\xB0T`@Qc\x11\x17\x8F-`\xE3\x1B\x81R\x92\x93P\x16\x90c\x88\xBCyh\x90aG\x1D\x90\x84\x90`\x04\x01a\x8F\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xE5W=`\0\x80>=`\0\xFD[`\x0C\x81` \x81\x11\x15aG_WaG_a}\xCBV[\x03aG\xD7W`\0aGs\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aG\x80\x91\x90a\x90AV[``\x81\x01QQQ\x90\x91PaG\x93\x90aW\xEAV[`\xB0T``\x82\x01QQQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xF0\xD3\xCE\x90\x83\x90aG\xBA\x90a2\xA8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xAA\x92\x91\x90a\x90uV[`\x08\x81` \x81\x11\x15aG\xEBWaG\xEBa}\xCBV[\x03aH\x89W`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaH3\x81`\x01aS\xB2V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81` \x81\x11\x15aH\x9DWaH\x9Da}\xCBV[\x03aI\xBEW`\0aH\xB1\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aH\xBE\x91\x90a\x90\xA0V[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaH\xD4\x91aXHV[aIy\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\xA0\x01`@R\x80`v\x81R` \x01a\x94\x1A`v\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93R\x90\x82\x16\x92\x85\x01\x92\x90\x92R\x16`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01a@:V[\x80QQaI\x8E\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91a@\xAA\x91`\x04\x01ap\xD2V[`\n\x81` \x81\x11\x15aI\xD2WaI\xD2a}\xCBV[\x03aJ\xD3W`\0aI\xE6\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aI\xF3\x91\x90a\x8E\xC4V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaJ\t\x91aXHV[aJ\x8E\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`C\x81R` \x01a\x93=`C\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80QQaJ\xA3\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91a@\xAA\x91`\x04\x01aa\x15V[`\x1E\x81` \x81\x11\x15aJ\xE7WaJ\xE7a}\xCBV[\x03aLIW`\0aJ\xFB\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aK\x08\x91\x90a\x90\xD4V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaK\x1E\x91aXHV[aK\x8D\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`8\x81R` \x01a\x92\x99`8\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[\x80QQaK\xA2\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x88<q\x85`\xE0\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88<q\x85\x90`\x84\x01a@\xAAV[`\x1F\x81` \x81\x11\x15aL]WaL]a}\xCBV[\x03aM\xBFW`\0aLq\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aL~\x91\x90a\x90\xD4V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaL\x94\x91aXHV[aM\x03\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x92\xD1`6\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[\x80QQaM\x18\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x1C\xD4\x0F_`\xE3\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\xA0z\xF8\x90`\x84\x01a@\xAAV[`\x10\x81` \x81\x11\x15aM\xD3WaM\xD3a}\xCBV[\x03aN\nW`\x9AT`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x13\x81` \x81\x11\x15aN\x1EWaN\x1Ea}\xCBV[\x03aN\xEFW`\0aN2\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aN?\x91\x90a\x91\x08V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaNU\x91aXHV[aN\xBB\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x93\x07`6\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81` \x81\x11\x15aO\x03WaO\x03a}\xCBV[\x03aO:W`\x9AT`@Qc&fm-`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99\x99\xB4\xB4\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x17\x81` \x81\x11\x15aONWaONa}\xCBV[\x03aPZW`\0aOb\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aOo\x91\x90a\x91<V[\x90PaO\x82\x81`\0\x01Q` \x01QaW[V[aP\x01\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a\x92N`K\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80Q\x80Q``\x90\x91\x01QaP\x15\x91\x90aXHV[\x80QQaP*\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a@\xAA\x91`\x04\x01acyV[`\x19\x81` \x81\x11\x15aPnWaPna}\xCBV[\x03aP\xA5W`\x9AT`@QcJg\xD9\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\xCF\xB2\x02\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x1A\x81` \x81\x11\x15aP\xB9WaP\xB9a}\xCBV[\x03aP\xF0W`\x9AT`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x1C\x81` \x81\x11\x15aQ\x04WaQ\x04a}\xCBV[\x03aQ\xB9W`\0aQ\x18\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aQ%\x91\x90a\x91pV[`\xB0T\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA2z%\n\x90\x84\x90aQN\x90a2\xA8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQk\x92\x91\x90a\x91\xA4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aQ\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xAE\x91\x90a\x83\x89V[\x90Pa=v\x81aW[V[` \x81` \x81\x11\x15aQ\xCDWaQ\xCDa}\xCBV[\x03a\x05\xF9W`\x9AT`@Qc~\x92v\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c~\x92v\xD7\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaR'\x90a\x833V[\x90P`\0\x03a@UW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa2Q\x92\x84\x01\x90a^\x04V[`\x01`\x01`\xA0\x1B\x03\x83\x16aRlW`\0\x80\xFD[`\x9ATa2Q\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14a@UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`\0Ta\x01\0\x90\x04`\xFF\x16aS5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a\"WaZ\x9CV[`\0Ta\x01\0\x90\x04`\xFF\x16aS\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a@U\x82\x82a[\x10V[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x93\xF7`#\x919\x90aT\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aTe\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaT\x91\x90a\x833V[\x80\x15aT\xDEW\x80`\x1F\x10aT\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aT\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aT\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aT\xFF\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aU<`\x01\x83\x01\x82a]\xCAV[PP\x81\x80aUWWPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aU\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[PFazi\x03aU\xC7W` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aE\xF8\x92`\x04\x01a\x83gV[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aU\xFA\x92\x90\x91`\x04\x01a\x83gV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aV\x14W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aV%WP`\x01[a.\xB4Wb\x03\xD0\x90Z\x11\x15\x80aVEWPaVA`\x02\x82a\x91\xB7V[Z\x11\x15[\x15aVLW\xFE[a.\xB4V[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x15\x12V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aW WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a2QW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a;,W`\xA2\x80T`\0\x90aW\x92\x90`\x01`\x01`@\x1B\x03\x16a}\x88V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[`\x01\x81\x14\x80aX\x0FWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90a@UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aXq\x83a}\x88V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a2QWaX\xC2\x81`\x01`\x01`@\x1B\x03\x16a[\x95V[`@Q` \x01aX\xD2\x91\x90a\x91\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x15\x12\x91`\x04\x01aq\xF1V[a@U\x82\x82`\0aY\xC5V[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aYr\x91\x90a}\xAEV[\x90P\x90V[`\0a\x11waY\x84a\\4V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aY\xE1\x86a\x92\x1EV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aZ0W`\0\x80\xFD[PZ\xF1\x15\x80\x15aZDW=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aZp\x90\x84\x90`\x0F\x0Ba\x8EcV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a[\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a\"W3aV\xABV[`\0Ta\x01\0\x90\x04`\xFF\x16a[{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0a[\xA2\x83a\\\xAFV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a[\xC1Wa[\xC1ae\x8AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a[\xEBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a[\xF5WP\x93\x92PPPV[`\0aYr\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\\c`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\\\xF8Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a]$Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a]BWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a]ZWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a]nWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a]\x80W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11wW`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80Ta]\xD6\x90a\x833V[`\0\x82U\x80`\x1F\x10a]\xE6WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a;,\x91\x90a`\\V[\x82\x80Ta^\x10\x90a\x833V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a^2W`\0\x85Ua^xV[\x82`\x1F\x10a^KW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua^xV[\x82\x80\x01`\x01\x01\x85U\x82\x15a^xW\x91\x82\x01[\x82\x81\x11\x15a^xW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a^]V[Pa^\x84\x92\x91Pa`\\V[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01a^\xB1a^\xE5V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a_\x95a^\xE5V[\x81R` \x01a^\xB1a^\xE5V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01a_\x95a^\xE5V[`@Q\x80`\xA0\x01`@R\x80a`;a`qV[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15a^\x84W`\0\x81U`\x01\x01a`]V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a`\xB3W`\0\x80\xFD[a!\xD9\x83\x83a`\x8FV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11wV[`\0`\x80\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aa\x0BW`\0\x80\xFD[a!\xD9\x83\x83a`\xE7V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11wV[`\0`\xA0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aa|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aa\x92W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01aaXV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15aa\xC1W\x81\x81\x01Q\x83\x82\x01R` \x01aa\xA9V[\x83\x81\x11\x15a.\xB4WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Raa\xEA\x81` \x86\x01` \x86\x01aa\xA6V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[abB\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01Raa\x9E`\xA0\x85\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84aa\xFEV[`\0`@\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ab\x94W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ab\xAAW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01abpV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ab\xCAV[P\x94\x95\x94PPPPPV[`\0` \x80\x83R``\x83\x01\x84Q`@\x83\x86\x01R\x81\x81Q\x80\x84R`\x80\x87\x01\x91P`\x80\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15acQW`\x7F\x19\x88\x86\x03\x01\x83Rac?\x85\x85Qaa\xD2V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01ac#V[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90acp\x81\x83ab\xB6V[\x95\x94PPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11wV[`\0`@\x82\x84\x03\x12\x15ac\xC8W`\0\x80\xFD[a!\xD9\x83\x83abpV[`\0` \x82\x84\x03\x12\x15ac\xE4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac\xFAW`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a!\xD9W`\0\x80\xFD[`\0a\x01\0\x82Q\x80Q\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R`@\x81\x01Q`\x0F\x0B`@\x86\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x88\x01R\x80`\x80\x84\x01Q\x16`\x80\x88\x01RPP`\xA0\x81\x01Q`\x0F\x0B`\xA0\x86\x01RP` \x83\x01Qadz`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Q\x81`\xE0\x86\x01Racp\x82\x86\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84ad\rV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x11wV[`\0\x80\x83`\x1F\x84\x01\x12ad\xD7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xEEW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15ae\tW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15ae#W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ae9W`\0\x80\xFD[aeE\x85\x82\x86\x01ad\xC5V[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a;,W`\0\x80\xFD[\x805aen\x81aeQV[\x91\x90PV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aenW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15af\x9AWaf\x9Aae\x8AV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12af\xB3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\xCCWaf\xCCae\x8AV[af\xDF`\x1F\x82\x01`\x1F\x19\x16` \x01afrV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15af\xF4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15ag'W`\0\x80\xFD[\x845\x93P` \x85\x015ag9\x81aeQV[\x92PagG`@\x86\x01aesV[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15agbW`\0\x80\xFD[agn\x87\x82\x88\x01af\xA2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ag\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ag\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xBBW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a`\x8FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ag\xDBV[` \x81R`\0\x82Q``` \x84\x01Rah\x16`\x80\x84\x01\x82ag\xC7V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Rah4\x83\x83ag\xC7V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPacp\x82\x82ag\xC7V[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Raa\x9E``\x84\x01\x82aa\xD2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a;,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xA7W`\0\x80\xFD[\x815a!\xD9\x81ah\x80V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15ah\xDFW`\0\x80\xFD[ah\xE8\x85ah\xB2V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ai\x03W`\0\x80\xFD[ai\x0F\x87\x82\x88\x01ad\xC5V[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15ai4W`\0\x80\xFD[ai<ae\xA0V[aiE\x83ah\xB2V[\x81RaiS` \x84\x01ah\xB2V[` \x82\x01Raid`@\x84\x01ah\xB2V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ai\x82W`\0\x80\xFD[\x815a!\xD9\x81aeQV[`\0` \x82\x84\x03\x12\x15ai\x9FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xB5W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a`\xE7V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01Raa\x9E`\xC0\x85\x01\x82aa\xD2V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01Raa\x9E`\x80\x85\x01\x82ai\xC1V[` \x81R`\0a!\xD9` \x83\x01\x84aj V[`\0` \x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aj\x96W`\0\x80\xFD[a!\xD9\x83\x83ajrV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aj\xB4V[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rak\x04`\xE0\x85\x01\x82aj\xA0V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rak!\x82\x82ab\xB6V[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90acp\x81\x83aa\xD2V[` \x81R`\0\x82Q`@` \x84\x01Rakp``\x84\x01\x82ab\xB6V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Racp\x82\x82ag\xC7V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15ak\xA6Wak\xA6ae\x8AV[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a;,W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ak\xD0W`\0\x80\xFD[\x815` ak\xE5ak\xE0\x83ak\x8DV[afrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15al\x04W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805al\x1B\x81ak\xB0V[\x83R\x91\x83\x01\x91\x83\x01al\x08V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15alLW`\0\x80\xFD[\x865alW\x81ah\x80V[\x95P` \x87\x015alg\x81ah\x80V[\x94P`@\x87\x015alw\x81ah\x80V[\x93P``\x87\x015al\x87\x81ah\x80V[\x92P`\x80\x87\x015al\x97\x81ah\x80V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15al\xB2W`\0\x80\xFD[al\xBE\x89\x82\x8A\x01ak\xBFV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01Raa\x9E`@\x85\x01\x82ag\xC7V[` \x81R`\0a!\xD9` \x83\x01\x84al\xCBV[`\0`\xC0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15am*W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15am@W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01am\x06V[` \x81R`\0a!\xD9` \x83\x01\x84ai\xC1V[\x805\x80\x15\x15\x81\x14aenW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15am\x82W`\0\x80\xFD[\x825am\x8D\x81ah\x80V[\x91Pam\x9B` \x84\x01am_V[\x90P\x92P\x92\x90PV[` \x81Ram\xE9` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Raa\x9E`\xC0\x84\x01\x82aa\xD2V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11wV[\x805`\xFF\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15an`W`\0\x80\xFD[ani\x87ah\xB2V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15an\x84W`\0\x80\xFD[an\x90\x89\x82\x8A\x01ad\xC5V[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Pan\xB0`\x80\x88\x01an6V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Ran\xEB` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Raa\x9E`\xA0\x84\x01\x82aa\xD2V[`\0\x80\x83`\x1F\x84\x01\x12ao\x17W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ao.W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15ae\tW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15ao[W`\0\x80\xFD[\x835aof\x81ah\x80V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\x81W`\0\x80\xFD[ao\x8D\x86\x82\x87\x01ao\x05V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15ao\xADW`\0\x80\xFD[\x825ao\xB8\x81aeQV[\x91P` \x83\x015ao\xC8\x81ak\xB0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15ao\xE6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xFCW`\0\x80\xFD[aeE\x85\x82\x86\x01ao\x05V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rap-``\x85\x01\x82ai\xC1V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Racp\x82\x82ai\xC1V[` \x81R`\0a!\xD9` \x83\x01\x84ap\x08V[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15ap\x86W`\0\x80\xFD[ap\x8F\x84apYV[\x92P` \x84\x015ap\x9F\x81aeQV[\x91Pap\xAD`@\x85\x01aesV[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ap\xC8W`\0\x80\xFD[a!\xD9\x83\x83am\x06V[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aqYW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aqoW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01aq5V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Raa\x9Ea\x01\0\x84\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84aa\xD2V[`\0`\xE0\x82\x84\x03\x12\x15ar\x16W`\0\x80\xFD[a!\xD9\x83\x83aq5V[`\0\x80`@\x83\x85\x03\x12\x15ar3W`\0\x80\xFD[ar<\x83ah\xB2V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15arXW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15arlW`\0\x80\xFD[artae\xA0V[ar}\x83ah\xB2V[\x81R` \x83\x015ar\x8D\x81ah\x80V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ar\xA4W`\0\x80\xFD[ar\xB0\x88\x82\x86\x01af\xA2V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01RasI`\xC0\x85\x01\x82aj\xA0V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Racp\x81\x83aa\xD2V[as\xAC\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xA0``\x85\x01Ras\xC7`\xA0\x85\x01\x82aa\xD2V[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a!\xD9` \x83\x01\x84as{V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Ratk``\x85\x01\x82ab\xB6V[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15al(W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90at\x8CV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Raa\x9E`\x80\x84\x01\x82ab\xB6V[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Rau\x1C`\xA0\x84\x01\x82ai\xC1V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Racp\x82\x82ai\xC1V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15au\xCFW\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01au\xB0V[PPP\x83\x01Q`\xE0`\x80\x84\x01Rau\xEAa\x01\0\x84\x01\x82aa\xD2V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Rav\x08\x83\x83ab\xB6V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPav&\x82\x82ab\xB6V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15av\xADW`\0\x80\xFD[av\xB6\x86apYV[\x94P` \x86\x015av\xC6\x81aeQV[\x93Pav\xD4`@\x87\x01aesV[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15av\xEFW`\0\x80\xFD[av\xFB\x88\x82\x89\x01ao\x05V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aw\x1EW`\0\x80\xFD[a!\xD9\x82ah\xB2V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rawj`\xC0\x85\x01\x82aa\xD2V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15aw\x95W`\0\x80\xFD[aw\x9Dae\xA0V[\x90P\x815\x81R` \x82\x015` \x82\x01Raw\xB9`@\x83\x01ah\xB2V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15aw\xD6W`\0\x80\xFD[a!\xD9\x83\x83aw\x83V[`\0`\x80\x82\x84\x03\x12\x15aw\xF2W`\0\x80\xFD[aw\xFAae\xC8V[\x90P\x815\x81R` \x82\x015ax\x0E\x81aeQV[` \x82\x01Rax\x1F`@\x83\x01aesV[`@\x82\x01Rax0``\x83\x01ah\xB2V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15axMW`\0\x80\xFD[a!\xD9\x83\x83aw\xE0V[`\0`\x80\x82\x84\x03\x12\x15axiW`\0\x80\xFD[axqae\xC8V[\x825\x81R` \x83\x015ax\x83\x81aeQV[` \x82\x01Rax\x94`@\x84\x01aesV[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15ax\xBFW`\0\x80\xFD[ax\xC7ae\xEAV[\x90Pax\xD3\x83\x83aw\xE0V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[ax\xFA\x84\x82\x85\x01af\xA2V[` \x83\x01RP\x92\x91PPV[`\0a\x11w6\x83ax\xADV[`\0\x82`\x1F\x83\x01\x12ay#W`\0\x80\xFD[\x815` ay3ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ayRW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805\x83R\x91\x83\x01\x91\x83\x01ayVV[`\0`@\x826\x03\x12\x15ay\x7FW`\0\x80\xFD[ay\x87ae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ay\x9EW`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12ay\xB1W`\0\x80\xFD[\x815` ay\xC1ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15ay\xE0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15az\x18W\x805\x86\x81\x11\x15ay\xFCW`\0\x80\x81\xFD[az\n6\x86\x83\x8B\x01\x01af\xA2V[\x84RP\x91\x83\x01\x91\x83\x01ay\xE4V[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15az/W`\0\x80\xFD[az;6\x85\x89\x01ay\x12V[\x90\x85\x01RP\x91\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15az[W`\0\x80\xFD[azcae\xC8V[\x90P\x815\x81R` \x82\x015` \x82\x01Rax\x1F`@\x83\x01aesV[`\0`\x80\x82\x84\x03\x12\x15az\x91W`\0\x80\xFD[a!\xD9\x83\x83azIV[`\0`\x80\x82\x84\x03\x12\x15az\xADW`\0\x80\xFD[az\xB5ae\xC8V[\x825\x81R` \x83\x015az\xC7\x81aeQV[` \x82\x01R`@\x83\x015az\xDA\x81ak\xB0V[`@\x82\x01R``\x83\x015az\xED\x81ak\xB0V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a{\x0BW`\0\x80\xFD[a{\x13ae\xEAV[a{\x1C\x83aesV[\x81R` \x83\x015a{,\x81ah\x80V[` \x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a{JW`\0\x80\xFD[a{Rae\xEAV[\x825a{]\x81ak\xB0V[\x81R` \x83\x015a{,\x81ak\xB0V[`\0\x81\x83\x03a\x01\0\x81\x12\x15a{\x81W`\0\x80\xFD[a{\x89ae\xA0V[\x91P`\xC0\x81\x12\x15a{\x99W`\0\x80\xFD[Pa{\xA2af\x0CV[\x825\x81R` \x83\x015a{\xB4\x81ak\xB0V[` \x82\x01R`@\x83\x015a{\xC7\x81ak\xB0V[`@\x82\x01Ra{\xD8``\x84\x01ah\xB2V[``\x82\x01Ra{\xE9`\x80\x84\x01ah\xB2V[`\x80\x82\x01R`\xA0\x83\x015a{\xFC\x81ak\xB0V[`\xA0\x82\x01R\x81Ra|\x0F`\xC0\x83\x01aecV[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a|-W`\0\x80\xFD[a|9\x84\x82\x85\x01af\xA2V[`@\x83\x01RP\x92\x91PPV[`\0a\x11w6\x83a{mV[`\0`@\x82\x84\x03\x12\x15a|cW`\0\x80\xFD[a|kae\xEAV[\x90P\x815a|x\x81aeQV[\x81R` \x82\x015a|\x88\x81ak\xB0V[` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a|\xA5W`\0\x80\xFD[a!\xD9\x83\x83a|QV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a|\xDCW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a|\xF6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15ae\tW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a}3Wa}3a}\x0BV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a}\\Wa}\\a}\x0BV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80a}~Wa}~a}\x0BV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a}\xA4Wa}\xA4a}\x0BV[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a}\xC0W`\0\x80\xFD[\x81Qa!\xD9\x81ah\x80V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qa~\x02\x81`\x01\x85\x01` \x87\x01aa\xA6V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x826\x03\x12\x15a~\"W`\0\x80\xFD[a~*ae\xA0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a~AW`\0\x80\xFD[a~M6\x83\x87\x01ak\xBFV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a~cW`\0\x80\xFD[a~o6\x83\x87\x01ak\xBFV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a~\x88W`\0\x80\xFD[Pa|96\x82\x86\x01ak\xBFV[`\0`@\x82\x84\x03\x12\x15a~\xA7W`\0\x80\xFD[a~\xAFae\xEAV[\x90P\x815a~\xBC\x81ah\x80V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0a\x11w6\x83a~\x95V[`\0\x82\x82\x10\x15a~\xF5Wa~\xF5a}\x0BV[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14aenW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x7F\x1EW`\0\x80\xFD[a\x7F&ae\xC8V[\x825a\x7F1\x81ah\x80V[\x81R` \x83\x015a\x7FA\x81aeQV[` \x82\x01Ra\x7FR`@\x84\x01a~\xFAV[`@\x82\x01Raz\xED``\x84\x01a~\xFAV[`\0\x81\x83\x03`\xC0\x81\x12\x15a\x7FvW`\0\x80\xFD[a\x7F~ae\xEAV[\x91P`\xA0\x81\x12\x15a\x7F\x8EW`\0\x80\xFD[Pa\x7F\x97af.V[\x825\x81R` \x83\x015a\x7F\xA9\x81ak\xB0V[` \x82\x01R`@\x83\x015a\x7F\xBC\x81ak\xB0V[`@\x82\x01Ra\x7F\xCD``\x84\x01ah\xB2V[``\x82\x01Ra\x7F\xDE`\x80\x84\x01ah\xB2V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x80\x10W`\0\x80\xFD[a\x80\x18ae\xC8V[\x90P\x815a\x80%\x81aeQV[\x81R` \x82\x015a\x805\x81ak\xB0V[` \x82\x01R`@\x82\x015a\x80H\x81ak\xB0V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x80fW`\0\x80\xFD[a\x80r\x84\x82\x85\x01a\x7FcV[``\x83\x01RP\x92\x91PPV[`\0a\x11w6\x83a\x7F\xFEV[`\0` \x82\x84\x03\x12\x15a\x80\x9CW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x80\xBEWa\x80\xBEae\x8AV[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x80\xDDW`\0\x80\xFD[a\x80\xE5ae\xA0V[\x825\x81R` \x83\x015a\x80\xF7\x81aeQV[` \x82\x01Raid`@\x84\x01aesV[`\0\x82`\x1F\x83\x01\x12a\x81\x19W`\0\x80\xFD[\x815` a\x81)ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x81HW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805a\x81_\x81aeQV[\x83R\x91\x83\x01\x91\x83\x01a\x81LV[`\0`@\x826\x03\x12\x15a\x81~W`\0\x80\xFD[a\x81\x86ae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x81\x9DW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15a\x81\xB2W`\0\x80\xFD[a\x81\xBAae\xC8V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x81\xD0W`\0\x80\xFD[a\x81\xDC6\x82\x86\x01a\x81\x08V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x81\xF4W`\0\x80\xFD[a\x82\x006\x82\x86\x01ay\x12V[`@\x83\x01RPa\x82\x12``\x84\x01ah\xB2V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82-W`\0\x80\xFD[Pax\xFA6\x82\x86\x01af\xA2V[`\0`@\x826\x03\x12\x15a\x82LW`\0\x80\xFD[a\x82Tae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x82kW`\0\x80\xFD[a\x82w6\x83\x87\x01ay\x12V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82\x8DW`\0\x80\xFD[Pax\xFA6\x82\x86\x01ak\xBFV[` \x81\x01`\x02\x83\x10a\x82\xBCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a}\xA4Wa}\xA4a}\x0BV[`\0`@\x82\x84\x03\x12\x15a\x82\xEDW`\0\x80\xFD[a\x82\xF5ae\xEAV[\x90Pa\x83\0\x82aesV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x1BW`\0\x80\xFD[ax\xFA\x84\x82\x85\x01ak\xBFV[`\0a\x11w6\x83a\x82\xDBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x83GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AxWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aa\x9E`@\x83\x01\x84aa\xD2V[`\0` \x82\x84\x03\x12\x15a\x83\x9BW`\0\x80\xFD[PQ\x91\x90PV[`\0a\x11w6\x83a\x7FcV[`\0`\xA0\x82\x84\x03\x12\x15a\x83\xC0W`\0\x80\xFD[a\x83\xC8ae\xEAV[\x90Pax\xD3\x83\x83azIV[`\0a\x11w6\x83a\x83\xAEV[`\0``\x82\x84\x03\x12\x15a\x83\xF2W`\0\x80\xFD[a\x83\xFAae\xA0V[\x90P\x815\x81Ra\x84\x0C` \x83\x01aesV[` \x82\x01Raw\xB9`@\x83\x01ah\xB2V[`\0``\x82\x84\x03\x12\x15a\x84/W`\0\x80\xFD[a!\xD9\x83\x83a\x83\xE0V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x84eW`\0\x80\xFD[a\x84mae\xEAV[\x90Pa\x84y\x83\x83aw\x83V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0a\x11w6\x83a\x84SV[`\0\x80\x85\x85\x11\x15a\x84\xB0W`\0\x80\xFD[\x83\x86\x11\x15a\x84\xBDW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0aa\x9E` \x83\x01\x84\x86a\x84\xCAV[`\0`\xC0\x82\x84\x03\x12\x15a\x85\x19W`\0\x80\xFD[a\x85!af\x0CV[\x90P\x815\x81R` \x82\x015a\x855\x81aeQV[` \x82\x01Ra\x85F`@\x83\x01aesV[`@\x82\x01Ra\x85W``\x83\x01aesV[``\x82\x01Ra\x85h`\x80\x83\x01aesV[`\x80\x82\x01Ra\x85y`\xA0\x83\x01ah\xB2V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x85\x96W`\0\x80\xFD[a!\xD9\x83\x83a\x85\x07V[`\0` \x82\x84\x03\x12\x15a\x85\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xC8W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a~\x95V[`@\x81R`\0a\x85\xE8`@\x83\x01\x85\x87a\x84\xCAV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x86\x14W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x86*W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x86;W`\0\x80\xFD[\x80Qa\x86Iak\xE0\x82ak\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x86hW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x86\x8FW\x83Qa\x86\x80\x81aeQV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x86mV[\x97\x96PPPPPPPV[` \x81R`\0a!\xD9` \x83\x01\x84ag\xC7V[`\0` \x82\x84\x03\x12\x15a\x86\xBFW`\0\x80\xFD[\x81Qa!\xD9\x81aeQV[`\0`\xC0\x82\x84\x03\x12\x15a\x86\xDCW`\0\x80\xFD[a\x86\xE4af\x0CV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a\x87\x02\x81aeQV[`@\x82\x01Ra\x87\x13``\x83\x01am_V[``\x82\x01R`\x80\x82\x015a\x85h\x81ak\xB0V[`\0`\xE0\x82\x84\x03\x12\x15a\x878W`\0\x80\xFD[a\x87@ae\xEAV[\x90Pa\x87L\x83\x83a\x86\xCAV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x87yW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x87\x8FW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x87&V[`\0``\x82\x84\x03\x12\x15a\x87\xADW`\0\x80\xFD[a\x87\xB5ae\xA0V[\x90P\x815a\x87\xC2\x81aeQV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x87\xDEW`\0\x80\xFD[a\x87\xEA\x85\x83\x86\x01a\x7FcV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x88\x03W`\0\x80\xFD[Pa|9\x84\x82\x85\x01a\x7FcV[`\0a\x11w6\x83a\x87\x9BV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1AxW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x11w6\x83a\x87&V[`\0` \x82\x84\x03\x12\x15a\x88^W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x88\x80Wa\x88\x80ae\x8AV[`@R\x90P\x80a\x88\x8F\x83aesV[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x88\xA9W`\0\x80\xFD[a!\xD9\x83\x83a\x88LV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aenW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x88\xEDW`\0\x80\xFD[a\x88\xF5afPV[\x825\x81R` \x83\x015a\x89\x07\x81aeQV[` \x82\x01R`@\x83\x015a\x89\x1A\x81aeQV[`@\x82\x01Ra\x89+``\x84\x01a\x88\xB3V[``\x82\x01Ra\x89<`\x80\x84\x01a\x88\xB3V[`\x80\x82\x01Ra\x89M`\xA0\x84\x01ah\xB2V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x89fW`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x89\x84W`\0\x80\xFD[a\x89\x8Cae\xA0V[\x825a\x89\x97\x81aeQV[\x81Ra\x89\xA5` \x84\x01aesV[` \x82\x01R`@\x83\x015aid\x81ah\x80V[`\0`\xC0\x82\x84\x03\x12\x15a\x89\xCAW`\0\x80\xFD[a!\xD9\x83\x83a\x86\xCAV[`\0`@\x826\x03\x12\x15a\x89\xE6W`\0\x80\xFD[a\x89\xEEae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8A\x05W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x8A\x1AW`\0\x80\xFD[a\x8A\"ae\xA0V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x8A8W`\0\x80\xFD[a\x8AD6\x82\x86\x01a\x81\x08V[` \x83\x01RPa\x8AV`@\x84\x01ah\xB2V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82-W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x8A\x83W`\0\x80\xFD[a\x8A\x8Bae\xA0V[\x90Pa\x8A\x97\x83\x83a\x83\xE0V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\xB2W`\0\x80\xFD[a\x8A\xBE\x84\x82\x85\x01af\xA2V[` \x83\x01RP`\x80\x82\x015aw\xB9\x81ak\xB0V[`\0a\x11w6\x83a\x8AqV[`\0`\xC0\x82\x84\x03\x12\x15a\x8A\xF0W`\0\x80\xFD[a\x8A\xF8af\x0CV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x8B\x14`@\x83\x01an6V[`@\x82\x01R``\x82\x015a\x87\x13\x81aeQV[`\0`\xE0\x826\x03\x12\x15a\x8B9W`\0\x80\xFD[a\x8BAae\xEAV[a\x8BK6\x84a\x8A\xDEV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8BfW`\0\x80\xFD[ax\xFA6\x82\x86\x01af\xA2V[`\0`@\x826\x03\x12\x15a\x8B\x84W`\0\x80\xFD[a\x8B\x8Cae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8B\xA3W`\0\x80\xFD[a\x8B\xAF6\x83\x87\x01ay\x12V[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x8B\xC6W`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x8B\xD9W`\0\x80\xFD[\x805a\x8B\xE7ak\xE0\x82ak\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x8C\x06W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x8C$W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x8C\x0BV[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x8CGW`\0\x80\xFD[a\x8COae\xA0V[\x825a\x8CZ\x81aeQV[\x81R` \x83\x015a\x8Cj\x81ak\xB0V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8C\x88W`\0\x80\xFD[a|96\x82\x86\x01ay\x12V[`\0`\x80\x826\x03\x12\x15a\x8C\xA6W`\0\x80\xFD[a\x8C\xAEae\xC8V[\x825a\x8C\xB9\x81aeQV[\x81Ra\x8C\xC7` \x84\x01am_V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8C\xE6W`\0\x80\xFD[a\x8C\xF26\x83\x87\x01a\x7FcV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x8D\x0BW`\0\x80\xFD[Pa\x80r6\x82\x86\x01a\x7FcV[`\0`\xE0\x82\x84\x03\x12\x15a\x8D*W`\0\x80\xFD[a\x8D2ae\xEAV[\x90Pa\x87L\x83\x83a\x85\x07V[`\0a\x11w6\x83a\x8D\x18V[`\0\x82`\x1F\x83\x01\x12a\x8D[W`\0\x80\xFD[a\x8Dcae\xA0V[\x80``\x84\x01\x85\x81\x11\x15a\x8DuW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x8D\x8FW\x805\x84R` \x93\x84\x01\x93\x01a\x8DwV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x8D\xACW`\0\x80\xFD[a\x8D\xB4af.V[a\x8D\xBE6\x84a\x8DJV[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8D\xDAW`\0\x80\xFD[a\x8D\xE66\x83\x87\x01af\xA2V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x8D\xFFW`\0\x80\xFD[a\x8E\x0B6\x83\x87\x01ay\x12V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x8E$W`\0\x80\xFD[Pa\x8E16\x82\x86\x01ay\x12V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x8EYW`\0\x80\xFD[a!\xD9\x83\x83a\x8A\xDEV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x8E\x96Wa\x8E\x96a}\x0BV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x8E\xBBWa\x8E\xBBa}\x0BV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8E\xD6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E\xECW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01ax\xADV[`\0` \x82\x84\x03\x12\x15a\x8F\nW`\0\x80\xFD[\x81Qa!\xD9\x81ak\xB0V[`\0` \x82\x84\x03\x12\x15a\x8F'W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F=W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x82\xDBV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x8FiWa\x8Fia}\x0BV[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aa\x9E`@\x83\x01\x84ag\xC7V[`\0\x80`@\x83\x85\x03\x12\x15a\x8F\xA6W`\0\x80\xFD[\x82Qa\x8F\xB1\x81aeQV[` \x84\x01Q\x90\x92Pao\xC8\x81ak\xB0V[`\0` \x82\x84\x03\x12\x15a\x8F\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xEAW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x87\x9BV[` \x81R`\0\x82Q``` \x84\x01Ra\x90\x12`\x80\x84\x01\x82ap\x08V[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x90SW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90iW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x7F\xFEV[`@\x81R`\0a\x90\x88`@\x83\x01\x85aj V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x90\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\xC8W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x8D\x18V[`\0` \x82\x84\x03\x12\x15a\x90\xE6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\xFCW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x8AqV[`\0` \x82\x84\x03\x12\x15a\x91\x1AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x910W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x84SV[`\0` \x82\x84\x03\x12\x15a\x91NW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x91dW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x83\xAEV[`\0` \x82\x84\x03\x12\x15a\x91\x82W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x91\x98W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a{mV[`@\x81R`\0a\x90\x88`@\x83\x01\x85ad\rV[`\0\x82a\x91\xD4WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x92\x11\x81`\x19\x85\x01` \x87\x01aa\xA6V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x92DWa\x92Da}\x0BV[`\0\x03\x92\x91PPV\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)MintVlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)BurnVlp(bytes32 sender,uint128 vlpAmount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)BurnLp(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)no slow mode transactions remainingMintLp(bytes32 sender,uint32 productId,uint128 amountBase,uint128 quoteAmountLow,uint128 quoteAmountHigh,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x8F\xE9a\x18m\xBE\xD1\xAC}l\r-\xEC\xEA~^y'{p5\xA3\x1D\x86\xF7\xF0\xA82\x8C\x8B\x0F\xCAdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static ENDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\xF9W`\x005`\xE0\x1C\x80c}\xB6\xA2[\x11a\x03\x19W\x80c\xB1\xC8\xEC+\x11a\x01\xA7W\x80c\xD4\xDE\x8D\x9D\x11a\0\xF9W\x80c\xEERU&\x11a\0\xA2W\x80c\xF8\x0F|\xE5\x11a\0|W\x80c\xF8\x0F|\xE5\x14a\r\x95W\x80c\xFA\xB2\xC4i\x14a\x11DW\x80c\xFB\xF4\x19\x84\x14a\x10\x1FW`\0\x80\xFD[\x80c\xEERU&\x14a\x10\xE6W\x80c\xEFd\xED\x0E\x14a\x11\x08W\x80c\xF2\xFD\xE3\x8B\x14a\x111W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x10mW\x80c\xE9\xABw\xE5\x14a\x10\x80W\x80c\xE9\xBCtb\x14a\x10\xD3W`\0\x80\xFD[\x80c\xD4\xDE\x8D\x9D\x14a\x10\x1FW\x80c\xDBZP!\x14a\x10-W\x80c\xDCB\xE6\x1B\x14a\x10MW`\0\x80\xFD[\x80c\xB7\x0E\xB2c\x11a\x01[W\x80c\xC4\xF9\xB2_\x11a\x015W\x80c\xC4\xF9\xB2_\x14a\x0F\xE7W\x80c\xC5\x105\x9F\x14a\x0F\xF8W\x80c\xD3\x8C;\x9C\x14a\x0F\xFFW`\0\x80\xFD[\x80c\xB7\x0E\xB2c\x14a\x0F\xC1W\x80c\xBC\x85\xCA\x86\x14a\x06'W\x80c\xC3ES\x0B\x14a\x0F\xD4W`\0\x80\xFD[\x80c\xB2\xBBcg\x11a\x01\x8CW\x80c\xB2\xBBcg\x14a\x0FaW\x80c\xB3\x14}\x17\x14a\x0F\x81W\x80c\xB3d\x88\xB8\x14a\x0F\xA1W`\0\x80\xFD[\x80c\xB1\xC8\xEC+\x14a\x0BGW\x80c\xB1\xFB\xD6\x0B\x14a\x0FAW`\0\x80\xFD[\x80c\x91\x0E`j\x11a\x02kW\x80c\x98\xCD2\xFE\x11a\x02\x14W\x80c\xA0\x82\xC5\xAA\x11a\x01\xEEW\x80c\xA0\x82\xC5\xAA\x14a\x0F\x01W\x80c\xA0\xCCc\r\x14a\x0F!W\x80c\xA7\x80\xA4\xBE\x14a\x0F!W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x14a\x0E\x81W\x80c\x9A\x08\xE55\x14a\x0E\x94W\x80c\x9E\x85\x14$\x14a\x0E\xE1W`\0\x80\xFD[\x80c\x94\xFA\xEF\xE5\x11a\x02EW\x80c\x94\xFA\xEF\xE5\x14a\r\x95W\x80c\x954\xDD>\x14a\r\xC0W\x80c\x96\xC4|o\x14a\r\xE0W`\0\x80\xFD[\x80c\x91\x0E`j\x14a\rBW\x80c\x91q\xD0\x8B\x14a\rbW\x80c\x91\xC1\xE3\xD7\x14a\r\x82W`\0\x80\xFD[\x80c\x8CX\xE1\n\x11a\x02\xCDW\x80c\x8D\xA5\xCB[\x11a\x02\xA7W\x80c\x8D\xA5\xCB[\x14a\r\rW\x80c\x8E]X\x8C\x14a\r\x1EW\x80c\x8FO\x8E\xCC\x14a\r1W`\0\x80\xFD[\x80c\x8CX\xE1\n\x14a\x0C\x96W\x80c\x8D\n\xCC\x9B\x14a\x0C\xDAW\x80c\x8D< \xB1\x14a\x0C\xEDW`\0\x80\xFD[\x80c\x85\xC8>\x9D\x11a\x02\xFEW\x80c\x85\xC8>\x9D\x14a\x0CFW\x80c\x872C8\x14a\x0CfW\x80c\x8C=/t\x14a\x0CyW`\0\x80\xFD[\x80c}\xB6\xA2[\x14a\x0C3W\x80c\x82fD\xF7\x14a\x0C\x13W`\0\x80\xFD[\x80c/\x9A'D\x11a\x04\x96W\x80cTDV\x9D\x11a\x03\xE8W\x80ce\xDD\x13f\x11a\x03\x91W\x80co;\nr\x11a\x03kW\x80co;\nr\x14a\x0B\xEBW\x80cqP\x18\xA6\x14a\x0C\x0BW\x80cy\xF1$3\x14a\x0C\x13W`\0\x80\xFD[\x80ce\xDD\x13f\x14a\x0B\x90W\x80clEup\x14a\x0B\x98W\x80cl\xFE_\xE4\x14a\x0B\xB8W`\0\x80\xFD[\x80c[\xB4\xC1&\x11a\x03\xC2W\x80c[\xB4\xC1&\x14a\x0BgW\x80c]O_\x97\x14a\x0B}W\x80ca\x0B.^\x14a\x06\x9CW`\0\x80\xFD[\x80cTDV\x9D\x14a\x0B\x14W\x80cU~\xD1\xBA\x14a\x0B'W\x80cZ\0\x92;\x14a\x0BGW`\0\x80\xFD[\x80c8B\xE7^\x11a\x04JW\x80cB\xC7M\x1D\x11a\x04$W\x80cB\xC7M\x1D\x14a\n\xA6W\x80cM\x96\xA9\n\x14a\n\xC6W\x80cO\xCF\xAEX\x14a\n\xEBW`\0\x80\xFD[\x80c8B\xE7^\x14a\n W\x80c<\xECK\x93\x14a\nBW\x80c>\xDF,[\x14a\n\x86W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04{W\x80c5c\x9AO\x14a\tzW\x80c6\x8EF\x86\x14a\t\xDAW\x80c6\xB9\x0CQ\x14a\n\0W`\0\x80\xFD[\x80c/\x9A'D\x14a\t\x06W\x80c2\x16\xC0b\x14a\t\x19W`\0\x80\xFD[\x80c\x1D\x97\xD2/\x11a\x05OW\x80c\"\x1F\t9\x11a\x05\x03W\x80c,\x8Co\xFB\x11a\x04\xDDW\x80c,\x8Co\xFB\x14a\x08\x94W\x80c,\xD7\x1B\x16\x14a\x08\xB4W\x80c-\x035\xAB\x14a\x08\xD4W`\0\x80\xFD[\x80c\"\x1F\t9\x14a\x08XW\x80c\"\xD4\xA8-\x14a\x08kW\x80c'ay\x97\x14a\x08\x13W`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x054W\x80c\x1F\x18k'\x14a\x083W\x80c!\x04u\x89\x14a\x08HW\x80c\"\0`F\x14a\x08PW`\0\x80\xFD[\x80c\x1D\x97\xD2/\x14a\x06\xDCW\x80c\x1D\x9E\xED\xA5\x14a\x08\x13W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05\xB1W\x80c\x15\xD3\"\xE7\x11a\x05\x8BW\x80c\x15\xD3\"\xE7\x14a\x07\x93W\x80c\x16\xCD\xB6\x90\x14a\x07\xC8W\x80c\x18\xED\x16\xEB\x14a\x07\xE8W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06\xDCW\x80c\x0FKP\x9D\x14a\x06\xFCW\x80c\x14sWU\x14a\x07SW`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05\xE2W\x80c\x07H\xA2\x19\x14a\x06GW\x80c\rU\xE2k\x14a\x06\x9CW\x80c\x0Ef&[\x14a\x06\xBCW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\xFEW\x80c\x06\xC0\xBA\xFD\x14a\x06'W[`\0\x80\xFD[a\x06\x11a\x06\x0C6`\x04a`\xA1V[a\x11KV[`@Qa\x06\x1E\x91\x90a`\xBDV[`@Q\x80\x91\x03\x90\xF3[a\x06:a\x0656`\x04a`\xF9V[a\x11}V[`@Qa\x06\x1E\x91\x90aa\x15V[a\x06Za\x06U6`\x04a`\xF9V[a\x11\xB0V[`@Qa\x06\x1E\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06\xAFa\x06\xAA6`\x04aajV[a\x11\xE3V[`@Qa\x06\x1E\x91\x90ab]V[a\x06\xCFa\x06\xCA6`\x04ab\x82V[a\x11\xF4V[`@Qa\x06\x1E\x91\x90ab\xF1V[a\x06\xEFa\x06\xEA6`\x04a`\xF9V[a\x12\x11V[`@Qa\x06\x1E\x91\x90acyV[a\x07\x0Fa\x07\n6`\x04a`\xF9V[a\x12DV[`@Qa\x06\x1E\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x07fa\x07a6`\x04ac\xB6V[a\x12wV[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x06\x1EV[a\x07\xA6a\x07\xA16`\x04ac\xB6V[a\x12\x9AV[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x93\x84\x01Q\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x06\x1EV[a\x07\xDBa\x07\xD66`\x04ac\xD2V[a\x12\xBDV[`@Qa\x06\x1E\x91\x90ad\x92V[`\xA6Ta\x07\xFB\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x08&a\x08!6`\x04ac\xB6V[a\x13\x10V[`@Qa\x06\x1E\x91\x90ad\xA5V[a\x08Fa\x08A6`\x04ae\x10V[a\x133V[\0[a\x08Fa\x13\xCFV[a\x07\xFBa\x14\x7FV[a\x08Fa\x08f6`\x04ag\x11V[a\x14\xC4V[a\x07\xFBa\x08y6`\x04agzV[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08\xA7a\x08\xA26`\x04ag\x93V[a\x18SV[`@Qa\x06\x1E\x91\x90ag\xFAV[a\x08\xC7a\x08\xC26`\x04ab\x82V[a\x18\x80V[`@Qa\x06\x1E\x91\x90ahRV[a\x07\xFBa\x08\xE26`\x04ah\x95V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08Fa\t\x146`\x04ah\xC9V[a\x18\x9EV[a\x08Fa\t'6`\x04ai\"V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\t\x8Da\t\x886`\x04a`\xF9V[a\x19\xF0V[`@Qa\x06\x1E\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\t\xEDa\t\xE86`\x04aipV[a\x1A#V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x06\x1EV[a\n\x13a\n\x0E6`\x04ai\x8DV[a\x1A~V[`@Qa\x06\x1E\x91\x90aj_V[a\n3a\n.6`\x04aj\x84V[a\x1A\x8FV[`@Q\x90Q\x81R` \x01a\x06\x1EV[a\nUa\nP6`\x04a`\xA1V[a\x1A\xADV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x06\x1EV[a\n\x99a\n\x946`\x04ab\x82V[a\x1A\xD9V[`@Qa\x06\x1E\x91\x90aj\xD6V[a\n\xB9a\n\xB46`\x04ab\x82V[a\x1A\xEAV[`@Qa\x06\x1E\x91\x90akTV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\t\xEDa\n\xF96`\x04aipV[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xAA` R`@\x90 T`\x0F\x0B\x90V[a\x08Fa\x0B\"6`\x04al3V[a\x1B\x07V[a\x0B/a\x1FAV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x0BZa\x0BU6`\x04ab\x82V[a\x1F\xCFV[`@Qa\x06\x1E\x91\x90al\xF3V[a\x0Boa\x1F\xEDV[`@Q\x90\x81R` \x01a\x06\x1EV[`\x9ATa\n\xD3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08Fa!\xE0V[a\x0B\xABa\x0B\xA66`\x04am\x18V[a\"#V[`@Qa\x06\x1E\x91\x90amLV[a\x08Fa\x0B\xC66`\x04amoV[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x0B\xFEa\x0B\xF96`\x04aajV[a\"4V[`@Qa\x06\x1E\x91\x90am\xA4V[a\x08Fa\"EV[a\x0C&a\x0C!6`\x04a`\xA1V[a\"YV[`@Qa\x06\x1E\x91\x90an\x03V[a\x08Fa\x0CA6`\x04anGV[a\"\x85V[a\x0CYa\x0CT6`\x04ai\x8DV[a$WV[`@Qa\x06\x1E\x91\x90an\xBCV[a\x08Fa\x0Ct6`\x04aoFV[a$hV[a\x0C\x81a.\xBAV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x06\x1EV[a\x08Fa\x0C\xA46`\x04ao\x9AV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x08Fa\x0C\xE86`\x04ao\xD3V[a0\xA6V[a\r\0a\x0C\xFB6`\x04ag\x93V[a1\xD5V[`@Qa\x06\x1E\x91\x90apFV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\n\xD3V[a\x08Fa\r,6`\x04apqV[a1\xE6V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\n\xD3V[a\rUa\rP6`\x04ap\xB6V[a2VV[`@Qa\x06\x1E\x91\x90ap\xD2V[a\rua\rp6`\x04aqGV[a2\x97V[`@Qa\x06\x1E\x91\x90aq{V[a\n\xD3a\r\x906`\x04agzV[a2\xA8V[a\r\xA8a\r\xA36`\x04aj\x84V[a3iV[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x06\x1EV[a\r\xD3a\r\xCE6`\x04ah\x95V[a3\x87V[`@Qa\x06\x1E\x91\x90aq\xF1V[a\r\xF3a\r\xEE6`\x04ar\x04V[a4!V[`@Qa\x06\x1E\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x08Fa\x0E\x8F6`\x04ar V[a4iV[a\x0E\xA7a\x0E\xA26`\x04a`\xA1V[a5\xD1V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x06\x1EV[a\x0E\xF4a\x0E\xEF6`\x04ap\xB6V[a5\xFDV[`@Qa\x06\x1E\x91\x90ar\xC3V[a\x0F\x14a\x0F\x0F6`\x04ab\x82V[a6>V[`@Qa\x06\x1E\x91\x90as\x1AV[a\x0F4a\x0F/6`\x04aajV[a6OV[`@Qa\x06\x1E\x91\x90as\xE0V[a\x0FTa\x0FO6`\x04aqGV[a6\x88V[`@Qa\x06\x1E\x91\x90as\xF3V[a\x0Fta\x0Fo6`\x04ab\x82V[a6\x99V[`@Qa\x06\x1E\x91\x90atOV[a\x0F\x94a\x0F\x8F6`\x04ag\x93V[a6\xB6V[`@Qa\x06\x1E\x91\x90at\xACV[a\x0F\xB4a\x0F\xAF6`\x04ai\x8DV[a6\xDBV[`@Qa\x06\x1E\x91\x90at\xE4V[a\x0Boa\x0F\xCF6`\x04aoFV[a6\xECV[a\x0C\x81a\x0F\xE26`\x04aoFV[a7\x89V[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x07\xFBV[`\0a\t\xEDV[a\x10\x12a\x10\r6`\x04aqGV[a8iV[`@Qa\x06\x1E\x91\x90au9V[g\r\xE0\xB6\xB3\xA7d\0\0a\t\xEDV[a\x10@a\x10;6`\x04aqGV[a8zV[`@Qa\x06\x1E\x91\x90au\xA0V[a\x10`a\x10[6`\x04ap\xB6V[a8\x8BV[`@Qa\x06\x1E\x91\x90av=V[a\x08Fa\x10{6`\x04ao\xD3V[a8\xCCV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x06\x1EV[a\x08Fa\x10\xE16`\x04av\x95V[a<\xEFV[a\x10\xF9a\x10\xF46`\x04aw\x0CV[a=}V[`@Qa\x06\x1E\x93\x92\x91\x90aw'V[a\x0Boa\x11\x166`\x04aw\x0CV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x08Fa\x11?6`\x04ah\x95V[a>\x98V[`\xA6a\x0BoV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83aw\xC4V[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83ax;V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83axWV[a\x11\xEBa]\x91V[a\x11w\x82ay\x06V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82aymV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83az\x7FV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83az\x9BV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83az\xF9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83a{8V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a|EV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11w6\x83\x90\x03\x83\x01\x83a|\x93V[`\0[\x81\x81\x10\x15a\x13\x88W6`\0\x84\x84\x84\x81\x81\x10a\x13SWa\x13Sa|\xAFV[\x90P` \x02\x81\x01\x90a\x13e\x91\x90a|\xC5V[\x91P\x91Pa\x13s\x82\x82a?%V[PP\x80\x80a\x13\x80\x90a}!V[\x91PPa\x136V[P`\xA6\x80T\x82\x91\x90`\0\x90a\x13\xA7\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a}:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\t'W`\xA8`\0\x82` \x01\x80Qa\x14=\x90a}eV[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x14x`\x01\x83\x01\x82a]\xCAV[PPa\x14\x08V[`\xA6\x80T`\0\x91\x90\x82\x90a\x14\x9B\x90`\x01`\x01`@\x1B\x03\x16a}\x88V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA6T\x16\x91\x90PV[\x80Q`\0\x03a\x14\xD2W`\0\x80\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x15\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`@Q\x80\x91\x03\x90\xFD[P``\x84\x90\x1Ca\x15R\x813\x81\x14a\x15LW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaR\x04V[\x83aR\x04V[`\x01\x85\x14\x80\x15\x90a\x15xWP`\0\x85\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x15\xF0W`\x9AT`@Qc\xD6\x93\xC5\xF1`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD6\x93\xC5\xF1\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xEBW=`\0\x80>=`\0\xFD[PPPP[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ra\x16t\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16d\x91\x90a}\xAEV[3\x85`\x01`\x01`\x80\x1B\x03\x16aRYV[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x16\xC1Bb\x03\xF4\x80a}:V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x17E\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17c\x92\x91` \x01a}\xE1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a\x17\x8B\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x17\xF7\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPV[a\x18w`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x11w\x82a~\x10V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11w\x82a~\xD7V[`\0Z\x90Pa\x18\xAC\x85aR\x82V[`\0[\x83\x81\x10\x15a\x19wW6`\0\x86\x86\x84\x81\x81\x10a\x18\xCCWa\x18\xCCa|\xAFV[\x90P` \x02\x81\x01\x90a\x18\xDE\x91\x90a|\xC5V[\x91P\x91Pa\x18\xEC\x82\x82a?%V[\x84Za\x18\xF8\x90\x86a~\xE3V[\x11\x15a\x19bW`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19IW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19]W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x19o\x90a}!V[\x91PPa\x18\xAFV[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x19\x95\x90\x85a~\xE3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xD1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19\xE5W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x7F\x0CV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1AxW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x91\x90PV[a\x1A\x86a^\x88V[a\x11w\x82a\x80~V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11w6\x83\x90\x03\x83\x01\x83a\x80\x8AV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x80\xCBV[a\x1A\xE1a^\xB6V[a\x11w\x82a\x81lV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82a\x82:V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1B'WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1BAWP0;\x15\x80\x15a\x1BAWP`\0T`\xFF\x16`\x01\x14[a\x1B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x12V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B\xD6W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1B\xDEaR\xCAV[a\x1CR`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaS=V[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1C\xCD\x90`\0\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x0E\x91\x90a}\xAEV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1DR\x90`\x01\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x93\x91\x90a}\xAEV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x15\x91\x90a}\xAEV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\xF1W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\xA0Wa\x1E\xA0a|\xAFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1E\xE9\x81a\x82\xC2V[\x91PPa\x1EyV[P\x80\x15a\x1F8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F~W\x81Qa\x1F\x84V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1F\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11w\x82a\x83'V[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a \xE0\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x0C\x90a\x833V[\x80\x15a!YW\x80`\x1F\x10a!.Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!YV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!<W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\x93\x92\x90\x91`\x04\x01a\x83gV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a!\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra!\xC9\x91\x81\x01\x90a\x83\x89V[`\x01[a!\xD9WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\t'\x81`\0aS\xB2V[a\"+a^\xE5V[a\x11w\x82a\x83\xA2V[a\"<a_\x1CV[a\x11w\x82a\x83\xD4V[a\"MaVQV[a\"W`\0aV\xABV[V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x84\x1DV[a\"\x8E\x86aR\x82V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA5W`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a#?W\x81\x87\x87\x83\x81\x81\x10a\"\xF2Wa\"\xF2a|\xAFV[\x90P` \x02\x81\x01\x90a#\x04\x91\x90a|\xC5V[`@Q` \x01a#\x16\x93\x92\x91\x90a\x849V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a#8\x90a}!V[\x90Pa\"\xD7V[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xB1W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a$MW6`\0\x88\x88\x84\x81\x81\x10a#\xD5Wa#\xD5a|\xAFV[\x90P` \x02\x81\x01\x90a#\xE7\x91\x90a|\xC5V[\x91P\x91Pa#\xF5\x82\x82a?%V[`\xA6\x80T`\x01\x91\x90`\0\x90a$\x14\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a}:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a$E\x90a}!V[\x91PPa#\xB8V[PPPPPPPPV[a$_a_LV[a\x11w\x82a\x84\x94V[30\x14a$tW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a$\x89Wa$\x89a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a$\xA4Wa$\xA4a}\xCBV[\x90P`\x01\x81` \x81\x11\x15a$\xBAWa$\xBAa}\xCBV[\x03a%~W`\0a$\xCE\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a$\xDB\x91\x90a\x80\xCBV[\x90Pa$\xEB\x81`\0\x01Q\x86aV\xFDV[\x80Qa$\xF6\x90aW[V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%`W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%tW=`\0\x80>=`\0\xFD[PPPPPa.\xB4V[`\x02\x81` \x81\x11\x15a%\x92Wa%\x92a}\xCBV[\x03a&6W`\0a%\xA6\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a%\xB3\x91\x90ax;V[\x90Pa%\xC3\x81`\0\x01Q\x86aV\xFDV[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA6T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a%FV[`\x07\x81` \x81\x11\x15a&JWa&Ja}\xCBV[\x03a&\xB8W`\x9AT`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a&\x81\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&\xAFW=`\0\x80>=`\0\xFD[PPPPa.\xB4V[`\t\x81` \x81\x11\x15a&\xCCWa&\xCCa}\xCBV[\x03a(\\W`\0a&\xE0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a&\xED\x91\x90a\x85\x84V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a'\x1F\x90`\0\x90`\x04\x01a\x82\x9AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'`\x91\x90a}\xAEV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xDC\x91\x90a}\xAEV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a(\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P\x80Qa(,\x90\x86aV\xFDV[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a%F\x90\x84\x90`\x04\x01ap\xD2V[`\n\x81` \x81\x11\x15a(pWa(pa}\xCBV[\x03a(\xD1W`\0a(\x84\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a(\x91\x91\x90ax;V[\x90Pa(\xA1\x81`\0\x01Q\x86aV\xFDV[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a%F\x90\x84\x90`\x04\x01aa\x15V[`\x0B\x81` \x81\x11\x15a(\xE5Wa(\xE5a}\xCBV[\x03a)}W`\0a(\xF9\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a)\x06\x91\x90az\x9BV[\x90Pa)\x16\x81`\0\x01Q\x86aV\xFDV[\x80Qa)!\x90aW\xEAV[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a%FV[`\x12\x81` \x81\x11\x15a)\x91Wa)\x91a}\xCBV[\x03a)\xE7W`\0a)\xA5\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a)\xB2\x91\x90a\x85\xA0V[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a%F\x91`\x04\x01aq\xF1V[`\x13\x81` \x81\x11\x15a)\xFBWa)\xFBa}\xCBV[\x03a*jW`\0a*\x0F\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a*\x1C\x91\x90aw\xC4V[\x90Pa*,\x81`\0\x01Q\x86aV\xFDV[\x80Qa*7\x90aW\xEAV[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua.\xB4V[`\x15\x81` \x81\x11\x15a*~Wa*~a}\xCBV[\x03a+\x1AW`\0a*\x92\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a*\x9F\x91\x90axWV[\x90Pa*\xAF\x81`\0\x01Q\x86aV\xFDV[a*\xBC\x81``\x01QaW[V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a%FV[`\x1B\x81` \x81\x11\x15a+.Wa+.a}\xCBV[\x03a+tW`\x9AT`\xA6T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a&\x81\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x85\xD4V[`\x1D\x81` \x81\x11\x15a+\x88Wa+\x88a}\xCBV[\x03a+\xBFW`\x9AT`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a&\x81\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\r\x81` \x81\x11\x15a+\xD3Wa+\xD3a}\xCBV[\x03a.ZW`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,(W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,<W=`\0\x80>=`\0\xFD[PPPP`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xBD\x91\x90\x81\x01\x90a\x86\x01V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xDAWa,\xDAae\x8AV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-\x03W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a-\xE6W`\xAA`\0\x84\x83\x81Q\x81\x10a-(Wa-(a|\xAFV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a-jWa-ja|\xAFV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10a-\x96Wa-\x96a|\xAFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a-\xDE\x81a}!V[\x91PPa-\tV[Pa-\xF1`\x01aW\xEAV[`\x9AT`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a.!\x90\x84\x90`\x04\x01a\x86\x9AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.;W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.OW=`\0\x80>=`\0\xFD[PPPPPPa.\xB4V[`\x18\x81` \x81\x11\x15a.nWa.na}\xCBV[\x03a\x05\xF9W`\x9AT`\xA6T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a&\x81\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x85\xD4V[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a/OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a/\xAD\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xD9\x90a\x833V[\x80\x15a0&W\x80`\x1F\x10a/\xFBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0&V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a0`\x92\x90\x91`\x04\x01a\x83gV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a0\x99WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra0\x96\x91\x81\x01\x90a\x86\xADV[`\x01[a!\xD9W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a0\xBBWa0\xBBa|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a0\xD6Wa0\xD6a}\xCBV[\x90P`\0\x81` \x81\x11\x15a0\xECWa0\xECa}\xCBV[\x03a1>W`\0a1\0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a1\r\x91\x90a\x87gV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa1#\x91aXHV[\x80QQa18\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[Pa1\x92V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x15\x12V[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a1\xAB\x83a}\x88V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a1\xDDa_uV[a\x11w\x82a\x88\x10V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra2Q\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra2/\x90a\x88\x1CV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x14\xC4V[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x85\x84V[a2\x9Fa_\xA2V[a\x11w\x82a\x88@V[`\0b\xFF\xFF\xFF\x82\x16biso\x14a2\xD6W`\0\x82\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x11wV[`\xB0T`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xAB\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3I\x91\x90a\x83\x89V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11w6\x83\x90\x03\x83\x01\x83a\x88\x97V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta3\xA0\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3\xCC\x90a\x833V[\x80\x15a4\x19W\x80`\x1F\x10a3\xEEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\x19V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x88\xDBV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a4\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x12V[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a5\x0C\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a5x\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x89rV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x89\xB8V[a6Fa_\xE1V[a\x11w\x82a\x89\xD4V[`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a\x8A\xD2V[a6\x90a_\xA2V[a\x11w\x82a\x8B'V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11w\x82a\x8BrV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11w\x82a\x8C5V[a6\xE3a`\tV[a\x11w\x82a\x8C\x94V[`\0\x80\x83\x83`\0\x81\x81\x10a7\x02Wa7\x02a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\x1DWa7\x1Da}\xCBV[\x90P`\x13\x81` \x81\x11\x15a73Wa73a}\xCBV[\x03a7~W`\0a7G\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7T\x91\x90aw\xC4V[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a7rW`\0a7uV[\x80Q[\x92PPPa!\xD9V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a7\x9FWa7\x9Fa|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\xBAWa7\xBAa}\xCBV[\x90P`\t\x81` \x81\x11\x15a7\xD0Wa7\xD0a}\xCBV[\x03a7\xFFW`\0a7\xE4\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90a\x85\x84V[` \x01Q\x92Pa!\xD9\x91PPV[`\n\x81` \x81\x11\x15a8\x13Wa8\x13a}\xCBV[\x03a84W`\0a8'\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90ax;V[`\x0B\x81` \x81\x11\x15a8HWa8Ha}\xCBV[\x03a7~W`\0a8\\\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a7\xF1\x91\x90az\x9BV[a8qa_\xA2V[a\x11w\x82a\x8D>V[a8\x82a`(V[a\x11w\x82a\x8D\x9AV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11w6\x83\x90\x03\x83\x01\x83a\x8EGV[`\0\x82\x82`\0\x81\x81\x10a8\xE1Wa8\xE1a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a8\xFCWa8\xFCa}\xCBV[\x90P3`\x01\x82` \x81\x11\x15a9\x13Wa9\x13a}\xCBV[\x03a9\x1DW`\0\x80\xFD[`\x07\x82` \x81\x11\x15a91Wa91a}\xCBV[\x03a9wW`\0a9E\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a9R\x91\x90a\x88\x97V[\x90Pa9qa9_aY\x04V[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aRYV[Pa;rV[`\x12\x82` \x81\x11\x15a9\x8BWa9\x8Ba}\xCBV[\x03a9\xBFW`gT`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a9\xBAW`\0\x80\xFD[a;rV[`\x15\x82` \x81\x11\x15a9\xD3Wa9\xD3a}\xCBV[\x03a:yW`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a:/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P`\0a:?\x84`\x01\x81\x88a\x84\xA0V[\x81\x01\x90a:L\x91\x90axWV[\x90Pa9q\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaR\x04V[`\x1B\x82` \x81\x11\x15a:\x8DWa:\x8Da}\xCBV[\x03a:\xA3W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\x1D\x82` \x81\x11\x15a:\xB7Wa:\xB7a}\xCBV[\x03a:\xCDW`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\r\x82` \x81\x11\x15a:\xE1Wa:\xE1a}\xCBV[\x03a:\xF7W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[`\x18\x82` \x81\x11\x15a;\x0BWa;\x0Ba}\xCBV[\x03a;!W`gT`\x01`\x01`\xA0\x1B\x03\x16a9\x9DV[a;/a;,aY\x04V[PV[`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a;J\x90\x84\x90`\x0F\x0Ba\x8EcV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a;\xBFBb\x03\xF4\x80a}:V[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA8\x93P\x90a<(\x82a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a<\x94\x92`\x01\x85\x01\x92\x90\x91\x01\x90a^\x04V[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra=v\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra=8\x90a\x88\x1CV[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14\xC4\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a>\x07\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta>3\x90a\x833V[\x80\x15a>\x80W\x80`\x1F\x10a>UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a>\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a>cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a>\xA0aVQV[`\x01`\x01`\xA0\x1B\x03\x81\x16a?\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x12V[a;,\x81aV\xABV[`\0\x82\x82`\0\x81\x81\x10a?:Wa?:a|\xAFV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a?UWa?Ua}\xCBV[\x90P`\0\x81` \x81\x11\x15a?kWa?ka}\xCBV[\x03a@\xD8W`\0a?\x7F\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90a?\x8C\x91\x90a\x87gV[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92Pa?\xA2\x91aXHV[a@Y\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a\x93\x80`w\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aYwV[PPV[\x80QQa@e\x90aW\xEAV[\x80QQa@z\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a@\xAA\x91`\x04\x01ar\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$MW=`\0\x80>=`\0\xFD[`\x02\x81` \x81\x11\x15a@\xECWa@\xECa}\xCBV[\x03aB\xA7W`\0aA\0\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aA\r\x91\x90a\x8E\xC4V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaA#\x91aXHV[aA\xA8\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a\x94\x90`O\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01RaB2\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB'\x91\x90a\x8E\xF8V[\x83Q` \x01QaY\xC5V[`\x9AT\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA6T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a@\xAAV[`\x03\x81` \x81\x11\x15aB\xBBWaB\xBBa}\xCBV[\x03aC\xBFW`\0aB\xCF\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aB\xDC\x91\x90a\x8F\x15V[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15aC(W` \x82\x01Q\x83QaC#\x91\x90a\x8FIV[aC+V[`\0[`\x9BT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCzW`\0\x80\xFD[PZ\xF1\x15\x80\x15aC\x8EW=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa2Q\x91PPV[`\x0F\x81` \x81\x11\x15aC\xD3WaC\xD3a}\xCBV[\x03aD\xD4W`\0aC\xE7\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aC\xF4\x91\x90a\x8F\x15V[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15aD<W\x81Q\x83QaD7\x91\x90a\x8FIV[aD?V[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91aDx\x91\x85\x91\x90`\x04\x01a\x8FqV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15aD\xA6W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa2Q\x91PPV[`\x04\x81` \x81\x11\x15aD\xE8WaD\xE8a}\xCBV[\x03aE\xADW`\x9AT`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90aE$\x90\x88\x90\x88\x90`\x04\x01a\x84\xF3V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEf\x91\x90a\x8F\x93V[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a=vWc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90UPPPPPV[`\x05\x81` \x81\x11\x15aE\xC1WaE\xC1a}\xCBV[\x03aF&W`\x9AT`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F8W=`\0\x80>=`\0\xFD[`\x06\x81` \x81\x11\x15aF:WaF:a}\xCBV[\x14\x80aFWWP`\x16\x81` \x81\x11\x15aFUWaFUa}\xCBV[\x14[\x15aGKW`\0aFk\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aFx\x91\x90a\x8F\xC2V[` \x81\x01QQQ\x90\x91PaF\x8B\x90aW\xEAV[`@\x81\x01QQQaF\x9B\x90aW\xEAV[`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01aF\xC3\x84` \x01Q`\0\x01Q`\0\x01Qa2\xA8V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aF\xE6\x84`@\x01Q`\0\x01Q`\0\x01Qa2\xA8V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R`\xB0T`@Qc\x11\x17\x8F-`\xE3\x1B\x81R\x92\x93P\x16\x90c\x88\xBCyh\x90aG\x1D\x90\x84\x90`\x04\x01a\x8F\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xE5W=`\0\x80>=`\0\xFD[`\x0C\x81` \x81\x11\x15aG_WaG_a}\xCBV[\x03aG\xD7W`\0aGs\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aG\x80\x91\x90a\x90AV[``\x81\x01QQQ\x90\x91PaG\x93\x90aW\xEAV[`\xB0T``\x82\x01QQQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xF0\xD3\xCE\x90\x83\x90aG\xBA\x90a2\xA8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xAA\x92\x91\x90a\x90uV[`\x08\x81` \x81\x11\x15aG\xEBWaG\xEBa}\xCBV[\x03aH\x89W`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaH3\x81`\x01aS\xB2V[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81` \x81\x11\x15aH\x9DWaH\x9Da}\xCBV[\x03aI\xBEW`\0aH\xB1\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aH\xBE\x91\x90a\x90\xA0V[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaH\xD4\x91aXHV[aIy\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\xA0\x01`@R\x80`v\x81R` \x01a\x94\x1A`v\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93R\x90\x82\x16\x92\x85\x01\x92\x90\x92R\x16`\xC0\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01a@:V[\x80QQaI\x8E\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91a@\xAA\x91`\x04\x01ap\xD2V[`\n\x81` \x81\x11\x15aI\xD2WaI\xD2a}\xCBV[\x03aJ\xD3W`\0aI\xE6\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aI\xF3\x91\x90a\x8E\xC4V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaJ\t\x91aXHV[aJ\x8E\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`C\x81R` \x01a\x93=`C\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80QQaJ\xA3\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91a@\xAA\x91`\x04\x01aa\x15V[`\x1E\x81` \x81\x11\x15aJ\xE7WaJ\xE7a}\xCBV[\x03aLIW`\0aJ\xFB\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aK\x08\x91\x90a\x90\xD4V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaK\x1E\x91aXHV[aK\x8D\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`8\x81R` \x01a\x92\x99`8\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[\x80QQaK\xA2\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x88<q\x85`\xE0\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88<q\x85\x90`\x84\x01a@\xAAV[`\x1F\x81` \x81\x11\x15aL]WaL]a}\xCBV[\x03aM\xBFW`\0aLq\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aL~\x91\x90a\x90\xD4V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaL\x94\x91aXHV[aM\x03\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x92\xD1`6\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[\x80QQaM\x18\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x1C\xD4\x0F_`\xE3\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\xA0z\xF8\x90`\x84\x01a@\xAAV[`\x10\x81` \x81\x11\x15aM\xD3WaM\xD3a}\xCBV[\x03aN\nW`\x9AT`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x13\x81` \x81\x11\x15aN\x1EWaN\x1Ea}\xCBV[\x03aN\xEFW`\0aN2\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aN?\x91\x90a\x91\x08V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaNU\x91aXHV[aN\xBB\x81`\0\x01Q`\0\x01Qa@U`@Q\x80``\x01`@R\x80`6\x81R` \x01a\x93\x07`6\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a@:V[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81` \x81\x11\x15aO\x03WaO\x03a}\xCBV[\x03aO:W`\x9AT`@Qc&fm-`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99\x99\xB4\xB4\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x17\x81` \x81\x11\x15aONWaONa}\xCBV[\x03aPZW`\0aOb\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aOo\x91\x90a\x91<V[\x90PaO\x82\x81`\0\x01Q` \x01QaW[V[aP\x01\x81`\0\x01Q`\0\x01Qa@U`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a\x92N`K\x919\x80Q` \x91\x82\x01 \x85Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`@\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a@:V[\x80Q\x80Q``\x90\x91\x01QaP\x15\x91\x90aXHV[\x80QQaP*\x90g\r\xE0\xB6\xB3\xA7d\0\0aX\xF8V[`\x9AT\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a@\xAA\x91`\x04\x01acyV[`\x19\x81` \x81\x11\x15aPnWaPna}\xCBV[\x03aP\xA5W`\x9AT`@QcJg\xD9\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\xCF\xB2\x02\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x1A\x81` \x81\x11\x15aP\xB9WaP\xB9a}\xCBV[\x03aP\xF0W`\x9AT`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x1C\x81` \x81\x11\x15aQ\x04WaQ\x04a}\xCBV[\x03aQ\xB9W`\0aQ\x18\x83`\x01\x81\x87a\x84\xA0V[\x81\x01\x90aQ%\x91\x90a\x91pV[`\xB0T\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA2z%\n\x90\x84\x90aQN\x90a2\xA8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQk\x92\x91\x90a\x91\xA4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aQ\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xAE\x91\x90a\x83\x89V[\x90Pa=v\x81aW[V[` \x81` \x81\x11\x15aQ\xCDWaQ\xCDa}\xCBV[\x03a\x05\xF9W`\x9AT`@Qc~\x92v\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c~\x92v\xD7\x90aE\xF8\x90\x86\x90\x86\x90`\x04\x01a\x84\xF3V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaR'\x90a\x833V[\x90P`\0\x03a@UW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa2Q\x92\x84\x01\x90a^\x04V[`\x01`\x01`\xA0\x1B\x03\x83\x16aRlW`\0\x80\xFD[`\x9ATa2Q\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14a@UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`\0Ta\x01\0\x90\x04`\xFF\x16aS5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a\"WaZ\x9CV[`\0Ta\x01\0\x90\x04`\xFF\x16aS\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a@U\x82\x82a[\x10V[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x93\xF7`#\x919\x90aT\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aTe\x90a\x833V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaT\x91\x90a\x833V[\x80\x15aT\xDEW\x80`\x1F\x10aT\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aT\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aT\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aT\xFF\x90a}\x88V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aU<`\x01\x83\x01\x82a]\xCAV[PP\x81\x80aUWWPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aU\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[PFazi\x03aU\xC7W` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aE\xF8\x92`\x04\x01a\x83gV[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aU\xFA\x92\x90\x91`\x04\x01a\x83gV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aV\x14W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aV%WP`\x01[a.\xB4Wb\x03\xD0\x90Z\x11\x15\x80aVEWPaVA`\x02\x82a\x91\xB7V[Z\x11\x15[\x15aVLW\xFE[a.\xB4V[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x15\x12V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aW WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a2QW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a;,W`\xA2\x80T`\0\x90aW\x92\x90`\x01`\x01`@\x1B\x03\x16a}\x88V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[`\x01\x81\x14\x80aX\x0FWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90a@UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x12\x91\x90aq\xF1V[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aXq\x83a}\x88V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a2QWaX\xC2\x81`\x01`\x01`@\x1B\x03\x16a[\x95V[`@Q` \x01aX\xD2\x91\x90a\x91\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x15\x12\x91`\x04\x01aq\xF1V[a@U\x82\x82`\0aY\xC5V[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYNW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aYr\x91\x90a}\xAEV[\x90P\x90V[`\0a\x11waY\x84a\\4V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aY\xE1\x86a\x92\x1EV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aZ0W`\0\x80\xFD[PZ\xF1\x15\x80\x15aZDW=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aZp\x90\x84\x90`\x0F\x0Ba\x8EcV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a[\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[a\"W3aV\xABV[`\0Ta\x01\0\x90\x04`\xFF\x16a[{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x12V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0a[\xA2\x83a\\\xAFV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a[\xC1Wa[\xC1ae\x8AV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a[\xEBW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a[\xF5WP\x93\x92PPPV[`\0aYr\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\\c`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a\\\xF8Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a]$Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a]BWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a]ZWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a]nWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a]\x80W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11wW`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80Ta]\xD6\x90a\x833V[`\0\x82U\x80`\x1F\x10a]\xE6WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a;,\x91\x90a`\\V[\x82\x80Ta^\x10\x90a\x833V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a^2W`\0\x85Ua^xV[\x82`\x1F\x10a^KW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua^xV[\x82\x80\x01`\x01\x01\x85U\x82\x15a^xW\x91\x82\x01[\x82\x81\x11\x15a^xW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a^]V[Pa^\x84\x92\x91Pa`\\V[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01a^\xB1a^\xE5V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a_\x95a^\xE5V[\x81R` \x01a^\xB1a^\xE5V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a]\xBDV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01a_\x95a^\xE5V[`@Q\x80`\xA0\x01`@R\x80a`;a`qV[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15a^\x84W`\0\x81U`\x01\x01a`]V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a`\xB3W`\0\x80\xFD[a!\xD9\x83\x83a`\x8FV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11wV[`\0`\x80\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aa\x0BW`\0\x80\xFD[a!\xD9\x83\x83a`\xE7V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11wV[`\0`\xA0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aa|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aa\x92W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01aaXV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15aa\xC1W\x81\x81\x01Q\x83\x82\x01R` \x01aa\xA9V[\x83\x81\x11\x15a.\xB4WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Raa\xEA\x81` \x86\x01` \x86\x01aa\xA6V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[abB\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01Raa\x9E`\xA0\x85\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84aa\xFEV[`\0`@\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ab\x94W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ab\xAAW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01abpV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ab\xCAV[P\x94\x95\x94PPPPPV[`\0` \x80\x83R``\x83\x01\x84Q`@\x83\x86\x01R\x81\x81Q\x80\x84R`\x80\x87\x01\x91P`\x80\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15acQW`\x7F\x19\x88\x86\x03\x01\x83Rac?\x85\x85Qaa\xD2V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01ac#V[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90acp\x81\x83ab\xB6V[\x95\x94PPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11wV[`\0`@\x82\x84\x03\x12\x15ac\xC8W`\0\x80\xFD[a!\xD9\x83\x83abpV[`\0` \x82\x84\x03\x12\x15ac\xE4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac\xFAW`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a!\xD9W`\0\x80\xFD[`\0a\x01\0\x82Q\x80Q\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R`@\x81\x01Q`\x0F\x0B`@\x86\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x88\x01R\x80`\x80\x84\x01Q\x16`\x80\x88\x01RPP`\xA0\x81\x01Q`\x0F\x0B`\xA0\x86\x01RP` \x83\x01Qadz`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Q\x81`\xE0\x86\x01Racp\x82\x86\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84ad\rV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x11wV[`\0\x80\x83`\x1F\x84\x01\x12ad\xD7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xEEW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15ae\tW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15ae#W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ae9W`\0\x80\xFD[aeE\x85\x82\x86\x01ad\xC5V[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a;,W`\0\x80\xFD[\x805aen\x81aeQV[\x91\x90PV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aenW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ae\xC2Wae\xC2ae\x8AV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15af\x9AWaf\x9Aae\x8AV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12af\xB3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\xCCWaf\xCCae\x8AV[af\xDF`\x1F\x82\x01`\x1F\x19\x16` \x01afrV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15af\xF4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15ag'W`\0\x80\xFD[\x845\x93P` \x85\x015ag9\x81aeQV[\x92PagG`@\x86\x01aesV[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15agbW`\0\x80\xFD[agn\x87\x82\x88\x01af\xA2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ag\x8CW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ag\xA5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xBBW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a`\x8FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ag\xDBV[` \x81R`\0\x82Q``` \x84\x01Rah\x16`\x80\x84\x01\x82ag\xC7V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Rah4\x83\x83ag\xC7V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPacp\x82\x82ag\xC7V[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Raa\x9E``\x84\x01\x82aa\xD2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a;,W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xA7W`\0\x80\xFD[\x815a!\xD9\x81ah\x80V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15ah\xDFW`\0\x80\xFD[ah\xE8\x85ah\xB2V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ai\x03W`\0\x80\xFD[ai\x0F\x87\x82\x88\x01ad\xC5V[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15ai4W`\0\x80\xFD[ai<ae\xA0V[aiE\x83ah\xB2V[\x81RaiS` \x84\x01ah\xB2V[` \x82\x01Raid`@\x84\x01ah\xB2V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ai\x82W`\0\x80\xFD[\x815a!\xD9\x81aeQV[`\0` \x82\x84\x03\x12\x15ai\x9FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xB5W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a`\xE7V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01Raa\x9E`\xC0\x85\x01\x82aa\xD2V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01Raa\x9E`\x80\x85\x01\x82ai\xC1V[` \x81R`\0a!\xD9` \x83\x01\x84aj V[`\0` \x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aj\x96W`\0\x80\xFD[a!\xD9\x83\x83ajrV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ab\xE6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aj\xB4V[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rak\x04`\xE0\x85\x01\x82aj\xA0V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rak!\x82\x82ab\xB6V[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90acp\x81\x83aa\xD2V[` \x81R`\0\x82Q`@` \x84\x01Rakp``\x84\x01\x82ab\xB6V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Racp\x82\x82ag\xC7V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15ak\xA6Wak\xA6ae\x8AV[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a;,W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ak\xD0W`\0\x80\xFD[\x815` ak\xE5ak\xE0\x83ak\x8DV[afrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15al\x04W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805al\x1B\x81ak\xB0V[\x83R\x91\x83\x01\x91\x83\x01al\x08V[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15alLW`\0\x80\xFD[\x865alW\x81ah\x80V[\x95P` \x87\x015alg\x81ah\x80V[\x94P`@\x87\x015alw\x81ah\x80V[\x93P``\x87\x015al\x87\x81ah\x80V[\x92P`\x80\x87\x015al\x97\x81ah\x80V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15al\xB2W`\0\x80\xFD[al\xBE\x89\x82\x8A\x01ak\xBFV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01Raa\x9E`@\x85\x01\x82ag\xC7V[` \x81R`\0a!\xD9` \x83\x01\x84al\xCBV[`\0`\xC0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15am*W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15am@W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01am\x06V[` \x81R`\0a!\xD9` \x83\x01\x84ai\xC1V[\x805\x80\x15\x15\x81\x14aenW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15am\x82W`\0\x80\xFD[\x825am\x8D\x81ah\x80V[\x91Pam\x9B` \x84\x01am_V[\x90P\x92P\x92\x90PV[` \x81Ram\xE9` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Raa\x9E`\xC0\x84\x01\x82aa\xD2V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11wV[\x805`\xFF\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15an`W`\0\x80\xFD[ani\x87ah\xB2V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15an\x84W`\0\x80\xFD[an\x90\x89\x82\x8A\x01ad\xC5V[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Pan\xB0`\x80\x88\x01an6V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Ran\xEB` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Raa\x9E`\xA0\x84\x01\x82aa\xD2V[`\0\x80\x83`\x1F\x84\x01\x12ao\x17W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ao.W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15ae\tW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15ao[W`\0\x80\xFD[\x835aof\x81ah\x80V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ao\x81W`\0\x80\xFD[ao\x8D\x86\x82\x87\x01ao\x05V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15ao\xADW`\0\x80\xFD[\x825ao\xB8\x81aeQV[\x91P` \x83\x015ao\xC8\x81ak\xB0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15ao\xE6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xFCW`\0\x80\xFD[aeE\x85\x82\x86\x01ao\x05V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rap-``\x85\x01\x82ai\xC1V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Racp\x82\x82ai\xC1V[` \x81R`\0a!\xD9` \x83\x01\x84ap\x08V[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14aenW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15ap\x86W`\0\x80\xFD[ap\x8F\x84apYV[\x92P` \x84\x015ap\x9F\x81aeQV[\x91Pap\xAD`@\x85\x01aesV[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ap\xC8W`\0\x80\xFD[a!\xD9\x83\x83am\x06V[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x1AxW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aqYW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aqoW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01aq5V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Raa\x9Ea\x01\0\x84\x01\x82aa\xD2V[` \x81R`\0a!\xD9` \x83\x01\x84aa\xD2V[`\0`\xE0\x82\x84\x03\x12\x15ar\x16W`\0\x80\xFD[a!\xD9\x83\x83aq5V[`\0\x80`@\x83\x85\x03\x12\x15ar3W`\0\x80\xFD[ar<\x83ah\xB2V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15arXW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15arlW`\0\x80\xFD[artae\xA0V[ar}\x83ah\xB2V[\x81R` \x83\x015ar\x8D\x81ah\x80V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ar\xA4W`\0\x80\xFD[ar\xB0\x88\x82\x86\x01af\xA2V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01RasI`\xC0\x85\x01\x82aj\xA0V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Racp\x81\x83aa\xD2V[as\xAC\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xA0``\x85\x01Ras\xC7`\xA0\x85\x01\x82aa\xD2V[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a!\xD9` \x83\x01\x84as{V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Ratk``\x85\x01\x82ab\xB6V[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15al(W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90at\x8CV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Raa\x9E`\x80\x84\x01\x82ab\xB6V[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Rau\x1C`\xA0\x84\x01\x82ai\xC1V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Racp\x82\x82ai\xC1V[` \x81Raq\xD6` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15au\xCFW\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01au\xB0V[PPP\x83\x01Q`\xE0`\x80\x84\x01Rau\xEAa\x01\0\x84\x01\x82aa\xD2V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Rav\x08\x83\x83ab\xB6V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPav&\x82\x82ab\xB6V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x11w\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15av\xADW`\0\x80\xFD[av\xB6\x86apYV[\x94P` \x86\x015av\xC6\x81aeQV[\x93Pav\xD4`@\x87\x01aesV[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15av\xEFW`\0\x80\xFD[av\xFB\x88\x82\x89\x01ao\x05V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aw\x1EW`\0\x80\xFD[a!\xD9\x82ah\xB2V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rawj`\xC0\x85\x01\x82aa\xD2V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15aw\x95W`\0\x80\xFD[aw\x9Dae\xA0V[\x90P\x815\x81R` \x82\x015` \x82\x01Raw\xB9`@\x83\x01ah\xB2V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15aw\xD6W`\0\x80\xFD[a!\xD9\x83\x83aw\x83V[`\0`\x80\x82\x84\x03\x12\x15aw\xF2W`\0\x80\xFD[aw\xFAae\xC8V[\x90P\x815\x81R` \x82\x015ax\x0E\x81aeQV[` \x82\x01Rax\x1F`@\x83\x01aesV[`@\x82\x01Rax0``\x83\x01ah\xB2V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15axMW`\0\x80\xFD[a!\xD9\x83\x83aw\xE0V[`\0`\x80\x82\x84\x03\x12\x15axiW`\0\x80\xFD[axqae\xC8V[\x825\x81R` \x83\x015ax\x83\x81aeQV[` \x82\x01Rax\x94`@\x84\x01aesV[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15ax\xBFW`\0\x80\xFD[ax\xC7ae\xEAV[\x90Pax\xD3\x83\x83aw\xE0V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[ax\xFA\x84\x82\x85\x01af\xA2V[` \x83\x01RP\x92\x91PPV[`\0a\x11w6\x83ax\xADV[`\0\x82`\x1F\x83\x01\x12ay#W`\0\x80\xFD[\x815` ay3ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ayRW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805\x83R\x91\x83\x01\x91\x83\x01ayVV[`\0`@\x826\x03\x12\x15ay\x7FW`\0\x80\xFD[ay\x87ae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ay\x9EW`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12ay\xB1W`\0\x80\xFD[\x815` ay\xC1ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15ay\xE0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15az\x18W\x805\x86\x81\x11\x15ay\xFCW`\0\x80\x81\xFD[az\n6\x86\x83\x8B\x01\x01af\xA2V[\x84RP\x91\x83\x01\x91\x83\x01ay\xE4V[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15az/W`\0\x80\xFD[az;6\x85\x89\x01ay\x12V[\x90\x85\x01RP\x91\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15az[W`\0\x80\xFD[azcae\xC8V[\x90P\x815\x81R` \x82\x015` \x82\x01Rax\x1F`@\x83\x01aesV[`\0`\x80\x82\x84\x03\x12\x15az\x91W`\0\x80\xFD[a!\xD9\x83\x83azIV[`\0`\x80\x82\x84\x03\x12\x15az\xADW`\0\x80\xFD[az\xB5ae\xC8V[\x825\x81R` \x83\x015az\xC7\x81aeQV[` \x82\x01R`@\x83\x015az\xDA\x81ak\xB0V[`@\x82\x01R``\x83\x015az\xED\x81ak\xB0V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a{\x0BW`\0\x80\xFD[a{\x13ae\xEAV[a{\x1C\x83aesV[\x81R` \x83\x015a{,\x81ah\x80V[` \x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a{JW`\0\x80\xFD[a{Rae\xEAV[\x825a{]\x81ak\xB0V[\x81R` \x83\x015a{,\x81ak\xB0V[`\0\x81\x83\x03a\x01\0\x81\x12\x15a{\x81W`\0\x80\xFD[a{\x89ae\xA0V[\x91P`\xC0\x81\x12\x15a{\x99W`\0\x80\xFD[Pa{\xA2af\x0CV[\x825\x81R` \x83\x015a{\xB4\x81ak\xB0V[` \x82\x01R`@\x83\x015a{\xC7\x81ak\xB0V[`@\x82\x01Ra{\xD8``\x84\x01ah\xB2V[``\x82\x01Ra{\xE9`\x80\x84\x01ah\xB2V[`\x80\x82\x01R`\xA0\x83\x015a{\xFC\x81ak\xB0V[`\xA0\x82\x01R\x81Ra|\x0F`\xC0\x83\x01aecV[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a|-W`\0\x80\xFD[a|9\x84\x82\x85\x01af\xA2V[`@\x83\x01RP\x92\x91PPV[`\0a\x11w6\x83a{mV[`\0`@\x82\x84\x03\x12\x15a|cW`\0\x80\xFD[a|kae\xEAV[\x90P\x815a|x\x81aeQV[\x81R` \x82\x015a|\x88\x81ak\xB0V[` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a|\xA5W`\0\x80\xFD[a!\xD9\x83\x83a|QV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a|\xDCW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a|\xF6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15ae\tW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a}3Wa}3a}\x0BV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a}\\Wa}\\a}\x0BV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80a}~Wa}~a}\x0BV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a}\xA4Wa}\xA4a}\x0BV[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a}\xC0W`\0\x80\xFD[\x81Qa!\xD9\x81ah\x80V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qa~\x02\x81`\x01\x85\x01` \x87\x01aa\xA6V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x826\x03\x12\x15a~\"W`\0\x80\xFD[a~*ae\xA0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a~AW`\0\x80\xFD[a~M6\x83\x87\x01ak\xBFV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a~cW`\0\x80\xFD[a~o6\x83\x87\x01ak\xBFV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a~\x88W`\0\x80\xFD[Pa|96\x82\x86\x01ak\xBFV[`\0`@\x82\x84\x03\x12\x15a~\xA7W`\0\x80\xFD[a~\xAFae\xEAV[\x90P\x815a~\xBC\x81ah\x80V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0a\x11w6\x83a~\x95V[`\0\x82\x82\x10\x15a~\xF5Wa~\xF5a}\x0BV[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14aenW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x7F\x1EW`\0\x80\xFD[a\x7F&ae\xC8V[\x825a\x7F1\x81ah\x80V[\x81R` \x83\x015a\x7FA\x81aeQV[` \x82\x01Ra\x7FR`@\x84\x01a~\xFAV[`@\x82\x01Raz\xED``\x84\x01a~\xFAV[`\0\x81\x83\x03`\xC0\x81\x12\x15a\x7FvW`\0\x80\xFD[a\x7F~ae\xEAV[\x91P`\xA0\x81\x12\x15a\x7F\x8EW`\0\x80\xFD[Pa\x7F\x97af.V[\x825\x81R` \x83\x015a\x7F\xA9\x81ak\xB0V[` \x82\x01R`@\x83\x015a\x7F\xBC\x81ak\xB0V[`@\x82\x01Ra\x7F\xCD``\x84\x01ah\xB2V[``\x82\x01Ra\x7F\xDE`\x80\x84\x01ah\xB2V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x80\x10W`\0\x80\xFD[a\x80\x18ae\xC8V[\x90P\x815a\x80%\x81aeQV[\x81R` \x82\x015a\x805\x81ak\xB0V[` \x82\x01R`@\x82\x015a\x80H\x81ak\xB0V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x80fW`\0\x80\xFD[a\x80r\x84\x82\x85\x01a\x7FcV[``\x83\x01RP\x92\x91PPV[`\0a\x11w6\x83a\x7F\xFEV[`\0` \x82\x84\x03\x12\x15a\x80\x9CW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x80\xBEWa\x80\xBEae\x8AV[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x80\xDDW`\0\x80\xFD[a\x80\xE5ae\xA0V[\x825\x81R` \x83\x015a\x80\xF7\x81aeQV[` \x82\x01Raid`@\x84\x01aesV[`\0\x82`\x1F\x83\x01\x12a\x81\x19W`\0\x80\xFD[\x815` a\x81)ak\xE0\x83ak\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x81HW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15al(W\x805a\x81_\x81aeQV[\x83R\x91\x83\x01\x91\x83\x01a\x81LV[`\0`@\x826\x03\x12\x15a\x81~W`\0\x80\xFD[a\x81\x86ae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x81\x9DW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15a\x81\xB2W`\0\x80\xFD[a\x81\xBAae\xC8V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x81\xD0W`\0\x80\xFD[a\x81\xDC6\x82\x86\x01a\x81\x08V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x81\xF4W`\0\x80\xFD[a\x82\x006\x82\x86\x01ay\x12V[`@\x83\x01RPa\x82\x12``\x84\x01ah\xB2V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82-W`\0\x80\xFD[Pax\xFA6\x82\x86\x01af\xA2V[`\0`@\x826\x03\x12\x15a\x82LW`\0\x80\xFD[a\x82Tae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x82kW`\0\x80\xFD[a\x82w6\x83\x87\x01ay\x12V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82\x8DW`\0\x80\xFD[Pax\xFA6\x82\x86\x01ak\xBFV[` \x81\x01`\x02\x83\x10a\x82\xBCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a}\xA4Wa}\xA4a}\x0BV[`\0`@\x82\x84\x03\x12\x15a\x82\xEDW`\0\x80\xFD[a\x82\xF5ae\xEAV[\x90Pa\x83\0\x82aesV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x1BW`\0\x80\xFD[ax\xFA\x84\x82\x85\x01ak\xBFV[`\0a\x11w6\x83a\x82\xDBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x83GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AxWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aa\x9E`@\x83\x01\x84aa\xD2V[`\0` \x82\x84\x03\x12\x15a\x83\x9BW`\0\x80\xFD[PQ\x91\x90PV[`\0a\x11w6\x83a\x7FcV[`\0`\xA0\x82\x84\x03\x12\x15a\x83\xC0W`\0\x80\xFD[a\x83\xC8ae\xEAV[\x90Pax\xD3\x83\x83azIV[`\0a\x11w6\x83a\x83\xAEV[`\0``\x82\x84\x03\x12\x15a\x83\xF2W`\0\x80\xFD[a\x83\xFAae\xA0V[\x90P\x815\x81Ra\x84\x0C` \x83\x01aesV[` \x82\x01Raw\xB9`@\x83\x01ah\xB2V[`\0``\x82\x84\x03\x12\x15a\x84/W`\0\x80\xFD[a!\xD9\x83\x83a\x83\xE0V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x84eW`\0\x80\xFD[a\x84mae\xEAV[\x90Pa\x84y\x83\x83aw\x83V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0a\x11w6\x83a\x84SV[`\0\x80\x85\x85\x11\x15a\x84\xB0W`\0\x80\xFD[\x83\x86\x11\x15a\x84\xBDW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0aa\x9E` \x83\x01\x84\x86a\x84\xCAV[`\0`\xC0\x82\x84\x03\x12\x15a\x85\x19W`\0\x80\xFD[a\x85!af\x0CV[\x90P\x815\x81R` \x82\x015a\x855\x81aeQV[` \x82\x01Ra\x85F`@\x83\x01aesV[`@\x82\x01Ra\x85W``\x83\x01aesV[``\x82\x01Ra\x85h`\x80\x83\x01aesV[`\x80\x82\x01Ra\x85y`\xA0\x83\x01ah\xB2V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x85\x96W`\0\x80\xFD[a!\xD9\x83\x83a\x85\x07V[`\0` \x82\x84\x03\x12\x15a\x85\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xC8W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a~\x95V[`@\x81R`\0a\x85\xE8`@\x83\x01\x85\x87a\x84\xCAV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x86\x14W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x86*W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x86;W`\0\x80\xFD[\x80Qa\x86Iak\xE0\x82ak\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x86hW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x86\x8FW\x83Qa\x86\x80\x81aeQV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x86mV[\x97\x96PPPPPPPV[` \x81R`\0a!\xD9` \x83\x01\x84ag\xC7V[`\0` \x82\x84\x03\x12\x15a\x86\xBFW`\0\x80\xFD[\x81Qa!\xD9\x81aeQV[`\0`\xC0\x82\x84\x03\x12\x15a\x86\xDCW`\0\x80\xFD[a\x86\xE4af\x0CV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a\x87\x02\x81aeQV[`@\x82\x01Ra\x87\x13``\x83\x01am_V[``\x82\x01R`\x80\x82\x015a\x85h\x81ak\xB0V[`\0`\xE0\x82\x84\x03\x12\x15a\x878W`\0\x80\xFD[a\x87@ae\xEAV[\x90Pa\x87L\x83\x83a\x86\xCAV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ax\xEEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x87yW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x87\x8FW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x87&V[`\0``\x82\x84\x03\x12\x15a\x87\xADW`\0\x80\xFD[a\x87\xB5ae\xA0V[\x90P\x815a\x87\xC2\x81aeQV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x87\xDEW`\0\x80\xFD[a\x87\xEA\x85\x83\x86\x01a\x7FcV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x88\x03W`\0\x80\xFD[Pa|9\x84\x82\x85\x01a\x7FcV[`\0a\x11w6\x83a\x87\x9BV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1AxW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x11w6\x83a\x87&V[`\0` \x82\x84\x03\x12\x15a\x88^W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x88\x80Wa\x88\x80ae\x8AV[`@R\x90P\x80a\x88\x8F\x83aesV[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x88\xA9W`\0\x80\xFD[a!\xD9\x83\x83a\x88LV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aenW`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x88\xEDW`\0\x80\xFD[a\x88\xF5afPV[\x825\x81R` \x83\x015a\x89\x07\x81aeQV[` \x82\x01R`@\x83\x015a\x89\x1A\x81aeQV[`@\x82\x01Ra\x89+``\x84\x01a\x88\xB3V[``\x82\x01Ra\x89<`\x80\x84\x01a\x88\xB3V[`\x80\x82\x01Ra\x89M`\xA0\x84\x01ah\xB2V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x89fW`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x89\x84W`\0\x80\xFD[a\x89\x8Cae\xA0V[\x825a\x89\x97\x81aeQV[\x81Ra\x89\xA5` \x84\x01aesV[` \x82\x01R`@\x83\x015aid\x81ah\x80V[`\0`\xC0\x82\x84\x03\x12\x15a\x89\xCAW`\0\x80\xFD[a!\xD9\x83\x83a\x86\xCAV[`\0`@\x826\x03\x12\x15a\x89\xE6W`\0\x80\xFD[a\x89\xEEae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8A\x05W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x8A\x1AW`\0\x80\xFD[a\x8A\"ae\xA0V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x8A8W`\0\x80\xFD[a\x8AD6\x82\x86\x01a\x81\x08V[` \x83\x01RPa\x8AV`@\x84\x01ah\xB2V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x82-W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x8A\x83W`\0\x80\xFD[a\x8A\x8Bae\xA0V[\x90Pa\x8A\x97\x83\x83a\x83\xE0V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\xB2W`\0\x80\xFD[a\x8A\xBE\x84\x82\x85\x01af\xA2V[` \x83\x01RP`\x80\x82\x015aw\xB9\x81ak\xB0V[`\0a\x11w6\x83a\x8AqV[`\0`\xC0\x82\x84\x03\x12\x15a\x8A\xF0W`\0\x80\xFD[a\x8A\xF8af\x0CV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x8B\x14`@\x83\x01an6V[`@\x82\x01R``\x82\x015a\x87\x13\x81aeQV[`\0`\xE0\x826\x03\x12\x15a\x8B9W`\0\x80\xFD[a\x8BAae\xEAV[a\x8BK6\x84a\x8A\xDEV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8BfW`\0\x80\xFD[ax\xFA6\x82\x86\x01af\xA2V[`\0`@\x826\x03\x12\x15a\x8B\x84W`\0\x80\xFD[a\x8B\x8Cae\xEAV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8B\xA3W`\0\x80\xFD[a\x8B\xAF6\x83\x87\x01ay\x12V[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x8B\xC6W`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x8B\xD9W`\0\x80\xFD[\x805a\x8B\xE7ak\xE0\x82ak\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x8C\x06W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x8C$W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x8C\x0BV[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x8CGW`\0\x80\xFD[a\x8COae\xA0V[\x825a\x8CZ\x81aeQV[\x81R` \x83\x015a\x8Cj\x81ak\xB0V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8C\x88W`\0\x80\xFD[a|96\x82\x86\x01ay\x12V[`\0`\x80\x826\x03\x12\x15a\x8C\xA6W`\0\x80\xFD[a\x8C\xAEae\xC8V[\x825a\x8C\xB9\x81aeQV[\x81Ra\x8C\xC7` \x84\x01am_V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8C\xE6W`\0\x80\xFD[a\x8C\xF26\x83\x87\x01a\x7FcV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x8D\x0BW`\0\x80\xFD[Pa\x80r6\x82\x86\x01a\x7FcV[`\0`\xE0\x82\x84\x03\x12\x15a\x8D*W`\0\x80\xFD[a\x8D2ae\xEAV[\x90Pa\x87L\x83\x83a\x85\x07V[`\0a\x11w6\x83a\x8D\x18V[`\0\x82`\x1F\x83\x01\x12a\x8D[W`\0\x80\xFD[a\x8Dcae\xA0V[\x80``\x84\x01\x85\x81\x11\x15a\x8DuW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x8D\x8FW\x805\x84R` \x93\x84\x01\x93\x01a\x8DwV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x8D\xACW`\0\x80\xFD[a\x8D\xB4af.V[a\x8D\xBE6\x84a\x8DJV[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8D\xDAW`\0\x80\xFD[a\x8D\xE66\x83\x87\x01af\xA2V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x8D\xFFW`\0\x80\xFD[a\x8E\x0B6\x83\x87\x01ay\x12V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x8E$W`\0\x80\xFD[Pa\x8E16\x82\x86\x01ay\x12V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x8EYW`\0\x80\xFD[a!\xD9\x83\x83a\x8A\xDEV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x8E\x96Wa\x8E\x96a}\x0BV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x8E\xBBWa\x8E\xBBa}\x0BV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8E\xD6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E\xECW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01ax\xADV[`\0` \x82\x84\x03\x12\x15a\x8F\nW`\0\x80\xFD[\x81Qa!\xD9\x81ak\xB0V[`\0` \x82\x84\x03\x12\x15a\x8F'W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F=W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x82\xDBV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x8FiWa\x8Fia}\x0BV[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0aa\x9E`@\x83\x01\x84ag\xC7V[`\0\x80`@\x83\x85\x03\x12\x15a\x8F\xA6W`\0\x80\xFD[\x82Qa\x8F\xB1\x81aeQV[` \x84\x01Q\x90\x92Pao\xC8\x81ak\xB0V[`\0` \x82\x84\x03\x12\x15a\x8F\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xEAW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x87\x9BV[` \x81R`\0\x82Q``` \x84\x01Ra\x90\x12`\x80\x84\x01\x82ap\x08V[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x90SW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90iW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x7F\xFEV[`@\x81R`\0a\x90\x88`@\x83\x01\x85aj V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x90\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\xC8W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x8D\x18V[`\0` \x82\x84\x03\x12\x15a\x90\xE6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\xFCW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x8AqV[`\0` \x82\x84\x03\x12\x15a\x91\x1AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x910W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x84SV[`\0` \x82\x84\x03\x12\x15a\x91NW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x91dW`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a\x83\xAEV[`\0` \x82\x84\x03\x12\x15a\x91\x82W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x91\x98W`\0\x80\xFD[aa\x9E\x84\x82\x85\x01a{mV[`@\x81R`\0a\x90\x88`@\x83\x01\x85ad\rV[`\0\x82a\x91\xD4WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x92\x11\x81`\x19\x85\x01` \x87\x01aa\xA6V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x92DWa\x92Da}\x0BV[`\0\x03\x92\x91PPV\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)MintVlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)BurnVlp(bytes32 sender,uint128 vlpAmount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)BurnLp(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)no slow mode transactions remainingMintLp(bytes32 sender,uint32 productId,uint128 amountBase,uint128 quoteAmountLow,uint128 quoteAmountHigh,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x8F\xE9a\x18m\xBE\xD1\xAC}l\r-\xEC\xEA~^y'{p5\xA3\x1D\x86\xF7\xF0\xA82\x8C\x8B\x0F\xCAdsolcC\0\x08\r\x003";
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
        ///Calls the contract's `assertCode` (0x0e66265b) function
        pub fn assert_code(
            &self,
            p: AssertCode,
        ) -> ::ethers::contract::builders::ContractCall<M, AssertCode> {
            self.0
                .method_hash([14, 102, 38, 91], (p,))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `createIsolatedSubaccount` (0x16cdb690) function
        pub fn create_isolated_subaccount(
            &self,
            p: CreateIsolatedSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, CreateIsolatedSubaccount> {
            self.0
                .method_hash([22, 205, 182, 144], (p,))
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
        ///Calls the contract's `getSequencerFee` (0x4fcfae58) function
        pub fn get_sequencer_fee(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([79, 207, 174, 88], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlots` (0xfab2c469) function
        pub fn get_slots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 178, 196, 105], ())
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
        ///Calls the contract's `incrementSubmissions` (0x22006046) function
        pub fn increment_submissions(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
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
        ///Calls the contract's `rebalanceVlp` (0x15d322e7) function
        pub fn rebalance_vlp(
            &self,
            p: RebalanceVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, RebalanceVlp> {
            self.0
                .method_hash([21, 211, 34, 231], (p,))
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
            p1: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 254, 95, 228], (wallet, p1))
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
        ///Calls the contract's `signedBurnVlp` (0xa0cc630d) function
        pub fn signed_burn_vlp(
            &self,
            p: SignedBurnVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedBurnVlp> {
            self.0
                .method_hash([160, 204, 99, 13], (p,))
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
        ///Calls the contract's `signedMintVlp` (0xa780a4be) function
        pub fn signed_mint_vlp(
            &self,
            p: SignedMintVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedMintVlp> {
            self.0
                .method_hash([167, 128, 164, 190], (p,))
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
        ///Calls the contract's `unsignedBurnVlp` (0x79f12433) function
        pub fn unsigned_burn_vlp(
            &self,
            p: BurnVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, BurnVlp> {
            self.0
                .method_hash([121, 241, 36, 51], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDelistProduct` (0xb3147d17) function
        pub fn unsigned_delist_product(
            &self,
            p: DelistProduct,
        ) -> ::ethers::contract::builders::ContractCall<M, DelistProduct> {
            self.0
                .method_hash([179, 20, 125, 23], (p,))
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
        ///Calls the contract's `unsignedMintVlp` (0x826644f7) function
        pub fn unsigned_mint_vlp(
            &self,
            p: MintVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, MintVlp> {
            self.0
                .method_hash([130, 102, 68, 247], (p,))
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
        ///Calls the contract's `unsignedWithdrawInsurance` (0x14735755) function
        pub fn unsigned_withdraw_insurance(
            &self,
            p: WithdrawInsurance,
        ) -> ::ethers::contract::builders::ContractCall<M, WithdrawInsurance> {
            self.0
                .method_hash([20, 115, 87, 85], (p,))
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
        ///Calls the contract's `updateMinDepositRate` (0x27617997) function
        pub fn update_min_deposit_rate(
            &self,
            p: UpdateMinDepositRate,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateMinDepositRate> {
            self.0
                .method_hash([39, 97, 121, 151], (p,))
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
    ///Container type for all input parameters for the `assertCode` function with signature `assertCode((string[],bytes32[]))` and selector `0x0e66265b`
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
    #[ethcall(name = "assertCode", abi = "assertCode((string[],bytes32[]))")]
    pub struct AssertCodeCall {
        pub p: AssertCode,
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
    ///Container type for all input parameters for the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,int128),uint32,bytes))` and selector `0x16cdb690`
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
        name = "createIsolatedSubaccount",
        abi = "createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,int128),uint32,bytes))"
    )]
    pub struct CreateIsolatedSubaccountCall {
        pub p: CreateIsolatedSubaccount,
    }
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
    ///Container type for all input parameters for the `getSequencerFee` function with signature `getSequencerFee(uint32)` and selector `0x4fcfae58`
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
    #[ethcall(name = "getSequencerFee", abi = "getSequencerFee(uint32)")]
    pub struct GetSequencerFeeCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSlots` function with signature `getSlots()` and selector `0xfab2c469`
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
    #[ethcall(name = "getSlots", abi = "getSlots()")]
    pub struct GetSlotsCall;
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
    ///Container type for all input parameters for the `rebalanceVlp` function with signature `rebalanceVlp((int128,int128))` and selector `0x15d322e7`
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
    #[ethcall(name = "rebalanceVlp", abi = "rebalanceVlp((int128,int128))")]
    pub struct RebalanceVlpCall {
        pub p: RebalanceVlp,
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
        pub p1: bool,
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
    ///Container type for all input parameters for the `signedBurnVlp` function with signature `signedBurnVlp(((bytes32,uint128,uint64),bytes,int128))` and selector `0xa0cc630d`
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
        name = "signedBurnVlp",
        abi = "signedBurnVlp(((bytes32,uint128,uint64),bytes,int128))"
    )]
    pub struct SignedBurnVlpCall {
        pub p: SignedBurnVlp,
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
    ///Container type for all input parameters for the `signedMintVlp` function with signature `signedMintVlp(((bytes32,uint128,uint64),bytes,int128))` and selector `0xa780a4be`
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
        name = "signedMintVlp",
        abi = "signedMintVlp(((bytes32,uint128,uint64),bytes,int128))"
    )]
    pub struct SignedMintVlpCall {
        pub p: SignedMintVlp,
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
    ///Container type for all input parameters for the `unsignedBurnVlp` function with signature `unsignedBurnVlp((bytes32,uint128,uint64))` and selector `0x79f12433`
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
        name = "unsignedBurnVlp",
        abi = "unsignedBurnVlp((bytes32,uint128,uint64))"
    )]
    pub struct UnsignedBurnVlpCall {
        pub p: BurnVlp,
    }
    ///Container type for all input parameters for the `unsignedDelistProduct` function with signature `unsignedDelistProduct((uint32,int128,bytes32[]))` and selector `0xb3147d17`
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
        name = "unsignedDelistProduct",
        abi = "unsignedDelistProduct((uint32,int128,bytes32[]))"
    )]
    pub struct UnsignedDelistProductCall {
        pub p: DelistProduct,
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
    ///Container type for all input parameters for the `unsignedMintVlp` function with signature `unsignedMintVlp((bytes32,uint128,uint64))` and selector `0x826644f7`
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
        name = "unsignedMintVlp",
        abi = "unsignedMintVlp((bytes32,uint128,uint64))"
    )]
    pub struct UnsignedMintVlpCall {
        pub p: MintVlp,
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
    ///Container type for all input parameters for the `unsignedWithdrawInsurance` function with signature `unsignedWithdrawInsurance((uint128,address))` and selector `0x14735755`
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
        name = "unsignedWithdrawInsurance",
        abi = "unsignedWithdrawInsurance((uint128,address))"
    )]
    pub struct UnsignedWithdrawInsuranceCall {
        pub p: WithdrawInsurance,
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
    ///Container type for all input parameters for the `updateMinDepositRate` function with signature `updateMinDepositRate((uint32,int128))` and selector `0x27617997`
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
        name = "updateMinDepositRate",
        abi = "updateMinDepositRate((uint32,int128))"
    )]
    pub struct UpdateMinDepositRateCall {
        pub p: UpdateMinDepositRate,
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
        AssertCode(AssertCodeCall),
        BurnLpAndTransfer(BurnLpAndTransferCall),
        ChainlinkFullReport(ChainlinkFullReportCall),
        ChainlinkReportBlob(ChainlinkReportBlobCall),
        CheckLpAction(CheckLpActionCall),
        CheckLpActionInner(CheckLpActionInnerCall),
        CheckSlowModeTxInner(CheckSlowModeTxInnerCall),
        CheckSlowModeTxLinkSigner(CheckSlowModeTxLinkSignerCall),
        ClaimSequencerFees(ClaimSequencerFeesCall),
        Clearinghouse(ClearinghouseCall),
        CreateIsolatedSubaccount(CreateIsolatedSubaccountCall),
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
        GetSequencerFee(GetSequencerFeeCall),
        GetSlots(GetSlotsCall),
        GetSlowModeTx(GetSlowModeTxCall),
        GetSubaccountById(GetSubaccountByIdCall),
        GetSubaccountId(GetSubaccountIdCall),
        GetTakerSequencerFee(GetTakerSequencerFeeCall),
        GetTime(GetTimeCall),
        GetTimes(GetTimesCall),
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
        RebalanceVlp(RebalanceVlpCall),
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
        SignedBurnVlp(SignedBurnVlpCall),
        SignedCancellation(SignedCancellationCall),
        SignedCancellationProducts(SignedCancellationProductsCall),
        SignedLinkSigner(SignedLinkSignerCall),
        SignedLiquidateSubaccount(SignedLiquidateSubaccountCall),
        SignedMintLp(SignedMintLpCall),
        SignedMintVlp(SignedMintVlpCall),
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
        UnsignedBurnVlp(UnsignedBurnVlpCall),
        UnsignedDelistProduct(UnsignedDelistProductCall),
        UnsignedDepositCollateral(UnsignedDepositCollateralCall),
        UnsignedDepositInsurance(UnsignedDepositInsuranceCall),
        UnsignedLinkSigner(UnsignedLinkSignerCall),
        UnsignedLiquidateSubaccount(UnsignedLiquidateSubaccountCall),
        UnsignedMintLp(UnsignedMintLpCall),
        UnsignedMintVlp(UnsignedMintVlpCall),
        UnsignedTransferQuote(UnsignedTransferQuoteCall),
        UnsignedWithdrawCollateral(UnsignedWithdrawCollateralCall),
        UnsignedWithdrawInsurance(UnsignedWithdrawInsuranceCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpdateMinDepositRate(UpdateMinDepositRateCall),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
    }
    impl ::ethers::core::abi::AbiDecode for EndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssertCodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssertCode(decoded));
            }
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
                <CreateIsolatedSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateIsolatedSubaccount(decoded));
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
            if let Ok(decoded) =
                <GetSequencerFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSequencerFee(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
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
            if let Ok(decoded) = <RebalanceVlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceVlp(decoded));
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
            if let Ok(decoded) = <SignedBurnVlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedBurnVlp(decoded));
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
            if let Ok(decoded) = <SignedMintVlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedMintVlp(decoded));
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
                <UnsignedBurnVlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedBurnVlp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDelistProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDelistProduct(decoded));
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
                <UnsignedMintVlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedMintVlp(decoded));
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
                <UnsignedWithdrawInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedWithdrawInsurance(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) =
                <UpdateMinDepositRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMinDepositRate(decoded));
            }
            if let Ok(decoded) = <UpdatePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePrice(decoded));
            }
            if let Ok(decoded) = <UpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateProduct(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssertCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::CreateIsolatedSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::GetSequencerFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlowModeTx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountById(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTakerSequencerFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTimes(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::RebalanceVlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::SignedBurnVlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::SignedMintVlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::UnsignedBurnVlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedDelistProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::UnsignedMintVlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedTransferQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedWithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedWithdrawInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeRates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMinDepositRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EndpointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLpAndTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkFullReport(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkReportBlob(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckLpAction(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckLpActionInner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxInner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimSequencerFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateIsolatedSubaccount(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::GetSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlowModeTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountById(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTakerSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimes(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::RebalanceVlp(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SignedBurnVlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellationProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMintVlp(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UnsignedBurnVlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDelistProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedMintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedMintVlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedTransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedWithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedWithdrawInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMinDepositRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertCodeCall> for EndpointCalls {
        fn from(value: AssertCodeCall) -> Self {
            Self::AssertCode(value)
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
    impl ::core::convert::From<CreateIsolatedSubaccountCall> for EndpointCalls {
        fn from(value: CreateIsolatedSubaccountCall) -> Self {
            Self::CreateIsolatedSubaccount(value)
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
    impl ::core::convert::From<GetSequencerFeeCall> for EndpointCalls {
        fn from(value: GetSequencerFeeCall) -> Self {
            Self::GetSequencerFee(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for EndpointCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
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
    impl ::core::convert::From<RebalanceVlpCall> for EndpointCalls {
        fn from(value: RebalanceVlpCall) -> Self {
            Self::RebalanceVlp(value)
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
    impl ::core::convert::From<SignedBurnVlpCall> for EndpointCalls {
        fn from(value: SignedBurnVlpCall) -> Self {
            Self::SignedBurnVlp(value)
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
    impl ::core::convert::From<SignedMintVlpCall> for EndpointCalls {
        fn from(value: SignedMintVlpCall) -> Self {
            Self::SignedMintVlp(value)
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
    impl ::core::convert::From<UnsignedBurnVlpCall> for EndpointCalls {
        fn from(value: UnsignedBurnVlpCall) -> Self {
            Self::UnsignedBurnVlp(value)
        }
    }
    impl ::core::convert::From<UnsignedDelistProductCall> for EndpointCalls {
        fn from(value: UnsignedDelistProductCall) -> Self {
            Self::UnsignedDelistProduct(value)
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
    impl ::core::convert::From<UnsignedMintVlpCall> for EndpointCalls {
        fn from(value: UnsignedMintVlpCall) -> Self {
            Self::UnsignedMintVlp(value)
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
    impl ::core::convert::From<UnsignedWithdrawInsuranceCall> for EndpointCalls {
        fn from(value: UnsignedWithdrawInsuranceCall) -> Self {
            Self::UnsignedWithdrawInsurance(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRatesCall> for EndpointCalls {
        fn from(value: UpdateFeeRatesCall) -> Self {
            Self::UpdateFeeRates(value)
        }
    }
    impl ::core::convert::From<UpdateMinDepositRateCall> for EndpointCalls {
        fn from(value: UpdateMinDepositRateCall) -> Self {
            Self::UpdateMinDepositRate(value)
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
    ///Container type for all return fields from the `assertCode` function with signature `assertCode((string[],bytes32[]))` and selector `0x0e66265b`
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
    pub struct AssertCodeReturn(pub AssertCode);
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
    ///Container type for all return fields from the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,int128),uint32,bytes))` and selector `0x16cdb690`
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
    pub struct CreateIsolatedSubaccountReturn(pub CreateIsolatedSubaccount);
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
    ///Container type for all return fields from the `getSequencerFee` function with signature `getSequencerFee(uint32)` and selector `0x4fcfae58`
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
    pub struct GetSequencerFeeReturn(pub i128);
    ///Container type for all return fields from the `getSlots` function with signature `getSlots()` and selector `0xfab2c469`
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
    pub struct GetSlotsReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub n_submissions_slot: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `incrementSubmissions` function with signature `incrementSubmissions()` and selector `0x22006046`
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
    pub struct IncrementSubmissionsReturn(pub u64);
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
    ///Container type for all return fields from the `rebalanceVlp` function with signature `rebalanceVlp((int128,int128))` and selector `0x15d322e7`
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
    pub struct RebalanceVlpReturn(pub RebalanceVlp);
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
    ///Container type for all return fields from the `signedBurnVlp` function with signature `signedBurnVlp(((bytes32,uint128,uint64),bytes,int128))` and selector `0xa0cc630d`
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
    pub struct SignedBurnVlpReturn(pub SignedBurnVlp);
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
    ///Container type for all return fields from the `signedMintVlp` function with signature `signedMintVlp(((bytes32,uint128,uint64),bytes,int128))` and selector `0xa780a4be`
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
    pub struct SignedMintVlpReturn(pub SignedMintVlp);
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
    ///Container type for all return fields from the `unsignedBurnVlp` function with signature `unsignedBurnVlp((bytes32,uint128,uint64))` and selector `0x79f12433`
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
    pub struct UnsignedBurnVlpReturn(pub BurnVlp);
    ///Container type for all return fields from the `unsignedDelistProduct` function with signature `unsignedDelistProduct((uint32,int128,bytes32[]))` and selector `0xb3147d17`
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
    pub struct UnsignedDelistProductReturn(pub DelistProduct);
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
    ///Container type for all return fields from the `unsignedMintVlp` function with signature `unsignedMintVlp((bytes32,uint128,uint64))` and selector `0x826644f7`
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
    pub struct UnsignedMintVlpReturn(pub MintVlp);
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
    ///Container type for all return fields from the `unsignedWithdrawInsurance` function with signature `unsignedWithdrawInsurance((uint128,address))` and selector `0x14735755`
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
    pub struct UnsignedWithdrawInsuranceReturn(pub WithdrawInsurance);
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
    ///Container type for all return fields from the `updateMinDepositRate` function with signature `updateMinDepositRate((uint32,int128))` and selector `0x27617997`
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
    pub struct UpdateMinDepositRateReturn(pub UpdateMinDepositRate);
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub raw_rs: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
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
    ///`AssertCode(string[],bytes32[])`
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
    pub struct AssertCode {
        pub contract_names: ::std::vec::Vec<::std::string::String>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub code_hashes: ::std::vec::Vec<[u8; 32]>,
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
    ///`BurnVlp(bytes32,uint128,uint64)`
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
    pub struct BurnVlp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub vlp_amount: u128,
        pub nonce: u64,
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
    ///`CreateIsolatedSubaccount((bytes32,int128,int128,uint64,uint64,int128),uint32,bytes)`
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
    pub struct CreateIsolatedSubaccount {
        pub order: IsolatedOrder,
        pub product_id: u32,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`DelistProduct(uint32,int128,bytes32[])`
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
    pub struct DelistProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
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
    ///`IsolatedOrder(bytes32,int128,int128,uint64,uint64,int128)`
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
    pub struct IsolatedOrder {
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub margin: i128,
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
    ///`MintVlp(bytes32,uint128,uint64)`
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
    pub struct MintVlp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub quote_amount: u128,
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
    ///`RebalanceVlp(int128,int128)`
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
    pub struct RebalanceVlp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub vlp_amount: i128,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
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
    ///`SignedBurnVlp((bytes32,uint128,uint64),bytes,int128)`
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
    pub struct SignedBurnVlp {
        pub tx: BurnVlp,
        pub signature: ::ethers::core::types::Bytes,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
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
    ///`SignedMintVlp((bytes32,uint128,uint64),bytes,int128)`
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
    pub struct SignedMintVlp {
        pub tx: MintVlp,
        pub signature: ::ethers::core::types::Bytes,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
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
    ///`UpdateMinDepositRate(uint32,int128)`
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
    pub struct UpdateMinDepositRate {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_deposit_rate_x18: i128,
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
    ///`WithdrawInsurance(uint128,address)`
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
    pub struct WithdrawInsurance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
}
