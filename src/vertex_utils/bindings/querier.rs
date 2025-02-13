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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa<W\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ct\x174\x04\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02pW\x80c\xC4\xD6m\xE8\x14a\x02\x8BW\x80c\xD7\xB2)\xB6\x14a\x02\xA0W\x80c\xEE\x99(\xC9\x14a\x02\xC0W`\0\x80\xFD[\x80ct\x174\x04\x14a\x01\xCDW\x80cu\xA5\xAB<\x14a\x01\xEDW\x80c\x86\x99R\xFD\x14a\x02\rW`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xC8W\x80c%\x93\xEB_\x14a\x01MW\x80c1TmQ\x14a\x01mW\x80cW#e?\x14a\x01\x8DW\x80c]p.\x1A\x14a\x01\xADW`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xEFW\x80c\x02\xEE:R\x14a\x01\x18W\x80c\x1A\xE1\x0B\xC5\x14a\x01-W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a(XV[a\x02\xE0V[`@Qa\x01\x0F\x91\x90a(uV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x04QV[`@Qa\x01\x0F\x91\x90a,\x1BV[a\x01@a\x01;6`\x04a(XV[a\x04\x9FV[`@Qa\x01\x0F\x91\x90a,]V[a\x01`a\x01[6`\x04a-\x97V[a\x06\x85V[`@Qa\x01\x0F\x91\x90a.zV[a\x01\x80a\x01{6`\x04a-\x97V[a\x07[V[`@Qa\x01\x0F\x91\x90a/\x04V[a\x01\xA0a\x01\x9B6`\x04a(XV[a\x08*V[`@Qa\x01\x0F\x91\x90a/\x17V[a\x01\xC0a\x01\xBB6`\x04a/&V[a\n'V[`@Qa\x01\x0F\x91\x90a0\x1BV[a\x01\xE0a\x01\xDB6`\x04a1#V[a\x1C\x8CV[`@Qa\x01\x0F\x91\x90a1SV[a\x02\0a\x01\xFB6`\x04a1aV[a\x1D>V[`@Qa\x01\x0F\x91\x90a1\x9EV[a\x01\x02a\x02\x1B6`\x04a1\xFCV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[a\x02\x9Ea\x02\x996`\x04a2\x8BV[a\x1E\x12V[\0[a\x02\xB3a\x02\xAE6`\x04a1#V[a \x18V[`@Qa\x01\x0F\x91\x90a2\xA8V[a\x02\xD3a\x02\xCE6`\x04a1aV[a \x9FV[`@Qa\x01\x0F\x91\x90a2\xB6V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x03\x13a!mV[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a2\xC9V[\x90P`\0a\x03\x8Fa!mV[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFF\x91\x90a3\x9EV[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x04pa!\xE0V[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x04\x88\x84a\x1D>V[\x81R` \x01a\x04\x96\x83a \x9FV[\x90R\x93\x92PPPV[a\x04\xA7a%\xF6V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05&\x91\x90a5\x9DV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA3\x91\x90a5\xF7V[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x06\"\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06qW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06zV[a\x06z\x87a\x02\xE0V[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xA1Wa\x06\xA1a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xDAW\x81` \x01[a\x06\xC7a&\xC4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBFW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07TW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\tWa\x07\ta6\x13V[` \x02` \x01\x01Q\x90Pa\x07\x1D\x85\x82a \x18V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x075Wa\x075a6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07L\x90a6?V[\x91PPa\x06\xE0V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07wWa\x07wa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xB0W\x81` \x01[a\x07\x9Da'\"V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x95W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07TW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xDFWa\x07\xDFa6\x13V[` \x02` \x01\x01Q\x90Pa\x07\xF3\x85\x82a\x1C\x8CV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08\x0BWa\x08\x0Ba6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x08\"\x90a6?V[\x91PPa\x07\xB6V[a\x082a'lV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB1\x91\x90a6bV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a5\xF7V[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\t\xAD\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\"\x91\x90a6\xF0V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\n\xD2a!\xE0V[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\n\xF7WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\x91W\x81\x84\x82\x81Q\x81\x10a\x0BHWa\x0BHa6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\x7FW\x83\x81\x81Q\x81\x10a\x0BnWa\x0Bna6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\x89\x81a7^V[\x91PPa\x0B,V[P`\0[\x82Q\x81\x10\x15a\x0B\xFAW\x81\x83\x82\x81Q\x81\x10a\x0B\xB1Wa\x0B\xB1a6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xE8W\x82\x81\x81Q\x81\x10a\x0B\xD7Wa\x0B\xD7a6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\xF2\x81a7^V[\x91PPa\x0B\x95V[Pa\x0C\x06\x81`\x01a7wV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x1EWa\x0C\x1Ea,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CQW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C<W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C\xB4W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\x96Wa\x0C\x96a6\x13V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C\xAC\x90a7^V[\x91PPa\x0CZV[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD0Wa\x0C\xD0a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\tW\x81` \x01[a\x0C\xF6a'\"V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xEEW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r)Wa\r)a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rbW\x81` \x01[a\rOa&\xC4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rGW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x82Wa\r\x82a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xBBW\x81` \x01[a\r\xA8a'lV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xA0W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDCWa\r\xDCa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x15W\x81` \x01[a\x0E\x02a%\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xFAW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x12HW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0EHWa\x0EHa6\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD7\x91\x90a6bV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FW\x91\x90a5\xF7V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a6\x13V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10'\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA1\x91\x90a6\xF0V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x10\xBB\x89a\x02\xE0V[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x10\xD3\x82a6?V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xF8Wa\x10\xF8a6\x13V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12/W\x83Q`\0\x90a\x11G\x90\x84\x90a\x11>\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[a\"\xE1V[`\x0F\x0B\x90a#zV[\x87Q\x90\x91P`\x0F\x0B\x15a\x11\xCAW\x87Q\x87Q`\0\x91a\x11h\x91`\x0F\x0B\x90a#\xFDV[\x90Pa\x11\xBCa\x11\x88\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[`@\x8B\x01QQa\x11>\x90a\x11\x9F\x90`\x0F\x0B\x85a#zV[` \x8D\x01QQa\x11\xB2\x90`\x0F\x0B\x86a#zV[\x89`\x80\x01Qa$fV[a\x11\xC6\x90\x83a7\xA5V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xE7Wa\x11\xE7a6\x13V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\x03Wa\x12\x03a6\x13V[` \x02` \x01\x01\x81\x81Qa\x12\x17\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x12(\x90P\x81a7\xF4V[\x90Pa\x11\x06V[PPPPPPPP\x80a\x12A\x90a6?V[\x90Pa\x0E\x1FV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16/W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12uWa\x12ua6\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x04\x91\x90a5\x9DV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x84\x91\x90a5\xF7V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xD0Wa\x13\xD0a6\x13V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x14T\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x14n\x89a\x02\xE0V[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x14\x86\x82a6?V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\xABWa\x14\xABa6\x13V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x16\x16W`\0\x84` \x01Qa\x14\xF6\x84a\x11>\x88`\0\x01Qa\x11>\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[a\x15\0\x91\x90a7\xA5V[\x87Q\x90\x91P`\x0F\x0B\x15a\x15\xB1W\x87Q\x87Q`\0\x91a\x15!\x91`\x0F\x0B\x90a#\xFDV[\x90Pa\x15I\x88`\0\x01Qa\x11>\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba$\xA2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\x99a\x15g\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[``\x8C\x01Qa\x11>\x90a\x15}\x90`\x0F\x0B\x86a#zV[`\x80\x8E\x01Qa\x15\x8F\x90`\x0F\x0B\x87a#zV[\x8A`\x80\x01Qa$fV[a\x15\xA3\x91\x90a7\xA5V[a\x15\xAD\x90\x83a7\xA5V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xCEWa\x15\xCEa6\x13V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x15\xEAWa\x15\xEAa6\x13V[` \x02` \x01\x01\x81\x81Qa\x15\xFE\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x16\x0F\x90P\x81a7\xF4V[\x90Pa\x14\xB9V[PPPPPPPP\x80a\x16(\x90a6?V[\x90Pa\x12LV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xA8\x91\x90a8\x13V[\x90P[\x80\x15a\x1A4W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1A,W`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x16\xFBWa\x16\xFBa7\x8FV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x19\x93\x92\x91\x90a8,V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x176W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17Z\x91\x90a8]V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x17oWPa\x1A\x1AV[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x17\xA0Wa\x17\xA0a7\x8FV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xBE\x93\x92\x91\x90a8,V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xFF\x91\x90a8]V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18\"WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18.WPPa\x1A\x1AV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x18]W\x81Q\x83Qa\x18V\x91\x90a\x18Q\x90a8yV[a%\x0BV[\x90Pa\x18\x80V[\x81Q\x83Qa\x18t\x91\x90a\x18o\x90a8yV[a%'V[a\x18}\x90a8yV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x18\x98\x91\x90a7\xA5V[a\x18\xA2\x91\x90a8\xB5V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x18\xF2W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xCF\x91\x90a8\xFCV[a\x18\xD9\x91\x90a8\xB5V[a\x18\xEB\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90Pa\x19+V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x0C\x91\x90a8\xFCV[a\x19\x16\x91\x90a8\xB5V[a\x19(\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90P[`\0`\x02a\x19^a\x19<\x85\x85a8\xFCV[a\x11>\x89` \x01Q\x89` \x01Qa\x19S\x91\x90a7\xA5V[`\x0F\x89\x90\x0B\x90a#zV[a\x19h\x91\x90a8\xB5V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\x87Wa\x19\x87a6\x13V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xA3Wa\x19\xA3a6\x13V[` \x02` \x01\x01\x81\x81Qa\x19\xB7\x91\x90a7\xA5V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x19\xDDWa\x19\xDDa6\x13V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xF9Wa\x19\xF9a6\x13V[` \x02` \x01\x01\x81\x81Qa\x1A\r\x91\x90a7\xA5V[`\x0F\x0B\x90RPPPPPPP[\x80a\x1A$\x81a7\xF4V[\x91PPa\x16\xC4V[PPPa\x16\xABV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\x81W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1B\xD8W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\x83Wa\x1A\x83a6\x13V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xA5Wa\x1A\xA5a6\x13V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1BAW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xD5Wa\x1A\xD5a6\x13V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a6\x13V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x1EWa\x1B\x1Ea6\x13V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1B6\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x1B\xC8V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B`Wa\x1B`a6\x13V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x82Wa\x1B\x82a6\x13V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA9Wa\x1B\xA9a6\x13V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1B\xC1\x91\x90a8\xFCV[`\x0F\x0B\x90RP[a\x1B\xD1\x81a9LV[\x90Pa\x1ALV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xF8Wa\x1B\xF8a6\x13V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C#Wa\x1C#a6\x13V[` \x02` \x01\x01Q`\0\x01Qa\x1C9\x91\x90a8\xFCV[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CXWa\x1CXa6\x13V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1Cz\x90a9LV[\x90Pa\x1A7V[P\x92\x95\x94PPPPPV[a\x1C\x94a'\"V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x13\x91\x90a6bV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DZWa\x1DZa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x93W\x81` \x01[a\x1D\x80a'lV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1DxW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\x0CW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2a6\x13V[` \x02` \x01\x01Q\x90Pa\x1D\xD5\x81a\x08*V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1D\xEDWa\x1D\xEDa6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1E\x04\x90a6?V[\x91PPa\x1D\x99V[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1EpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xED\x91\x90a9hV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1F/\x91`\x04\x01a9\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fp\x91\x90a9hV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F\xB4\x90`\x01\x90`\x04\x01a9\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xF5\x91\x90a9hV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a  a&\xC4V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a {W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x13\x91\x90a5\x9DV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xBBWa \xBBa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xF4W\x81` \x01[a \xE1a%\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xD9W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\x0CW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!#Wa!#a6\x13V[` \x02` \x01\x01Q\x90Pa!6\x81a\x04\x9FV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!NWa!Na6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a!e\x90a6?V[\x91PPa \xFAV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xDB\x91\x90a9hV[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"^\x91\x90\x81\x01\x90a9\x9FV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\xDB\x91\x90\x81\x01\x90a9\x9FV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"\xF7Wa\"\xF7a7\x8FV[\x03a#\x0BWPg\r\xE0\xB6\xB3\xA7d\0\0a#sV[`\0\x80\x84`\x0F\x0B\x12a#DW`\0\x83`\x02\x81\x11\x15a#+Wa#+a7\x8FV[\x14a#:W\x84`@\x01Qa#=V[\x84Q[\x90Pa#pV[`\0\x83`\x02\x81\x11\x15a#XWa#Xa7\x8FV[\x14a#gW\x84``\x01Qa#mV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xBCWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a$AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a#\x91Wa#\x91a8\x9FV[`\0a$\x97\x83`\x0F\x0Ba$\x85\x84\x87`\x0F\x0Ba#z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba$\x92\x91\x90a:\x8EV[a%<V[a#p\x90`\x02a;\x15V[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xBCWP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a#\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a% W\x81a#sV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a% W\x81a#sV[`\0\x80\x82\x12\x15a%\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EgV[`\x03\x82\x13\x15a%\xE7WP\x80`\0a%\xA6`\x02\x83a;\xB3V[a%\xB1\x90`\x01a;\xE1V[\x90P[\x81\x81\x12\x15a\x1E\x0CW\x90P\x80`\x02\x81a%\xCC\x81\x86a;\xB3V[a%\xD6\x91\x90a;\xE1V[a%\xE0\x91\x90a;\xB3V[\x90Pa%\xB4V[\x81\x15a%\xF1WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a&\xFF`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a&\xBF`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a&\x8E`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a'D`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(UW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(jW`\0\x80\xFD[\x815a#s\x81a(CV[`\xA0\x81\x01a\x1D8\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa(\xE9` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[PPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa)r`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa*\x01a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa*\x15a\x02\0\x84\x01\x82a(\xBDV[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa*\x86\x87\x83Qa)\x0FV[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*sV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa+\x08`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa+H`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa+\x96a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa,\x07\x87\x83Qa*\xA5V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a+\xF4V[` \x81R`\0\x82Q`@` \x84\x01Ra,7``\x84\x01\x82a*_V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra,T\x82\x82a+\xE0V[\x95\x94PPPPPV[a\x02\xA0\x81\x01a\x1D8\x82\x84a*\xA5V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xA5Wa,\xA5a,lV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xA5Wa,\xA5a,lV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xF7Wa,\xF7a,lV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-\x19Wa-\x19a,lV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a-4W`\0\x80\xFD[\x815` a-Ia-D\x83a,\xFFV[a,\xCEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a-hW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a-\x8CW\x805a-\x7F\x81a(CV[\x83R\x91\x83\x01\x91\x83\x01a-lV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a-\xAAW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xC8W`\0\x80\xFD[a-\xD4\x85\x82\x86\x01a-#V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa.\r` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa)\n``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa.g\x87\x83Qa-\xDEV[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.TV[` \x81R`\0a#s` \x83\x01\x84a.@V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa)\n`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa.\xF1\x87\x83Qa.\x8DV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xDEV[` \x81R`\0a#s` \x83\x01\x84a.\xCAV[a\x03@\x81\x01a\x1D8\x82\x84a)\x0FV[`\0` \x82\x84\x03\x12\x15a/8W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa/\x86\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a/SV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0\x80[\x85\x81\x10\x15a0\rW\x86\x84\x03`\x1F\x19\x01\x8AR\x82Q\x80Q\x80\x86R\x90\x86\x01\x90\x86\x86\x01\x90\x84[\x81\x81\x10\x15a/\xF8W\x83Q`\x0F\x0B\x83R\x92\x88\x01\x92\x91\x88\x01\x91`\x01\x01a/\xD9V[PP\x9A\x86\x01\x9A\x94PP\x91\x84\x01\x91`\x01\x01a/\xB7V[P\x91\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa0;`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra0Xa\x01`\x85\x01\x83a/?V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra0v\x84\x83a/\x99V[\x93P`\x80\x87\x01Q\x91Pa0\x91`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra0\xBF\x84\x83a.\xCAV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra0\xDE\x85\x84a.@V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra0\xFD\x85\x84a*_V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa1\x19\x83\x82a+\xE0V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a16W`\0\x80\xFD[\x825\x91P` \x83\x015a1H\x81a(CV[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\x1D8\x82\x84a.\x8DV[`\0` \x82\x84\x03\x12\x15a1sW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x8AW`\0\x80\xFD[a1\x96\x84\x82\x85\x01a-#V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\xE1Wa1\xCD\x83\x85Qa)\x0FV[\x92\x84\x01\x92a\x03@\x92\x90\x92\x01\x91`\x01\x01a1\xBAV[P\x90\x96\x95PPPPPPV[\x80`\x0F\x0B\x81\x14a(UW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a2\x0EW`\0\x80\xFD[a2\x16a,\x82V[\x825a2!\x81a1\xEDV[\x81R` \x83\x015a21\x81a1\xEDV[` \x82\x01R`@\x83\x015a2D\x81a1\xEDV[`@\x82\x01R``\x83\x015a2W\x81a1\xEDV[``\x82\x01R`\x80\x83\x015a2j\x81a1\xEDV[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a2\x9DW`\0\x80\xFD[\x815a#s\x81a2vV[`\xC0\x81\x01a\x1D8\x82\x84a-\xDEV[` \x81R`\0a#s` \x83\x01\x84a+\xE0V[`\0`\x80\x82\x84\x03\x12\x15a2\xDBW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a2\xFEWa2\xFEa,lV[`@R\x82Qa3\x0C\x81a(CV[\x81R` \x83\x01Qa3\x1C\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa3/\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa3B\x81a1\xEDV[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3`W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\x83Wa3\x83a,lV[\x80`@RP\x80\x91P\x82Qa3\x96\x81a1\xEDV[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xB0W`\0\x80\xFD[a#s\x83\x83a3NV[`\0`\xA0\x82\x84\x03\x12\x15a3\xCCW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\xEFWa3\xEFa,lV[\x80`@RP\x80\x91P\x82Qa4\x02\x81a1\xEDV[\x81R` \x83\x01Qa4\x12\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa4%\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa48\x81a1\xEDV[``\x82\x01R`\x80\x83\x01Qa4K\x81a1\xEDV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a4jW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\x8DWa4\x8Da,lV[\x80`@RP\x80\x91P\x82Qa4\xA0\x81a1\xEDV[\x81R` \x83\x01Qa4\xB0\x81a1\xEDV[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a4\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xF2Wa4\xF2a,lV[\x80`@RP\x80\x91P\x82Qa5\x05\x81a1\xEDV[\x81R` \x83\x01Qa5\x15\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa5(\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa5;\x81a1\xEDV[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5ZW`\0\x80\xFD[a5ba,\xABV[\x90P\x81Qa5o\x81a1\xEDV[\x81R` \x82\x01Qa5\x7F\x81a1\xEDV[` \x82\x01R`@\x82\x01Qa5\x92\x81a1\xEDV[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a5\xB4W`\0\x80\xFD[a5\xBE\x86\x86a3\xBAV[\x93Pa5\xCD\x86`\xA0\x87\x01a4XV[\x92Pa5\xDC\x86`\xE0\x87\x01a4\xBDV[\x91Pa5\xEC\x86a\x01`\x87\x01a5HV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a6\tW`\0\x80\xFD[a#s\x83\x83a3\xBAV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a6XWa6Xa6)V[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a6zW`\0\x80\xFD[`\xA0\x81\x12\x15a6\x88W`\0\x80\xFD[Pa6\x91a,\xABV[\x85Qa6\x9C\x81a1\xEDV[\x81Ra6\xAB\x87` \x88\x01a4XV[` \x82\x01Ra6\xBD\x87``\x88\x01a4XV[`@\x82\x01R\x93Pa6\xD1\x86`\xA0\x87\x01a3NV[\x92Pa6\xE0\x86`\xC0\x87\x01a4\xBDV[\x91Pa5\xEC\x86a\x01@\x87\x01a4XV[`\0`\xA0\x82\x84\x03\x12\x15a7\x02W`\0\x80\xFD[a7\na,\x82V[\x82Qa7\x15\x81a2vV[\x81R` \x83\x01Qa7%\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa78\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa7K\x81a1\xEDV[``\x82\x01R`\x80\x83\x01Qa2j\x81a1\xEDV[`\0`\x01\x82\x01a7pWa7pa6)V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a7\x8AWa7\x8Aa6)V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a7\xCFWa7\xCFa6)V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a7\xEBWa7\xEBa6)V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a8\nWa8\na6)V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8%W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a8OWa8Oa7\x8FV[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a8oW`\0\x80\xFD[a#s\x83\x83a5HV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a8\x96Wa8\x96a6)V[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a8\xCCWa8\xCCa8\x9FV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a8\xF3Wa8\xF3a6)V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a9'Wa9'a6)V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a9BWa9Ba6)V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a6XWa6Xa6)V[`\0` \x82\x84\x03\x12\x15a9zW`\0\x80\xFD[\x81Qa#s\x81a2vV[` \x81\x01`\x02\x83\x10a9\x99Wa9\x99a7\x8FV[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a9\xB2W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xC9W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\xDAW`\0\x80\xFD[\x80Qa9\xE8a-D\x82a,\xFFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a:\x07W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a:.W\x83Qa:\x1F\x81a(CV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a:\x0CV[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:fW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:JV[\x81\x81\x11\x15a:xW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a:\xB6Wa:\xB6a6)V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a:\xD5Wa:\xD5a6)V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a:\xF1Wa:\xF1a6)V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\x07Wa;\x07a6)V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a;EWa;Ea6)V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a;qWa;qa6)V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a;\x8DWa;\x8Da6)V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\xA3Wa;\xA3a6)V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a;\xC2Wa;\xC2a8\x9FV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a;\xDCWa;\xDCa6)V[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15a<\x02Wa<\x02a6)V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a<\x1BWa<\x1Ba6)V[PP\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2\x96\x1FF\x1B?\xEA\xF8\x84\x17\xEA\x1Ev\x9B\x8E\x02j\x96\x05\x06x\xAC\xE8<{\x84\xF1Hll\x88[dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ct\x174\x04\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02pW\x80c\xC4\xD6m\xE8\x14a\x02\x8BW\x80c\xD7\xB2)\xB6\x14a\x02\xA0W\x80c\xEE\x99(\xC9\x14a\x02\xC0W`\0\x80\xFD[\x80ct\x174\x04\x14a\x01\xCDW\x80cu\xA5\xAB<\x14a\x01\xEDW\x80c\x86\x99R\xFD\x14a\x02\rW`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xC8W\x80c%\x93\xEB_\x14a\x01MW\x80c1TmQ\x14a\x01mW\x80cW#e?\x14a\x01\x8DW\x80c]p.\x1A\x14a\x01\xADW`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xEFW\x80c\x02\xEE:R\x14a\x01\x18W\x80c\x1A\xE1\x0B\xC5\x14a\x01-W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a(XV[a\x02\xE0V[`@Qa\x01\x0F\x91\x90a(uV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x04QV[`@Qa\x01\x0F\x91\x90a,\x1BV[a\x01@a\x01;6`\x04a(XV[a\x04\x9FV[`@Qa\x01\x0F\x91\x90a,]V[a\x01`a\x01[6`\x04a-\x97V[a\x06\x85V[`@Qa\x01\x0F\x91\x90a.zV[a\x01\x80a\x01{6`\x04a-\x97V[a\x07[V[`@Qa\x01\x0F\x91\x90a/\x04V[a\x01\xA0a\x01\x9B6`\x04a(XV[a\x08*V[`@Qa\x01\x0F\x91\x90a/\x17V[a\x01\xC0a\x01\xBB6`\x04a/&V[a\n'V[`@Qa\x01\x0F\x91\x90a0\x1BV[a\x01\xE0a\x01\xDB6`\x04a1#V[a\x1C\x8CV[`@Qa\x01\x0F\x91\x90a1SV[a\x02\0a\x01\xFB6`\x04a1aV[a\x1D>V[`@Qa\x01\x0F\x91\x90a1\x9EV[a\x01\x02a\x02\x1B6`\x04a1\xFCV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[a\x02\x9Ea\x02\x996`\x04a2\x8BV[a\x1E\x12V[\0[a\x02\xB3a\x02\xAE6`\x04a1#V[a \x18V[`@Qa\x01\x0F\x91\x90a2\xA8V[a\x02\xD3a\x02\xCE6`\x04a1aV[a \x9FV[`@Qa\x01\x0F\x91\x90a2\xB6V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x90a\x03\x13a!mV[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a2\xC9V[\x90P`\0a\x03\x8Fa!mV[`@QcH!\xC8\xB5`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cH!\xC8\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFF\x91\x90a3\x9EV[\x90P`@Q\x80`\xA0\x01`@R\x80\x83`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83``\x01Q`\x0F\x0B\x81R` \x01\x82`\0\x01Q`\x0F\x0B\x81RP\x92PPP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x04pa!\xE0V[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x04\x88\x84a\x1D>V[\x81R` \x01a\x04\x96\x83a \x9FV[\x90R\x93\x92PPPV[a\x04\xA7a%\xF6V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05&\x91\x90a5\x9DV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA3\x91\x90a5\xF7V[\x90P`@Q\x80`\xC0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\x06\"\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x83\x81R` \x01\x84\x81R` \x01\x86c\xFF\xFF\xFF\xFF\x16`\0\x03a\x06qW`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x06zV[a\x06z\x87a\x02\xE0V[\x90R\x95\x94PPPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xA1Wa\x06\xA1a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xDAW\x81` \x01[a\x06\xC7a&\xC4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBFW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07TW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\tWa\x07\ta6\x13V[` \x02` \x01\x01Q\x90Pa\x07\x1D\x85\x82a \x18V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x075Wa\x075a6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07L\x90a6?V[\x91PPa\x06\xE0V[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07wWa\x07wa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xB0W\x81` \x01[a\x07\x9Da'\"V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x95W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x07TW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xDFWa\x07\xDFa6\x13V[` \x02` \x01\x01Q\x90Pa\x07\xF3\x85\x82a\x1C\x8CV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x08\x0BWa\x08\x0Ba6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x08\"\x90a6?V[\x91PPa\x07\xB6V[a\x082a'lV[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xB1\x91\x90a6bV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R\x93\x95P\x90\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a5\xF7V[\x90P`@Q\x80`\xE0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01a\t\xAD\x83`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\"\x91\x90a6\xF0V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\n\xD2a!\xE0V[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\n\xF7WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\x91W\x81\x84\x82\x81Q\x81\x10a\x0BHWa\x0BHa6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\x7FW\x83\x81\x81Q\x81\x10a\x0BnWa\x0Bna6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\x89\x81a7^V[\x91PPa\x0B,V[P`\0[\x82Q\x81\x10\x15a\x0B\xFAW\x81\x83\x82\x81Q\x81\x10a\x0B\xB1Wa\x0B\xB1a6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xE8W\x82\x81\x81Q\x81\x10a\x0B\xD7Wa\x0B\xD7a6\x13V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\xF2\x81a7^V[\x91PPa\x0B\x95V[Pa\x0C\x06\x81`\x01a7wV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x1EWa\x0C\x1Ea,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CQW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C<W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C\xB4W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\x96Wa\x0C\x96a6\x13V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C\xAC\x90a7^V[\x91PPa\x0CZV[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD0Wa\x0C\xD0a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\tW\x81` \x01[a\x0C\xF6a'\"V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xEEW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r)Wa\r)a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rbW\x81` \x01[a\rOa&\xC4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rGW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x82Wa\r\x82a,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xBBW\x81` \x01[a\r\xA8a'lV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xA0W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xDCWa\r\xDCa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x15W\x81` \x01[a\x0E\x02a%\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xFAW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x12HW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0EHWa\x0EHa6\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD7\x91\x90a6bV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FW\x91\x90a5\xF7V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xC0\x01Q\x8C`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a6\x13V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xE0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x10'\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R`\x02T`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R` \x90\x92\x01\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA1\x91\x90a6\xF0V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x10\xBB\x89a\x02\xE0V[\x90Ra\x01\0\x8C\x01Q`\x80\x8D\x01\x80Q\x90a\x10\xD3\x82a6?V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xF8Wa\x10\xF8a6\x13V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x12/W\x83Q`\0\x90a\x11G\x90\x84\x90a\x11>\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[a\"\xE1V[`\x0F\x0B\x90a#zV[\x87Q\x90\x91P`\x0F\x0B\x15a\x11\xCAW\x87Q\x87Q`\0\x91a\x11h\x91`\x0F\x0B\x90a#\xFDV[\x90Pa\x11\xBCa\x11\x88\x86`\x01\x86`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[`@\x8B\x01QQa\x11>\x90a\x11\x9F\x90`\x0F\x0B\x85a#zV[` \x8D\x01QQa\x11\xB2\x90`\x0F\x0B\x86a#zV[\x89`\x80\x01Qa$fV[a\x11\xC6\x90\x83a7\xA5V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xE7Wa\x11\xE7a6\x13V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\x03Wa\x12\x03a6\x13V[` \x02` \x01\x01\x81\x81Qa\x12\x17\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x12(\x90P\x81a7\xF4V[\x90Pa\x11\x06V[PPPPPPPP\x80a\x12A\x90a6?V[\x90Pa\x0E\x1FV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x16/W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12uWa\x12ua6\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x04\x91\x90a5\x9DV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x8B\x16`\x04\x82\x01R\x94\x98P\x92\x96P\x90\x94P\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x84\x91\x90a5\xF7V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80``\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x84\x81RP\x8B`\xE0\x01Q\x8C`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\xD0Wa\x13\xD0a6\x13V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xC0\x01`@R\x80\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01a\x14T\x84`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x81R\x83\x85\x01\x83\x81R``\x80\x86\x01\x85\x81R`\x80\x87\x01\x95\x90\x95R\x87Q`\x0F\x90\x81\x0B\x87R\x93\x88\x01Q\x84\x0B\x90\x92R\x94\x86\x01Q\x82\x0B\x90\x94R\x93\x90\x92\x01Q\x90\x92\x0B\x90R\x90V[\x81R` \x01\x85\x81R` \x01\x87\x81R` \x01a\x14n\x89a\x02\xE0V[\x90Ra\x01 \x8C\x01Q`\xA0\x8D\x01\x80Q\x90a\x14\x86\x82a6?V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x14\xABWa\x14\xABa6\x13V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x16\x16W`\0\x84` \x01Qa\x14\xF6\x84a\x11>\x88`\0\x01Qa\x11>\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[a\x15\0\x91\x90a7\xA5V[\x87Q\x90\x91P`\x0F\x0B\x15a\x15\xB1W\x87Q\x87Q`\0\x91a\x15!\x91`\x0F\x0B\x90a#\xFDV[\x90Pa\x15I\x88`\0\x01Qa\x11>\x8A` \x01Q\x8C`@\x01Q`\x0F\x0Ba$\xA2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x15\x99a\x15g\x87`\x01\x87`\xFF\x16`\x02\x81\x11\x15a\x119Wa\x119a7\x8FV[``\x8C\x01Qa\x11>\x90a\x15}\x90`\x0F\x0B\x86a#zV[`\x80\x8E\x01Qa\x15\x8F\x90`\x0F\x0B\x87a#zV[\x8A`\x80\x01Qa$fV[a\x15\xA3\x91\x90a7\xA5V[a\x15\xAD\x90\x83a7\xA5V[\x91PP[\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x15\xCEWa\x15\xCEa6\x13V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x15\xEAWa\x15\xEAa6\x13V[` \x02` \x01\x01\x81\x81Qa\x15\xFE\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x16\x0F\x90P\x81a7\xF4V[\x90Pa\x14\xB9V[PPPPPPPP\x80a\x16(\x90a6?V[\x90Pa\x12LV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xA8\x91\x90a8\x13V[\x90P[\x80\x15a\x1A4W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x1A,W`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x16\xFBWa\x16\xFBa7\x8FV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x19\x93\x92\x91\x90a8,V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x176W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17Z\x91\x90a8]V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x17oWPa\x1A\x1AV[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x17\xA0Wa\x17\xA0a7\x8FV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xBE\x93\x92\x91\x90a8,V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xFF\x91\x90a8]V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x18\"WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x18.WPPa\x1A\x1AV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x18]W\x81Q\x83Qa\x18V\x91\x90a\x18Q\x90a8yV[a%\x0BV[\x90Pa\x18\x80V[\x81Q\x83Qa\x18t\x91\x90a\x18o\x90a8yV[a%'V[a\x18}\x90a8yV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x18\x98\x91\x90a7\xA5V[a\x18\xA2\x91\x90a8\xB5V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x18\xF2W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x18\xCF\x91\x90a8\xFCV[a\x18\xD9\x91\x90a8\xB5V[a\x18\xEB\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90Pa\x19+V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x19\x0C\x91\x90a8\xFCV[a\x19\x16\x91\x90a8\xB5V[a\x19(\x90g\r\xE0\xB6\xB3\xA7d\0\0a8\xFCV[\x90P[`\0`\x02a\x19^a\x19<\x85\x85a8\xFCV[a\x11>\x89` \x01Q\x89` \x01Qa\x19S\x91\x90a7\xA5V[`\x0F\x89\x90\x0B\x90a#zV[a\x19h\x91\x90a8\xB5V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\x87Wa\x19\x87a6\x13V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xA3Wa\x19\xA3a6\x13V[` \x02` \x01\x01\x81\x81Qa\x19\xB7\x91\x90a7\xA5V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x19\xDDWa\x19\xDDa6\x13V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x19\xF9Wa\x19\xF9a6\x13V[` \x02` \x01\x01\x81\x81Qa\x1A\r\x91\x90a7\xA5V[`\x0F\x0B\x90RPPPPPPP[\x80a\x1A$\x81a7\xF4V[\x91PPa\x16\xC4V[PPPa\x16\xABV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1C\x81W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1B\xD8W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\x83Wa\x1A\x83a6\x13V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xA5Wa\x1A\xA5a6\x13V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x1BAW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xD5Wa\x1A\xD5a6\x13V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a6\x13V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x1EWa\x1B\x1Ea6\x13V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x1B6\x91\x90a7\xA5V[`\x0F\x0B\x90RPa\x1B\xC8V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B`Wa\x1B`a6\x13V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\x82Wa\x1B\x82a6\x13V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xA9Wa\x1B\xA9a6\x13V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x1B\xC1\x91\x90a8\xFCV[`\x0F\x0B\x90RP[a\x1B\xD1\x81a9LV[\x90Pa\x1ALV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1B\xF8Wa\x1B\xF8a6\x13V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1C#Wa\x1C#a6\x13V[` \x02` \x01\x01Q`\0\x01Qa\x1C9\x91\x90a8\xFCV[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1CXWa\x1CXa6\x13V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1Cz\x90a9LV[\x90Pa\x1A7V[P\x92\x95\x94PPPPPV[a\x1C\x94a'\"V[`\x02T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x13\x91\x90a6bV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x8A\x16\x81R` \x81\x01\x94\x90\x94R\x83\x01RP\x93PPPP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DZWa\x1DZa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x93W\x81` \x01[a\x1D\x80a'lV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1DxW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\x0CW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2a6\x13V[` \x02` \x01\x01Q\x90Pa\x1D\xD5\x81a\x08*V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1D\xEDWa\x1D\xEDa6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1E\x04\x90a6?V[\x91PPa\x1D\x99V[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1EpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xED\x91\x90a9hV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1F/\x91`\x04\x01a9\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fp\x91\x90a9hV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F\xB4\x90`\x01\x90`\x04\x01a9\x85V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xF5\x91\x90a9hV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a  a&\xC4V[`\x03T`@Qc\x0FW2w`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c=\\\xC9\xDC\x90`D\x01a\x01\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a {W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x13\x91\x90a5\x9DV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xBBWa \xBBa,lV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xF4W\x81` \x01[a \xE1a%\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xD9W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E\x0CW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!#Wa!#a6\x13V[` \x02` \x01\x01Q\x90Pa!6\x81a\x04\x9FV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!NWa!Na6\x13V[` \x02` \x01\x01\x81\x90RPP\x80\x80a!e\x90a6?V[\x91PPa \xFAV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xDB\x91\x90a9hV[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"^\x91\x90\x81\x01\x90a9\x9FV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\xDB\x91\x90\x81\x01\x90a9\x9FV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"\xF7Wa\"\xF7a7\x8FV[\x03a#\x0BWPg\r\xE0\xB6\xB3\xA7d\0\0a#sV[`\0\x80\x84`\x0F\x0B\x12a#DW`\0\x83`\x02\x81\x11\x15a#+Wa#+a7\x8FV[\x14a#:W\x84`@\x01Qa#=V[\x84Q[\x90Pa#pV[`\0\x83`\x02\x81\x11\x15a#XWa#Xa7\x8FV[\x14a#gW\x84``\x01Qa#mV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xBCWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[P\x93\x92PPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a$AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a#\x91Wa#\x91a8\x9FV[`\0a$\x97\x83`\x0F\x0Ba$\x85\x84\x87`\x0F\x0Ba#z\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0Ba$\x92\x91\x90a:\x8EV[a%<V[a#p\x90`\x02a;\x15V[`\0`\x0F\x82\x81\x0B\x90\x84\x90\x0B\x03o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xBCWP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\x01`\x01`\x7F\x1B\x03\x82\x13\x15a#\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Eg\x91\x90a:9V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a% W\x81a#sV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a% W\x81a#sV[`\0\x80\x82\x12\x15a%\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EgV[`\x03\x82\x13\x15a%\xE7WP\x80`\0a%\xA6`\x02\x83a;\xB3V[a%\xB1\x90`\x01a;\xE1V[\x90P[\x81\x81\x12\x15a\x1E\x0CW\x90P\x80`\x02\x81a%\xCC\x81\x86a;\xB3V[a%\xD6\x91\x90a;\xE1V[a%\xE0\x91\x90a;\xB3V[\x90Pa%\xB4V[\x81\x15a%\xF1WP`\x01[\x91\x90PV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90[\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01a&\xFF`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01R\x91\x01R\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01\x90[\x81R` \x01a&\xBF`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R` \x01a&\x8E`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a'D`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(UW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(jW`\0\x80\xFD[\x815a#s\x81a(CV[`\xA0\x81\x01a\x1D8\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa(\xE9` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[PPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa)r`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01RP`\x80\x81\x01Qa*\x01a\x01\x80\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\xA0\x81\x01Qa*\x15a\x02\0\x84\x01\x82a(\xBDV[P`\xC0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\xA0\x84\x01R` \x82\x01Q\x81\x0Ba\x02\xC0\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\xE0\x84\x01R``\x82\x01Q\x81\x0Ba\x03\0\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x03 \x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa*\x86\x87\x83Qa)\x0FV[a\x03@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*sV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa+\x08`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa+H`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x81\x01Qa+\x96a\x01`\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02\0\x84\x01R` \x82\x01Q\x81\x0Ba\x02 \x84\x01R`@\x82\x01Q\x81\x0Ba\x02@\x84\x01R``\x82\x01Q\x81\x0Ba\x02`\x84\x01R`\x80\x90\x91\x01Q\x90\x0Ba\x02\x80\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa,\x07\x87\x83Qa*\xA5V[a\x02\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a+\xF4V[` \x81R`\0\x82Q`@` \x84\x01Ra,7``\x84\x01\x82a*_V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra,T\x82\x82a+\xE0V[\x95\x94PPPPPV[a\x02\xA0\x81\x01a\x1D8\x82\x84a*\xA5V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xA5Wa,\xA5a,lV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xA5Wa,\xA5a,lV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a,\xF7Wa,\xF7a,lV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a-\x19Wa-\x19a,lV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a-4W`\0\x80\xFD[\x815` a-Ia-D\x83a,\xFFV[a,\xCEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a-hW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a-\x8CW\x805a-\x7F\x81a(CV[\x83R\x91\x83\x01\x91\x83\x01a-lV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a-\xAAW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xC8W`\0\x80\xFD[a-\xD4\x85\x82\x86\x01a-#V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa.\r` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x81\x01Qa)\n``\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa.g\x87\x83Qa-\xDEV[`\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.TV[` \x81R`\0a#s` \x83\x01\x84a.@V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01QQ`\x0F\x0B` \x83\x01R`@\x81\x01Qa)\n`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa.\xF1\x87\x83Qa.\x8DV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a.\xDEV[` \x81R`\0a#s` \x83\x01\x84a.\xCAV[a\x03@\x81\x01a\x1D8\x82\x84a)\x0FV[`\0` \x82\x84\x03\x12\x15a/8W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a*\x9AWa/\x86\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a/SV[`\0\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01`\0\x80[\x85\x81\x10\x15a0\rW\x86\x84\x03`\x1F\x19\x01\x8AR\x82Q\x80Q\x80\x86R\x90\x86\x01\x90\x86\x86\x01\x90\x84[\x81\x81\x10\x15a/\xF8W\x83Q`\x0F\x0B\x83R\x92\x88\x01\x92\x91\x88\x01\x91`\x01\x01a/\xD9V[PP\x9A\x86\x01\x9A\x94PP\x91\x84\x01\x91`\x01\x01a/\xB7V[P\x91\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa0;`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra0Xa\x01`\x85\x01\x83a/?V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra0v\x84\x83a/\x99V[\x93P`\x80\x87\x01Q\x91Pa0\x91`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra0\xBF\x84\x83a.\xCAV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra0\xDE\x85\x84a.@V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra0\xFD\x85\x84a*_V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa1\x19\x83\x82a+\xE0V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a16W`\0\x80\xFD[\x825\x91P` \x83\x015a1H\x81a(CV[\x80\x91PP\x92P\x92\x90PV[`\x80\x81\x01a\x1D8\x82\x84a.\x8DV[`\0` \x82\x84\x03\x12\x15a1sW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x8AW`\0\x80\xFD[a1\x96\x84\x82\x85\x01a-#V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a1\xE1Wa1\xCD\x83\x85Qa)\x0FV[\x92\x84\x01\x92a\x03@\x92\x90\x92\x01\x91`\x01\x01a1\xBAV[P\x90\x96\x95PPPPPPV[\x80`\x0F\x0B\x81\x14a(UW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a2\x0EW`\0\x80\xFD[a2\x16a,\x82V[\x825a2!\x81a1\xEDV[\x81R` \x83\x015a21\x81a1\xEDV[` \x82\x01R`@\x83\x015a2D\x81a1\xEDV[`@\x82\x01R``\x83\x015a2W\x81a1\xEDV[``\x82\x01R`\x80\x83\x015a2j\x81a1\xEDV[`\x80\x82\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a2\x9DW`\0\x80\xFD[\x815a#s\x81a2vV[`\xC0\x81\x01a\x1D8\x82\x84a-\xDEV[` \x81R`\0a#s` \x83\x01\x84a+\xE0V[`\0`\x80\x82\x84\x03\x12\x15a2\xDBW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a2\xFEWa2\xFEa,lV[`@R\x82Qa3\x0C\x81a(CV[\x81R` \x83\x01Qa3\x1C\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa3/\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa3B\x81a1\xEDV[``\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3`W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\x83Wa3\x83a,lV[\x80`@RP\x80\x91P\x82Qa3\x96\x81a1\xEDV[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\xB0W`\0\x80\xFD[a#s\x83\x83a3NV[`\0`\xA0\x82\x84\x03\x12\x15a3\xCCW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a3\xEFWa3\xEFa,lV[\x80`@RP\x80\x91P\x82Qa4\x02\x81a1\xEDV[\x81R` \x83\x01Qa4\x12\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa4%\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa48\x81a1\xEDV[``\x82\x01R`\x80\x83\x01Qa4K\x81a1\xEDV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a4jW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\x8DWa4\x8Da,lV[\x80`@RP\x80\x91P\x82Qa4\xA0\x81a1\xEDV[\x81R` \x83\x01Qa4\xB0\x81a1\xEDV[` \x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a4\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a4\xF2Wa4\xF2a,lV[\x80`@RP\x80\x91P\x82Qa5\x05\x81a1\xEDV[\x81R` \x83\x01Qa5\x15\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa5(\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa5;\x81a1\xEDV[``\x91\x90\x91\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a5ZW`\0\x80\xFD[a5ba,\xABV[\x90P\x81Qa5o\x81a1\xEDV[\x81R` \x82\x01Qa5\x7F\x81a1\xEDV[` \x82\x01R`@\x82\x01Qa5\x92\x81a1\xEDV[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01\xC0\x85\x87\x03\x12\x15a5\xB4W`\0\x80\xFD[a5\xBE\x86\x86a3\xBAV[\x93Pa5\xCD\x86`\xA0\x87\x01a4XV[\x92Pa5\xDC\x86`\xE0\x87\x01a4\xBDV[\x91Pa5\xEC\x86a\x01`\x87\x01a5HV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xA0\x82\x84\x03\x12\x15a6\tW`\0\x80\xFD[a#s\x83\x83a3\xBAV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a6XWa6Xa6)V[`\x01\x01\x93\x92PPPV[`\0\x80`\0\x80\x84\x86\x03a\x01\x80\x81\x12\x15a6zW`\0\x80\xFD[`\xA0\x81\x12\x15a6\x88W`\0\x80\xFD[Pa6\x91a,\xABV[\x85Qa6\x9C\x81a1\xEDV[\x81Ra6\xAB\x87` \x88\x01a4XV[` \x82\x01Ra6\xBD\x87``\x88\x01a4XV[`@\x82\x01R\x93Pa6\xD1\x86`\xA0\x87\x01a3NV[\x92Pa6\xE0\x86`\xC0\x87\x01a4\xBDV[\x91Pa5\xEC\x86a\x01@\x87\x01a4XV[`\0`\xA0\x82\x84\x03\x12\x15a7\x02W`\0\x80\xFD[a7\na,\x82V[\x82Qa7\x15\x81a2vV[\x81R` \x83\x01Qa7%\x81a1\xEDV[` \x82\x01R`@\x83\x01Qa78\x81a1\xEDV[`@\x82\x01R``\x83\x01Qa7K\x81a1\xEDV[``\x82\x01R`\x80\x83\x01Qa2j\x81a1\xEDV[`\0`\x01\x82\x01a7pWa7pa6)V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a7\x8AWa7\x8Aa6)V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a7\xCFWa7\xCFa6)V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a7\xEBWa7\xEBa6)V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a8\nWa8\na6)V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a8%W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a8OWa8Oa7\x8FV[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a8oW`\0\x80\xFD[a#s\x83\x83a5HV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a8\x96Wa8\x96a6)V[`\0\x03\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a8\xCCWa8\xCCa8\x9FV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a8\xF3Wa8\xF3a6)V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a9'Wa9'a6)V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a9BWa9Ba6)V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a6XWa6Xa6)V[`\0` \x82\x84\x03\x12\x15a9zW`\0\x80\xFD[\x81Qa#s\x81a2vV[` \x81\x01`\x02\x83\x10a9\x99Wa9\x99a7\x8FV[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a9\xB2W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xC9W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\xDAW`\0\x80\xFD[\x80Qa9\xE8a-D\x82a,\xFFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a:\x07W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a:.W\x83Qa:\x1F\x81a(CV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a:\x0CV[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:fW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:JV[\x81\x81\x11\x15a:xW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15a:\xB6Wa:\xB6a6)V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15a:\xD5Wa:\xD5a6)V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15a:\xF1Wa:\xF1a6)V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\x07Wa;\x07a6)V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a;EWa;Ea6)V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a;qWa;qa6)V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a;\x8DWa;\x8Da6)V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a;\xA3Wa;\xA3a6)V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82a;\xC2Wa;\xC2a8\x9FV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a;\xDCWa;\xDCa6)V[P\x05\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15a<\x02Wa<\x02a6)V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a<\x1BWa<\x1Ba6)V[PP\x01\x90V\xFE\xA2dipfsX\"\x12 \xF2\x96\x1FF\x1B?\xEA\xF8\x84\x17\xEA\x1Ev\x9B\x8E\x02j\x96\x05\x06x\xAC\xE8<{\x84\xF1Hll\x88[dsolcC\0\x08\r\x003";
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
        ///Calls the contract's `getAllProducts` (0x02ee3a52) function
        pub fn get_all_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([2, 238, 58, 82], ())
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
    ///Container type for all input parameters for the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
