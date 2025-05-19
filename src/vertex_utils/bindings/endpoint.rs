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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                    ::std::borrow::ToOwned::to_owned("setVertexGateway"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setVertexGateway"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_vertexGateway"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pa\x91\x1C\x80b\0\0\"`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x06\x04W`\x005`\xE0\x1C\x80c\x82fD\xF7\x11a\x03\x19W\x80c\xB1\xFB\xD6\x0B\x11a\x01\xA7W\x80c\xDBZP!\x11a\0\xF9W\x80c\xEERU&\x11a\0\xA2W\x80c\xF8\x0F|\xE5\x11a\0|W\x80c\xF8\x0F|\xE5\x14a\r~W\x80c\xFA\xB2\xC4i\x14a\x11sW\x80c\xFB\xF4\x19\x84\x14a\x10\x08W`\0\x80\xFD[\x80c\xEERU&\x14a\x11\x15W\x80c\xEFd\xED\x0E\x14a\x117W\x80c\xF2\xFD\xE3\x8B\x14a\x11`W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x10\x9CW\x80c\xE9\xABw\xE5\x14a\x10\xAFW\x80c\xE9\xBCtb\x14a\x11\x02W`\0\x80\xFD[\x80c\xDBZP!\x14a\x10\x16W\x80c\xDCB\xE6\x1B\x14a\x106W\x80c\xDF~h\xE1\x14a\x10VW`\0\x80\xFD[\x80c\xBC\x85\xCA\x86\x11a\x01[W\x80c\xC5\x105\x9F\x11a\x015W\x80c\xC5\x105\x9F\x14a\x0F\xE1W\x80c\xD3\x8C;\x9C\x14a\x0F\xE8W\x80c\xD4\xDE\x8D\x9D\x14a\x10\x08W`\0\x80\xFD[\x80c\xBC\x85\xCA\x86\x14a\x062W\x80c\xC3ES\x0B\x14a\x0F\xBDW\x80c\xC4\xF9\xB2_\x14a\x0F\xD0W`\0\x80\xFD[\x80c\xB3\x14}\x17\x11a\x01\x8CW\x80c\xB3\x14}\x17\x14a\x0FjW\x80c\xB3d\x88\xB8\x14a\x0F\x8AW\x80c\xB7\x0E\xB2c\x14a\x0F\xAAW`\0\x80\xFD[\x80c\xB1\xFB\xD6\x0B\x14a\x0F*W\x80c\xB2\xBBcg\x14a\x0FJW`\0\x80\xFD[\x80c\x91q\xD0\x8B\x11a\x02kW\x80c\x9A\x08\xE55\x11a\x02\x14W\x80c\xA0\xCCc\r\x11a\x01\xEEW\x80c\xA0\xCCc\r\x14a\x0F\nW\x80c\xA7\x80\xA4\xBE\x14a\x0F\nW\x80c\xB1\xC8\xEC+\x14a\x0B\x1DW`\0\x80\xFD[\x80c\x9A\x08\xE55\x14a\x0E}W\x80c\x9E\x85\x14$\x14a\x0E\xCAW\x80c\xA0\x82\xC5\xAA\x14a\x0E\xEAW`\0\x80\xFD[\x80c\x954\xDD>\x11a\x02EW\x80c\x954\xDD>\x14a\r\xA9W\x80c\x96\xC4|o\x14a\r\xC9W\x80c\x98\xCD2\xFE\x14a\x0EjW`\0\x80\xFD[\x80c\x91q\xD0\x8B\x14a\rKW\x80c\x91\xC1\xE3\xD7\x14a\rkW\x80c\x94\xFA\xEF\xE5\x14a\r~W`\0\x80\xFD[\x80c\x8D\n\xCC\x9B\x11a\x02\xCDW\x80c\x8E]X\x8C\x11a\x02\xA7W\x80c\x8E]X\x8C\x14a\r\x07W\x80c\x8FO\x8E\xCC\x14a\r\x1AW\x80c\x91\x0E`j\x14a\r+W`\0\x80\xFD[\x80c\x8D\n\xCC\x9B\x14a\x0C\xC3W\x80c\x8D< \xB1\x14a\x0C\xD6W\x80c\x8D\xA5\xCB[\x14a\x0C\xF6W`\0\x80\xFD[\x80c\x872C8\x11a\x02\xFEW\x80c\x872C8\x14a\x0COW\x80c\x8C=/t\x14a\x0CbW\x80c\x8CX\xE1\n\x14a\x0C\x7FW`\0\x80\xFD[\x80c\x82fD\xF7\x14a\x0B\xFCW\x80c\x85\xC8>\x9D\x14a\x0C/W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04\x96W\x80cZ\0\x92;\x11a\x03\xE8W\x80cl\xFE_\xE4\x11a\x03\x91W\x80cqP\x18\xA6\x11a\x03kW\x80cqP\x18\xA6\x14a\x0B\xF4W\x80cy\xF1$3\x14a\x0B\xFCW\x80c}\xB6\xA2[\x14a\x0C\x1CW`\0\x80\xFD[\x80cl\xFE_\xE4\x14a\x0B\x8EW\x80co;\nr\x14a\x0B\xC1W\x80cp\xBEE|\x14a\x0B\xE1W`\0\x80\xFD[\x80ca\x0B.^\x11a\x03\xC2W\x80ca\x0B.^\x14a\x06\xA7W\x80ce\xDD\x13f\x14a\x0BfW\x80clEup\x14a\x0BnW`\0\x80\xFD[\x80cZ\0\x92;\x14a\x0B\x1DW\x80c[\xB4\xC1&\x14a\x0B=W\x80c]O_\x97\x14a\x0BSW`\0\x80\xFD[\x80c>\xDF,[\x11a\x04JW\x80cO\xCF\xAEX\x11a\x04$W\x80cO\xCF\xAEX\x14a\n\xC1W\x80cTDV\x9D\x14a\n\xEAW\x80cU~\xD1\xBA\x14a\n\xFDW`\0\x80\xFD[\x80c>\xDF,[\x14a\n\\W\x80cB\xC7M\x1D\x14a\n|W\x80cM\x96\xA9\n\x14a\n\x9CW`\0\x80\xFD[\x80c6\xB9\x0CQ\x11a\x04{W\x80c6\xB9\x0CQ\x14a\t\xD6W\x80c8B\xE7^\x14a\t\xF6W\x80c<\xECK\x93\x14a\n\x18W`\0\x80\xFD[\x80c5c\x9AO\x14a\tPW\x80c6\x8EF\x86\x14a\t\xB0W`\0\x80\xFD[\x80c\x1D\x9E\xED\xA5\x11a\x05ZW\x80c'ay\x97\x11a\x05\x03W\x80c-\x035\xAB\x11a\x04\xDDW\x80c-\x035\xAB\x14a\x08\xAAW\x80c/\x9A'D\x14a\x08\xDCW\x80c2\x16\xC0b\x14a\x08\xEFW`\0\x80\xFD[\x80c'ay\x97\x14a\x07\xE9W\x80c,\x8Co\xFB\x14a\x08jW\x80c,\xD7\x1B\x16\x14a\x08\x8AW`\0\x80\xFD[\x80c\"\0`F\x11a\x054W\x80c\"\0`F\x14a\x08&W\x80c\"\x1F\t9\x14a\x08.W\x80c\"\xD4\xA8-\x14a\x08AW`\0\x80\xFD[\x80c\x1D\x9E\xED\xA5\x14a\x07\xE9W\x80c\x1F\x18k'\x14a\x08\tW\x80c!\x04u\x89\x14a\x08\x1EW`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05\xBCW\x80c\x16\xCD\xB6\x90\x11a\x05\x96W\x80c\x16\xCD\xB6\x90\x14a\x07\x9EW\x80c\x18\xED\x16\xEB\x14a\x07\xBEW\x80c\x1D\x97\xD2/\x14a\x06\xE7W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06\xE7W\x80c\x0FKP\x9D\x14a\x07\x07W\x80c\x14sWU\x14a\x07^W`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05\xEDW\x80c\x07H\xA2\x19\x14a\x06RW\x80c\rU\xE2k\x14a\x06\xA7W\x80c\x0Ef&[\x14a\x06\xC7W`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x06\tW\x80c\x06\xC0\xBA\xFD\x14a\x062W[`\0\x80\xFD[a\x06\x1Ca\x06\x176`\x04a^\xE7V[a\x11zV[`@Qa\x06)\x91\x90a_\x03V[`@Q\x80\x91\x03\x90\xF3[a\x06Ea\x06@6`\x04a_?V[a\x11\xACV[`@Qa\x06)\x91\x90a_[V[a\x06ea\x06`6`\x04a_?V[a\x11\xDFV[`@Qa\x06)\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06\xBAa\x06\xB56`\x04a_\xB0V[a\x12\x12V[`@Qa\x06)\x91\x90a`\xA3V[a\x06\xDAa\x06\xD56`\x04a`\xC8V[a\x12#V[`@Qa\x06)\x91\x90aa7V[a\x06\xFAa\x06\xF56`\x04a_?V[a\x12@V[`@Qa\x06)\x91\x90aa\xBFV[a\x07\x1Aa\x07\x156`\x04a_?V[a\x12sV[`@Qa\x06)\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x07qa\x07l6`\x04aa\xFCV[a\x12\xA6V[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x06)V[a\x07\xB1a\x07\xAC6`\x04ab\x18V[a\x12\xC9V[`@Qa\x06)\x91\x90ab\xD8V[`\xA6Ta\x07\xD1\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\x07\xFCa\x07\xF76`\x04aa\xFCV[a\x13\x1CV[`@Qa\x06)\x91\x90ab\xEBV[a\x08\x1Ca\x08\x176`\x04acVV[a\x13?V[\0[a\x08\x1Ca\x13\xDBV[a\x07\xD1a\x14\x8BV[a\x08\x1Ca\x08<6`\x04aeWV[a\x14\xD0V[a\x07\xD1a\x08O6`\x04ae\xC0V[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08}a\x08x6`\x04ae\xD9V[a\x18\xA7V[`@Qa\x06)\x91\x90af@V[a\x08\x9Da\x08\x986`\x04a`\xC8V[a\x18\xD4V[`@Qa\x06)\x91\x90af\x98V[a\x07\xD1a\x08\xB86`\x04af\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08\x1Ca\x08\xEA6`\x04ag\x0FV[a\x18\xF2V[a\x08\x1Ca\x08\xFD6`\x04aghV[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\tca\t^6`\x04a_?V[a\x1ADV[`@Qa\x06)\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\t\xC3a\t\xBE6`\x04ag\xB6V[a\x1AwV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x06)V[a\t\xE9a\t\xE46`\x04ag\xD3V[a\x1A\xD2V[`@Qa\x06)\x91\x90ah\xA5V[a\n\ta\n\x046`\x04ah\xCAV[a\x1A\xE3V[`@Q\x90Q\x81R` \x01a\x06)V[a\n+a\n&6`\x04a^\xE7V[a\x1B\x01V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x06)V[a\noa\nj6`\x04a`\xC8V[a\x1B-V[`@Qa\x06)\x91\x90ai\x1CV[a\n\x8Fa\n\x8A6`\x04a`\xC8V[a\x1B>V[`@Qa\x06)\x91\x90ai\x9AV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\t\xC3a\n\xCF6`\x04ag\xB6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xAA` R`@\x90 T`\x0F\x0B\x90V[a\x08\x1Ca\n\xF86`\x04ajyV[a\x1B[V[a\x0B\x05a\x1F\x95V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\x0B0a\x0B+6`\x04a`\xC8V[a #V[`@Qa\x06)\x91\x90ak9V[a\x0BEa AV[`@Q\x90\x81R` \x01a\x06)V[`\x9ATa\n\xA9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08\x1Ca\"4V[a\x0B\x81a\x0B|6`\x04ak^V[a\"wV[`@Qa\x06)\x91\x90ak\x92V[a\x08\x1Ca\x0B\x9C6`\x04ak\xB5V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x0B\xD4a\x0B\xCF6`\x04a_\xB0V[a\"\x88V[`@Qa\x06)\x91\x90ak\xEAV[a\x08\x1Ca\x0B\xEF6`\x04af\xDBV[a\"\x99V[a\x08\x1Ca#\x1CV[a\x0C\x0Fa\x0C\n6`\x04a^\xE7V[a#0V[`@Qa\x06)\x91\x90alIV[a\x08\x1Ca\x0C*6`\x04al\x8DV[a#\\V[a\x0CBa\x0C=6`\x04ag\xD3V[a%.V[`@Qa\x06)\x91\x90am\x02V[a\x08\x1Ca\x0C]6`\x04am\x8CV[a%?V[a\x0Cja/\x91V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x06)V[a\x08\x1Ca\x0C\x8D6`\x04am\xE0V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x08\x1Ca\x0C\xD16`\x04an\x19V[a1}V[a\x0C\xE9a\x0C\xE46`\x04ae\xD9V[a2\xB7V[`@Qa\x06)\x91\x90an\x8CV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\n\xA9V[a\x08\x1Ca\r\x156`\x04an\xB7V[a2\xC8V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\n\xA9V[a\r>a\r96`\x04an\xFCV[a38V[`@Qa\x06)\x91\x90ao\x18V[a\r^a\rY6`\x04ao\x8DV[a3yV[`@Qa\x06)\x91\x90ao\xC1V[a\n\xA9a\ry6`\x04ae\xC0V[a3\x8AV[a\r\x91a\r\x8C6`\x04ah\xCAV[a4KV[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x06)V[a\r\xBCa\r\xB76`\x04af\xDBV[a4iV[`@Qa\x06)\x91\x90ap7V[a\r\xDCa\r\xD76`\x04apJV[a5\x03V[`@Qa\x06)\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x08\x1Ca\x0Ex6`\x04apfV[a5KV[a\x0E\x90a\x0E\x8B6`\x04a^\xE7V[a6\xB3V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x06)V[a\x0E\xDDa\x0E\xD86`\x04an\xFCV[a6\xDFV[`@Qa\x06)\x91\x90aq\tV[a\x0E\xFDa\x0E\xF86`\x04a`\xC8V[a7 V[`@Qa\x06)\x91\x90aq`V[a\x0F\x1Da\x0F\x186`\x04a_\xB0V[a71V[`@Qa\x06)\x91\x90ar&V[a\x0F=a\x0F86`\x04ao\x8DV[a7jV[`@Qa\x06)\x91\x90ar9V[a\x0F]a\x0FX6`\x04a`\xC8V[a7{V[`@Qa\x06)\x91\x90ar\x95V[a\x0F}a\x0Fx6`\x04ae\xD9V[a7\x98V[`@Qa\x06)\x91\x90ar\xF2V[a\x0F\x9Da\x0F\x986`\x04ag\xD3V[a7\xBDV[`@Qa\x06)\x91\x90as*V[a\x0BEa\x0F\xB86`\x04am\x8CV[a7\xCEV[a\x0Cja\x0F\xCB6`\x04am\x8CV[a8kV[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x07\xD1V[`\0a\t\xC3V[a\x0F\xFBa\x0F\xF66`\x04ao\x8DV[a9KV[`@Qa\x06)\x91\x90as\x7FV[g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC3V[a\x10)a\x10$6`\x04ao\x8DV[a9\\V[`@Qa\x06)\x91\x90as\xE6V[a\x10Ia\x10D6`\x04an\xFCV[a9mV[`@Qa\x06)\x91\x90at\x83V[a\x10ia\x10d6`\x04a^\xE7V[a9\xAEV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x0F\x90\x81\x0B\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x06)V[a\x08\x1Ca\x10\xAA6`\x04an\x19V[a9\xDAV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x06)V[a\x08\x1Ca\x11\x106`\x04at\xDBV[a>^V[a\x11(a\x11#6`\x04auRV[a>\xECV[`@Qa\x06)\x93\x92\x91\x90aumV[a\x0BEa\x11E6`\x04auRV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x08\x1Ca\x11n6`\x04af\xDBV[a@\x07V[`\xA6a\x0BEV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\nV[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\x81V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\x9DV[a\x12\x1Aa[\xD7V[a\x11\xA6\x82awLV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82aw\xB3V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83ax\xC5V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83ax\xE1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11\xA66\x83\x90\x03\x83\x01\x83ay?V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82azsV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11\xA66\x83\x90\x03\x83\x01\x83az\xC1V[`\0[\x81\x81\x10\x15a\x13\x94W6`\0\x84\x84\x84\x81\x81\x10a\x13_Wa\x13_az\xDDV[\x90P` \x02\x81\x01\x90a\x13q\x91\x90az\xF3V[\x91P\x91Pa\x13\x7F\x82\x82a@\x94V[PP\x80\x80a\x13\x8C\x90a{OV[\x91PPa\x13BV[P`\xA6\x80T\x82\x91\x90`\0\x90a\x13\xB3\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a{hV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x08\xFDW`\xA8`\0\x82` \x01\x80Qa\x14I\x90a{\x93V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x14\x84`\x01\x83\x01\x82a\\\x10V[PPa\x14\x14V[`\xA6\x80T`\0\x91\x90\x82\x90a\x14\xA7\x90`\x01`\x01`@\x1B\x03\x16a{\xB6V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA6T\x16\x91\x90PV[\x80Q`\0\x03a\x14\xDEW`\0\x80\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x15'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`@Q\x80\x91\x03\x90\xFD[P`\x01\x19\x84\x01a\x15pW`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16\x15a\x15nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P[``\x84\x90\x1Ca\x15\xA6\x813\x81\x14a\x15\xA0W`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaP?V[\x83aP?V[`\x01\x85\x14\x80\x15\x90a\x15\xCCWP`\0\x85\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x16DW`\x9AT`@Qc\xD6\x93\xC5\xF1`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD6\x93\xC5\xF1\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16?W=`\0\x80>=`\0\xFD[PPPP[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ra\x16\xC8\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xB8\x91\x90a{\xDCV[3\x85`\x01`\x01`\x80\x1B\x03\x16aP\x94V[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x17\x15Bb\x03\xF4\x80a{hV[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x17\x99\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xB7\x92\x91` \x01a|\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a\x17\xDF\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x18K\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPV[a\x18\xCB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x11\xA6\x82a|>V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11\xA6\x82a}\x05V[`\0Z\x90Pa\x19\0\x85aP\xBDV[`\0[\x83\x81\x10\x15a\x19\xCBW6`\0\x86\x86\x84\x81\x81\x10a\x19 Wa\x19 az\xDDV[\x90P` \x02\x81\x01\x90a\x192\x91\x90az\xF3V[\x91P\x91Pa\x19@\x82\x82a@\x94V[\x84Za\x19L\x90\x86a}\x11V[\x11\x15a\x19\xB6W`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\x9DW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19\xB1W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x19\xC3\x90a{OV[\x91PPa\x19\x03V[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x19\xE9\x90\x85a}\x11V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A%W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A9W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a}:V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1A\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x91\x90PV[a\x1A\xDAa\\\xCEV[a\x11\xA6\x82a~\xACV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11\xA66\x83\x90\x03\x83\x01\x83a~\xB8V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a~\xF9V[a\x1B5a\\\xFCV[a\x11\xA6\x82a\x7F\x9AV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82a\x80hV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1B{WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1B\x95WP0;\x15\x80\x15a\x1B\x95WP`\0T`\xFF\x16`\x01\x14[a\x1C\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x1EV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C*W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C2aQ\x05V[a\x1C\xA6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaQxV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1D!\x90`\0\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Db\x91\x90a{\xDCV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1D\xA6\x90`\x01\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE7\x91\x90a{\xDCV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1EEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ei\x91\x90a{\xDCV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1FEW\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\xF4Wa\x1E\xF4az\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1F=\x81a\x80\xF0V[\x91PPa\x1E\xCDV[P\x80\x15a\x1F\x8CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F\xD2W\x81Qa\x1F\xD8V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a \x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11\xA6\x82a\x81UV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a!4\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!`\x90a\x81aV[\x80\x15a!\xADW\x80`\x1F\x10a!\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xADV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\xE7\x92\x90\x91`\x04\x01a\x81\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\" WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\"\x1D\x91\x81\x01\x90a\x81\xB7V[`\x01[a\"-WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x08\xFD\x81`\0aQ\xEDV[a\"\x7Fa]+V[a\x11\xA6\x82a\x81\xD0V[a\"\x90a]bV[a\x11\xA6\x82a\x82\x02V[a\"\xA1aT\x8CV[`\xB2T`\x01`\x01`\xA0\x1B\x03\x16\x15a\"\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Falready set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x1EV[`\xB2\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a#$aT\x8CV[a#.`\0aT\xE6V[V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x82KV[a#e\x86aP\xBDV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a#|W`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a$\x16W\x81\x87\x87\x83\x81\x81\x10a#\xC9Wa#\xC9az\xDDV[\x90P` \x02\x81\x01\x90a#\xDB\x91\x90az\xF3V[`@Q` \x01a#\xED\x93\x92\x91\x90a\x82gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a$\x0F\x90a{OV[\x90Pa#\xAEV[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\x88W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a%$W6`\0\x88\x88\x84\x81\x81\x10a$\xACWa$\xACaz\xDDV[\x90P` \x02\x81\x01\x90a$\xBE\x91\x90az\xF3V[\x91P\x91Pa$\xCC\x82\x82a@\x94V[`\xA6\x80T`\x01\x91\x90`\0\x90a$\xEB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a{hV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a%\x1C\x90a{OV[\x91PPa$\x8FV[PPPPPPPPV[a%6a]\x92V[a\x11\xA6\x82a\x82\xC2V[30\x14a%KW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a%`Wa%`az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a%{Wa%{a{\xF9V[\x90P`\x01\x81` \x81\x11\x15a%\x91Wa%\x91a{\xF9V[\x03a&UW`\0a%\xA5\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a%\xB2\x91\x90a~\xF9V[\x90Pa%\xC2\x81`\0\x01Q\x86aU8V[\x80Qa%\xCD\x90aU\x96V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a&KW=`\0\x80>=`\0\xFD[PPPPPa/\x8BV[`\x02\x81` \x81\x11\x15a&iWa&ia{\xF9V[\x03a'\rW`\0a&}\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a&\x8A\x91\x90av\x81V[\x90Pa&\x9A\x81`\0\x01Q\x86aU8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA6T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a&\x1DV[`\x07\x81` \x81\x11\x15a'!Wa'!a{\xF9V[\x03a'\x8FW`\x9AT`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a'X\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'\x86W=`\0\x80>=`\0\xFD[PPPPa/\x8BV[`\t\x81` \x81\x11\x15a'\xA3Wa'\xA3a{\xF9V[\x03a)3W`\0a'\xB7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a'\xC4\x91\x90a\x83\xB2V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a'\xF6\x90`\0\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(7\x91\x90a{\xDCV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xB3\x91\x90a{\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a(\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x80Qa)\x03\x90\x86aU8V[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a&\x1D\x90\x84\x90`\x04\x01ao\x18V[`\n\x81` \x81\x11\x15a)GWa)Ga{\xF9V[\x03a)\xA8W`\0a)[\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a)h\x91\x90av\x81V[\x90Pa)x\x81`\0\x01Q\x86aU8V[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a&\x1D\x90\x84\x90`\x04\x01a_[V[`\x0B\x81` \x81\x11\x15a)\xBCWa)\xBCa{\xF9V[\x03a*TW`\0a)\xD0\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a)\xDD\x91\x90ax\xE1V[\x90Pa)\xED\x81`\0\x01Q\x86aU8V[\x80Qa)\xF8\x90aV%V[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a&\x1DV[`\x12\x81` \x81\x11\x15a*hWa*ha{\xF9V[\x03a*\xBEW`\0a*|\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a*\x89\x91\x90a\x83\xCEV[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a&\x1D\x91`\x04\x01ap7V[`\x13\x81` \x81\x11\x15a*\xD2Wa*\xD2a{\xF9V[\x03a+AW`\0a*\xE6\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a*\xF3\x91\x90av\nV[\x90Pa+\x03\x81`\0\x01Q\x86aU8V[\x80Qa+\x0E\x90aV%V[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua/\x8BV[`\x15\x81` \x81\x11\x15a+UWa+Ua{\xF9V[\x03a+\xF1W`\0a+i\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a+v\x91\x90av\x9DV[\x90Pa+\x86\x81`\0\x01Q\x86aU8V[a+\x93\x81``\x01QaU\x96V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a&\x1DV[`\x1B\x81` \x81\x11\x15a,\x05Wa,\x05a{\xF9V[\x03a,KW`\x9AT`\xA6T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a'X\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x84\x02V[`\x1D\x81` \x81\x11\x15a,_Wa,_a{\xF9V[\x03a,\x96W`\x9AT`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a'X\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\r\x81` \x81\x11\x15a,\xAAWa,\xAAa{\xF9V[\x03a/1W`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\x13W=`\0\x80>=`\0\xFD[PPPP`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x94\x91\x90\x81\x01\x90a\x84/V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xB1Wa-\xB1ac\xD0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a.\xBDW`\xAA`\0\x84\x83\x81Q\x81\x10a-\xFFWa-\xFFaz\xDDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a.AWa.Aaz\xDDV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10a.mWa.maz\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a.\xB5\x81a{OV[\x91PPa-\xE0V[Pa.\xC8`\x01aV%V[`\x9AT`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a.\xF8\x90\x84\x90`\x04\x01a\x84\xC8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/&W=`\0\x80>=`\0\xFD[PPPPPPa/\x8BV[`\x18\x81` \x81\x11\x15a/EWa/Ea{\xF9V[\x03a\x06\x04W`\x9AT`\xA6T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a'X\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x84\x02V[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a0&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a0\x84\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xB0\x90a\x81aV[\x80\x15a0\xFDW\x80`\x1F\x10a0\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a17\x92\x90\x91`\x04\x01a\x81\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a1pWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra1m\x91\x81\x01\x90a\x84\xDBV[`\x01[a\"-W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a1\x92Wa1\x92az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a1\xADWa1\xADa{\xF9V[\x90P`\0\x81` \x81\x11\x15a1\xC3Wa1\xC3a{\xF9V[\x03a2 W`\0a1\xD7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a1\xE4\x91\x90a\x85\x95V[\x80QQ\x90\x91P`\x02\x14a2\x1AW\x80Q\x80Q`\xA0\x90\x91\x01Qa2\x05\x91\x90aV\x8EV[\x80QQa2\x1A\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[Pa2tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a2\x8D\x83a{\xB6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a2\xBFa]\xBBV[a\x11\xA6\x82a\x86>V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra33\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra3\x11\x90a\x86JV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x14\xD0V[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x83\xB2V[a3\x81a]\xE8V[a\x11\xA6\x82a\x86nV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a3\xB8W`\0\x82\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x11\xA6V[`\xB0T`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xAB\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4+\x91\x90a\x81\xB7V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x86\xC5V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta4\x82\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\xAE\x90a\x81aV[\x80\x15a4\xFBW\x80`\x1F\x10a4\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\xFBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a4\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\tV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a5\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x1EV[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a5\xEE\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a6Z\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\xA0V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\xE6V[a7(a^'V[a\x11\xA6\x82a\x88\x02V[`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82a\x89\0V[a7ra]\xE8V[a\x11\xA6\x82a\x89UV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82a\x89\xA0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82a\x8AcV[a7\xC5a^OV[a\x11\xA6\x82a\x8A\xC2V[`\0\x80\x83\x83`\0\x81\x81\x10a7\xE4Wa7\xE4az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\xFFWa7\xFFa{\xF9V[\x90P`\x13\x81` \x81\x11\x15a8\x15Wa8\x15a{\xF9V[\x03a8`W`\0a8)\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a86\x91\x90av\nV[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a8TW`\0a8WV[\x80Q[\x92PPPa\"-V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a8\x81Wa8\x81az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a8\x9CWa8\x9Ca{\xF9V[\x90P`\t\x81` \x81\x11\x15a8\xB2Wa8\xB2a{\xF9V[\x03a8\xE1W`\0a8\xC6\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90a\x83\xB2V[` \x01Q\x92Pa\"-\x91PPV[`\n\x81` \x81\x11\x15a8\xF5Wa8\xF5a{\xF9V[\x03a9\x16W`\0a9\t\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90av\x81V[`\x0B\x81` \x81\x11\x15a9*Wa9*a{\xF9V[\x03a8`W`\0a9>\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90ax\xE1V[a9Sa]\xE8V[a\x11\xA6\x82a\x8BlV[a9da^nV[a\x11\xA6\x82a\x8B\xC8V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x8CuV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x8C\x91V[`\0\x82\x82`\0\x81\x81\x10a9\xEFWa9\xEFaz\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a:\nWa:\na{\xF9V[\x90P3`\x01\x82` \x81\x11\x15a:!Wa:!a{\xF9V[\x03a:+W`\0\x80\xFD[`\x07\x82` \x81\x11\x15a:?Wa:?a{\xF9V[\x03a:\x85W`\0a:S\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a:`\x91\x90a\x86\xC5V[\x90Pa:\x7Fa:maWJV[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aP\x94V[Pa<\xE1V[`\x12\x82` \x81\x11\x15a:\x99Wa:\x99a{\xF9V[\x03a:\xCDW`gT`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a:\xC8W`\0\x80\xFD[a<\xE1V[`\x15\x82` \x81\x11\x15a:\xE1Wa:\xE1a{\xF9V[\x03a;\x87W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a;=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P`\0a;M\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a;Z\x91\x90av\x9DV[\x90Pa:\x7F\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaP?V[`\x1B\x82` \x81\x11\x15a;\x9BWa;\x9Ba{\xF9V[\x03a;\xB1W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\x1D\x82` \x81\x11\x15a;\xC5Wa;\xC5a{\xF9V[\x03a;\xDBW`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\r\x82` \x81\x11\x15a;\xEFWa;\xEFa{\xF9V[\x03a<\x05W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\x18\x82` \x81\x11\x15a<\x19Wa<\x19a{\xF9V[\x03a</W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\xB2T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x80\x15a<]WP`\x13\x82` \x81\x11\x15a<[Wa<[a{\xF9V[\x14[\x80\x15a<\x84WPFb\x15\xF9\0\x14\x80a<wWPFb\x16\x1C(\x14[\x80a<\x84WPFb\x15\xF9\x02\x14[\x15a<\x90WP0a<\xE1V[a<\x9Ea<\x9BaWJV[PV[`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a<\xB9\x90\x84\x90`\x0F\x0Ba\x8C\xD9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a=.Bb\x03\xF4\x80a{hV[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA8\x93P\x90a=\x97\x82a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a>\x03\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra>\xE5\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra>\xA7\x90a\x86JV[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14\xD0\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a?v\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta?\xA2\x90a\x81aV[\x80\x15a?\xEFW\x80`\x1F\x10a?\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a?\xEFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a?\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a@\x0FaT\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x1EV[a<\x9B\x81aT\xE6V[`\0\x82\x82`\0\x81\x81\x10a@\xA9Wa@\xA9az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a@\xC4Wa@\xC4a{\xF9V[\x90P`\0\x81` \x81\x11\x15a@\xDAWa@\xDAa{\xF9V[\x03aA\xC8W`\0a@\xEE\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a@\xFB\x91\x90a\x85\x95V[\x80QQ\x90\x91P`\x02\x14aAjW\x80Q\x80Q`\xA0\x90\x91\x01QaA\x1C\x91\x90aV\x8EV[\x80QQaAI\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[`\0\x93\x92PPPV[aW\xBDV[PPV[\x80QQaAU\x90aV%V[\x80QQaAj\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91aA\x9A\x91`\x04\x01aq\tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aA\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%$W=`\0\x80>=`\0\xFD[`\x02\x81` \x81\x11\x15aA\xDCWaA\xDCa{\xF9V[\x03aC-W`\0aA\xF0\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aA\xFD\x91\x90a\x8D:V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaB\x13\x91aV\x8EV[\x80QQaB.\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01RaB\xB8\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xAD\x91\x90a\x8DnV[\x83Q` \x01QaX\x0BV[`\x9AT\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA6T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01aA\x9AV[`\x03\x81` \x81\x11\x15aCAWaCAa{\xF9V[\x03aDEW`\0aCU\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aCb\x91\x90a\x8D\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15aC\xAEW` \x82\x01Q\x83QaC\xA9\x91\x90a\x8D\xBFV[aC\xB1V[`\0[`\x9BT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\0W`\0\x80\xFD[PZ\xF1\x15\x80\x15aD\x14W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa33\x91PPV[`\x0F\x81` \x81\x11\x15aDYWaDYa{\xF9V[\x03aEZW`\0aDm\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aDz\x91\x90a\x8D\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15aD\xC2W\x81Q\x83QaD\xBD\x91\x90a\x8D\xBFV[aD\xC5V[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91aD\xFE\x91\x85\x91\x90`\x04\x01a\x8D\xE7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15aE,W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa33\x91PPV[`\x04\x81` \x81\x11\x15aEnWaEna{\xF9V[\x03aF3W`\x9AT`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90aE\xAA\x90\x88\x90\x88\x90`\x04\x01a\x83!V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aE\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xEC\x91\x90a\x8E\tV[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a>\xE5Wc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90UPPPPPV[`\x05\x81` \x81\x11\x15aFGWaFGa{\xF9V[\x03aF\xACW`\x9AT`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\x8CW=`\0\x80>=`\0\xFD[`\x06\x81` \x81\x11\x15aF\xC0WaF\xC0a{\xF9V[\x14\x80aF\xDDWP`\x16\x81` \x81\x11\x15aF\xDBWaF\xDBa{\xF9V[\x14[\x15aG\xD1W`\0aF\xF1\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aF\xFE\x91\x90a\x8E8V[` \x81\x01QQQ\x90\x91PaG\x11\x90aV%V[`@\x81\x01QQQaG!\x90aV%V[`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01aGI\x84` \x01Q`\0\x01Q`\0\x01Qa3\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aGl\x84`@\x01Q`\0\x01Q`\0\x01Qa3\x8AV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R`\xB0T`@Qc\x11\x17\x8F-`\xE3\x1B\x81R\x92\x93P\x16\x90c\x88\xBCyh\x90aG\xA3\x90\x84\x90`\x04\x01a\x8ElV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG\xBDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A9W=`\0\x80>=`\0\xFD[`\x0C\x81` \x81\x11\x15aG\xE5WaG\xE5a{\xF9V[\x03aH]W`\0aG\xF9\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aH\x06\x91\x90a\x8E\xB7V[``\x81\x01QQQ\x90\x91PaH\x19\x90aV%V[`\xB0T``\x82\x01QQQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xF0\xD3\xCE\x90\x83\x90aH@\x90a3\x8AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\x9A\x92\x91\x90a\x8E\xEBV[`\x08\x81` \x81\x11\x15aHqWaHqa{\xF9V[\x03aI\x0FW`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaH\xB9\x81`\x01aQ\xEDV[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81` \x81\x11\x15aI#WaI#a{\xF9V[\x03aI\xBAW`\0aI7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aID\x91\x90a\x8F\x16V[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaIZ\x91aV\x8EV[\x80QQaIu\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaI\x8A\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91aA\x9A\x91`\x04\x01ao\x18V[`\n\x81` \x81\x11\x15aI\xCEWaI\xCEa{\xF9V[\x03aJeW`\0aI\xE2\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aI\xEF\x91\x90a\x8D:V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaJ\x05\x91aV\x8EV[\x80QQaJ \x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaJ5\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91aA\x9A\x91`\x04\x01a_[V[`\x1E\x81` \x81\x11\x15aJyWaJya{\xF9V[\x03aK\x87W`\0aJ\x8D\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aJ\x9A\x91\x90a\x8FJV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaJ\xB0\x91aV\x8EV[\x80QQaJ\xCB\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaJ\xE0\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x88<q\x85`\xE0\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88<q\x85\x90`\x84\x01aA\x9AV[`\x1F\x81` \x81\x11\x15aK\x9BWaK\x9Ba{\xF9V[\x03aL\xA9W`\0aK\xAF\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aK\xBC\x91\x90a\x8FJV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaK\xD2\x91aV\x8EV[\x80QQaK\xED\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaL\x02\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x1C\xD4\x0F_`\xE3\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\xA0z\xF8\x90`\x84\x01aA\x9AV[`\x10\x81` \x81\x11\x15aL\xBDWaL\xBDa{\xF9V[\x03aL\xF4W`\x9AT`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x13\x81` \x81\x11\x15aM\x08WaM\x08a{\xF9V[\x03aM\x8EW`\0aM\x1C\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aM)\x91\x90a\x8F~V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaM?\x91aV\x8EV[\x80QQaMZ\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81` \x81\x11\x15aM\xA2WaM\xA2a{\xF9V[\x03aM\xD9W`\x9AT`@Qc&fm-`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99\x99\xB4\xB4\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x17\x81` \x81\x11\x15aM\xEDWaM\xEDa{\xF9V[\x03aN\x95W`\0aN\x01\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aN\x0E\x91\x90a\x8F\xB2V[\x90PaN!\x81`\0\x01Q` \x01QaU\x96V[\x80QQaN<\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80Q\x80Q``\x90\x91\x01QaNP\x91\x90aV\x8EV[\x80QQaNe\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91aA\x9A\x91`\x04\x01aa\xBFV[`\x19\x81` \x81\x11\x15aN\xA9WaN\xA9a{\xF9V[\x03aN\xE0W`\x9AT`@QcJg\xD9\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\xCF\xB2\x02\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x1A\x81` \x81\x11\x15aN\xF4WaN\xF4a{\xF9V[\x03aO+W`\x9AT`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x1C\x81` \x81\x11\x15aO?WaO?a{\xF9V[\x03aO\xF4W`\0aOS\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aO`\x91\x90a\x8F\xE6V[`\xB0T\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA2z%\n\x90\x84\x90aO\x89\x90a3\x8AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xA6\x92\x91\x90a\x90\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aO\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xE9\x91\x90a\x81\xB7V[\x90Pa>\xE5\x81aU\x96V[` \x81` \x81\x11\x15aP\x08WaP\x08a{\xF9V[\x03a\x06\x04W`\x9AT`@Qc~\x92v\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c~\x92v\xD7\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaPb\x90a\x81aV[\x90P`\0\x03aAEW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa33\x92\x84\x01\x90a\\JV[`\x01`\x01`\xA0\x1B\x03\x83\x16aP\xA7W`\0\x80\xFD[`\x9ATa33\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14aAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`\0Ta\x01\0\x90\x04`\xFF\x16aQpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[a#.aX\xE2V[`\0Ta\x01\0\x90\x04`\xFF\x16aQ\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[aAE\x82\x82aYVV[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x90\xC4`#\x919\x90aRAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aR\xA0\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaR\xCC\x90a\x81aV[\x80\x15aS\x19W\x80`\x1F\x10aR\xEEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aS\x19V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aR\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aS:\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aSw`\x01\x83\x01\x82a\\\x10V[PP\x81\x80aS\x92WPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aS\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[PFazi\x03aT\x02W` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aF~\x92`\x04\x01a\x81\x95V[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aT5\x92\x90\x91`\x04\x01a\x81\x95V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aTOW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aT`WP`\x01[a/\x8BWb\x03\xD0\x90Z\x11\x15\x80aT\x80WPaT|`\x02\x82a\x90-V[Z\x11\x15[\x15aT\x87W\xFE[a/\x8BV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a#.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x15\x1EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aU[WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a33W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a<\x9BW`\xA2\x80T`\0\x90aU\xCD\x90`\x01`\x01`@\x1B\x03\x16a{\xB6V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[`\x01\x81\x14\x80aV4WP`\x02\x81\x14[\x80aVUWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90aAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aV\xB7\x83a{\xB6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a33WaW\x08\x81`\x01`\x01`@\x1B\x03\x16aY\xDBV[`@Q` \x01aW\x18\x91\x90a\x90OV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x15\x1E\x91`\x04\x01ap7V[aAE\x82\x82`\0aX\x0BV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xB8\x91\x90a{\xDCV[\x90P\x90V[`\0a\x11\xA6aW\xCAaZzV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aX'\x86a\x90\x94V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aXvW`\0\x80\xFD[PZ\xF1\x15\x80\x15aX\x8AW=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aX\xB6\x90\x84\x90`\x0F\x0Ba\x8C\xD9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16aYMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[a#.3aT\xE6V[`\0Ta\x01\0\x90\x04`\xFF\x16aY\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0aY\xE8\x83aZ\xF5V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\x07WaZ\x07ac\xD0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aZ1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aZ;WP\x93\x92PPPV[`\0aW\xB8\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaZ\xA9`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a[>Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a[jWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a[\x88Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a[\xA0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a[\xB4Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a[\xC6W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11\xA6W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80Ta\\\x1C\x90a\x81aV[`\0\x82U\x80`\x1F\x10a\\,WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a<\x9B\x91\x90a^\xA2V[\x82\x80Ta\\V\x90a\x81aV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\\xW`\0\x85Ua\\\xBEV[\x82`\x1F\x10a\\\x91W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\\\xBEV[\x82\x80\x01`\x01\x01\x85U\x82\x15a\\\xBEW\x91\x82\x01[\x82\x81\x11\x15a\\\xBEW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\\\xA3V[Pa\\\xCA\x92\x91Pa^\xA2V[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01a\\\xF7a]+V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a]\xDBa]+V[\x81R` \x01a\\\xF7a]+V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01a]\xDBa]+V[`@Q\x80`\xA0\x01`@R\x80a^\x81a^\xB7V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15a\\\xCAW`\0\x81U`\x01\x01a^\xA3V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a^\xF9W`\0\x80\xFD[a\"-\x83\x83a^\xD5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11\xA6V[`\0`\x80\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a_QW`\0\x80\xFD[a\"-\x83\x83a_-V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11\xA6V[`\0`\xA0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a_\xC2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xD8W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a_\x9EV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a`\x07W\x81\x81\x01Q\x83\x82\x01R` \x01a_\xEFV[\x83\x81\x11\x15a/\x8BWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra`0\x81` \x86\x01` \x86\x01a_\xECV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a`\x88\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01Ra_\xE4`\xA0\x85\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84a`DV[`\0`@\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a`\xDAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a`\xF0W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a`\xB6V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aa\x10V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R``\x83\x01\x84Q`@\x83\x86\x01R\x81\x81Q\x80\x84R`\x80\x87\x01\x91P`\x80\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15aa\x97W`\x7F\x19\x88\x86\x03\x01\x83Raa\x85\x85\x85Qa`\x18V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01aaiV[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90aa\xB6\x81\x83a`\xFCV[\x95\x94PPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11\xA6V[`\0`@\x82\x84\x03\x12\x15ab\x0EW`\0\x80\xFD[a\"-\x83\x83a`\xB6V[`\0` \x82\x84\x03\x12\x15ab*W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ab@W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\"-W`\0\x80\xFD[`\0a\x01\0\x82Q\x80Q\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R`@\x81\x01Q`\x0F\x0B`@\x86\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x88\x01R\x80`\x80\x84\x01Q\x16`\x80\x88\x01RPP`\xA0\x81\x01Q`\x0F\x0B`\xA0\x86\x01RP` \x83\x01Qab\xC0`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Q\x81`\xE0\x86\x01Raa\xB6\x82\x86\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84abSV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x11\xA6V[`\0\x80\x83`\x1F\x84\x01\x12ac\x1DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15acOW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aciW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ac\x7FW`\0\x80\xFD[ac\x8B\x85\x82\x86\x01ac\x0BV[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a<\x9BW`\0\x80\xFD[\x805ac\xB4\x81ac\x97V[\x91\x90PV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\xE0Wad\xE0ac\xD0V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12ad\xF9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ae\x12Wae\x12ac\xD0V[ae%`\x1F\x82\x01`\x1F\x19\x16` \x01ad\xB8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15ae:W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aemW`\0\x80\xFD[\x845\x93P` \x85\x015ae\x7F\x81ac\x97V[\x92Pae\x8D`@\x86\x01ac\xB9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xA8W`\0\x80\xFD[ae\xB4\x87\x82\x88\x01ad\xE8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ae\xD2W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ae\xEBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\x01W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a^\xD5V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01af!V[` \x81R`\0\x82Q``` \x84\x01Raf\\`\x80\x84\x01\x82af\rV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Rafz\x83\x83af\rV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPaa\xB6\x82\x82af\rV[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra_\xE4``\x84\x01\x82a`\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a<\x9BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15af\xEDW`\0\x80\xFD[\x815a\"-\x81af\xC6V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15ag%W`\0\x80\xFD[ag.\x85af\xF8V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15agIW`\0\x80\xFD[agU\x87\x82\x88\x01ac\x0BV[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15agzW`\0\x80\xFD[ag\x82ac\xE6V[ag\x8B\x83af\xF8V[\x81Rag\x99` \x84\x01af\xF8V[` \x82\x01Rag\xAA`@\x84\x01af\xF8V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ag\xC8W`\0\x80\xFD[\x815a\"-\x81ac\x97V[`\0` \x82\x84\x03\x12\x15ag\xE5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xFBW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a_-V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01Ra_\xE4`\xC0\x85\x01\x82a`\x18V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01Ra_\xE4`\x80\x85\x01\x82ah\x07V[` \x81R`\0a\"-` \x83\x01\x84ahfV[`\0` \x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xDCW`\0\x80\xFD[a\"-\x83\x83ah\xB8V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ah\xFAV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01RaiJ`\xE0\x85\x01\x82ah\xE6V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Raig\x82\x82a`\xFCV[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90aa\xB6\x81\x83a`\x18V[` \x81R`\0\x82Q`@` \x84\x01Rai\xB6``\x84\x01\x82a`\xFCV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Raa\xB6\x82\x82af\rV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15ai\xECWai\xECac\xD0V[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a<\x9BW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aj\x16W`\0\x80\xFD[\x815` aj+aj&\x83ai\xD3V[ad\xB8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ajJW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805aja\x81ai\xF6V[\x83R\x91\x83\x01\x91\x83\x01ajNV[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aj\x92W`\0\x80\xFD[\x865aj\x9D\x81af\xC6V[\x95P` \x87\x015aj\xAD\x81af\xC6V[\x94P`@\x87\x015aj\xBD\x81af\xC6V[\x93P``\x87\x015aj\xCD\x81af\xC6V[\x92P`\x80\x87\x015aj\xDD\x81af\xC6V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aj\xF8W`\0\x80\xFD[ak\x04\x89\x82\x8A\x01aj\x05V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01Ra_\xE4`@\x85\x01\x82af\rV[` \x81R`\0a\"-` \x83\x01\x84ak\x11V[`\0`\xC0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15akpW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ak\x86W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01akLV[` \x81R`\0a\"-` \x83\x01\x84ah\x07V[\x805\x80\x15\x15\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ak\xC8W`\0\x80\xFD[\x825ak\xD3\x81af\xC6V[\x91Pak\xE1` \x84\x01ak\xA5V[\x90P\x92P\x92\x90PV[` \x81Ral/` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Ra_\xE4`\xC0\x84\x01\x82a`\x18V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11\xA6V[\x805`\xFF\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15al\xA6W`\0\x80\xFD[al\xAF\x87af\xF8V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15al\xCAW`\0\x80\xFD[al\xD6\x89\x82\x8A\x01ac\x0BV[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Pal\xF6`\x80\x88\x01al|V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Ram1` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Ra_\xE4`\xA0\x84\x01\x82a`\x18V[`\0\x80\x83`\x1F\x84\x01\x12am]W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15amtW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15acOW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15am\xA1W`\0\x80\xFD[\x835am\xAC\x81af\xC6V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am\xC7W`\0\x80\xFD[am\xD3\x86\x82\x87\x01amKV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15am\xF3W`\0\x80\xFD[\x825am\xFE\x81ac\x97V[\x91P` \x83\x015an\x0E\x81ai\xF6V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15an,W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15anBW`\0\x80\xFD[ac\x8B\x85\x82\x86\x01amKV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rans``\x85\x01\x82ah\x07V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Raa\xB6\x82\x82ah\x07V[` \x81R`\0a\"-` \x83\x01\x84anNV[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15an\xCCW`\0\x80\xFD[an\xD5\x84an\x9FV[\x92P` \x84\x015an\xE5\x81ac\x97V[\x91Pan\xF3`@\x85\x01ac\xB9V[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ao\x0EW`\0\x80\xFD[a\"-\x83\x83akLV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ao\x9FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xB5W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01ao{V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Ra_\xE4a\x01\0\x84\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84a`\x18V[`\0`\xE0\x82\x84\x03\x12\x15ap\\W`\0\x80\xFD[a\"-\x83\x83ao{V[`\0\x80`@\x83\x85\x03\x12\x15apyW`\0\x80\xFD[ap\x82\x83af\xF8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ap\x9EW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15ap\xB2W`\0\x80\xFD[ap\xBAac\xE6V[ap\xC3\x83af\xF8V[\x81R` \x83\x015ap\xD3\x81af\xC6V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ap\xEAW`\0\x80\xFD[ap\xF6\x88\x82\x86\x01ad\xE8V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Raq\x8F`\xC0\x85\x01\x82ah\xE6V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Raa\xB6\x81\x83a`\x18V[aq\xF2\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xA0``\x85\x01Rar\r`\xA0\x85\x01\x82a`\x18V[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a\"-` \x83\x01\x84aq\xC1V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Rar\xB1``\x85\x01\x82a`\xFCV[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15ajnW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90ar\xD2V[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Ra_\xE4`\x80\x84\x01\x82a`\xFCV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Rasb`\xA0\x84\x01\x82ah\x07V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Raa\xB6\x82\x82ah\x07V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15at\x15W\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01as\xF6V[PPP\x83\x01Q`\xE0`\x80\x84\x01Rat0a\x01\0\x84\x01\x82a`\x18V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01RatN\x83\x83a`\xFCV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPatl\x82\x82a`\xFCV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15at\xF3W`\0\x80\xFD[at\xFC\x86an\x9FV[\x94P` \x86\x015au\x0C\x81ac\x97V[\x93Pau\x1A`@\x87\x01ac\xB9V[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15au5W`\0\x80\xFD[auA\x88\x82\x89\x01amKV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15audW`\0\x80\xFD[a\"-\x82af\xF8V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rau\xB0`\xC0\x85\x01\x82a`\x18V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15au\xDBW`\0\x80\xFD[au\xE3ac\xE6V[\x90P\x815\x81R` \x82\x015` \x82\x01Rau\xFF`@\x83\x01af\xF8V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15av\x1CW`\0\x80\xFD[a\"-\x83\x83au\xC9V[`\0`\x80\x82\x84\x03\x12\x15av8W`\0\x80\xFD[av@ad\x0EV[\x90P\x815\x81R` \x82\x015avT\x81ac\x97V[` \x82\x01Rave`@\x83\x01ac\xB9V[`@\x82\x01Ravv``\x83\x01af\xF8V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15av\x93W`\0\x80\xFD[a\"-\x83\x83av&V[`\0`\x80\x82\x84\x03\x12\x15av\xAFW`\0\x80\xFD[av\xB7ad\x0EV[\x825\x81R` \x83\x015av\xC9\x81ac\x97V[` \x82\x01Rav\xDA`@\x84\x01ac\xB9V[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aw\x05W`\0\x80\xFD[aw\rad0V[\x90Paw\x19\x83\x83av&V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[aw@\x84\x82\x85\x01ad\xE8V[` \x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83av\xF3V[`\0\x82`\x1F\x83\x01\x12awiW`\0\x80\xFD[\x815` awyaj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aw\x98W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805\x83R\x91\x83\x01\x91\x83\x01aw\x9CV[`\0`@\x826\x03\x12\x15aw\xC5W`\0\x80\xFD[aw\xCDad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aw\xE4W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12aw\xF7W`\0\x80\xFD[\x815` ax\x07aj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15ax&W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ax^W\x805\x86\x81\x11\x15axBW`\0\x80\x81\xFD[axP6\x86\x83\x8B\x01\x01ad\xE8V[\x84RP\x91\x83\x01\x91\x83\x01ax*V[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15axuW`\0\x80\xFD[ax\x816\x85\x89\x01awXV[\x90\x85\x01RP\x91\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15ax\xA1W`\0\x80\xFD[ax\xA9ad\x0EV[\x90P\x815\x81R` \x82\x015` \x82\x01Rave`@\x83\x01ac\xB9V[`\0`\x80\x82\x84\x03\x12\x15ax\xD7W`\0\x80\xFD[a\"-\x83\x83ax\x8FV[`\0`\x80\x82\x84\x03\x12\x15ax\xF3W`\0\x80\xFD[ax\xFBad\x0EV[\x825\x81R` \x83\x015ay\r\x81ac\x97V[` \x82\x01R`@\x83\x015ay \x81ai\xF6V[`@\x82\x01R``\x83\x015ay3\x81ai\xF6V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ayQW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aysWaysac\xD0V[`@Ray\x7F\x83ac\xB9V[\x81R` \x83\x015ay\x8F\x81af\xC6V[` \x82\x01R\x93\x92PPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15ay\xAFW`\0\x80\xFD[ay\xB7ac\xE6V[\x91P`\xC0\x81\x12\x15ay\xC7W`\0\x80\xFD[Pay\xD0adRV[\x825\x81R` \x83\x015ay\xE2\x81ai\xF6V[` \x82\x01R`@\x83\x015ay\xF5\x81ai\xF6V[`@\x82\x01Raz\x06``\x84\x01af\xF8V[``\x82\x01Raz\x17`\x80\x84\x01af\xF8V[`\x80\x82\x01R`\xA0\x83\x015az*\x81ai\xF6V[`\xA0\x82\x01R\x81Raz=`\xC0\x83\x01ac\xA9V[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15az[W`\0\x80\xFD[azg\x84\x82\x85\x01ad\xE8V[`@\x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83ay\x9BV[`\0`@\x82\x84\x03\x12\x15az\x91W`\0\x80\xFD[az\x99ad0V[\x90P\x815az\xA6\x81ac\x97V[\x81R` \x82\x015az\xB6\x81ai\xF6V[` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15az\xD3W`\0\x80\xFD[a\"-\x83\x83az\x7FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a{\nW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a{$W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15acOW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a{aWa{aa{9V[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a{\x8AWa{\x8Aa{9V[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80a{\xACWa{\xACa{9V[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a{\xD2Wa{\xD2a{9V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a{\xEEW`\0\x80\xFD[\x81Qa\"-\x81af\xC6V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qa|0\x81`\x01\x85\x01` \x87\x01a_\xECV[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x826\x03\x12\x15a|PW`\0\x80\xFD[a|Xac\xE6V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a|oW`\0\x80\xFD[a|{6\x83\x87\x01aj\x05V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a|\x91W`\0\x80\xFD[a|\x9D6\x83\x87\x01aj\x05V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a|\xB6W`\0\x80\xFD[Pazg6\x82\x86\x01aj\x05V[`\0`@\x82\x84\x03\x12\x15a|\xD5W`\0\x80\xFD[a|\xDDad0V[\x90P\x815a|\xEA\x81af\xC6V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0a\x11\xA66\x83a|\xC3V[`\0\x82\x82\x10\x15a}#Wa}#a{9V[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14ac\xB4W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a}LW`\0\x80\xFD[a}Tad\x0EV[\x825a}_\x81af\xC6V[\x81R` \x83\x015a}o\x81ac\x97V[` \x82\x01Ra}\x80`@\x84\x01a}(V[`@\x82\x01Ray3``\x84\x01a}(V[`\0\x81\x83\x03`\xC0\x81\x12\x15a}\xA4W`\0\x80\xFD[a}\xACad0V[\x91P`\xA0\x81\x12\x15a}\xBCW`\0\x80\xFD[Pa}\xC5adtV[\x825\x81R` \x83\x015a}\xD7\x81ai\xF6V[` \x82\x01R`@\x83\x015a}\xEA\x81ai\xF6V[`@\x82\x01Ra}\xFB``\x84\x01af\xF8V[``\x82\x01Ra~\x0C`\x80\x84\x01af\xF8V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a~>W`\0\x80\xFD[a~Fad\x0EV[\x90P\x815a~S\x81ac\x97V[\x81R` \x82\x015a~c\x81ai\xF6V[` \x82\x01R`@\x82\x015a~v\x81ai\xF6V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a~\x94W`\0\x80\xFD[a~\xA0\x84\x82\x85\x01a}\x91V[``\x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83a~,V[`\0` \x82\x84\x03\x12\x15a~\xCAW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a~\xECWa~\xECac\xD0V[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x7F\x0BW`\0\x80\xFD[a\x7F\x13ac\xE6V[\x825\x81R` \x83\x015a\x7F%\x81ac\x97V[` \x82\x01Rag\xAA`@\x84\x01ac\xB9V[`\0\x82`\x1F\x83\x01\x12a\x7FGW`\0\x80\xFD[\x815` a\x7FWaj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x7FvW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805a\x7F\x8D\x81ac\x97V[\x83R\x91\x83\x01\x91\x83\x01a\x7FzV[`\0`@\x826\x03\x12\x15a\x7F\xACW`\0\x80\xFD[a\x7F\xB4ad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x7F\xCBW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15a\x7F\xE0W`\0\x80\xFD[a\x7F\xE8ad\x0EV[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x7F\xFEW`\0\x80\xFD[a\x80\n6\x82\x86\x01a\x7F6V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x80\"W`\0\x80\xFD[a\x80.6\x82\x86\x01awXV[`@\x83\x01RPa\x80@``\x84\x01af\xF8V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80[W`\0\x80\xFD[Paw@6\x82\x86\x01ad\xE8V[`\0`@\x826\x03\x12\x15a\x80zW`\0\x80\xFD[a\x80\x82ad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x80\x99W`\0\x80\xFD[a\x80\xA56\x83\x87\x01awXV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80\xBBW`\0\x80\xFD[Paw@6\x82\x86\x01aj\x05V[` \x81\x01`\x02\x83\x10a\x80\xEAWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a{\xD2Wa{\xD2a{9V[`\0`@\x82\x84\x03\x12\x15a\x81\x1BW`\0\x80\xFD[a\x81#ad0V[\x90Pa\x81.\x82ac\xB9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x81IW`\0\x80\xFD[aw@\x84\x82\x85\x01aj\x05V[`\0a\x11\xA66\x83a\x81\tV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x81uW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a_\xE4`@\x83\x01\x84a`\x18V[`\0` \x82\x84\x03\x12\x15a\x81\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0a\x11\xA66\x83a}\x91V[`\0`\xA0\x82\x84\x03\x12\x15a\x81\xEEW`\0\x80\xFD[a\x81\xF6ad0V[\x90Paw\x19\x83\x83ax\x8FV[`\0a\x11\xA66\x83a\x81\xDCV[`\0``\x82\x84\x03\x12\x15a\x82 W`\0\x80\xFD[a\x82(ac\xE6V[\x90P\x815\x81Ra\x82:` \x83\x01ac\xB9V[` \x82\x01Rau\xFF`@\x83\x01af\xF8V[`\0``\x82\x84\x03\x12\x15a\x82]W`\0\x80\xFD[a\"-\x83\x83a\x82\x0EV[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x82\x93W`\0\x80\xFD[a\x82\x9Bad0V[\x90Pa\x82\xA7\x83\x83au\xC9V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0a\x11\xA66\x83a\x82\x81V[`\0\x80\x85\x85\x11\x15a\x82\xDEW`\0\x80\xFD[\x83\x86\x11\x15a\x82\xEBW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a_\xE4` \x83\x01\x84\x86a\x82\xF8V[`\0`\xC0\x82\x84\x03\x12\x15a\x83GW`\0\x80\xFD[a\x83OadRV[\x90P\x815\x81R` \x82\x015a\x83c\x81ac\x97V[` \x82\x01Ra\x83t`@\x83\x01ac\xB9V[`@\x82\x01Ra\x83\x85``\x83\x01ac\xB9V[``\x82\x01Ra\x83\x96`\x80\x83\x01ac\xB9V[`\x80\x82\x01Ra\x83\xA7`\xA0\x83\x01af\xF8V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x83\xC4W`\0\x80\xFD[a\"-\x83\x83a\x835V[`\0` \x82\x84\x03\x12\x15a\x83\xE0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\xF6W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a|\xC3V[`@\x81R`\0a\x84\x16`@\x83\x01\x85\x87a\x82\xF8V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x84BW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84XW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x84iW`\0\x80\xFD[\x80Qa\x84waj&\x82ai\xD3V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x84\x96W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x84\xBDW\x83Qa\x84\xAE\x81ac\x97V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x84\x9BV[\x97\x96PPPPPPPV[` \x81R`\0a\"-` \x83\x01\x84af\rV[`\0` \x82\x84\x03\x12\x15a\x84\xEDW`\0\x80\xFD[\x81Qa\"-\x81ac\x97V[`\0`\xC0\x82\x84\x03\x12\x15a\x85\nW`\0\x80\xFD[a\x85\x12adRV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a\x850\x81ac\x97V[`@\x82\x01Ra\x85A``\x83\x01ak\xA5V[``\x82\x01R`\x80\x82\x015a\x83\x96\x81ai\xF6V[`\0`\xE0\x82\x84\x03\x12\x15a\x85fW`\0\x80\xFD[a\x85nad0V[\x90Pa\x85z\x83\x83a\x84\xF8V[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x85\xA7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xBDW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x85TV[`\0``\x82\x84\x03\x12\x15a\x85\xDBW`\0\x80\xFD[a\x85\xE3ac\xE6V[\x90P\x815a\x85\xF0\x81ac\x97V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x86\x0CW`\0\x80\xFD[a\x86\x18\x85\x83\x86\x01a}\x91V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x861W`\0\x80\xFD[Pazg\x84\x82\x85\x01a}\x91V[`\0a\x11\xA66\x83a\x85\xC9V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1A\xCCW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x11\xA66\x83a\x85TV[`\0` \x82\x84\x03\x12\x15a\x86\x8CW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x86\xAEWa\x86\xAEac\xD0V[`@R\x90P\x80a\x86\xBD\x83ac\xB9V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x86\xD7W`\0\x80\xFD[a\"-\x83\x83a\x86zV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x87\x1BW`\0\x80\xFD[a\x87#ad\x96V[\x825\x81R` \x83\x015a\x875\x81ac\x97V[` \x82\x01R`@\x83\x015a\x87H\x81ac\x97V[`@\x82\x01Ra\x87Y``\x84\x01a\x86\xE1V[``\x82\x01Ra\x87j`\x80\x84\x01a\x86\xE1V[`\x80\x82\x01Ra\x87{`\xA0\x84\x01af\xF8V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x87\x94W`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x87\xB2W`\0\x80\xFD[a\x87\xBAac\xE6V[\x825a\x87\xC5\x81ac\x97V[\x81Ra\x87\xD3` \x84\x01ac\xB9V[` \x82\x01R`@\x83\x015ag\xAA\x81af\xC6V[`\0`\xC0\x82\x84\x03\x12\x15a\x87\xF8W`\0\x80\xFD[a\"-\x83\x83a\x84\xF8V[`\0`@\x826\x03\x12\x15a\x88\x14W`\0\x80\xFD[a\x88\x1Cad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x883W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x88HW`\0\x80\xFD[a\x88Pac\xE6V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x88fW`\0\x80\xFD[a\x88r6\x82\x86\x01a\x7F6V[` \x83\x01RPa\x88\x84`@\x84\x01af\xF8V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80[W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x88\xB1W`\0\x80\xFD[a\x88\xB9ac\xE6V[\x90Pa\x88\xC5\x83\x83a\x82\x0EV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88\xE0W`\0\x80\xFD[a\x88\xEC\x84\x82\x85\x01ad\xE8V[` \x83\x01RP`\x80\x82\x015au\xFF\x81ai\xF6V[`\0a\x11\xA66\x83a\x88\x9FV[`\0`\xC0\x82\x84\x03\x12\x15a\x89\x1EW`\0\x80\xFD[a\x89&adRV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x89B`@\x83\x01al|V[`@\x82\x01R``\x82\x015a\x85A\x81ac\x97V[`\0`\xE0\x826\x03\x12\x15a\x89gW`\0\x80\xFD[a\x89oad0V[a\x89y6\x84a\x89\x0CV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x89\x94W`\0\x80\xFD[aw@6\x82\x86\x01ad\xE8V[`\0`@\x826\x03\x12\x15a\x89\xB2W`\0\x80\xFD[a\x89\xBAad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x89\xD1W`\0\x80\xFD[a\x89\xDD6\x83\x87\x01awXV[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x89\xF4W`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x8A\x07W`\0\x80\xFD[\x805a\x8A\x15aj&\x82ai\xD3V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x8A4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x8ARW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x8A9V[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x8AuW`\0\x80\xFD[a\x8A}ac\xE6V[\x825a\x8A\x88\x81ac\x97V[\x81R` \x83\x015a\x8A\x98\x81ai\xF6V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\xB6W`\0\x80\xFD[azg6\x82\x86\x01awXV[`\0`\x80\x826\x03\x12\x15a\x8A\xD4W`\0\x80\xFD[a\x8A\xDCad\x0EV[\x825a\x8A\xE7\x81ac\x97V[\x81Ra\x8A\xF5` \x84\x01ak\xA5V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8B\x14W`\0\x80\xFD[a\x8B 6\x83\x87\x01a}\x91V[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x8B9W`\0\x80\xFD[Pa~\xA06\x82\x86\x01a}\x91V[`\0`\xE0\x82\x84\x03\x12\x15a\x8BXW`\0\x80\xFD[a\x8B`ad0V[\x90Pa\x85z\x83\x83a\x835V[`\0a\x11\xA66\x83a\x8BFV[`\0\x82`\x1F\x83\x01\x12a\x8B\x89W`\0\x80\xFD[a\x8B\x91ac\xE6V[\x80``\x84\x01\x85\x81\x11\x15a\x8B\xA3W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x8B\xBDW\x805\x84R` \x93\x84\x01\x93\x01a\x8B\xA5V[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x8B\xDAW`\0\x80\xFD[a\x8B\xE2adtV[a\x8B\xEC6\x84a\x8BxV[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8C\x08W`\0\x80\xFD[a\x8C\x146\x83\x87\x01ad\xE8V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x8C-W`\0\x80\xFD[a\x8C96\x83\x87\x01awXV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x8CRW`\0\x80\xFD[Pa\x8C_6\x82\x86\x01awXV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x8C\x87W`\0\x80\xFD[a\"-\x83\x83a\x89\x0CV[`\0``\x82\x84\x03\x12\x15a\x8C\xA3W`\0\x80\xFD[a\x8C\xABac\xE6V[\x825a\x8C\xB6\x81ac\x97V[\x81R` \x83\x015a\x8C\xC6\x81ai\xF6V[` \x82\x01R`@\x83\x015ag\xAA\x81ai\xF6V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x8D\x0CWa\x8D\x0Ca{9V[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x8D1Wa\x8D1a{9V[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8DLW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8DbW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01av\xF3V[`\0` \x82\x84\x03\x12\x15a\x8D\x80W`\0\x80\xFD[\x81Qa\"-\x81ai\xF6V[`\0` \x82\x84\x03\x12\x15a\x8D\x9DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8D\xB3W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x81\tV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x8D\xDFWa\x8D\xDFa{9V[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a_\xE4`@\x83\x01\x84af\rV[`\0\x80`@\x83\x85\x03\x12\x15a\x8E\x1CW`\0\x80\xFD[\x82Qa\x8E'\x81ac\x97V[` \x84\x01Q\x90\x92Pan\x0E\x81ai\xF6V[`\0` \x82\x84\x03\x12\x15a\x8EJW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E`W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x85\xC9V[` \x81R`\0\x82Q``` \x84\x01Ra\x8E\x88`\x80\x84\x01\x82anNV[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x8E\xC9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E\xDFW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a~,V[`@\x81R`\0a\x8E\xFE`@\x83\x01\x85ahfV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8F(W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F>W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x8BFV[`\0` \x82\x84\x03\x12\x15a\x8F\\W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8FrW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x88\x9FV[`\0` \x82\x84\x03\x12\x15a\x8F\x90W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xA6W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x82\x81V[`\0` \x82\x84\x03\x12\x15a\x8F\xC4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xDAW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x81\xDCV[`\0` \x82\x84\x03\x12\x15a\x8F\xF8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\x0EW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01ay\x9BV[`@\x81R`\0a\x8E\xFE`@\x83\x01\x85abSV[`\0\x82a\x90JWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x90\x87\x81`\x19\x85\x01` \x87\x01a_\xECV[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x90\xBAWa\x90\xBAa{9V[`\0\x03\x92\x91PPV\xFEno slow mode transactions remaining\xA2dipfsX\"\x12 \x14\xB9a\x14\x0E\x8A\x860)\x92G\x13\x01\xBD\x94I\xDBB\xAD/\xCD\xCC\xDC\xB1\xA9l]\x81\x7F\xE2w\x97dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static ENDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x06\x04W`\x005`\xE0\x1C\x80c\x82fD\xF7\x11a\x03\x19W\x80c\xB1\xFB\xD6\x0B\x11a\x01\xA7W\x80c\xDBZP!\x11a\0\xF9W\x80c\xEERU&\x11a\0\xA2W\x80c\xF8\x0F|\xE5\x11a\0|W\x80c\xF8\x0F|\xE5\x14a\r~W\x80c\xFA\xB2\xC4i\x14a\x11sW\x80c\xFB\xF4\x19\x84\x14a\x10\x08W`\0\x80\xFD[\x80c\xEERU&\x14a\x11\x15W\x80c\xEFd\xED\x0E\x14a\x117W\x80c\xF2\xFD\xE3\x8B\x14a\x11`W`\0\x80\xFD[\x80c\xE6\x04\xED\x9E\x11a\0\xD3W\x80c\xE6\x04\xED\x9E\x14a\x10\x9CW\x80c\xE9\xABw\xE5\x14a\x10\xAFW\x80c\xE9\xBCtb\x14a\x11\x02W`\0\x80\xFD[\x80c\xDBZP!\x14a\x10\x16W\x80c\xDCB\xE6\x1B\x14a\x106W\x80c\xDF~h\xE1\x14a\x10VW`\0\x80\xFD[\x80c\xBC\x85\xCA\x86\x11a\x01[W\x80c\xC5\x105\x9F\x11a\x015W\x80c\xC5\x105\x9F\x14a\x0F\xE1W\x80c\xD3\x8C;\x9C\x14a\x0F\xE8W\x80c\xD4\xDE\x8D\x9D\x14a\x10\x08W`\0\x80\xFD[\x80c\xBC\x85\xCA\x86\x14a\x062W\x80c\xC3ES\x0B\x14a\x0F\xBDW\x80c\xC4\xF9\xB2_\x14a\x0F\xD0W`\0\x80\xFD[\x80c\xB3\x14}\x17\x11a\x01\x8CW\x80c\xB3\x14}\x17\x14a\x0FjW\x80c\xB3d\x88\xB8\x14a\x0F\x8AW\x80c\xB7\x0E\xB2c\x14a\x0F\xAAW`\0\x80\xFD[\x80c\xB1\xFB\xD6\x0B\x14a\x0F*W\x80c\xB2\xBBcg\x14a\x0FJW`\0\x80\xFD[\x80c\x91q\xD0\x8B\x11a\x02kW\x80c\x9A\x08\xE55\x11a\x02\x14W\x80c\xA0\xCCc\r\x11a\x01\xEEW\x80c\xA0\xCCc\r\x14a\x0F\nW\x80c\xA7\x80\xA4\xBE\x14a\x0F\nW\x80c\xB1\xC8\xEC+\x14a\x0B\x1DW`\0\x80\xFD[\x80c\x9A\x08\xE55\x14a\x0E}W\x80c\x9E\x85\x14$\x14a\x0E\xCAW\x80c\xA0\x82\xC5\xAA\x14a\x0E\xEAW`\0\x80\xFD[\x80c\x954\xDD>\x11a\x02EW\x80c\x954\xDD>\x14a\r\xA9W\x80c\x96\xC4|o\x14a\r\xC9W\x80c\x98\xCD2\xFE\x14a\x0EjW`\0\x80\xFD[\x80c\x91q\xD0\x8B\x14a\rKW\x80c\x91\xC1\xE3\xD7\x14a\rkW\x80c\x94\xFA\xEF\xE5\x14a\r~W`\0\x80\xFD[\x80c\x8D\n\xCC\x9B\x11a\x02\xCDW\x80c\x8E]X\x8C\x11a\x02\xA7W\x80c\x8E]X\x8C\x14a\r\x07W\x80c\x8FO\x8E\xCC\x14a\r\x1AW\x80c\x91\x0E`j\x14a\r+W`\0\x80\xFD[\x80c\x8D\n\xCC\x9B\x14a\x0C\xC3W\x80c\x8D< \xB1\x14a\x0C\xD6W\x80c\x8D\xA5\xCB[\x14a\x0C\xF6W`\0\x80\xFD[\x80c\x872C8\x11a\x02\xFEW\x80c\x872C8\x14a\x0COW\x80c\x8C=/t\x14a\x0CbW\x80c\x8CX\xE1\n\x14a\x0C\x7FW`\0\x80\xFD[\x80c\x82fD\xF7\x14a\x0B\xFCW\x80c\x85\xC8>\x9D\x14a\x0C/W`\0\x80\xFD[\x80c5c\x9AO\x11a\x04\x96W\x80cZ\0\x92;\x11a\x03\xE8W\x80cl\xFE_\xE4\x11a\x03\x91W\x80cqP\x18\xA6\x11a\x03kW\x80cqP\x18\xA6\x14a\x0B\xF4W\x80cy\xF1$3\x14a\x0B\xFCW\x80c}\xB6\xA2[\x14a\x0C\x1CW`\0\x80\xFD[\x80cl\xFE_\xE4\x14a\x0B\x8EW\x80co;\nr\x14a\x0B\xC1W\x80cp\xBEE|\x14a\x0B\xE1W`\0\x80\xFD[\x80ca\x0B.^\x11a\x03\xC2W\x80ca\x0B.^\x14a\x06\xA7W\x80ce\xDD\x13f\x14a\x0BfW\x80clEup\x14a\x0BnW`\0\x80\xFD[\x80cZ\0\x92;\x14a\x0B\x1DW\x80c[\xB4\xC1&\x14a\x0B=W\x80c]O_\x97\x14a\x0BSW`\0\x80\xFD[\x80c>\xDF,[\x11a\x04JW\x80cO\xCF\xAEX\x11a\x04$W\x80cO\xCF\xAEX\x14a\n\xC1W\x80cTDV\x9D\x14a\n\xEAW\x80cU~\xD1\xBA\x14a\n\xFDW`\0\x80\xFD[\x80c>\xDF,[\x14a\n\\W\x80cB\xC7M\x1D\x14a\n|W\x80cM\x96\xA9\n\x14a\n\x9CW`\0\x80\xFD[\x80c6\xB9\x0CQ\x11a\x04{W\x80c6\xB9\x0CQ\x14a\t\xD6W\x80c8B\xE7^\x14a\t\xF6W\x80c<\xECK\x93\x14a\n\x18W`\0\x80\xFD[\x80c5c\x9AO\x14a\tPW\x80c6\x8EF\x86\x14a\t\xB0W`\0\x80\xFD[\x80c\x1D\x9E\xED\xA5\x11a\x05ZW\x80c'ay\x97\x11a\x05\x03W\x80c-\x035\xAB\x11a\x04\xDDW\x80c-\x035\xAB\x14a\x08\xAAW\x80c/\x9A'D\x14a\x08\xDCW\x80c2\x16\xC0b\x14a\x08\xEFW`\0\x80\xFD[\x80c'ay\x97\x14a\x07\xE9W\x80c,\x8Co\xFB\x14a\x08jW\x80c,\xD7\x1B\x16\x14a\x08\x8AW`\0\x80\xFD[\x80c\"\0`F\x11a\x054W\x80c\"\0`F\x14a\x08&W\x80c\"\x1F\t9\x14a\x08.W\x80c\"\xD4\xA8-\x14a\x08AW`\0\x80\xFD[\x80c\x1D\x9E\xED\xA5\x14a\x07\xE9W\x80c\x1F\x18k'\x14a\x08\tW\x80c!\x04u\x89\x14a\x08\x1EW`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x11a\x05\xBCW\x80c\x16\xCD\xB6\x90\x11a\x05\x96W\x80c\x16\xCD\xB6\x90\x14a\x07\x9EW\x80c\x18\xED\x16\xEB\x14a\x07\xBEW\x80c\x1D\x97\xD2/\x14a\x06\xE7W`\0\x80\xFD[\x80c\x0E\xDA\xAC\xCE\x14a\x06\xE7W\x80c\x0FKP\x9D\x14a\x07\x07W\x80c\x14sWU\x14a\x07^W`\0\x80\xFD[\x80c\x07H\xA2\x19\x11a\x05\xEDW\x80c\x07H\xA2\x19\x14a\x06RW\x80c\rU\xE2k\x14a\x06\xA7W\x80c\x0Ef&[\x14a\x06\xC7W`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x06\tW\x80c\x06\xC0\xBA\xFD\x14a\x062W[`\0\x80\xFD[a\x06\x1Ca\x06\x176`\x04a^\xE7V[a\x11zV[`@Qa\x06)\x91\x90a_\x03V[`@Q\x80\x91\x03\x90\xF3[a\x06Ea\x06@6`\x04a_?V[a\x11\xACV[`@Qa\x06)\x91\x90a_[V[a\x06ea\x06`6`\x04a_?V[a\x11\xDFV[`@Qa\x06)\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[a\x06\xBAa\x06\xB56`\x04a_\xB0V[a\x12\x12V[`@Qa\x06)\x91\x90a`\xA3V[a\x06\xDAa\x06\xD56`\x04a`\xC8V[a\x12#V[`@Qa\x06)\x91\x90aa7V[a\x06\xFAa\x06\xF56`\x04a_?V[a\x12@V[`@Qa\x06)\x91\x90aa\xBFV[a\x07\x1Aa\x07\x156`\x04a_?V[a\x12sV[`@Qa\x06)\x91\x90`\0`\x80\x82\x01\x90P\x82Q\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x07qa\x07l6`\x04aa\xFCV[a\x12\xA6V[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x06)V[a\x07\xB1a\x07\xAC6`\x04ab\x18V[a\x12\xC9V[`@Qa\x06)\x91\x90ab\xD8V[`\xA6Ta\x07\xD1\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\x07\xFCa\x07\xF76`\x04aa\xFCV[a\x13\x1CV[`@Qa\x06)\x91\x90ab\xEBV[a\x08\x1Ca\x08\x176`\x04acVV[a\x13?V[\0[a\x08\x1Ca\x13\xDBV[a\x07\xD1a\x14\x8BV[a\x08\x1Ca\x08<6`\x04aeWV[a\x14\xD0V[a\x07\xD1a\x08O6`\x04ae\xC0V[`\0\x90\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08}a\x08x6`\x04ae\xD9V[a\x18\xA7V[`@Qa\x06)\x91\x90af@V[a\x08\x9Da\x08\x986`\x04a`\xC8V[a\x18\xD4V[`@Qa\x06)\x91\x90af\x98V[a\x07\xD1a\x08\xB86`\x04af\xDBV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA5` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x08\x1Ca\x08\xEA6`\x04ag\x0FV[a\x18\xF2V[a\x08\x1Ca\x08\xFD6`\x04aghV[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\tca\t^6`\x04a_?V[a\x1ADV[`@Qa\x06)\x91\x90`\0`\x80\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`\x07\x0B`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R\x92\x91PPV[a\t\xC3a\t\xBE6`\x04ag\xB6V[a\x1AwV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x06)V[a\t\xE9a\t\xE46`\x04ag\xD3V[a\x1A\xD2V[`@Qa\x06)\x91\x90ah\xA5V[a\n\ta\n\x046`\x04ah\xCAV[a\x1A\xE3V[`@Q\x90Q\x81R` \x01a\x06)V[a\n+a\n&6`\x04a^\xE7V[a\x1B\x01V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x06)V[a\noa\nj6`\x04a`\xC8V[a\x1B-V[`@Qa\x06)\x91\x90ai\x1CV[a\n\x8Fa\n\x8A6`\x04a`\xC8V[a\x1B>V[`@Qa\x06)\x91\x90ai\x9AV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\t\xC3a\n\xCF6`\x04ag\xB6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xAA` R`@\x90 T`\x0F\x0B\x90V[a\x08\x1Ca\n\xF86`\x04ajyV[a\x1B[V[a\x0B\x05a\x1F\x95V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x06)V[a\x0B0a\x0B+6`\x04a`\xC8V[a #V[`@Qa\x06)\x91\x90ak9V[a\x0BEa AV[`@Q\x90\x81R` \x01a\x06)V[`\x9ATa\n\xA9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08\x1Ca\"4V[a\x0B\x81a\x0B|6`\x04ak^V[a\"wV[`@Qa\x06)\x91\x90ak\x92V[a\x08\x1Ca\x0B\x9C6`\x04ak\xB5V[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xAE` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x0B\xD4a\x0B\xCF6`\x04a_\xB0V[a\"\x88V[`@Qa\x06)\x91\x90ak\xEAV[a\x08\x1Ca\x0B\xEF6`\x04af\xDBV[a\"\x99V[a\x08\x1Ca#\x1CV[a\x0C\x0Fa\x0C\n6`\x04a^\xE7V[a#0V[`@Qa\x06)\x91\x90alIV[a\x08\x1Ca\x0C*6`\x04al\x8DV[a#\\V[a\x0CBa\x0C=6`\x04ag\xD3V[a%.V[`@Qa\x06)\x91\x90am\x02V[a\x08\x1Ca\x0C]6`\x04am\x8CV[a%?V[a\x0Cja/\x91V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x06)V[a\x08\x1Ca\x0C\x8D6`\x04am\xE0V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x08\x1Ca\x0C\xD16`\x04an\x19V[a1}V[a\x0C\xE9a\x0C\xE46`\x04ae\xD9V[a2\xB7V[`@Qa\x06)\x91\x90an\x8CV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\n\xA9V[a\x08\x1Ca\r\x156`\x04an\xB7V[a2\xC8V[`\xB0T`\x01`\x01`\xA0\x1B\x03\x16a\n\xA9V[a\r>a\r96`\x04an\xFCV[a38V[`@Qa\x06)\x91\x90ao\x18V[a\r^a\rY6`\x04ao\x8DV[a3yV[`@Qa\x06)\x91\x90ao\xC1V[a\n\xA9a\ry6`\x04ae\xC0V[a3\x8AV[a\r\x91a\r\x8C6`\x04ah\xCAV[a4KV[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x06)V[a\r\xBCa\r\xB76`\x04af\xDBV[a4iV[`@Qa\x06)\x91\x90ap7V[a\r\xDCa\r\xD76`\x04apJV[a5\x03V[`@Qa\x06)\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x08\x1Ca\x0Ex6`\x04apfV[a5KV[a\x0E\x90a\x0E\x8B6`\x04a^\xE7V[a6\xB3V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x06)V[a\x0E\xDDa\x0E\xD86`\x04an\xFCV[a6\xDFV[`@Qa\x06)\x91\x90aq\tV[a\x0E\xFDa\x0E\xF86`\x04a`\xC8V[a7 V[`@Qa\x06)\x91\x90aq`V[a\x0F\x1Da\x0F\x186`\x04a_\xB0V[a71V[`@Qa\x06)\x91\x90ar&V[a\x0F=a\x0F86`\x04ao\x8DV[a7jV[`@Qa\x06)\x91\x90ar9V[a\x0F]a\x0FX6`\x04a`\xC8V[a7{V[`@Qa\x06)\x91\x90ar\x95V[a\x0F}a\x0Fx6`\x04ae\xD9V[a7\x98V[`@Qa\x06)\x91\x90ar\xF2V[a\x0F\x9Da\x0F\x986`\x04ag\xD3V[a7\xBDV[`@Qa\x06)\x91\x90as*V[a\x0BEa\x0F\xB86`\x04am\x8CV[a7\xCEV[a\x0Cja\x0F\xCB6`\x04am\x8CV[a8kV[`\xA2T`\x01`\x01`@\x1B\x03\x16a\x07\xD1V[`\0a\t\xC3V[a\x0F\xFBa\x0F\xF66`\x04ao\x8DV[a9KV[`@Qa\x06)\x91\x90as\x7FV[g\r\xE0\xB6\xB3\xA7d\0\0a\t\xC3V[a\x10)a\x10$6`\x04ao\x8DV[a9\\V[`@Qa\x06)\x91\x90as\xE6V[a\x10Ia\x10D6`\x04an\xFCV[a9mV[`@Qa\x06)\x91\x90at\x83V[a\x10ia\x10d6`\x04a^\xE7V[a9\xAEV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x0F\x90\x81\x0B\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x06)V[a\x08\x1Ca\x10\xAA6`\x04an\x19V[a9\xDAV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x06)V[a\x08\x1Ca\x11\x106`\x04at\xDBV[a>^V[a\x11(a\x11#6`\x04auRV[a>\xECV[`@Qa\x06)\x93\x92\x91\x90aumV[a\x0BEa\x11E6`\x04auRV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA1` R`@\x90 T\x90V[a\x08\x1Ca\x11n6`\x04af\xDBV[a@\x07V[`\xA6a\x0BEV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\nV[\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\x81V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83av\x9DV[a\x12\x1Aa[\xD7V[a\x11\xA6\x82awLV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82aw\xB3V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83ax\xC5V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83ax\xE1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11\xA66\x83\x90\x03\x83\x01\x83ay?V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82azsV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11\xA66\x83\x90\x03\x83\x01\x83az\xC1V[`\0[\x81\x81\x10\x15a\x13\x94W6`\0\x84\x84\x84\x81\x81\x10a\x13_Wa\x13_az\xDDV[\x90P` \x02\x81\x01\x90a\x13q\x91\x90az\xF3V[\x91P\x91Pa\x13\x7F\x82\x82a@\x94V[PP\x80\x80a\x13\x8C\x90a{OV[\x91PPa\x13BV[P`\xA6\x80T\x82\x91\x90`\0\x90a\x13\xB3\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a{hV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x08\xFDW`\xA8`\0\x82` \x01\x80Qa\x14I\x90a{\x93V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x14\x84`\x01\x83\x01\x82a\\\x10V[PPa\x14\x14V[`\xA6\x80T`\0\x91\x90\x82\x90a\x14\xA7\x90`\x01`\x01`@\x1B\x03\x16a{\xB6V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA6T\x16\x91\x90PV[\x80Q`\0\x03a\x14\xDEW`\0\x80\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x15'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`@Q\x80\x91\x03\x90\xFD[P`\x01\x19\x84\x01a\x15pW`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16\x15a\x15nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P[``\x84\x90\x1Ca\x15\xA6\x813\x81\x14a\x15\xA0W`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaP?V[\x83aP?V[`\x01\x85\x14\x80\x15\x90a\x15\xCCWP`\0\x85\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x16DW`\x9AT`@Qc\xD6\x93\xC5\xF1`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD6\x93\xC5\xF1\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16?W=`\0\x80>=`\0\xFD[PPPP[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01Ra\x16\xC8\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xB8\x91\x90a{\xDCV[3\x85`\x01`\x01`\x80\x1B\x03\x16aP\x94V[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x17\x15Bb\x03\xF4\x80a{hV[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8A\x81R` \x01\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x17\x99\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xB7\x92\x91` \x01a|\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a\x17\xDF\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x18K\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPV[a\x18\xCB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x11\xA6\x82a|>V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11\xA6\x82a}\x05V[`\0Z\x90Pa\x19\0\x85aP\xBDV[`\0[\x83\x81\x10\x15a\x19\xCBW6`\0\x86\x86\x84\x81\x81\x10a\x19 Wa\x19 az\xDDV[\x90P` \x02\x81\x01\x90a\x192\x91\x90az\xF3V[\x91P\x91Pa\x19@\x82\x82a@\x94V[\x84Za\x19L\x90\x86a}\x11V[\x11\x15a\x19\xB6W`\xB1T`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\x9DW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x19\xB1W=`\0\x80>=`\0\xFD[PPPP[PP\x80\x80a\x19\xC3\x90a{OV[\x91PPa\x19\x03V[P`\xB1T`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x19\xE9\x90\x85a}\x11V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A%W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A9W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a}:V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAF` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1A\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x91\x90PV[a\x1A\xDAa\\\xCEV[a\x11\xA6\x82a~\xACV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11\xA66\x83\x90\x03\x83\x01\x83a~\xB8V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a~\xF9V[a\x1B5a\\\xFCV[a\x11\xA6\x82a\x7F\x9AV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82a\x80hV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1B{WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1B\x95WP0;\x15\x80\x15a\x1B\x95WP`\0T`\xFF\x16`\x01\x14[a\x1C\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x1EV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C*W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C2aQ\x05V[a\x1C\xA6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPaQxV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x87\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xB0\x80T\x89\x85\x16\x90\x84\x16\x17\x90U`\xB1\x80T\x87\x85\x16\x90\x84\x16\x17\x90U`\x9D\x80T\x93\x8B\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1D!\x90`\0\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Db\x91\x90a{\xDCV[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1D\xA6\x90`\x01\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE7\x91\x90a{\xDCV[`\x9C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@\x80Qc\x17\x17U\xB1`\xE0\x1B\x81R\x90Q\x91\x90\x92\x16\x91c\x17\x17U\xB1\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1EEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ei\x91\x90a{\xDCV[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x01\x81\x90R`\xA7\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1FEW\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\xF4Wa\x1E\xF4az\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\xAF\x90\x92R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1F=\x81a\x80\xF0V[\x91PPa\x1E\xCDV[P\x80\x15a\x1F\x8CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F\xD2W\x81Qa\x1F\xD8V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a \x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x11\xA6\x82a\x81UV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a!4\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!`\x90a\x81aV[\x80\x15a!\xADW\x80`\x1F\x10a!\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xADV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x90W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\xE7\x92\x90\x91`\x04\x01a\x81\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\" WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\"\x1D\x91\x81\x01\x90a\x81\xB7V[`\x01[a\"-WP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x08\xFD\x81`\0aQ\xEDV[a\"\x7Fa]+V[a\x11\xA6\x82a\x81\xD0V[a\"\x90a]bV[a\x11\xA6\x82a\x82\x02V[a\"\xA1aT\x8CV[`\xB2T`\x01`\x01`\xA0\x1B\x03\x16\x15a\"\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Falready set\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x1EV[`\xB2\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a#$aT\x8CV[a#.`\0aT\xE6V[V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x82KV[a#e\x86aP\xBDV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a#|W`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a$\x16W\x81\x87\x87\x83\x81\x81\x10a#\xC9Wa#\xC9az\xDDV[\x90P` \x02\x81\x01\x90a#\xDB\x91\x90az\xF3V[`@Q` \x01a#\xED\x93\x92\x91\x90a\x82gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a$\x0F\x90a{OV[\x90Pa#\xAEV[P`\xB1T`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a$\x88W=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a%$W6`\0\x88\x88\x84\x81\x81\x10a$\xACWa$\xACaz\xDDV[\x90P` \x02\x81\x01\x90a$\xBE\x91\x90az\xF3V[\x91P\x91Pa$\xCC\x82\x82a@\x94V[`\xA6\x80T`\x01\x91\x90`\0\x90a$\xEB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a{hV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a%\x1C\x90a{OV[\x91PPa$\x8FV[PPPPPPPPV[a%6a]\x92V[a\x11\xA6\x82a\x82\xC2V[30\x14a%KW`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a%`Wa%`az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a%{Wa%{a{\xF9V[\x90P`\x01\x81` \x81\x11\x15a%\x91Wa%\x91a{\xF9V[\x03a&UW`\0a%\xA5\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a%\xB2\x91\x90a~\xF9V[\x90Pa%\xC2\x81`\0\x01Q\x86aU8V[\x80Qa%\xCD\x90aU\x96V[`\x9AT`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a&KW=`\0\x80>=`\0\xFD[PPPPPa/\x8BV[`\x02\x81` \x81\x11\x15a&iWa&ia{\xF9V[\x03a'\rW`\0a&}\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a&\x8A\x91\x90av\x81V[\x90Pa&\x9A\x81`\0\x01Q\x86aU8V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA6T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a&\x1DV[`\x07\x81` \x81\x11\x15a'!Wa'!a{\xF9V[\x03a'\x8FW`\x9AT`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a'X\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'\x86W=`\0\x80>=`\0\xFD[PPPPa/\x8BV[`\t\x81` \x81\x11\x15a'\xA3Wa'\xA3a{\xF9V[\x03a)3W`\0a'\xB7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a'\xC4\x91\x90a\x83\xB2V[`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a'\xF6\x90`\0\x90`\x04\x01a\x80\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(7\x91\x90a{\xDCV[`\x9AT` \x83\x01Q`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x90\x91\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xB3\x91\x90a{\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a(\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P\x80Qa)\x03\x90\x86aU8V[`\x9AT`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE6q\xB1k\x90a&\x1D\x90\x84\x90`\x04\x01ao\x18V[`\n\x81` \x81\x11\x15a)GWa)Ga{\xF9V[\x03a)\xA8W`\0a)[\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a)h\x91\x90av\x81V[\x90Pa)x\x81`\0\x01Q\x86aU8V[`\x9AT`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x1F\xB3!\x90a&\x1D\x90\x84\x90`\x04\x01a_[V[`\x0B\x81` \x81\x11\x15a)\xBCWa)\xBCa{\xF9V[\x03a*TW`\0a)\xD0\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a)\xDD\x91\x90ax\xE1V[\x90Pa)\xED\x81`\0\x01Q\x86aU8V[\x80Qa)\xF8\x90aV%V[`\xB0T`@\x80Qc\x0FKP\x9D`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x0FKP\x9D\x90`\x84\x01a&\x1DV[`\x12\x81` \x81\x11\x15a*hWa*ha{\xF9V[\x03a*\xBEW`\0a*|\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a*\x89\x91\x90a\x83\xCEV[\x80Q` \x82\x01Q`@Qc\xC9\xFE\x9A\xC3`\xE0\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xC9\xFE\x9A\xC3\x91a&\x1D\x91`\x04\x01ap7V[`\x13\x81` \x81\x11\x15a*\xD2Wa*\xD2a{\xF9V[\x03a+AW`\0a*\xE6\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a*\xF3\x91\x90av\nV[\x90Pa+\x03\x81`\0\x01Q\x86aU8V[\x80Qa+\x0E\x90aV%V[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua/\x8BV[`\x15\x81` \x81\x11\x15a+UWa+Ua{\xF9V[\x03a+\xF1W`\0a+i\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a+v\x91\x90av\x9DV[\x90Pa+\x86\x81`\0\x01Q\x86aU8V[a+\x93\x81``\x01QaU\x96V[`\x9AT`@\x80Qc\x07H\xA2\x19`\xE0\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R``\x83\x01Q`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x07H\xA2\x19\x90`\x84\x01a&\x1DV[`\x1B\x81` \x81\x11\x15a,\x05Wa,\x05a{\xF9V[\x03a,KW`\x9AT`\xA6T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a'X\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x84\x02V[`\x1D\x81` \x81\x11\x15a,_Wa,_a{\xF9V[\x03a,\x96W`\x9AT`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a'X\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\r\x81` \x81\x11\x15a,\xAAWa,\xAAa{\xF9V[\x03a/1W`\xB0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\x13W=`\0\x80>=`\0\xFD[PPPP`\0`\x9B`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x94\x91\x90\x81\x01\x90a\x84/V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xB1Wa-\xB1ac\xD0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a-\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a.\xBDW`\xAA`\0\x84\x83\x81Q\x81\x10a-\xFFWa-\xFFaz\xDDV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a.AWa.Aaz\xDDV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xAA`\0\x85\x84\x81Q\x81\x10a.mWa.maz\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a.\xB5\x81a{OV[\x91PPa-\xE0V[Pa.\xC8`\x01aV%V[`\x9AT`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a.\xF8\x90\x84\x90`\x04\x01a\x84\xC8V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/&W=`\0\x80>=`\0\xFD[PPPPPPa/\x8BV[`\x18\x81` \x81\x11\x15a/EWa/Ea{\xF9V[\x03a\x06\x04W`\x9AT`\xA6T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a'X\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a\x84\x02V[PPPPV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a0&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a0\x84\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xB0\x90a\x81aV[\x80\x15a0\xFDW\x80`\x1F\x10a0\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xC3ES\x0B`\xE0\x1B\x81R\x92\x93P0\x92c\xC3ES\x0B\x92a17\x92\x90\x91`\x04\x01a\x81\x95V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a1pWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra1m\x91\x81\x01\x90a\x84\xDBV[`\x01[a\"-W`\0\x92PPP\x90V[`\0\x82\x82`\0\x81\x81\x10a1\x92Wa1\x92az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a1\xADWa1\xADa{\xF9V[\x90P`\0\x81` \x81\x11\x15a1\xC3Wa1\xC3a{\xF9V[\x03a2 W`\0a1\xD7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a1\xE4\x91\x90a\x85\x95V[\x80QQ\x90\x91P`\x02\x14a2\x1AW\x80Q\x80Q`\xA0\x90\x91\x01Qa2\x05\x91\x90aV\x8EV[\x80QQa2\x1A\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[Pa2tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[`\xA6\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a2\x8D\x83a{\xB6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[a2\xBFa]\xBBV[a\x11\xA6\x82a\x86>V[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra33\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra3\x11\x90a\x86JV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x14\xD0V[PPPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x83\xB2V[a3\x81a]\xE8V[a\x11\xA6\x82a\x86nV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a3\xB8W`\0\x82\x81R`\xAB` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x11\xA6V[`\xB0T`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xAB\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4+\x91\x90a\x81\xB7V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x86\xC5V[`\xAD` R`\0\x90\x81R`@\x90 \x80Ta4\x82\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta4\xAE\x90a\x81aV[\x80\x15a4\xFBW\x80`\x1F\x10a4\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\xFBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a4\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\tV[`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a5\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x15\x1EV[\x81`\xA8`\0\x83` \x01\x80Q\x80\x91\x90a5\xEE\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a6Z\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\xA0V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x87\xE6V[a7(a^'V[a\x11\xA6\x82a\x88\x02V[`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82a\x89\0V[a7ra]\xE8V[a\x11\xA6\x82a\x89UV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x11\xA6\x82a\x89\xA0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x11\xA6\x82a\x8AcV[a7\xC5a^OV[a\x11\xA6\x82a\x8A\xC2V[`\0\x80\x83\x83`\0\x81\x81\x10a7\xE4Wa7\xE4az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a7\xFFWa7\xFFa{\xF9V[\x90P`\x13\x81` \x81\x11\x15a8\x15Wa8\x15a{\xF9V[\x03a8`W`\0a8)\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a86\x91\x90av\nV[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a8TW`\0a8WV[\x80Q[\x92PPPa\"-V[P`\0\x94\x93PPPPV[`\0\x80\x83\x83`\0\x81\x81\x10a8\x81Wa8\x81az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a8\x9CWa8\x9Ca{\xF9V[\x90P`\t\x81` \x81\x11\x15a8\xB2Wa8\xB2a{\xF9V[\x03a8\xE1W`\0a8\xC6\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90a\x83\xB2V[` \x01Q\x92Pa\"-\x91PPV[`\n\x81` \x81\x11\x15a8\xF5Wa8\xF5a{\xF9V[\x03a9\x16W`\0a9\t\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90av\x81V[`\x0B\x81` \x81\x11\x15a9*Wa9*a{\xF9V[\x03a8`W`\0a9>\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a8\xD3\x91\x90ax\xE1V[a9Sa]\xE8V[a\x11\xA6\x82a\x8BlV[a9da^nV[a\x11\xA6\x82a\x8B\xC8V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x8CuV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x11\xA66\x83\x90\x03\x83\x01\x83a\x8C\x91V[`\0\x82\x82`\0\x81\x81\x10a9\xEFWa9\xEFaz\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a:\nWa:\na{\xF9V[\x90P3`\x01\x82` \x81\x11\x15a:!Wa:!a{\xF9V[\x03a:+W`\0\x80\xFD[`\x07\x82` \x81\x11\x15a:?Wa:?a{\xF9V[\x03a:\x85W`\0a:S\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a:`\x91\x90a\x86\xC5V[\x90Pa:\x7Fa:maWJV[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aP\x94V[Pa<\xE1V[`\x12\x82` \x81\x11\x15a:\x99Wa:\x99a{\xF9V[\x03a:\xCDW`gT`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a:\xC8W`\0\x80\xFD[a<\xE1V[`\x15\x82` \x81\x11\x15a:\xE1Wa:\xE1a{\xF9V[\x03a;\x87W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xAE` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x03\x83Rb\x15\xD3\x95`\xEA\x1B\x91\x83\x01\x91\x90\x91R`\xFF\x16a;=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P`\0a;M\x84`\x01\x81\x88a\x82\xCEV[\x81\x01\x90a;Z\x91\x90av\x9DV[\x90Pa:\x7F\x81``\x01Q``\x1C`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPaP?V[`\x1B\x82` \x81\x11\x15a;\x9BWa;\x9Ba{\xF9V[\x03a;\xB1W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\x1D\x82` \x81\x11\x15a;\xC5Wa;\xC5a{\xF9V[\x03a;\xDBW`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\r\x82` \x81\x11\x15a;\xEFWa;\xEFa{\xF9V[\x03a<\x05W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\x18\x82` \x81\x11\x15a<\x19Wa<\x19a{\xF9V[\x03a</W`gT`\x01`\x01`\xA0\x1B\x03\x16a:\xABV[`\xB2T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x80\x15a<]WP`\x13\x82` \x81\x11\x15a<[Wa<[a{\xF9V[\x14[\x80\x15a<\x84WPFb\x15\xF9\0\x14\x80a<wWPFb\x16\x1C(\x14[\x80a<\x84WPFb\x15\xF9\x02\x14[\x15a<\x90WP0a<\xE1V[a<\x9Ea<\x9BaWJV[PV[`\xAC\x80Tb\x0FB@\x91\x90`\0\x90a<\xB9\x90\x84\x90`\x0F\x0Ba\x8C\xD9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a=.Bb\x03\xF4\x80a{hV[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA8\x93P\x90a=\x97\x82a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a>\x03\x92`\x01\x85\x01\x92\x90\x91\x01\x90a\\JV[PP\x81Q`\xA7\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x87\x16`4\x82\x01Ra>\xE5\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra>\xA7\x90a\x86JV[\x85\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x14\xD0\x92PPPV[PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA8\x82R\x85\x83 `\xA7T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a?v\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta?\xA2\x90a\x81aV[\x80\x15a?\xEFW\x80`\x1F\x10a?\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a?\xEFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a?\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a@\x0FaT\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x15\x1EV[a<\x9B\x81aT\xE6V[`\0\x82\x82`\0\x81\x81\x10a@\xA9Wa@\xA9az\xDDV[\x91\x90\x91\x015`\xF8\x1C\x90P` \x81\x11\x15a@\xC4Wa@\xC4a{\xF9V[\x90P`\0\x81` \x81\x11\x15a@\xDAWa@\xDAa{\xF9V[\x03aA\xC8W`\0a@\xEE\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90a@\xFB\x91\x90a\x85\x95V[\x80QQ\x90\x91P`\x02\x14aAjW\x80Q\x80Q`\xA0\x90\x91\x01QaA\x1C\x91\x90aV\x8EV[\x80QQaAI\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[`\0\x93\x92PPPV[aW\xBDV[PPV[\x80QQaAU\x90aV%V[\x80QQaAj\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91aA\x9A\x91`\x04\x01aq\tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aA\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a%$W=`\0\x80>=`\0\xFD[`\x02\x81` \x81\x11\x15aA\xDCWaA\xDCa{\xF9V[\x03aC-W`\0aA\xF0\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aA\xFD\x91\x90a\x8D:V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaB\x13\x91aV\x8EV[\x80QQaB.\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01RaB\xB8\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xAD\x91\x90a\x8DnV[\x83Q` \x01QaX\x0BV[`\x9AT\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA6T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01aA\x9AV[`\x03\x81` \x81\x11\x15aCAWaCAa{\xF9V[\x03aDEW`\0aCU\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aCb\x91\x90a\x8D\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15aC\xAEW` \x82\x01Q\x83QaC\xA9\x91\x90a\x8D\xBFV[aC\xB1V[`\0[`\x9BT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\0W`\0\x80\xFD[PZ\xF1\x15\x80\x15aD\x14W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA9UPa33\x91PPV[`\x0F\x81` \x81\x11\x15aDYWaDYa{\xF9V[\x03aEZW`\0aDm\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aDz\x91\x90a\x8D\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\xA9T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15aD\xC2W\x81Q\x83QaD\xBD\x91\x90a\x8D\xBFV[aD\xC5V[`\0[`\x9CT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91aD\xFE\x91\x85\x91\x90`\x04\x01a\x8D\xE7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE\x18W`\0\x80\xFD[PZ\xF1\x15\x80\x15aE,W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA9UPa33\x91PPV[`\x04\x81` \x81\x11\x15aEnWaEna{\xF9V[\x03aF3W`\x9AT`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90aE\xAA\x90\x88\x90\x88\x90`\x04\x01a\x83!V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aE\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xEC\x91\x90a\x8E\tV[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a>\xE5Wc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAF` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90UPPPPPV[`\x05\x81` \x81\x11\x15aFGWaFGa{\xF9V[\x03aF\xACW`\x9AT`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\x8CW=`\0\x80>=`\0\xFD[`\x06\x81` \x81\x11\x15aF\xC0WaF\xC0a{\xF9V[\x14\x80aF\xDDWP`\x16\x81` \x81\x11\x15aF\xDBWaF\xDBa{\xF9V[\x14[\x15aG\xD1W`\0aF\xF1\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aF\xFE\x91\x90a\x8E8V[` \x81\x01QQQ\x90\x91PaG\x11\x90aV%V[`@\x81\x01QQQaG!\x90aV%V[`\0`@Q\x80``\x01`@R\x80\x83\x81R` \x01aGI\x84` \x01Q`\0\x01Q`\0\x01Qa3\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aGl\x84`@\x01Q`\0\x01Q`\0\x01Qa3\x8AV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R`\xB0T`@Qc\x11\x17\x8F-`\xE3\x1B\x81R\x92\x93P\x16\x90c\x88\xBCyh\x90aG\xA3\x90\x84\x90`\x04\x01a\x8ElV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG\xBDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A9W=`\0\x80>=`\0\xFD[`\x0C\x81` \x81\x11\x15aG\xE5WaG\xE5a{\xF9V[\x03aH]W`\0aG\xF9\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aH\x06\x91\x90a\x8E\xB7V[``\x81\x01QQQ\x90\x91PaH\x19\x90aV%V[`\xB0T``\x82\x01QQQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cx\xF0\xD3\xCE\x90\x83\x90aH@\x90a3\x8AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\x9A\x92\x91\x90a\x8E\xEBV[`\x08\x81` \x81\x11\x15aHqWaHqa{\xF9V[\x03aI\x0FW`@\x80Q``\x81\x01\x82R`\xA7T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaH\xB9\x81`\x01aQ\xEDV[\x80Q`\xA7\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\t\x81` \x81\x11\x15aI#WaI#a{\xF9V[\x03aI\xBAW`\0aI7\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aID\x91\x90a\x8F\x16V[\x80Q\x80Q`\xA0\x90\x91\x01Q\x91\x92PaIZ\x91aV\x8EV[\x80QQaIu\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaI\x8A\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\xE6q\xB1k`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE6q\xB1k\x91aA\x9A\x91`\x04\x01ao\x18V[`\n\x81` \x81\x11\x15aI\xCEWaI\xCEa{\xF9V[\x03aJeW`\0aI\xE2\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aI\xEF\x91\x90a\x8D:V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92PaJ\x05\x91aV\x8EV[\x80QQaJ \x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaJ5\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\xBF\x1F\xB3!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xBF\x1F\xB3!\x91aA\x9A\x91`\x04\x01a_[V[`\x1E\x81` \x81\x11\x15aJyWaJya{\xF9V[\x03aK\x87W`\0aJ\x8D\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aJ\x9A\x91\x90a\x8FJV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaJ\xB0\x91aV\x8EV[\x80QQaJ\xCB\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaJ\xE0\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x88<q\x85`\xE0\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88<q\x85\x90`\x84\x01aA\x9AV[`\x1F\x81` \x81\x11\x15aK\x9BWaK\x9Ba{\xF9V[\x03aL\xA9W`\0aK\xAF\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aK\xBC\x91\x90a\x8FJV[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaK\xD2\x91aV\x8EV[\x80QQaK\xED\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80QQaL\x02\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`@\x81\x81\x01\x80Q`\x99`\0R`\xAF` \x90\x81R\x7F\xA4\x8D.\x89\xAF\x1D3\xD7u\xF6\x88L\x9F1:\xE6\x12\x0B\x98c\xA0\xCF\xD7\xE5!\x14v\xCA\xD1M\xD7[\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x17\x90U`\x9AT\x85Q\x93Q\x85Qc\x1C\xD4\x0F_`\xE3\x1B\x81R\x85Q`\x04\x82\x01R\x92\x85\x01Q\x90\x93\x16`$\x83\x01R\x92\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`D\x84\x01R`\x0F\x0B`d\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\xA0z\xF8\x90`\x84\x01aA\x9AV[`\x10\x81` \x81\x11\x15aL\xBDWaL\xBDa{\xF9V[\x03aL\xF4W`\x9AT`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x13\x81` \x81\x11\x15aM\x08WaM\x08a{\xF9V[\x03aM\x8EW`\0aM\x1C\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aM)\x91\x90a\x8F~V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaM?\x91aV\x8EV[\x80QQaMZ\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xAB\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x14\x81` \x81\x11\x15aM\xA2WaM\xA2a{\xF9V[\x03aM\xD9W`\x9AT`@Qc&fm-`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99\x99\xB4\xB4\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x17\x81` \x81\x11\x15aM\xEDWaM\xEDa{\xF9V[\x03aN\x95W`\0aN\x01\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aN\x0E\x91\x90a\x8F\xB2V[\x90PaN!\x81`\0\x01Q` \x01QaU\x96V[\x80QQaN<\x90aAEaA@\x85aA7\x88`\x01\x81\x8Ca\x82\xCEV[\x80Q\x80Q``\x90\x91\x01QaNP\x91\x90aV\x8EV[\x80QQaNe\x90g\r\xE0\xB6\xB3\xA7d\0\0aW>V[`\x9AT\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91aA\x9A\x91`\x04\x01aa\xBFV[`\x19\x81` \x81\x11\x15aN\xA9WaN\xA9a{\xF9V[\x03aN\xE0W`\x9AT`@QcJg\xD9\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\xCF\xB2\x02\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x1A\x81` \x81\x11\x15aN\xF4WaN\xF4a{\xF9V[\x03aO+W`\x9AT`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x1C\x81` \x81\x11\x15aO?WaO?a{\xF9V[\x03aO\xF4W`\0aOS\x83`\x01\x81\x87a\x82\xCEV[\x81\x01\x90aO`\x91\x90a\x8F\xE6V[`\xB0T\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA2z%\n\x90\x84\x90aO\x89\x90a3\x8AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xA6\x92\x91\x90a\x90\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aO\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xE9\x91\x90a\x81\xB7V[\x90Pa>\xE5\x81aU\x96V[` \x81` \x81\x11\x15aP\x08WaP\x08a{\xF9V[\x03a\x06\x04W`\x9AT`@Qc~\x92v\xD7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c~\x92v\xD7\x90aF~\x90\x86\x90\x86\x90`\x04\x01a\x83!V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80TaPb\x90a\x81aV[\x90P`\0\x03aAEW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x90\x91 \x82Qa33\x92\x84\x01\x90a\\JV[`\x01`\x01`\xA0\x1B\x03\x83\x16aP\xA7W`\0\x80\xFD[`\x9ATa33\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA6T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14aAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`\0Ta\x01\0\x90\x04`\xFF\x16aQpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[a#.aX\xE2V[`\0Ta\x01\0\x90\x04`\xFF\x16aQ\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[aAE\x82\x82aYVV[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x90\xC4`#\x919\x90aRAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA8` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aR\xA0\x90a\x81aV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaR\xCC\x90a\x81aV[\x80\x15aS\x19W\x80`\x1F\x10aR\xEEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aS\x19V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aR\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA8`\0\x84`@\x01\x80Q\x80\x91\x90aS:\x90a{\xB6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aSw`\x01\x83\x01\x82a\\\x10V[PP\x81\x80aS\x92WPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aS\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[PFazi\x03aT\x02W` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aF~\x92`\x04\x01a\x81\x95V[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aT5\x92\x90\x91`\x04\x01a\x81\x95V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aTOW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aT`WP`\x01[a/\x8BWb\x03\xD0\x90Z\x11\x15\x80aT\x80WPaT|`\x02\x82a\x90-V[Z\x11\x15[\x15aT\x87W\xFE[a/\x8BV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a#.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x15\x1EV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aU[WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a33W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[`\0\x81\x81R`\xA0` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a<\x9BW`\xA2\x80T`\0\x90aU\xCD\x90`\x01`\x01`@\x1B\x03\x16a{\xB6V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\xA0` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA2T\x90\x93\x16\x81R`\xA1\x90\x92R\x90 UV[`\x01\x81\x14\x80aV4WP`\x02\x81\x14[\x80aVUWP`\0\x81\x81R`\xA0` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90aAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x15\x1E\x91\x90ap7V[``\x82\x90\x1C`\0\x90\x81R`\xA5` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aV\xB7\x83a{\xB6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a33WaW\x08\x81`\x01`\x01`@\x1B\x03\x16aY\xDBV[`@Q` \x01aW\x18\x91\x90a\x90OV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x15\x1E\x91`\x04\x01ap7V[aAE\x82\x82`\0aX\x0BV[`\x9BT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xB8\x91\x90a{\xDCV[\x90P\x90V[`\0a\x11\xA6aW\xCAaZzV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aX'\x86a\x90\x94V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aXvW`\0\x80\xFD[PZ\xF1\x15\x80\x15aX\x8AW=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAA` R`@\x81 \x80T\x84\x92\x90aX\xB6\x90\x84\x90`\x0F\x0Ba\x8C\xD9V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16aYMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[a#.3aT\xE6V[`\0Ta\x01\0\x90\x04`\xFF\x16aY\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x15\x1EV[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[```\0aY\xE8\x83aZ\xF5V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\x07WaZ\x07ac\xD0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aZ1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aZ;WP\x93\x92PPPV[`\0aW\xB8\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaZ\xA9`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a[>Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a[jWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a[\x88Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a[\xA0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a[\xB4Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a[\xC6W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x11\xA6W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80Ta\\\x1C\x90a\x81aV[`\0\x82U\x80`\x1F\x10a\\,WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a<\x9B\x91\x90a^\xA2V[\x82\x80Ta\\V\x90a\x81aV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\\xW`\0\x85Ua\\\xBEV[\x82`\x1F\x10a\\\x91W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\\\xBEV[\x82\x80\x01`\x01\x01\x85U\x82\x15a\\\xBEW\x91\x82\x01[\x82\x81\x11\x15a\\\xBEW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\\\xA3V[Pa\\\xCA\x92\x91Pa^\xA2V[P\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x81\x01a\\\xF7a]+V[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xE0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a]\xDBa]+V[\x81R` \x01a\\\xF7a]+V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90a\\\x03V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01R\x90\x81\x01a]\xDBa]+V[`@Q\x80`\xA0\x01`@R\x80a^\x81a^\xB7V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[[\x80\x82\x11\x15a\\\xCAW`\0\x81U`\x01\x01a^\xA3V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a^\xF9W`\0\x80\xFD[a\"-\x83\x83a^\xD5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11\xA6V[`\0`\x80\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a_QW`\0\x80\xFD[a\"-\x83\x83a_-V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11\xA6V[`\0`\xA0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a_\xC2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xD8W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a_\x9EV[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a`\x07W\x81\x81\x01Q\x83\x82\x01R` \x01a_\xEFV[\x83\x81\x11\x15a/\x8BWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra`0\x81` \x86\x01` \x86\x01a_\xECV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[a`\x88\x82\x82Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x82\x01Q`\xA0`\x80\x85\x01Ra_\xE4`\xA0\x85\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84a`DV[`\0`@\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a`\xDAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a`\xF0W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a`\xB6V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aa\x10V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R``\x83\x01\x84Q`@\x83\x86\x01R\x81\x81Q\x80\x84R`\x80\x87\x01\x91P`\x80\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15aa\x97W`\x7F\x19\x88\x86\x03\x01\x83Raa\x85\x85\x85Qa`\x18V[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01aaiV[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90aa\xB6\x81\x83a`\xFCV[\x95\x94PPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x11\xA6V[`\0`@\x82\x84\x03\x12\x15ab\x0EW`\0\x80\xFD[a\"-\x83\x83a`\xB6V[`\0` \x82\x84\x03\x12\x15ab*W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ab@W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\"-W`\0\x80\xFD[`\0a\x01\0\x82Q\x80Q\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R`@\x81\x01Q`\x0F\x0B`@\x86\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x88\x01R\x80`\x80\x84\x01Q\x16`\x80\x88\x01RPP`\xA0\x81\x01Q`\x0F\x0B`\xA0\x86\x01RP` \x83\x01Qab\xC0`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Q\x81`\xE0\x86\x01Raa\xB6\x82\x86\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84abSV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x11\xA6V[`\0\x80\x83`\x1F\x84\x01\x12ac\x1DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15acOW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aciW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15ac\x7FW`\0\x80\xFD[ac\x8B\x85\x82\x86\x01ac\x0BV[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a<\x9BW`\0\x80\xFD[\x805ac\xB4\x81ac\x97V[\x91\x90PV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\x08Wad\x08ac\xD0V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15ad\xE0Wad\xE0ac\xD0V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12ad\xF9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ae\x12Wae\x12ac\xD0V[ae%`\x1F\x82\x01`\x1F\x19\x16` \x01ad\xB8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15ae:W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aemW`\0\x80\xFD[\x845\x93P` \x85\x015ae\x7F\x81ac\x97V[\x92Pae\x8D`@\x86\x01ac\xB9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ae\xA8W`\0\x80\xFD[ae\xB4\x87\x82\x88\x01ad\xE8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ae\xD2W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ae\xEBW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15af\x01W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a^\xD5V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01af!V[` \x81R`\0\x82Q``` \x84\x01Raf\\`\x80\x84\x01\x82af\rV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Rafz\x83\x83af\rV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPaa\xB6\x82\x82af\rV[` \x81R`\x01`\x01`\xA0\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra_\xE4``\x84\x01\x82a`\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a<\x9BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15af\xEDW`\0\x80\xFD[\x815a\"-\x81af\xC6V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15ag%W`\0\x80\xFD[ag.\x85af\xF8V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15agIW`\0\x80\xFD[agU\x87\x82\x88\x01ac\x0BV[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15agzW`\0\x80\xFD[ag\x82ac\xE6V[ag\x8B\x83af\xF8V[\x81Rag\x99` \x84\x01af\xF8V[` \x82\x01Rag\xAA`@\x84\x01af\xF8V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15ag\xC8W`\0\x80\xFD[\x815a\"-\x81ac\x97V[`\0` \x82\x84\x03\x12\x15ag\xE5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xFBW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a_-V[`\0\x81Q\x80Q\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R`@\x81\x01Q`\x0F\x0B`@\x85\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x87\x01R\x80`\x80\x84\x01Q\x16`\x80\x87\x01RPPP` \x82\x01Q`\xC0`\xA0\x85\x01Ra_\xE4`\xC0\x85\x01\x82a`\x18V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R`\0``\x82\x01Q`\x80``\x85\x01Ra_\xE4`\x80\x85\x01\x82ah\x07V[` \x81R`\0a\"-` \x83\x01\x84ahfV[`\0` \x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ah\xDCW`\0\x80\xFD[a\"-\x83\x83ah\xB8V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aa,W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ah\xFAV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01RaiJ`\xE0\x85\x01\x82ah\xE6V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Raig\x82\x82a`\xFCV[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90aa\xB6\x81\x83a`\x18V[` \x81R`\0\x82Q`@` \x84\x01Rai\xB6``\x84\x01\x82a`\xFCV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Raa\xB6\x82\x82af\rV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15ai\xECWai\xECac\xD0V[P`\x05\x1B` \x01\x90V[\x80`\x0F\x0B\x81\x14a<\x9BW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aj\x16W`\0\x80\xFD[\x815` aj+aj&\x83ai\xD3V[ad\xB8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ajJW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805aja\x81ai\xF6V[\x83R\x91\x83\x01\x91\x83\x01ajNV[P\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aj\x92W`\0\x80\xFD[\x865aj\x9D\x81af\xC6V[\x95P` \x87\x015aj\xAD\x81af\xC6V[\x94P`@\x87\x015aj\xBD\x81af\xC6V[\x93P``\x87\x015aj\xCD\x81af\xC6V[\x92P`\x80\x87\x015aj\xDD\x81af\xC6V[\x91P`\xA0\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aj\xF8W`\0\x80\xFD[ak\x04\x89\x82\x8A\x01aj\x05V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x01`\x01`\x80\x1B\x03\x81Q\x16\x82R`\0` \x82\x01Q`@` \x85\x01Ra_\xE4`@\x85\x01\x82af\rV[` \x81R`\0a\"-` \x83\x01\x84ak\x11V[`\0`\xC0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15akpW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ak\x86W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01akLV[` \x81R`\0a\"-` \x83\x01\x84ah\x07V[\x805\x80\x15\x15\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15ak\xC8W`\0\x80\xFD[\x825ak\xD3\x81af\xC6V[\x91Pak\xE1` \x84\x01ak\xA5V[\x90P\x92P\x92\x90PV[` \x81Ral/` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Ra_\xE4`\xC0\x84\x01\x82a`\x18V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x11\xA6V[\x805`\xFF\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15al\xA6W`\0\x80\xFD[al\xAF\x87af\xF8V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15al\xCAW`\0\x80\xFD[al\xD6\x89\x82\x8A\x01ac\x0BV[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91Pal\xF6`\x80\x88\x01al|V[\x90P\x92\x95P\x92\x95P\x92\x95V[` \x81Ram1` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Ra_\xE4`\xA0\x84\x01\x82a`\x18V[`\0\x80\x83`\x1F\x84\x01\x12am]W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15amtW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15acOW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15am\xA1W`\0\x80\xFD[\x835am\xAC\x81af\xC6V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15am\xC7W`\0\x80\xFD[am\xD3\x86\x82\x87\x01amKV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15am\xF3W`\0\x80\xFD[\x825am\xFE\x81ac\x97V[\x91P` \x83\x015an\x0E\x81ai\xF6V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15an,W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15anBW`\0\x80\xFD[ac\x8B\x85\x82\x86\x01amKV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rans``\x85\x01\x82ah\x07V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Raa\xB6\x82\x82ah\x07V[` \x81R`\0a\"-` \x83\x01\x84anNV[\x805`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15an\xCCW`\0\x80\xFD[an\xD5\x84an\x9FV[\x92P` \x84\x015an\xE5\x81ac\x97V[\x91Pan\xF3`@\x85\x01ac\xB9V[\x90P\x92P\x92P\x92V[`\0`\xC0\x82\x84\x03\x12\x15ao\x0EW`\0\x80\xFD[a\"-\x83\x83akLV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ao\x9FW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ao\xB5W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01ao{V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Ra_\xE4a\x01\0\x84\x01\x82a`\x18V[` \x81R`\0a\"-` \x83\x01\x84a`\x18V[`\0`\xE0\x82\x84\x03\x12\x15ap\\W`\0\x80\xFD[a\"-\x83\x83ao{V[`\0\x80`@\x83\x85\x03\x12\x15apyW`\0\x80\xFD[ap\x82\x83af\xF8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ap\x9EW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15ap\xB2W`\0\x80\xFD[ap\xBAac\xE6V[ap\xC3\x83af\xF8V[\x81R` \x83\x015ap\xD3\x81af\xC6V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ap\xEAW`\0\x80\xFD[ap\xF6\x88\x82\x86\x01ad\xE8V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Raq\x8F`\xC0\x85\x01\x82ah\xE6V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Raa\xB6\x81\x83a`\x18V[aq\xF2\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xA0``\x85\x01Rar\r`\xA0\x85\x01\x82a`\x18V[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a\"-` \x83\x01\x84aq\xC1V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Rar\xB1``\x85\x01\x82a`\xFCV[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15ajnW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90ar\xD2V[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Ra_\xE4`\x80\x84\x01\x82a`\xFCV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q\x15\x15`@\x82\x01R`\0`@\x83\x01Q`\x80``\x84\x01Rasb`\xA0\x84\x01\x82ah\x07V[\x90P``\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\x80\x85\x01Raa\xB6\x82\x82ah\x07V[` \x81Rap\x1C` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`@\x81\x01Q`\x01`\x01`\x80\x1B\x03\x80\x82\x16`@\x85\x01R\x80``\x84\x01Q\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15at\x15W\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01as\xF6V[PPP\x83\x01Q`\xE0`\x80\x84\x01Rat0a\x01\0\x84\x01\x82a`\x18V[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01RatN\x83\x83a`\xFCV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPatl\x82\x82a`\xFCV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\xC0\x81\x01a\x11\xA6\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01R`\xFF`@\x82\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15at\xF3W`\0\x80\xFD[at\xFC\x86an\x9FV[\x94P` \x86\x015au\x0C\x81ac\x97V[\x93Pau\x1A`@\x87\x01ac\xB9V[\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15au5W`\0\x80\xFD[auA\x88\x82\x89\x01amKV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15audW`\0\x80\xFD[a\"-\x82af\xF8V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rau\xB0`\xC0\x85\x01\x82a`\x18V[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15au\xDBW`\0\x80\xFD[au\xE3ac\xE6V[\x90P\x815\x81R` \x82\x015` \x82\x01Rau\xFF`@\x83\x01af\xF8V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15av\x1CW`\0\x80\xFD[a\"-\x83\x83au\xC9V[`\0`\x80\x82\x84\x03\x12\x15av8W`\0\x80\xFD[av@ad\x0EV[\x90P\x815\x81R` \x82\x015avT\x81ac\x97V[` \x82\x01Rave`@\x83\x01ac\xB9V[`@\x82\x01Ravv``\x83\x01af\xF8V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15av\x93W`\0\x80\xFD[a\"-\x83\x83av&V[`\0`\x80\x82\x84\x03\x12\x15av\xAFW`\0\x80\xFD[av\xB7ad\x0EV[\x825\x81R` \x83\x015av\xC9\x81ac\x97V[` \x82\x01Rav\xDA`@\x84\x01ac\xB9V[`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aw\x05W`\0\x80\xFD[aw\rad0V[\x90Paw\x19\x83\x83av&V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[aw@\x84\x82\x85\x01ad\xE8V[` \x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83av\xF3V[`\0\x82`\x1F\x83\x01\x12awiW`\0\x80\xFD[\x815` awyaj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aw\x98W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805\x83R\x91\x83\x01\x91\x83\x01aw\x9CV[`\0`@\x826\x03\x12\x15aw\xC5W`\0\x80\xFD[aw\xCDad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aw\xE4W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12aw\xF7W`\0\x80\xFD[\x815` ax\x07aj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15ax&W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ax^W\x805\x86\x81\x11\x15axBW`\0\x80\x81\xFD[axP6\x86\x83\x8B\x01\x01ad\xE8V[\x84RP\x91\x83\x01\x91\x83\x01ax*V[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15axuW`\0\x80\xFD[ax\x816\x85\x89\x01awXV[\x90\x85\x01RP\x91\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15ax\xA1W`\0\x80\xFD[ax\xA9ad\x0EV[\x90P\x815\x81R` \x82\x015` \x82\x01Rave`@\x83\x01ac\xB9V[`\0`\x80\x82\x84\x03\x12\x15ax\xD7W`\0\x80\xFD[a\"-\x83\x83ax\x8FV[`\0`\x80\x82\x84\x03\x12\x15ax\xF3W`\0\x80\xFD[ax\xFBad\x0EV[\x825\x81R` \x83\x015ay\r\x81ac\x97V[` \x82\x01R`@\x83\x015ay \x81ai\xF6V[`@\x82\x01R``\x83\x015ay3\x81ai\xF6V[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ayQW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aysWaysac\xD0V[`@Ray\x7F\x83ac\xB9V[\x81R` \x83\x015ay\x8F\x81af\xC6V[` \x82\x01R\x93\x92PPPV[`\0\x81\x83\x03a\x01\0\x81\x12\x15ay\xAFW`\0\x80\xFD[ay\xB7ac\xE6V[\x91P`\xC0\x81\x12\x15ay\xC7W`\0\x80\xFD[Pay\xD0adRV[\x825\x81R` \x83\x015ay\xE2\x81ai\xF6V[` \x82\x01R`@\x83\x015ay\xF5\x81ai\xF6V[`@\x82\x01Raz\x06``\x84\x01af\xF8V[``\x82\x01Raz\x17`\x80\x84\x01af\xF8V[`\x80\x82\x01R`\xA0\x83\x015az*\x81ai\xF6V[`\xA0\x82\x01R\x81Raz=`\xC0\x83\x01ac\xA9V[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15az[W`\0\x80\xFD[azg\x84\x82\x85\x01ad\xE8V[`@\x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83ay\x9BV[`\0`@\x82\x84\x03\x12\x15az\x91W`\0\x80\xFD[az\x99ad0V[\x90P\x815az\xA6\x81ac\x97V[\x81R` \x82\x015az\xB6\x81ai\xF6V[` \x82\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15az\xD3W`\0\x80\xFD[a\"-\x83\x83az\x7FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a{\nW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a{$W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15acOW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a{aWa{aa{9V[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a{\x8AWa{\x8Aa{9V[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80a{\xACWa{\xACa{9V[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a{\xD2Wa{\xD2a{9V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a{\xEEW`\0\x80\xFD[\x81Qa\"-\x81af\xC6V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qa|0\x81`\x01\x85\x01` \x87\x01a_\xECV[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0``\x826\x03\x12\x15a|PW`\0\x80\xFD[a|Xac\xE6V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a|oW`\0\x80\xFD[a|{6\x83\x87\x01aj\x05V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a|\x91W`\0\x80\xFD[a|\x9D6\x83\x87\x01aj\x05V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a|\xB6W`\0\x80\xFD[Pazg6\x82\x86\x01aj\x05V[`\0`@\x82\x84\x03\x12\x15a|\xD5W`\0\x80\xFD[a|\xDDad0V[\x90P\x815a|\xEA\x81af\xC6V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0a\x11\xA66\x83a|\xC3V[`\0\x82\x82\x10\x15a}#Wa}#a{9V[P\x03\x90V[\x805`\x07\x81\x90\x0B\x81\x14ac\xB4W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a}LW`\0\x80\xFD[a}Tad\x0EV[\x825a}_\x81af\xC6V[\x81R` \x83\x015a}o\x81ac\x97V[` \x82\x01Ra}\x80`@\x84\x01a}(V[`@\x82\x01Ray3``\x84\x01a}(V[`\0\x81\x83\x03`\xC0\x81\x12\x15a}\xA4W`\0\x80\xFD[a}\xACad0V[\x91P`\xA0\x81\x12\x15a}\xBCW`\0\x80\xFD[Pa}\xC5adtV[\x825\x81R` \x83\x015a}\xD7\x81ai\xF6V[` \x82\x01R`@\x83\x015a}\xEA\x81ai\xF6V[`@\x82\x01Ra}\xFB``\x84\x01af\xF8V[``\x82\x01Ra~\x0C`\x80\x84\x01af\xF8V[`\x80\x82\x01R\x81R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a~>W`\0\x80\xFD[a~Fad\x0EV[\x90P\x815a~S\x81ac\x97V[\x81R` \x82\x015a~c\x81ai\xF6V[` \x82\x01R`@\x82\x015a~v\x81ai\xF6V[`@\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a~\x94W`\0\x80\xFD[a~\xA0\x84\x82\x85\x01a}\x91V[``\x83\x01RP\x92\x91PPV[`\0a\x11\xA66\x83a~,V[`\0` \x82\x84\x03\x12\x15a~\xCAW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a~\xECWa~\xECac\xD0V[`@R\x915\x82RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x7F\x0BW`\0\x80\xFD[a\x7F\x13ac\xE6V[\x825\x81R` \x83\x015a\x7F%\x81ac\x97V[` \x82\x01Rag\xAA`@\x84\x01ac\xB9V[`\0\x82`\x1F\x83\x01\x12a\x7FGW`\0\x80\xFD[\x815` a\x7FWaj&\x83ai\xD3V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x7FvW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ajnW\x805a\x7F\x8D\x81ac\x97V[\x83R\x91\x83\x01\x91\x83\x01a\x7FzV[`\0`@\x826\x03\x12\x15a\x7F\xACW`\0\x80\xFD[a\x7F\xB4ad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x7F\xCBW`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15a\x7F\xE0W`\0\x80\xFD[a\x7F\xE8ad\x0EV[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x7F\xFEW`\0\x80\xFD[a\x80\n6\x82\x86\x01a\x7F6V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a\x80\"W`\0\x80\xFD[a\x80.6\x82\x86\x01awXV[`@\x83\x01RPa\x80@``\x84\x01af\xF8V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80[W`\0\x80\xFD[Paw@6\x82\x86\x01ad\xE8V[`\0`@\x826\x03\x12\x15a\x80zW`\0\x80\xFD[a\x80\x82ad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x80\x99W`\0\x80\xFD[a\x80\xA56\x83\x87\x01awXV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80\xBBW`\0\x80\xFD[Paw@6\x82\x86\x01aj\x05V[` \x81\x01`\x02\x83\x10a\x80\xEAWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a{\xD2Wa{\xD2a{9V[`\0`@\x82\x84\x03\x12\x15a\x81\x1BW`\0\x80\xFD[a\x81#ad0V[\x90Pa\x81.\x82ac\xB9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x81IW`\0\x80\xFD[aw@\x84\x82\x85\x01aj\x05V[`\0a\x11\xA66\x83a\x81\tV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x81uW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A\xCCWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a_\xE4`@\x83\x01\x84a`\x18V[`\0` \x82\x84\x03\x12\x15a\x81\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0a\x11\xA66\x83a}\x91V[`\0`\xA0\x82\x84\x03\x12\x15a\x81\xEEW`\0\x80\xFD[a\x81\xF6ad0V[\x90Paw\x19\x83\x83ax\x8FV[`\0a\x11\xA66\x83a\x81\xDCV[`\0``\x82\x84\x03\x12\x15a\x82 W`\0\x80\xFD[a\x82(ac\xE6V[\x90P\x815\x81Ra\x82:` \x83\x01ac\xB9V[` \x82\x01Rau\xFF`@\x83\x01af\xF8V[`\0``\x82\x84\x03\x12\x15a\x82]W`\0\x80\xFD[a\"-\x83\x83a\x82\x0EV[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x82\x93W`\0\x80\xFD[a\x82\x9Bad0V[\x90Pa\x82\xA7\x83\x83au\xC9V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0a\x11\xA66\x83a\x82\x81V[`\0\x80\x85\x85\x11\x15a\x82\xDEW`\0\x80\xFD[\x83\x86\x11\x15a\x82\xEBW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a_\xE4` \x83\x01\x84\x86a\x82\xF8V[`\0`\xC0\x82\x84\x03\x12\x15a\x83GW`\0\x80\xFD[a\x83OadRV[\x90P\x815\x81R` \x82\x015a\x83c\x81ac\x97V[` \x82\x01Ra\x83t`@\x83\x01ac\xB9V[`@\x82\x01Ra\x83\x85``\x83\x01ac\xB9V[``\x82\x01Ra\x83\x96`\x80\x83\x01ac\xB9V[`\x80\x82\x01Ra\x83\xA7`\xA0\x83\x01af\xF8V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x83\xC4W`\0\x80\xFD[a\"-\x83\x83a\x835V[`\0` \x82\x84\x03\x12\x15a\x83\xE0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\xF6W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a|\xC3V[`@\x81R`\0a\x84\x16`@\x83\x01\x85\x87a\x82\xF8V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x84BW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84XW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x84iW`\0\x80\xFD[\x80Qa\x84waj&\x82ai\xD3V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x84\x96W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x84\xBDW\x83Qa\x84\xAE\x81ac\x97V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x84\x9BV[\x97\x96PPPPPPPV[` \x81R`\0a\"-` \x83\x01\x84af\rV[`\0` \x82\x84\x03\x12\x15a\x84\xEDW`\0\x80\xFD[\x81Qa\"-\x81ac\x97V[`\0`\xC0\x82\x84\x03\x12\x15a\x85\nW`\0\x80\xFD[a\x85\x12adRV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a\x850\x81ac\x97V[`@\x82\x01Ra\x85A``\x83\x01ak\xA5V[``\x82\x01R`\x80\x82\x015a\x83\x96\x81ai\xF6V[`\0`\xE0\x82\x84\x03\x12\x15a\x85fW`\0\x80\xFD[a\x85nad0V[\x90Pa\x85z\x83\x83a\x84\xF8V[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aw4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x85\xA7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xBDW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x85TV[`\0``\x82\x84\x03\x12\x15a\x85\xDBW`\0\x80\xFD[a\x85\xE3ac\xE6V[\x90P\x815a\x85\xF0\x81ac\x97V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x86\x0CW`\0\x80\xFD[a\x86\x18\x85\x83\x86\x01a}\x91V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x861W`\0\x80\xFD[Pazg\x84\x82\x85\x01a}\x91V[`\0a\x11\xA66\x83a\x85\xC9V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1A\xCCW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0a\x11\xA66\x83a\x85TV[`\0` \x82\x84\x03\x12\x15a\x86\x8CW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x86\xAEWa\x86\xAEac\xD0V[`@R\x90P\x80a\x86\xBD\x83ac\xB9V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x86\xD7W`\0\x80\xFD[a\"-\x83\x83a\x86zV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14ac\xB4W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x87\x1BW`\0\x80\xFD[a\x87#ad\x96V[\x825\x81R` \x83\x015a\x875\x81ac\x97V[` \x82\x01R`@\x83\x015a\x87H\x81ac\x97V[`@\x82\x01Ra\x87Y``\x84\x01a\x86\xE1V[``\x82\x01Ra\x87j`\x80\x84\x01a\x86\xE1V[`\x80\x82\x01Ra\x87{`\xA0\x84\x01af\xF8V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x87\x94W`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x87\xB2W`\0\x80\xFD[a\x87\xBAac\xE6V[\x825a\x87\xC5\x81ac\x97V[\x81Ra\x87\xD3` \x84\x01ac\xB9V[` \x82\x01R`@\x83\x015ag\xAA\x81af\xC6V[`\0`\xC0\x82\x84\x03\x12\x15a\x87\xF8W`\0\x80\xFD[a\"-\x83\x83a\x84\xF8V[`\0`@\x826\x03\x12\x15a\x88\x14W`\0\x80\xFD[a\x88\x1Cad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x883W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x88HW`\0\x80\xFD[a\x88Pac\xE6V[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x88fW`\0\x80\xFD[a\x88r6\x82\x86\x01a\x7F6V[` \x83\x01RPa\x88\x84`@\x84\x01af\xF8V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x80[W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x88\xB1W`\0\x80\xFD[a\x88\xB9ac\xE6V[\x90Pa\x88\xC5\x83\x83a\x82\x0EV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88\xE0W`\0\x80\xFD[a\x88\xEC\x84\x82\x85\x01ad\xE8V[` \x83\x01RP`\x80\x82\x015au\xFF\x81ai\xF6V[`\0a\x11\xA66\x83a\x88\x9FV[`\0`\xC0\x82\x84\x03\x12\x15a\x89\x1EW`\0\x80\xFD[a\x89&adRV[\x90P\x815\x81R` \x82\x015` \x82\x01Ra\x89B`@\x83\x01al|V[`@\x82\x01R``\x82\x015a\x85A\x81ac\x97V[`\0`\xE0\x826\x03\x12\x15a\x89gW`\0\x80\xFD[a\x89oad0V[a\x89y6\x84a\x89\x0CV[\x81R`\xC0\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x89\x94W`\0\x80\xFD[aw@6\x82\x86\x01ad\xE8V[`\0`@\x826\x03\x12\x15a\x89\xB2W`\0\x80\xFD[a\x89\xBAad0V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x89\xD1W`\0\x80\xFD[a\x89\xDD6\x83\x87\x01awXV[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x89\xF4W`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x8A\x07W`\0\x80\xFD[\x805a\x8A\x15aj&\x82ai\xD3V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x8A4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x8ARW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x8A9V[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x8AuW`\0\x80\xFD[a\x8A}ac\xE6V[\x825a\x8A\x88\x81ac\x97V[\x81R` \x83\x015a\x8A\x98\x81ai\xF6V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\xB6W`\0\x80\xFD[azg6\x82\x86\x01awXV[`\0`\x80\x826\x03\x12\x15a\x8A\xD4W`\0\x80\xFD[a\x8A\xDCad\x0EV[\x825a\x8A\xE7\x81ac\x97V[\x81Ra\x8A\xF5` \x84\x01ak\xA5V[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8B\x14W`\0\x80\xFD[a\x8B 6\x83\x87\x01a}\x91V[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15a\x8B9W`\0\x80\xFD[Pa~\xA06\x82\x86\x01a}\x91V[`\0`\xE0\x82\x84\x03\x12\x15a\x8BXW`\0\x80\xFD[a\x8B`ad0V[\x90Pa\x85z\x83\x83a\x835V[`\0a\x11\xA66\x83a\x8BFV[`\0\x82`\x1F\x83\x01\x12a\x8B\x89W`\0\x80\xFD[a\x8B\x91ac\xE6V[\x80``\x84\x01\x85\x81\x11\x15a\x8B\xA3W`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x8B\xBDW\x805\x84R` \x93\x84\x01\x93\x01a\x8B\xA5V[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x8B\xDAW`\0\x80\xFD[a\x8B\xE2adtV[a\x8B\xEC6\x84a\x8BxV[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x8C\x08W`\0\x80\xFD[a\x8C\x146\x83\x87\x01ad\xE8V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x8C-W`\0\x80\xFD[a\x8C96\x83\x87\x01awXV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x8CRW`\0\x80\xFD[Pa\x8C_6\x82\x86\x01awXV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0`\xC0\x82\x84\x03\x12\x15a\x8C\x87W`\0\x80\xFD[a\"-\x83\x83a\x89\x0CV[`\0``\x82\x84\x03\x12\x15a\x8C\xA3W`\0\x80\xFD[a\x8C\xABac\xE6V[\x825a\x8C\xB6\x81ac\x97V[\x81R` \x83\x015a\x8C\xC6\x81ai\xF6V[` \x82\x01R`@\x83\x015ag\xAA\x81ai\xF6V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x8D\x0CWa\x8D\x0Ca{9V[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x8D1Wa\x8D1a{9V[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8DLW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8DbW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01av\xF3V[`\0` \x82\x84\x03\x12\x15a\x8D\x80W`\0\x80\xFD[\x81Qa\"-\x81ai\xF6V[`\0` \x82\x84\x03\x12\x15a\x8D\x9DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8D\xB3W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x81\tV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x8D\xDFWa\x8D\xDFa{9V[\x03\x93\x92PPPV[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a_\xE4`@\x83\x01\x84af\rV[`\0\x80`@\x83\x85\x03\x12\x15a\x8E\x1CW`\0\x80\xFD[\x82Qa\x8E'\x81ac\x97V[` \x84\x01Q\x90\x92Pan\x0E\x81ai\xF6V[`\0` \x82\x84\x03\x12\x15a\x8EJW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E`W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x85\xC9V[` \x81R`\0\x82Q``` \x84\x01Ra\x8E\x88`\x80\x84\x01\x82anNV[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x8E\xC9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8E\xDFW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a~,V[`@\x81R`\0a\x8E\xFE`@\x83\x01\x85ahfV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x8F(W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F>W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x8BFV[`\0` \x82\x84\x03\x12\x15a\x8F\\W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8FrW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x88\x9FV[`\0` \x82\x84\x03\x12\x15a\x8F\x90W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xA6W`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x82\x81V[`\0` \x82\x84\x03\x12\x15a\x8F\xC4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8F\xDAW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01a\x81\xDCV[`\0` \x82\x84\x03\x12\x15a\x8F\xF8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x90\x0EW`\0\x80\xFD[a_\xE4\x84\x82\x85\x01ay\x9BV[`@\x81R`\0a\x8E\xFE`@\x83\x01\x85abSV[`\0\x82a\x90JWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x90\x87\x81`\x19\x85\x01` \x87\x01a_\xECV[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x90\xBAWa\x90\xBAa{9V[`\0\x03\x92\x91PPV\xFEno slow mode transactions remaining\xA2dipfsX\"\x12 \x14\xB9a\x14\x0E\x8A\x860)\x92G\x13\x01\xBD\x94I\xDBB\xAD/\xCD\xCC\xDC\xB1\xA9l]\x81\x7F\xE2w\x97dsolcC\0\x08\r\x003";
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
        ///Calls the contract's `rebalanceVlp` (0xdf7e68e1) function
        pub fn rebalance_vlp(
            &self,
            p: RebalanceVlp,
        ) -> ::ethers::contract::builders::ContractCall<M, RebalanceVlp> {
            self.0
                .method_hash([223, 126, 104, 225], (p,))
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
        ///Calls the contract's `setVertexGateway` (0x70be457c) function
        pub fn set_vertex_gateway(
            &self,
            vertex_gateway: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 190, 69, 124], vertex_gateway)
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
    ///Container type for all input parameters for the `rebalanceVlp` function with signature `rebalanceVlp((uint32,int128,int128))` and selector `0xdf7e68e1`
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
    #[ethcall(name = "rebalanceVlp", abi = "rebalanceVlp((uint32,int128,int128))")]
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
    ///Container type for all input parameters for the `setVertexGateway` function with signature `setVertexGateway(address)` and selector `0x70be457c`
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
    #[ethcall(name = "setVertexGateway", abi = "setVertexGateway(address)")]
    pub struct SetVertexGatewayCall {
        pub vertex_gateway: ::ethers::core::types::Address,
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
        SetVertexGateway(SetVertexGatewayCall),
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
            if let Ok(decoded) =
                <SetVertexGatewayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetVertexGateway(decoded));
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
                Self::SetVertexGateway(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::SetVertexGateway(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<SetVertexGatewayCall> for EndpointCalls {
        fn from(value: SetVertexGatewayCall) -> Self {
            Self::SetVertexGateway(value)
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
    ///Container type for all return fields from the `rebalanceVlp` function with signature `rebalanceVlp((uint32,int128,int128))` and selector `0xdf7e68e1`
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
    ///`RebalanceVlp(uint32,int128,int128)`
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
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub base_amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_amount: i128,
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
