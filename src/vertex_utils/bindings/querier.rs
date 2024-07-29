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
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAllProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllProducts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isoGroup"),
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
                    ::std::borrow::ToOwned::to_owned("getCrossProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCrossProducts"),
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
                (
                    ::std::borrow::ToOwned::to_owned("legacyRisk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("legacyRisk"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
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
                                        ::std::borrow::ToOwned::to_owned("struct RiskHelper.Risk"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
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
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FQuerier.LegacyRisk",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QUERIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa<\x8A\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80ci\xD3;\xE2\x11a\0\x97W\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\x8EW\x80c\xC4\xD6m\xE8\x14a\x02\xA9W\x80c\xD7\xB2)\xB6\x14a\x02\xBEW\x80c\xEE\x99(\xC9\x14a\x02\xDEW`\0\x80\xFD[\x80ci\xD3;\xE2\x14a\x01\xE3W\x80ct\x174\x04\x14a\x01\xEBW\x80cu\xA5\xAB<\x14a\x02\x0BW\x80c\x86\x99R\xFD\x14a\x02+W`\0\x80\xFD[\x80c1TmQ\x11a\0\xD3W\x80c1TmQ\x14a\x01cW\x80cI\xFE\x1B<\x14a\x01\x83W\x80cW#e?\x14a\x01\xA3W\x80c]p.\x1A\x14a\x01\xC3W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xFAW\x80c\x1A\xE1\x0B\xC5\x14a\x01#W\x80c%\x93\xEB_\x14a\x01CW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a(\x8FV[a\x02\xFEV[`@Qa\x01\x1A\x91\x90a(\xACV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a(\x8FV[a\x04oV[`@Qa\x01\x1A\x91\x90a*4V[a\x01Va\x01Q6`\x04a+nV[a\x06UV[`@Qa\x01\x1A\x91\x90a,\x17V[a\x01va\x01q6`\x04a+nV[a\x07+V[`@Qa\x01\x1A\x91\x90a,\xE7V[a\x01\x96a\x01\x916`\x04a(\x8FV[a\x07\xFAV[`@Qa\x01\x1A\x91\x90a/\rV[a\x01\xB6a\x01\xB16`\x04a(\x8FV[a\x08JV[`@Qa\x01\x1A\x91\x90a/OV[a\x01\xD6a\x01\xD16`\x04a/^V[a\nGV[`@Qa\x01\x1A\x91\x90a0\x8AV[a\x01\x96a\x1C\xADV[a\x01\xFEa\x01\xF96`\x04a1\x92V[a\x1C\xD0V[`@Qa\x01\x1A\x91\x90a1\xC2V[a\x02\x1Ea\x02\x196`\x04a1\xD0V[a\x1D\x82V[`@Qa\x01\x1A\x91\x90a2\rV[a\x01\ra\x0296`\x04a2/V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x02\xBCa\x02\xB76`\x04a2\xBEV[a\x1EVV[\0[a\x02\xD1a\x02\xCC6`\x04a1\x92V[a \\V[`@Qa\x01\x1A\x91\x90a2\xDBV[a\x02\xF1a\x02\xEC6`\x04a1\xD0V[a \xE3V[`@Qa\x01\x1A\x91\x90a2\xE9V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x031a!\xB1V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA1\x91\x90a2\xFCV[\x90P`\0a\x03\xADa!\xB1V[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1D\x91\x90a3\xD1V[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[a\x04wa&-V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF6\x91\x90a5\xD0V[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05s\x91\x90a6*V[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x05\xF2\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06AW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06JV[a\x06J\x87a\x02\xFEV[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06qWa\x06qa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xAAW\x81` \x01[a\x06\x97a&\xFBV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x8FW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07$W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xD9Wa\x06\xD9a6FV[` \x02` \x01\x01Q\x90Pa\x06\xED\x85\x82a \\V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x05Wa\x07\x05a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\x1C\x90a6rV[\x91PPa\x06\xB0V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07GWa\x07Ga*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x80W\x81` \x01[a\x07ma'YV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07eW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07$W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xAFWa\x07\xAFa6FV[` \x02` \x01\x01Q\x90Pa\x07\xC3\x85\x82a\x1C\xD0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xDBWa\x07\xDBa6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xF2\x90a6rV[\x91PPa\x07\x86V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x08\x1A\x84a\"\x1FV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x082\x84a\x1D\x82V[\x81R` \x01a\x08@\x83a \xE3V[\x90R\x94\x93PPPPV[a\x08Ra'\xA3V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a6\x95V[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tN\x91\x90a6*V[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\t\xCD\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF2\x91\x90a7#V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\n\xF3\x81a\"\x1FV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\x18WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\xB2W\x81\x84\x82\x81Q\x81\x10a\x0BiWa\x0Bia6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xA0W\x83\x81\x81Q\x81\x10a\x0B\x8FWa\x0B\x8Fa6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\xAA\x81a7\x91V[\x91PPa\x0BMV[P`\0[\x82Q\x81\x10\x15a\x0C\x1BW\x81\x83\x82\x81Q\x81\x10a\x0B\xD2Wa\x0B\xD2a6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\tW\x82\x81\x81Q\x81\x10a\x0B\xF8Wa\x0B\xF8a6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0C\x13\x81a7\x91V[\x91PPa\x0B\xB6V[Pa\x0C'\x81`\x01a7\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C?Wa\x0C?a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CrW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C]W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C\xD5W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\xB7Wa\x0C\xB7a6FV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C\xCD\x90a7\x91V[\x91PPa\x0C{V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xF1Wa\x0C\xF1a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r*W\x81` \x01[a\r\x17a'YV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x0FW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rJWa\rJa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x83W\x81` \x01[a\rpa&\xFBV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rhW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA3Wa\r\xA3a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xDCW\x81` \x01[a\r\xC9a'\xA3V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xC1W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xFDWa\r\xFDa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E6W\x81` \x01[a\x0E#a&-V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x1BW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x12iW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0EiWa\x0Eia6FV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90a6\x95V[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fx\x91\x90a6*V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xC4Wa\x0F\xC4a6FV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10H\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xC2\x91\x90a7#V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x10\xDC\x89a\x02\xFEV[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x10\xF4\x82a6rV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\x19Wa\x11\x19a6FV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12PW\x83Q`\0\x90a\x11h\x90\x84\x90a\x11_\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[a#\x18V[`\x0F\x0B\x90a#\xB1V[\x87Q\x90\x91P`\x0F\x0B\x15a\x11\xEBW\x87Q\x87Q`\0\x91a\x11\x89\x91`\x0F\x0B\x90a$4V[\x90Pa\x11\xDDa\x11\xA9\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[`@\x8B\x01QQa\x11_\x90a\x11\xC0\x90`\x0F\x0B\x85a#\xB1V[` \x8D\x01QQa\x11\xD3\x90`\x0F\x0B\x86a#\xB1V[\x89`\x80\x01Qa$\x9DV[a\x11\xE7\x90\x83a7\xD8V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x08Wa\x12\x08a6FV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12$Wa\x12$a6FV[` \x02` \x01\x01\x81\x81Qa\x128\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x12I\x90P\x81a8'V[\x90Pa\x11'V[PPPPPPPP\x80a\x12b\x90a6rV[\x90Pa\x0E@V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16PW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x96Wa\x12\x96a6FV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13%\x91\x90a5\xD0V[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA5\x91\x90a6*V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xF1Wa\x13\xF1a6FV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x14u\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x14\x8F\x89a\x02\xFEV[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x14\xA7\x82a6rV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\xCCWa\x14\xCCa6FV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x167W`\0\x84` \x01Qa\x15\x17\x84a\x11_\x88`\0\x01Qa\x11_\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[a\x15!\x91\x90a7\xD8V[\x87Q\x90\x91P`\x0F\x0B\x15a\x15\xD2W\x87Q\x87Q`\0\x91a\x15B\x91`\x0F\x0B\x90a$4V[\x90Pa\x15j\x88`\0\x01Qa\x11_\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba$\xD9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\xBAa\x15\x88\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[``\x8C\x01Qa\x11_\x90a\x15\x9E\x90`\x0F\x0B\x86a#\xB1V[`\x80\x8E\x01Qa\x15\xB0\x90`\x0F\x0B\x87a#\xB1V[\x8A`\x80\x01Qa$\x9DV[a\x15\xC4\x91\x90a7\xD8V[a\x15\xCE\x90\x83a7\xD8V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xEFWa\x15\xEFa6FV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x16\x0BWa\x16\x0Ba6FV[` \x02` \x01\x01\x81\x81Qa\x16\x1F\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x160\x90P\x81a8'V[\x90Pa\x14\xDAV[PPPPPPPP\x80a\x16I\x90a6rV[\x90Pa\x12mV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC9\x91\x90a8FV[\x90P[\x80\x15a\x1AUW`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1AMW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x17\x1CWa\x17\x1Ca7\xC2V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17:\x93\x92\x91\x90a8_V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17{\x91\x90a8\x90V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x17\x90WPa\x1A;V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x17\xC1Wa\x17\xC1a7\xC2V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDF\x93\x92\x91\x90a8_V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18 \x91\x90a8\x90V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18CWP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18OWPPa\x1A;V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x18~W\x81Q\x83Qa\x18w\x91\x90a\x18r\x90a8\xACV[a%BV[\x90Pa\x18\xA1V[\x81Q\x83Qa\x18\x95\x91\x90a\x18\x90\x90a8\xACV[a%^V[a\x18\x9E\x90a8\xACV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x18\xB9\x91\x90a7\xD8V[a\x18\xC3\x91\x90a8\xE8V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x19\x13W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xF0\x91\x90a9/V[a\x18\xFA\x91\x90a8\xE8V[a\x19\x0C\x90g\r\xE0\xB6\xB3\xA7d\0\0a9/V[\x90Pa\x19LV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19-\x91\x90a9/V[a\x197\x91\x90a8\xE8V[a\x19I\x90g\r\xE0\xB6\xB3\xA7d\0\0a9/V[\x90P[`\0`\x02a\x19\x7Fa\x19]\x85\x85a9/V[a\x11_\x89` \x01Q\x89` \x01Qa\x19t\x91\x90a7\xD8V[`\x0F\x89\x90\x0B\x90a#\xB1V[a\x19\x89\x91\x90a8\xE8V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\xA8Wa\x19\xA8a6FV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xC4Wa\x19\xC4a6FV[` \x02` \x01\x01\x81\x81Qa\x19\xD8\x91\x90a7\xD8V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x19\xFEWa\x19\xFEa6FV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1A\x1AWa\x1A\x1Aa6FV[` \x02` \x01\x01\x81\x81Qa\x1A.\x91\x90a7\xD8V[`\x0F\x0B\x90RPPPPPPP[\x80a\x1AE\x81a8'V[\x91PPa\x16\xE5V[PPPa\x16\xCCV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\xA2W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1B\xF9W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xA4Wa\x1A\xA4a6FV[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xC6Wa\x1A\xC6a6FV[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1BbW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xF6Wa\x1A\xF6a6FV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x18Wa\x1B\x18a6FV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B?Wa\x1B?a6FV[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1BW\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x1B\xE9V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x81Wa\x1B\x81a6FV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA3Wa\x1B\xA3a6FV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xCAWa\x1B\xCAa6FV[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1B\xE2\x91\x90a9/V[`\x0F\x0B\x90RP[a\x1B\xF2\x81a9\x7FV[\x90Pa\x1AmV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\x19Wa\x1C\x19a6FV[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CDWa\x1CDa6FV[` \x02` \x01\x01Q`\0\x01Qa\x1CZ\x91\x90a9/V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CyWa\x1Cya6FV[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1C\x9B\x90a9\x7FV[\x90Pa\x1AXV[P\x92\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x1C\xCB`\0a\x07\xFAV[\x90P\x90V[a\x1C\xD8a'YV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DW\x91\x90a6\x95V[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9EWa\x1D\x9Ea*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xD7W\x81` \x01[a\x1D\xC4a'\xA3V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xBCW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EPW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\x06Wa\x1E\x06a6FV[` \x02` \x01\x01Q\x90Pa\x1E\x19\x81a\x08JV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E1Wa\x1E1a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1EH\x90a6rV[\x91PPa\x1D\xDDV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1E\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F1\x91\x90a9\x9BV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1Fs\x91`\x04\x01a9\xB8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xB4\x91\x90a9\x9BV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F\xF8\x90`\x01\x90`\x04\x01a9\xB8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 9\x91\x90a9\x9BV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a da&\xFBV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DW\x91\x90a5\xD0V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFFWa \xFFa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!8W\x81` \x01[a!%a&-V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\x1DW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EPW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!gWa!ga6FV[` \x02` \x01\x01Q\x90Pa!z\x81a\x04oV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\x92Wa!\x92a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a!\xA9\x90a6rV[\x91PPa!>V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xCB\x91\x90a9\x9BV[`\x02T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R``\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x9A\x91\x90\x81\x01\x90a9\xD2V[`\x03T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x11\x91\x90\x81\x01\x90a9\xD2V[\x90P\x91P\x91V[`\0`\x02\x82`\x02\x81\x11\x15a#.Wa#.a7\xC2V[\x03a#BWPg\r\xE0\xB6\xB3\xA7d\0\0a#\xAAV[`\0\x80\x84`\x0F\x0B\x12a#{W`\0\x83`\x02\x81\x11\x15a#bWa#ba7\xC2V[\x14a#qW\x84`@\x01Qa#tV[\x84Q[\x90Pa#\xA7V[`\0\x83`\x02\x81\x11\x15a#\x8FWa#\x8Fa7\xC2V[\x14a#\x9EW\x84``\x01Qa#\xA4V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xF3WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a$xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a#\xC8Wa#\xC8a8\xD2V[`\0a$\xCE\x83`\x0F\x0Ba$\xBC\x84\x87`\x0F\x0Ba#\xB1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba$\xC9\x91\x90a:\xC1V[a%sV[a#\xA7\x90`\x02a;HV[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xF3WP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a%WW\x81a#\xAAV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a%WW\x81a#\xAAV[`\0\x80\x82\x12\x15a%\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1E\xABV[`\x03\x82\x13\x15a&\x1EWP\x80`\0a%\xDD`\x02\x83a;\xE6V[a%\xE8\x90`\x01a<\x14V[\x90P[\x81\x81\x12\x15a\x1EPW\x90P\x80`\x02\x81a&\x03\x81\x86a;\xE6V[a&\r\x91\x90a<\x14V[a&\x17\x91\x90a;\xE6V[\x90Pa%\xEBV[\x81\x15a&(WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a'6`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a&\xF6`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a&\xC5`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a'{`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\x8CW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(\xA1W`\0\x80\xFD[\x815a#\xAA\x81a(zV[`\xA0\x81\x01a\x1D|\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa)W`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa)\x97`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa)\xE5a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[PPPV[a\x02\xA0\x81\x01a\x1D|\x82\x84a(\xF4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a*CV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a*CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\xCEWa*\xCEa*CV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*\xF0Wa*\xF0a*CV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a+\x0BW`\0\x80\xFD[\x815` a+ a+\x1B\x83a*\xD6V[a*\xA5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a+?W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a+cW\x805a+V\x81a(zV[\x83R\x91\x83\x01\x91\x83\x01a+CV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x81W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x9FW`\0\x80\xFD[a+\xAB\x85\x82\x86\x01a*\xFAV[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa+\xE4` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa*/``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a,YWa,F\x83\x85Qa+\xB5V[\x92\x84\x01\x92`\xC0\x92\x90\x92\x01\x91`\x01\x01a,3V[P\x90\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa*/`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa,\xC9\x87\x83Qa,eV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a,\xB6V[P\x94\x95\x94PPPPPV[` \x81R`\0a#\xAA` \x83\x01\x84a,\xA2V[\x80Q`\x0F\x0B\x82R` \x81\x01Qa-&` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa-\xAA`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa.9a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa.Ma\x02\0\x84\x01\x82a,\xFAV[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa.\xBE\x87\x83Qa-GV[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xABV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa.\xF9\x87\x83Qa(\xF4V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xE6V[` \x81R`\0\x82Q`@` \x84\x01Ra/)``\x84\x01\x82a.\x97V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra/F\x82\x82a.\xD2V[\x95\x94PPPPPV[a\x03@\x81\x01a\x1D|\x82\x84a-GV[`\0` \x82\x84\x03\x12\x15a/pW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa/\xBE\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a/\x8BV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a0BW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a0-W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a0\x0EV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a/\xF0V[P\x92\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa0w\x87\x83Qa+\xB5V[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a0dV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa0\xAA`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra0\xC7a\x01`\x85\x01\x83a/wV[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra0\xE5\x84\x83a/\xD1V[\x93P`\x80\x87\x01Q\x91Pa1\0`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra1.\x84\x83a,\xA2V[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra1M\x85\x84a0PV[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra1l\x85\x84a.\x97V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa1\x88\x83\x82a.\xD2V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xA5W`\0\x80\xFD[\x825\x91P` \x83\x015a1\xB7\x81a(zV[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\x1D|\x82\x84a,eV[`\0` \x82\x84\x03\x12\x15a1\xE2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xF9W`\0\x80\xFD[a2\x05\x84\x82\x85\x01a*\xFAV[\x94\x93PPPPV[` \x81R`\0a#\xAA` \x83\x01\x84a.\x97V[\x80`\x0F\x0B\x81\x14a(\x8CW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a2AW`\0\x80\xFD[a2Ia*YV[\x825a2T\x81a2 V[\x81R` \x83\x015a2d\x81a2 V[` \x82\x01R`@\x83\x015a2w\x81a2 V[`@\x82\x01R``\x83\x015a2\x8A\x81a2 V[``\x82\x01R`\x80\x83\x015a2\x9D\x81a2 V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\x8CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a2\xD0W`\0\x80\xFD[\x815a#\xAA\x81a2\xA9V[`\xC0\x81\x01a\x1D|\x82\x84a+\xB5V[` \x81R`\0a#\xAA` \x83\x01\x84a.\xD2V[`\0`\x80\x82\x84\x03\x12\x15a3\x0EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a31Wa31a*CV[`@R\x82Qa3?\x81a(zV[\x81R` \x83\x01Qa3O\x81a2 V[` \x82\x01R`@\x83\x01Qa3b\x81a2 V[`@\x82\x01R``\x83\x01Qa3u\x81a2 V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\x93W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\xB6Wa3\xB6a*CV[\x80`@RP\x80\x91P\x82Qa3\xC9\x81a2 V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xE3W`\0\x80\xFD[a#\xAA\x83\x83a3\x81V[`\0`\xA0\x82\x84\x03\x12\x15a3\xFFW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\"Wa4\"a*CV[\x80`@RP\x80\x91P\x82Qa45\x81a2 V[\x81R` \x83\x01Qa4E\x81a2 V[` \x82\x01R`@\x83\x01Qa4X\x81a2 V[`@\x82\x01R``\x83\x01Qa4k\x81a2 V[``\x82\x01R`\x80\x83\x01Qa4~\x81a2 V[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a4\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xC0Wa4\xC0a*CV[\x80`@RP\x80\x91P\x82Qa4\xD3\x81a2 V[\x81R` \x83\x01Qa4\xE3\x81a2 V[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a5\x02W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a5%Wa5%a*CV[\x80`@RP\x80\x91P\x82Qa58\x81a2 V[\x81R` \x83\x01Qa5H\x81a2 V[` \x82\x01R`@\x83\x01Qa5[\x81a2 V[`@\x82\x01R``\x83\x01Qa5n\x81a2 V[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5\x8DW`\0\x80\xFD[a5\x95a*\x82V[\x90P\x81Qa5\xA2\x81a2 V[\x81R` \x82\x01Qa5\xB2\x81a2 V[` \x82\x01R`@\x82\x01Qa5\xC5\x81a2 V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a5\xE7W`\0\x80\xFD[a5\xF1\x86\x86a3\xEDV[\x93Pa6\0\x86`\xA0\x87\x01a4\x8BV[\x92Pa6\x0F\x86`\xE0\x87\x01a4\xF0V[\x91Pa6\x1F\x86a\x01`\x87\x01a5{V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a6<W`\0\x80\xFD[a#\xAA\x83\x83a3\xEDV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a6\x8BWa6\x8Ba6\\V[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a6\xADW`\0\x80\xFD[`\xA0\x81\x12\x15a6\xBBW`\0\x80\xFD[Pa6\xC4a*\x82V[\x85Qa6\xCF\x81a2 V[\x81Ra6\xDE\x87` \x88\x01a4\x8BV[` \x82\x01Ra6\xF0\x87``\x88\x01a4\x8BV[`@\x82\x01R\x93Pa7\x04\x86`\xA0\x87\x01a3\x81V[\x92Pa7\x13\x86`\xC0\x87\x01a4\xF0V[\x91Pa6\x1F\x86a\x01@\x87\x01a4\x8BV[`\0`\xA0\x82\x84\x03\x12\x15a75W`\0\x80\xFD[a7=a*YV[\x82Qa7H\x81a2\xA9V[\x81R` \x83\x01Qa7X\x81a2 V[` \x82\x01R`@\x83\x01Qa7k\x81a2 V[`@\x82\x01R``\x83\x01Qa7~\x81a2 V[``\x82\x01R`\x80\x83\x01Qa2\x9D\x81a2 V[`\0`\x01\x82\x01a7\xA3Wa7\xA3a6\\V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a7\xBDWa7\xBDa6\\V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a8\x02Wa8\x02a6\\V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a8\x1EWa8\x1Ea6\\V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a8=Wa8=a6\\V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8XW`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a8\x82Wa8\x82a7\xC2V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a8\xA2W`\0\x80\xFD[a#\xAA\x83\x83a5{V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a8\xC9Wa8\xC9a6\\V[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a8\xFFWa8\xFFa8\xD2V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a9&Wa9&a6\\V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a9ZWa9Za6\\V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a9uWa9ua6\\V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a6\x8BWa6\x8Ba6\\V[`\0` \x82\x84\x03\x12\x15a9\xADW`\0\x80\xFD[\x81Qa#\xAA\x81a2\xA9V[` \x81\x01`\x02\x83\x10a9\xCCWa9\xCCa7\xC2V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a9\xE5W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xFCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a:\rW`\0\x80\xFD[\x80Qa:\x1Ba+\x1B\x82a*\xD6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a::W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a:aW\x83Qa:R\x81a(zV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a:?V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:\x99W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:}V[\x81\x81\x11\x15a:\xABW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a:\xE9Wa:\xE9a6\\V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a;\x08Wa;\x08a6\\V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a;$Wa;$a6\\V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a;:Wa;:a6\\V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a;xWa;xa6\\V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a;\xA4Wa;\xA4a6\\V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a;\xC0Wa;\xC0a6\\V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\xD6Wa;\xD6a6\\V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a;\xF5Wa;\xF5a8\xD2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a<\x0FWa<\x0Fa6\\V[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15a<5Wa<5a6\\V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a<NWa<Na6\\V[PP\x01\x90V\xFE\xA2dipfsX\"\x12 }\xEA\xE3/\xFE\x14\xBA\x9B-\xA4\xD5\xD1\xB6\xB5\xB6\x16\x88\xEC\xE1\x9B\x87\x89\xE3:\x02\xD0\x86\xB9\xC9P\xB4IdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80ci\xD3;\xE2\x11a\0\x97W\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\x8EW\x80c\xC4\xD6m\xE8\x14a\x02\xA9W\x80c\xD7\xB2)\xB6\x14a\x02\xBEW\x80c\xEE\x99(\xC9\x14a\x02\xDEW`\0\x80\xFD[\x80ci\xD3;\xE2\x14a\x01\xE3W\x80ct\x174\x04\x14a\x01\xEBW\x80cu\xA5\xAB<\x14a\x02\x0BW\x80c\x86\x99R\xFD\x14a\x02+W`\0\x80\xFD[\x80c1TmQ\x11a\0\xD3W\x80c1TmQ\x14a\x01cW\x80cI\xFE\x1B<\x14a\x01\x83W\x80cW#e?\x14a\x01\xA3W\x80c]p.\x1A\x14a\x01\xC3W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xFAW\x80c\x1A\xE1\x0B\xC5\x14a\x01#W\x80c%\x93\xEB_\x14a\x01CW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a(\x8FV[a\x02\xFEV[`@Qa\x01\x1A\x91\x90a(\xACV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a(\x8FV[a\x04oV[`@Qa\x01\x1A\x91\x90a*4V[a\x01Va\x01Q6`\x04a+nV[a\x06UV[`@Qa\x01\x1A\x91\x90a,\x17V[a\x01va\x01q6`\x04a+nV[a\x07+V[`@Qa\x01\x1A\x91\x90a,\xE7V[a\x01\x96a\x01\x916`\x04a(\x8FV[a\x07\xFAV[`@Qa\x01\x1A\x91\x90a/\rV[a\x01\xB6a\x01\xB16`\x04a(\x8FV[a\x08JV[`@Qa\x01\x1A\x91\x90a/OV[a\x01\xD6a\x01\xD16`\x04a/^V[a\nGV[`@Qa\x01\x1A\x91\x90a0\x8AV[a\x01\x96a\x1C\xADV[a\x01\xFEa\x01\xF96`\x04a1\x92V[a\x1C\xD0V[`@Qa\x01\x1A\x91\x90a1\xC2V[a\x02\x1Ea\x02\x196`\x04a1\xD0V[a\x1D\x82V[`@Qa\x01\x1A\x91\x90a2\rV[a\x01\ra\x0296`\x04a2/V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x02\xBCa\x02\xB76`\x04a2\xBEV[a\x1EVV[\0[a\x02\xD1a\x02\xCC6`\x04a1\x92V[a \\V[`@Qa\x01\x1A\x91\x90a2\xDBV[a\x02\xF1a\x02\xEC6`\x04a1\xD0V[a \xE3V[`@Qa\x01\x1A\x91\x90a2\xE9V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x031a!\xB1V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA1\x91\x90a2\xFCV[\x90P`\0a\x03\xADa!\xB1V[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1D\x91\x90a3\xD1V[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[a\x04wa&-V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF6\x91\x90a5\xD0V[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05s\x91\x90a6*V[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x05\xF2\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06AW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06JV[a\x06J\x87a\x02\xFEV[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06qWa\x06qa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xAAW\x81` \x01[a\x06\x97a&\xFBV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x8FW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07$W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xD9Wa\x06\xD9a6FV[` \x02` \x01\x01Q\x90Pa\x06\xED\x85\x82a \\V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x05Wa\x07\x05a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\x1C\x90a6rV[\x91PPa\x06\xB0V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07GWa\x07Ga*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x80W\x81` \x01[a\x07ma'YV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07eW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07$W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xAFWa\x07\xAFa6FV[` \x02` \x01\x01Q\x90Pa\x07\xC3\x85\x82a\x1C\xD0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xDBWa\x07\xDBa6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xF2\x90a6rV[\x91PPa\x07\x86V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x08\x1A\x84a\"\x1FV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x082\x84a\x1D\x82V[\x81R` \x01a\x08@\x83a \xE3V[\x90R\x94\x93PPPPV[a\x08Ra'\xA3V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a6\x95V[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tN\x91\x90a6*V[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\t\xCD\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF2\x91\x90a7#V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\n\xF3\x81a\"\x1FV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\x18WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\xB2W\x81\x84\x82\x81Q\x81\x10a\x0BiWa\x0Bia6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xA0W\x83\x81\x81Q\x81\x10a\x0B\x8FWa\x0B\x8Fa6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\xAA\x81a7\x91V[\x91PPa\x0BMV[P`\0[\x82Q\x81\x10\x15a\x0C\x1BW\x81\x83\x82\x81Q\x81\x10a\x0B\xD2Wa\x0B\xD2a6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\tW\x82\x81\x81Q\x81\x10a\x0B\xF8Wa\x0B\xF8a6FV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0C\x13\x81a7\x91V[\x91PPa\x0B\xB6V[Pa\x0C'\x81`\x01a7\xAAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C?Wa\x0C?a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CrW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C]W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C\xD5W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\xB7Wa\x0C\xB7a6FV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C\xCD\x90a7\x91V[\x91PPa\x0C{V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xF1Wa\x0C\xF1a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r*W\x81` \x01[a\r\x17a'YV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x0FW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rJWa\rJa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x83W\x81` \x01[a\rpa&\xFBV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rhW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA3Wa\r\xA3a*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xDCW\x81` \x01[a\r\xC9a'\xA3V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xC1W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xFDWa\r\xFDa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E6W\x81` \x01[a\x0E#a&-V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x1BW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x12iW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0EiWa\x0Eia6FV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90a6\x95V[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fx\x91\x90a6*V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xC4Wa\x0F\xC4a6FV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10H\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xC2\x91\x90a7#V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x10\xDC\x89a\x02\xFEV[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x10\xF4\x82a6rV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\x19Wa\x11\x19a6FV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12PW\x83Q`\0\x90a\x11h\x90\x84\x90a\x11_\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[a#\x18V[`\x0F\x0B\x90a#\xB1V[\x87Q\x90\x91P`\x0F\x0B\x15a\x11\xEBW\x87Q\x87Q`\0\x91a\x11\x89\x91`\x0F\x0B\x90a$4V[\x90Pa\x11\xDDa\x11\xA9\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[`@\x8B\x01QQa\x11_\x90a\x11\xC0\x90`\x0F\x0B\x85a#\xB1V[` \x8D\x01QQa\x11\xD3\x90`\x0F\x0B\x86a#\xB1V[\x89`\x80\x01Qa$\x9DV[a\x11\xE7\x90\x83a7\xD8V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x08Wa\x12\x08a6FV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12$Wa\x12$a6FV[` \x02` \x01\x01\x81\x81Qa\x128\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x12I\x90P\x81a8'V[\x90Pa\x11'V[PPPPPPPP\x80a\x12b\x90a6rV[\x90Pa\x0E@V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16PW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x96Wa\x12\x96a6FV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13%\x91\x90a5\xD0V[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x81W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA5\x91\x90a6*V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xF1Wa\x13\xF1a6FV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x14u\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x14\x8F\x89a\x02\xFEV[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x14\xA7\x82a6rV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\xCCWa\x14\xCCa6FV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x167W`\0\x84` \x01Qa\x15\x17\x84a\x11_\x88`\0\x01Qa\x11_\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[a\x15!\x91\x90a7\xD8V[\x87Q\x90\x91P`\x0F\x0B\x15a\x15\xD2W\x87Q\x87Q`\0\x91a\x15B\x91`\x0F\x0B\x90a$4V[\x90Pa\x15j\x88`\0\x01Qa\x11_\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba$\xD9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\xBAa\x15\x88\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x11ZWa\x11Za7\xC2V[``\x8C\x01Qa\x11_\x90a\x15\x9E\x90`\x0F\x0B\x86a#\xB1V[`\x80\x8E\x01Qa\x15\xB0\x90`\x0F\x0B\x87a#\xB1V[\x8A`\x80\x01Qa$\x9DV[a\x15\xC4\x91\x90a7\xD8V[a\x15\xCE\x90\x83a7\xD8V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xEFWa\x15\xEFa6FV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x16\x0BWa\x16\x0Ba6FV[` \x02` \x01\x01\x81\x81Qa\x16\x1F\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x160\x90P\x81a8'V[\x90Pa\x14\xDAV[PPPPPPPP\x80a\x16I\x90a6rV[\x90Pa\x12mV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC9\x91\x90a8FV[\x90P[\x80\x15a\x1AUW`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1AMW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x17\x1CWa\x17\x1Ca7\xC2V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17:\x93\x92\x91\x90a8_V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17{\x91\x90a8\x90V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x17\x90WPa\x1A;V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x17\xC1Wa\x17\xC1a7\xC2V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDF\x93\x92\x91\x90a8_V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18 \x91\x90a8\x90V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18CWP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18OWPPa\x1A;V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x18~W\x81Q\x83Qa\x18w\x91\x90a\x18r\x90a8\xACV[a%BV[\x90Pa\x18\xA1V[\x81Q\x83Qa\x18\x95\x91\x90a\x18\x90\x90a8\xACV[a%^V[a\x18\x9E\x90a8\xACV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x18\xB9\x91\x90a7\xD8V[a\x18\xC3\x91\x90a8\xE8V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x19\x13W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xF0\x91\x90a9/V[a\x18\xFA\x91\x90a8\xE8V[a\x19\x0C\x90g\r\xE0\xB6\xB3\xA7d\0\0a9/V[\x90Pa\x19LV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19-\x91\x90a9/V[a\x197\x91\x90a8\xE8V[a\x19I\x90g\r\xE0\xB6\xB3\xA7d\0\0a9/V[\x90P[`\0`\x02a\x19\x7Fa\x19]\x85\x85a9/V[a\x11_\x89` \x01Q\x89` \x01Qa\x19t\x91\x90a7\xD8V[`\x0F\x89\x90\x0B\x90a#\xB1V[a\x19\x89\x91\x90a8\xE8V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\xA8Wa\x19\xA8a6FV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xC4Wa\x19\xC4a6FV[` \x02` \x01\x01\x81\x81Qa\x19\xD8\x91\x90a7\xD8V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x19\xFEWa\x19\xFEa6FV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1A\x1AWa\x1A\x1Aa6FV[` \x02` \x01\x01\x81\x81Qa\x1A.\x91\x90a7\xD8V[`\x0F\x0B\x90RPPPPPPP[\x80a\x1AE\x81a8'V[\x91PPa\x16\xE5V[PPPa\x16\xCCV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\xA2W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1B\xF9W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xA4Wa\x1A\xA4a6FV[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xC6Wa\x1A\xC6a6FV[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1BbW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xF6Wa\x1A\xF6a6FV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x18Wa\x1B\x18a6FV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B?Wa\x1B?a6FV[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1BW\x91\x90a7\xD8V[`\x0F\x0B\x90RPa\x1B\xE9V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x81Wa\x1B\x81a6FV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA3Wa\x1B\xA3a6FV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xCAWa\x1B\xCAa6FV[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1B\xE2\x91\x90a9/V[`\x0F\x0B\x90RP[a\x1B\xF2\x81a9\x7FV[\x90Pa\x1AmV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\x19Wa\x1C\x19a6FV[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CDWa\x1CDa6FV[` \x02` \x01\x01Q`\0\x01Qa\x1CZ\x91\x90a9/V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CyWa\x1Cya6FV[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1C\x9B\x90a9\x7FV[\x90Pa\x1AXV[P\x92\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x1C\xCB`\0a\x07\xFAV[\x90P\x90V[a\x1C\xD8a'YV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DW\x91\x90a6\x95V[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9EWa\x1D\x9Ea*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xD7W\x81` \x01[a\x1D\xC4a'\xA3V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xBCW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EPW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E\x06Wa\x1E\x06a6FV[` \x02` \x01\x01Q\x90Pa\x1E\x19\x81a\x08JV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1E1Wa\x1E1a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1EH\x90a6rV[\x91PPa\x1D\xDDV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1E\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F1\x91\x90a9\x9BV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1Fs\x91`\x04\x01a9\xB8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xB4\x91\x90a9\x9BV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F\xF8\x90`\x01\x90`\x04\x01a9\xB8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 9\x91\x90a9\x9BV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a da&\xFBV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DW\x91\x90a5\xD0V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFFWa \xFFa*CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!8W\x81` \x01[a!%a&-V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a!\x1DW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1EPW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!gWa!ga6FV[` \x02` \x01\x01Q\x90Pa!z\x81a\x04oV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\x92Wa!\x92a6FV[` \x02` \x01\x01\x81\x90RPP\x80\x80a!\xA9\x90a6rV[\x91PPa!>V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xCB\x91\x90a9\x9BV[`\x02T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R``\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x9A\x91\x90\x81\x01\x90a9\xD2V[`\x03T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x11\x91\x90\x81\x01\x90a9\xD2V[\x90P\x91P\x91V[`\0`\x02\x82`\x02\x81\x11\x15a#.Wa#.a7\xC2V[\x03a#BWPg\r\xE0\xB6\xB3\xA7d\0\0a#\xAAV[`\0\x80\x84`\x0F\x0B\x12a#{W`\0\x83`\x02\x81\x11\x15a#bWa#ba7\xC2V[\x14a#qW\x84`@\x01Qa#tV[\x84Q[\x90Pa#\xA7V[`\0\x83`\x02\x81\x11\x15a#\x8FWa#\x8Fa7\xC2V[\x14a#\x9EW\x84``\x01Qa#\xA4V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xF3WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a$xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a#\xC8Wa#\xC8a8\xD2V[`\0a$\xCE\x83`\x0F\x0Ba$\xBC\x84\x87`\x0F\x0Ba#\xB1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba$\xC9\x91\x90a:\xC1V[a%sV[a#\xA7\x90`\x02a;HV[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xF3WP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E\xAB\x91\x90a:lV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a%WW\x81a#\xAAV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a%WW\x81a#\xAAV[`\0\x80\x82\x12\x15a%\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1E\xABV[`\x03\x82\x13\x15a&\x1EWP\x80`\0a%\xDD`\x02\x83a;\xE6V[a%\xE8\x90`\x01a<\x14V[\x90P[\x81\x81\x12\x15a\x1EPW\x90P\x80`\x02\x81a&\x03\x81\x86a;\xE6V[a&\r\x91\x90a<\x14V[a&\x17\x91\x90a;\xE6V[\x90Pa%\xEBV[\x81\x15a&(WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a'6`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a&\xF6`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a&\xC5`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a'{`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(\x8CW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(\xA1W`\0\x80\xFD[\x815a#\xAA\x81a(zV[`\xA0\x81\x01a\x1D|\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa)W`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa)\x97`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa)\xE5a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[PPPV[a\x02\xA0\x81\x01a\x1D|\x82\x84a(\xF4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a*CV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a*CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\xCEWa*\xCEa*CV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*\xF0Wa*\xF0a*CV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a+\x0BW`\0\x80\xFD[\x815` a+ a+\x1B\x83a*\xD6V[a*\xA5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a+?W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a+cW\x805a+V\x81a(zV[\x83R\x91\x83\x01\x91\x83\x01a+CV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x81W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\x9FW`\0\x80\xFD[a+\xAB\x85\x82\x86\x01a*\xFAV[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa+\xE4` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa*/``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a,YWa,F\x83\x85Qa+\xB5V[\x92\x84\x01\x92`\xC0\x92\x90\x92\x01\x91`\x01\x01a,3V[P\x90\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa*/`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa,\xC9\x87\x83Qa,eV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a,\xB6V[P\x94\x95\x94PPPPPV[` \x81R`\0a#\xAA` \x83\x01\x84a,\xA2V[\x80Q`\x0F\x0B\x82R` \x81\x01Qa-&` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa-\xAA`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa.9a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa.Ma\x02\0\x84\x01\x82a,\xFAV[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa.\xBE\x87\x83Qa-GV[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xABV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa.\xF9\x87\x83Qa(\xF4V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xE6V[` \x81R`\0\x82Q`@` \x84\x01Ra/)``\x84\x01\x82a.\x97V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra/F\x82\x82a.\xD2V[\x95\x94PPPPPV[a\x03@\x81\x01a\x1D|\x82\x84a-GV[`\0` \x82\x84\x03\x12\x15a/pW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa/\xBE\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a/\x8BV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a0BW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a0-W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a0\x0EV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a/\xF0V[P\x92\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a,\xDCWa0w\x87\x83Qa+\xB5V[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a0dV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa0\xAA`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra0\xC7a\x01`\x85\x01\x83a/wV[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra0\xE5\x84\x83a/\xD1V[\x93P`\x80\x87\x01Q\x91Pa1\0`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra1.\x84\x83a,\xA2V[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra1M\x85\x84a0PV[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra1l\x85\x84a.\x97V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa1\x88\x83\x82a.\xD2V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xA5W`\0\x80\xFD[\x825\x91P` \x83\x015a1\xB7\x81a(zV[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\x1D|\x82\x84a,eV[`\0` \x82\x84\x03\x12\x15a1\xE2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xF9W`\0\x80\xFD[a2\x05\x84\x82\x85\x01a*\xFAV[\x94\x93PPPPV[` \x81R`\0a#\xAA` \x83\x01\x84a.\x97V[\x80`\x0F\x0B\x81\x14a(\x8CW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a2AW`\0\x80\xFD[a2Ia*YV[\x825a2T\x81a2 V[\x81R` \x83\x015a2d\x81a2 V[` \x82\x01R`@\x83\x015a2w\x81a2 V[`@\x82\x01R``\x83\x015a2\x8A\x81a2 V[``\x82\x01R`\x80\x83\x015a2\x9D\x81a2 V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\x8CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a2\xD0W`\0\x80\xFD[\x815a#\xAA\x81a2\xA9V[`\xC0\x81\x01a\x1D|\x82\x84a+\xB5V[` \x81R`\0a#\xAA` \x83\x01\x84a.\xD2V[`\0`\x80\x82\x84\x03\x12\x15a3\x0EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a31Wa31a*CV[`@R\x82Qa3?\x81a(zV[\x81R` \x83\x01Qa3O\x81a2 V[` \x82\x01R`@\x83\x01Qa3b\x81a2 V[`@\x82\x01R``\x83\x01Qa3u\x81a2 V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\x93W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\xB6Wa3\xB6a*CV[\x80`@RP\x80\x91P\x82Qa3\xC9\x81a2 V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xE3W`\0\x80\xFD[a#\xAA\x83\x83a3\x81V[`\0`\xA0\x82\x84\x03\x12\x15a3\xFFW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\"Wa4\"a*CV[\x80`@RP\x80\x91P\x82Qa45\x81a2 V[\x81R` \x83\x01Qa4E\x81a2 V[` \x82\x01R`@\x83\x01Qa4X\x81a2 V[`@\x82\x01R``\x83\x01Qa4k\x81a2 V[``\x82\x01R`\x80\x83\x01Qa4~\x81a2 V[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a4\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xC0Wa4\xC0a*CV[\x80`@RP\x80\x91P\x82Qa4\xD3\x81a2 V[\x81R` \x83\x01Qa4\xE3\x81a2 V[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a5\x02W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a5%Wa5%a*CV[\x80`@RP\x80\x91P\x82Qa58\x81a2 V[\x81R` \x83\x01Qa5H\x81a2 V[` \x82\x01R`@\x83\x01Qa5[\x81a2 V[`@\x82\x01R``\x83\x01Qa5n\x81a2 V[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5\x8DW`\0\x80\xFD[a5\x95a*\x82V[\x90P\x81Qa5\xA2\x81a2 V[\x81R` \x82\x01Qa5\xB2\x81a2 V[` \x82\x01R`@\x82\x01Qa5\xC5\x81a2 V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a5\xE7W`\0\x80\xFD[a5\xF1\x86\x86a3\xEDV[\x93Pa6\0\x86`\xA0\x87\x01a4\x8BV[\x92Pa6\x0F\x86`\xE0\x87\x01a4\xF0V[\x91Pa6\x1F\x86a\x01`\x87\x01a5{V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a6<W`\0\x80\xFD[a#\xAA\x83\x83a3\xEDV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a6\x8BWa6\x8Ba6\\V[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a6\xADW`\0\x80\xFD[`\xA0\x81\x12\x15a6\xBBW`\0\x80\xFD[Pa6\xC4a*\x82V[\x85Qa6\xCF\x81a2 V[\x81Ra6\xDE\x87` \x88\x01a4\x8BV[` \x82\x01Ra6\xF0\x87``\x88\x01a4\x8BV[`@\x82\x01R\x93Pa7\x04\x86`\xA0\x87\x01a3\x81V[\x92Pa7\x13\x86`\xC0\x87\x01a4\xF0V[\x91Pa6\x1F\x86a\x01@\x87\x01a4\x8BV[`\0`\xA0\x82\x84\x03\x12\x15a75W`\0\x80\xFD[a7=a*YV[\x82Qa7H\x81a2\xA9V[\x81R` \x83\x01Qa7X\x81a2 V[` \x82\x01R`@\x83\x01Qa7k\x81a2 V[`@\x82\x01R``\x83\x01Qa7~\x81a2 V[``\x82\x01R`\x80\x83\x01Qa2\x9D\x81a2 V[`\0`\x01\x82\x01a7\xA3Wa7\xA3a6\\V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a7\xBDWa7\xBDa6\\V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a8\x02Wa8\x02a6\\V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a8\x1EWa8\x1Ea6\\V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a8=Wa8=a6\\V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8XW`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a8\x82Wa8\x82a7\xC2V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a8\xA2W`\0\x80\xFD[a#\xAA\x83\x83a5{V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a8\xC9Wa8\xC9a6\\V[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a8\xFFWa8\xFFa8\xD2V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a9&Wa9&a6\\V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a9ZWa9Za6\\V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a9uWa9ua6\\V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a6\x8BWa6\x8Ba6\\V[`\0` \x82\x84\x03\x12\x15a9\xADW`\0\x80\xFD[\x81Qa#\xAA\x81a2\xA9V[` \x81\x01`\x02\x83\x10a9\xCCWa9\xCCa7\xC2V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a9\xE5W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xFCW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a:\rW`\0\x80\xFD[\x80Qa:\x1Ba+\x1B\x82a*\xD6V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a::W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a:aW\x83Qa:R\x81a(zV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a:?V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:\x99W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:}V[\x81\x81\x11\x15a:\xABW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a:\xE9Wa:\xE9a6\\V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a;\x08Wa;\x08a6\\V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a;$Wa;$a6\\V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a;:Wa;:a6\\V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a;xWa;xa6\\V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a;\xA4Wa;\xA4a6\\V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a;\xC0Wa;\xC0a6\\V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\xD6Wa;\xD6a6\\V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a;\xF5Wa;\xF5a8\xD2V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a<\x0FWa<\x0Fa6\\V[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15a<5Wa<5a6\\V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a<NWa<Na6\\V[PP\x01\x90V\xFE\xA2dipfsX\"\x12 }\xEA\xE3/\xFE\x14\xBA\x9B-\xA4\xD5\xD1\xB6\xB5\xB6\x16\x88\xEC\xE1\x9B\x87\x89\xE3:\x02\xD0\x86\xB9\xC9P\xB4IdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static QUERIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
                QUERIER_ABI.clone(),
                QUERIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getAllProducts` (0x49fe1b3c) function
        pub fn get_all_products(
            &self,
            iso_group: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([73, 254, 27, 60], iso_group)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBookInfo` (0x01cfa9d1) function
        pub fn get_book_info(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, BookInfo> {
            self.0
                .method_hash([1, 207, 169, 209], product_id)
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
        ///Calls the contract's `getCrossProducts` (0x69d33be2) function
        pub fn get_cross_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([105, 211, 59, 226], ())
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
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], clearinghouse)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `legacyRisk` (0x869952fd) function
        pub fn legacy_risk(
            &self,
            r: Risk,
        ) -> ::ethers::contract::builders::ContractCall<M, LegacyRisk> {
            self.0
                .method_hash([134, 153, 82, 253], (r,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Querier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getAllProducts` function with signature `getAllProducts(uint32)` and selector `0x49fe1b3c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAllProducts", abi = "getAllProducts(uint32)")]
    pub struct GetAllProductsCall {
        pub iso_group: u32,
    }
    ///Container type for all input parameters for the `getBookInfo` function with signature `getBookInfo(uint32)` and selector `0x01cfa9d1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBookInfo", abi = "getBookInfo(uint32)")]
    pub struct GetBookInfoCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all input parameters for the `getCrossProducts` function with signature `getCrossProducts()` and selector `0x69d33be2`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCrossProducts", abi = "getCrossProducts()")]
    pub struct GetCrossProductsCall;
    ///Container type for all input parameters for the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `0x2593eb5f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `0x1ae10bc5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `0x31546d51`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `0x5723653f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all input parameters for the `legacyRisk` function with signature `legacyRisk((int128,int128,int128,int128,int128))` and selector `0x869952fd`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        name = "legacyRisk",
        abi = "legacyRisk((int128,int128,int128,int128,int128))"
    )]
    pub struct LegacyRiskCall {
        pub r: Risk,
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
    pub enum QuerierCalls {
        GetAllProducts(GetAllProductsCall),
        GetBookInfo(GetBookInfoCall),
        GetClearinghouse(GetClearinghouseCall),
        GetCrossProducts(GetCrossProductsCall),
        GetPerpBalance(GetPerpBalanceCall),
        GetPerpBalances(GetPerpBalancesCall),
        GetPerpProduct(GetPerpProductCall),
        GetPerpProducts(GetPerpProductsCall),
        GetSpotBalance(GetSpotBalanceCall),
        GetSpotBalances(GetSpotBalancesCall),
        GetSpotProduct(GetSpotProductCall),
        GetSpotProducts(GetSpotProductsCall),
        GetSubaccountInfo(GetSubaccountInfoCall),
        Initialize(InitializeCall),
        LegacyRisk(LegacyRiskCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuerierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
                <GetCrossProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCrossProducts(decoded));
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
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LegacyRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LegacyRisk(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QuerierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAllProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBookInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCrossProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LegacyRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for QuerierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAllProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBookInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCrossProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LegacyRisk(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    impl ::core::convert::From<GetCrossProductsCall> for QuerierCalls {
        fn from(value: GetCrossProductsCall) -> Self {
            Self::GetCrossProducts(value)
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
    impl ::core::convert::From<InitializeCall> for QuerierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LegacyRiskCall> for QuerierCalls {
        fn from(value: LegacyRiskCall) -> Self {
            Self::LegacyRisk(value)
        }
    }
    ///Container type for all return fields from the `getAllProducts` function with signature `getAllProducts(uint32)` and selector `0x49fe1b3c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getBookInfo` function with signature `getBookInfo(uint32)` and selector `0x01cfa9d1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getCrossProducts` function with signature `getCrossProducts()` and selector `0x69d33be2`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCrossProductsReturn(pub ProductInfo);
    ///Container type for all return fields from the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `legacyRisk` function with signature `legacyRisk((int128,int128,int128,int128,int128))` and selector `0x869952fd`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LegacyRiskReturn {
        pub l: LegacyRisk,
    }
    ///`BookInfo(int128,int128,int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_increment_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_fees: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub lp_spread_x18: i128,
    }
    ///`HealthInfo(int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub assets: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub liabilities: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub health: i128,
    }
    ///`LegacyRisk(int128,int128,int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
    pub struct LegacyRisk {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub large_position_penalty_x18: i128,
    }
    ///`PerpBalance(uint32,(int128,int128),(int128,int128,int128))`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
        pub lp_balance: crate::bindings::perp_engine::LpBalance,
        pub balance: crate::bindings::perp_engine::Balance,
    }
    ///`PerpProduct(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: LegacyRisk,
        pub state: crate::bindings::perp_engine::State,
        pub lp_state: crate::bindings::perp_engine::LpState,
        pub book_info: BookInfo,
    }
    ///`ProductInfo((uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
        pub lp_balance: crate::bindings::spot_engine::LpBalance,
        pub balance: crate::bindings::spot_engine::Balance,
    }
    ///`SpotProduct(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: LegacyRisk,
        pub config: Config,
        pub state: crate::bindings::spot_engine::State,
        pub lp_state: crate::bindings::spot_engine::LpState,
        pub book_info: BookInfo,
    }
    ///`SubaccountInfo(bytes32,bool,(int128,int128,int128)[],int128[][],uint32,uint32,(uint32,(int128),(int128,int128))[],(uint32,(int128,int128),(int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,(int128,int128),(int128,int128)),(int128,int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128,int128),(int128,int128,int128,int128,int128))[])`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub exists: bool,
        pub healths: ::std::vec::Vec<HealthInfo>,
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
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    ///`IperpEngineLpBalance(int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    ///`IperpEngineLpState(int128,int128,int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub supply: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_per_lp_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub base: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote: i128,
    }
    ///`IperpEngineState(int128,int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub open_interest: i128,
    }
    ///`IspotEngineBalance(int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_multiplier_x18: i128,
    }
    ///`Config(address,int128,int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_inflection_util_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_floor_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_small_cap_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_large_cap_x18: i128,
    }
    ///`IspotEngineLpBalance(int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
    }
    ///`IspotEngineLpState(int128,(int128,int128),(int128,int128))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub supply: i128,
        pub quote: crate::bindings::spot_engine::Balance,
        pub base: crate::bindings::spot_engine::Balance,
    }
    ///`IspotEngineState(int128,int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_deposits_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_borrows_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub total_deposits_normalized: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub total_borrows_normalized: i128,
    }
    ///`Risk(int128,int128,int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
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
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
}
