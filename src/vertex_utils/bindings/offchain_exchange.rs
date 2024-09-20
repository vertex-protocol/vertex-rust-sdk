pub use offchain_exchange::*;
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
pub mod offchain_exchange {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dropDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropDigest"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("digest"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dropOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropOrder"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dumpFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dumpFees"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("filledAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("filledAmounts"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getAllFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllFeeRates"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("users"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.FeeRates[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllVirtualBooks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllVirtualBooks"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedMakerFees",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedTakerFees",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCustomFeeAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCustomFeeAddresses",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("startAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("limit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDigest"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEndpoint"),
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
                    ::std::borrow::ToOwned::to_owned("getFeeFractionX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeeFractionX18"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                                name: ::std::borrow::ToOwned::to_owned("taker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLpParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLpParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.LpParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarketInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMarketInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("m"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.MarketInfo",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinSize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinSize"),
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
                    ::std::borrow::ToOwned::to_owned("getOrderFilledAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOrderFilledAmounts",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSizeIncrement"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSizeIncrement"),
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
                    ::std::borrow::ToOwned::to_owned("getVirtualBook"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getVirtualBook"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("incrementFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("incrementFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerFee"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_endpoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
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
                                        "struct IEndpoint.MatchOrderAMM",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerLinkedSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.MatchOrdersWithSigner",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("setFeeRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeRate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                                name: ::std::borrow::ToOwned::to_owned("feeRate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IOffchainExchange.FeeRates",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("filledAmount"),
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
                    ::std::borrow::ToOwned::to_owned("swapAMM"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapAMM"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txn"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("updateCollectedFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateCollectedFees",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedFees"),
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
                    ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                                name: ::std::borrow::ToOwned::to_owned("makerRateX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerRateX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int64"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateMarket"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quoteId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("virtualBook"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sizeIncrement"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minSize"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpSpreadX18"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FillOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FillOrder"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("priceX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("expiration"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isTaker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("baseDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quoteDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OFFCHAINEXCHANGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaN\x92\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x0BW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01*W\x80c\xCE\x93>Y\x11a\0\xBDW\x80c\xE1\xE7\x18\x8D\x11a\0\x8CW\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x064W\x80c\xFBB\x0CY\x14a\x06GW\x80c\xFF\x0B\xE9\xEF\x14a\x06ZW`\0\x80\xFD[\x80c\xE1\xE7\x18\x8D\x14a\x05\xF5W\x80c\xF2\xB2c1\x14a\x06!W`\0\x80\xFD[\x80c\xCE\x93>Y\x14a\x05\x07W\x80c\xCE\xBA\x89S\x14a\x05\x0FW\x80c\xD8\x95 *\x14a\x05\x99W\x80c\xDE\x10x\xBD\x14a\x05\xB9W`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x11a\0\xF9W\x80c\xAE\xD8\xE9g\x14a\x04\xBDW\x80c\xB5\xCB\xD7\x0E\x14a\x04\xCEW\x80c\xB6\n\xAA|\x14a\x04\xE1W\x80c\xB7mx\xE3\x14a\x04\xF4W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04eW\x80c\x93v\0>\x14a\x04vW\x80c\x95\xEE`q\x14a\x04\x89W\x80c\xA3\x9B\x9D\xCD\x14a\x04\xAAW`\0\x80\xFD[\x80cH!\xC8\xB5\x11a\x01\xA2W\x80cqP\x18\xA6\x11a\x01qW\x80cqP\x18\xA6\x14a\x03\xF6W\x80cx\xF0\xD3\xCE\x14a\x03\xFEW\x80c\x81&\t\xF1\x14a\x04\x11W\x80c\x88\xBCyh\x14a\x04RW`\0\x80\xFD[\x80cH!\xC8\xB5\x14a\x03@W\x80cH\\\xC9U\x14a\x03\x94W\x80cf\xF8{\xD1\x14a\x03\xA7W\x80cp|\x8BX\x14a\x03\xEEW`\0\x80\xFD[\x80c-\xA1\xC5\x9B\x11a\x01\xDEW\x80c-\xA1\xC5\x9B\x14a\x02\xC4W\x80c5\xEDNm\x14a\x02\xD7W\x80c?\xCE\xEA(\x14a\x02\xEAW\x80c@\xF1\xA3M\x14a\x03\nW`\0\x80\xFD[\x80c\x0F,\x87\x8E\x14a\x02\x10W\x80c\x0FKP\x9D\x14a\x02BW\x80c\x1D\x02\x9BM\x14a\x02WW\x80c\x1FL\xE0\x16\x14a\x02\xB1W[`\0\x80\xFD[a\x02#a\x02\x1E6`\x04a?]V[a\x06\x94V[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Ua\x02P6`\x04a?\xA5V[a\x07.V[\0[a\x02ja\x02e6`\x04a?\xC1V[a\x0B V[`@Qa\x029\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x02Ua\x02\xBF6`\x04a?\xEDV[a\x0B\xE1V[a\x02Ua\x02\xD26`\x04a@MV[a\x0CZV[a\x02#a\x02\xE56`\x04a@\xCFV[a\x0E\x14V[a\x02\xFDa\x02\xF86`\x04a@\xFDV[a\x0E\xDAV[`@Qa\x029\x91\x90aA+V[a\x03-a\x03\x186`\x04aAxV[`\x9E` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x029V[a\x03\x82a\x03N6`\x04a?\xC1V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x81R`\x9D\x83R\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[`@Q\x90Q`\x0F\x0B\x81R` \x01a\x029V[a\x02Ua\x03\xA26`\x04aA\x91V[a\x10\nV[a\x03\xD6a\x03\xB56`\x04a?\xC1V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9C` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x029V[a\x02Ua\x12\xCFV[a\x02Ua\x17YV[a\x02Ua\x04\x0C6`\x04aA\xBFV[a\x17mV[a\x02Ua\x04\x1F6`\x04aB\x06V[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02Ua\x04`6`\x04aB4V[a\x1A\xA9V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD6V[a\x02Ua\x04\x846`\x04aC\x8AV[a \xACV[a\x04\x9Ca\x04\x976`\x04aC\x8AV[a \xD9V[`@Q\x90\x81R` \x01a\x029V[a\x02#a\x04\xB86`\x04aC\xC0V[a\"YV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD6V[a\x03-a\x04\xDC6`\x04aD\x08V[a\"\xA0V[a\x03-a\x04\xEF6`\x04a?\xC1V[a#@V[a\x02Ua\x05\x026`\x04aDVV[a#mV[a\x02\xFDa(#V[a\x02Ua\x05\x1D6`\x04aD\xAEV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x90\x94\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x90\x94\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x94\x90\x95\x16\x93\x90\x93\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90UV[a\x05\xACa\x05\xA76`\x04aE\xF8V[a*\xEEV[`@Qa\x029\x91\x90aF\xBAV[a\x02Ua\x05\xC76`\x04aG\x1DV[`\0\x91\x82R`\x9E` R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02Ua\x06\x036`\x04aAxV[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UV[a\x03-a\x06/6`\x04a?\xC1V[a,\x86V[a\x02Ua\x06B6`\x04aGBV[a,\xB4V[a\x02Ua\x06U6`\x04aC\x8AV[a-DV[a\x02#a\x06h6`\x04a?\xC1V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xA5` \x90\x81R`@\x80\x83 T`\xA6\x90\x92R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[``\x82\x81\x1C`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x94\x85\x01\x82RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x90\x0B\x92\x85\x01\x92\x90\x92R`\x01`\x80\x1B\x90\x91\x04`\xFF\x16\x90\x83\x01\x81\x90R\x90\x91\x82\x91\x82\x03a\x07\x17WP`@\x80Q``\x81\x01\x82R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R[` \x81\x01Q\x90Q`\x07\x91\x82\x0B\x96\x91\x0B\x94P\x92PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xB5a\x02e`@\x84\x01` \x85\x01a?\xC1V[\x90P`\0a\x07\xD1a\x07\xCC`@\x85\x01` \x86\x01a?\xC1V[a-\xA8V[\x90P`\0a\x07\xE5`\x80\x85\x01``\x86\x01aG_V[`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b$\xA8)`\xE9\x1B\x81RP\x90a\x08#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P\x80`@\x01Q\x15a\x08\x8EW\x81`@\x01Q\x83`@\x01` \x81\x01\x90a\x08F\x91\x90aG_V[a\x08P\x91\x90aG\xE7V[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb\x04\x955`\xEC\x1B` \x82\x01R\x90`\x0F\x0B\x15a\x08\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[`\0\x81`@\x01Qa\x08\xA3W\x81` \x01Qa\x08\xA6V[\x81Q[\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16c\xC7\x16|\xF5a\x08\xCA`@\x89\x01` \x8A\x01a?\xC1V[a\x08\xDA``\x8A\x01`@\x8B\x01aG_V[a\t\x06a\x08\xED`\x80\x8C\x01``\x8D\x01aG_V[a\x08\xFD``\x8D\x01`@\x8E\x01aG_V[`\x0F\x0B\x90a.\xCAV[a\t\x0F\x90aH\x1FV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`\x0F\x91\x82\x0B`$\x84\x01R\x90\x0B`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9F\x91\x90aHEV[\x91P\x91P\x81a\t\xAD\x90aH\x1FV[\x91Pa\t\xB8\x81aH\x1FV[\x90P`\0a\n\x02a\t\xCF`@\x89\x01` \x8A\x01a?\xC1V[\x885\x88\x86\x86`\0`\x0F\x83\x90\x0B\x13a\t\xF3W\x8B` \x01Qa\t\xEE\x90aH\x1FV[a\t\xF9V[\x8B` \x01Q[`\0`\x01a/EV[\x87Q\x90\x93P\x90\x91Pa\n\x19\x90\x86\x90\x895\x86\x86a/\x91V[`\x9AT`@Qc\x88\xB6Io`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88\xB6Io\x90a\nM\x90\x8B5\x90\x85\x90`\x04\x01aH\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\njW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x8E\x91\x90aH\xAEV[`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\n\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P``\x86\x01Q`\x9B`\0a\n\xE6`@\x8B\x01` \x8C\x01a?\xC1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x86R`\x9B\x85R\x87\x86 \x88Q\x94\x85\x01\x89RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x81\x0B\x86\x88\x01R`\x01`\x80\x1B\x90\x91\x04`\x0F\x90\x81\x0B\x86\x8B\x01\x90\x81R\x92\x88R`\xA4\x90\x96R\x97\x90\x95 T\x16\x85R\x92Q\x90\x91\x0B\x90\x91R\x80Q\x91\x92\x90\x91a\x0B\xB3\x91\x0Bc;\x9A\xCA\0aH\xCBV[`\x0F\x0B` \x80\x84\x01\x91\x90\x91R\x81\x01Qa\x0B\xD3\x90`\x07\x0Bc;\x9A\xCA\0aH\xCBV[`\x0F\x0B`@\x83\x01RP\x91\x90PV[a\x0B\xEB\x83\x82a1~V[a\x0B\xF5\x83\x83a1\xD1V[a\x0B\xFF\x81\x83aIiV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x10\x90a\x0C.\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\r\x04Wc\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x9C` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fvirtual book already set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x97V[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x9C` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U[c\xFF\xFF\xFF\xFF\x85\x81\x16\x14a\r:Wc\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R`\xA4` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x87\x16\x91\x90\x91\x17\x90U[a\rHc;\x9A\xCA\0\x83aI\xB8V[c\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\r\x8Bc;\x9A\xCA\0\x84aI\xB8V[c\xFF\xFF\xFF\xFF\x96\x90\x96\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x80\x83 \x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x9C\x16\x9B\x90\x9B\x02\x9A\x90\x9A\x17\x90\x99U\x88Q\x80\x82\x01\x8AR`\x0F\x94\x90\x94\x0B\x84R\x91\x81R`\x9D\x90\x91R\x95\x90\x95 \x94Q\x85T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x90\x94UPPPPV[`\0\x80\x84\x15a\x0EfW`\0\x85\x81R`\x9E` R`@\x81 \x80T\x85\x92\x90a\x0E>\x90\x84\x90`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83\x15a\x0E\xB5W`\0\x84\x81R`\x9E` R`@\x81 \x80T\x85\x92\x90a\x0E\x8D\x90\x84\x90`\x0F\x0BaI\xFFV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPP`\0\x91\x82R`\x9E` R`@\x80\x83 T\x91\x83R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[```\0a\x0E\xE8\x83\x85aJOV[`\xA3T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x83\x16\x11\x15a\x0F\x03W\x80\x91P[\x80c\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0F\x1BW\x80\x94P[`\0a\x0F'\x86\x84aJwV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FEWa\x0FEaBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FnW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x85[\x83c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10\0W`\xA3\x81c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F\xA0Wa\x0F\xA0aJ\x9CV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0F\xC0\x89\x84aJwV[c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xD6Wa\x0F\xD6aJ\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0F\xF8\x81aJ\xB2V[\x91PPa\x0FsV[P\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x10*WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x10DWP0;\x15\x80\x15a\x10DWP`\0T`\xFF\x16`\x01\x14[a\x10\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x97V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x10\xD9W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x10\xE1a1\xF9V[a\x10\xEA\x82a2lV[a\x11^`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa2\x96V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x11\x9E\x90`\0\x90`\x04\x01aJ\xD5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xDF\x91\x90aJ\xEFV[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x12#\x90`\x01\x90`\x04\x01aJ\xD5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90aJ\xEFV[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x12\xCAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\x9FT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x13\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xAD\x91\x90\x81\x01\x90aK\x0CV[\x90P`\x01[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x15GW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xDBWa\x13\xDBaJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x12a\x14=WPPa\x155V[`\x9FTc\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\xA4` R`@\x80\x82 T\x85\x82\x01Q\x91Qc\xE0\xB0b\x1F`\xE0\x1B\x81R\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xC2W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a\x15?\x81aJ\xB2V[\x91PPa\x13\xB2V[P`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\xC3\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x17UW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xF1Wa\x15\xF1aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a\x16SWPPa\x17CV[`\xA0T`@\x82\x81\x01Q\x90Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xBCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xD0W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a\x17M\x81aJ\xB2V[\x91PPa\x15\xC8V[PPV[a\x17aa3\x0BV[a\x17k`\0a3eV[V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\0a\x17\xECa\x07\xCC` \x85\x01\x85a?\xC1V[\x90P`\0a\x18\0a\x02e` \x86\x01\x86a?\xC1V[\x90P`\0a\x181a\x18\x14` \x87\x01\x87a?\xC1V[a\x18!``\x88\x01\x88aK\xA6V[a\x04\x97\x906\x81\x90\x03\x81\x01\x90aK\xC6V[\x90P`\0a\x18B``\x87\x01\x87aK\xA6V[a\x18S\x90``\x81\x01\x90`@\x01aG_V[\x90P`\0a\x18d``\x88\x01\x88aK\xA6V[a\x18m\x90aK\xE2V[\x90Pa\x18|\x85\x85\x83\x86\x8Aa3\xB7V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x18\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x80a\x18\xE3\x87a\x18\xCD`@\x8C\x01` \x8D\x01aG_V[a\x18\xDD``\x8D\x01`@\x8E\x01aG_V[\x86a5\xF3V[\x90\x92P\x90P`\0a\x19>a\x18\xFA` \x8C\x01\x8Ca?\xC1V[\x85Q\x80Q`@\x90\x91\x01Q\x8A\x90\x87\x90\x87\x90\x82\x90a\x19\x16\x90\x8DaI\xFFV[a\x19 \x91\x90aI\xFFV[a\x19.`\x0F\x8A\x90\x0B\x8Ba8|V[a\x197\x90aH\x1FV[`\x01a/EV[\x88Q\x86QQ\x91\x94P\x91\x92Pa\x19V\x91\x8A\x91\x86\x86a/\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP\x83QQ\x86a\x19\x81` \x8D\x01\x8Da?\xC1V[\x86Q` \x80\x82\x01Q``\x80\x84\x01Q`\x80\x94\x85\x01Q`@\x80Q`\x0F\x95\x86\x0B\x81R\x8F\x86\x0B\x96\x81\x01\x96\x90\x96Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x86\x01R\x16\x90\x83\x01R`\x01\x92\x82\x01\x92\x90\x92R\x85\x82\x0B`\xA0\x82\x01R\x87\x82\x0B`\xC0\x82\x01R\x90\x86\x90\x0B`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA4``\x87\x01Q`\x9B`\0a\x1A/` \x8E\x01\x8Ea?\xC1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x93\x84\x16`\x01`\x80\x1B\x02\x93\x16\x92\x90\x92\x17\x90\x91U\x84Q\x01Qa\x1Aq\x90\x86aI\xFFV[`\0\x96\x87R`\x9E` R`@\x90\x96 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x97\x16\x96\x90\x96\x17\x90\x95UPPPPPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\0a\x1B3a\x1B%\x83\x80aL\x8FV[a\x07\xCC\x90` \x81\x01\x90a?\xC1V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x91\x92P\x90\x81\x90\x81\x90\x81\x90`\0a\x1Bh\x87``\x01Qa\x0B V[\x90P`\0a\x1Bv\x89\x80aL\x8FV[a\x1B\x84\x90` \x81\x01\x90aK\xA6V[a\x1B\x8D\x90aK\xE2V[\x90P`\0a\x1B\x9B\x8A\x80aL\x8FV[a\x1B\xA9\x90`@\x81\x01\x90aK\xA6V[a\x1B\xB2\x90aK\xE2V[\x90P`@Q\x80``\x01`@R\x80a\x1B\xD1\x8B``\x01Q\x85`\0\x01Qa \xD9V[\x81R` \x01a\x1B\xE8\x8B``\x01Q\x84`\0\x01Qa \xD9V[\x81R` \x01\x82`\0\x01Q`@\x01Q`\x0F\x0B\x81RP\x93P\x81`\0\x01Q`@\x01Q\x97Pa\x1C,\x89\x84\x84\x87`\0\x01Q\x8E` \x01` \x81\x01\x90a\x1C'\x91\x90aGBV[a3\xB7V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x1CeW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa\x1C\x84\x89\x84\x83\x87` \x01Q\x8E`@\x01` \x81\x01\x90a\x1C'\x91\x90aGBV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a\x1C\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P\x81Q`@\x90\x81\x01Q\x82Q\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x04\x83RcOCBM`\xE0\x1B` \x84\x01R`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x03a\x1D\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\x1D\x80W\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x1DzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa\x1D\xD7V[\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x1D\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[a\x1D\xEC\x89\x84\x84`\0\x01Q\x84`\0\x01Q\x88a8\xE5V[\x80\x96P\x81\x97PPPa\x1E4\x89``\x01Q\x83`\0\x01Q`\0\x01Q\x85\x89\x89\x8B\x88`\0\x01Q`@\x01Q\x8Fa\x1E\x1D\x91\x90aI\xFFV[a\x1E'\x91\x90aI\xFFV[\x87Q` \x01Q`\x01a/EV[\x84Q\x84QQ\x92\x99P\x90\x96Pa\x1EM\x91\x8B\x91\x90\x89\x89a/\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIM`\xF0\x1B` \x82\x01RP``\x80\x84\x01Q\x90\x8A\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U`\x01a\x1E\xC6\x8B\x80aL\x8FV[a\x1E\xD4\x90` \x81\x01\x90aK\xA6V[5\x14a\x1F\x19W\x81Q`@\x01Qa\x1E\xEA\x90\x89aI\xFFV[\x84Q`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01a\x1F%\x8B\x80aL\x8FV[a\x1F3\x90`@\x81\x01\x90aK\xA6V[5\x14a\x1F\x84W\x80`\0\x01Q`@\x01Q\x84`@\x01Qa\x1FQ\x91\x90aI\xFFV[` \x85\x81\x01Q`\0\x90\x81R`\x9E\x90\x91R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U[Pa\x1F\x93\x91P\x88\x90P\x80aL\x8FV[a\x1F\xA1\x90` \x81\x01\x90aK\xA6V[\x81Q``\x88\x01Q\x915\x91c\xFF\xFF\xFF\xFF\x16\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9a\x1F\xDC\x8B\x80aL\x8FV[a\x1F\xEA\x90` \x81\x01\x90aK\xA6V[a\x1F\xFB\x90`@\x81\x01\x90` \x01aG_V[\x89a \x06\x8D\x80aL\x8FV[a \x14\x90` \x81\x01\x90aK\xA6V[a %\x90`\x80\x81\x01\x90``\x01aL\xA5V[a /\x8E\x80aL\x8FV[a =\x90` \x81\x01\x90aK\xA6V[a N\x90`\xA0\x81\x01\x90`\x80\x01aL\xA5V[`@\x80Q`\x0F\x95\x86\x0B\x81R\x93\x85\x0B` \x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x84\x01R\x16``\x82\x01R`\x01`\x80\x82\x01R\x89\x82\x0B`\xA0\x82\x01R\x88\x82\x0B`\xC0\x82\x01R\x90\x87\x90\x0B`\xE0\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPV[`\0a \xB8\x83\x83a \xD9V[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UPPPV[`\0\x80`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01aN\x0B`R\x919\x90P`\0\x81\x80Q\x90` \x01 \x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a!c\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94R`\x0F\x92\x83\x0B`@\x86\x01R\x91\x0B``\x84\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa!\xA7`fT\x90V[`gTc\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x91\x82\x90 T\x82Q\x91\x82\x01\x95\x90\x95R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\"O\x81\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x96\x95PPPPPPV[`\0\x80`\0a\"h\x86\x86a \xD9V[\x90P`\0a\"v\x87\x86a \xD9V[`\0\x92\x83R`\x9E` R`@\x80\x84 T\x91\x84R\x90\x92 T`\x0F\x92\x83\x0B\x98\x92\x0B\x96P\x90\x94PPPPPV[``\x83\x81\x1C`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x82R\x80\x83 \x81Q\x94\x85\x01\x82RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x90\x0B\x92\x85\x01\x92\x90\x92R`\x01`\x80\x1B\x90\x91\x04`\xFF\x16\x90\x83\x01\x81\x90R\x90\x91\x90\x82\x03a#\"WP`@\x80Q``\x81\x01\x82R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R[\x82a#.W\x80Qa#4V[\x80` \x01Q[`\x07\x0B\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta#g\x90`\x07\x0Bc;\x9A\xCA\0aH\xCBV[\x92\x91PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xA2` R`@\x90 T`\xFF\x16a$`W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\xA2` R`@\x81 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\xA3\x80T\x91\x82\x01\x81U\x90\x91R\x7F`\x85\x91\x88\xCF\xFE)\x7FD\xDD\xE2\x9F-(ecF!\xF2b\x15\x04\x9C\xAE\xB3\x04\xCC\xBAVj\x8B\x17\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U[c\xFF\xFF\xFF\xFF\x83\x16a'\x8BW`\x9FT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xDC\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%[\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a&\x8AW`\0c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a%\x8FWa%\x8FaJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x03\x15a&xW`@Q\x80``\x01`@R\x80\x86`\x07\x0B\x81R` \x01\x85`\x07\x0B\x81R` \x01`\x01`\xFF\x16\x81RP`\xA1`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&\x07Wa&\x07aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x83Q\x81T\x93\x85\x01Q\x94\x90\x92\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x95\x90\x93\x16\x94\x90\x94\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x80a&\x82\x81aJ\xB2V[\x91PPa%`V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a'\x83W`@Q\x80``\x01`@R\x80\x86`\x07\x0B\x81R` \x01\x85`\x07\x0B\x81R` \x01`\x01`\xFF\x16\x81RP`\xA1`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\x01Wa'\x01aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x83Q\x81T\x93\x85\x01Q\x94\x90\x92\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x95\x90\x93\x16\x94\x90\x94\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U\x80a'{\x81aJ\xB2V[\x91PPa&\x8EV[PPPa(\x1DV[`@\x80Q``\x81\x01\x82R`\x07\x84\x81\x0B\x82R\x83\x90\x0B` \x80\x83\x01\x91\x82R`\x01\x83\x85\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\xA1\x83R\x85\x81 c\xFF\xFF\xFF\xFF\x8A\x16\x82R\x90\x92R\x93\x90 \x91Q\x82T\x91Q\x93Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x95\x90\x92\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U[PPPPV[```\0`\x9F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\xA2\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)!\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0\x80[\x83Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x9EW\x81c\xFF\xFF\xFF\xFF\x16\x84\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)UWa)UaJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a)\x8CW\x83\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\x81Wa)\x81aJ\x9CV[` \x02` \x01\x01Q\x91P[\x80a)\x96\x81aJ\xB2V[\x91PPa)'V[P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a*\x19W\x81c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\xD0Wa)\xD0aJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a*\x07W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\xFCWa)\xFCaJ\x9CV[` \x02` \x01\x01Q\x91P[\x80a*\x11\x81aJ\xB2V[\x91PPa)\xA2V[P`\0a*'\x82`\x01aJOV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*EWa*EaBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a*\xE5Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x9C` R`@\x90 T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x84\x91\x81\x10a*\xBBWa*\xBBaJ\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a*\xDD\x81aJ\xB2V[\x91PPa*tV[P\x94\x93PPPPV[```\0\x82Q\x84Qa+\0\x91\x90aL\xC0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x18Wa+\x18aBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+cW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a+6W\x90P[P\x90P`\0[\x84Q\x81\x10\x15a,~W`\0[\x84Q\x81\x10\x15a,kW`\xA1`\0\x87\x84\x81Q\x81\x10a+\x94Wa+\x94aJ\x9CV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86\x83\x81Q\x81\x10a+\xD0Wa+\xD0aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x93\x82\x01\x93\x90\x93R`\x01`\x80\x1B\x90\x92\x04`\xFF\x16\x90\x82\x01R\x85Q\x84\x90\x83\x90a,3\x90\x86aL\xC0V[a,=\x91\x90aL\xDFV[\x81Q\x81\x10a,MWa,MaJ\x9CV[` \x02` \x01\x01\x81\x90RP\x80\x80a,c\x90aL\xF7V[\x91PPa+uV[P\x80a,v\x81aL\xF7V[\x91PPa+iV[P\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta#g\x90`\x01`@\x1B\x90\x04`\x07\x0Bc;\x9A\xCA\0aH\xCBV[a,\xBCa3\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16a-8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x97V[a-A\x81a3eV[PV[`\0a-P\x83\x83a \xD9V[`@\x80\x84\x01Q`\0\x83\x81R`\x9E` R\x91\x90\x91 T\x91\x92P`\x0F\x91\x82\x0B\x91\x0B\x14\x80a-\x83WPa-\x83\x82``\x01Qa:\xBCV[\x15a\x12\xCAW`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.?\x91\x90aJ\xEFV[`\xA0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a.\x90WP`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R`\0` \x83\x01R`\x01\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x9FT`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a/\x0CWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a,~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[`\0\x80`\0\x80a/[\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca:\xEDV[\x91P\x91P\x84\x15a/tWa/o\x8C\x83a1~V[a/~V[a/~\x8C\x83a1\xD1V[\x90\x92P\x90P[\x98P\x98\x96PPPPPPPV[\x84`@\x01Q\x15a0\"W\x84Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\x05W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x19W=`\0\x80>=`\0\xFD[PPPPa1wV[c\xFF\xFF\xFF\xFF\x84\x16a0\x84W` \x85\x01Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01a/\xEBV[` \x85\x01Q``\x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xFCW=`\0\x80>=`\0\xFD[PPP` \x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1rW=`\0\x80>=`\0\xFD[PPPP[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xA6` R`@\x81 \x80T\x83\x92\x90a1\xA6\x90\x84\x90`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xA5` R`@\x81 \x80T\x83\x92\x90a1\xA6\x90\x84\x90`\x0F\x0BaIiV[`\0Ta\x01\0\x90\x04`\xFF\x16a2dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17ka<bV[a2ta3\x0BV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a3\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17U\x82\x82a<\xD6V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x97V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82QQ`\0\x90`\0\x19\x01a3\xCDWP`\x01a5\xEAV[\x83Q`\0\x84\x81R`\x9E` R`@\x90\x81\x90 T\x90\x82\x01\x80Q`\x0F\x92\x90\x92\x0B\x91\x82\x91\x90a3\xFA\x90\x83\x90aI\xFFV[`\x0F\x0B\x90RP``\x82\x01Q`=\x1C`\x01\x90\x81\x16\x03a5\xB2W`\0\x88`@\x01Qa4\xA3W` \x89\x01Q``\x8A\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x9D\x91\x90aM\x10V[Qa5#V[\x88Q``\x8A\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5!\x91\x90aMoV[Q[\x90P`\0\x81`\x0F\x0B\x13\x15\x15`\0\x84`@\x01Q`\x0F\x0B\x13\x15\x15\x03a5LW`\0`@\x84\x01Ra5\xB0V[`\0\x83`@\x01Q`\x0F\x0B\x13\x15a5\x80Wa5s\x83`@\x01Q\x82a5n\x90aH\x1FV[a=[V[`\x0F\x0B`@\x84\x01Ra5\xB0V[`\0\x83`@\x01Q`\x0F\x0B\x12\x15a5\xB0Wa5\xA7\x83`@\x01Q\x82a5\xA2\x90aH\x1FV[a=yV[`\x0F\x0B`@\x84\x01R[P[`\0\x82` \x01Q`\x0F\x0B\x13\x80\x15a5\xCFWP`@\x82\x01Q`\x0F\x0B\x15\x15[\x80\x15a5\xE5WPa5\xE3\x82``\x01Qa:\xBCV[\x15[\x92PPP[\x95\x94PPPPPV[`\0\x80\x80a6\x10a6\x08`\x0F\x87\x90\x0B\x88a8|V[`\x0F\x0Ba=\x8EV[\x90P`\0\x84`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a6\xDEW\x83`\0\x01Q` \x01Q`\x0F\x0B\x81`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a6sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x86`\x0F\x0B\x12\x80\x15a6\x9DWPa6\x8B\x86aH\x1FV[`\x0F\x0B\x84`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a6\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa7\x92V[\x83`\0\x01Q` \x01Q`\x0F\x0B\x81`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a7+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x86`\x0F\x0B\x13\x80\x15a7UWPa7C\x86aH\x1FV[`\x0F\x0B\x84`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a7\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[`\0\x87`@\x01Qa7\xA7W\x87` \x01Qa7\xAAV[\x87Q[``\x89\x01Q`@Qc\xC7\x16|\xF5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x0F\x89\x81\x0B`$\x83\x01R\x88\x90\x0B`D\x82\x01R\x90\x91P`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xC7\x16|\xF5\x90`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a8\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a89\x91\x90aHEV[\x91P\x91P\x81\x87`\0\x01Q`@\x01\x81\x81Qa8S\x91\x90aIiV[`\x0F\x0B\x90RPa8b\x82aH\x1FV[a8k\x82aH\x1FV[\x95P\x95PPPPP\x94P\x94\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a8\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a.\xE1Wa.\xE1aG\xD1V[`\0\x80`\0\x85`@\x01Q`\x0F\x0B\x12\x15a9\x15Wa9\x0E\x85`@\x01Q\x85`@\x01Qa5\xA2\x90aH\x1FV[\x91Pa9FV[`\0\x85`@\x01Q`\x0F\x0B\x13\x15a9;Wa9\x0E\x85`@\x01Q\x85`@\x01Qa5n\x90aH\x1FV[P`\0\x90P\x80a:\xB2V[`@\x86\x01Qa9U\x90\x83aG\xE7V[a9_\x90\x83aI\xFFV[\x91P`\0a9}\x85` \x01Q\x84`\x0F\x0Ba.\xCA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa9\x88\x81aH\x1FV[\x91P`\0a9\xAF\x89``\x01Q\x87`\0\x01Q\x8A\x87a9\xA4\x90aH\x1FV[\x86`\0\x80`\0a/EV[\x80\x93P\x81\x92PPP\x83\x87`@\x01\x81\x81Qa9\xC9\x91\x90aI\xFFV[`\x0F\x0B\x90RP`@\x86\x01\x80Q\x85\x91\x90a9\xE3\x90\x83\x90aIiV[`\x0F\x0B\x90RP\x87Q\x86Qa:\x02\x91\x8B\x91a9\xFC\x88aH\x1FV[\x86a/\x91V[\x85`\0\x01Q\x85` \x01Q\x8A``\x01Qc\xFF\xFF\xFF\xFF\x16\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9\x89` \x01Q\x89`@\x01Q\x8B``\x01Q\x8C`\x80\x01Q`\0\x89\x8Da:Y\x90aH\x1FV[`@\x80Q`\x0F\x98\x89\x0B\x81R\x96\x88\x0B` \x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16\x90\x87\x01R\x93\x90\x92\x16``\x85\x01R\x15\x15`\x80\x84\x01R\x83\x0B`\xA0\x83\x01R\x82\x0B`\xC0\x82\x01R\x90\x87\x90\x0B`\xE0\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90\xA4PP[\x95P\x95\x93PPPPV[`\0a:\xC6a=\xF8V[`\x01`\x01`\x80\x1B\x03\x16\x82g\x03\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x90P\x91\x90PV[`\0\x80`\0\x19\x89\x01a;\x04WP`\0\x90P\x84a/\x84V[`\0\x83\x15a;\xBFW\x85`\x0F\x0B`\0\x03a;MW` \x89\x01Qa;)\x90`\x0F\x0B\x86a.\xCAV[a;3\x90\x82aIiV[\x90P`\0\x87`\x0F\x0B\x12\x15a;MWa;J\x81aH\x1FV[\x90P[`\0a;[\x89`\x0F\x0Ba=\x8EV[\x90P`\0\x8A` \x01Qa;x\x8B\x8Aa;s\x91\x90aIiV[a>kV[a;\x82\x91\x90aI\xFFV[\x90Pa;\x8E\x81\x83a=[V[\x90P`\0\x81`\x0F\x0B\x13\x15a;\xB8Wa;\xAB`\x0F\x8A\x90\x0B\x82\x84a>\x86V[a;\xB5\x90\x84aIiV[\x92P[PPa;\xCCV[a;\xC9\x87\x82aIiV[\x90P[`\0a;\xD9\x8B\x8D\x87a\"\xA0V[a;\xEB\x90g\r\xE0\xB6\xB3\xA7d\0\0aI\xFFV[\x90P`\0\x80\x83`\x0F\x0B\x13a<\x0CWa<\x07`\x0F\x84\x90\x0B\x83a8|V[a<\x1AV[a<\x1A`\x0F\x84\x90\x0B\x83a.\xCAV[\x90P`\0a<(\x82\x85aI\xFFV[\x90P\x80\x8C``\x01\x81\x81Qa<<\x91\x90aIiV[`\x0F\x0B\x90RP\x80a<M\x81\x8CaI\xFFV[\x95P\x95PPPPP\x98P\x98\x96PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a<\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17k3a3eV[`\0Ta\x01\0\x90\x04`\xFF\x16a=AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `f\x91\x90\x91U`gUV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a=pW\x81a=rV[\x82[\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a=pW\x81a=rV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03a=\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0B\x12a=\xF1W\x81a#gV[P`\0\x03\x90V[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a>BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>f\x91\x90aM\xE1V[\x90P\x90V[`\0\x80\x82`\x0F\x0B\x12a>}W\x81a#gV[a#g\x82aH\x1FV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a>\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0B\x84`\x0F\x0B\x86`\x0F\x0B\x02\x81a>\xE7Wa>\xE7aG\xD1V[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a?\x12WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a*\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a?pW`\0\x80\xFD[\x825\x91P` \x83\x015a?\x82\x81a?KV[\x80\x91PP\x92P\x92\x90PV[`\0`\x80\x82\x84\x03\x12\x15a?\x9FW`\0\x80\xFD[P\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a?\xB7W`\0\x80\xFD[a=r\x83\x83a?\x8DV[`\0` \x82\x84\x03\x12\x15a?\xD3W`\0\x80\xFD[\x815a=r\x81a?KV[\x80`\x0F\x0B\x81\x14a-AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x02W`\0\x80\xFD[\x835a@\r\x81a?KV[\x92P` \x84\x015a@\x1D\x81a?\xDEV[\x91P`@\x84\x015a@-\x81a?\xDEV[\x80\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a@fW`\0\x80\xFD[\x865a@q\x81a?KV[\x95P` \x87\x015a@\x81\x81a?KV[\x94P`@\x87\x015a@\x91\x81a@8V[\x93P``\x87\x015a@\xA1\x81a?\xDEV[\x92P`\x80\x87\x015a@\xB1\x81a?\xDEV[\x91P`\xA0\x87\x015a@\xC1\x81a?\xDEV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a@\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015a@-\x81a?\xDEV[`\0\x80`@\x83\x85\x03\x12\x15aA\x10W`\0\x80\xFD[\x825aA\x1B\x81a?KV[\x91P` \x83\x015a?\x82\x81a?KV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aAlW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aAGV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aA\x8AW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aA\xA4W`\0\x80\xFD[\x825aA\xAF\x81a@8V[\x91P` \x83\x015a?\x82\x81a@8V[`\0\x80`@\x83\x85\x03\x12\x15aA\xD2W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xE9W`\0\x80\xFD[aA\xF5\x85\x82\x86\x01a?\x8DV[\x92PP` \x83\x015a?\x82\x81a@8V[`\0\x80`@\x83\x85\x03\x12\x15aB\x19W`\0\x80\xFD[\x825aB$\x81a?KV[\x91P` \x83\x015a?\x82\x81a?\xDEV[`\0` \x82\x84\x03\x12\x15aBFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB]W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a=rW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xA8WaB\xA8aBoV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xD7WaB\xD7aBoV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aB\xF7W`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15aC\x0EW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aC1WaC1aBoV[`@R\x825\x81R\x90P\x80` \x83\x015aCI\x81a?\xDEV[` \x82\x01R`@\x83\x015aC\\\x81a?\xDEV[`@\x82\x01RaCm``\x84\x01aB\xDFV[``\x82\x01RaC~`\x80\x84\x01aB\xDFV[`\x80\x82\x01RP\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15aC\x9DW`\0\x80\xFD[\x825aC\xA8\x81a?KV[\x91PaC\xB7\x84` \x85\x01aB\xFCV[\x90P\x92P\x92\x90PV[`\0\x80`\0a\x01`\x84\x86\x03\x12\x15aC\xD6W`\0\x80\xFD[\x835aC\xE1\x81a?KV[\x92PaC\xF0\x85` \x86\x01aB\xFCV[\x91PaC\xFF\x85`\xC0\x86\x01aB\xFCV[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aD\x1DW`\0\x80\xFD[\x835\x92P` \x84\x015aD/\x81a?KV[\x91P`@\x84\x015\x80\x15\x15\x81\x14a@-W`\0\x80\xFD[\x805`\x07\x81\x90\x0B\x81\x14aB\xF7W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aDlW`\0\x80\xFD[\x845aDw\x81a@8V[\x93P` \x85\x015aD\x87\x81a?KV[\x92PaD\x95`@\x86\x01aDDV[\x91PaD\xA3``\x86\x01aDDV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aD\xC4W`\0\x80\xFD[\x845aD\xCF\x81a@8V[\x93P` \x85\x015aD\xDF\x81a?KV[\x92P```?\x19\x82\x01\x12\x15aD\xF3W`\0\x80\xFD[P`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\x17WaE\x17aBoV[\x80`@RPaE(`@\x86\x01aDDV[\x81RaE6``\x86\x01aDDV[` \x82\x01R`\x80\x85\x015`\xFF\x81\x16\x81\x14aEOW`\0\x80\xFD[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aEzWaEzaBoV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aE\x95W`\0\x80\xFD[\x815` aE\xAAaE\xA5\x83aE`V[aB\xAEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aE\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xEDW\x805aE\xE0\x81a?KV[\x83R\x91\x83\x01\x91\x83\x01aE\xCDV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aF\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aF#W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aF7W`\0\x80\xFD[\x815` aFGaE\xA5\x83aE`V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aFfW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aF\x8DW\x855aF~\x81a@8V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aFkV[\x96PP\x86\x015\x92PP\x80\x82\x11\x15aF\xA3W`\0\x80\xFD[PaF\xB0\x85\x82\x86\x01aE\x84V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15aG\x10W\x81Q\x80Q`\x07\x90\x81\x0B\x86R\x87\x82\x01Q\x90\x0B\x87\x86\x01R\x85\x01Q`\xFF\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01aF\xD7V[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG0W`\0\x80\xFD[\x825\x91P` \x83\x015a?\x82\x81a?\xDEV[`\0` \x82\x84\x03\x12\x15aGTW`\0\x80\xFD[\x815a=r\x81a@8V[`\0` \x82\x84\x03\x12\x15aGqW`\0\x80\xFD[\x815a=r\x81a?\xDEV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aG\xA9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aG\x8DV[\x81\x81\x11\x15aG\xBBW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82`\x0F\x0B\x80aG\xFAWaG\xFAaG\xD1V[\x80\x83`\x0F\x0B\x07\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aH<WaH<aH\tV[`\0\x03\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aHXW`\0\x80\xFD[\x82QaHc\x81a?\xDEV[` \x84\x01Q\x90\x92Pa?\x82\x81a?\xDEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@\x81\x01`\x03\x83\x10aH\xA1WaH\xA1aHtV[\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aH\xC0W`\0\x80\xFD[\x81Qa=r\x81a?\xDEV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aH\xFBWaH\xFBaH\tV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aI'WaI'aH\tV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aICWaICaH\tV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aIYWaIYaH\tV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aI\x93WaI\x93aH\tV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aI\xAFWaI\xAFaH\tV[P\x01\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80aI\xCFWaI\xCFaG\xD1V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15aI\xF6WaI\xF6aH\tV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aJ*WaJ*aH\tV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aJEWaJEaH\tV[P\x90\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aJnWaJnaH\tV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aJ\x94WaJ\x94aH\tV[\x03\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aJ\xCBWaJ\xCBaH\tV[`\x01\x01\x93\x92PPPV[` \x81\x01`\x02\x83\x10aJ\xE9WaJ\xE9aHtV[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aK\x01W`\0\x80\xFD[\x81Qa=r\x81a@8V[`\0` \x80\x83\x85\x03\x12\x15aK\x1FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aKGW`\0\x80\xFD[\x80QaKUaE\xA5\x82aE`V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aKtW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aK\x9BW\x83QaK\x8C\x81a?KV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aKyV[\x97\x96PPPPPPPV[`\0\x825`\xBE\x19\x836\x03\x01\x81\x12aK\xBCW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aK\xD8W`\0\x80\xFD[a=r\x83\x83aB\xFCV[`\0`\xC0\x826\x03\x12\x15aK\xF4W`\0\x80\xFD[aK\xFCaB\x85V[aL\x066\x84aB\xFCV[\x81R`\xA0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aL#W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12aL6W`\0\x80\xFD[\x815` \x82\x82\x11\x15aLJWaLJaBoV[aL\\`\x1F\x83\x01`\x1F\x19\x16\x82\x01aB\xAEV[\x92P\x81\x83R6\x81\x83\x86\x01\x01\x11\x15aLrW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x91\x83\x01\x81\x01\x91\x90\x91R\x83\x01RP\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aK\xBCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xB7W`\0\x80\xFD[a=r\x82aB\xDFV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aL\xDAWaL\xDAaH\tV[P\x02\x90V[`\0\x82\x19\x82\x11\x15aL\xF2WaL\xF2aH\tV[P\x01\x90V[`\0`\x01\x82\x01aM\tWaM\taH\tV[P`\x01\x01\x90V[`\0`@\x82\x84\x03\x12\x15aM\"W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aMEWaMEaBoV[`@R\x82QaMS\x81a?\xDEV[\x81R` \x83\x01QaMc\x81a?\xDEV[` \x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15aM\x81W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aM\xA4WaM\xA4aBoV[`@R\x82QaM\xB2\x81a?\xDEV[\x81R` \x83\x01QaM\xC2\x81a?\xDEV[` \x82\x01R`@\x83\x01QaM\xD5\x81a?\xDEV[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aM\xF3W`\0\x80\xFD[\x81Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a=rW`\0\x80\xFD\xFEOrder(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce)\xA2dipfsX\"\x12 \xD0\x99\xCE\xE8\"\xE8\x8D\xC9\x03\xB6(\xC5\xBAeM\xD8s\xC9\xB6m\x7F4_\xEA|\xA7\xFF\xAEW\xDD\xA6\0dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static OFFCHAINEXCHANGE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x0BW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01*W\x80c\xCE\x93>Y\x11a\0\xBDW\x80c\xE1\xE7\x18\x8D\x11a\0\x8CW\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x064W\x80c\xFBB\x0CY\x14a\x06GW\x80c\xFF\x0B\xE9\xEF\x14a\x06ZW`\0\x80\xFD[\x80c\xE1\xE7\x18\x8D\x14a\x05\xF5W\x80c\xF2\xB2c1\x14a\x06!W`\0\x80\xFD[\x80c\xCE\x93>Y\x14a\x05\x07W\x80c\xCE\xBA\x89S\x14a\x05\x0FW\x80c\xD8\x95 *\x14a\x05\x99W\x80c\xDE\x10x\xBD\x14a\x05\xB9W`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x11a\0\xF9W\x80c\xAE\xD8\xE9g\x14a\x04\xBDW\x80c\xB5\xCB\xD7\x0E\x14a\x04\xCEW\x80c\xB6\n\xAA|\x14a\x04\xE1W\x80c\xB7mx\xE3\x14a\x04\xF4W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04eW\x80c\x93v\0>\x14a\x04vW\x80c\x95\xEE`q\x14a\x04\x89W\x80c\xA3\x9B\x9D\xCD\x14a\x04\xAAW`\0\x80\xFD[\x80cH!\xC8\xB5\x11a\x01\xA2W\x80cqP\x18\xA6\x11a\x01qW\x80cqP\x18\xA6\x14a\x03\xF6W\x80cx\xF0\xD3\xCE\x14a\x03\xFEW\x80c\x81&\t\xF1\x14a\x04\x11W\x80c\x88\xBCyh\x14a\x04RW`\0\x80\xFD[\x80cH!\xC8\xB5\x14a\x03@W\x80cH\\\xC9U\x14a\x03\x94W\x80cf\xF8{\xD1\x14a\x03\xA7W\x80cp|\x8BX\x14a\x03\xEEW`\0\x80\xFD[\x80c-\xA1\xC5\x9B\x11a\x01\xDEW\x80c-\xA1\xC5\x9B\x14a\x02\xC4W\x80c5\xEDNm\x14a\x02\xD7W\x80c?\xCE\xEA(\x14a\x02\xEAW\x80c@\xF1\xA3M\x14a\x03\nW`\0\x80\xFD[\x80c\x0F,\x87\x8E\x14a\x02\x10W\x80c\x0FKP\x9D\x14a\x02BW\x80c\x1D\x02\x9BM\x14a\x02WW\x80c\x1FL\xE0\x16\x14a\x02\xB1W[`\0\x80\xFD[a\x02#a\x02\x1E6`\x04a?]V[a\x06\x94V[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02Ua\x02P6`\x04a?\xA5V[a\x07.V[\0[a\x02ja\x02e6`\x04a?\xC1V[a\x0B V[`@Qa\x029\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x02Ua\x02\xBF6`\x04a?\xEDV[a\x0B\xE1V[a\x02Ua\x02\xD26`\x04a@MV[a\x0CZV[a\x02#a\x02\xE56`\x04a@\xCFV[a\x0E\x14V[a\x02\xFDa\x02\xF86`\x04a@\xFDV[a\x0E\xDAV[`@Qa\x029\x91\x90aA+V[a\x03-a\x03\x186`\x04aAxV[`\x9E` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x029V[a\x03\x82a\x03N6`\x04a?\xC1V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x81R`\x9D\x83R\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[`@Q\x90Q`\x0F\x0B\x81R` \x01a\x029V[a\x02Ua\x03\xA26`\x04aA\x91V[a\x10\nV[a\x03\xD6a\x03\xB56`\x04a?\xC1V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9C` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x029V[a\x02Ua\x12\xCFV[a\x02Ua\x17YV[a\x02Ua\x04\x0C6`\x04aA\xBFV[a\x17mV[a\x02Ua\x04\x1F6`\x04aB\x06V[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02Ua\x04`6`\x04aB4V[a\x1A\xA9V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD6V[a\x02Ua\x04\x846`\x04aC\x8AV[a \xACV[a\x04\x9Ca\x04\x976`\x04aC\x8AV[a \xD9V[`@Q\x90\x81R` \x01a\x029V[a\x02#a\x04\xB86`\x04aC\xC0V[a\"YV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD6V[a\x03-a\x04\xDC6`\x04aD\x08V[a\"\xA0V[a\x03-a\x04\xEF6`\x04a?\xC1V[a#@V[a\x02Ua\x05\x026`\x04aDVV[a#mV[a\x02\xFDa(#V[a\x02Ua\x05\x1D6`\x04aD\xAEV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x90\x94\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x90\x94\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x94\x90\x95\x16\x93\x90\x93\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90UV[a\x05\xACa\x05\xA76`\x04aE\xF8V[a*\xEEV[`@Qa\x029\x91\x90aF\xBAV[a\x02Ua\x05\xC76`\x04aG\x1DV[`\0\x91\x82R`\x9E` R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02Ua\x06\x036`\x04aAxV[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UV[a\x03-a\x06/6`\x04a?\xC1V[a,\x86V[a\x02Ua\x06B6`\x04aGBV[a,\xB4V[a\x02Ua\x06U6`\x04aC\x8AV[a-DV[a\x02#a\x06h6`\x04a?\xC1V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xA5` \x90\x81R`@\x80\x83 T`\xA6\x90\x92R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[``\x82\x81\x1C`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x94\x85\x01\x82RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x90\x0B\x92\x85\x01\x92\x90\x92R`\x01`\x80\x1B\x90\x91\x04`\xFF\x16\x90\x83\x01\x81\x90R\x90\x91\x82\x91\x82\x03a\x07\x17WP`@\x80Q``\x81\x01\x82R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R[` \x81\x01Q\x90Q`\x07\x91\x82\x0B\x96\x91\x0B\x94P\x92PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xB5a\x02e`@\x84\x01` \x85\x01a?\xC1V[\x90P`\0a\x07\xD1a\x07\xCC`@\x85\x01` \x86\x01a?\xC1V[a-\xA8V[\x90P`\0a\x07\xE5`\x80\x85\x01``\x86\x01aG_V[`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b$\xA8)`\xE9\x1B\x81RP\x90a\x08#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P\x80`@\x01Q\x15a\x08\x8EW\x81`@\x01Q\x83`@\x01` \x81\x01\x90a\x08F\x91\x90aG_V[a\x08P\x91\x90aG\xE7V[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb\x04\x955`\xEC\x1B` \x82\x01R\x90`\x0F\x0B\x15a\x08\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[`\0\x81`@\x01Qa\x08\xA3W\x81` \x01Qa\x08\xA6V[\x81Q[\x90P`\0\x80`\x01`\x01`\xA0\x1B\x03\x83\x16c\xC7\x16|\xF5a\x08\xCA`@\x89\x01` \x8A\x01a?\xC1V[a\x08\xDA``\x8A\x01`@\x8B\x01aG_V[a\t\x06a\x08\xED`\x80\x8C\x01``\x8D\x01aG_V[a\x08\xFD``\x8D\x01`@\x8E\x01aG_V[`\x0F\x0B\x90a.\xCAV[a\t\x0F\x90aH\x1FV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`\x0F\x91\x82\x0B`$\x84\x01R\x90\x0B`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9F\x91\x90aHEV[\x91P\x91P\x81a\t\xAD\x90aH\x1FV[\x91Pa\t\xB8\x81aH\x1FV[\x90P`\0a\n\x02a\t\xCF`@\x89\x01` \x8A\x01a?\xC1V[\x885\x88\x86\x86`\0`\x0F\x83\x90\x0B\x13a\t\xF3W\x8B` \x01Qa\t\xEE\x90aH\x1FV[a\t\xF9V[\x8B` \x01Q[`\0`\x01a/EV[\x87Q\x90\x93P\x90\x91Pa\n\x19\x90\x86\x90\x895\x86\x86a/\x91V[`\x9AT`@Qc\x88\xB6Io`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x88\xB6Io\x90a\nM\x90\x8B5\x90\x85\x90`\x04\x01aH\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\njW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x8E\x91\x90aH\xAEV[`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\n\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P``\x86\x01Q`\x9B`\0a\n\xE6`@\x8B\x01` \x8C\x01a?\xC1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UPPPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x86R`\x9B\x85R\x87\x86 \x88Q\x94\x85\x01\x89RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x81\x0B\x86\x88\x01R`\x01`\x80\x1B\x90\x91\x04`\x0F\x90\x81\x0B\x86\x8B\x01\x90\x81R\x92\x88R`\xA4\x90\x96R\x97\x90\x95 T\x16\x85R\x92Q\x90\x91\x0B\x90\x91R\x80Q\x91\x92\x90\x91a\x0B\xB3\x91\x0Bc;\x9A\xCA\0aH\xCBV[`\x0F\x0B` \x80\x84\x01\x91\x90\x91R\x81\x01Qa\x0B\xD3\x90`\x07\x0Bc;\x9A\xCA\0aH\xCBV[`\x0F\x0B`@\x83\x01RP\x91\x90PV[a\x0B\xEB\x83\x82a1~V[a\x0B\xF5\x83\x83a1\xD1V[a\x0B\xFF\x81\x83aIiV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x10\x90a\x0C.\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\r\x04Wc\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x9C` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fvirtual book already set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x97V[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x9C` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U[c\xFF\xFF\xFF\xFF\x85\x81\x16\x14a\r:Wc\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R`\xA4` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x87\x16\x91\x90\x91\x17\x90U[a\rHc;\x9A\xCA\0\x83aI\xB8V[c\xFF\xFF\xFF\xFF\x87\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\r\x8Bc;\x9A\xCA\0\x84aI\xB8V[c\xFF\xFF\xFF\xFF\x96\x90\x96\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x80\x83 \x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x9C\x16\x9B\x90\x9B\x02\x9A\x90\x9A\x17\x90\x99U\x88Q\x80\x82\x01\x8AR`\x0F\x94\x90\x94\x0B\x84R\x91\x81R`\x9D\x90\x91R\x95\x90\x95 \x94Q\x85T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x90\x94UPPPPV[`\0\x80\x84\x15a\x0EfW`\0\x85\x81R`\x9E` R`@\x81 \x80T\x85\x92\x90a\x0E>\x90\x84\x90`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83\x15a\x0E\xB5W`\0\x84\x81R`\x9E` R`@\x81 \x80T\x85\x92\x90a\x0E\x8D\x90\x84\x90`\x0F\x0BaI\xFFV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPP`\0\x91\x82R`\x9E` R`@\x80\x83 T\x91\x83R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[```\0a\x0E\xE8\x83\x85aJOV[`\xA3T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x83\x16\x11\x15a\x0F\x03W\x80\x91P[\x80c\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0F\x1BW\x80\x94P[`\0a\x0F'\x86\x84aJwV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FEWa\x0FEaBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FnW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x85[\x83c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10\0W`\xA3\x81c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F\xA0Wa\x0F\xA0aJ\x9CV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x0F\xC0\x89\x84aJwV[c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xD6Wa\x0F\xD6aJ\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0F\xF8\x81aJ\xB2V[\x91PPa\x0FsV[P\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x10*WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x10DWP0;\x15\x80\x15a\x10DWP`\0T`\xFF\x16`\x01\x14[a\x10\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x97V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x10\xD9W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x10\xE1a1\xF9V[a\x10\xEA\x82a2lV[a\x11^`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FVertex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa2\x96V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x11\x9E\x90`\0\x90`\x04\x01aJ\xD5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xDF\x91\x90aJ\xEFV[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x12#\x90`\x01\x90`\x04\x01aJ\xD5V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90aJ\xEFV[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x12\xCAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\x9FT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x13\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xAD\x91\x90\x81\x01\x90aK\x0CV[\x90P`\x01[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x15GW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xDBWa\x13\xDBaJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x12a\x14=WPPa\x155V[`\x9FTc\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\xA4` R`@\x80\x82 T\x85\x82\x01Q\x91Qc\xE0\xB0b\x1F`\xE0\x1B\x81R\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xAEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xC2W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a\x15?\x81aJ\xB2V[\x91PPa\x13\xB2V[P`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\xC3\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x17UW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xF1Wa\x15\xF1aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a\x16SWPPa\x17CV[`\xA0T`@\x82\x81\x01Q\x90Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\xBCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xD0W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a\x17M\x81aJ\xB2V[\x91PPa\x15\xC8V[PPV[a\x17aa3\x0BV[a\x17k`\0a3eV[V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\0a\x17\xECa\x07\xCC` \x85\x01\x85a?\xC1V[\x90P`\0a\x18\0a\x02e` \x86\x01\x86a?\xC1V[\x90P`\0a\x181a\x18\x14` \x87\x01\x87a?\xC1V[a\x18!``\x88\x01\x88aK\xA6V[a\x04\x97\x906\x81\x90\x03\x81\x01\x90aK\xC6V[\x90P`\0a\x18B``\x87\x01\x87aK\xA6V[a\x18S\x90``\x81\x01\x90`@\x01aG_V[\x90P`\0a\x18d``\x88\x01\x88aK\xA6V[a\x18m\x90aK\xE2V[\x90Pa\x18|\x85\x85\x83\x86\x8Aa3\xB7V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x18\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x80a\x18\xE3\x87a\x18\xCD`@\x8C\x01` \x8D\x01aG_V[a\x18\xDD``\x8D\x01`@\x8E\x01aG_V[\x86a5\xF3V[\x90\x92P\x90P`\0a\x19>a\x18\xFA` \x8C\x01\x8Ca?\xC1V[\x85Q\x80Q`@\x90\x91\x01Q\x8A\x90\x87\x90\x87\x90\x82\x90a\x19\x16\x90\x8DaI\xFFV[a\x19 \x91\x90aI\xFFV[a\x19.`\x0F\x8A\x90\x0B\x8Ba8|V[a\x197\x90aH\x1FV[`\x01a/EV[\x88Q\x86QQ\x91\x94P\x91\x92Pa\x19V\x91\x8A\x91\x86\x86a/\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP\x83QQ\x86a\x19\x81` \x8D\x01\x8Da?\xC1V[\x86Q` \x80\x82\x01Q``\x80\x84\x01Q`\x80\x94\x85\x01Q`@\x80Q`\x0F\x95\x86\x0B\x81R\x8F\x86\x0B\x96\x81\x01\x96\x90\x96Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x86\x01R\x16\x90\x83\x01R`\x01\x92\x82\x01\x92\x90\x92R\x85\x82\x0B`\xA0\x82\x01R\x87\x82\x0B`\xC0\x82\x01R\x90\x86\x90\x0B`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA4``\x87\x01Q`\x9B`\0a\x1A/` \x8E\x01\x8Ea?\xC1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x93\x84\x16`\x01`\x80\x1B\x02\x93\x16\x92\x90\x92\x17\x90\x91U\x84Q\x01Qa\x1Aq\x90\x86aI\xFFV[`\0\x96\x87R`\x9E` R`@\x90\x96 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x97\x16\x96\x90\x96\x17\x90\x95UPPPPPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\0a\x1B3a\x1B%\x83\x80aL\x8FV[a\x07\xCC\x90` \x81\x01\x90a?\xC1V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x91\x92P\x90\x81\x90\x81\x90\x81\x90`\0a\x1Bh\x87``\x01Qa\x0B V[\x90P`\0a\x1Bv\x89\x80aL\x8FV[a\x1B\x84\x90` \x81\x01\x90aK\xA6V[a\x1B\x8D\x90aK\xE2V[\x90P`\0a\x1B\x9B\x8A\x80aL\x8FV[a\x1B\xA9\x90`@\x81\x01\x90aK\xA6V[a\x1B\xB2\x90aK\xE2V[\x90P`@Q\x80``\x01`@R\x80a\x1B\xD1\x8B``\x01Q\x85`\0\x01Qa \xD9V[\x81R` \x01a\x1B\xE8\x8B``\x01Q\x84`\0\x01Qa \xD9V[\x81R` \x01\x82`\0\x01Q`@\x01Q`\x0F\x0B\x81RP\x93P\x81`\0\x01Q`@\x01Q\x97Pa\x1C,\x89\x84\x84\x87`\0\x01Q\x8E` \x01` \x81\x01\x90a\x1C'\x91\x90aGBV[a3\xB7V[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x1CeW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa\x1C\x84\x89\x84\x83\x87` \x01Q\x8E`@\x01` \x81\x01\x90a\x1C'\x91\x90aGBV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a\x1C\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P\x81Q`@\x90\x81\x01Q\x82Q\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x04\x83RcOCBM`\xE0\x1B` \x84\x01R`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x03a\x1D\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\x1D\x80W\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x1DzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa\x1D\xD7V[\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x1D\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[a\x1D\xEC\x89\x84\x84`\0\x01Q\x84`\0\x01Q\x88a8\xE5V[\x80\x96P\x81\x97PPPa\x1E4\x89``\x01Q\x83`\0\x01Q`\0\x01Q\x85\x89\x89\x8B\x88`\0\x01Q`@\x01Q\x8Fa\x1E\x1D\x91\x90aI\xFFV[a\x1E'\x91\x90aI\xFFV[\x87Q` \x01Q`\x01a/EV[\x84Q\x84QQ\x92\x99P\x90\x96Pa\x1EM\x91\x8B\x91\x90\x89\x89a/\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIM`\xF0\x1B` \x82\x01RP``\x80\x84\x01Q\x90\x8A\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U`\x01a\x1E\xC6\x8B\x80aL\x8FV[a\x1E\xD4\x90` \x81\x01\x90aK\xA6V[5\x14a\x1F\x19W\x81Q`@\x01Qa\x1E\xEA\x90\x89aI\xFFV[\x84Q`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01a\x1F%\x8B\x80aL\x8FV[a\x1F3\x90`@\x81\x01\x90aK\xA6V[5\x14a\x1F\x84W\x80`\0\x01Q`@\x01Q\x84`@\x01Qa\x1FQ\x91\x90aI\xFFV[` \x85\x81\x01Q`\0\x90\x81R`\x9E\x90\x91R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U[Pa\x1F\x93\x91P\x88\x90P\x80aL\x8FV[a\x1F\xA1\x90` \x81\x01\x90aK\xA6V[\x81Q``\x88\x01Q\x915\x91c\xFF\xFF\xFF\xFF\x16\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9a\x1F\xDC\x8B\x80aL\x8FV[a\x1F\xEA\x90` \x81\x01\x90aK\xA6V[a\x1F\xFB\x90`@\x81\x01\x90` \x01aG_V[\x89a \x06\x8D\x80aL\x8FV[a \x14\x90` \x81\x01\x90aK\xA6V[a %\x90`\x80\x81\x01\x90``\x01aL\xA5V[a /\x8E\x80aL\x8FV[a =\x90` \x81\x01\x90aK\xA6V[a N\x90`\xA0\x81\x01\x90`\x80\x01aL\xA5V[`@\x80Q`\x0F\x95\x86\x0B\x81R\x93\x85\x0B` \x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x84\x01R\x16``\x82\x01R`\x01`\x80\x82\x01R\x89\x82\x0B`\xA0\x82\x01R\x88\x82\x0B`\xC0\x82\x01R\x90\x87\x90\x0B`\xE0\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPV[`\0a \xB8\x83\x83a \xD9V[`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UPPPV[`\0\x80`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01aN\x0B`R\x919\x90P`\0\x81\x80Q\x90` \x01 \x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a!c\x96\x95\x94\x93\x92\x91\x90\x95\x86R` \x86\x01\x94\x90\x94R`\x0F\x92\x83\x0B`@\x86\x01R\x91\x0B``\x84\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa!\xA7`fT\x90V[`gTc\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x91\x82\x90 T\x82Q\x91\x82\x01\x95\x90\x95R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\"O\x81\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x96\x95PPPPPPV[`\0\x80`\0a\"h\x86\x86a \xD9V[\x90P`\0a\"v\x87\x86a \xD9V[`\0\x92\x83R`\x9E` R`@\x80\x84 T\x91\x84R\x90\x92 T`\x0F\x92\x83\x0B\x98\x92\x0B\x96P\x90\x94PPPPPV[``\x83\x81\x1C`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x82R\x80\x83 \x81Q\x94\x85\x01\x82RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x90\x0B\x92\x85\x01\x92\x90\x92R`\x01`\x80\x1B\x90\x91\x04`\xFF\x16\x90\x83\x01\x81\x90R\x90\x91\x90\x82\x03a#\"WP`@\x80Q``\x81\x01\x82R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R[\x82a#.W\x80Qa#4V[\x80` \x01Q[`\x07\x0B\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta#g\x90`\x07\x0Bc;\x9A\xCA\0aH\xCBV[\x92\x91PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x07\x97V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xA2` R`@\x90 T`\xFF\x16a$`W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81R`\xA2` R`@\x81 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\xA3\x80T\x91\x82\x01\x81U\x90\x91R\x7F`\x85\x91\x88\xCF\xFE)\x7FD\xDD\xE2\x9F-(ecF!\xF2b\x15\x04\x9C\xAE\xB3\x04\xCC\xBAVj\x8B\x17\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U[c\xFF\xFF\xFF\xFF\x83\x16a'\x8BW`\x9FT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xDC\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%[\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a&\x8AW`\0c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a%\x8FWa%\x8FaJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x03\x15a&xW`@Q\x80``\x01`@R\x80\x86`\x07\x0B\x81R` \x01\x85`\x07\x0B\x81R` \x01`\x01`\xFF\x16\x81RP`\xA1`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&\x07Wa&\x07aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x83Q\x81T\x93\x85\x01Q\x94\x90\x92\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x95\x90\x93\x16\x94\x90\x94\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x80a&\x82\x81aJ\xB2V[\x91PPa%`V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a'\x83W`@Q\x80``\x01`@R\x80\x86`\x07\x0B\x81R` \x01\x85`\x07\x0B\x81R` \x01`\x01`\xFF\x16\x81RP`\xA1`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\x01Wa'\x01aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x83Q\x81T\x93\x85\x01Q\x94\x90\x92\x01Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x95\x90\x93\x16\x94\x90\x94\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U\x80a'{\x81aJ\xB2V[\x91PPa&\x8EV[PPPa(\x1DV[`@\x80Q``\x81\x01\x82R`\x07\x84\x81\x0B\x82R\x83\x90\x0B` \x80\x83\x01\x91\x82R`\x01\x83\x85\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\xA1\x83R\x85\x81 c\xFF\xFF\xFF\xFF\x8A\x16\x82R\x90\x92R\x93\x90 \x91Q\x82T\x91Q\x93Q`\xFF\x16`\x01`\x80\x1B\x02`\xFF`\x80\x1B\x19g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x95\x90\x92\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U[PPPPV[```\0`\x9F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\xA2\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0`\xA0`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)!\x91\x90\x81\x01\x90aK\x0CV[\x90P`\0\x80[\x83Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x9EW\x81c\xFF\xFF\xFF\xFF\x16\x84\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)UWa)UaJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a)\x8CW\x83\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\x81Wa)\x81aJ\x9CV[` \x02` \x01\x01Q\x91P[\x80a)\x96\x81aJ\xB2V[\x91PPa)'V[P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a*\x19W\x81c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\xD0Wa)\xD0aJ\x9CV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a*\x07W\x82\x81c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a)\xFCWa)\xFCaJ\x9CV[` \x02` \x01\x01Q\x91P[\x80a*\x11\x81aJ\xB2V[\x91PPa)\xA2V[P`\0a*'\x82`\x01aJOV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*EWa*EaBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a*\xE5Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x9C` R`@\x90 T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x84\x91\x81\x10a*\xBBWa*\xBBaJ\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a*\xDD\x81aJ\xB2V[\x91PPa*tV[P\x94\x93PPPPV[```\0\x82Q\x84Qa+\0\x91\x90aL\xC0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x18Wa+\x18aBoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+cW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a+6W\x90P[P\x90P`\0[\x84Q\x81\x10\x15a,~W`\0[\x84Q\x81\x10\x15a,kW`\xA1`\0\x87\x84\x81Q\x81\x10a+\x94Wa+\x94aJ\x9CV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86\x83\x81Q\x81\x10a+\xD0Wa+\xD0aJ\x9CV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x93\x82\x01\x93\x90\x93R`\x01`\x80\x1B\x90\x92\x04`\xFF\x16\x90\x82\x01R\x85Q\x84\x90\x83\x90a,3\x90\x86aL\xC0V[a,=\x91\x90aL\xDFV[\x81Q\x81\x10a,MWa,MaJ\x9CV[` \x02` \x01\x01\x81\x90RP\x80\x80a,c\x90aL\xF7V[\x91PPa+uV[P\x80a,v\x81aL\xF7V[\x91PPa+iV[P\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta#g\x90`\x01`@\x1B\x90\x04`\x07\x0Bc;\x9A\xCA\0aH\xCBV[a,\xBCa3\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16a-8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x97V[a-A\x81a3eV[PV[`\0a-P\x83\x83a \xD9V[`@\x80\x84\x01Q`\0\x83\x81R`\x9E` R\x91\x90\x91 T\x91\x92P`\x0F\x91\x82\x0B\x91\x0B\x14\x80a-\x83WPa-\x83\x82``\x01Qa:\xBCV[\x15a\x12\xCAW`\0\x90\x81R`\x9E` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90UPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.?\x91\x90aJ\xEFV[`\xA0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a.\x90WP`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R`\0` \x83\x01R`\x01\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x9FT`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a/\x0CWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a,~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[`\0\x80`\0\x80a/[\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca:\xEDV[\x91P\x91P\x84\x15a/tWa/o\x8C\x83a1~V[a/~V[a/~\x8C\x83a1\xD1V[\x90\x92P\x90P[\x98P\x98\x96PPPPPPPV[\x84`@\x01Q\x15a0\"W\x84Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\x05W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x19W=`\0\x80>=`\0\xFD[PPPPa1wV[c\xFF\xFF\xFF\xFF\x84\x16a0\x84W` \x85\x01Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01a/\xEBV[` \x85\x01Q``\x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xFCW=`\0\x80>=`\0\xFD[PPP` \x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1rW=`\0\x80>=`\0\xFD[PPPP[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xA6` R`@\x81 \x80T\x83\x92\x90a1\xA6\x90\x84\x90`\x0F\x0BaIiV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xA5` R`@\x81 \x80T\x83\x92\x90a1\xA6\x90\x84\x90`\x0F\x0BaIiV[`\0Ta\x01\0\x90\x04`\xFF\x16a2dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17ka<bV[a2ta3\x0BV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a3\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17U\x82\x82a<\xD6V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x97V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82QQ`\0\x90`\0\x19\x01a3\xCDWP`\x01a5\xEAV[\x83Q`\0\x84\x81R`\x9E` R`@\x90\x81\x90 T\x90\x82\x01\x80Q`\x0F\x92\x90\x92\x0B\x91\x82\x91\x90a3\xFA\x90\x83\x90aI\xFFV[`\x0F\x0B\x90RP``\x82\x01Q`=\x1C`\x01\x90\x81\x16\x03a5\xB2W`\0\x88`@\x01Qa4\xA3W` \x89\x01Q``\x8A\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\x9D\x91\x90aM\x10V[Qa5#V[\x88Q``\x8A\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5!\x91\x90aMoV[Q[\x90P`\0\x81`\x0F\x0B\x13\x15\x15`\0\x84`@\x01Q`\x0F\x0B\x13\x15\x15\x03a5LW`\0`@\x84\x01Ra5\xB0V[`\0\x83`@\x01Q`\x0F\x0B\x13\x15a5\x80Wa5s\x83`@\x01Q\x82a5n\x90aH\x1FV[a=[V[`\x0F\x0B`@\x84\x01Ra5\xB0V[`\0\x83`@\x01Q`\x0F\x0B\x12\x15a5\xB0Wa5\xA7\x83`@\x01Q\x82a5\xA2\x90aH\x1FV[a=yV[`\x0F\x0B`@\x84\x01R[P[`\0\x82` \x01Q`\x0F\x0B\x13\x80\x15a5\xCFWP`@\x82\x01Q`\x0F\x0B\x15\x15[\x80\x15a5\xE5WPa5\xE3\x82``\x01Qa:\xBCV[\x15[\x92PPP[\x95\x94PPPPPV[`\0\x80\x80a6\x10a6\x08`\x0F\x87\x90\x0B\x88a8|V[`\x0F\x0Ba=\x8EV[\x90P`\0\x84`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a6\xDEW\x83`\0\x01Q` \x01Q`\x0F\x0B\x81`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a6sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x86`\x0F\x0B\x12\x80\x15a6\x9DWPa6\x8B\x86aH\x1FV[`\x0F\x0B\x84`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a6\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[Pa7\x92V[\x83`\0\x01Q` \x01Q`\x0F\x0B\x81`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a7+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x86`\x0F\x0B\x13\x80\x15a7UWPa7C\x86aH\x1FV[`\x0F\x0B\x84`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a7\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P[`\0\x87`@\x01Qa7\xA7W\x87` \x01Qa7\xAAV[\x87Q[``\x89\x01Q`@Qc\xC7\x16|\xF5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x0F\x89\x81\x0B`$\x83\x01R\x88\x90\x0B`D\x82\x01R\x90\x91P`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xC7\x16|\xF5\x90`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a8\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a89\x91\x90aHEV[\x91P\x91P\x81\x87`\0\x01Q`@\x01\x81\x81Qa8S\x91\x90aIiV[`\x0F\x0B\x90RPa8b\x82aH\x1FV[a8k\x82aH\x1FV[\x95P\x95PPPPP\x94P\x94\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a8\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a.\xE1Wa.\xE1aG\xD1V[`\0\x80`\0\x85`@\x01Q`\x0F\x0B\x12\x15a9\x15Wa9\x0E\x85`@\x01Q\x85`@\x01Qa5\xA2\x90aH\x1FV[\x91Pa9FV[`\0\x85`@\x01Q`\x0F\x0B\x13\x15a9;Wa9\x0E\x85`@\x01Q\x85`@\x01Qa5n\x90aH\x1FV[P`\0\x90P\x80a:\xB2V[`@\x86\x01Qa9U\x90\x83aG\xE7V[a9_\x90\x83aI\xFFV[\x91P`\0a9}\x85` \x01Q\x84`\x0F\x0Ba.\xCA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa9\x88\x81aH\x1FV[\x91P`\0a9\xAF\x89``\x01Q\x87`\0\x01Q\x8A\x87a9\xA4\x90aH\x1FV[\x86`\0\x80`\0a/EV[\x80\x93P\x81\x92PPP\x83\x87`@\x01\x81\x81Qa9\xC9\x91\x90aI\xFFV[`\x0F\x0B\x90RP`@\x86\x01\x80Q\x85\x91\x90a9\xE3\x90\x83\x90aIiV[`\x0F\x0B\x90RP\x87Q\x86Qa:\x02\x91\x8B\x91a9\xFC\x88aH\x1FV[\x86a/\x91V[\x85`\0\x01Q\x85` \x01Q\x8A``\x01Qc\xFF\xFF\xFF\xFF\x16\x7F|WE\x9DoO\x0F\xB2\xFC[\x1E)\x8C\x8C\x0E\xB28B)D\x96J\xA1\xE2I\xEA\xA7\x87G\xF0\xCC\xA9\x89` \x01Q\x89`@\x01Q\x8B``\x01Q\x8C`\x80\x01Q`\0\x89\x8Da:Y\x90aH\x1FV[`@\x80Q`\x0F\x98\x89\x0B\x81R\x96\x88\x0B` \x88\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16\x90\x87\x01R\x93\x90\x92\x16``\x85\x01R\x15\x15`\x80\x84\x01R\x83\x0B`\xA0\x83\x01R\x82\x0B`\xC0\x82\x01R\x90\x87\x90\x0B`\xE0\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90\xA4PP[\x95P\x95\x93PPPPV[`\0a:\xC6a=\xF8V[`\x01`\x01`\x80\x1B\x03\x16\x82g\x03\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x90P\x91\x90PV[`\0\x80`\0\x19\x89\x01a;\x04WP`\0\x90P\x84a/\x84V[`\0\x83\x15a;\xBFW\x85`\x0F\x0B`\0\x03a;MW` \x89\x01Qa;)\x90`\x0F\x0B\x86a.\xCAV[a;3\x90\x82aIiV[\x90P`\0\x87`\x0F\x0B\x12\x15a;MWa;J\x81aH\x1FV[\x90P[`\0a;[\x89`\x0F\x0Ba=\x8EV[\x90P`\0\x8A` \x01Qa;x\x8B\x8Aa;s\x91\x90aIiV[a>kV[a;\x82\x91\x90aI\xFFV[\x90Pa;\x8E\x81\x83a=[V[\x90P`\0\x81`\x0F\x0B\x13\x15a;\xB8Wa;\xAB`\x0F\x8A\x90\x0B\x82\x84a>\x86V[a;\xB5\x90\x84aIiV[\x92P[PPa;\xCCV[a;\xC9\x87\x82aIiV[\x90P[`\0a;\xD9\x8B\x8D\x87a\"\xA0V[a;\xEB\x90g\r\xE0\xB6\xB3\xA7d\0\0aI\xFFV[\x90P`\0\x80\x83`\x0F\x0B\x13a<\x0CWa<\x07`\x0F\x84\x90\x0B\x83a8|V[a<\x1AV[a<\x1A`\x0F\x84\x90\x0B\x83a.\xCAV[\x90P`\0a<(\x82\x85aI\xFFV[\x90P\x80\x8C``\x01\x81\x81Qa<<\x91\x90aIiV[`\x0F\x0B\x90RP\x80a<M\x81\x8CaI\xFFV[\x95P\x95PPPPP\x98P\x98\x96PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a<\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[a\x17k3a3eV[`\0Ta\x01\0\x90\x04`\xFF\x16a=AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x07\x97V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `f\x91\x90\x91U`gUV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a=pW\x81a=rV[\x82[\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a=pW\x81a=rV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03a=\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0B\x12a=\xF1W\x81a#gV[P`\0\x03\x90V[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a>BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>f\x91\x90aM\xE1V[\x90P\x90V[`\0\x80\x82`\x0F\x0B\x12a>}W\x81a#gV[a#g\x82aH\x1FV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a>\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[P`\0\x82`\x0F\x0B\x84`\x0F\x0B\x86`\x0F\x0B\x02\x81a>\xE7Wa>\xE7aG\xD1V[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a?\x12WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a*\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x97\x91\x90aG|V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a?pW`\0\x80\xFD[\x825\x91P` \x83\x015a?\x82\x81a?KV[\x80\x91PP\x92P\x92\x90PV[`\0`\x80\x82\x84\x03\x12\x15a?\x9FW`\0\x80\xFD[P\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a?\xB7W`\0\x80\xFD[a=r\x83\x83a?\x8DV[`\0` \x82\x84\x03\x12\x15a?\xD3W`\0\x80\xFD[\x815a=r\x81a?KV[\x80`\x0F\x0B\x81\x14a-AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x02W`\0\x80\xFD[\x835a@\r\x81a?KV[\x92P` \x84\x015a@\x1D\x81a?\xDEV[\x91P`@\x84\x015a@-\x81a?\xDEV[\x80\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-AW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a@fW`\0\x80\xFD[\x865a@q\x81a?KV[\x95P` \x87\x015a@\x81\x81a?KV[\x94P`@\x87\x015a@\x91\x81a@8V[\x93P``\x87\x015a@\xA1\x81a?\xDEV[\x92P`\x80\x87\x015a@\xB1\x81a?\xDEV[\x91P`\xA0\x87\x015a@\xC1\x81a?\xDEV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a@\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015a@-\x81a?\xDEV[`\0\x80`@\x83\x85\x03\x12\x15aA\x10W`\0\x80\xFD[\x825aA\x1B\x81a?KV[\x91P` \x83\x015a?\x82\x81a?KV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aAlW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aAGV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aA\x8AW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aA\xA4W`\0\x80\xFD[\x825aA\xAF\x81a@8V[\x91P` \x83\x015a?\x82\x81a@8V[`\0\x80`@\x83\x85\x03\x12\x15aA\xD2W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xE9W`\0\x80\xFD[aA\xF5\x85\x82\x86\x01a?\x8DV[\x92PP` \x83\x015a?\x82\x81a@8V[`\0\x80`@\x83\x85\x03\x12\x15aB\x19W`\0\x80\xFD[\x825aB$\x81a?KV[\x91P` \x83\x015a?\x82\x81a?\xDEV[`\0` \x82\x84\x03\x12\x15aBFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB]W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15a=rW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xA8WaB\xA8aBoV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xD7WaB\xD7aBoV[`@R\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aB\xF7W`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15aC\x0EW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aC1WaC1aBoV[`@R\x825\x81R\x90P\x80` \x83\x015aCI\x81a?\xDEV[` \x82\x01R`@\x83\x015aC\\\x81a?\xDEV[`@\x82\x01RaCm``\x84\x01aB\xDFV[``\x82\x01RaC~`\x80\x84\x01aB\xDFV[`\x80\x82\x01RP\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15aC\x9DW`\0\x80\xFD[\x825aC\xA8\x81a?KV[\x91PaC\xB7\x84` \x85\x01aB\xFCV[\x90P\x92P\x92\x90PV[`\0\x80`\0a\x01`\x84\x86\x03\x12\x15aC\xD6W`\0\x80\xFD[\x835aC\xE1\x81a?KV[\x92PaC\xF0\x85` \x86\x01aB\xFCV[\x91PaC\xFF\x85`\xC0\x86\x01aB\xFCV[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aD\x1DW`\0\x80\xFD[\x835\x92P` \x84\x015aD/\x81a?KV[\x91P`@\x84\x015\x80\x15\x15\x81\x14a@-W`\0\x80\xFD[\x805`\x07\x81\x90\x0B\x81\x14aB\xF7W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aDlW`\0\x80\xFD[\x845aDw\x81a@8V[\x93P` \x85\x015aD\x87\x81a?KV[\x92PaD\x95`@\x86\x01aDDV[\x91PaD\xA3``\x86\x01aDDV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15aD\xC4W`\0\x80\xFD[\x845aD\xCF\x81a@8V[\x93P` \x85\x015aD\xDF\x81a?KV[\x92P```?\x19\x82\x01\x12\x15aD\xF3W`\0\x80\xFD[P`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\x17WaE\x17aBoV[\x80`@RPaE(`@\x86\x01aDDV[\x81RaE6``\x86\x01aDDV[` \x82\x01R`\x80\x85\x015`\xFF\x81\x16\x81\x14aEOW`\0\x80\xFD[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aEzWaEzaBoV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aE\x95W`\0\x80\xFD[\x815` aE\xAAaE\xA5\x83aE`V[aB\xAEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aE\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xEDW\x805aE\xE0\x81a?KV[\x83R\x91\x83\x01\x91\x83\x01aE\xCDV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aF\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aF#W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aF7W`\0\x80\xFD[\x815` aFGaE\xA5\x83aE`V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aFfW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aF\x8DW\x855aF~\x81a@8V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aFkV[\x96PP\x86\x015\x92PP\x80\x82\x11\x15aF\xA3W`\0\x80\xFD[PaF\xB0\x85\x82\x86\x01aE\x84V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15aG\x10W\x81Q\x80Q`\x07\x90\x81\x0B\x86R\x87\x82\x01Q\x90\x0B\x87\x86\x01R\x85\x01Q`\xFF\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01aF\xD7V[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG0W`\0\x80\xFD[\x825\x91P` \x83\x015a?\x82\x81a?\xDEV[`\0` \x82\x84\x03\x12\x15aGTW`\0\x80\xFD[\x815a=r\x81a@8V[`\0` \x82\x84\x03\x12\x15aGqW`\0\x80\xFD[\x815a=r\x81a?\xDEV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aG\xA9W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aG\x8DV[\x81\x81\x11\x15aG\xBBW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82`\x0F\x0B\x80aG\xFAWaG\xFAaG\xD1V[\x80\x83`\x0F\x0B\x07\x91PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aH<WaH<aH\tV[`\0\x03\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aHXW`\0\x80\xFD[\x82QaHc\x81a?\xDEV[` \x84\x01Q\x90\x92Pa?\x82\x81a?\xDEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@\x81\x01`\x03\x83\x10aH\xA1WaH\xA1aHtV[\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aH\xC0W`\0\x80\xFD[\x81Qa=r\x81a?\xDEV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aH\xFBWaH\xFBaH\tV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aI'WaI'aH\tV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aICWaICaH\tV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aIYWaIYaH\tV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aI\x93WaI\x93aH\tV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aI\xAFWaI\xAFaH\tV[P\x01\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80aI\xCFWaI\xCFaG\xD1V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15aI\xF6WaI\xF6aH\tV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aJ*WaJ*aH\tV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aJEWaJEaH\tV[P\x90\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aJnWaJnaH\tV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aJ\x94WaJ\x94aH\tV[\x03\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aJ\xCBWaJ\xCBaH\tV[`\x01\x01\x93\x92PPPV[` \x81\x01`\x02\x83\x10aJ\xE9WaJ\xE9aHtV[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aK\x01W`\0\x80\xFD[\x81Qa=r\x81a@8V[`\0` \x80\x83\x85\x03\x12\x15aK\x1FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aKGW`\0\x80\xFD[\x80QaKUaE\xA5\x82aE`V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aKtW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aK\x9BW\x83QaK\x8C\x81a?KV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aKyV[\x97\x96PPPPPPPV[`\0\x825`\xBE\x19\x836\x03\x01\x81\x12aK\xBCW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aK\xD8W`\0\x80\xFD[a=r\x83\x83aB\xFCV[`\0`\xC0\x826\x03\x12\x15aK\xF4W`\0\x80\xFD[aK\xFCaB\x85V[aL\x066\x84aB\xFCV[\x81R`\xA0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aL#W`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12aL6W`\0\x80\xFD[\x815` \x82\x82\x11\x15aLJWaLJaBoV[aL\\`\x1F\x83\x01`\x1F\x19\x16\x82\x01aB\xAEV[\x92P\x81\x83R6\x81\x83\x86\x01\x01\x11\x15aLrW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x91\x83\x01\x81\x01\x91\x90\x91R\x83\x01RP\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aK\xBCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xB7W`\0\x80\xFD[a=r\x82aB\xDFV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aL\xDAWaL\xDAaH\tV[P\x02\x90V[`\0\x82\x19\x82\x11\x15aL\xF2WaL\xF2aH\tV[P\x01\x90V[`\0`\x01\x82\x01aM\tWaM\taH\tV[P`\x01\x01\x90V[`\0`@\x82\x84\x03\x12\x15aM\"W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aMEWaMEaBoV[`@R\x82QaMS\x81a?\xDEV[\x81R` \x83\x01QaMc\x81a?\xDEV[` \x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15aM\x81W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aM\xA4WaM\xA4aBoV[`@R\x82QaM\xB2\x81a?\xDEV[\x81R` \x83\x01QaM\xC2\x81a?\xDEV[` \x82\x01R`@\x83\x01QaM\xD5\x81a?\xDEV[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aM\xF3W`\0\x80\xFD[\x81Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a=rW`\0\x80\xFD\xFEOrder(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce)\xA2dipfsX\"\x12 \xD0\x99\xCE\xE8\"\xE8\x8D\xC9\x03\xB6(\xC5\xBAeM\xD8s\xC9\xB6m\x7F4_\xEA|\xA7\xFF\xAEW\xDD\xA6\0dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static OFFCHAINEXCHANGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OffchainExchange<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OffchainExchange<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OffchainExchange<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OffchainExchange<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OffchainExchange<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OffchainExchange))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OffchainExchange<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                OFFCHAINEXCHANGE_ABI.clone(),
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
                OFFCHAINEXCHANGE_ABI.clone(),
                OFFCHAINEXCHANGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dropDigest` (0xe1e7188d) function
        pub fn drop_digest(
            &self,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 231, 24, 141], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropOrder` (0x9376003e) function
        pub fn drop_order(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 118, 0, 62], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropOrderChecked` (0xfb420c59) function
        pub fn drop_order_checked(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 66, 12, 89], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpFees` (0x707c8b58) function
        pub fn dump_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `filledAmounts` (0x40f1a34d) function
        pub fn filled_amounts(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([64, 241, 163, 77], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllFeeRates` (0xd895202a) function
        pub fn get_all_fee_rates(
            &self,
            users: ::std::vec::Vec<::ethers::core::types::Address>,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FeeRates>> {
            self.0
                .method_hash([216, 149, 32, 42], (users, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllVirtualBooks` (0xce933e59) function
        pub fn get_all_virtual_books(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([206, 147, 62, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollectedFees` (0xff0be9ef) function
        pub fn get_collected_fees(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([255, 11, 233, 239], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCustomFeeAddresses` (0x3fceea28) function
        pub fn get_custom_fee_addresses(
            &self,
            start_at: u32,
            limit: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 206, 234, 40], (start_at, limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigest` (0x95ee6071) function
        pub fn get_digest(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([149, 238, 96, 113], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEndpoint` (0xaed8e967) function
        pub fn get_endpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([174, 216, 233, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeFractionX18` (0xb5cbd70e) function
        pub fn get_fee_fraction_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            taker: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([181, 203, 215, 14], (subaccount, product_id, taker))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeRatesX18` (0x0f2c878e) function
        pub fn get_fee_rates_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([15, 44, 135, 142], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLpParams` (0x4821c8b5) function
        pub fn get_lp_params(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, LpParams> {
            self.0
                .method_hash([72, 33, 200, 181], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarketInfo` (0x1d029b4d) function
        pub fn get_market_info(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, MarketInfo> {
            self.0
                .method_hash([29, 2, 155, 77], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinSize` (0xb60aaa7c) function
        pub fn get_min_size(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([182, 10, 170, 124], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderFilledAmounts` (0xa39b9dcd) function
        pub fn get_order_filled_amounts(
            &self,
            product_id: u32,
            order_1: Order,
            order_2: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([163, 155, 157, 205], (product_id, order_1, order_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSizeIncrement` (0xf2b26331) function
        pub fn get_size_increment(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([242, 178, 99, 49], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVirtualBook` (0x66f87bd1) function
        pub fn get_virtual_book(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([102, 248, 123, 209], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementFees` (0x1f4ce016) function
        pub fn increment_fees(
            &self,
            product_id: u32,
            maker_fee: i128,
            taker_fee: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 76, 224, 22], (product_id, maker_fee, taker_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (clearinghouse, endpoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrderAMM` (0x78f0d3ce) function
        pub fn match_order_amm(
            &self,
            txn: MatchOrderAMM,
            taker_linked_signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 240, 211, 206], (txn, taker_linked_signer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrders` (0x88bc7968) function
        pub fn match_orders(
            &self,
            txn: MatchOrdersWithSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 188, 121, 104], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyFilledAmount` (0x35ed4e6d) function
        pub fn modify_filled_amount(
            &self,
            taker_digest: [u8; 32],
            maker_digest: [u8; 32],
            taker_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash(
                    [53, 237, 78, 109],
                    (taker_digest, maker_digest, taker_delta),
                )
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRate` (0xceba8953) function
        pub fn set_fee_rate(
            &self,
            user: ::ethers::core::types::Address,
            product_id: u32,
            fee_rate: FeeRates,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 186, 137, 83], (user, product_id, fee_rate))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFilledAmount` (0xde1078bd) function
        pub fn set_filled_amount(
            &self,
            digest: [u8; 32],
            filled_amount: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 16, 120, 189], (digest, filled_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapAMM` (0x0f4b509d) function
        pub fn swap_amm(&self, txn: SwapAMM) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 75, 80, 157], (txn,))
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
        ///Calls the contract's `updateCollectedFees` (0x812609f1) function
        pub fn update_collected_fees(
            &self,
            product_id: u32,
            collected_fees: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 38, 9, 241], (product_id, collected_fees))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeRates` (0xb76d78e3) function
        pub fn update_fee_rates(
            &self,
            user: ::ethers::core::types::Address,
            product_id: u32,
            maker_rate_x18: i64,
            taker_rate_x18: i64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 109, 120, 227],
                    (user, product_id, maker_rate_x18, taker_rate_x18),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMarket` (0x2da1c59b) function
        pub fn update_market(
            &self,
            product_id: u32,
            quote_id: u32,
            virtual_book: ::ethers::core::types::Address,
            size_increment: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 161, 197, 155],
                    (
                        product_id,
                        quote_id,
                        virtual_book,
                        size_increment,
                        min_size,
                        lp_spread_x18,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FillOrder` event
        pub fn fill_order_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FillOrderFilter> {
            self.0.event()
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OffchainExchangeEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for OffchainExchange<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OffchainExchangeEvents {
        FillOrderFilter(FillOrderFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for OffchainExchangeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillOrderFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::FillOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OffchainExchangeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FillOrderFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FillOrderFilter> for OffchainExchangeEvents {
        fn from(value: FillOrderFilter) -> Self {
            Self::FillOrderFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for OffchainExchangeEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for OffchainExchangeEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `dropDigest` function with signature `dropDigest(bytes32)` and selector `0xe1e7188d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dropDigest", abi = "dropDigest(bytes32)")]
    pub struct DropDigestCall {
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `dropOrder` function with signature `dropOrder(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `0x9376003e`
    #[derive(
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
        name = "dropOrder",
        abi = "dropOrder(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCall {
        pub product_id: u32,
        pub order: Order,
    }
    ///Container type for all input parameters for the `dropOrderChecked` function with signature `dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `0xfb420c59`
    #[derive(
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
        name = "dropOrderChecked",
        abi = "dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCheckedCall {
        pub product_id: u32,
        pub order: Order,
    }
    ///Container type for all input parameters for the `dumpFees` function with signature `dumpFees()` and selector `0x707c8b58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    ///Container type for all input parameters for the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "filledAmounts", abi = "filledAmounts(bytes32)")]
    pub struct FilledAmountsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `getAllFeeRates` function with signature `getAllFeeRates(address[],uint32[])` and selector `0xd895202a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAllFeeRates", abi = "getAllFeeRates(address[],uint32[])")]
    pub struct GetAllFeeRatesCall {
        pub users: ::std::vec::Vec<::ethers::core::types::Address>,
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getAllVirtualBooks` function with signature `getAllVirtualBooks()` and selector `0xce933e59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAllVirtualBooks", abi = "getAllVirtualBooks()")]
    pub struct GetAllVirtualBooksCall;
    ///Container type for all input parameters for the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `0xff0be9ef`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCollectedFees", abi = "getCollectedFees(uint32)")]
    pub struct GetCollectedFeesCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `0x3fceea28`
    #[derive(
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
        name = "getCustomFeeAddresses",
        abi = "getCustomFeeAddresses(uint32,uint32)"
    )]
    pub struct GetCustomFeeAddressesCall {
        pub start_at: u32,
        pub limit: u32,
    }
    ///Container type for all input parameters for the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `0x95ee6071`
    #[derive(
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
        name = "getDigest",
        abi = "getDigest(uint32,(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetDigestCall {
        pub product_id: u32,
        pub order: Order,
    }
    ///Container type for all input parameters for the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getEndpoint", abi = "getEndpoint()")]
    pub struct GetEndpointCall;
    ///Container type for all input parameters for the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `0xb5cbd70e`
    #[derive(
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
        name = "getFeeFractionX18",
        abi = "getFeeFractionX18(bytes32,uint32,bool)"
    )]
    pub struct GetFeeFractionX18Call {
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub taker: bool,
    }
    ///Container type for all input parameters for the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `0x0f2c878e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getFeeRatesX18", abi = "getFeeRatesX18(bytes32,uint32)")]
    pub struct GetFeeRatesX18Call {
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getLpParams` function with signature `getLpParams(uint32)` and selector `0x4821c8b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLpParams", abi = "getLpParams(uint32)")]
    pub struct GetLpParamsCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `0x1d029b4d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getMarketInfo", abi = "getMarketInfo(uint32)")]
    pub struct GetMarketInfoCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getMinSize` function with signature `getMinSize(uint32)` and selector `0xb60aaa7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getMinSize", abi = "getMinSize(uint32)")]
    pub struct GetMinSizeCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `0xa39b9dcd`
    #[derive(
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
        name = "getOrderFilledAmounts",
        abi = "getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetOrderFilledAmountsCall {
        pub product_id: u32,
        pub order_1: Order,
        pub order_2: Order,
    }
    ///Container type for all input parameters for the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `0xf2b26331`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSizeIncrement", abi = "getSizeIncrement(uint32)")]
    pub struct GetSizeIncrementCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getVirtualBook` function with signature `getVirtualBook(uint32)` and selector `0x66f87bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getVirtualBook", abi = "getVirtualBook(uint32)")]
    pub struct GetVirtualBookCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `incrementFees` function with signature `incrementFees(uint32,int128,int128)` and selector `0x1f4ce016`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "incrementFees", abi = "incrementFees(uint32,int128,int128)")]
    pub struct IncrementFeesCall {
        pub product_id: u32,
        pub maker_fee: i128,
        pub taker_fee: i128,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)` and selector `0x78f0d3ce`
    #[derive(
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
        abi = "matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)"
    )]
    pub struct MatchOrderAMMCall {
        pub txn: MatchOrderAMM,
        pub taker_linked_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))` and selector `0x88bc7968`
    #[derive(
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
        abi = "matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))"
    )]
    pub struct MatchOrdersCall {
        pub txn: MatchOrdersWithSigner,
    }
    ///Container type for all input parameters for the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `0x35ed4e6d`
    #[derive(
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
        name = "modifyFilledAmount",
        abi = "modifyFilledAmount(bytes32,bytes32,int128)"
    )]
    pub struct ModifyFilledAmountCall {
        pub taker_digest: [u8; 32],
        pub maker_digest: [u8; 32],
        pub taker_delta: i128,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
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
    ///Container type for all input parameters for the `setFeeRate` function with signature `setFeeRate(address,uint32,(int64,int64,uint8))` and selector `0xceba8953`
    #[derive(
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
        name = "setFeeRate",
        abi = "setFeeRate(address,uint32,(int64,int64,uint8))"
    )]
    pub struct SetFeeRateCall {
        pub user: ::ethers::core::types::Address,
        pub product_id: u32,
        pub fee_rate: FeeRates,
    }
    ///Container type for all input parameters for the `setFilledAmount` function with signature `setFilledAmount(bytes32,int128)` and selector `0xde1078bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFilledAmount", abi = "setFilledAmount(bytes32,int128)")]
    pub struct SetFilledAmountCall {
        pub digest: [u8; 32],
        pub filled_amount: i128,
    }
    ///Container type for all input parameters for the `swapAMM` function with signature `swapAMM((bytes32,uint32,int128,int128))` and selector `0x0f4b509d`
    #[derive(
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
        pub txn: SwapAMM,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
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
    ///Container type for all input parameters for the `updateCollectedFees` function with signature `updateCollectedFees(uint32,int128)` and selector `0x812609f1`
    #[derive(
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
        name = "updateCollectedFees",
        abi = "updateCollectedFees(uint32,int128)"
    )]
    pub struct UpdateCollectedFeesCall {
        pub product_id: u32,
        pub collected_fees: i128,
    }
    ///Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates(address,uint32,int64,int64)` and selector `0xb76d78e3`
    #[derive(
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
        abi = "updateFeeRates(address,uint32,int64,int64)"
    )]
    pub struct UpdateFeeRatesCall {
        pub user: ::ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    ///Container type for all input parameters for the `updateMarket` function with signature `updateMarket(uint32,uint32,address,int128,int128,int128)` and selector `0x2da1c59b`
    #[derive(
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
        name = "updateMarket",
        abi = "updateMarket(uint32,uint32,address,int128,int128,int128)"
    )]
    pub struct UpdateMarketCall {
        pub product_id: u32,
        pub quote_id: u32,
        pub virtual_book: ::ethers::core::types::Address,
        pub size_increment: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
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
        GetVirtualBook(GetVirtualBookCall),
        IncrementFees(IncrementFeesCall),
        Initialize(InitializeCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        ModifyFilledAmount(ModifyFilledAmountCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetFeeRate(SetFeeRateCall),
        SetFilledAmount(SetFilledAmountCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateCollectedFees(UpdateCollectedFeesCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpdateMarket(UpdateMarketCall),
    }
    impl ::ethers::core::abi::AbiDecode for OffchainExchangeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DropDigestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropDigest(decoded));
            }
            if let Ok(decoded) = <DropOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropOrder(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCheckedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DropOrderChecked(decoded));
            }
            if let Ok(decoded) = <DumpFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DumpFees(decoded));
            }
            if let Ok(decoded) = <FilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetAllFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllFeeRates(decoded));
            }
            if let Ok(decoded) =
                <GetAllVirtualBooksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllVirtualBooks(decoded));
            }
            if let Ok(decoded) =
                <GetCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <GetCustomFeeAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCustomFeeAddresses(decoded));
            }
            if let Ok(decoded) = <GetDigestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDigest(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFractionX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetFeeRatesX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeeRatesX18(decoded));
            }
            if let Ok(decoded) = <GetLpParamsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLpParams(decoded));
            }
            if let Ok(decoded) = <GetMarketInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMarketInfo(decoded));
            }
            if let Ok(decoded) = <GetMinSizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinSize(decoded));
            }
            if let Ok(decoded) =
                <GetOrderFilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOrderFilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetSizeIncrementCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSizeIncrement(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualBookCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetVirtualBook(decoded));
            }
            if let Ok(decoded) = <IncrementFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementFees(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MatchOrderAMMCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <ModifyFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModifyFilledAmount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetFeeRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFilledAmount(decoded));
            }
            if let Ok(decoded) = <SwapAMMCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapAMM(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <UpdateFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) = <UpdateMarketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMarket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OffchainExchangeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DropDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DropOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DropOrderChecked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DumpFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FilledAmounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllFeeRates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllVirtualBooks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollectedFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCustomFeeAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeFractionX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeRatesX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLpParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMarketInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOrderFilledAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSizeIncrement(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVirtualBook(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrderAMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ModifyFilledAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFilledAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapAMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateCollectedFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeRates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OffchainExchangeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DropDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrderChecked(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FilledAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllVirtualBooks(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollectedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCustomFeeAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeFractionX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeRatesX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLpParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarketInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderFilledAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSizeIncrement(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVirtualBook(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrderAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyFilledAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFilledAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateCollectedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMarket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DropDigestCall> for OffchainExchangeCalls {
        fn from(value: DropDigestCall) -> Self {
            Self::DropDigest(value)
        }
    }
    impl ::core::convert::From<DropOrderCall> for OffchainExchangeCalls {
        fn from(value: DropOrderCall) -> Self {
            Self::DropOrder(value)
        }
    }
    impl ::core::convert::From<DropOrderCheckedCall> for OffchainExchangeCalls {
        fn from(value: DropOrderCheckedCall) -> Self {
            Self::DropOrderChecked(value)
        }
    }
    impl ::core::convert::From<DumpFeesCall> for OffchainExchangeCalls {
        fn from(value: DumpFeesCall) -> Self {
            Self::DumpFees(value)
        }
    }
    impl ::core::convert::From<FilledAmountsCall> for OffchainExchangeCalls {
        fn from(value: FilledAmountsCall) -> Self {
            Self::FilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetAllFeeRatesCall> for OffchainExchangeCalls {
        fn from(value: GetAllFeeRatesCall) -> Self {
            Self::GetAllFeeRates(value)
        }
    }
    impl ::core::convert::From<GetAllVirtualBooksCall> for OffchainExchangeCalls {
        fn from(value: GetAllVirtualBooksCall) -> Self {
            Self::GetAllVirtualBooks(value)
        }
    }
    impl ::core::convert::From<GetCollectedFeesCall> for OffchainExchangeCalls {
        fn from(value: GetCollectedFeesCall) -> Self {
            Self::GetCollectedFees(value)
        }
    }
    impl ::core::convert::From<GetCustomFeeAddressesCall> for OffchainExchangeCalls {
        fn from(value: GetCustomFeeAddressesCall) -> Self {
            Self::GetCustomFeeAddresses(value)
        }
    }
    impl ::core::convert::From<GetDigestCall> for OffchainExchangeCalls {
        fn from(value: GetDigestCall) -> Self {
            Self::GetDigest(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for OffchainExchangeCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetFeeFractionX18Call> for OffchainExchangeCalls {
        fn from(value: GetFeeFractionX18Call) -> Self {
            Self::GetFeeFractionX18(value)
        }
    }
    impl ::core::convert::From<GetFeeRatesX18Call> for OffchainExchangeCalls {
        fn from(value: GetFeeRatesX18Call) -> Self {
            Self::GetFeeRatesX18(value)
        }
    }
    impl ::core::convert::From<GetLpParamsCall> for OffchainExchangeCalls {
        fn from(value: GetLpParamsCall) -> Self {
            Self::GetLpParams(value)
        }
    }
    impl ::core::convert::From<GetMarketInfoCall> for OffchainExchangeCalls {
        fn from(value: GetMarketInfoCall) -> Self {
            Self::GetMarketInfo(value)
        }
    }
    impl ::core::convert::From<GetMinSizeCall> for OffchainExchangeCalls {
        fn from(value: GetMinSizeCall) -> Self {
            Self::GetMinSize(value)
        }
    }
    impl ::core::convert::From<GetOrderFilledAmountsCall> for OffchainExchangeCalls {
        fn from(value: GetOrderFilledAmountsCall) -> Self {
            Self::GetOrderFilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetSizeIncrementCall> for OffchainExchangeCalls {
        fn from(value: GetSizeIncrementCall) -> Self {
            Self::GetSizeIncrement(value)
        }
    }
    impl ::core::convert::From<GetVirtualBookCall> for OffchainExchangeCalls {
        fn from(value: GetVirtualBookCall) -> Self {
            Self::GetVirtualBook(value)
        }
    }
    impl ::core::convert::From<IncrementFeesCall> for OffchainExchangeCalls {
        fn from(value: IncrementFeesCall) -> Self {
            Self::IncrementFees(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OffchainExchangeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MatchOrderAMMCall> for OffchainExchangeCalls {
        fn from(value: MatchOrderAMMCall) -> Self {
            Self::MatchOrderAMM(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for OffchainExchangeCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<ModifyFilledAmountCall> for OffchainExchangeCalls {
        fn from(value: ModifyFilledAmountCall) -> Self {
            Self::ModifyFilledAmount(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for OffchainExchangeCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for OffchainExchangeCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetFeeRateCall> for OffchainExchangeCalls {
        fn from(value: SetFeeRateCall) -> Self {
            Self::SetFeeRate(value)
        }
    }
    impl ::core::convert::From<SetFilledAmountCall> for OffchainExchangeCalls {
        fn from(value: SetFilledAmountCall) -> Self {
            Self::SetFilledAmount(value)
        }
    }
    impl ::core::convert::From<SwapAMMCall> for OffchainExchangeCalls {
        fn from(value: SwapAMMCall) -> Self {
            Self::SwapAMM(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for OffchainExchangeCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateCollectedFeesCall> for OffchainExchangeCalls {
        fn from(value: UpdateCollectedFeesCall) -> Self {
            Self::UpdateCollectedFees(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRatesCall> for OffchainExchangeCalls {
        fn from(value: UpdateFeeRatesCall) -> Self {
            Self::UpdateFeeRates(value)
        }
    }
    impl ::core::convert::From<UpdateMarketCall> for OffchainExchangeCalls {
        fn from(value: UpdateMarketCall) -> Self {
            Self::UpdateMarket(value)
        }
    }
    ///Container type for all return fields from the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FilledAmountsReturn(pub i128);
    ///Container type for all return fields from the `getAllFeeRates` function with signature `getAllFeeRates(address[],uint32[])` and selector `0xd895202a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAllFeeRatesReturn(pub ::std::vec::Vec<FeeRates>);
    ///Container type for all return fields from the `getAllVirtualBooks` function with signature `getAllVirtualBooks()` and selector `0xce933e59`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAllVirtualBooksReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `0xff0be9ef`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCollectedFeesReturn {
        pub collected_maker_fees: i128,
        pub collected_taker_fees: i128,
    }
    ///Container type for all return fields from the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `0x3fceea28`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCustomFeeAddressesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64))` and selector `0x95ee6071`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDigestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetEndpointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `0xb5cbd70e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetFeeFractionX18Return(pub i128);
    ///Container type for all return fields from the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `0x0f2c878e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetFeeRatesX18Return(pub i128, pub i128);
    ///Container type for all return fields from the `getLpParams` function with signature `getLpParams(uint32)` and selector `0x4821c8b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLpParamsReturn(pub LpParams);
    ///Container type for all return fields from the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `0x1d029b4d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMarketInfoReturn {
        pub m: MarketInfo,
    }
    ///Container type for all return fields from the `getMinSize` function with signature `getMinSize(uint32)` and selector `0xb60aaa7c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMinSizeReturn(pub i128);
    ///Container type for all return fields from the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `0xa39b9dcd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetOrderFilledAmountsReturn(pub i128, pub i128);
    ///Container type for all return fields from the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `0xf2b26331`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSizeIncrementReturn(pub i128);
    ///Container type for all return fields from the `getVirtualBook` function with signature `getVirtualBook(uint32)` and selector `0x66f87bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetVirtualBookReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `0x35ed4e6d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ModifyFilledAmountReturn(pub i128, pub i128);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
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
    ///`MatchOrderAMM(uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
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
        pub base_delta: i128,
        pub quote_delta: i128,
        pub taker: SignedOrder,
    }
    ///`MatchOrders(uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
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
    ///`MatchOrdersWithSigner((uint32,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MatchOrdersWithSigner {
        pub match_orders: MatchOrders,
        pub taker_linked_signer: ::ethers::core::types::Address,
        pub maker_linked_signer: ::ethers::core::types::Address,
    }
    ///`Order(bytes32,int128,int128,uint64,uint64)`
    #[derive(
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
        pub sender: [u8; 32],
        pub price_x18: i128,
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
    }
    ///`SignedOrder((bytes32,int128,int128,uint64,uint64),bytes)`
    #[derive(
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
    ///`SwapAMM(bytes32,uint32,int128,int128)`
    #[derive(
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
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: i128,
        pub price_x18: i128,
    }
    ///`FeeRates(int64,int64,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeRates {
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
        pub is_non_default: u8,
    }
    ///`LpParams(int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpParams {
        pub lp_spread_x18: i128,
    }
    ///`MarketInfo(uint32,int128,int128,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MarketInfo {
        pub quote_id: u32,
        pub min_size: i128,
        pub size_increment: i128,
        pub collected_fees: i128,
    }
}
