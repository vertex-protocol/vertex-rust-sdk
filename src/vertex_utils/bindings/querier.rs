pub use querier::*;
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
pub mod querier {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use super::super::super::serialize_utils::{
        deserialize_bytes32, deserialize_i128, deserialize_i256, deserialize_nested_vec_i128,
        deserialize_u128, deserialize_u256, deserialize_vec_i128, serialize_bytes32,
        serialize_i128, serialize_i256, serialize_nested_vec_i128, serialize_u128, serialize_u256,
        serialize_vec_i128,
    };
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
    use serde::{Deserialize, Serialize};

    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAllBooks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllBooks"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllProducts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                        ],
                                                                    ),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.ProductInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBookInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBookInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("engine"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IProductEngine"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bookInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct FQuerier.BookInfo"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getClearinghouse"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getClearinghouse"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getContractVersions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getContractVersions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_endpoint"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_clearinghouseLiq"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_spotEngine"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_perpEngine"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_querier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Versions"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPerpBalance"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.PerpBalance",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPerpBalances"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("perpBalances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.PerpBalance[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPerpProduct"),
                            inputs: ::std::vec![
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.PerpProduct",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPerpProducts"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("perpProducts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.PerpProduct[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotBalance"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.SpotBalance",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotBalances"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spotBalances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.SpotBalance[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotProduct"),
                            inputs: ::std::vec![
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.SpotProduct",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotProducts"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spotProducts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.SpotProduct[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubaccountInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSubaccountInfo"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                        ],
                                                                    ),
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                        ],
                                                                    ),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.SubaccountInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVersion"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "PRBMathSD59x18__SqrtNegativeInput",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMathSD59x18__SqrtNegativeInput",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRBMathSD59x18__SqrtOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRBMathSD59x18__SqrtOverflow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QUERIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct Querier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Querier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Querier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Querier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Querier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Querier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Querier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                QUERIER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getAllBooks` (0x354528e8) function
        pub fn get_all_books(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([53, 69, 40, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllProducts` (0x02ee3a52) function
        pub fn get_all_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([2, 238, 58, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBookInfo` (0x6124e5ff) function
        pub fn get_book_info(
            &self,
            product_id: u32,
            engine: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, BookInfo> {
            self.0
                .method_hash([97, 36, 229, 255], (product_id, engine))
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
        ///Calls the contract's `getContractVersions` (0xe4717e72) function
        pub fn get_contract_versions(
            &self,
            endpoint: ::ethers::core::types::Address,
            fees: ::ethers::core::types::Address,
            clearinghouse: ::ethers::core::types::Address,
            clearinghouse_liq: ::ethers::core::types::Address,
            spot_engine: ::ethers::core::types::Address,
            perp_engine: ::ethers::core::types::Address,
            querier: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Versions> {
            self.0
                .method_hash(
                    [228, 113, 126, 114],
                    (
                        endpoint,
                        fees,
                        clearinghouse,
                        clearinghouse_liq,
                        spot_engine,
                        perp_engine,
                        querier,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpBalance` (0xd7b229b6) function
        pub fn get_perp_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpBalance> {
            self.0
                .method_hash([215, 178, 41, 182], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpBalances` (0x2593eb5f) function
        pub fn get_perp_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpBalance>> {
            self.0
                .method_hash([37, 147, 235, 95], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpProduct` (0x1ae10bc5) function
        pub fn get_perp_product(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpProduct> {
            self.0
                .method_hash([26, 225, 11, 197], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpProducts` (0xee9928c9) function
        pub fn get_perp_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpProduct>> {
            self.0
                .method_hash([238, 153, 40, 201], product_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotBalance` (0x74173404) function
        pub fn get_spot_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotBalance> {
            self.0
                .method_hash([116, 23, 52, 4], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotBalances` (0x31546d51) function
        pub fn get_spot_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotBalance>> {
            self.0
                .method_hash([49, 84, 109, 81], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotProduct` (0x5723653f) function
        pub fn get_spot_product(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotProduct> {
            self.0
                .method_hash([87, 35, 101, 63], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotProducts` (0x75a5ab3c) function
        pub fn get_spot_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotProduct>> {
            self.0
                .method_hash([117, 165, 171, 60], product_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountInfo` (0x5d702e1a) function
        pub fn get_subaccount_info(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, SubaccountInfo> {
            self.0
                .method_hash([93, 112, 46, 26], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], clearinghouse)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Querier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `PRBMathSD59x18__SqrtNegativeInput` with signature `PRBMathSD59x18__SqrtNegativeInput(int256)` and selector `0xc11907fe`
    #[derive(
        Serialize,
        Deserialize,
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
        #[serde(
            serialize_with = "serialize_i256",
            deserialize_with = "deserialize_i256"
        )]
        pub x: ::ethers::core::types::I256,
    }
    ///Custom Error type `PRBMathSD59x18__SqrtOverflow` with signature `PRBMathSD59x18__SqrtOverflow(int256)` and selector `0x2c482c39`
    #[derive(
        Serialize,
        Deserialize,
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
        #[serde(
            serialize_with = "serialize_i256",
            deserialize_with = "deserialize_i256"
        )]
        pub x: ::ethers::core::types::I256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QuerierErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for QuerierErrors {
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
    impl ::ethers::core::abi::AbiEncode for QuerierErrors {
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
    impl ::ethers::contract::ContractRevert for QuerierErrors {
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
    impl ::core::fmt::Display for QuerierErrors {
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
    impl ::core::convert::From<::std::string::String> for QuerierErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtNegativeInput> for QuerierErrors {
        fn from(value: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            Self::PRBMathSD59x18__SqrtNegativeInput(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtOverflow> for QuerierErrors {
        fn from(value: PRBMathSD59x18__SqrtOverflow) -> Self {
            Self::PRBMathSD59x18__SqrtOverflow(value)
        }
    }
    ///Container type for all input parameters for the `getAllBooks` function with signature `getAllBooks()` and selector `0x354528e8`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAllBooks", abi = "getAllBooks()")]
    pub struct GetAllBooksCall;
    ///Container type for all input parameters for the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAllProducts", abi = "getAllProducts()")]
    pub struct GetAllProductsCall;
    ///Container type for all input parameters for the `getBookInfo` function with signature `getBookInfo(uint32,address)` and selector `0x6124e5ff`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBookInfo", abi = "getBookInfo(uint32,address)")]
    pub struct GetBookInfoCall {
        pub product_id: u32,
        pub engine: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
    #[derive(
        Serialize,
        Deserialize,
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
    ///Container type for all input parameters for the `getContractVersions` function with signature `getContractVersions(address,address,address,address,address,address,address)` and selector `0xe4717e72`
    #[derive(
        Serialize,
        Deserialize,
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
        name = "getContractVersions",
        abi = "getContractVersions(address,address,address,address,address,address,address)"
    )]
    pub struct GetContractVersionsCall {
        pub endpoint: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub clearinghouse_liq: ::ethers::core::types::Address,
        pub spot_engine: ::ethers::core::types::Address,
        pub perp_engine: ::ethers::core::types::Address,
        pub querier: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPerpBalance", abi = "getPerpBalance(bytes32,uint32)")]
    pub struct GetPerpBalanceCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `0x2593eb5f`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPerpBalances", abi = "getPerpBalances(bytes32,uint32[])")]
    pub struct GetPerpBalancesCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `0x1ae10bc5`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPerpProduct", abi = "getPerpProduct(uint32)")]
    pub struct GetPerpProductCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `0xee9928c9`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPerpProducts", abi = "getPerpProducts(uint32[])")]
    pub struct GetPerpProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `0x74173404`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSpotBalance", abi = "getSpotBalance(bytes32,uint32)")]
    pub struct GetSpotBalanceCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `0x31546d51`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSpotBalances", abi = "getSpotBalances(bytes32,uint32[])")]
    pub struct GetSpotBalancesCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `0x5723653f`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSpotProduct", abi = "getSpotProduct(uint32)")]
    pub struct GetSpotProductCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `0x75a5ab3c`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSpotProducts", abi = "getSpotProducts(uint32[])")]
    pub struct GetSpotProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `0x5d702e1a`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSubaccountInfo", abi = "getSubaccountInfo(bytes32)")]
    pub struct GetSubaccountInfoCall {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
    #[derive(
        Serialize,
        Deserialize,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
    #[derive(
        Serialize,
        Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QuerierCalls {
        GetAllBooks(GetAllBooksCall),
        GetAllProducts(GetAllProductsCall),
        GetBookInfo(GetBookInfoCall),
        GetClearinghouse(GetClearinghouseCall),
        GetContractVersions(GetContractVersionsCall),
        GetPerpBalance(GetPerpBalanceCall),
        GetPerpBalances(GetPerpBalancesCall),
        GetPerpProduct(GetPerpProductCall),
        GetPerpProducts(GetPerpProductsCall),
        GetSpotBalance(GetSpotBalanceCall),
        GetSpotBalances(GetSpotBalancesCall),
        GetSpotProduct(GetSpotProductCall),
        GetSpotProducts(GetSpotProductsCall),
        GetSubaccountInfo(GetSubaccountInfoCall),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuerierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetAllBooksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllBooks(decoded));
            }
            if let Ok(decoded) =
                <GetAllProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllProducts(decoded));
            }
            if let Ok(decoded) = <GetBookInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBookInfo(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
            }
            if let Ok(decoded) =
                <GetContractVersionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetContractVersions(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpBalance(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpBalances(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpProduct(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotBalance(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotBalances(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotProduct(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountInfo(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QuerierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAllBooks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBookInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetContractVersions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPerpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for QuerierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAllBooks(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBookInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetContractVersions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAllBooksCall> for QuerierCalls {
        fn from(value: GetAllBooksCall) -> Self {
            Self::GetAllBooks(value)
        }
    }
    impl ::core::convert::From<GetAllProductsCall> for QuerierCalls {
        fn from(value: GetAllProductsCall) -> Self {
            Self::GetAllProducts(value)
        }
    }
    impl ::core::convert::From<GetBookInfoCall> for QuerierCalls {
        fn from(value: GetBookInfoCall) -> Self {
            Self::GetBookInfo(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for QuerierCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetContractVersionsCall> for QuerierCalls {
        fn from(value: GetContractVersionsCall) -> Self {
            Self::GetContractVersions(value)
        }
    }
    impl ::core::convert::From<GetPerpBalanceCall> for QuerierCalls {
        fn from(value: GetPerpBalanceCall) -> Self {
            Self::GetPerpBalance(value)
        }
    }
    impl ::core::convert::From<GetPerpBalancesCall> for QuerierCalls {
        fn from(value: GetPerpBalancesCall) -> Self {
            Self::GetPerpBalances(value)
        }
    }
    impl ::core::convert::From<GetPerpProductCall> for QuerierCalls {
        fn from(value: GetPerpProductCall) -> Self {
            Self::GetPerpProduct(value)
        }
    }
    impl ::core::convert::From<GetPerpProductsCall> for QuerierCalls {
        fn from(value: GetPerpProductsCall) -> Self {
            Self::GetPerpProducts(value)
        }
    }
    impl ::core::convert::From<GetSpotBalanceCall> for QuerierCalls {
        fn from(value: GetSpotBalanceCall) -> Self {
            Self::GetSpotBalance(value)
        }
    }
    impl ::core::convert::From<GetSpotBalancesCall> for QuerierCalls {
        fn from(value: GetSpotBalancesCall) -> Self {
            Self::GetSpotBalances(value)
        }
    }
    impl ::core::convert::From<GetSpotProductCall> for QuerierCalls {
        fn from(value: GetSpotProductCall) -> Self {
            Self::GetSpotProduct(value)
        }
    }
    impl ::core::convert::From<GetSpotProductsCall> for QuerierCalls {
        fn from(value: GetSpotProductsCall) -> Self {
            Self::GetSpotProducts(value)
        }
    }
    impl ::core::convert::From<GetSubaccountInfoCall> for QuerierCalls {
        fn from(value: GetSubaccountInfoCall) -> Self {
            Self::GetSubaccountInfo(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for QuerierCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for QuerierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    ///Container type for all return fields from the `getAllBooks` function with signature `getAllBooks()` and selector `0x354528e8`
    #[derive(
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
    pub struct GetAllBooksReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
    #[derive(
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
    pub struct GetAllProductsReturn(pub ProductInfo);
    ///Container type for all return fields from the `getBookInfo` function with signature `getBookInfo(uint32,address)` and selector `0x6124e5ff`
    #[derive(
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
    pub struct GetBookInfoReturn {
        pub book_info: BookInfo,
    }
    ///Container type for all return fields from the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
    #[derive(
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
    pub struct GetClearinghouseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getContractVersions` function with signature `getContractVersions(address,address,address,address,address,address,address)` and selector `0xe4717e72`
    #[derive(
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
    pub struct GetContractVersionsReturn(pub Versions);
    ///Container type for all return fields from the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
    #[derive(
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
    pub struct GetPerpBalanceReturn(pub PerpBalance);
    ///Container type for all return fields from the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `0x2593eb5f`
    #[derive(
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
    pub struct GetPerpBalancesReturn {
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
    }
    ///Container type for all return fields from the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `0x1ae10bc5`
    #[derive(
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
    pub struct GetPerpProductReturn(pub PerpProduct);
    ///Container type for all return fields from the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `0xee9928c9`
    #[derive(
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
    pub struct GetPerpProductsReturn {
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///Container type for all return fields from the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `0x74173404`
    #[derive(
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
    pub struct GetSpotBalanceReturn(pub SpotBalance);
    ///Container type for all return fields from the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `0x31546d51`
    #[derive(
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
    pub struct GetSpotBalancesReturn {
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
    }
    ///Container type for all return fields from the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `0x5723653f`
    #[derive(
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
    pub struct GetSpotProductReturn(pub SpotProduct);
    ///Container type for all return fields from the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `0x75a5ab3c`
    #[derive(
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
    pub struct GetSpotProductsReturn {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
    }
    ///Container type for all return fields from the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `0x5d702e1a`
    #[derive(
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
    pub struct GetSubaccountInfoReturn(pub SubaccountInfo);
    ///Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
    #[derive(
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
    pub struct GetVersionReturn(pub u64);
    ///`BookInfo(int128,int128,int128,int128,int128)`
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
    #[archive(check_bytes)]
    pub struct BookInfo {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub price_increment_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub collected_fees: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub lp_spread_x18: i128,
    }
    ///`HealthInfo(int128,int128,int128)`
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
    #[archive(check_bytes)]
    pub struct HealthInfo {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub assets: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub liabilities: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub health: i128,
    }
    ///`PerpBalance(uint32,(int128,int128),(int128,int128,int128))`
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
    #[archive(check_bytes)]
    pub struct PerpBalance {
        pub product_id: u32,
        pub lp_balance: crate::vertex_utils::bindings::perp_engine::LpBalance,
        pub balance: crate::vertex_utils::bindings::perp_engine::Balance,
    }
    ///`PerpProduct(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))`
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
    #[archive(check_bytes)]
    pub struct PerpProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub state: crate::vertex_utils::bindings::perp_engine::State,
        pub lp_state: crate::vertex_utils::bindings::perp_engine::LpState,
        pub book_info: BookInfo,
    }
    ///`ProductInfo((uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`
    #[derive(
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
    pub struct ProductInfo {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///`SpotBalance(uint32,(int128),(int128,int128))`
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
    #[archive(check_bytes)]
    pub struct SpotBalance {
        pub product_id: u32,
        pub lp_balance: crate::vertex_utils::bindings::spot_engine::LpBalance,
        pub balance: crate::vertex_utils::bindings::spot_engine::Balance,
    }
    ///`SpotProduct(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))`
    #[derive(
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
    pub struct SpotProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub config: Config,
        pub state: crate::vertex_utils::bindings::spot_engine::State,
        pub lp_state: crate::vertex_utils::bindings::spot_engine::LpState,
        pub book_info: BookInfo,
    }
    ///`SubaccountInfo(bytes32,bool,(int128,int128,int128)[],int128[][],uint32,uint32,(uint32,(int128),(int128,int128))[],(uint32,(int128,int128),(int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`
    #[derive(
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
    pub struct SubaccountInfo {
        #[serde(
            serialize_with = "serialize_bytes32",
            deserialize_with = "deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub exists: bool,
        pub healths: ::std::vec::Vec<HealthInfo>,
        #[serde(
            serialize_with = "serialize_nested_vec_i128",
            deserialize_with = "deserialize_nested_vec_i128"
        )]
        pub health_contributions: ::std::vec::Vec<::std::vec::Vec<i128>>,
        pub spot_count: u32,
        pub perp_count: u32,
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///`IperpEngineBalance(int128,int128,int128)`
    #[derive(
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
    pub struct IperpEngineBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    ///`IperpEngineLpBalance(int128,int128)`
    #[derive(
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
    pub struct IperpEngineLpBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    ///`IperpEngineLpState(int128,int128,int128,int128,int128)`
    #[derive(
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
    pub struct IperpEngineLpState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub supply: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_per_lp_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub base: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub quote: i128,
    }
    ///`IperpEngineState(int128,int128,int128,int128)`
    #[derive(
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
    pub struct IperpEngineState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub open_interest: i128,
    }
    ///`IspotEngineBalance(int128,int128)`
    #[derive(
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
    pub struct IspotEngineBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub last_cumulative_multiplier_x18: i128,
    }
    ///`Config(address,int128,int128,int128,int128)`
    #[derive(
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
    pub struct Config {
        pub token: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_inflection_util_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_floor_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_small_cap_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub interest_large_cap_x18: i128,
    }
    ///`IspotEngineLpBalance(int128)`
    #[derive(
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
    pub struct IspotEngineLpBalance {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub amount: i128,
    }
    ///`IspotEngineLpState(int128,(int128,int128),(int128,int128))`
    #[derive(
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
    pub struct IspotEngineLpState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub supply: i128,
        pub quote: crate::vertex_utils::bindings::spot_engine::Balance,
        pub base: crate::vertex_utils::bindings::spot_engine::Balance,
    }
    ///`IspotEngineState(int128,int128,int128,int128)`
    #[derive(
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
    pub struct IspotEngineState {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_deposits_multiplier_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub cumulative_borrows_multiplier_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub total_deposits_normalized: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub total_borrows_normalized: i128,
    }
    ///`Risk(int128,int128,int128,int128,int128)`
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
    #[archive(check_bytes)]
    pub struct Risk {
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub long_weight_initial_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub short_weight_initial_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub long_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub short_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "serialize_i128",
            deserialize_with = "deserialize_i128"
        )]
        pub large_position_penalty_x18: i128,
    }
    ///`Versions(uint64,uint64,uint64,uint64,uint64,uint64,uint64,uint64[])`
    #[derive(
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
    pub struct Versions {
        pub endpoint: u64,
        pub fee_calculator: u64,
        pub clearinghouse: u64,
        pub clearinghouse_liq: u64,
        pub spot_engine: u64,
        pub perp_engine: u64,
        pub querier: u64,
        pub books: ::std::vec::Vec<u64>,
    }
}
