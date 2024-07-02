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
                    ::std::borrow::ToOwned::to_owned("getContractVersions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getContractVersions",
                            ),
                            inputs: ::std::vec![],
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaAv\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x1BW`\x005`\xE0\x1C\x80ci\xD3;\xE2\x11a\0\xB2W\x80c\x86\x99R\xFD\x11a\0\x81W\x80c\xC4\xD6m\xE8\x11a\0fW\x80c\xC4\xD6m\xE8\x14a\x03TW\x80c\xD7\xB2)\xB6\x14a\x03iW\x80c\xEE\x99(\xC9\x14a\x03\x89W`\0\x80\xFD[\x80c\x86\x99R\xFD\x14a\x02\xD6W\x80c\xB1\xCB\x0FB\x14a\x039W`\0\x80\xFD[\x80ci\xD3;\xE2\x14a\x02\x18W\x80cq$Z\xB1\x14a\x02 W\x80ct\x174\x04\x14a\x02\x96W\x80cu\xA5\xAB<\x14a\x02\xB6W`\0\x80\xFD[\x80c1TmQ\x11a\0\xEEW\x80c1TmQ\x14a\x01\x98W\x80cI\xFE\x1B<\x14a\x01\xB8W\x80cW#e?\x14a\x01\xD8W\x80c]p.\x1A\x14a\x01\xF8W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\x01 W\x80c\r\x8En,\x14a\x01IW\x80c\x1A\xE1\x0B\xC5\x14a\x01XW\x80c%\x93\xEB_\x14a\x01xW[`\0\x80\xFD[a\x013a\x01.6`\x04a-NV[a\x03\xA9V[`@Qa\x01@\x91\x90a-kV[`@Q\x80\x91\x03\x90\xF3[`@Q`\x1B\x81R` \x01a\x01@V[a\x01ka\x01f6`\x04a-NV[a\x05\x1AV[`@Qa\x01@\x91\x90a.\xF3V[a\x01\x8Ba\x01\x866`\x04a0-V[a\x07\0V[`@Qa\x01@\x91\x90a0\xD6V[a\x01\xABa\x01\xA66`\x04a0-V[a\x07\xD6V[`@Qa\x01@\x91\x90a1\xA6V[a\x01\xCBa\x01\xC66`\x04a-NV[a\x08\xA5V[`@Qa\x01@\x91\x90a3\xCCV[a\x01\xEBa\x01\xE66`\x04a-NV[a\x08\xF5V[`@Qa\x01@\x91\x90a4\x0EV[a\x02\x0Ba\x02\x066`\x04a4\x1DV[a\n\xF2V[`@Qa\x01@\x91\x90a5LV[a\x01\xCBa\x1DXV[a\x02(a\x1D{V[`@Qa\x01@\x91\x90`\0`\xE0\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01R\x80`\xC0\x85\x01Q\x16`\xC0\x84\x01RP\x92\x91PPV[a\x02\xA9a\x02\xA46`\x04a6TV[a!\x8FV[`@Qa\x01@\x91\x90a6\x84V[a\x02\xC9a\x02\xC46`\x04a6\x92V[a\"AV[`@Qa\x01@\x91\x90a6\xCFV[a\x013a\x02\xE46`\x04a6\xF1V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01@V[a\x03ga\x03b6`\x04a7\x80V[a#\x15V[\0[a\x03|a\x03w6`\x04a6TV[a%\x1BV[`@Qa\x01@\x91\x90a7\x9DV[a\x03\x9Ca\x03\x976`\x04a6\x92V[a%\xA2V[`@Qa\x01@\x91\x90a7\xABV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x03\xDCa&pV[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04L\x91\x90a7\xBEV[\x90P`\0a\x04Xa&pV[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC8\x91\x90a8\x93V[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[a\x05\"a*\xECV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA1\x91\x90a:\x92V[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1E\x91\x90a:\xECV[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x06\x9D\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06\xECW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06\xF5V[a\x06\xF5\x87a\x03\xA9V[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x1CWa\x07\x1Ca/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07UW\x81` \x01[a\x07Ba+\xBAV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07:W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07\xCFW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x84Wa\x07\x84a;\x08V[` \x02` \x01\x01Q\x90Pa\x07\x98\x85\x82a%\x1BV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xB0Wa\x07\xB0a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xC7\x90a;4V[\x91PPa\x07[V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF2Wa\x07\xF2a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08+W\x81` \x01[a\x08\x18a,\x18V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\x10W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07\xCFW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08ZWa\x08Za;\x08V[` \x02` \x01\x01Q\x90Pa\x08n\x85\x82a!\x8FV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08\x86Wa\x08\x86a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x08\x9D\x90a;4V[\x91PPa\x081V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x08\xC5\x84a&\xDEV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x08\xDD\x84a\"AV[\x81R` \x01a\x08\xEB\x83a%\xA2V[\x90R\x94\x93PPPPV[a\x08\xFDa,bV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t|\x91\x90a;WV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF9\x91\x90a:\xECV[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\nx\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9D\x91\x90a;\xE5V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\x0B\x9E\x81a&\xDEV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xC3WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0C]W\x81\x84\x82\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0CKW\x83\x81\x81Q\x81\x10a\x0C:Wa\x0C:a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0CU\x81a<SV[\x91PPa\x0B\xF8V[P`\0[\x82Q\x81\x10\x15a\x0C\xC6W\x81\x83\x82\x81Q\x81\x10a\x0C}Wa\x0C}a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xB4W\x82\x81\x81Q\x81\x10a\x0C\xA3Wa\x0C\xA3a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0C\xBE\x81a<SV[\x91PPa\x0CaV[Pa\x0C\xD2\x81`\x01a<lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xEAWa\x0C\xEAa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x1DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x08W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\r\x80W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\rbWa\rba;\x08V[` \x02` \x01\x01\x81\x90RP\x80\x80a\rx\x90a<SV[\x91PPa\r&V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9CWa\r\x9Ca/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xD5W\x81` \x01[a\r\xC2a,\x18V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xBAW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xF5Wa\r\xF5a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E.W\x81` \x01[a\x0E\x1Ba+\xBAV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x13W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ENWa\x0ENa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x87W\x81` \x01[a\x0Eta,bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0ElW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xA8Wa\x0E\xA8a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xE1W\x81` \x01[a\x0E\xCEa*\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\xC6W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\x14W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\x14Wa\x0F\x14a;\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA3\x91\x90a;WV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10#\x91\x90a:\xECV[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10oWa\x10oa;\x08V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10\xF3\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11m\x91\x90a;\xE5V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x11\x87\x89a\x03\xA9V[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x11\x9F\x82a;4V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xC4Wa\x11\xC4a;\x08V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12\xFBW\x83Q`\0\x90a\x12\x13\x90\x84\x90a\x12\n\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[a'\xD7V[`\x0F\x0B\x90a(pV[\x87Q\x90\x91P`\x0F\x0B\x15a\x12\x96W\x87Q\x87Q`\0\x91a\x124\x91`\x0F\x0B\x90a(\xF3V[\x90Pa\x12\x88a\x12T\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[`@\x8B\x01QQa\x12\n\x90a\x12k\x90`\x0F\x0B\x85a(pV[` \x8D\x01QQa\x12~\x90`\x0F\x0B\x86a(pV[\x89`\x80\x01Qa)\\V[a\x12\x92\x90\x83a<\x9AV[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xB3Wa\x12\xB3a;\x08V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\xCFWa\x12\xCFa;\x08V[` \x02` \x01\x01\x81\x81Qa\x12\xE3\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x12\xF4\x90P\x81a<\xE9V[\x90Pa\x11\xD2V[PPPPPPPP\x80a\x13\r\x90a;4V[\x90Pa\x0E\xEBV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16\xFBW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13AWa\x13Aa;\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD0\x91\x90a:\x92V[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14P\x91\x90a:\xECV[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\x9CWa\x14\x9Ca;\x08V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x15 \x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x15:\x89a\x03\xA9V[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x15R\x82a;4V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15wWa\x15wa;\x08V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x16\xE2W`\0\x84` \x01Qa\x15\xC2\x84a\x12\n\x88`\0\x01Qa\x12\n\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[a\x15\xCC\x91\x90a<\x9AV[\x87Q\x90\x91P`\x0F\x0B\x15a\x16}W\x87Q\x87Q`\0\x91a\x15\xED\x91`\x0F\x0B\x90a(\xF3V[\x90Pa\x16\x15\x88`\0\x01Qa\x12\n\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba)\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16ea\x163\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[``\x8C\x01Qa\x12\n\x90a\x16I\x90`\x0F\x0B\x86a(pV[`\x80\x8E\x01Qa\x16[\x90`\x0F\x0B\x87a(pV[\x8A`\x80\x01Qa)\\V[a\x16o\x91\x90a<\x9AV[a\x16y\x90\x83a<\x9AV[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\x9AWa\x16\x9Aa;\x08V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x16\xB6Wa\x16\xB6a;\x08V[` \x02` \x01\x01\x81\x81Qa\x16\xCA\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x16\xDB\x90P\x81a<\xE9V[\x90Pa\x15\x85V[PPPPPPPP\x80a\x16\xF4\x90a;4V[\x90Pa\x13\x18V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17t\x91\x90a=\x08V[\x90P[\x80\x15a\x1B\0W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1A\xF8W`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x17\xC7Wa\x17\xC7a<\x84V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xE5\x93\x92\x91\x90a=!V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18&\x91\x90a=RV[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x18;WPa\x1A\xE6V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x18lWa\x18la<\x84V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x8A\x93\x92\x91\x90a=!V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCB\x91\x90a=RV[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18\xEEWP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18\xFAWPPa\x1A\xE6V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x19)W\x81Q\x83Qa\x19\"\x91\x90a\x19\x1D\x90a=nV[a*\x01V[\x90Pa\x19LV[\x81Q\x83Qa\x19@\x91\x90a\x19;\x90a=nV[a*\x1DV[a\x19I\x90a=nV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x19d\x91\x90a<\x9AV[a\x19n\x91\x90a=\xAAV[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x19\xBEW`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x9B\x91\x90a=\xF1V[a\x19\xA5\x91\x90a=\xAAV[a\x19\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\xF1V[\x90Pa\x19\xF7V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xD8\x91\x90a=\xF1V[a\x19\xE2\x91\x90a=\xAAV[a\x19\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\xF1V[\x90P[`\0`\x02a\x1A*a\x1A\x08\x85\x85a=\xF1V[a\x12\n\x89` \x01Q\x89` \x01Qa\x1A\x1F\x91\x90a<\x9AV[`\x0F\x89\x90\x0B\x90a(pV[a\x1A4\x91\x90a=\xAAV[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1ASWa\x1ASa;\x08V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1AoWa\x1Aoa;\x08V[` \x02` \x01\x01\x81\x81Qa\x1A\x83\x91\x90a<\x9AV[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x1A\xA9Wa\x1A\xA9a;\x08V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1A\xC5Wa\x1A\xC5a;\x08V[` \x02` \x01\x01\x81\x81Qa\x1A\xD9\x91\x90a<\x9AV[`\x0F\x0B\x90RPPPPPPP[\x80a\x1A\xF0\x81a<\xE9V[\x91PPa\x17\x90V[PPPa\x17wV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1DMW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\xA4W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1BOWa\x1BOa;\x08V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1BqWa\x1Bqa;\x08V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1C\rW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA1Wa\x1B\xA1a;\x08V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xC3Wa\x1B\xC3a;\x08V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xEAWa\x1B\xEAa;\x08V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1C\x02\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x1C\x94V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C,Wa\x1C,a;\x08V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CNWa\x1CNa;\x08V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CuWa\x1Cua;\x08V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1C\x8D\x91\x90a=\xF1V[`\x0F\x0B\x90RP[a\x1C\x9D\x81a>AV[\x90Pa\x1B\x18V[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\xC4Wa\x1C\xC4a;\x08V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\xEFWa\x1C\xEFa;\x08V[` \x02` \x01\x01Q`\0\x01Qa\x1D\x05\x91\x90a=\xF1V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1D$Wa\x1D$a;\x08V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1DF\x90a>AV[\x90Pa\x1B\x03V[P\x92\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x1Dv`\0a\x08\xA5V[\x90P\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`@Q\x80`\xE0\x01`@R\x80`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E6\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBA\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9B\x08a\xC1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F>\x91\x90a>\x87V[`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x9F\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x02T`@\x80Qc\x03c\x9B\x8B`\xE2\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\r\x8En,\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x1A\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x03T`@\x80Qc\x03c\x9B\x8B`\xE2\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\r\x8En,\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x95\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x06\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a!\x1Da&pV[`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!~\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90R\x91\x90PV[a!\x97a,\x18V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x16\x91\x90a;WV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"]Wa\"]a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x96W\x81` \x01[a\"\x83a,bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"{W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\x0FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\"\xC5Wa\"\xC5a;\x08V[` \x02` \x01\x01Q\x90Pa\"\xD8\x81a\x08\xF5V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\"\xF0Wa\"\xF0a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a#\x07\x90a;4V[\x91PPa\"\x9CV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a#sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xF0\x91\x90a>\x87V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a$2\x91`\x04\x01a>\xA4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$s\x91\x90a>\x87V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a$\xB7\x90`\x01\x90`\x04\x01a>\xA4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xF8\x91\x90a>\x87V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a%#a+\xBAV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x16\x91\x90a:\x92V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBEWa%\xBEa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xF7W\x81` \x01[a%\xE4a*\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a%\xDCW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\x0FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&&Wa&&a;\x08V[` \x02` \x01\x01Q\x90Pa&9\x81a\x05\x1AV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&QWa&Qa;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a&h\x90a;4V[\x91PPa%\xFDV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dv\x91\x90a>\x87V[`\x02T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R``\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'Y\x91\x90\x81\x01\x90a>\xBEV[`\x03T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xD0\x91\x90\x81\x01\x90a>\xBEV[\x90P\x91P\x91V[`\0`\x02\x82`\x02\x81\x11\x15a'\xEDWa'\xEDa<\x84V[\x03a(\x01WPg\r\xE0\xB6\xB3\xA7d\0\0a(iV[`\0\x80\x84`\x0F\x0B\x12a(:W`\0\x83`\x02\x81\x11\x15a(!Wa(!a<\x84V[\x14a(0W\x84`@\x01Qa(3V[\x84Q[\x90Pa(fV[`\0\x83`\x02\x81\x11\x15a(NWa(Na<\x84V[\x14a(]W\x84``\x01Qa(cV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a(\xB2WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a(\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a)7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a(\x87Wa(\x87a=\x94V[`\0a)\x8D\x83`\x0F\x0Ba){\x84\x87`\x0F\x0Ba(p\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba)\x88\x91\x90a?\xADV[a*2V[a(f\x90`\x02a@4V[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a(\xB2WP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a(\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a*\x16W\x81a(iV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a*\x16W\x81a(iV[`\0\x80\x82\x12\x15a*\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a#jV[`\x03\x82\x13\x15a*\xDDWP\x80`\0a*\x9C`\x02\x83a@\xD2V[a*\xA7\x90`\x01aA\0V[\x90P[\x81\x81\x12\x15a#\x0FW\x90P\x80`\x02\x81a*\xC2\x81\x86a@\xD2V[a*\xCC\x91\x90aA\0V[a*\xD6\x91\x90a@\xD2V[\x90Pa*\xAAV[\x81\x15a*\xE7WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a+\xF5`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a+\xB5`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a+\x84`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a,:`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-KW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a-`W`\0\x80\xFD[\x815a(i\x81a-9V[`\xA0\x81\x01a\";\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa.\x16`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa.V`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa.\xA4a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[PPPV[a\x02\xA0\x81\x01a\";\x82\x84a-\xB3V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/;Wa/;a/\x02V[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/;Wa/;a/\x02V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/\x8DWa/\x8Da/\x02V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\xAFWa/\xAFa/\x02V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/\xCAW`\0\x80\xFD[\x815` a/\xDFa/\xDA\x83a/\x95V[a/dV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/\xFEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a0\"W\x805a0\x15\x81a-9V[\x83R\x91\x83\x01\x91\x83\x01a0\x02V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a0@W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0^W`\0\x80\xFD[a0j\x85\x82\x86\x01a/\xB9V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa0\xA3` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa.\xEE``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\x18Wa1\x05\x83\x85Qa0tV[\x92\x84\x01\x92`\xC0\x92\x90\x92\x01\x91`\x01\x01a0\xF2V[P\x90\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa.\xEE`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa1\x88\x87\x83Qa1$V[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a1uV[P\x94\x95\x94PPPPPV[` \x81R`\0a(i` \x83\x01\x84a1aV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa1\xE5` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa2i`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa2\xF8a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa3\x0Ca\x02\0\x84\x01\x82a1\xB9V[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa3}\x87\x83Qa2\x06V[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3jV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa3\xB8\x87\x83Qa-\xB3V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3\xA5V[` \x81R`\0\x82Q`@` \x84\x01Ra3\xE8``\x84\x01\x82a3VV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra4\x05\x82\x82a3\x91V[\x95\x94PPPPPV[a\x03@\x81\x01a\";\x82\x84a2\x06V[`\0` \x82\x84\x03\x12\x15a4/W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa4}\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a4JV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0\x80[\x85\x81\x10\x15a5\x04W\x86\x84\x03`\x1F\x19\x01\x8AR\x82Q\x80Q\x80\x86R\x90\x86\x01\x90\x86\x86\x01\x90\x84[\x81\x81\x10\x15a4\xEFW\x83Q`\x0F\x0B\x83R\x92\x88\x01\x92\x91\x88\x01\x91`\x01\x01a4\xD0V[PP\x9A\x86\x01\x9A\x94PP\x91\x84\x01\x91`\x01\x01a4\xAEV[P\x91\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa59\x87\x83Qa0tV[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a5&V[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa5l`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra5\x89a\x01`\x85\x01\x83a46V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra5\xA7\x84\x83a4\x90V[\x93P`\x80\x87\x01Q\x91Pa5\xC2`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra5\xF0\x84\x83a1aV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra6\x0F\x85\x84a5\x12V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra6.\x85\x84a3VV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa6J\x83\x82a3\x91V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a6gW`\0\x80\xFD[\x825\x91P` \x83\x015a6y\x81a-9V[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\";\x82\x84a1$V[`\0` \x82\x84\x03\x12\x15a6\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xBBW`\0\x80\xFD[a6\xC7\x84\x82\x85\x01a/\xB9V[\x94\x93PPPPV[` \x81R`\0a(i` \x83\x01\x84a3VV[\x80`\x0F\x0B\x81\x14a-KW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a7\x03W`\0\x80\xFD[a7\x0Ba/\x18V[\x825a7\x16\x81a6\xE2V[\x81R` \x83\x015a7&\x81a6\xE2V[` \x82\x01R`@\x83\x015a79\x81a6\xE2V[`@\x82\x01R``\x83\x015a7L\x81a6\xE2V[``\x82\x01R`\x80\x83\x015a7_\x81a6\xE2V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-KW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\x92W`\0\x80\xFD[\x815a(i\x81a7kV[`\xC0\x81\x01a\";\x82\x84a0tV[` \x81R`\0a(i` \x83\x01\x84a3\x91V[`\0`\x80\x82\x84\x03\x12\x15a7\xD0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a7\xF3Wa7\xF3a/\x02V[`@R\x82Qa8\x01\x81a-9V[\x81R` \x83\x01Qa8\x11\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa8$\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa87\x81a6\xE2V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a8UW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8xWa8xa/\x02V[\x80`@RP\x80\x91P\x82Qa8\x8B\x81a6\xE2V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8\xA5W`\0\x80\xFD[a(i\x83\x83a8CV[`\0`\xA0\x82\x84\x03\x12\x15a8\xC1W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xE4Wa8\xE4a/\x02V[\x80`@RP\x80\x91P\x82Qa8\xF7\x81a6\xE2V[\x81R` \x83\x01Qa9\x07\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa9\x1A\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa9-\x81a6\xE2V[``\x82\x01R`\x80\x83\x01Qa9@\x81a6\xE2V[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a9_W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9\x82Wa9\x82a/\x02V[\x80`@RP\x80\x91P\x82Qa9\x95\x81a6\xE2V[\x81R` \x83\x01Qa9\xA5\x81a6\xE2V[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a9\xC4W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9\xE7Wa9\xE7a/\x02V[\x80`@RP\x80\x91P\x82Qa9\xFA\x81a6\xE2V[\x81R` \x83\x01Qa:\n\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa:\x1D\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa:0\x81a6\xE2V[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a:OW`\0\x80\xFD[a:Wa/AV[\x90P\x81Qa:d\x81a6\xE2V[\x81R` \x82\x01Qa:t\x81a6\xE2V[` \x82\x01R`@\x82\x01Qa:\x87\x81a6\xE2V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a:\xA9W`\0\x80\xFD[a:\xB3\x86\x86a8\xAFV[\x93Pa:\xC2\x86`\xA0\x87\x01a9MV[\x92Pa:\xD1\x86`\xE0\x87\x01a9\xB2V[\x91Pa:\xE1\x86a\x01`\x87\x01a:=V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a:\xFEW`\0\x80\xFD[a(i\x83\x83a8\xAFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a;MWa;Ma;\x1EV[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a;oW`\0\x80\xFD[`\xA0\x81\x12\x15a;}W`\0\x80\xFD[Pa;\x86a/AV[\x85Qa;\x91\x81a6\xE2V[\x81Ra;\xA0\x87` \x88\x01a9MV[` \x82\x01Ra;\xB2\x87``\x88\x01a9MV[`@\x82\x01R\x93Pa;\xC6\x86`\xA0\x87\x01a8CV[\x92Pa;\xD5\x86`\xC0\x87\x01a9\xB2V[\x91Pa:\xE1\x86a\x01@\x87\x01a9MV[`\0`\xA0\x82\x84\x03\x12\x15a;\xF7W`\0\x80\xFD[a;\xFFa/\x18V[\x82Qa<\n\x81a7kV[\x81R` \x83\x01Qa<\x1A\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa<-\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa<@\x81a6\xE2V[``\x82\x01R`\x80\x83\x01Qa7_\x81a6\xE2V[`\0`\x01\x82\x01a<eWa<ea;\x1EV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a<\x7FWa<\x7Fa;\x1EV[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a<\xC4Wa<\xC4a;\x1EV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a<\xE0Wa<\xE0a;\x1EV[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a<\xFFWa<\xFFa;\x1EV[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=\x1AW`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a=DWa=Da<\x84V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a=dW`\0\x80\xFD[a(i\x83\x83a:=V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a=\x8BWa=\x8Ba;\x1EV[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a=\xC1Wa=\xC1a=\x94V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a=\xE8Wa=\xE8a;\x1EV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a>\x1CWa>\x1Ca;\x1EV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a>7Wa>7a;\x1EV[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a;MWa;Ma;\x1EV[`\0` \x82\x84\x03\x12\x15a>oW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(iW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x99W`\0\x80\xFD[\x81Qa(i\x81a7kV[` \x81\x01`\x02\x83\x10a>\xB8Wa>\xB8a<\x84V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a>\xD1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xE8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a>\xF9W`\0\x80\xFD[\x80Qa?\x07a/\xDA\x82a/\x95V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a?&W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a?MW\x83Qa?>\x81a-9V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a?+V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a?\x85W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a?iV[\x81\x81\x11\x15a?\x97W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a?\xD5Wa?\xD5a;\x1EV[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a?\xF4Wa?\xF4a;\x1EV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a@\x10Wa@\x10a;\x1EV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a@&Wa@&a;\x1EV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a@dWa@da;\x1EV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a@\x90Wa@\x90a;\x1EV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a@\xACWa@\xACa;\x1EV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a@\xC2Wa@\xC2a;\x1EV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a@\xE1Wa@\xE1a=\x94V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a@\xFBWa@\xFBa;\x1EV[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15aA!WaA!a;\x1EV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15aA:WaA:a;\x1EV[PP\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2>\xA5\x045q\r\xD3b\xF3a\xF9c\xAA\xFB\xF2\x90\x98\x0Bo\x91\x80\x12e\xF4\x9D\x19\n\xEEbl\x06dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x1BW`\x005`\xE0\x1C\x80ci\xD3;\xE2\x11a\0\xB2W\x80c\x86\x99R\xFD\x11a\0\x81W\x80c\xC4\xD6m\xE8\x11a\0fW\x80c\xC4\xD6m\xE8\x14a\x03TW\x80c\xD7\xB2)\xB6\x14a\x03iW\x80c\xEE\x99(\xC9\x14a\x03\x89W`\0\x80\xFD[\x80c\x86\x99R\xFD\x14a\x02\xD6W\x80c\xB1\xCB\x0FB\x14a\x039W`\0\x80\xFD[\x80ci\xD3;\xE2\x14a\x02\x18W\x80cq$Z\xB1\x14a\x02 W\x80ct\x174\x04\x14a\x02\x96W\x80cu\xA5\xAB<\x14a\x02\xB6W`\0\x80\xFD[\x80c1TmQ\x11a\0\xEEW\x80c1TmQ\x14a\x01\x98W\x80cI\xFE\x1B<\x14a\x01\xB8W\x80cW#e?\x14a\x01\xD8W\x80c]p.\x1A\x14a\x01\xF8W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\x01 W\x80c\r\x8En,\x14a\x01IW\x80c\x1A\xE1\x0B\xC5\x14a\x01XW\x80c%\x93\xEB_\x14a\x01xW[`\0\x80\xFD[a\x013a\x01.6`\x04a-NV[a\x03\xA9V[`@Qa\x01@\x91\x90a-kV[`@Q\x80\x91\x03\x90\xF3[`@Q`\x1B\x81R` \x01a\x01@V[a\x01ka\x01f6`\x04a-NV[a\x05\x1AV[`@Qa\x01@\x91\x90a.\xF3V[a\x01\x8Ba\x01\x866`\x04a0-V[a\x07\0V[`@Qa\x01@\x91\x90a0\xD6V[a\x01\xABa\x01\xA66`\x04a0-V[a\x07\xD6V[`@Qa\x01@\x91\x90a1\xA6V[a\x01\xCBa\x01\xC66`\x04a-NV[a\x08\xA5V[`@Qa\x01@\x91\x90a3\xCCV[a\x01\xEBa\x01\xE66`\x04a-NV[a\x08\xF5V[`@Qa\x01@\x91\x90a4\x0EV[a\x02\x0Ba\x02\x066`\x04a4\x1DV[a\n\xF2V[`@Qa\x01@\x91\x90a5LV[a\x01\xCBa\x1DXV[a\x02(a\x1D{V[`@Qa\x01@\x91\x90`\0`\xE0\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01R\x80``\x85\x01Q\x16``\x84\x01R\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01R\x80`\xC0\x85\x01Q\x16`\xC0\x84\x01RP\x92\x91PPV[a\x02\xA9a\x02\xA46`\x04a6TV[a!\x8FV[`@Qa\x01@\x91\x90a6\x84V[a\x02\xC9a\x02\xC46`\x04a6\x92V[a\"AV[`@Qa\x01@\x91\x90a6\xCFV[a\x013a\x02\xE46`\x04a6\xF1V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01@V[a\x03ga\x03b6`\x04a7\x80V[a#\x15V[\0[a\x03|a\x03w6`\x04a6TV[a%\x1BV[`@Qa\x01@\x91\x90a7\x9DV[a\x03\x9Ca\x03\x976`\x04a6\x92V[a%\xA2V[`@Qa\x01@\x91\x90a7\xABV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x03\xDCa&pV[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04L\x91\x90a7\xBEV[\x90P`\0a\x04Xa&pV[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC8\x91\x90a8\x93V[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[a\x05\"a*\xECV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA1\x91\x90a:\x92V[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1E\x91\x90a:\xECV[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x06\x9D\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06\xECW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06\xF5V[a\x06\xF5\x87a\x03\xA9V[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x1CWa\x07\x1Ca/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07UW\x81` \x01[a\x07Ba+\xBAV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07:W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07\xCFW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x84Wa\x07\x84a;\x08V[` \x02` \x01\x01Q\x90Pa\x07\x98\x85\x82a%\x1BV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xB0Wa\x07\xB0a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xC7\x90a;4V[\x91PPa\x07[V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF2Wa\x07\xF2a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08+W\x81` \x01[a\x08\x18a,\x18V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\x10W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07\xCFW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08ZWa\x08Za;\x08V[` \x02` \x01\x01Q\x90Pa\x08n\x85\x82a!\x8FV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08\x86Wa\x08\x86a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x08\x9D\x90a;4V[\x91PPa\x081V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x08\xC5\x84a&\xDEV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x08\xDD\x84a\"AV[\x81R` \x01a\x08\xEB\x83a%\xA2V[\x90R\x94\x93PPPPV[a\x08\xFDa,bV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t|\x91\x90a;WV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF9\x91\x90a:\xECV[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\nx\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9D\x91\x90a;\xE5V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\x0B\x9E\x81a&\xDEV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xC3WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0C]W\x81\x84\x82\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0CKW\x83\x81\x81Q\x81\x10a\x0C:Wa\x0C:a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0CU\x81a<SV[\x91PPa\x0B\xF8V[P`\0[\x82Q\x81\x10\x15a\x0C\xC6W\x81\x83\x82\x81Q\x81\x10a\x0C}Wa\x0C}a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xB4W\x82\x81\x81Q\x81\x10a\x0C\xA3Wa\x0C\xA3a;\x08V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0C\xBE\x81a<SV[\x91PPa\x0CaV[Pa\x0C\xD2\x81`\x01a<lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xEAWa\x0C\xEAa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x1DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x08W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\r\x80W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\rbWa\rba;\x08V[` \x02` \x01\x01\x81\x90RP\x80\x80a\rx\x90a<SV[\x91PPa\r&V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9CWa\r\x9Ca/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xD5W\x81` \x01[a\r\xC2a,\x18V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xBAW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xF5Wa\r\xF5a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E.W\x81` \x01[a\x0E\x1Ba+\xBAV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x13W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ENWa\x0ENa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x87W\x81` \x01[a\x0Eta,bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0ElW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xA8Wa\x0E\xA8a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xE1W\x81` \x01[a\x0E\xCEa*\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\xC6W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\x14W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\x14Wa\x0F\x14a;\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA3\x91\x90a;WV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10#\x91\x90a:\xECV[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10oWa\x10oa;\x08V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10\xF3\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11m\x91\x90a;\xE5V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x11\x87\x89a\x03\xA9V[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x11\x9F\x82a;4V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xC4Wa\x11\xC4a;\x08V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12\xFBW\x83Q`\0\x90a\x12\x13\x90\x84\x90a\x12\n\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[a'\xD7V[`\x0F\x0B\x90a(pV[\x87Q\x90\x91P`\x0F\x0B\x15a\x12\x96W\x87Q\x87Q`\0\x91a\x124\x91`\x0F\x0B\x90a(\xF3V[\x90Pa\x12\x88a\x12T\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[`@\x8B\x01QQa\x12\n\x90a\x12k\x90`\x0F\x0B\x85a(pV[` \x8D\x01QQa\x12~\x90`\x0F\x0B\x86a(pV[\x89`\x80\x01Qa)\\V[a\x12\x92\x90\x83a<\x9AV[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xB3Wa\x12\xB3a;\x08V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\xCFWa\x12\xCFa;\x08V[` \x02` \x01\x01\x81\x81Qa\x12\xE3\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x12\xF4\x90P\x81a<\xE9V[\x90Pa\x11\xD2V[PPPPPPPP\x80a\x13\r\x90a;4V[\x90Pa\x0E\xEBV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16\xFBW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13AWa\x13Aa;\x08V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD0\x91\x90a:\x92V[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14P\x91\x90a:\xECV[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\x9CWa\x14\x9Ca;\x08V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x15 \x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x15:\x89a\x03\xA9V[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x15R\x82a;4V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15wWa\x15wa;\x08V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x16\xE2W`\0\x84` \x01Qa\x15\xC2\x84a\x12\n\x88`\0\x01Qa\x12\n\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[a\x15\xCC\x91\x90a<\x9AV[\x87Q\x90\x91P`\x0F\x0B\x15a\x16}W\x87Q\x87Q`\0\x91a\x15\xED\x91`\x0F\x0B\x90a(\xF3V[\x90Pa\x16\x15\x88`\0\x01Qa\x12\n\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba)\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16ea\x163\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x12\x05Wa\x12\x05a<\x84V[``\x8C\x01Qa\x12\n\x90a\x16I\x90`\x0F\x0B\x86a(pV[`\x80\x8E\x01Qa\x16[\x90`\x0F\x0B\x87a(pV[\x8A`\x80\x01Qa)\\V[a\x16o\x91\x90a<\x9AV[a\x16y\x90\x83a<\x9AV[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\x9AWa\x16\x9Aa;\x08V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x16\xB6Wa\x16\xB6a;\x08V[` \x02` \x01\x01\x81\x81Qa\x16\xCA\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x16\xDB\x90P\x81a<\xE9V[\x90Pa\x15\x85V[PPPPPPPP\x80a\x16\xF4\x90a;4V[\x90Pa\x13\x18V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17t\x91\x90a=\x08V[\x90P[\x80\x15a\x1B\0W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1A\xF8W`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x17\xC7Wa\x17\xC7a<\x84V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xE5\x93\x92\x91\x90a=!V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18&\x91\x90a=RV[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x18;WPa\x1A\xE6V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x18lWa\x18la<\x84V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x8A\x93\x92\x91\x90a=!V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCB\x91\x90a=RV[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18\xEEWP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18\xFAWPPa\x1A\xE6V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x19)W\x81Q\x83Qa\x19\"\x91\x90a\x19\x1D\x90a=nV[a*\x01V[\x90Pa\x19LV[\x81Q\x83Qa\x19@\x91\x90a\x19;\x90a=nV[a*\x1DV[a\x19I\x90a=nV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x19d\x91\x90a<\x9AV[a\x19n\x91\x90a=\xAAV[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x19\xBEW`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x9B\x91\x90a=\xF1V[a\x19\xA5\x91\x90a=\xAAV[a\x19\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\xF1V[\x90Pa\x19\xF7V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\xD8\x91\x90a=\xF1V[a\x19\xE2\x91\x90a=\xAAV[a\x19\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a=\xF1V[\x90P[`\0`\x02a\x1A*a\x1A\x08\x85\x85a=\xF1V[a\x12\n\x89` \x01Q\x89` \x01Qa\x1A\x1F\x91\x90a<\x9AV[`\x0F\x89\x90\x0B\x90a(pV[a\x1A4\x91\x90a=\xAAV[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1ASWa\x1ASa;\x08V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1AoWa\x1Aoa;\x08V[` \x02` \x01\x01\x81\x81Qa\x1A\x83\x91\x90a<\x9AV[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x1A\xA9Wa\x1A\xA9a;\x08V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x1A\xC5Wa\x1A\xC5a;\x08V[` \x02` \x01\x01\x81\x81Qa\x1A\xD9\x91\x90a<\x9AV[`\x0F\x0B\x90RPPPPPPP[\x80a\x1A\xF0\x81a<\xE9V[\x91PPa\x17\x90V[PPPa\x17wV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1DMW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\xA4W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1BOWa\x1BOa;\x08V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1BqWa\x1Bqa;\x08V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1C\rW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA1Wa\x1B\xA1a;\x08V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xC3Wa\x1B\xC3a;\x08V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xEAWa\x1B\xEAa;\x08V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1C\x02\x91\x90a<\x9AV[`\x0F\x0B\x90RPa\x1C\x94V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C,Wa\x1C,a;\x08V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CNWa\x1CNa;\x08V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CuWa\x1Cua;\x08V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1C\x8D\x91\x90a=\xF1V[`\x0F\x0B\x90RP[a\x1C\x9D\x81a>AV[\x90Pa\x1B\x18V[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\xC4Wa\x1C\xC4a;\x08V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C\xEFWa\x1C\xEFa;\x08V[` \x02` \x01\x01Q`\0\x01Qa\x1D\x05\x91\x90a=\xF1V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1D$Wa\x1D$a;\x08V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1DF\x90a>AV[\x90Pa\x1B\x03V[P\x92\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x1Dv`\0a\x08\xA5V[\x90P\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`@Q\x80`\xE0\x01`@R\x80`\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E6\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBA\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9B\x08a\xC1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F>\x91\x90a>\x87V[`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x9F\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x02T`@\x80Qc\x03c\x9B\x8B`\xE2\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\r\x8En,\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x1A\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x03T`@\x80Qc\x03c\x9B\x8B`\xE2\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\r\x8En,\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x95\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x06\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a!\x1Da&pV[`\x01`\x01`\xA0\x1B\x03\x16c\r\x8En,`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!~\x91\x90a>]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90R\x91\x90PV[a!\x97a,\x18V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x16\x91\x90a;WV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"]Wa\"]a/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x96W\x81` \x01[a\"\x83a,bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"{W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\x0FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\"\xC5Wa\"\xC5a;\x08V[` \x02` \x01\x01Q\x90Pa\"\xD8\x81a\x08\xF5V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\"\xF0Wa\"\xF0a;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a#\x07\x90a;4V[\x91PPa\"\x9CV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a#sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xF0\x91\x90a>\x87V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a$2\x91`\x04\x01a>\xA4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$s\x91\x90a>\x87V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a$\xB7\x90`\x01\x90`\x04\x01a>\xA4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xF8\x91\x90a>\x87V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a%#a+\xBAV[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x16\x91\x90a:\x92V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBEWa%\xBEa/\x02V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xF7W\x81` \x01[a%\xE4a*\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a%\xDCW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a#\x0FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&&Wa&&a;\x08V[` \x02` \x01\x01Q\x90Pa&9\x81a\x05\x1AV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a&QWa&Qa;\x08V[` \x02` \x01\x01\x81\x90RPP\x80\x80a&h\x90a;4V[\x91PPa%\xFDV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dv\x91\x90a>\x87V[`\x02T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R``\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'Y\x91\x90\x81\x01\x90a>\xBEV[`\x03T`@Qc\xF4\xC8\xC5\x8D`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x93P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF4\xC8\xC5\x8D\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\xD0\x91\x90\x81\x01\x90a>\xBEV[\x90P\x91P\x91V[`\0`\x02\x82`\x02\x81\x11\x15a'\xEDWa'\xEDa<\x84V[\x03a(\x01WPg\r\xE0\xB6\xB3\xA7d\0\0a(iV[`\0\x80\x84`\x0F\x0B\x12a(:W`\0\x83`\x02\x81\x11\x15a(!Wa(!a<\x84V[\x14a(0W\x84`@\x01Qa(3V[\x84Q[\x90Pa(fV[`\0\x83`\x02\x81\x11\x15a(NWa(Na<\x84V[\x14a(]W\x84``\x01Qa(cV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a(\xB2WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a(\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a)7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a(\x87Wa(\x87a=\x94V[`\0a)\x8D\x83`\x0F\x0Ba){\x84\x87`\x0F\x0Ba(p\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba)\x88\x91\x90a?\xADV[a*2V[a(f\x90`\x02a@4V[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a(\xB2WP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a(\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a#j\x91\x90a?XV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a*\x16W\x81a(iV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a*\x16W\x81a(iV[`\0\x80\x82\x12\x15a*\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a#jV[`\x03\x82\x13\x15a*\xDDWP\x80`\0a*\x9C`\x02\x83a@\xD2V[a*\xA7\x90`\x01aA\0V[\x90P[\x81\x81\x12\x15a#\x0FW\x90P\x80`\x02\x81a*\xC2\x81\x86a@\xD2V[a*\xCC\x91\x90aA\0V[a*\xD6\x91\x90a@\xD2V[\x90Pa*\xAAV[\x81\x15a*\xE7WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a+\xF5`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a+\xB5`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a+\x84`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a,:`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-KW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a-`W`\0\x80\xFD[\x815a(i\x81a-9V[`\xA0\x81\x01a\";\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa.\x16`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa.V`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa.\xA4a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[PPPV[a\x02\xA0\x81\x01a\";\x82\x84a-\xB3V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/;Wa/;a/\x02V[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/;Wa/;a/\x02V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/\x8DWa/\x8Da/\x02V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\xAFWa/\xAFa/\x02V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a/\xCAW`\0\x80\xFD[\x815` a/\xDFa/\xDA\x83a/\x95V[a/dV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/\xFEW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a0\"W\x805a0\x15\x81a-9V[\x83R\x91\x83\x01\x91\x83\x01a0\x02V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a0@W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0^W`\0\x80\xFD[a0j\x85\x82\x86\x01a/\xB9V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa0\xA3` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa.\xEE``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\x18Wa1\x05\x83\x85Qa0tV[\x92\x84\x01\x92`\xC0\x92\x90\x92\x01\x91`\x01\x01a0\xF2V[P\x90\x96\x95PPPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa.\xEE`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa1\x88\x87\x83Qa1$V[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a1uV[P\x94\x95\x94PPPPPV[` \x81R`\0a(i` \x83\x01\x84a1aV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa1\xE5` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa2i`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa2\xF8a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa3\x0Ca\x02\0\x84\x01\x82a1\xB9V[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa3}\x87\x83Qa2\x06V[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3jV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa3\xB8\x87\x83Qa-\xB3V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a3\xA5V[` \x81R`\0\x82Q`@` \x84\x01Ra3\xE8``\x84\x01\x82a3VV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra4\x05\x82\x82a3\x91V[\x95\x94PPPPPV[a\x03@\x81\x01a\";\x82\x84a2\x06V[`\0` \x82\x84\x03\x12\x15a4/W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa4}\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a4JV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0\x80[\x85\x81\x10\x15a5\x04W\x86\x84\x03`\x1F\x19\x01\x8AR\x82Q\x80Q\x80\x86R\x90\x86\x01\x90\x86\x86\x01\x90\x84[\x81\x81\x10\x15a4\xEFW\x83Q`\x0F\x0B\x83R\x92\x88\x01\x92\x91\x88\x01\x91`\x01\x01a4\xD0V[PP\x9A\x86\x01\x9A\x94PP\x91\x84\x01\x91`\x01\x01a4\xAEV[P\x91\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a1\x9BWa59\x87\x83Qa0tV[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a5&V[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa5l`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra5\x89a\x01`\x85\x01\x83a46V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra5\xA7\x84\x83a4\x90V[\x93P`\x80\x87\x01Q\x91Pa5\xC2`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra5\xF0\x84\x83a1aV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra6\x0F\x85\x84a5\x12V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra6.\x85\x84a3VV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa6J\x83\x82a3\x91V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a6gW`\0\x80\xFD[\x825\x91P` \x83\x015a6y\x81a-9V[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\";\x82\x84a1$V[`\0` \x82\x84\x03\x12\x15a6\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xBBW`\0\x80\xFD[a6\xC7\x84\x82\x85\x01a/\xB9V[\x94\x93PPPPV[` \x81R`\0a(i` \x83\x01\x84a3VV[\x80`\x0F\x0B\x81\x14a-KW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a7\x03W`\0\x80\xFD[a7\x0Ba/\x18V[\x825a7\x16\x81a6\xE2V[\x81R` \x83\x015a7&\x81a6\xE2V[` \x82\x01R`@\x83\x015a79\x81a6\xE2V[`@\x82\x01R``\x83\x015a7L\x81a6\xE2V[``\x82\x01R`\x80\x83\x015a7_\x81a6\xE2V[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-KW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\x92W`\0\x80\xFD[\x815a(i\x81a7kV[`\xC0\x81\x01a\";\x82\x84a0tV[` \x81R`\0a(i` \x83\x01\x84a3\x91V[`\0`\x80\x82\x84\x03\x12\x15a7\xD0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a7\xF3Wa7\xF3a/\x02V[`@R\x82Qa8\x01\x81a-9V[\x81R` \x83\x01Qa8\x11\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa8$\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa87\x81a6\xE2V[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a8UW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8xWa8xa/\x02V[\x80`@RP\x80\x91P\x82Qa8\x8B\x81a6\xE2V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8\xA5W`\0\x80\xFD[a(i\x83\x83a8CV[`\0`\xA0\x82\x84\x03\x12\x15a8\xC1W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8\xE4Wa8\xE4a/\x02V[\x80`@RP\x80\x91P\x82Qa8\xF7\x81a6\xE2V[\x81R` \x83\x01Qa9\x07\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa9\x1A\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa9-\x81a6\xE2V[``\x82\x01R`\x80\x83\x01Qa9@\x81a6\xE2V[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a9_W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9\x82Wa9\x82a/\x02V[\x80`@RP\x80\x91P\x82Qa9\x95\x81a6\xE2V[\x81R` \x83\x01Qa9\xA5\x81a6\xE2V[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a9\xC4W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9\xE7Wa9\xE7a/\x02V[\x80`@RP\x80\x91P\x82Qa9\xFA\x81a6\xE2V[\x81R` \x83\x01Qa:\n\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa:\x1D\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa:0\x81a6\xE2V[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a:OW`\0\x80\xFD[a:Wa/AV[\x90P\x81Qa:d\x81a6\xE2V[\x81R` \x82\x01Qa:t\x81a6\xE2V[` \x82\x01R`@\x82\x01Qa:\x87\x81a6\xE2V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a:\xA9W`\0\x80\xFD[a:\xB3\x86\x86a8\xAFV[\x93Pa:\xC2\x86`\xA0\x87\x01a9MV[\x92Pa:\xD1\x86`\xE0\x87\x01a9\xB2V[\x91Pa:\xE1\x86a\x01`\x87\x01a:=V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a:\xFEW`\0\x80\xFD[a(i\x83\x83a8\xAFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a;MWa;Ma;\x1EV[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a;oW`\0\x80\xFD[`\xA0\x81\x12\x15a;}W`\0\x80\xFD[Pa;\x86a/AV[\x85Qa;\x91\x81a6\xE2V[\x81Ra;\xA0\x87` \x88\x01a9MV[` \x82\x01Ra;\xB2\x87``\x88\x01a9MV[`@\x82\x01R\x93Pa;\xC6\x86`\xA0\x87\x01a8CV[\x92Pa;\xD5\x86`\xC0\x87\x01a9\xB2V[\x91Pa:\xE1\x86a\x01@\x87\x01a9MV[`\0`\xA0\x82\x84\x03\x12\x15a;\xF7W`\0\x80\xFD[a;\xFFa/\x18V[\x82Qa<\n\x81a7kV[\x81R` \x83\x01Qa<\x1A\x81a6\xE2V[` \x82\x01R`@\x83\x01Qa<-\x81a6\xE2V[`@\x82\x01R``\x83\x01Qa<@\x81a6\xE2V[``\x82\x01R`\x80\x83\x01Qa7_\x81a6\xE2V[`\0`\x01\x82\x01a<eWa<ea;\x1EV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a<\x7FWa<\x7Fa;\x1EV[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a<\xC4Wa<\xC4a;\x1EV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a<\xE0Wa<\xE0a;\x1EV[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a<\xFFWa<\xFFa;\x1EV[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=\x1AW`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a=DWa=Da<\x84V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a=dW`\0\x80\xFD[a(i\x83\x83a:=V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a=\x8BWa=\x8Ba;\x1EV[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a=\xC1Wa=\xC1a=\x94V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a=\xE8Wa=\xE8a;\x1EV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a>\x1CWa>\x1Ca;\x1EV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a>7Wa>7a;\x1EV[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a;MWa;Ma;\x1EV[`\0` \x82\x84\x03\x12\x15a>oW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(iW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x99W`\0\x80\xFD[\x81Qa(i\x81a7kV[` \x81\x01`\x02\x83\x10a>\xB8Wa>\xB8a<\x84V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a>\xD1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xE8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a>\xF9W`\0\x80\xFD[\x80Qa?\x07a/\xDA\x82a/\x95V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a?&W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a?MW\x83Qa?>\x81a-9V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a?+V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a?\x85W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a?iV[\x81\x81\x11\x15a?\x97W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a?\xD5Wa?\xD5a;\x1EV[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a?\xF4Wa?\xF4a;\x1EV[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a@\x10Wa@\x10a;\x1EV[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a@&Wa@&a;\x1EV[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a@dWa@da;\x1EV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a@\x90Wa@\x90a;\x1EV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a@\xACWa@\xACa;\x1EV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a@\xC2Wa@\xC2a;\x1EV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a@\xE1Wa@\xE1a=\x94V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a@\xFBWa@\xFBa;\x1EV[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15aA!WaA!a;\x1EV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15aA:WaA:a;\x1EV[PP\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2>\xA5\x045q\r\xD3b\xF3a\xF9c\xAA\xFB\xF2\x90\x98\x0Bo\x91\x80\x12e\xF4\x9D\x19\n\xEEbl\x06dsolcC\0\x08\r\x003";
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
        ///Calls the contract's `getContractVersions` (0x71245ab1) function
        pub fn get_contract_versions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Versions> {
            self.0
                .method_hash([113, 36, 90, 177], ())
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
    ///Container type for all input parameters for the `getContractVersions` function with signature `getContractVersions()` and selector `0x71245ab1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getContractVersions", abi = "getContractVersions()")]
    pub struct GetContractVersionsCall;
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
        GetContractVersions(GetContractVersionsCall),
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
        GetVersion(GetVersionCall),
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
                <GetContractVersionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetContractVersions(decoded));
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
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
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
                Self::GetContractVersions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetContractVersions(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetContractVersionsCall> for QuerierCalls {
        fn from(value: GetContractVersionsCall) -> Self {
            Self::GetContractVersions(value)
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
    ///Container type for all return fields from the `getContractVersions` function with signature `getContractVersions()` and selector `0x71245ab1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///`Versions(uint64,uint64,uint64,uint64,uint64,uint64,uint64)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        pub clearinghouse: u64,
        pub clearinghouse_liq: u64,
        pub spot_engine: u64,
        pub perp_engine: u64,
        pub querier: u64,
        pub exchange: u64,
    }
}
