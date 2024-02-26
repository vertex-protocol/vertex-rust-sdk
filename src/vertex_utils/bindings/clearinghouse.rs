pub use clearinghouse::*;
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
pub mod clearinghouse {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addEngine"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addEngine"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("engine"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("engineType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IProductEngine.EngineType",
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
                    ::std::borrow::ToOwned::to_owned("burnLp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnLp"),
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
                    ::std::borrow::ToOwned::to_owned("burnLpAndTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnLpAndTransfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.BurnLpAndTransfer",
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
                    ::std::borrow::ToOwned::to_owned("claimSequencerFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimSequencerFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.ClaimSequencerFees",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fees"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositCollateral"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.DepositCollateral",
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
                    ::std::borrow::ToOwned::to_owned("depositInsurance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositInsurance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.DepositInsurance",
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
                    ::std::borrow::ToOwned::to_owned("getClearinghouseLiq"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getClearinghouseLiq",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getEngineByProduct"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEngineByProduct"),
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
                    ::std::borrow::ToOwned::to_owned("getEngineByType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEngineByType"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("engineType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IProductEngine.EngineType",
                                        ),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getHealth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getHealth"),
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
                                    name: ::std::borrow::ToOwned::to_owned("healthType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IProductEngine.HealthType",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("health"),
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
                    ::std::borrow::ToOwned::to_owned("getInsurance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInsurance"),
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
                    ::std::borrow::ToOwned::to_owned("getMaxHealthGroup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxHealthGroup"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumProducts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNumProducts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("getOrderbook"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOrderbook"),
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
                    ::std::borrow::ToOwned::to_owned("getQuote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getQuote"),
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
                    ::std::borrow::ToOwned::to_owned("getRisk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRisk"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_endpoint"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_fees"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("insurance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("insurance"),
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
                    ::std::borrow::ToOwned::to_owned("isAboveInitial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAboveInitial"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isUnderInitial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isUnderInitial"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liqDecomposeLps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liqDecomposeLps"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liqFinalizeSubaccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liqFinalizeSubaccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liqLiquidationPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liqLiquidationPayment",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
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
                    ::std::borrow::ToOwned::to_owned("liqSettleAgainstLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liqSettleAgainstLiquidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
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
                    ::std::borrow::ToOwned::to_owned("liquidateSubaccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateSubaccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
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
                    ::std::borrow::ToOwned::to_owned("liquidateSubaccountImpl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "liquidateSubaccountImpl",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.LiquidateSubaccount",
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
                    ::std::borrow::ToOwned::to_owned("mintLp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintLp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintLp"),
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
                    ::std::borrow::ToOwned::to_owned("mintLpSlowMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintLpSlowMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintLp"),
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
                    ::std::borrow::ToOwned::to_owned("modifyProductConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyProductConfig",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                        ],
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("registerProductForId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerProductForId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("book"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IClearinghouseState.RiskStore",
                                        ),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("setDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("setInsurance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setInsurance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("settlePnl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settlePnl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.SettlePnl",
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
                    ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateFeeRates"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.UpdateFeeRates",
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
                    ::std::borrow::ToOwned::to_owned("upgradeClearinghouseLiq"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "upgradeClearinghouseLiq",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_clearinghouseLiq"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawCollateral"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IEndpoint.WithdrawCollateral",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ClearinghouseInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ClearinghouseInitialized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("endpoint"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("Liquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Liquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatorSubaccount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidateeSubaccount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("healthGroup"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountQuote"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("insuranceCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ModifyCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ModifyCollateral"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
    pub static CLEARINGHOUSE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Clearinghouse<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Clearinghouse<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Clearinghouse<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Clearinghouse<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Clearinghouse<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Clearinghouse))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Clearinghouse<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CLEARINGHOUSE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addEngine` (0x1750893e) function
        pub fn add_engine(
            &self,
            engine: ::ethers::core::types::Address,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 80, 137, 62], (engine, engine_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLp` (0xbf1fb321) function
        pub fn burn_lp(
            &self,
            txn: BurnLp,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 31, 179, 33], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLpAndTransfer` (0x0748a219) function
        pub fn burn_lp_and_transfer(
            &self,
            txn: BurnLpAndTransfer,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 72, 162, 25], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimSequencerFees` (0xf0390afe) function
        pub fn claim_sequencer_fees(
            &self,
            txn: ClaimSequencerFees,
            fees: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 57, 10, 254], (txn, fees))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateral` (0x67271722) function
        pub fn deposit_collateral(
            &self,
            txn: DepositCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 39, 23, 34], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositInsurance` (0x3a91c58b) function
        pub fn deposit_insurance(
            &self,
            txn: DepositInsurance,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 145, 197, 139], (txn,))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `getClearinghouseLiq` (0x9b0861c1) function
        pub fn get_clearinghouse_liq(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([155, 8, 97, 193], ())
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
        ///Calls the contract's `getEngineByProduct` (0xdeb14ec3) function
        pub fn get_engine_by_product(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 177, 78, 195], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEngineByType` (0x5d2e9ad1) function
        pub fn get_engine_by_type(
            &self,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([93, 46, 154, 209], engine_type)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealth` (0x88b6496f) function
        pub fn get_health(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([136, 182, 73, 111], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInsurance` (0x267a8da0) function
        pub fn get_insurance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([38, 122, 141, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxHealthGroup` (0xf8fac037) function
        pub fn get_max_health_group(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([248, 250, 192, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumProducts` (0x6bad77e1) function
        pub fn get_num_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([107, 173, 119, 225], ())
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([68, 39, 149, 45], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuote` (0x171755b1) function
        pub fn get_quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([23, 23, 85, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRisk` (0xecd9cba8) function
        pub fn get_risk(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Risk> {
            self.0
                .method_hash([236, 217, 203, 168], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf8c8765e) function
        pub fn initialize(
            &self,
            endpoint: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
            fees: ::ethers::core::types::Address,
            clearinghouse_liq: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (endpoint, quote, fees, clearinghouse_liq),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `insurance` (0x89cf3204) function
        pub fn insurance(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([137, 207, 50, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAboveInitial` (0x56bc3c38) function
        pub fn is_above_initial(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([86, 188, 60, 56], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isUnderInitial` (0xb5fc6205) function
        pub fn is_under_initial(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 252, 98, 5], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liqDecomposeLps` (0x4dafc1a6) function
        pub fn liq_decompose_lps(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([77, 175, 193, 166], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liqFinalizeSubaccount` (0x81210112) function
        pub fn liq_finalize_subaccount(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([129, 33, 1, 18], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liqLiquidationPayment` (0x191bdc11) function
        pub fn liq_liquidation_payment(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 27, 220, 17], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liqSettleAgainstLiquidator` (0xcb0138c3) function
        pub fn liq_settle_against_liquidator(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 1, 56, 195], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateSubaccount` (0x3ee8c44c) function
        pub fn liquidate_subaccount(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 232, 196, 76], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidateSubaccountImpl` (0xf8f0722d) function
        pub fn liquidate_subaccount_impl(
            &self,
            txn: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 240, 114, 45], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintLp` (0xe671b16b) function
        pub fn mint_lp(
            &self,
            txn: MintLp,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 113, 177, 107], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintLpSlowMode` (0xa3d02fa6) function
        pub fn mint_lp_slow_mode(
            &self,
            txn: MintLp,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 208, 47, 166], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyProductConfig` (0xccb6f69a) function
        pub fn modify_product_config(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 182, 246, 154], (product_id, risk_store))
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
        ///Calls the contract's `registerProductForId` (0x26df2414) function
        pub fn register_product_for_id(
            &self,
            book: ::ethers::core::types::Address,
            risk_store: RiskStore,
            health_group: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([38, 223, 36, 20], (book, risk_store, health_group))
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
        ///Calls the contract's `setDecimals` (0x6302345c) function
        pub fn set_decimals(
            &self,
            product_id: u32,
            decimals: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 2, 52, 92], (product_id, decimals))
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
        ///Calls the contract's `setInsurance` (0x02a0f0c5) function
        pub fn set_insurance(
            &self,
            amount: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 160, 240, 197], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settlePnl` (0xb2bb6367) function
        pub fn settle_pnl(
            &self,
            txn: SettlePnl,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 187, 99, 103], (txn,))
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
        ///Calls the contract's `updateFeeRates` (0x35639a4f) function
        pub fn update_fee_rates(
            &self,
            txn: UpdateFeeRates,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 99, 154, 79], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeClearinghouseLiq` (0x3c54c2de) function
        pub fn upgrade_clearinghouse_liq(
            &self,
            clearinghouse_liq: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 84, 194, 222], clearinghouse_liq)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawCollateral` (0xc354dd90) function
        pub fn withdraw_collateral(
            &self,
            txn: WithdrawCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 84, 221, 144], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ClearinghouseInitialized` event
        pub fn clearinghouse_initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClearinghouseInitializedFilter,
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
        ///Gets the contract's `Liquidation` event
        pub fn liquidation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ModifyCollateral` event
        pub fn modify_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ModifyCollateralFilter,
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
            ClearinghouseEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Clearinghouse<M> {
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
        Hash
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
        Hash
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
    pub enum ClearinghouseErrors {
        PRBMathSD59x18__SqrtNegativeInput(PRBMathSD59x18__SqrtNegativeInput),
        PRBMathSD59x18__SqrtOverflow(PRBMathSD59x18__SqrtOverflow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ClearinghouseErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <PRBMathSD59x18__SqrtNegativeInput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMathSD59x18__SqrtNegativeInput(decoded));
            }
            if let Ok(decoded) = <PRBMathSD59x18__SqrtOverflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PRBMathSD59x18__SqrtOverflow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClearinghouseErrors {
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
    impl ::ethers::contract::ContractRevert for ClearinghouseErrors {
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
    impl ::core::fmt::Display for ClearinghouseErrors {
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
    impl ::core::convert::From<::std::string::String> for ClearinghouseErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtNegativeInput>
    for ClearinghouseErrors {
        fn from(value: PRBMathSD59x18__SqrtNegativeInput) -> Self {
            Self::PRBMathSD59x18__SqrtNegativeInput(value)
        }
    }
    impl ::core::convert::From<PRBMathSD59x18__SqrtOverflow> for ClearinghouseErrors {
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
        Hash
    )]
    #[ethevent(
        name = "ClearinghouseInitialized",
        abi = "ClearinghouseInitialized(address,address,address)"
    )]
    pub struct ClearinghouseInitializedFilter {
        pub endpoint: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
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
        name = "Liquidation",
        abi = "Liquidation(bytes32,bytes32,uint8,uint32,int128,int128,int128)"
    )]
    pub struct LiquidationFilter {
        #[ethevent(indexed)]
        pub liquidator_subaccount: [u8; 32],
        #[ethevent(indexed)]
        pub liquidatee_subaccount: [u8; 32],
        #[ethevent(indexed)]
        pub mode: u8,
        pub health_group: u32,
        pub amount: i128,
        pub amount_quote: i128,
        pub insurance_cover: i128,
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
        name = "ModifyCollateral",
        abi = "ModifyCollateral(int128,bytes32,uint32)"
    )]
    pub struct ModifyCollateralFilter {
        pub amount: i128,
        #[ethevent(indexed)]
        pub subaccount: [u8; 32],
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
    pub enum ClearinghouseEvents {
        ClearinghouseInitializedFilter(ClearinghouseInitializedFilter),
        InitializedFilter(InitializedFilter),
        LiquidationFilter(LiquidationFilter),
        ModifyCollateralFilter(ModifyCollateralFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for ClearinghouseEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ClearinghouseInitializedFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::ClearinghouseInitializedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::LiquidationFilter(decoded));
            }
            if let Ok(decoded) = ModifyCollateralFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::ModifyCollateralFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ClearinghouseEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ClearinghouseEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClearinghouseInitializedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyCollateralFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ClearinghouseInitializedFilter> for ClearinghouseEvents {
        fn from(value: ClearinghouseInitializedFilter) -> Self {
            Self::ClearinghouseInitializedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for ClearinghouseEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationFilter> for ClearinghouseEvents {
        fn from(value: LiquidationFilter) -> Self {
            Self::LiquidationFilter(value)
        }
    }
    impl ::core::convert::From<ModifyCollateralFilter> for ClearinghouseEvents {
        fn from(value: ModifyCollateralFilter) -> Self {
            Self::ModifyCollateralFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ClearinghouseEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `addEngine` function with signature `addEngine(address,uint8)` and selector `0x1750893e`
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
    #[ethcall(name = "addEngine", abi = "addEngine(address,uint8)")]
    pub struct AddEngineCall {
        pub engine: ::ethers::core::types::Address,
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `burnLp` function with signature `burnLp((bytes32,uint32,uint128,uint64))` and selector `0xbf1fb321`
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
    #[ethcall(name = "burnLp", abi = "burnLp((bytes32,uint32,uint128,uint64))")]
    pub struct BurnLpCall {
        pub txn: BurnLp,
    }
    ///Container type for all input parameters for the `burnLpAndTransfer` function with signature `burnLpAndTransfer((bytes32,uint32,uint128,bytes32))` and selector `0x0748a219`
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
        name = "burnLpAndTransfer",
        abi = "burnLpAndTransfer((bytes32,uint32,uint128,bytes32))"
    )]
    pub struct BurnLpAndTransferCall {
        pub txn: BurnLpAndTransfer,
    }
    ///Container type for all input parameters for the `claimSequencerFees` function with signature `claimSequencerFees((bytes32),int128[])` and selector `0xf0390afe`
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
        name = "claimSequencerFees",
        abi = "claimSequencerFees((bytes32),int128[])"
    )]
    pub struct ClaimSequencerFeesCall {
        pub txn: ClaimSequencerFees,
        pub fees: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral((bytes32,uint32,uint128))` and selector `0x67271722`
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
        name = "depositCollateral",
        abi = "depositCollateral((bytes32,uint32,uint128))"
    )]
    pub struct DepositCollateralCall {
        pub txn: DepositCollateral,
    }
    ///Container type for all input parameters for the `depositInsurance` function with signature `depositInsurance((uint128))` and selector `0x3a91c58b`
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
    #[ethcall(name = "depositInsurance", abi = "depositInsurance((uint128))")]
    pub struct DepositInsuranceCall {
        pub txn: DepositInsurance,
    }
    ///Container type for all input parameters for the `getAllBooks` function with signature `getAllBooks()` and selector `0x354528e8`
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
    #[ethcall(name = "getAllBooks", abi = "getAllBooks()")]
    pub struct GetAllBooksCall;
    ///Container type for all input parameters for the `getClearinghouseLiq` function with signature `getClearinghouseLiq()` and selector `0x9b0861c1`
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
    #[ethcall(name = "getClearinghouseLiq", abi = "getClearinghouseLiq()")]
    pub struct GetClearinghouseLiqCall;
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
    ///Container type for all input parameters for the `getEngineByProduct` function with signature `getEngineByProduct(uint32)` and selector `0xdeb14ec3`
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
    #[ethcall(name = "getEngineByProduct", abi = "getEngineByProduct(uint32)")]
    pub struct GetEngineByProductCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `0x5d2e9ad1`
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
    #[ethcall(name = "getEngineByType", abi = "getEngineByType(uint8)")]
    pub struct GetEngineByTypeCall {
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `0x88b6496f`
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
    #[ethcall(name = "getHealth", abi = "getHealth(bytes32,uint8)")]
    pub struct GetHealthCall {
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getInsurance` function with signature `getInsurance()` and selector `0x267a8da0`
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
    #[ethcall(name = "getInsurance", abi = "getInsurance()")]
    pub struct GetInsuranceCall;
    ///Container type for all input parameters for the `getMaxHealthGroup` function with signature `getMaxHealthGroup()` and selector `0xf8fac037`
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
    #[ethcall(name = "getMaxHealthGroup", abi = "getMaxHealthGroup()")]
    pub struct GetMaxHealthGroupCall;
    ///Container type for all input parameters for the `getNumProducts` function with signature `getNumProducts()` and selector `0x6bad77e1`
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
    #[ethcall(name = "getNumProducts", abi = "getNumProducts()")]
    pub struct GetNumProductsCall;
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
    ///Container type for all input parameters for the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `0x4427952d`
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
    #[ethcall(name = "getOrderbook", abi = "getOrderbook(uint32)")]
    pub struct GetOrderbookCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getQuote` function with signature `getQuote()` and selector `0x171755b1`
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
    #[ethcall(name = "getQuote", abi = "getQuote()")]
    pub struct GetQuoteCall;
    ///Container type for all input parameters for the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
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
    #[ethcall(name = "getRisk", abi = "getRisk(uint32)")]
    pub struct GetRiskCall {
        pub product_id: u32,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `0xf8c8765e`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address,address)")]
    pub struct InitializeCall {
        pub endpoint: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
        pub fees: ::ethers::core::types::Address,
        pub clearinghouse_liq: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `insurance` function with signature `insurance()` and selector `0x89cf3204`
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
    #[ethcall(name = "insurance", abi = "insurance()")]
    pub struct InsuranceCall;
    ///Container type for all input parameters for the `isAboveInitial` function with signature `isAboveInitial(bytes32)` and selector `0x56bc3c38`
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
    #[ethcall(name = "isAboveInitial", abi = "isAboveInitial(bytes32)")]
    pub struct IsAboveInitialCall {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `isUnderInitial` function with signature `isUnderInitial(bytes32)` and selector `0xb5fc6205`
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
    #[ethcall(name = "isUnderInitial", abi = "isUnderInitial(bytes32)")]
    pub struct IsUnderInitialCall {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `liqDecomposeLps` function with signature `liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x4dafc1a6`
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
        name = "liqDecomposeLps",
        abi = "liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqDecomposeLpsCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liqFinalizeSubaccount` function with signature `liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x81210112`
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
        name = "liqFinalizeSubaccount",
        abi = "liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqFinalizeSubaccountCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liqLiquidationPayment` function with signature `liqLiquidationPayment((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x191bdc11`
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
        name = "liqLiquidationPayment",
        abi = "liqLiquidationPayment((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqLiquidationPaymentCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liqSettleAgainstLiquidator` function with signature `liqSettleAgainstLiquidator((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0xcb0138c3`
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
        name = "liqSettleAgainstLiquidator",
        abi = "liqSettleAgainstLiquidator((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiqSettleAgainstLiquidatorCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liquidateSubaccount` function with signature `liquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x3ee8c44c`
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
        name = "liquidateSubaccount",
        abi = "liquidateSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiquidateSubaccountCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `liquidateSubaccountImpl` function with signature `liquidateSubaccountImpl((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0xf8f0722d`
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
        name = "liquidateSubaccountImpl",
        abi = "liquidateSubaccountImpl((bytes32,bytes32,uint8,uint32,int128,uint64))"
    )]
    pub struct LiquidateSubaccountImplCall {
        pub txn: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `mintLp` function with signature `mintLp((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `0xe671b16b`
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
        name = "mintLp",
        abi = "mintLp((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct MintLpCall {
        pub txn: MintLp,
    }
    ///Container type for all input parameters for the `mintLpSlowMode` function with signature `mintLpSlowMode((bytes32,uint32,uint128,uint128,uint128,uint64))` and selector `0xa3d02fa6`
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
        name = "mintLpSlowMode",
        abi = "mintLpSlowMode((bytes32,uint32,uint128,uint128,uint128,uint64))"
    )]
    pub struct MintLpSlowModeCall {
        pub txn: MintLp,
    }
    ///Container type for all input parameters for the `modifyProductConfig` function with signature `modifyProductConfig(uint32,(int32,int32,int32,int32,int32))` and selector `0xccb6f69a`
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
        name = "modifyProductConfig",
        abi = "modifyProductConfig(uint32,(int32,int32,int32,int32,int32))"
    )]
    pub struct ModifyProductConfigCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
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
    ///Container type for all input parameters for the `registerProductForId` function with signature `registerProductForId(address,(int32,int32,int32,int32,int32),uint32)` and selector `0x26df2414`
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
        name = "registerProductForId",
        abi = "registerProductForId(address,(int32,int32,int32,int32,int32),uint32)"
    )]
    pub struct RegisterProductForIdCall {
        pub book: ::ethers::core::types::Address,
        pub risk_store: RiskStore,
        pub health_group: u32,
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setDecimals` function with signature `setDecimals(uint32,uint8)` and selector `0x6302345c`
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
    #[ethcall(name = "setDecimals", abi = "setDecimals(uint32,uint8)")]
    pub struct SetDecimalsCall {
        pub product_id: u32,
        pub decimals: u8,
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
    ///Container type for all input parameters for the `setInsurance` function with signature `setInsurance(int128)` and selector `0x02a0f0c5`
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
    #[ethcall(name = "setInsurance", abi = "setInsurance(int128)")]
    pub struct SetInsuranceCall {
        pub amount: i128,
    }
    ///Container type for all input parameters for the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `0xb2bb6367`
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
    #[ethcall(name = "settlePnl", abi = "settlePnl((bytes32[],uint256[]))")]
    pub struct SettlePnlCall {
        pub txn: SettlePnl,
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
    ///Container type for all input parameters for the `updateFeeRates` function with signature `updateFeeRates((address,uint32,int64,int64))` and selector `0x35639a4f`
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
        name = "updateFeeRates",
        abi = "updateFeeRates((address,uint32,int64,int64))"
    )]
    pub struct UpdateFeeRatesCall {
        pub txn: UpdateFeeRates,
    }
    ///Container type for all input parameters for the `upgradeClearinghouseLiq` function with signature `upgradeClearinghouseLiq(address)` and selector `0x3c54c2de`
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
        name = "upgradeClearinghouseLiq",
        abi = "upgradeClearinghouseLiq(address)"
    )]
    pub struct UpgradeClearinghouseLiqCall {
        pub clearinghouse_liq: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawCollateral` function with signature `withdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `0xc354dd90`
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
        name = "withdrawCollateral",
        abi = "withdrawCollateral((bytes32,uint32,uint128,uint64))"
    )]
    pub struct WithdrawCollateralCall {
        pub txn: WithdrawCollateral,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ClearinghouseCalls {
        AddEngine(AddEngineCall),
        BurnLp(BurnLpCall),
        BurnLpAndTransfer(BurnLpAndTransferCall),
        ClaimSequencerFees(ClaimSequencerFeesCall),
        DepositCollateral(DepositCollateralCall),
        DepositInsurance(DepositInsuranceCall),
        GetAllBooks(GetAllBooksCall),
        GetClearinghouseLiq(GetClearinghouseLiqCall),
        GetEndpoint(GetEndpointCall),
        GetEngineByProduct(GetEngineByProductCall),
        GetEngineByType(GetEngineByTypeCall),
        GetHealth(GetHealthCall),
        GetInsurance(GetInsuranceCall),
        GetMaxHealthGroup(GetMaxHealthGroupCall),
        GetNumProducts(GetNumProductsCall),
        GetOraclePriceX18(GetOraclePriceX18Call),
        GetOraclePricesX18(GetOraclePricesX18Call),
        GetOrderbook(GetOrderbookCall),
        GetQuote(GetQuoteCall),
        GetRisk(GetRiskCall),
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
        Insurance(InsuranceCall),
        IsAboveInitial(IsAboveInitialCall),
        IsUnderInitial(IsUnderInitialCall),
        LiqDecomposeLps(LiqDecomposeLpsCall),
        LiqFinalizeSubaccount(LiqFinalizeSubaccountCall),
        LiqLiquidationPayment(LiqLiquidationPaymentCall),
        LiqSettleAgainstLiquidator(LiqSettleAgainstLiquidatorCall),
        LiquidateSubaccount(LiquidateSubaccountCall),
        LiquidateSubaccountImpl(LiquidateSubaccountImplCall),
        MintLp(MintLpCall),
        MintLpSlowMode(MintLpSlowModeCall),
        ModifyProductConfig(ModifyProductConfigCall),
        Owner(OwnerCall),
        RegisterProductForId(RegisterProductForIdCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDecimals(SetDecimalsCall),
        SetEndpoint(SetEndpointCall),
        SetInsurance(SetInsuranceCall),
        SettlePnl(SettlePnlCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateFeeRates(UpdateFeeRatesCall),
        UpgradeClearinghouseLiq(UpgradeClearinghouseLiqCall),
        WithdrawCollateral(WithdrawCollateralCall),
    }
    impl ::ethers::core::abi::AbiDecode for ClearinghouseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddEngineCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddEngine(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnLp(decoded));
            }
            if let Ok(decoded) = <BurnLpAndTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnLpAndTransfer(decoded));
            }
            if let Ok(decoded) = <ClaimSequencerFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimSequencerFees(decoded));
            }
            if let Ok(decoded) = <DepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositCollateral(decoded));
            }
            if let Ok(decoded) = <DepositInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositInsurance(decoded));
            }
            if let Ok(decoded) = <GetAllBooksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllBooks(decoded));
            }
            if let Ok(decoded) = <GetClearinghouseLiqCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetClearinghouseLiq(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) = <GetEngineByProductCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEngineByProduct(decoded));
            }
            if let Ok(decoded) = <GetEngineByTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEngineByType(decoded));
            }
            if let Ok(decoded) = <GetHealthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealth(decoded));
            }
            if let Ok(decoded) = <GetInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInsurance(decoded));
            }
            if let Ok(decoded) = <GetMaxHealthGroupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxHealthGroup(decoded));
            }
            if let Ok(decoded) = <GetNumProductsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNumProducts(decoded));
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
            if let Ok(decoded) = <GetOrderbookCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOrderbook(decoded));
            }
            if let Ok(decoded) = <GetQuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuote(decoded));
            }
            if let Ok(decoded) = <GetRiskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRisk(decoded));
            }
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVersion(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <InsuranceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Insurance(decoded));
            }
            if let Ok(decoded) = <IsAboveInitialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAboveInitial(decoded));
            }
            if let Ok(decoded) = <IsUnderInitialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsUnderInitial(decoded));
            }
            if let Ok(decoded) = <LiqDecomposeLpsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiqDecomposeLps(decoded));
            }
            if let Ok(decoded) = <LiqFinalizeSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiqFinalizeSubaccount(decoded));
            }
            if let Ok(decoded) = <LiqLiquidationPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiqLiquidationPayment(decoded));
            }
            if let Ok(decoded) = <LiqSettleAgainstLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiqSettleAgainstLiquidator(decoded));
            }
            if let Ok(decoded) = <LiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidateSubaccount(decoded));
            }
            if let Ok(decoded) = <LiquidateSubaccountImplCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidateSubaccountImpl(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintLp(decoded));
            }
            if let Ok(decoded) = <MintLpSlowModeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintLpSlowMode(decoded));
            }
            if let Ok(decoded) = <ModifyProductConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyProductConfig(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RegisterProductForIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterProductForId(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDecimals(decoded));
            }
            if let Ok(decoded) = <SetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEndpoint(decoded));
            }
            if let Ok(decoded) = <SetInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetInsurance(decoded));
            }
            if let Ok(decoded) = <SettlePnlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SettlePnl(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateFeeRates(decoded));
            }
            if let Ok(decoded) = <UpgradeClearinghouseLiqCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeClearinghouseLiq(decoded));
            }
            if let Ok(decoded) = <WithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawCollateral(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ClearinghouseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddEngine(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnLpAndTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimSequencerFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllBooks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetClearinghouseLiq(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEngineByProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEngineByType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxHealthGroup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNumProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOraclePriceX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOraclePricesX18(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderbook(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Insurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAboveInitial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsUnderInitial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiqDecomposeLps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiqFinalizeSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiqLiquidationPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiqSettleAgainstLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidateSubaccountImpl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintLpSlowMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyProductConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterProductForId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SettlePnl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeClearinghouseLiq(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ClearinghouseCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddEngine(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLpAndTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimSequencerFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllBooks(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouseLiq(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineByProduct(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEngineByType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealth(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxHealthGroup(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNumProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOraclePricesX18(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOrderbook(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Insurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAboveInitial(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsUnderInitial(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiqDecomposeLps(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiqFinalizeSubaccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiqLiquidationPayment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiqSettleAgainstLiquidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateSubaccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidateSubaccountImpl(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintLpSlowMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyProductConfig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterProductForId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlePnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeClearinghouseLiq(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddEngineCall> for ClearinghouseCalls {
        fn from(value: AddEngineCall) -> Self {
            Self::AddEngine(value)
        }
    }
    impl ::core::convert::From<BurnLpCall> for ClearinghouseCalls {
        fn from(value: BurnLpCall) -> Self {
            Self::BurnLp(value)
        }
    }
    impl ::core::convert::From<BurnLpAndTransferCall> for ClearinghouseCalls {
        fn from(value: BurnLpAndTransferCall) -> Self {
            Self::BurnLpAndTransfer(value)
        }
    }
    impl ::core::convert::From<ClaimSequencerFeesCall> for ClearinghouseCalls {
        fn from(value: ClaimSequencerFeesCall) -> Self {
            Self::ClaimSequencerFees(value)
        }
    }
    impl ::core::convert::From<DepositCollateralCall> for ClearinghouseCalls {
        fn from(value: DepositCollateralCall) -> Self {
            Self::DepositCollateral(value)
        }
    }
    impl ::core::convert::From<DepositInsuranceCall> for ClearinghouseCalls {
        fn from(value: DepositInsuranceCall) -> Self {
            Self::DepositInsurance(value)
        }
    }
    impl ::core::convert::From<GetAllBooksCall> for ClearinghouseCalls {
        fn from(value: GetAllBooksCall) -> Self {
            Self::GetAllBooks(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseLiqCall> for ClearinghouseCalls {
        fn from(value: GetClearinghouseLiqCall) -> Self {
            Self::GetClearinghouseLiq(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for ClearinghouseCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetEngineByProductCall> for ClearinghouseCalls {
        fn from(value: GetEngineByProductCall) -> Self {
            Self::GetEngineByProduct(value)
        }
    }
    impl ::core::convert::From<GetEngineByTypeCall> for ClearinghouseCalls {
        fn from(value: GetEngineByTypeCall) -> Self {
            Self::GetEngineByType(value)
        }
    }
    impl ::core::convert::From<GetHealthCall> for ClearinghouseCalls {
        fn from(value: GetHealthCall) -> Self {
            Self::GetHealth(value)
        }
    }
    impl ::core::convert::From<GetInsuranceCall> for ClearinghouseCalls {
        fn from(value: GetInsuranceCall) -> Self {
            Self::GetInsurance(value)
        }
    }
    impl ::core::convert::From<GetMaxHealthGroupCall> for ClearinghouseCalls {
        fn from(value: GetMaxHealthGroupCall) -> Self {
            Self::GetMaxHealthGroup(value)
        }
    }
    impl ::core::convert::From<GetNumProductsCall> for ClearinghouseCalls {
        fn from(value: GetNumProductsCall) -> Self {
            Self::GetNumProducts(value)
        }
    }
    impl ::core::convert::From<GetOraclePriceX18Call> for ClearinghouseCalls {
        fn from(value: GetOraclePriceX18Call) -> Self {
            Self::GetOraclePriceX18(value)
        }
    }
    impl ::core::convert::From<GetOraclePricesX18Call> for ClearinghouseCalls {
        fn from(value: GetOraclePricesX18Call) -> Self {
            Self::GetOraclePricesX18(value)
        }
    }
    impl ::core::convert::From<GetOrderbookCall> for ClearinghouseCalls {
        fn from(value: GetOrderbookCall) -> Self {
            Self::GetOrderbook(value)
        }
    }
    impl ::core::convert::From<GetQuoteCall> for ClearinghouseCalls {
        fn from(value: GetQuoteCall) -> Self {
            Self::GetQuote(value)
        }
    }
    impl ::core::convert::From<GetRiskCall> for ClearinghouseCalls {
        fn from(value: GetRiskCall) -> Self {
            Self::GetRisk(value)
        }
    }
    impl ::core::convert::From<GetVersionCall> for ClearinghouseCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ClearinghouseCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InsuranceCall> for ClearinghouseCalls {
        fn from(value: InsuranceCall) -> Self {
            Self::Insurance(value)
        }
    }
    impl ::core::convert::From<IsAboveInitialCall> for ClearinghouseCalls {
        fn from(value: IsAboveInitialCall) -> Self {
            Self::IsAboveInitial(value)
        }
    }
    impl ::core::convert::From<IsUnderInitialCall> for ClearinghouseCalls {
        fn from(value: IsUnderInitialCall) -> Self {
            Self::IsUnderInitial(value)
        }
    }
    impl ::core::convert::From<LiqDecomposeLpsCall> for ClearinghouseCalls {
        fn from(value: LiqDecomposeLpsCall) -> Self {
            Self::LiqDecomposeLps(value)
        }
    }
    impl ::core::convert::From<LiqFinalizeSubaccountCall> for ClearinghouseCalls {
        fn from(value: LiqFinalizeSubaccountCall) -> Self {
            Self::LiqFinalizeSubaccount(value)
        }
    }
    impl ::core::convert::From<LiqLiquidationPaymentCall> for ClearinghouseCalls {
        fn from(value: LiqLiquidationPaymentCall) -> Self {
            Self::LiqLiquidationPayment(value)
        }
    }
    impl ::core::convert::From<LiqSettleAgainstLiquidatorCall> for ClearinghouseCalls {
        fn from(value: LiqSettleAgainstLiquidatorCall) -> Self {
            Self::LiqSettleAgainstLiquidator(value)
        }
    }
    impl ::core::convert::From<LiquidateSubaccountCall> for ClearinghouseCalls {
        fn from(value: LiquidateSubaccountCall) -> Self {
            Self::LiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<LiquidateSubaccountImplCall> for ClearinghouseCalls {
        fn from(value: LiquidateSubaccountImplCall) -> Self {
            Self::LiquidateSubaccountImpl(value)
        }
    }
    impl ::core::convert::From<MintLpCall> for ClearinghouseCalls {
        fn from(value: MintLpCall) -> Self {
            Self::MintLp(value)
        }
    }
    impl ::core::convert::From<MintLpSlowModeCall> for ClearinghouseCalls {
        fn from(value: MintLpSlowModeCall) -> Self {
            Self::MintLpSlowMode(value)
        }
    }
    impl ::core::convert::From<ModifyProductConfigCall> for ClearinghouseCalls {
        fn from(value: ModifyProductConfigCall) -> Self {
            Self::ModifyProductConfig(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ClearinghouseCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterProductForIdCall> for ClearinghouseCalls {
        fn from(value: RegisterProductForIdCall) -> Self {
            Self::RegisterProductForId(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ClearinghouseCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDecimalsCall> for ClearinghouseCalls {
        fn from(value: SetDecimalsCall) -> Self {
            Self::SetDecimals(value)
        }
    }
    impl ::core::convert::From<SetEndpointCall> for ClearinghouseCalls {
        fn from(value: SetEndpointCall) -> Self {
            Self::SetEndpoint(value)
        }
    }
    impl ::core::convert::From<SetInsuranceCall> for ClearinghouseCalls {
        fn from(value: SetInsuranceCall) -> Self {
            Self::SetInsurance(value)
        }
    }
    impl ::core::convert::From<SettlePnlCall> for ClearinghouseCalls {
        fn from(value: SettlePnlCall) -> Self {
            Self::SettlePnl(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ClearinghouseCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateFeeRatesCall> for ClearinghouseCalls {
        fn from(value: UpdateFeeRatesCall) -> Self {
            Self::UpdateFeeRates(value)
        }
    }
    impl ::core::convert::From<UpgradeClearinghouseLiqCall> for ClearinghouseCalls {
        fn from(value: UpgradeClearinghouseLiqCall) -> Self {
            Self::UpgradeClearinghouseLiq(value)
        }
    }
    impl ::core::convert::From<WithdrawCollateralCall> for ClearinghouseCalls {
        fn from(value: WithdrawCollateralCall) -> Self {
            Self::WithdrawCollateral(value)
        }
    }
    ///Container type for all return fields from the `getAllBooks` function with signature `getAllBooks()` and selector `0x354528e8`
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
    pub struct GetAllBooksReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getClearinghouseLiq` function with signature `getClearinghouseLiq()` and selector `0x9b0861c1`
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
    pub struct GetClearinghouseLiqReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getEngineByProduct` function with signature `getEngineByProduct(uint32)` and selector `0xdeb14ec3`
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
    pub struct GetEngineByProductReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `0x5d2e9ad1`
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
    pub struct GetEngineByTypeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `0x88b6496f`
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
    pub struct GetHealthReturn {
        pub health: i128,
    }
    ///Container type for all return fields from the `getInsurance` function with signature `getInsurance()` and selector `0x267a8da0`
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
    pub struct GetInsuranceReturn(pub i128);
    ///Container type for all return fields from the `getMaxHealthGroup` function with signature `getMaxHealthGroup()` and selector `0xf8fac037`
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
    pub struct GetMaxHealthGroupReturn(pub u32);
    ///Container type for all return fields from the `getNumProducts` function with signature `getNumProducts()` and selector `0x6bad77e1`
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
    pub struct GetNumProductsReturn(pub u32);
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
    ///Container type for all return fields from the `getOrderbook` function with signature `getOrderbook(uint32)` and selector `0x4427952d`
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
    pub struct GetOrderbookReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getQuote` function with signature `getQuote()` and selector `0x171755b1`
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
    pub struct GetQuoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
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
    pub struct GetRiskReturn(pub Risk);
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
    ///Container type for all return fields from the `insurance` function with signature `insurance()` and selector `0x89cf3204`
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
    pub struct InsuranceReturn(pub i128);
    ///Container type for all return fields from the `isAboveInitial` function with signature `isAboveInitial(bytes32)` and selector `0x56bc3c38`
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
    pub struct IsAboveInitialReturn(pub bool);
    ///Container type for all return fields from the `isUnderInitial` function with signature `isUnderInitial(bytes32)` and selector `0xb5fc6205`
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
    pub struct IsUnderInitialReturn(pub bool);
    ///Container type for all return fields from the `liqDecomposeLps` function with signature `liqDecomposeLps((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x4dafc1a6`
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
    pub struct LiqDecomposeLpsReturn(pub bool);
    ///Container type for all return fields from the `liqFinalizeSubaccount` function with signature `liqFinalizeSubaccount((bytes32,bytes32,uint8,uint32,int128,uint64))` and selector `0x81210112`
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
    pub struct LiqFinalizeSubaccountReturn(pub bool);
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
    ///Container type for all return fields from the `registerProductForId` function with signature `registerProductForId(address,(int32,int32,int32,int32,int32),uint32)` and selector `0x26df2414`
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
    pub struct RegisterProductForIdReturn {
        pub product_id: u32,
    }
    ///`RiskStore(int32,int32,int32,int32,int32)`
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
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub large_position_penalty: i32,
    }
    ///`BurnLp(bytes32,uint32,uint128,uint64)`
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
    pub struct BurnLp {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub nonce: u64,
    }
    ///`BurnLpAndTransfer(bytes32,uint32,uint128,bytes32)`
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
    pub struct BurnLpAndTransfer {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub recipient: [u8; 32],
    }
    ///`ClaimSequencerFees(bytes32)`
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
    pub struct ClaimSequencerFees {
        pub subaccount: [u8; 32],
    }
    ///`DepositCollateral(bytes32,uint32,uint128)`
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
    pub struct DepositCollateral {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
    }
    ///`DepositInsurance(uint128)`
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
    pub struct DepositInsurance {
        pub amount: u128,
    }
    ///`LiquidateSubaccount(bytes32,bytes32,uint8,uint32,int128,uint64)`
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
    pub struct LiquidateSubaccount {
        pub sender: [u8; 32],
        pub liquidatee: [u8; 32],
        pub mode: u8,
        pub health_group: u32,
        pub amount: i128,
        pub nonce: u64,
    }
    ///`MintLp(bytes32,uint32,uint128,uint128,uint128,uint64)`
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
    pub struct MintLp {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount_base: u128,
        pub quote_amount_low: u128,
        pub quote_amount_high: u128,
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
    ///`SettlePnl(bytes32[],uint256[])`
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
    pub struct SettlePnl {
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
        pub product_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`UpdateFeeRates(address,uint32,int64,int64)`
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
    pub struct UpdateFeeRates {
        pub user: ::ethers::core::types::Address,
        pub product_id: u32,
        pub maker_rate_x18: i64,
        pub taker_rate_x18: i64,
    }
    ///`WithdrawCollateral(bytes32,uint32,uint128,uint64)`
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
    pub struct WithdrawCollateral {
        pub sender: [u8; 32],
        pub product_id: u32,
        pub amount: u128,
        pub nonce: u64,
    }
    ///`Risk(int128,int128,int128,int128,int128)`
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
    pub struct Risk {
        pub long_weight_initial_x18: i128,
        pub short_weight_initial_x18: i128,
        pub long_weight_maintenance_x18: i128,
        pub short_weight_maintenance_x18: i128,
        pub large_position_penalty_x18: i128,
    }
}
