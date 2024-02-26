pub use offchain_book::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod offchain_book {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimSequencerFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimSequencerFee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feesAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearinghouse"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("clearinghouse"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dropDigest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dropDigest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("dropOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dropOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
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
                    ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
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
                    ::std::borrow::ToOwned::to_owned("dumpFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dumpFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("engine"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("engine"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IProductEngine"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFeeCalculator"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("filledAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("filledAmounts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllStates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllStates"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FOffchainBook.AllStates",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "collectedMakerFees",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "collectedTakerFees",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDigest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDigest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEndpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEndpoint"),
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
                    ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
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
                                    name: ::std::borrow::ToOwned::to_owned("takerFeeRateX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("makerFeeRateX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMarket"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOffchainBook.Market",
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
                    ::std::borrow::ToOwned::to_owned("getMinSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMinSize"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOraclePriceX18"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOraclePriceX18"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOraclePricesX18"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOraclePricesX18"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("healthGroup"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.Prices"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOrderFilledAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOrderFilledAmounts",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("incrementFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("incrementFees"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sequencer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_engine"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IProductEngine"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IFeeCalculator"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sizeIncrement"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_priceIncrementX18",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minSize"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpSpreadX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
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
                    ::std::borrow::ToOwned::to_owned("market"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("market"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                                    name: ::std::borrow::ToOwned::to_owned("lpSpreadX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collectedFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sequencerFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchOrderAMM"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ],
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchOrders"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.MatchOrdersWithSigner",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("modifyConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("modifyConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sizeIncrement"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_priceIncrementX18",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minSize"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpSpreadX18"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
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
                    ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("takerDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("makerDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAllStates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAllStates"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allStates"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct FOffchainBook.AllStates",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("setEndpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEndpoint"),
                            inputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMarket"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_market"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOffchainBook.Market",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("swapAMM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapAMM"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.SwapAMM"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("validateOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signedOrder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.SignedOrder",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FillOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FillOrder"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
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
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OFFCHAINBOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct OffchainBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OffchainBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OffchainBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OffchainBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OffchainBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OffchainBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OffchainBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OFFCHAINBOOK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `claimSequencerFee` (0x1bcb460a) function
        pub fn claim_sequencer_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([27, 203, 70, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearinghouse` (0x5d4f5f97) function
        pub fn clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([93, 79, 95, 151], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `dropOrder` (0xee442697) function
        pub fn drop_order(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 68, 38, 151], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropOrderChecked` (0xd43c77d6) function
        pub fn drop_order_checked(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 60, 119, 214], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpFees` (0x707c8b58) function
        pub fn dump_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `engine` (0xc9d4623f) function
        pub fn engine(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([201, 212, 98, 63], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees` (0x9af1d35a) function
        pub fn fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([154, 241, 211, 90], ())
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
        ///Calls the contract's `getAllStates` (0xa4ddb071) function
        pub fn get_all_states(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, AllStates> {
            self.0
                .method_hash([164, 221, 176, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollectedFees` (0xd54a8d18) function
        pub fn get_collected_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([213, 74, 141, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigest` (0xe874b4f3) function
        pub fn get_digest(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 116, 180, 243], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEndpoint` (0xaed8e967) function
        pub fn get_endpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 216, 233, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeRatesX18` (0x5e4b6063) function
        pub fn get_fee_rates_x18(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([94, 75, 96, 99], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarket` (0xf1be1679) function
        pub fn get_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, Market> {
            self.0
                .method_hash([241, 190, 22, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinSize` (0xd07041e0) function
        pub fn get_min_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([208, 112, 65, 224], ())
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
        ///Calls the contract's `getOrderFilledAmounts` (0x3455d913) function
        pub fn get_order_filled_amounts(
            &self,
            order_1: Order,
            order_2: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([52, 85, 217, 19], (order_1, order_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementFees` (0xa54b7cf3) function
        pub fn increment_fees(
            &self,
            maker_fee: i128,
            taker_fee: i128,
            sequencer: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 75, 124, 243], (maker_fee, taker_fee, sequencer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x5a174b8e) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            engine: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
            fees: ::ethers::core::types::Address,
            product_id: u32,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 23, 75, 142],
                    (
                        clearinghouse,
                        engine,
                        endpoint,
                        admin,
                        fees,
                        product_id,
                        size_increment,
                        price_increment_x18,
                        min_size,
                        lp_spread_x18,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `market` (0x80f55605) function
        pub fn market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u32, i128, i128, i128, i128, i128),
        > {
            self.0
                .method_hash([128, 245, 86, 5], ())
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
        ///Calls the contract's `matchOrders` (0x665dc77e) function
        pub fn match_orders(
            &self,
            txn: MatchOrdersWithSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 93, 199, 126], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyConfig` (0xc40f02bd) function
        pub fn modify_config(
            &self,
            size_increment: i128,
            price_increment_x18: i128,
            min_size: i128,
            lp_spread_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 15, 2, 189],
                    (size_increment, price_increment_x18, min_size, lp_spread_x18),
                )
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAllStates` (0x81f9d62e) function
        pub fn set_all_states(
            &self,
            all_states: AllStates,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 249, 214, 46], (all_states,))
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
        ///Calls the contract's `setMarket` (0x945d8ef6) function
        pub fn set_market(
            &self,
            market: Market,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 93, 142, 246], (market,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapAMM` (0x0f4b509d) function
        pub fn swap_amm(
            &self,
            txn: SwapAMM,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
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
        ///Calls the contract's `validateOrder` (0xba3691ad) function
        pub fn validate_order(
            &self,
            signed_order: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 54, 145, 173], (signed_order,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FillOrder` event
        pub fn fill_order_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FillOrderFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OffchainBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OffchainBook<M> {
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
        Hash
    )]
    #[ethevent(
        name = "FillOrder",
        abi = "FillOrder(bytes32,bytes32,int128,int128,uint64,uint64,bool,int128,int128,int128)"
    )]
    pub struct FillOrderFilter {
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
        Hash
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
        Hash
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
    pub enum OffchainBookEvents {
        FillOrderFilter(FillOrderFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for OffchainBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillOrderFilter::decode_log(log) {
                return Ok(OffchainBookEvents::FillOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OffchainBookEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OffchainBookEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OffchainBookEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FillOrderFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FillOrderFilter> for OffchainBookEvents {
        fn from(value: FillOrderFilter) -> Self {
            Self::FillOrderFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for OffchainBookEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for OffchainBookEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `claimSequencerFee` function with signature `claimSequencerFee()` and selector `0x1bcb460a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimSequencerFee", abi = "claimSequencerFee()")]
    pub struct ClaimSequencerFeeCall;
    ///Container type for all input parameters for the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clearinghouse", abi = "clearinghouse()")]
    pub struct ClearinghouseCall;
    ///Container type for all input parameters for the `dropDigest` function with signature `dropDigest(bytes32)` and selector `0xe1e7188d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dropDigest", abi = "dropDigest(bytes32)")]
    pub struct DropDigestCall {
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `dropOrder` function with signature `dropOrder((bytes32,int128,int128,uint64,uint64))` and selector `0xee442697`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "dropOrder",
        abi = "dropOrder((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `dropOrderChecked` function with signature `dropOrderChecked((bytes32,int128,int128,uint64,uint64))` and selector `0xd43c77d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "dropOrderChecked",
        abi = "dropOrderChecked((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct DropOrderCheckedCall {
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
        Hash
    )]
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    ///Container type for all input parameters for the `engine` function with signature `engine()` and selector `0xc9d4623f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "engine", abi = "engine()")]
    pub struct EngineCall;
    ///Container type for all input parameters for the `fees` function with signature `fees()` and selector `0x9af1d35a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fees", abi = "fees()")]
    pub struct FeesCall;
    ///Container type for all input parameters for the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "filledAmounts", abi = "filledAmounts(bytes32)")]
    pub struct FilledAmountsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `getAllStates` function with signature `getAllStates()` and selector `0xa4ddb071`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAllStates", abi = "getAllStates()")]
    pub struct GetAllStatesCall;
    ///Container type for all input parameters for the `getCollectedFees` function with signature `getCollectedFees()` and selector `0xd54a8d18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCollectedFees", abi = "getCollectedFees()")]
    pub struct GetCollectedFeesCall;
    ///Container type for all input parameters for the `getDigest` function with signature `getDigest((bytes32,int128,int128,uint64,uint64))` and selector `0xe874b4f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getDigest",
        abi = "getDigest((bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetDigestCall {
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
        Hash
    )]
    #[ethcall(name = "getEndpoint", abi = "getEndpoint()")]
    pub struct GetEndpointCall;
    ///Container type for all input parameters for the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32)` and selector `0x5e4b6063`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getFeeRatesX18", abi = "getFeeRatesX18(bytes32)")]
    pub struct GetFeeRatesX18Call {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getMarket` function with signature `getMarket()` and selector `0xf1be1679`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMarket", abi = "getMarket()")]
    pub struct GetMarketCall;
    ///Container type for all input parameters for the `getMinSize` function with signature `getMinSize()` and selector `0xd07041e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMinSize", abi = "getMinSize()")]
    pub struct GetMinSizeCall;
    ///Container type for all input parameters for the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `0x2f8f1fb0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    #[ethcall(name = "getOraclePricesX18", abi = "getOraclePricesX18(uint32)")]
    pub struct GetOraclePricesX18Call {
        pub health_group: u32,
    }
    ///Container type for all input parameters for the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `0x3455d913`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getOrderFilledAmounts",
        abi = "getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))"
    )]
    pub struct GetOrderFilledAmountsCall {
        pub order_1: Order,
        pub order_2: Order,
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
        Hash
    )]
    #[ethcall(name = "getVersion", abi = "getVersion()")]
    pub struct GetVersionCall;
    ///Container type for all input parameters for the `incrementFees` function with signature `incrementFees(int128,int128,int128)` and selector `0xa54b7cf3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "incrementFees", abi = "incrementFees(int128,int128,int128)")]
    pub struct IncrementFeesCall {
        pub maker_fee: i128,
        pub taker_fee: i128,
        pub sequencer: i128,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address,uint32,int128,int128,int128,int128)` and selector `0x5a174b8e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,address,uint32,int128,int128,int128,int128)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub engine: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
    }
    ///Container type for all input parameters for the `market` function with signature `market()` and selector `0x80f55605`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "market", abi = "market()")]
    pub struct MarketCall;
    ///Container type for all input parameters for the `matchOrderAMM` function with signature `matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)` and selector `0x78f0d3ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "matchOrderAMM",
        abi = "matchOrderAMM((uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes)),address)"
    )]
    pub struct MatchOrderAMMCall {
        pub txn: MatchOrderAMM,
        pub taker_linked_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))` and selector `0x665dc77e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "matchOrders",
        abi = "matchOrders(((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address))"
    )]
    pub struct MatchOrdersCall {
        pub txn: MatchOrdersWithSigner,
    }
    ///Container type for all input parameters for the `modifyConfig` function with signature `modifyConfig(int128,int128,int128,int128)` and selector `0xc40f02bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "modifyConfig", abi = "modifyConfig(int128,int128,int128,int128)")]
    pub struct ModifyConfigCall {
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub min_size: i128,
        pub lp_spread_x18: i128,
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setAllStates` function with signature `setAllStates((address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128))` and selector `0x81f9d62e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setAllStates",
        abi = "setAllStates((address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128))"
    )]
    pub struct SetAllStatesCall {
        pub all_states: AllStates,
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
        Hash
    )]
    #[ethcall(name = "setEndpoint", abi = "setEndpoint(address)")]
    pub struct SetEndpointCall {
        pub endpoint: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "setFilledAmount", abi = "setFilledAmount(bytes32,int128)")]
    pub struct SetFilledAmountCall {
        pub digest: [u8; 32],
        pub filled_amount: i128,
    }
    ///Container type for all input parameters for the `setMarket` function with signature `setMarket((uint32,int128,int128,int128,int128,int128))` and selector `0x945d8ef6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setMarket",
        abi = "setMarket((uint32,int128,int128,int128,int128,int128))"
    )]
    pub struct SetMarketCall {
        pub market: Market,
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `validateOrder` function with signature `validateOrder(((bytes32,int128,int128,uint64,uint64),bytes))` and selector `0xba3691ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validateOrder",
        abi = "validateOrder(((bytes32,int128,int128,uint64,uint64),bytes))"
    )]
    pub struct ValidateOrderCall {
        pub signed_order: SignedOrder,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OffchainBookCalls {
        ClaimSequencerFee(ClaimSequencerFeeCall),
        Clearinghouse(ClearinghouseCall),
        DropDigest(DropDigestCall),
        DropOrder(DropOrderCall),
        DropOrderChecked(DropOrderCheckedCall),
        DumpFees(DumpFeesCall),
        Engine(EngineCall),
        Fees(FeesCall),
        FilledAmounts(FilledAmountsCall),
        GetAllStates(GetAllStatesCall),
        GetCollectedFees(GetCollectedFeesCall),
        GetDigest(GetDigestCall),
        GetEndpoint(GetEndpointCall),
        GetFeeRatesX18(GetFeeRatesX18Call),
        GetMarket(GetMarketCall),
        GetMinSize(GetMinSizeCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderFilledAmounts(GetOrderFilledAmountsCall),
        GetVersion(GetVersionCall),
        IncrementFees(IncrementFeesCall),
        Initialize(InitializeCall),
        Market(MarketCall),
        MatchOrderAMM(MatchOrderAMMCall),
        MatchOrders(MatchOrdersCall),
        ModifyConfig(ModifyConfigCall),
        ModifyFilledAmount(ModifyFilledAmountCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAllStates(SetAllStatesCall),
        SetEndpoint(SetEndpointCall),
        SetFilledAmount(SetFilledAmountCall),
        SetMarket(SetMarketCall),
        SwapAMM(SwapAMMCall),
        TransferOwnership(TransferOwnershipCall),
        ValidateOrder(ValidateOrderCall),
    }
    impl ::ethers::core::abi::AbiDecode for OffchainBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimSequencerFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimSequencerFee(decoded));
            }
            if let Ok(decoded) = <ClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Clearinghouse(decoded));
            }
            if let Ok(decoded) = <DropDigestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DropDigest(decoded));
            }
            if let Ok(decoded) = <DropOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DropOrder(decoded));
            }
            if let Ok(decoded) = <DropOrderCheckedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DropOrderChecked(decoded));
            }
            if let Ok(decoded) = <DumpFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DumpFees(decoded));
            }
            if let Ok(decoded) = <EngineCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Engine(decoded));
            }
            if let Ok(decoded) = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded) = <FilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FilledAmounts(decoded));
            }
            if let Ok(decoded) = <GetAllStatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllStates(decoded));
            }
            if let Ok(decoded) = <GetCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCollectedFees(decoded));
            }
            if let Ok(decoded) = <GetDigestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDigest(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) = <GetFeeRatesX18Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFeeRatesX18(decoded));
            }
            if let Ok(decoded) = <GetMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMarket(decoded));
            }
            if let Ok(decoded) = <GetMinSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMinSize(decoded));
            }
            if let Ok(decoded) = <GetOraclePriceX18Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOraclePriceX18(decoded));
            }
            if let Ok(decoded) = <GetOraclePricesX18Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOraclePricesX18(decoded));
            }
            if let Ok(decoded) = <GetOrderFilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOrderFilledAmounts(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) = <IncrementFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncrementFees(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Market(decoded));
            }
            if let Ok(decoded) = <MatchOrderAMMCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchOrderAMM(decoded));
            }
            if let Ok(decoded) = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded) = <ModifyConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyConfig(decoded));
            }
            if let Ok(decoded) = <ModifyFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyFilledAmount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetAllStatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAllStates(decoded));
            }
            if let Ok(decoded) = <SetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded) = <SetFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFilledAmount(decoded));
            }
            if let Ok(decoded) = <SetMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMarket(decoded));
            }
            if let Ok(decoded) = <SwapAMMCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapAMM(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <ValidateOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateOrder(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OffchainBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimSequencerFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clearinghouse(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DropDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DropOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DropOrderChecked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DumpFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Engine(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FilledAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllStates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollectedFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFeeRatesX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOraclePriceX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOraclePricesX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderFilledAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncrementFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Market(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrderAMM(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyFilledAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAllStates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFilledAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapAMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OffchainBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrderChecked(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Engine(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FilledAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllStates(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollectedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeRatesX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePricesX18(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOrderFilledAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Market(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrderAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyFilledAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAllStates(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFilledAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateOrder(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimSequencerFeeCall> for OffchainBookCalls {
        fn from(value: ClaimSequencerFeeCall) -> Self {
            Self::ClaimSequencerFee(value)
        }
    }
    impl ::core::convert::From<ClearinghouseCall> for OffchainBookCalls {
        fn from(value: ClearinghouseCall) -> Self {
            Self::Clearinghouse(value)
        }
    }
    impl ::core::convert::From<DropDigestCall> for OffchainBookCalls {
        fn from(value: DropDigestCall) -> Self {
            Self::DropDigest(value)
        }
    }
    impl ::core::convert::From<DropOrderCall> for OffchainBookCalls {
        fn from(value: DropOrderCall) -> Self {
            Self::DropOrder(value)
        }
    }
    impl ::core::convert::From<DropOrderCheckedCall> for OffchainBookCalls {
        fn from(value: DropOrderCheckedCall) -> Self {
            Self::DropOrderChecked(value)
        }
    }
    impl ::core::convert::From<DumpFeesCall> for OffchainBookCalls {
        fn from(value: DumpFeesCall) -> Self {
            Self::DumpFees(value)
        }
    }
    impl ::core::convert::From<EngineCall> for OffchainBookCalls {
        fn from(value: EngineCall) -> Self {
            Self::Engine(value)
        }
    }
    impl ::core::convert::From<FeesCall> for OffchainBookCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<FilledAmountsCall> for OffchainBookCalls {
        fn from(value: FilledAmountsCall) -> Self {
            Self::FilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetAllStatesCall> for OffchainBookCalls {
        fn from(value: GetAllStatesCall) -> Self {
            Self::GetAllStates(value)
        }
    }
    impl ::core::convert::From<GetCollectedFeesCall> for OffchainBookCalls {
        fn from(value: GetCollectedFeesCall) -> Self {
            Self::GetCollectedFees(value)
        }
    }
    impl ::core::convert::From<GetDigestCall> for OffchainBookCalls {
        fn from(value: GetDigestCall) -> Self {
            Self::GetDigest(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for OffchainBookCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetFeeRatesX18Call> for OffchainBookCalls {
        fn from(value: GetFeeRatesX18Call) -> Self {
            Self::GetFeeRatesX18(value)
        }
    }
    impl ::core::convert::From<GetMarketCall> for OffchainBookCalls {
        fn from(value: GetMarketCall) -> Self {
            Self::GetMarket(value)
        }
    }
    impl ::core::convert::From<GetMinSizeCall> for OffchainBookCalls {
        fn from(value: GetMinSizeCall) -> Self {
            Self::GetMinSize(value)
        }
    }
    impl ::core::convert::From<GetOraclePriceX18Call> for OffchainBookCalls {
        fn from(value: GetOraclePriceX18Call) -> Self {
            Self::GetOraclePriceX18(value)
        }
    }
    impl ::core::convert::From<GetOraclePricesX18Call> for OffchainBookCalls {
        fn from(value: GetOraclePricesX18Call) -> Self {
            Self::GetOraclePricesX18(value)
        }
    }
    impl ::core::convert::From<GetOrderFilledAmountsCall> for OffchainBookCalls {
        fn from(value: GetOrderFilledAmountsCall) -> Self {
            Self::GetOrderFilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for OffchainBookCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<IncrementFeesCall> for OffchainBookCalls {
        fn from(value: IncrementFeesCall) -> Self {
            Self::IncrementFees(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OffchainBookCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MarketCall> for OffchainBookCalls {
        fn from(value: MarketCall) -> Self {
            Self::Market(value)
        }
    }
    impl ::core::convert::From<MatchOrderAMMCall> for OffchainBookCalls {
        fn from(value: MatchOrderAMMCall) -> Self {
            Self::MatchOrderAMM(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for OffchainBookCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<ModifyConfigCall> for OffchainBookCalls {
        fn from(value: ModifyConfigCall) -> Self {
            Self::ModifyConfig(value)
        }
    }
    impl ::core::convert::From<ModifyFilledAmountCall> for OffchainBookCalls {
        fn from(value: ModifyFilledAmountCall) -> Self {
            Self::ModifyFilledAmount(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for OffchainBookCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for OffchainBookCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAllStatesCall> for OffchainBookCalls {
        fn from(value: SetAllStatesCall) -> Self {
            Self::SetAllStates(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for OffchainBookCalls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetFilledAmountCall> for OffchainBookCalls {
        fn from(value: SetFilledAmountCall) -> Self {
            Self::SetFilledAmount(value)
        }
    }
    impl ::core::convert::From<SetMarketCall> for OffchainBookCalls {
        fn from(value: SetMarketCall) -> Self {
            Self::SetMarket(value)
        }
    }
    impl ::core::convert::From<SwapAMMCall> for OffchainBookCalls {
        fn from(value: SwapAMMCall) -> Self {
            Self::SwapAMM(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for OffchainBookCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ValidateOrderCall> for OffchainBookCalls {
        fn from(value: ValidateOrderCall) -> Self {
            Self::ValidateOrder(value)
        }
    }
    ///Container type for all return fields from the `claimSequencerFee` function with signature `claimSequencerFee()` and selector `0x1bcb460a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClaimSequencerFeeReturn {
        pub fees_amount: i128,
    }
    ///Container type for all return fields from the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ClearinghouseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `engine` function with signature `engine()` and selector `0xc9d4623f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EngineReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fees` function with signature `fees()` and selector `0x9af1d35a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FilledAmountsReturn(pub i128);
    ///Container type for all return fields from the `getAllStates` function with signature `getAllStates()` and selector `0xa4ddb071`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAllStatesReturn(pub AllStates);
    ///Container type for all return fields from the `getCollectedFees` function with signature `getCollectedFees()` and selector `0xd54a8d18`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetCollectedFeesReturn {
        pub collected_maker_fees: i128,
        pub collected_taker_fees: i128,
    }
    ///Container type for all return fields from the `getDigest` function with signature `getDigest((bytes32,int128,int128,uint64,uint64))` and selector `0xe874b4f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct GetEndpointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32)` and selector `0x5e4b6063`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetFeeRatesX18Return {
        pub taker_fee_rate_x18: i128,
        pub maker_fee_rate_x18: i128,
    }
    ///Container type for all return fields from the `getMarket` function with signature `getMarket()` and selector `0xf1be1679`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMarketReturn(pub Market);
    ///Container type for all return fields from the `getMinSize` function with signature `getMinSize()` and selector `0xd07041e0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetMinSizeReturn(pub i128);
    ///Container type for all return fields from the `getOraclePriceX18` function with signature `getOraclePriceX18(uint32)` and selector `0x2f8f1fb0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct GetOraclePricesX18Return(pub Prices);
    ///Container type for all return fields from the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts((bytes32,int128,int128,uint64,uint64),(bytes32,int128,int128,uint64,uint64))` and selector `0x3455d913`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetOrderFilledAmountsReturn(pub i128, pub i128);
    ///Container type for all return fields from the `getVersion` function with signature `getVersion()` and selector `0x0d8e6e2c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetVersionReturn(pub u64);
    ///Container type for all return fields from the `market` function with signature `market()` and selector `0x80f55605`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MarketReturn {
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub lp_spread_x18: i128,
        pub collected_fees: i128,
        pub sequencer_fees: i128,
    }
    ///Container type for all return fields from the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `0x35ed4e6d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///`AllStates(address,address,address,address,(uint32,int128,int128,int128,int128,int128),int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllStates {
        pub endpoint: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub engine: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
        pub market: Market,
        pub min_size: i128,
    }
    ///`MatchOrderAMM(uint32,int128,int128,((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MatchOrderAMM {
        pub product_id: u32,
        pub base_delta: i128,
        pub quote_delta: i128,
        pub taker: SignedOrder,
    }
    ///`MatchOrders(uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MatchOrders {
        pub product_id: u32,
        pub amm: bool,
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    ///`MatchOrdersWithSigner((uint32,bool,((bytes32,int128,int128,uint64,uint64),bytes),((bytes32,int128,int128,uint64,uint64),bytes)),address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        Hash
    )]
    pub struct Order {
        pub sender: [u8; 32],
        pub price_x18: i128,
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
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
        Hash
    )]
    pub struct Prices {
        pub spot_price_x18: i128,
        pub perp_price_x18: i128,
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
        Hash
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
        Hash
    )]
    pub struct SwapAMM {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: i128,
        pub price_x18: i128,
    }
    ///`Market(uint32,int128,int128,int128,int128,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Market {
        pub product_id: u32,
        pub size_increment: i128,
        pub price_increment_x18: i128,
        pub lp_spread_x18: i128,
        pub collected_fees: i128,
        pub sequencer_fees: i128,
    }
}
