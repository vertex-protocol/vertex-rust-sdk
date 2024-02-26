pub use perp_engine::*;
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
pub mod perp_engine {

    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
    use super::super::super::serialize_utils::{
        deserialize_bytes32, deserialize_i128, deserialize_i256, deserialize_nested_vec_i128,
        deserialize_u128, deserialize_u256, deserialize_vec_i128, serialize_bytes32,
        serialize_i128, serialize_i256, serialize_nested_vec_i128, serialize_u128, serialize_u256,
        serialize_vec_i128,
    };
    use serde::{Deserialize, Serialize};

    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addProduct"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("healthGroup"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("book"),
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
                                name: ::std::borrow::ToOwned::to_owned("priceIncrementX18"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IClearinghouseState.RiskStore",
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
                    ::std::borrow::ToOwned::to_owned("applyDeltas"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("applyDeltas"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("deltas"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IProductEngine.ProductDelta[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balances"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("vQuoteBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastCumulativeFundingX18",),
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
                    ::std::borrow::ToOwned::to_owned("burnLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("burnLp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountLp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBase"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountQuote"),
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
                    ::std::borrow::ToOwned::to_owned("decomposeLps"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decomposeLps"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidatee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeCalculator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("liquidationFees"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAvailableSettle"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAvailableSettle"),
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
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalanceAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalanceAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalances"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPerpEngine.LpBalance",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getClearinghouse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getClearinghouse"),
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
                    ::std::borrow::ToOwned::to_owned("getEngineType"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEngineType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum IProductEngine.EngineType",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLpState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLpState"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpState",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOraclePriceX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOraclePriceX18"),
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
                    ::std::borrow::ToOwned::to_owned("getOraclePricesX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOraclePricesX18"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("healthGroup"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.Prices"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOrderbook"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOrderbook"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolState"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getPositionPnl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPositionPnl"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getProductIds"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawLpBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawLpBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpBalance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawLpState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawLpState"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpState",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawState"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSettlementState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSettlementState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("availableSettle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpState"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpState",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPerpEngine.LpBalance",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("state"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStateAndBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStateAndBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStatesAndBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStatesAndBalances",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpState",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPerpEngine.LpBalance",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("hasBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                name: ::std::borrow::ToOwned::to_owned("_quote"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_admin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fees"),
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
                    ::std::borrow::ToOwned::to_owned("lpBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpBalances"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastCumulativeFundingX18",),
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
                    ::std::borrow::ToOwned::to_owned("lpStates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpStates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("supply"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastCumulativeFundingX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cumulativeFundingPerLpX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("base"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
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
                    ::std::borrow::ToOwned::to_owned("manualAssert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("manualAssert"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("openInterests"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("markets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("markets"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IOffchainBook"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mintLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mintLp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountBase"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quoteAmountLow"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quoteAmountHigh"),
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
                    ::std::borrow::ToOwned::to_owned("perpPositionClosed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpPositionClosed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("setBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setEndpoint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_endpoint"),
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
                    ::std::borrow::ToOwned::to_owned("setLpBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setLpBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPerpEngine.LpBalance",
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
                    ::std::borrow::ToOwned::to_owned("setLpState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setLpState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpState"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.LpState",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("state"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
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
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("socializeSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("socializeSubaccount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("insurance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("states"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("states"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cumulativeFundingLongX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cumulativeFundingShortX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("availableSettle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("openInterest"),
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
                    ::std::borrow::ToOwned::to_owned("swapLp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quoteDelta"),
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
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
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
                                    name: ::std::borrow::ToOwned::to_owned("lpSpreadX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseSwapped"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quoteSwapped"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
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
                    ::std::borrow::ToOwned::to_owned("unsignedUpdateProductTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedUpdateProductTx",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPerpEngine.UpdateProductTx",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPerpEngine.UpdateProductTx",
                                ),
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
                            name: ::std::borrow::ToOwned::to_owned("tx"),
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
                    ::std::borrow::ToOwned::to_owned("updateStates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateStates"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avgPriceDiffs"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BalanceUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                (
                    ::std::borrow::ToOwned::to_owned("ProductUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProductUpdate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PRBMathSD59x18__SqrtNegativeInput"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMathSD59x18__SqrtNegativeInput",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMathSD59x18__SqrtOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PRBMathSD59x18__SqrtOverflow",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PERPENGINE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct PerpEngine<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PerpEngine<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PerpEngine<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PerpEngine<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PerpEngine<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PerpEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PerpEngine<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PERPENGINE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addProduct` (0x367bc290) function
        pub fn add_product(
            &self,
            health_group: u32,
            book: ::ethers::core::types::Address,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [54, 123, 194, 144],
                    (
                        health_group,
                        book,
                        size_increment,
                        price_increment_x18,
                        min_size,
                        lp_spread_x18,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyDeltas` (0x6c9a2ce3) function
        pub fn apply_deltas(
            &self,
            deltas: ::std::vec::Vec<ProductDelta>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 154, 44, 227], deltas)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0xbf4c8f5f) function
        pub fn balances(
            &self,
            p0: u32,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128, i128)> {
            self.0
                .method_hash([191, 76, 143, 95], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLp` (0xd98752ec) function
        pub fn burn_lp(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_lp: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([217, 135, 82, 236], (product_id, subaccount, amount_lp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decomposeLps` (0x011a447a) function
        pub fn decompose_lps(
            &self,
            liquidatee: [u8; 32],
            liquidator: [u8; 32],
            fee_calculator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([1, 26, 68, 122], (liquidatee, liquidator, fee_calculator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAvailableSettle` (0x3056f78f) function
        pub fn get_available_settle(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([48, 86, 247, 143], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalance` (0x7c1e1487) function
        pub fn get_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([124, 30, 20, 135], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalanceAmount` (0xe039bfa9) function
        pub fn get_balance_amount(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([224, 57, 191, 169], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalances` (0x6aca31a3) function
        pub fn get_balances(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (LpBalance, Balance)> {
            self.0
                .method_hash([106, 202, 49, 163], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getClearinghouse` (0xb1cb0f42) function
        pub fn get_clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 203, 15, 66], ())
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
        ///Calls the contract's `getEngineType` (0x4604d19b) function
        pub fn get_engine_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([70, 4, 209, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLpState` (0x3f857d5a) function
        pub fn get_lp_state(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, LpState> {
            self.0
                .method_hash([63, 133, 125, 90], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOraclePriceX18` (0x2f8f1fb0) function
        pub fn get_oracle_price_x18(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([47, 143, 31, 176], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOraclePricesX18` (0xd3d660cb) function
        pub fn get_oracle_prices_x18(
            &self,
            health_group: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Prices> {
            self.0
                .method_hash([211, 214, 96, 203], health_group)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderbook` (0x4427952d) function
        pub fn get_orderbook(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([68, 39, 149, 45], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolState` (0x8af6426a) function
        pub fn get_pool_state(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([138, 246, 66, 106], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositionPnl` (0x1769225f) function
        pub fn get_position_pnl(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([23, 105, 34, 95], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProductIds` (0x47428e7b) function
        pub fn get_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([71, 66, 142, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawBalance` (0xedf02653) function
        pub fn get_raw_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([237, 240, 38, 83], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawLpBalance` (0x6f27be34) function
        pub fn get_raw_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, LpBalance> {
            self.0
                .method_hash([111, 39, 190, 52], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawLpState` (0xc721bd65) function
        pub fn get_raw_lp_state(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, LpState> {
            self.0
                .method_hash([199, 33, 189, 101], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawState` (0xec7a79c9) function
        pub fn get_raw_state(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, State> {
            self.0
                .method_hash([236, 122, 121, 201], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSettlementState` (0x388927b8) function
        pub fn get_settlement_state(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, LpState, LpBalance, State, Balance)>
        {
            self.0
                .method_hash([56, 137, 39, 184], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStateAndBalance` (0xe334be33) function
        pub fn get_state_and_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (State, Balance)> {
            self.0
                .method_hash([227, 52, 190, 51], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStatesAndBalances` (0x3d5cc9dc) function
        pub fn get_states_and_balances(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (LpState, LpBalance, State, Balance)>
        {
            self.0
                .method_hash([61, 92, 201, 220], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasBalance` (0xde7ebc91) function
        pub fn has_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([222, 126, 188, 145], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
            fees: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, quote, endpoint, admin, fees),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpBalances` (0x08ed83c1) function
        pub fn lp_balances(
            &self,
            p0: u32,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([8, 237, 131, 193], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpStates` (0x042508e6) function
        pub fn lp_states(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128, i128, i128, i128)> {
            self.0
                .method_hash([4, 37, 8, 230], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0x9b6f762b) function
        pub fn manual_assert(
            &self,
            open_interests: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 111, 118, 43], open_interests)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markets` (0xece91e35) function
        pub fn markets(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 233, 30, 53], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintLp` (0x98de72fe) function
        pub fn mint_lp(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_base: i128,
            quote_amount_low: i128,
            quote_amount_high: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 222, 114, 254],
                    (
                        product_id,
                        subaccount,
                        amount_base,
                        quote_amount_low,
                        quote_amount_high,
                    ),
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
        ///Calls the contract's `perpPositionClosed` (0x64c42cc2) function
        pub fn perp_position_closed(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 196, 44, 194], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBalance` (0x385de9c3) function
        pub fn set_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            balance: Balance,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 93, 233, 195], (product_id, subaccount, balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEndpoint` (0xdbbb4155) function
        pub fn set_endpoint(
            &self,
            endpoint: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 187, 65, 85], endpoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLpBalance` (0x3025586a) function
        pub fn set_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            lp_balance: LpBalance,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 37, 88, 106], (product_id, subaccount, lp_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLpState` (0x26fa00ac) function
        pub fn set_lp_state(
            &self,
            product_id: u32,
            lp_state: LpState,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 250, 0, 172], (product_id, lp_state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setState` (0x7fa29d49) function
        pub fn set_state(
            &self,
            product_id: u32,
            state: State,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 162, 157, 73], (product_id, state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settlePnl` (0xd6b0e0b5) function
        pub fn settle_pnl(
            &self,
            subaccount: [u8; 32],
            product_ids: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([214, 176, 224, 181], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `socializeSubaccount` (0xb1cd4b8f) function
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([177, 205, 75, 143], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `states` (0x7f17baad) function
        pub fn states(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128, i128, i128)> {
            self.0
                .method_hash([127, 23, 186, 173], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapLp` (0xc7167cf5) function
        pub fn swap_lp(
            &self,
            product_id: u32,
            base_delta: i128,
            quote_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([199, 22, 124, 245], (product_id, base_delta, quote_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapLp` (0xd952f2a3) function
        pub fn swap_lp_with_product_id_and_amount_and_price_x_18(
            &self,
            product_id: u32,
            amount: i128,
            price_x18: i128,
            size_increment: i128,
            lp_spread_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash(
                    [217, 82, 242, 163],
                    (product_id, amount, price_x18, size_increment, lp_spread_x18),
                )
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
        ///Calls the contract's `unsignedUpdateProductTx` (0x3ec8c7da) function
        pub fn unsigned_update_product_tx(
            &self,
            p: UpdateProductTx,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateProductTx> {
            self.0
                .method_hash([62, 200, 199, 218], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateProduct` (0xc9fe9ac3) function
        pub fn update_product(
            &self,
            tx: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 254, 154, 195], tx)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateStates` (0x6736f5da) function
        pub fn update_states(
            &self,
            dt: u128,
            avg_price_diffs: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 54, 245, 218], (dt, avg_price_diffs))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddProduct` event
        pub fn add_product_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddProductFilter> {
            self.0.event()
        }
        ///Gets the contract's `BalanceUpdate` event
        pub fn balance_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BalanceUpdateFilter>
        {
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
        ///Gets the contract's `ProductUpdate` event
        pub fn product_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProductUpdateFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PerpEngineEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PerpEngine<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `PRBMathSD59x18__SqrtNegativeInput` with signature `PRBMathSD59x18__SqrtNegativeInput(int256)` and selector `0xc11907fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PRBMathSD59x18__SqrtNegativeInput",
        abi = "PRBMathSD59x18__SqrtNegativeInput(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtNegativeInput {
        pub x: ::ethers::core::types::I256,
    }
    ///Custom Error type `PRBMathSD59x18__SqrtOverflow` with signature `PRBMathSD59x18__SqrtOverflow(int256)` and selector `0x2c482c39`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PRBMathSD59x18__SqrtOverflow",
        abi = "PRBMathSD59x18__SqrtOverflow(int256)"
    )]
    pub struct PRBMathSD59x18__SqrtOverflow {
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PerpEngineErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PerpEngineErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtNegativeInput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMathSD59x18__SqrtNegativeInput(decoded));
            }
            if let Ok(decoded) =
                <PRBMathSD59x18__SqrtOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PerpEngineErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::PRBMathSD59x18__SqrtNegativeInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PRBMathSD59x18__SqrtOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PerpEngineErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <PRBMathSD59x18__SqrtNegativeInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PRBMathSD59x18__SqrtOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PerpEngineErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PRBMathSD59x18__SqrtNegativeInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PRBMathSD59x18__SqrtOverflow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PerpEngineErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtNegativeInput> for PerpEngineErrors {
        fn from(value: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            Self::PRBMathSD59x18__SqrtNegativeInput(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtOverflow> for PerpEngineErrors {
        fn from(value: PRBMathSD59x18__SqrtOverflow) -> Self {
            Self::PRBMathSD59x18__SqrtOverflow(value)
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
    #[ethevent(name = "AddProduct", abi = "AddProduct(uint32)")]
    pub struct AddProductFilter {
        pub product_id: u32,
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
    #[ethevent(name = "BalanceUpdate", abi = "BalanceUpdate(uint32,bytes32)")]
    pub struct BalanceUpdateFilter {
        pub product_id: u32,
        pub subaccount: [u8; 32],
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
    #[ethevent(name = "ProductUpdate", abi = "ProductUpdate(uint32)")]
    pub struct ProductUpdateFilter {
        pub product_id: u32,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PerpEngineEvents {
        AddProductFilter(AddProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProductUpdateFilter(ProductUpdateFilter),
    }
    impl ::ethers::contract::EthLogDecode for PerpEngineEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddProductFilter::decode_log(log) {
                return Ok(PerpEngineEvents::AddProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PerpEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PerpEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::ProductUpdateFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PerpEngineEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProductFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductFilter> for PerpEngineEvents {
        fn from(value: AddProductFilter) -> Self {
            Self::AddProductFilter(value)
        }
    }
    impl ::core::convert::From<BalanceUpdateFilter> for PerpEngineEvents {
        fn from(value: BalanceUpdateFilter) -> Self {
            Self::BalanceUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for PerpEngineEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PerpEngineEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProductUpdateFilter> for PerpEngineEvents {
        fn from(value: ProductUpdateFilter) -> Self {
            Self::ProductUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `addProduct` function with signature `addProduct(uint32,address,int128,int128,int128,int128,(int32,int32,int32,int32,int32))` and selector `0x367bc290`
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
        name = "addProduct",
        abi = "addProduct(uint32,address,int128,int128,int128,int128,(int32,int32,int32,int32,int32))"
    )]
    pub struct AddProductCall {
        pub health_group: u32,
        pub book: ::ethers::core::types::Address,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `applyDeltas` function with signature `applyDeltas((uint32,bytes32,int128,int128)[])` and selector `0x6c9a2ce3`
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
        name = "applyDeltas",
        abi = "applyDeltas((uint32,bytes32,int128,int128)[])"
    )]
    pub struct ApplyDeltasCall {
        pub deltas: ::std::vec::Vec<ProductDelta>,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(uint32,bytes32)` and selector `0xbf4c8f5f`
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
    #[ethcall(name = "balances", abi = "balances(uint32,bytes32)")]
    pub struct BalancesCall(pub u32, pub [u8; 32]);
    ///Container type for all input parameters for the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `0xd98752ec`
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
    #[ethcall(name = "burnLp", abi = "burnLp(uint32,bytes32,int128)")]
    pub struct BurnLpCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
        pub amount_lp: i128,
    }
    ///Container type for all input parameters for the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32,address)` and selector `0x011a447a`
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
    #[ethcall(name = "decomposeLps", abi = "decomposeLps(bytes32,bytes32,address)")]
    pub struct DecomposeLpsCall {
        pub liquidatee: [u8; 32],
        pub liquidator: [u8; 32],
        pub fee_calculator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `0x3056f78f`
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
    #[ethcall(name = "getAvailableSettle", abi = "getAvailableSettle(uint32)")]
    pub struct GetAvailableSettleCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
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
    #[ethcall(name = "getBalance", abi = "getBalance(uint32,bytes32)")]
    pub struct GetBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getBalanceAmount` function with signature `getBalanceAmount(uint32,bytes32)` and selector `0xe039bfa9`
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
    #[ethcall(name = "getBalanceAmount", abi = "getBalanceAmount(uint32,bytes32)")]
    pub struct GetBalanceAmountCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getBalances` function with signature `getBalances(uint32,bytes32)` and selector `0x6aca31a3`
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
    #[ethcall(name = "getBalances", abi = "getBalances(uint32,bytes32)")]
    pub struct GetBalancesCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
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
    #[ethcall(name = "getClearinghouse", abi = "getClearinghouse()")]
    pub struct GetClearinghouseCall;
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
    ///Container type for all input parameters for the `getEngineType` function with signature `getEngineType()` and selector `0x4604d19b`
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
    #[ethcall(name = "getEngineType", abi = "getEngineType()")]
    pub struct GetEngineTypeCall;
    ///Container type for all input parameters for the `getLpState` function with signature `getLpState(uint32)` and selector `0x3f857d5a`
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
    #[ethcall(name = "getLpState", abi = "getLpState(uint32)")]
    pub struct GetLpStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `0x2f8f1fb0`
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
    #[ethcall(name = "getOraclePriceX18", abi = "getOraclePriceX18(uint32)")]
    pub struct GetOraclePriceX18Call {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `0xd3d660cb`
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
    #[ethcall(name = "getOraclePricesX18", abi = "getOraclePricesX18(uint32)")]
    pub struct GetOraclePricesX18Call {
        pub health_group: u32,
    }
    ///Container type for all input parameters for the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `0x4427952d`
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
    #[ethcall(name = "getOrderbook", abi = "getOrderbook(uint32)")]
    pub struct GetOrderbookCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPoolState` function with signature `getPoolState(uint32)` and selector `0x8af6426a`
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
    #[ethcall(name = "getPoolState", abi = "getPoolState(uint32)")]
    pub struct GetPoolStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `0x1769225f`
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
    #[ethcall(name = "getPositionPnl", abi = "getPositionPnl(uint32,bytes32)")]
    pub struct GetPositionPnlCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
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
    #[ethcall(name = "getProductIds", abi = "getProductIds()")]
    pub struct GetProductIdsCall;
    ///Container type for all input parameters for the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `0xedf02653`
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
    #[ethcall(name = "getRawBalance", abi = "getRawBalance(uint32,bytes32)")]
    pub struct GetRawBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getRawLpBalance` function with signature `getRawLpBalance(uint32,bytes32)` and selector `0x6f27be34`
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
    #[ethcall(name = "getRawLpBalance", abi = "getRawLpBalance(uint32,bytes32)")]
    pub struct GetRawLpBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `0xc721bd65`
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
    #[ethcall(name = "getRawLpState", abi = "getRawLpState(uint32)")]
    pub struct GetRawLpStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getRawState` function with signature `getRawState(uint32)` and selector `0xec7a79c9`
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
    #[ethcall(name = "getRawState", abi = "getRawState(uint32)")]
    pub struct GetRawStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `0x388927b8`
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
        name = "getSettlementState",
        abi = "getSettlementState(uint32,bytes32)"
    )]
    pub struct GetSettlementStateCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
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
        name = "getStateAndBalance",
        abi = "getStateAndBalance(uint32,bytes32)"
    )]
    pub struct GetStateAndBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getStatesAndBalances` function with signature `getStatesAndBalances(uint32,bytes32)` and selector `0x3d5cc9dc`
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
        name = "getStatesAndBalances",
        abi = "getStatesAndBalances(uint32,bytes32)"
    )]
    pub struct GetStatesAndBalancesCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
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
    #[ethcall(name = "getVersion", abi = "getVersion()")]
    pub struct GetVersionCall;
    ///Container type for all input parameters for the `hasBalance` function with signature `hasBalance(uint32,bytes32)` and selector `0xde7ebc91`
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
    #[ethcall(name = "hasBalance", abi = "hasBalance(uint32,bytes32)")]
    pub struct HasBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address)` and selector `0x1459457a`
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
        name = "initialize",
        abi = "initialize(address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lpBalances` function with signature `lpBalances(uint32,bytes32)` and selector `0x08ed83c1`
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
    #[ethcall(name = "lpBalances", abi = "lpBalances(uint32,bytes32)")]
    pub struct LpBalancesCall(pub u32, pub [u8; 32]);
    ///Container type for all input parameters for the `lpStates` function with signature `lpStates(uint32)` and selector `0x042508e6`
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
    #[ethcall(name = "lpStates", abi = "lpStates(uint32)")]
    pub struct LpStatesCall(pub u32);
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert(int128[])` and selector `0x9b6f762b`
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
    #[ethcall(name = "manualAssert", abi = "manualAssert(int128[])")]
    pub struct ManualAssertCall {
        pub open_interests: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `markets` function with signature `markets(uint32)` and selector `0xece91e35`
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
    #[ethcall(name = "markets", abi = "markets(uint32)")]
    pub struct MarketsCall(pub u32);
    ///Container type for all input parameters for the `mintLp` function with signature `mintLp(uint32,bytes32,int128,int128,int128)` and selector `0x98de72fe`
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
    #[ethcall(name = "mintLp", abi = "mintLp(uint32,bytes32,int128,int128,int128)")]
    pub struct MintLpCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
        pub amount_base: i128,
        pub quote_amount_low: i128,
        pub quote_amount_high: i128,
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
    ///Container type for all input parameters for the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `0x64c42cc2`
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
        name = "perpPositionClosed",
        abi = "perpPositionClosed(uint32,bytes32)"
    )]
    pub struct PerpPositionClosedCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
    }
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
    ///Container type for all input parameters for the `setBalance` function with signature `setBalance(uint32,bytes32,(int128,int128,int128))` and selector `0x385de9c3`
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
        name = "setBalance",
        abi = "setBalance(uint32,bytes32,(int128,int128,int128))"
    )]
    pub struct SetBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
        pub balance: Balance,
    }
    ///Container type for all input parameters for the `setEndpoint` function with signature `setEndpoint(address)` and selector `0xdbbb4155`
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
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(address)")]
    pub struct SetEndpointCall {
        pub endpoint: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLpBalance` function with signature `setLpBalance(uint32,bytes32,(int128,int128))` and selector `0x3025586a`
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
        name = "setLpBalance",
        abi = "setLpBalance(uint32,bytes32,(int128,int128))"
    )]
    pub struct SetLpBalanceCall {
        pub product_id: u32,
        pub subaccount: [u8; 32],
        pub lp_balance: LpBalance,
    }
    ///Container type for all input parameters for the `setLpState` function with signature `setLpState(uint32,(int128,int128,int128,int128,int128))` and selector `0x26fa00ac`
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
        name = "setLpState",
        abi = "setLpState(uint32,(int128,int128,int128,int128,int128))"
    )]
    pub struct SetLpStateCall {
        pub product_id: u32,
        pub lp_state: LpState,
    }
    ///Container type for all input parameters for the `setState` function with signature `setState(uint32,(int128,int128,int128,int128))` and selector `0x7fa29d49`
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
        name = "setState",
        abi = "setState(uint32,(int128,int128,int128,int128))"
    )]
    pub struct SetStateCall {
        pub product_id: u32,
        pub state: State,
    }
    ///Container type for all input parameters for the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `0xd6b0e0b5`
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
    #[ethcall(name = "settlePnl", abi = "settlePnl(bytes32,uint256)")]
    pub struct SettlePnlCall {
        pub subaccount: [u8; 32],
        pub product_ids: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `0xb1cd4b8f`
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
        name = "socializeSubaccount",
        abi = "socializeSubaccount(bytes32,int128)"
    )]
    pub struct SocializeSubaccountCall {
        pub subaccount: [u8; 32],
        pub insurance: i128,
    }
    ///Container type for all input parameters for the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    #[ethcall(name = "states", abi = "states(uint32)")]
    pub struct StatesCall(pub u32);
    ///Container type for all input parameters for the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `0xc7167cf5`
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
    #[ethcall(name = "swapLp", abi = "swapLp(uint32,int128,int128)")]
    pub struct SwapLpCall {
        pub product_id: u32,
        pub base_delta: i128,
        pub quote_delta: i128,
    }
    ///Container type for all input parameters for the `swapLp` function with signature `swapLp(uint32,int128,int128,int128,int128)` and selector `0xd952f2a3`
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
    #[ethcall(name = "swapLp", abi = "swapLp(uint32,int128,int128,int128,int128)")]
    pub struct SwapLpWithProductIdAndAmountAndPriceX18Call {
        pub product_id: u32,
        pub amount: i128,
        pub price_x18: i128,
        pub size_increment: i128,
        pub lp_spread_x18: i128,
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
    ///Container type for all input parameters for the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))` and selector `0x3ec8c7da`
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
        name = "unsignedUpdateProductTx",
        abi = "unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))"
    )]
    pub struct UnsignedUpdateProductTxCall {
        pub p: UpdateProductTx,
    }
    ///Container type for all input parameters for the `updateProduct` function with signature `updateProduct(bytes)` and selector `0xc9fe9ac3`
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
    #[ethcall(name = "updateProduct", abi = "updateProduct(bytes)")]
    pub struct UpdateProductCall {
        pub tx: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateStates` function with signature `updateStates(uint128,int128[])` and selector `0x6736f5da`
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
    #[ethcall(name = "updateStates", abi = "updateStates(uint128,int128[])")]
    pub struct UpdateStatesCall {
        pub dt: u128,
        pub avg_price_diffs: ::std::vec::Vec<i128>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PerpEngineCalls {
        AddProduct(AddProductCall),
        ApplyDeltas(ApplyDeltasCall),
        Balances(BalancesCall),
        BurnLp(BurnLpCall),
        DecomposeLps(DecomposeLpsCall),
        GetAvailableSettle(GetAvailableSettleCall),
        GetBalance(GetBalanceCall),
        GetBalanceAmount(GetBalanceAmountCall),
        GetBalances(GetBalancesCall),
        GetClearinghouse(GetClearinghouseCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetLpState(GetLpStateCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderbook(GetOrderbookCall),
        GetPoolState(GetPoolStateCall),
        GetPositionPnl(GetPositionPnlCall),
        GetProductIds(GetProductIdsCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawLpBalance(GetRawLpBalanceCall),
        GetRawLpState(GetRawLpStateCall),
        GetRawState(GetRawStateCall),
        GetSettlementState(GetSettlementStateCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        GetStatesAndBalances(GetStatesAndBalancesCall),
        GetVersion(GetVersionCall),
        HasBalance(HasBalanceCall),
        Initialize(InitializeCall),
        LpBalances(LpBalancesCall),
        LpStates(LpStatesCall),
        ManualAssert(ManualAssertCall),
        Markets(MarketsCall),
        MintLp(MintLpCall),
        Owner(OwnerCall),
        PerpPositionClosed(PerpPositionClosedCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBalance(SetBalanceCall),
        SetEndpoint(SetEndpointCall),
        SetLpBalance(SetLpBalanceCall),
        SetLpState(SetLpStateCall),
        SetState(SetStateCall),
        SettlePnl(SettlePnlCall),
        SocializeSubaccount(SocializeSubaccountCall),
        States(StatesCall),
        SwapLp(SwapLpCall),
        SwapLpWithProductIdAndAmountAndPriceX18(SwapLpWithProductIdAndAmountAndPriceX18Call),
        TransferOwnership(TransferOwnershipCall),
        UnsignedUpdateProductTx(UnsignedUpdateProductTxCall),
        UpdateProduct(UpdateProductCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for PerpEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddProductCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddProduct(decoded));
            }
            if let Ok(decoded) = <ApplyDeltasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApplyDeltas(decoded));
            }
            if let Ok(decoded) = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnLp(decoded));
            }
            if let Ok(decoded) = <DecomposeLpsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DecomposeLps(decoded));
            }
            if let Ok(decoded) =
                <GetAvailableSettleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAvailableSettle(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetBalanceAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBalanceAmount(decoded));
            }
            if let Ok(decoded) = <GetBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalances(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) = <GetEngineTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEngineType(decoded));
            }
            if let Ok(decoded) = <GetLpStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLpState(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePriceX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) =
                <GetOraclePricesX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) = <GetOrderbookCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOrderbook(decoded));
            }
            if let Ok(decoded) = <GetPoolStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolState(decoded));
            }
            if let Ok(decoded) =
                <GetPositionPnlCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPositionPnl(decoded));
            }
            if let Ok(decoded) = <GetProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetProductIds(decoded));
            }
            if let Ok(decoded) = <GetRawBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawBalance(decoded));
            }
            if let Ok(decoded) =
                <GetRawLpBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawLpBalance(decoded));
            }
            if let Ok(decoded) = <GetRawLpStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawLpState(decoded));
            }
            if let Ok(decoded) = <GetRawStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRawState(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSettlementState(decoded));
            }
            if let Ok(decoded) =
                <GetStateAndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStateAndBalance(decoded));
            }
            if let Ok(decoded) =
                <GetStatesAndBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStatesAndBalances(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) = <HasBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasBalance(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LpBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LpBalances(decoded));
            }
            if let Ok(decoded) = <LpStatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LpStates(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Markets(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintLp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerpPositionClosedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpPositionClosed(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBalance(decoded));
            }
            if let Ok(decoded) = <SetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded) = <SetLpBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLpBalance(decoded));
            }
            if let Ok(decoded) = <SetLpStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetLpState(decoded));
            }
            if let Ok(decoded) = <SetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetState(decoded));
            }
            if let Ok(decoded) = <SettlePnlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SettlePnl(decoded));
            }
            if let Ok(decoded) =
                <SocializeSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) = <StatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::States(decoded));
            }
            if let Ok(decoded) = <SwapLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapLp(decoded));
            }
            if let Ok(decoded) = <SwapLpWithProductIdAndAmountAndPriceX18Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapLpWithProductIdAndAmountAndPriceX18(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnsignedUpdateProductTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedUpdateProductTx(decoded));
            }
            if let Ok(decoded) = <UpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateProduct(decoded));
            }
            if let Ok(decoded) = <UpdateStatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateStates(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PerpEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApplyDeltas(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Balances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DecomposeLps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAvailableSettle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalanceAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEngineType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLpState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOraclePriceX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOraclePricesX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderbook(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPositionPnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetProductIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawLpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawLpState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSettlementState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStateAndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStatesAndBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HasBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LpBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LpStates(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Markets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpPositionClosed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLpState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettlePnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SocializeSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::States(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapLpWithProductIdAndAmountAndPriceX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedUpdateProductTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateStates(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PerpEngineCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApplyDeltas(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecomposeLps(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAvailableSettle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalanceAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePricesX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderbook(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionPnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawLpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawLpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSettlementState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStateAndBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStatesAndBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpStates(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::Markets(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpPositionClosed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlePnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::SocializeSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::States(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLpWithProductIdAndAmountAndPriceX18(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedUpdateProductTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductCall> for PerpEngineCalls {
        fn from(value: AddProductCall) -> Self {
            Self::AddProduct(value)
        }
    }
    impl ::core::convert::From<ApplyDeltasCall> for PerpEngineCalls {
        fn from(value: ApplyDeltasCall) -> Self {
            Self::ApplyDeltas(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for PerpEngineCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<BurnLpCall> for PerpEngineCalls {
        fn from(value: BurnLpCall) -> Self {
            Self::BurnLp(value)
        }
    }
    impl ::core::convert::From<DecomposeLpsCall> for PerpEngineCalls {
        fn from(value: DecomposeLpsCall) -> Self {
            Self::DecomposeLps(value)
        }
    }
    impl ::core::convert::From<GetAvailableSettleCall> for PerpEngineCalls {
        fn from(value: GetAvailableSettleCall) -> Self {
            Self::GetAvailableSettle(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for PerpEngineCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetBalanceAmountCall> for PerpEngineCalls {
        fn from(value: GetBalanceAmountCall) -> Self {
            Self::GetBalanceAmount(value)
        }
    }
    impl ::core::convert::From<GetBalancesCall> for PerpEngineCalls {
        fn from(value: GetBalancesCall) -> Self {
            Self::GetBalances(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for PerpEngineCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for PerpEngineCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetEngineTypeCall> for PerpEngineCalls {
        fn from(value: GetEngineTypeCall) -> Self {
            Self::GetEngineType(value)
        }
    }
    impl ::core::convert::From<GetLpStateCall> for PerpEngineCalls {
        fn from(value: GetLpStateCall) -> Self {
            Self::GetLpState(value)
        }
    }
    impl ::core::convert::From<GetOraclePriceX18Call> for PerpEngineCalls {
        fn from(value: GetOraclePriceX18Call) -> Self {
            Self::GetOraclePriceX18(value)
        }
    }
    impl ::core::convert::From<GetOraclePricesX18Call> for PerpEngineCalls {
        fn from(value: GetOraclePricesX18Call) -> Self {
            Self::GetOraclePricesX18(value)
        }
    }
    impl ::core::convert::From<GetOrderbookCall> for PerpEngineCalls {
        fn from(value: GetOrderbookCall) -> Self {
            Self::GetOrderbook(value)
        }
    }
    impl ::core::convert::From<GetPoolStateCall> for PerpEngineCalls {
        fn from(value: GetPoolStateCall) -> Self {
            Self::GetPoolState(value)
        }
    }
    impl ::core::convert::From<GetPositionPnlCall> for PerpEngineCalls {
        fn from(value: GetPositionPnlCall) -> Self {
            Self::GetPositionPnl(value)
        }
    }
    impl ::core::convert::From<GetProductIdsCall> for PerpEngineCalls {
        fn from(value: GetProductIdsCall) -> Self {
            Self::GetProductIds(value)
        }
    }
    impl ::core::convert::From<GetRawBalanceCall> for PerpEngineCalls {
        fn from(value: GetRawBalanceCall) -> Self {
            Self::GetRawBalance(value)
        }
    }
    impl ::core::convert::From<GetRawLpBalanceCall> for PerpEngineCalls {
        fn from(value: GetRawLpBalanceCall) -> Self {
            Self::GetRawLpBalance(value)
        }
    }
    impl ::core::convert::From<GetRawLpStateCall> for PerpEngineCalls {
        fn from(value: GetRawLpStateCall) -> Self {
            Self::GetRawLpState(value)
        }
    }
    impl ::core::convert::From<GetRawStateCall> for PerpEngineCalls {
        fn from(value: GetRawStateCall) -> Self {
            Self::GetRawState(value)
        }
    }
    impl ::core::convert::From<GetSettlementStateCall> for PerpEngineCalls {
        fn from(value: GetSettlementStateCall) -> Self {
            Self::GetSettlementState(value)
        }
    }
    impl ::core::convert::From<GetStateAndBalanceCall> for PerpEngineCalls {
        fn from(value: GetStateAndBalanceCall) -> Self {
            Self::GetStateAndBalance(value)
        }
    }
    impl ::core::convert::From<GetStatesAndBalancesCall> for PerpEngineCalls {
        fn from(value: GetStatesAndBalancesCall) -> Self {
            Self::GetStatesAndBalances(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for PerpEngineCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<HasBalanceCall> for PerpEngineCalls {
        fn from(value: HasBalanceCall) -> Self {
            Self::HasBalance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for PerpEngineCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LpBalancesCall> for PerpEngineCalls {
        fn from(value: LpBalancesCall) -> Self {
            Self::LpBalances(value)
        }
    }
    impl ::core::convert::From<LpStatesCall> for PerpEngineCalls {
        fn from(value: LpStatesCall) -> Self {
            Self::LpStates(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for PerpEngineCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<MarketsCall> for PerpEngineCalls {
        fn from(value: MarketsCall) -> Self {
            Self::Markets(value)
        }
    }
    impl ::core::convert::From<MintLpCall> for PerpEngineCalls {
        fn from(value: MintLpCall) -> Self {
            Self::MintLp(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PerpEngineCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpPositionClosedCall> for PerpEngineCalls {
        fn from(value: PerpPositionClosedCall) -> Self {
            Self::PerpPositionClosed(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PerpEngineCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetBalanceCall> for PerpEngineCalls {
        fn from(value: SetBalanceCall) -> Self {
            Self::SetBalance(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for PerpEngineCalls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetLpBalanceCall> for PerpEngineCalls {
        fn from(value: SetLpBalanceCall) -> Self {
            Self::SetLpBalance(value)
        }
    }
    impl ::core::convert::From<SetLpStateCall> for PerpEngineCalls {
        fn from(value: SetLpStateCall) -> Self {
            Self::SetLpState(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for PerpEngineCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SettlePnlCall> for PerpEngineCalls {
        fn from(value: SettlePnlCall) -> Self {
            Self::SettlePnl(value)
        }
    }
    impl ::core::convert::From<SocializeSubaccountCall> for PerpEngineCalls {
        fn from(value: SocializeSubaccountCall) -> Self {
            Self::SocializeSubaccount(value)
        }
    }
    impl ::core::convert::From<StatesCall> for PerpEngineCalls {
        fn from(value: StatesCall) -> Self {
            Self::States(value)
        }
    }
    impl ::core::convert::From<SwapLpCall> for PerpEngineCalls {
        fn from(value: SwapLpCall) -> Self {
            Self::SwapLp(value)
        }
    }
    impl ::core::convert::From<SwapLpWithProductIdAndAmountAndPriceX18Call> for PerpEngineCalls {
        fn from(value: SwapLpWithProductIdAndAmountAndPriceX18Call) -> Self {
            Self::SwapLpWithProductIdAndAmountAndPriceX18(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PerpEngineCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnsignedUpdateProductTxCall> for PerpEngineCalls {
        fn from(value: UnsignedUpdateProductTxCall) -> Self {
            Self::UnsignedUpdateProductTx(value)
        }
    }
    impl ::core::convert::From<UpdateProductCall> for PerpEngineCalls {
        fn from(value: UpdateProductCall) -> Self {
            Self::UpdateProduct(value)
        }
    }
    impl ::core::convert::From<UpdateStatesCall> for PerpEngineCalls {
        fn from(value: UpdateStatesCall) -> Self {
            Self::UpdateStates(value)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(uint32,bytes32)` and selector `0xbf4c8f5f`
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
    pub struct BalancesReturn {
        pub amount: i128,
        pub v_quote_balance: i128,
        pub last_cumulative_funding_x18: i128,
    }
    ///Container type for all return fields from the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `0xd98752ec`
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
    pub struct BurnLpReturn {
        pub amount_base: i128,
        pub amount_quote: i128,
    }
    ///Container type for all return fields from the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32,address)` and selector `0x011a447a`
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
    pub struct DecomposeLpsReturn {
        pub liquidation_fees: i128,
    }
    ///Container type for all return fields from the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `0x3056f78f`
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
    pub struct GetAvailableSettleReturn(pub i128);
    ///Container type for all return fields from the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
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
    pub struct GetBalanceReturn(pub Balance);
    ///Container type for all return fields from the `getBalanceAmount` function with signature `getBalanceAmount(uint32,bytes32)` and selector `0xe039bfa9`
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
    pub struct GetBalanceAmountReturn(pub i128);
    ///Container type for all return fields from the `getBalances` function with signature `getBalances(uint32,bytes32)` and selector `0x6aca31a3`
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
    pub struct GetBalancesReturn(pub LpBalance, pub Balance);
    ///Container type for all return fields from the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
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
    pub struct GetClearinghouseReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getEngineType` function with signature `getEngineType()` and selector `0x4604d19b`
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
    pub struct GetEngineTypeReturn(pub u8);
    ///Container type for all return fields from the `getLpState` function with signature `getLpState(uint32)` and selector `0x3f857d5a`
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
    pub struct GetLpStateReturn(pub LpState);
    ///Container type for all return fields from the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `0x2f8f1fb0`
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
    pub struct GetOraclePriceX18Return(pub i128);
    ///Container type for all return fields from the `getOraclePricesX18` function with signature `getOraclePricesX18(uint32)` and selector `0xd3d660cb`
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
    pub struct GetOraclePricesX18Return(pub Prices);
    ///Container type for all return fields from the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `0x4427952d`
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
    pub struct GetOrderbookReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolState` function with signature `getPoolState(uint32)` and selector `0x8af6426a`
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
    pub struct GetPoolStateReturn(pub i128, pub i128);
    ///Container type for all return fields from the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `0x1769225f`
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
    pub struct GetPositionPnlReturn(pub i128);
    ///Container type for all return fields from the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
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
    pub struct GetProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `0xedf02653`
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
    pub struct GetRawBalanceReturn(pub Balance);
    ///Container type for all return fields from the `getRawLpBalance` function with signature `getRawLpBalance(uint32,bytes32)` and selector `0x6f27be34`
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
    pub struct GetRawLpBalanceReturn(pub LpBalance);
    ///Container type for all return fields from the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `0xc721bd65`
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
    pub struct GetRawLpStateReturn(pub LpState);
    ///Container type for all return fields from the `getRawState` function with signature `getRawState(uint32)` and selector `0xec7a79c9`
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
    pub struct GetRawStateReturn(pub State);
    ///Container type for all return fields from the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `0x388927b8`
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
    pub struct GetSettlementStateReturn {
        pub available_settle: i128,
        pub lp_state: LpState,
        pub lp_balance: LpBalance,
        pub state: State,
        pub balance: Balance,
    }
    ///Container type for all return fields from the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
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
    pub struct GetStateAndBalanceReturn(pub State, pub Balance);
    ///Container type for all return fields from the `getStatesAndBalances` function with signature `getStatesAndBalances(uint32,bytes32)` and selector `0x3d5cc9dc`
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
    pub struct GetStatesAndBalancesReturn(pub LpState, pub LpBalance, pub State, pub Balance);
    ///Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
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
    pub struct GetVersionReturn(pub u64);
    ///Container type for all return fields from the `hasBalance` function with signature `hasBalance(uint32,bytes32)` and selector `0xde7ebc91`
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
    pub struct HasBalanceReturn(pub bool);
    ///Container type for all return fields from the `lpBalances` function with signature `lpBalances(uint32,bytes32)` and selector `0x08ed83c1`
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
    pub struct LpBalancesReturn {
        pub amount: i128,
        pub last_cumulative_funding_x18: i128,
    }
    ///Container type for all return fields from the `lpStates` function with signature `lpStates(uint32)` and selector `0x042508e6`
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
    pub struct LpStatesReturn {
        pub supply: i128,
        pub last_cumulative_funding_x18: i128,
        pub cumulative_funding_per_lp_x18: i128,
        pub base: i128,
        pub quote: i128,
    }
    ///Container type for all return fields from the `markets` function with signature `markets(uint32)` and selector `0xece91e35`
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
    pub struct MarketsReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `0x64c42cc2`
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
    pub struct PerpPositionClosedReturn(pub bool);
    ///Container type for all return fields from the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `0xd6b0e0b5`
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
    pub struct SettlePnlReturn(pub i128);
    ///Container type for all return fields from the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `0xb1cd4b8f`
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
    pub struct SocializeSubaccountReturn(pub i128);
    ///Container type for all return fields from the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    pub struct StatesReturn {
        pub cumulative_funding_long_x18: i128,
        pub cumulative_funding_short_x18: i128,
        pub available_settle: i128,
        pub open_interest: i128,
    }
    ///Container type for all return fields from the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `0xc7167cf5`
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
    pub struct SwapLpReturn(pub i128, pub i128);
    ///Container type for all return fields from the `swapLp` function with signature `swapLp(uint32,int128,int128,int128,int128)` and selector `0xd952f2a3`
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
    pub struct SwapLpWithProductIdAndAmountAndPriceX18Return {
        pub base_swapped: i128,
        pub quote_swapped: i128,
    }
    ///Container type for all return fields from the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32)))` and selector `0x3ec8c7da`
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
    pub struct UnsignedUpdateProductTxReturn(pub UpdateProductTx);
    ///`RiskStore(int32,int32,int32,int32,int32)`
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
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub large_position_penalty: i32,
    }
    ///`Prices(int128,int128)`
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
    pub struct Prices {
        pub spot_price_x18: i128,
        pub perp_price_x18: i128,
    }
    ///`Balance(int128,int128,int128)`
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Balance {
        pub amount: i128,
        pub v_quote_balance: i128,
        pub last_cumulative_funding_x18: i128,
    }
    ///`LpBalance(int128,int128)`
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpBalance {
        pub amount: i128,
        pub last_cumulative_funding_x18: i128,
    }
    ///`LpState(int128,int128,int128,int128,int128)`
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpState {
        pub supply: i128,
        pub last_cumulative_funding_x18: i128,
        pub cumulative_funding_per_lp_x18: i128,
        pub base: i128,
        pub quote: i128,
    }
    ///`State(int128,int128,int128,int128)`
    #[derive(
        Archive,
        RkyvSerialize,
        RkyvDeserialize,
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct State {
        pub cumulative_funding_long_x18: i128,
        pub cumulative_funding_short_x18: i128,
        pub available_settle: i128,
        pub open_interest: i128,
    }
    ///`UpdateProductTx(uint32,int128,int128,int128,int128,(int32,int32,int32,int32,int32))`
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
    pub struct UpdateProductTx {
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
        pub risk_store: RiskStore,
    }
    ///`ProductDelta(uint32,bytes32,int128,int128)`
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
    pub struct ProductDelta {
        pub product_id: u32,
        pub subaccount: [u8; 32],
        pub amount_delta: i128,
        pub v_quote_delta: i128,
    }
}
