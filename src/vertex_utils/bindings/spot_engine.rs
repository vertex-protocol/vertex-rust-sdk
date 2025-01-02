pub use spot_engine::*;
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
pub mod spot_engine {
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
                                name: ::std::borrow::ToOwned::to_owned("config"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertUtilization"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![],
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConfig"),
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
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCoreRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCoreRisk"),
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
                                name: ::std::borrow::ToOwned::to_owned("healthType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.HealthType",
                                    ),
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
                                ::std::borrow::ToOwned::to_owned("struct IProductEngine.CoreRisk",),
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
                    ::std::borrow::ToOwned::to_owned("getHealthContribution"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHealthContribution",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("health"),
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
                    ::std::borrow::ToOwned::to_owned("getMinDepositRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinDepositRate"),
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
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balances",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.LpState",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRisk"),
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
                                ::std::borrow::ToOwned::to_owned("struct RiskHelper.Risk"),
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
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balancesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("statesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lpStatesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
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
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ],),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.LpState",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ISpotEngine.LpBalance",
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
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getToken"),
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
                    ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalBalances"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deposits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("borrows"),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWithdrawFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getWithdrawFee"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                name: ::std::borrow::ToOwned::to_owned("_offchainExchange"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("manualAssert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("manualAssert"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalDeposits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalBorrows"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrationFlag"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("migrationFlag"),
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
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("config"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                                ),
                            },
                        ],
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
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ISpotEngine.LpBalance",
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
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ],),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.LpState",),
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
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("socializeSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("socializeSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
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
                    ::std::borrow::ToOwned::to_owned("swapLp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISpotEngine.UpdateProductTx",
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
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISpotEngine.UpdateProductTx",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateBalance"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateBalance"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountDelta"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMinDepositRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateMinDepositRate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minDepositRateX18"),
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
                    ::std::borrow::ToOwned::to_owned("updatePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceX18"),
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
                    ::std::borrow::ToOwned::to_owned("updateProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rawTxn"),
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
                    ::std::borrow::ToOwned::to_owned("updateQuoteFromInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateQuoteFromInsurance",),
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
                    ::std::borrow::ToOwned::to_owned("updateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateRisk"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateStates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateStates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dt"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("InterestPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("InterestPayment"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositRateMultiplierX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("borrowRateMultiplierX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                        ],
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SPOTENGINE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pan\xCE\x80b\0\0!`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x15W`\x005`\xE0\x1C\x80c\xADs;\x8E\x11a\x01\xA7W\x80c\xE34\xBE3\x11a\0\xEEW\x80c\xF2\xFD\xE3\x8B\x11a\0\x97W\x80c\xF8\xA4.Q\x11a\0qW\x80c\xF8\xA4.Q\x14a\nRW\x80c\xFA\xB2\xC4i\x14a\neW\x80c\xFD\xF4\xA0\xC0\x14a\n\x85W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\n\x19W\x80c\xF3\x9E\xEB\x10\x14a\n,W\x80c\xF8f\x18\x84\x14a\n?W`\0\x80\xFD[\x80c\xECzy\xC9\x11a\0\xC8W\x80c\xECzy\xC9\x14a\x08yW\x80c\xEC\xD9\xCB\xA8\x14a\t\x10W\x80c\xED\xF0&S\x14a\ttW`\0\x80\xFD[\x80c\xE34\xBE3\x14a\x07\x92W\x80c\xE3Cs\x8C\x14a\x07\xB3W\x80c\xECbq\xD2\x14a\x08fW`\0\x80\xFD[\x80c\xC7\x16|\xF5\x11a\x01PW\x80c\xD3\x86\xC1\xE8\x11a\x01*W\x80c\xD3\x86\xC1\xE8\x14a\x07YW\x80c\xD9\x87R\xEC\x14a\x07lW\x80c\xE0\xB0b\x1F\x14a\x07\x7FW`\0\x80\xFD[\x80c\xC7\x16|\xF5\x14a\x07\x13W\x80c\xC7!\xBDe\x14a\x07&W\x80c\xC9\xFE\x9A\xC3\x14a\x07FW`\0\x80\xFD[\x80c\xB8\xD8\r\x8B\x11a\x01\x81W\x80c\xB8\xD8\r\x8B\x14a\x06\xC0W\x80c\xC3b\xD1\x9E\x14a\x06\xD3W\x80c\xC5V\x07\xB5\x14a\x07\0W`\0\x80\xFD[\x80c\xADs;\x8E\x14a\x06\x8BW\x80c\xAE\xD8\xE9g\x14a\x06\x9EW\x80c\xB1\xCB\x0FB\x14a\x06\xAFW`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x11a\x02kW\x80c\x8A\x1DC\xC9\x11a\x02\x14W\x80c\x98\xDEr\xFE\x11a\x01\xEEW\x80c\x98\xDEr\xFE\x14a\x06:W\x80c\x9B\xB9\x1Fj\x14a\x06MW\x80c\xA6z\xC3\"\x14a\x06`W`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x05\x98W\x80c\x8A\xF6Bj\x14a\x05\xD7W\x80c\x8D\xA5\xCB[\x14a\x06)W`\0\x80\xFD[\x80c\x7F\xA2\x9DI\x11a\x02EW\x80c\x7F\xA2\x9DI\x14a\x05_W\x80c\x87\x1D\t\x12\x14a\x05rW\x80c\x896\xF7\xCD\x14a\x05\x85W`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x14a\x05$W\x80cqP\x18\xA6\x14a\x057W\x80c|\x1E\x14\x87\x14a\x05?W`\0\x80\xFD[\x80c0\x97+P\x11a\x02\xCDW\x80cF\x04\xD1\x9B\x11a\x02\xA7W\x80cF\x04\xD1\x9B\x14a\x04)W\x80cGB\x8E{\x14a\x048W\x80cI\xF75h\x14a\x04MW`\0\x80\xFD[\x80c0\x97+P\x14a\x03\xACW\x80c=\\\xC9\xDC\x14a\x03\xBFW\x80cE\xBE~\xD6\x14a\x03\xE2W`\0\x80\xFD[\x80c\x14YEz\x11a\x02\xFEW\x80c\x14YEz\x14a\x03pW\x80c\x15<\xA6\xC0\x14a\x03\x83W\x80c+\xAFW\xD3\x14a\x03\x96W`\0\x80\xFD[\x80c\x10\xCA\x93\x88\x14a\x03\x1AW\x80c\x13\x0E\xA3s\x14a\x03/W[`\0\x80\xFD[a\x03-a\x03(6`\x04a\\\xC0V[a\n\x98V[\0[a\x03Xa\x03=6`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x90V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03-a\x03~6`\x04a](V[a\n\xBFV[a\x03-a\x03\x916`\x04a]\xA8V[a\r\x96V[a\x03\x9Ea\x0E[V[`@Qa\x03g\x92\x91\x90a^\x1FV[a\x03-a\x03\xBA6`\x04a^\x89V[a\x10\x86V[a\x03\xD2a\x03\xCD6`\x04a^\xF5V[a\x12gV[`@Qa\x03g\x94\x93\x92\x91\x90a_nV[a\x04\x11a\x03\xF06`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03gV[`\0`@Qa\x03g\x91\x90a_\xE9V[a\x04@a\x13\xB2V[`@Qa\x03g\x91\x90a`\x11V[a\x04`a\x04[6`\x04a`[V[a\x146V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x0F\x90\x81\x0B\x82\x84\x01R\x84\x84\x01Q\x81\x0B\x83\x85\x01R``\x80\x86\x01Q\x82\x0B\x81\x85\x01R`\x80\x80\x87\x01Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x01R\x80\x85\x01Q\x84\x0B`\xA0\x80\x88\x01\x91\x90\x91R\x81\x88\x01Q\x85\x0B`\xC0\x88\x01R\x81\x84\x01Q\x85\x0B`\xE0\x88\x01R\x90\x82\x01Q\x84\x0Ba\x01\0\x87\x01R\x90\x96\x01Q\x80Q`\x03\x90\x81\x0Ba\x01 \x87\x01R\x93\x81\x01Q\x84\x0Ba\x01@\x86\x01R\x94\x85\x01Q\x83\x0Ba\x01`\x85\x01R\x84\x01Q\x90\x91\x0Ba\x01\x80\x83\x01R\x91\x90\x92\x01Q\x90\x0Ba\x01\xA0\x82\x01Ra\x01\xC0\x01a\x03gV[a\x03-a\x0526`\x04a\\\xF6V[a\x14\xD0V[a\x03-a\x15gV[a\x05Ra\x05M6`\x04a^\xF5V[a\x15sV[`@Qa\x03g\x91\x90a`nV[a\x03-a\x05m6`\x04a`\x8FV[a\x16\x08V[a\x03Xa\x05\x806`\x04a`\xDFV[a\x16)V[a\x03-a\x05\x936`\x04aa\x02V[a\x17\xC9V[a\x05\xABa\x05\xA66`\x04aa\x1BV[a\x1CPV[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x03gV[a\x06\x0Fa\x05\xE56`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`n` R`@\x90 `\x02\x81\x01T`\x01\x90\x91\x01T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x03gV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[a\x03-a\x06H6`\x04aaYV[a\x1C\xC5V[a\x03-a\x06[6`\x04a\\\xC0V[a$'V[a\x06sa\x06n6`\x04a\\\xF6V[a$HV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x03gV[a\x03-a\x06\x996`\x04aa\xB3V[a$\xECV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[a\x03Xa\x06\xCE6`\x04aa\xDCV[a'\xF5V[`pTa\x06\xE7\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03gV[a\x03-a\x07\x0E6`\x04ac\x01V[a+\x81V[a\x06\x0Fa\x07!6`\x04ac.V[a,\xB2V[a\x079a\x0746`\x04a\\\xF6V[a/0V[`@Qa\x03g\x91\x90acyV[a\x03-a\x07T6`\x04ac\x87V[a/\xAFV[a\x03-a\x07g6`\x04ac\xF9V[a3}V[a\x06\x0Fa\x07z6`\x04ad\x9FV[a6\x86V[a\x03-a\x07\x8D6`\x04ad\x9FV[a<\xB2V[a\x07\xA5a\x07\xA06`\x04a^\xF5V[a=\xADV[`@Qa\x03g\x92\x91\x90ad\xD6V[a\x08Ya\x07\xC16`\x04a\\\xF6V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x90\x91\x01T\x80\x83\x0B``\x83\x01R\x92\x90\x92\x04\x90\x0B`\x80\x82\x01R\x90V[`@Qa\x03g\x91\x90ae.V[a\x03-a\x08t6`\x04a]\xA8V[a>lV[a\t\x03a\x08\x876`\x04a\\\xF6V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`l` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x03g\x91\x90ae|V[a\t#a\t\x1E6`\x04a\\\xF6V[a?jV[`@Qa\x03g\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\t\xF5a\t\x826`\x04a^\xF5V[`@\x80Q``\x80\x82\x01\x83R`\0\x82\x84\x01\x81\x81R\x83R\x83Q` \x80\x82\x01\x86R\x82\x82R\x93\x84\x01Rc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85R`m\x82R\x82\x85 \x93\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82R\x80T`\x0F\x90\x81\x0B\x84\x84\x01\x90\x81R\x84R\x82Q\x80\x86\x01\x90\x93R`\x01\x90\x91\x01T\x90\x0B\x81R\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82QQ`\x0F\x90\x81\x0B\x82R` \x93\x84\x01QQ\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x03gV[a\x03-a\n'6`\x04ae\xB7V[a?\x9EV[a\x03Xa\n:6`\x04ae\xD4V[a@&V[a\x03-a\nM6`\x04ae\xF9V[aA\xB2V[a\x03-a\n`6`\x04afBV[aA\xE1V[`@\x80Q`m\x81R`l` \x82\x01R`n\x91\x81\x01\x91\x90\x91R``\x01a\x03gV[a\x03Xa\n\x936`\x04a\\\xF6V[aDAV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\n\xB9\x82\x82af\x95V[PPPPV[a\n\xCB\x85\x85\x84\x84aE\x16V[`@\x80Q`\xA0\x80\x82\x01\x83R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x85\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x85\x87\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x88\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x8A\x01\x82\x81R`\0\x80\x80R`k\x88R\x9AQ\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9A\x16\x17\x90\x98U\x95Q\x93Q`\x01`\x01`\x80\x1B\x03\x94\x85\x16`\x01`\x80\x1B\x91\x86\x16\x82\x02\x17\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CU\x92Q\x96Q\x96\x84\x16\x96\x84\x16\x83\x02\x96\x90\x96\x17\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8DU\x88Q\x96\x87\x01\x89Rc;\x9A\xCA\0\x80\x88R\x87\x85\x01\x81\x81R\x88\x8B\x01\x82\x81R\x89\x84\x01\x92\x83R\x89\x88\x01\x89\x81R\x8B\x80R\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x88R\x99Q\x7F\xC1\x1D\xC3\x86\x04\x03\xBA\x02&?J\x19\ni\xCA\xB0[\x0E\xD2\xB0\xAD\xAE\x85U6}\x90\x9F\xC6]57\x80T\x93Q\x92Q\x94Q\x9BQ\x88\x16\x87\x02c\xFF\xFF\xFF\xFF\x9C\x8D\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x96\x8E\x16`\x01`@\x1B\x02\x96\x90\x96\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x94\x8E\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x93\x8E\x16\x93\x90\x93\x17\x94\x90\x94\x17\x92\x90\x92\x16\x17\x92\x90\x92\x17\x85\x16\x17\x90U\x88Q\x94\x85\x01\x89R\x85\x85R\x84\x84\x01\x95\x86R\x84\x89\x01\x88\x81R\x90\x85\x01\x88\x81R\x88\x80R`l\x85R\x94Q\x95Q\x83\x16\x82\x02\x95\x83\x16\x95\x90\x95\x17`\0\x80Q` anY\x839\x81Q\x91RU\x93Q\x92Q\x81\x16\x90\x93\x02\x91\x90\x92\x16\x17`\0\x80Q` any\x839\x81Q\x91RU`h\x80T`\x01\x81\x01\x82U\x90\x84R`\x08\x81\x04\x7F\xA2\x154 \xD8D\x92\x8BD!e\x02\x03\xC7{\xAB\xC8\xB3=\x7F.{E\x0E)f\xDB\x0C\"\twS\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x93\x02\x19\x16\x90\x91U\x91Q\x90\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[``\x80`\0`h\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xD8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\x9BW\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xF9Wa\x0E\xF9aa\xFEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F?Wa\x0F?aa\xFEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FhW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x10\x80W`\0\x82\x82\x81Q\x81\x10a\x0F\x8BWa\x0F\x8BagoV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`l\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0F\xFA\x92\x91aF\xB3\x16V[\x86\x84\x81Q\x81\x10a\x10\x0CWa\x10\x0CagoV[` \x02` \x01\x01\x81\x81Qa\x10 \x91\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x10@\x93P\x90\x91\x0B\x90aF\xB3V[\x85\x84\x81Q\x81\x10a\x10RWa\x10RagoV[` \x02` \x01\x01\x81\x81Qa\x10f\x91\x90ag\x9BV[`\x0F\x0B\x90RPa\x10y\x91P\x82\x90Pag\xEAV[\x90Pa\x0FnV[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x12`W`\0`h\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x10\xB8Wa\x10\xB8agoV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`l\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x11DWa\x11DagoV[\x90P` \x02\x01` \x81\x01\x90a\x11Y\x91\x90ah\x03V[`\x0F\x0Ba\x11z\x82`\0\x01Q\x83`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x11\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x11\xD6Wa\x11\xD6agoV[\x90P` \x02\x01` \x81\x01\x90a\x11\xEB\x91\x90ah\x03V[`\x0F\x0Ba\x12\x0C\x82` \x01Q\x83``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[PPP\x80a\x12Y\x90ahuV[\x90Pa\x10\x89V[PPPPPV[a\x12oa\\7V[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x83Q`\x80\x80\x82\x01\x86R\x82\x82R\x81\x84\x01\x83\x90R\x81\x86\x01\x83\x90R``\x80\x83\x01\x84\x90R\x86Q\x80\x88\x01\x88R\x84\x81R\x80\x86\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x8B\x16\x80\x86R`n\x87R\x88\x86 \x89Q\x80\x85\x01\x8BR\x81T`\x0F\x90\x81\x0B\x82R\x8BQ\x80\x8D\x01\x8DR`\x01\x80\x85\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8E\x01R\x84\x8D\x01\x92\x90\x92R\x8DQ\x80\x8F\x01\x8FR`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x82\x90\x04\x83\x0B\x85\x8D\x01R\x83\x8E\x01\x94\x90\x94R\x84\x8AR`l\x8BR\x8C\x8A \x8DQ\x98\x89\x01\x8ER\x80T\x80\x84\x0B\x8AR\x82\x90\x04\x83\x0B\x89\x8D\x01R\x84\x01T\x80\x83\x0B\x89\x8F\x01R\x04\x81\x0B\x87\x87\x01R\x92\x88R`m\x89R\x8A\x88 \x8D\x89R\x89R\x96\x8A\x90 \x8AQ\x94\x85\x01\x8BR\x80T\x83\x0B\x85\x8C\x01\x90\x81R\x85R\x8AQ\x80\x8A\x01\x90\x9BR\x01T\x90\x0B\x88R\x94\x81\x01\x87\x90R\x80Q\x95\x96\x92\x95\x91\x92\x90\x91\x84\x82\x85a\x13\x9D\x81\x85aG6V[\x92\x9E\x91\x9DP\x9BP\x90\x99P\x97PPPPPPPPV[```h\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14,W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x13\xEFW\x90P[PPPPP\x90P\x90V[a\x14\xBB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R\x84Q`\xA0\x81\x01\x86R\x83\x81R\x91\x82\x01\x83\x90R\x93\x81\x01\x82\x90R\x92\x83\x01\x81\x90R`\x80\x83\x81\x01\x91\x90\x91R\x90\x91\x90\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[a\x14\xCA6\x83\x90\x03\x83\x01\x83ah\x9BV[\x92\x91PPV[`\0a\x14\xDD\x82`\x01a=\xADV[P\x90P`\0a\x15\0\x82`\0\x01Q\x83`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x15\"\x83` \x01Q\x84``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x12`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[a\x15q`\0aG\x9DV[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x15\xFF\x82\x82aG6V[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`l` R`@\x90 \x81\x90a\n\xB9\x82\x82ai\x99V[`\0\x80a\x164a\x13\xB2V[\x90P\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x17\xC0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\x83Wa\x16\x83agoV[` \x02` \x01\x01Q\x90P`\0a\x16\x99\x82\x85aG\xEFV[\x90P`\0\x80a\x16\xA8\x84\x8BaI\x04V[\x91P\x91P`\0a\x16\xB9\x84\x84\x8CaI\x1DV[\x90Pa\x16\xC5\x82\x8Aag\x9BV[\x98P\x82`\x0F\x0B`\0\x14a\x17?Wa\x16\xE5g\r\xE0\xB6\xB3\xA7d\0\0`\x02aj$V[`\x0F\x0B\x81`\x0F\x0B\x03a\x17\x12Wo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x98PPPPPPPPPa\x14\xCAV[`\x80\x84\x01Qa\x172\x90a\x17)`\x0F\x86\x90\x0B\x84aF\xB3V[`\x0F\x0B\x90aF\xB3V[a\x17<\x90\x8Aag\x9BV[\x98P[PPP`\0\x80`\0a\x17Q\x85\x8CaI\xB4V[\x92P\x92P\x92P\x82`\x0F\x0B`\0\x14a\x17\xA8W`\0a\x17s\x84\x84\x87`\x80\x01QaJ\xCFV[\x90P\x81a\x17\x90a\x17\x85\x87`\x01\x8FaI\x1DV[`\x0F\x84\x90\x0B\x90aF\xB3V[a\x17\x9A\x91\x90ag\x9BV[a\x17\xA4\x90\x8Bag\x9BV[\x99PP[PPPPP\x80\x80a\x17\xB8\x90aj\xC2V[\x91PPa\x16ZV[PPP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0a\x18\x18a\x13\xB2V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1CKW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18LWa\x18LagoV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`l\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`m\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x18\xD1\x90\x83\x90aG6V[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x1C7W\x81Q`@\x83\x01Q`\0\x91a\x18\xFA\x91`\x0F\x0B\x90aF\xB3V[`@\x84\x01Q\x83Q\x91\x92Pa\x19\x1B\x91a\x19\x12\x90\x84ag\x9BV[`\x0F\x0B\x90aK\x0BV[`\x0F\x0B\x80\x84R`\0\x12a\x19-W`\0\x80\xFD[a\x19K\x83` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x19\\\x91\x90ag\x9BV[`\x0F\x0B\x90RPc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90Ua\x1A\xEDW`\0[\x86Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1A\xE7W`\0\x87\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\xC0Wa\x19\xC0agoV[` \x02` \x01\x01Q\x90P`\0c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x03a\x19\xE4WPa\x1A\xD7V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x82R\x86Q\x80\x88\x01\x88R`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x04\x90\x91\x0B\x93\x83\x01\x93\x90\x93R\x92\x83\x01RQa\x1Aa\x90\x87\x90aKtV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x83Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x84\x83\x01Q\x80Q\x90\x84\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x84\x01U\x93\x85\x01Q\x80Q\x93\x01Q\x92\x81\x16\x92\x16\x90\x92\x02\x17`\x02\x90\x91\x01Ua\x1A\xD4\x82aK\xEDV[PP[a\x1A\xE0\x81aj\xC2V[\x90Pa\x19\x97V[Pa\x1B\xDCV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x82\x90R\x90a\x1Bl\x90\x85\x90aKtV[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x83Q\x81T`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x82U\x84\x83\x01Q\x80Q\x90\x84\x01Q\x82\x16`\x01`\x80\x1B\x90\x81\x02\x91\x83\x16\x91\x90\x91\x17`\x01\x84\x01U\x94\x90\x93\x01Q\x80Q\x92\x01Q\x83\x16\x90\x93\x02\x91\x16\x17`\x02\x90\x91\x01U[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x91\x82\x90 \x85Q\x91\x86\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x86\x01Q``\x87\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua\x1C5\x84\x88aL)V[P[PPP\x80a\x1CD\x90ahuV[\x90Pa\x18\x1DV[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x1Cv\x84aLlV[\x90P`\0a\x1C\x84\x85\x87aI\x04V[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x1C\xB4\x84`\x01\x88aI\x1DV[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[`\0\x83`\x0F\x0B\x13\x80\x15a\x1C\xDBWP`\0\x82`\x0F\x0B\x13[\x80\x15a\x1C\xEAWP`\0\x81`\x0F\x0B\x13[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bILA`\xE8\x1B\x81RP\x90a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0a\x1D/aL\xC1V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x9F\x91\x90aj\xDBV[`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a\x1D\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x80\x82\x01\x84R\x82T`\x0F\x90\x81\x0B\x83R\x84Q\x80\x86\x01\x86R`\x01\x80\x86\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8A\x01R\x85\x89\x01\x92\x90\x92R\x86Q\x80\x88\x01\x88R`\x02\x90\x96\x01T\x80\x84\x0B\x87R\x82\x90\x04\x83\x0B\x86\x89\x01R\x84\x87\x01\x95\x86R\x98\x88R`l\x80\x88R\x86\x89 \x87Q`\x80\x80\x82\x01\x8AR\x82T\x80\x87\x0B\x83R\x85\x90\x04\x86\x0B\x82\x8C\x01R\x91\x90\x9B\x01T\x80\x85\x0B\x8C\x8A\x01R\x83\x90\x04\x84\x0B\x8B\x86\x01R\x89\x80R\x90\x88R\x86Q\x90\x81\x01\x87R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x84\x0B\x82R\x82\x90\x04\x83\x0B\x97\x81\x01\x97\x90\x97R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x96\x88\x01\x96\x90\x96R\x90\x94\x04\x84\x0B\x90\x85\x01R\x90QQ\x90\x94\x93\x91\x0B\x15a\x1F\x1BW`@\x84\x01QQ` \x85\x01QQa\x1F\x16\x91a\x1F\x0B\x91`\x0F\x0B\x90aK\x0BV[`\x0F\x89\x90\x0B\x90aF\xB3V[a\x1F6V[a\x1F6a\x1F'\x8AaLlV[`\x80\x01Q`\x0F\x89\x90\x0B\x90aF\xB3V[\x90P\x85`\x0F\x0B\x81`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\nj\x89`\xEB\x1B\x81RP\x90a\x1F|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x84`\x0F\x0B\x81`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\nj\x89`\xEB\x1B\x81RP\x90a\x1F\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x83Q`\0\x90`\x0F\x0B\x81\x03a\x1F\xE1Wa\x1F\xDA\x82\x89ag\x9BV[\x90Pa \0V[\x84Q`@\x86\x01QQa\x1F\xFD\x91\x90a\x17)\x90`\x0F\x8C\x90\x0B\x90aK\x0BV[\x90P[a \x0F\x84\x86`@\x01Q\x8AaM;V[a \x1E\x83\x86` \x01Q\x84aM;V[\x80\x85`\0\x01\x81\x81Qa 0\x91\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91Rc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x8E\x84R\x90\x91R\x81 `\x01\x01\x80T\x85\x94P\x90\x92a n\x91\x85\x91\x0Bag\x9BV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`\x80\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91Uc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x8AQ\x81T`\x01`\x01`\x80\x1B\x03\x19\x16\x90\x86\x16\x17\x81U\x8A\x83\x01Q\x80Q\x90\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17`\x01\x83\x01U\x8B\x83\x01Q\x80Q\x90\x85\x01Q\x90\x87\x16\x96\x16\x02\x94\x90\x94\x17`\x02\x90\x94\x01\x93\x90\x93U`m\x81R\x82\x82 \x8D\x83R\x81R\x82\x82 \x83Q\x80\x83\x01\x85R\x90T`\x0F\x90\x81\x0B\x82R\x8E\x84R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x83R\x92\x84\x90 \x84Q\x92\x83\x01\x90\x94R\x92T\x90\x91\x0B\x81R\x90\x91Pa!b\x86\x83a!]\x8DaknV[aN\xE1V[a!p\x85\x82a!]\x87aknV[\x81`m`\0\x8Ec\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x80`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x85`l`\0\x8Ec\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x84`l`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PPa$\x0E\x8C\x8CaL)V[a$\x19`\0\x8CaL)V[PPPPPPPPPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` R`@\x90 \x81\x90a\n\xB9\x82\x82ak\x94V[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a$\xE2\x92\x85\x92a$\xB5\x92\x91\x90aF\xB3\x16V[a$\xD3\x84`\0\x01Q\x85`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a$\xDD\x91\x90ak\xCBV[aOuV[a\x1C\xBE\x90\x83al\x1BV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a%YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra%\x8Bb\x01Q\x80`\x07alFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a%\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0[`hTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x1CKW`\0`h\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a&\tWa&\tagoV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`l\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x81a&\x83W\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a&\x99WPPa'\xE3V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x91\x90\x91Ra'\x18\x83\x83\x88aO\x91V[a'&\x82\x82`@\x01QaKtV[a'4\x85\x82` \x01QaKtV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x84Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x85\x84\x01Q\x80Q\x90\x85\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x80\x85\x01\x91\x90\x91U\x87\x85\x01Q\x80Q\x90\x87\x01Q\x90\x84\x16\x90\x84\x16\x83\x02\x17`\x02\x90\x94\x01\x93\x90\x93U`l\x85R\x94\x83\x90 \x87Q\x94\x88\x01Q\x94\x82\x16\x94\x82\x16\x86\x02\x94\x90\x94\x17\x84U\x91\x86\x01Q``\x87\x01Q\x90\x83\x16\x92\x16\x90\x93\x02\x17\x91\x01Ua'\xDF\x83aK\xEDV[PPP[\x80a'\xED\x81aj\xC2V[\x91PPa%\xDEV[`\0\x80a(\0a\x13\xB2V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a+yW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a(4Wa(4agoV[` \x02` \x01\x01Q\x90P`\0a(R\x82\x88`\x01`\x01`\x7F\x1B\x03a6\x86V[\x91PP\x80`\x0F\x0B`\0\x14a+fW`\0a(\x97`2a({a(s\x86aLlV[\x85`\x01aI\x1DV[a(\x8D\x90g\r\xE0\xB6\xB3\xA7d\0\0ak\xCBV[a\x17\x85\x91\x90al\x8CV[\x90P`\0a(\xB1`\x0F\x83\x90\x0Bg\x06\xF0[Y\xD3\xB2\0\0aF\xB3V[\x90Pa(\xBD\x81\x83ak\xCBV[\x91Pa(\xC9\x81\x88ag\x9BV[`@\x80Q`\x80\x81\x01\x82R`\0\x80Q` anY\x839\x81Q\x91RT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x84\x81\x01\x91\x90\x91R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x8E\x81R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x80\x84R\x85\x82 \x86Q\x80\x86\x01\x88R\x90T\x84\x0B\x81R\x8F\x83R\x90\x84R\x90\x85\x90 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x92\x99P\x91a)\x8E\x83\x83\x86a)\x84\x89aknV[a!]\x91\x90ak\xCBV[a)\x99\x83\x82\x87aN\xE1V[\x81`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x80`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x82`l`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PPa+U`\0\x8CaL)V[a+``\0\x8DaL)V[PPPPP[PP\x80a+r\x90ahuV[\x90Pa(\x05V[PP\x92\x91PPV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a+\xADWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a+\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92fc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x0F\x90\x81\x0B\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x88\x01R\x83\x87\x01\x91\x82R\x85Q\x80\x87\x01\x87R`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x04\x90\x91\x0B\x94\x83\x01\x94\x90\x94R\x91\x82\x01\x81\x90RQ\x91QQ\x83\x92a-7\x91\x87\x91\x87\x91aS\x9BV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a-pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`l` R`@\x80\x82 T\x91\x80R`\0\x80Q` anY\x839\x81Q\x91RT\x90\x83\x01Q\x80Q`\x0F\x93\x84\x0B\x93\x92\x90\x92\x0B\x91\x88\x91\x90a-\xBA\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RP` \x83\x01Q\x80Q\x87\x91\x90a-\xD5\x90\x83\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91Rc\xFF\xFF\xFF\xFF\x8A\x81\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x88Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x89\x83\x01Q\x80Q\x90\x84\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x84\x01U\x93\x8A\x01Q\x80Q\x93\x01Q\x92\x81\x16\x92\x16\x90\x92\x02\x17`\x02\x90\x91\x01Ua.Z\x92P\x90\x89\x90\x0B\x90\x84\x90aK\x0B\x16V[c\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`l` R`@\x81 `\x01\x01\x80T\x90\x91\x90a.\x85\x90\x84\x90`\x0F\x0Bag\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa.\xC2\x81\x87`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80\x80R`l` R`\0\x80Q` any\x839\x81Q\x91R\x80T\x90\x91\x90a.\xEE\x90\x84\x90`\x0F\x0Bag\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa/\x1E\x88aK\xEDV[\x86\x86\x94P\x94PPPP[\x93P\x93\x91PPV[a/8a\\7V[Pc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x91\x90\x91R\x90V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a0\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`\0a0*\x82\x84\x01\x84ah\x9BV[`\xA0\x81\x01Q\x81Q\x91\x92P\x90c\xFF\xFF\xFF\xFF\x16\x15a2\xFBW\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a0lWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a0\x9FWP`\x80\x82\x01QQ\x82Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a0\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x83Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T`\x03\x81\x81\x0B\x83Rd\x01\0\0\0\0\x82\x04\x81\x0B\x83\x86\x01\x90\x81R`\x01`@\x1B\x83\x04\x82\x0B\x84\x86\x01\x90\x81R`\x01``\x1B\x84\x04\x83\x0B``\x80\x87\x01\x91\x82R`\x01`\x80\x1B\x90\x95\x04`\x0F\x0B`\x80\x87\x01R\x89Q\x84\x0B\x86R\x96\x89\x01Q\x83\x0B\x90\x91R\x93\x87\x01Q\x81\x0B\x90\x93R\x85\x01Q\x90\x91\x0B\x90\x91R\x90P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x92\x83R`@\x90\x81\x90 \x84Q\x81T\x94\x86\x01Q\x92\x86\x01Q``\x87\x01Q`\x80\x90\x97\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x97\x86\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x87\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x87\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x98\x16\x93\x90\x96\x16\x92\x90\x92\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x92\x90\x92\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90Ua2_aL\xC1V[\x83Q` \x85\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qc-\xA1\xC5\x9B`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x94\x90\x94R`\0`D\x85\x01R`\x0F\x92\x83\x0B`d\x85\x01R\x82\x0B`\x84\x84\x01R\x90\x0B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c-\xA1\xC5\x9B\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a2\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a2\xF5W=`\0\x80>=`\0\xFD[PPPPP[P`\x80\x81\x81\x01Q\x91Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17`\x01\x83\x01U``\x85\x01Q\x94\x90\x93\x01Q\x93\x82\x16\x93\x90\x91\x16\x90\x91\x02\x91\x90\x91\x17`\x02\x90\x91\x01UPPV[c\xFF\xFF\xFF\xFF\x88\x16a3\x8DW`\0\x80\xFD[a3\xAA\x88\x88\x88\x88\x88\x88a3\xA56\x89\x90\x03\x89\x01\x89al\xD3V[aT9V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`k` R`@\x90 \x82\x90a3\xCB\x82\x82af\x95V[\x90PP`@Q\x80`\x80\x01`@R\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP`l`\0\x8Ac\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81RP\x81RP`n`\0\x8Ac\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`@\x82\x01Q\x81`\x02\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP\x90PPPPPPPPPPV[`\0\x80`\0\x83`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bILA`\xE8\x1B\x81RP\x90a6\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x80\x82\x01\x84R\x82T`\x0F\x90\x81\x0B\x83R\x84Q\x80\x86\x01\x86R`\x01\x80\x86\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8A\x01R\x85\x89\x01\x92\x90\x92R\x86Q\x80\x88\x01\x88R`\x02\x90\x96\x01T\x80\x84\x0B\x87R\x82\x90\x04\x83\x0B\x86\x89\x01R\x84\x87\x01\x95\x90\x95R\x88\x88R`m\x87R\x85\x88 \x8D\x89R\x87R\x85\x88 \x86Q\x80\x89\x01\x88R\x90\x86\x01T\x83\x0B\x81R\x98\x88R`l\x80\x88R\x86\x89 \x87Q`\x80\x80\x82\x01\x8AR\x82T\x80\x87\x0B\x83R\x85\x90\x04\x86\x0B\x82\x8C\x01R\x91\x90\x97\x01T\x80\x85\x0B\x88\x8A\x01R\x83\x90\x04\x84\x0B\x87\x86\x01R\x98\x80R\x87R\x85Q\x97\x88\x01\x86R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x83\x0B\x89R\x81\x90\x04\x82\x0B\x96\x88\x01\x96\x90\x96R`\0\x80Q` any\x839\x81Q\x91RT\x80\x82\x0B\x95\x88\x01\x95\x90\x95R\x94\x90\x93\x04\x84\x0B\x92\x85\x01\x92\x90\x92R\x90\x93\x92\x90\x91\x87\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19\x01a8\x11W\x82Q\x96P[\x86`\x0F\x0B`\0\x03a8,W`\0\x80\x95P\x95PPPPPa/(V[\x86`\x0F\x0B\x83`\0\x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x94\xC5`\xEC\x1B\x81RP\x90a8tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x86\x83`\0\x01\x81\x81Qa8\x87\x91\x90ak\xCBV[`\x0F\x90\x81\x0B\x90\x91R\x85Q`@\x87\x01QQ\x90\x82\x0B\x92Pa8\xAC\x91\x90\x81\x0B\x90\x8A\x90\x0Bal\xEFV[a8\xB6\x91\x90amvV[\x95P\x83`\0\x01Q`\x0F\x0B\x84` \x01Q`\0\x01Q`\x0F\x0B\x88`\x0F\x0Ba8\xDA\x91\x90al\xEFV[a8\xE4\x91\x90amvV[\x94Pa8\xFE\x82\x85`@\x01Q\x88a8\xF9\x90aknV[aM;V[a9\x11\x81\x85` \x01Q\x87a8\xF9\x90aknV[\x86\x84`\0\x01\x81\x81Qa9#\x91\x90ak\xCBV[\x91P\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x83`n`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`@\x82\x01Q\x81`\x02\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP\x90PP\x82`m`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP`\0`m`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80` \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B`\x0F\x0B`\x0F\x0B\x81RPP\x90P`\0`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80` \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B`\x0F\x0B`\x0F\x0B\x81RPP\x90Pa;\x8B\x84\x83\x8AaN\xE1V[a;\x96\x83\x82\x89aN\xE1V[c\xFF\xFF\xFF\xFF\x8B\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8E\x84R\x82R\x80\x83 \x86Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x17\x90\x92U\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x84R\x82\x85 \x87Q\x81T\x90\x93\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U\x93\x83R`l\x82R\x80\x83 \x88Q\x89\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17\x82U\x89\x83\x01Q``\x80\x8C\x01Q\x91\x88\x16\x91\x88\x16\x83\x02\x91\x90\x91\x17`\x01\x93\x90\x93\x01\x92\x90\x92U\x93\x80R\x87Q\x92\x88\x01Q\x92\x85\x16\x92\x85\x16\x84\x02\x92\x90\x92\x17`\0\x80Q` anY\x839\x81Q\x91RU\x86\x01Q\x90\x86\x01Q\x90\x83\x16\x92\x16\x02\x17`\0\x80Q` any\x839\x81Q\x91RUa<\x99\x8B\x8BaL)V[a<\xA4`\0\x8BaL)V[PPPPPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra=+\x82\x82\x85aN\xE1V[c\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x85Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x90\x91U\x93\x83R`l\x82R\x91\x82\x90 \x85Q\x91\x86\x01Q\x91\x84\x16`\x01`\x80\x1B\x92\x85\x16\x83\x02\x17\x81U\x91\x85\x01Q``\x86\x01Q\x90\x84\x16\x93\x16\x02\x91\x90\x91\x17`\x01\x91\x90\x91\x01Ua\x12`\x85\x85aL)V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a>^\x81\x83aG6V[\x93P\x93PPP[\x92P\x92\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a>\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`\0\x81`\x0F\x0B\x12\x15\x80\x15a>\xF9WPg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x82\x90\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a?3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`q` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x14\xCA\x82aLlV[`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xFFV[a@#\x81aG\x9DV[PV[`@\x80Q`\x80\x81\x01\x82R`\0\x80Q` anY\x839\x81Q\x91RT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a@\xC1\x83\x83aG6V[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15aA\nW`\0a@\xEFa@\xE8\x87a@\xE3\x85aknV[aY\x03V[`\0aOuV[\x90Pa@\xFB\x81\x87ak\xCBV[\x95PaA\x08\x84\x84\x83aN\xE1V[P[P`\0\x80\x80R`l` \x90\x81R\x83Q\x84\x82\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\0\x80Q` anY\x839\x81Q\x91RU`@\x80\x87\x01Q``\x90\x97\x01Q\x96\x83\x16\x96\x83\x16\x90\x91\x02\x95\x90\x95\x17`\0\x80Q` any\x839\x81Q\x91RU\x96\x82R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x90R\x91\x90\x91 \x90Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16\x94\x16\x93\x90\x93\x17\x90\x92U\x91\x90PV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81\x90`\x01\x01a\x12`\x82\x82am\xA4V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16aB\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R`\0\x80Q` any\x839\x81Q\x91RT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x95\x85R`m\x84R\x82\x85 \x89\x86R\x84R\x82\x85 \x83Q\x80\x86\x01\x85R\x90T\x83\x0B\x81R\x89\x86R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x85R\x94\x83\x90 \x83Q\x94\x85\x01\x90\x93R\x91T\x90\x0B\x82R\x92\x91\x90aC\x1E\x84\x83\x88aN\xE1V[aC)\x83\x82\x87aN\xE1V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x86Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x17\x90\x92U\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x84R\x82\x85 \x87Q\x81T\x90\x93\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U\x93\x83R`l\x82R\x80\x83 \x88Q\x89\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17\x82U\x89\x83\x01Q``\x80\x8C\x01Q\x91\x88\x16\x91\x88\x16\x83\x02\x91\x90\x91\x17`\x01\x93\x90\x93\x01\x92\x90\x92U\x93\x80R\x87Q\x92\x88\x01Q\x92\x85\x16\x92\x85\x16\x84\x02\x92\x90\x92\x17`\0\x80Q` anY\x839\x81Q\x91RU\x86\x01Q\x90\x86\x01Q\x90\x83\x16\x92\x16\x02\x17`\0\x80Q` any\x839\x81Q\x91RUaD,\x88\x88aL)V[aD7`\0\x88aL)V[PPPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x15\x80aD\\WP\x81c\xFF\xFF\xFF\xFF\x16`\x05\x14[\x80aDmWP\x81c\xFF\xFF\xFF\xFF\x16`\x1F\x14[\x80aD~WP\x81c\xFF\xFF\xFF\xFF\x16`)\x14[\x80aD\x8FWP\x81c\xFF\xFF\xFF\xFF\x16`m\x14[\x15aD\xA3WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[\x81c\xFF\xFF\xFF\xFF\x16`\x01\x03aD\xBEWPe$a9\xCA\x80\0\x91\x90PV[\x81c\xFF\xFF\xFF\xFF\x16`\x03\x14\x80aD\xD9WP\x81c\xFF\xFF\xFF\xFF\x16`[\x14[\x80aD\xEAWP\x81c\xFF\xFF\xFF\xFF\x16`]\x14[\x80aD\xFBWP\x81c\xFF\xFF\xFF\xFF\x16`o\x14[\x15aE\x0EWPf\x02!\xB2b\xDD\x80\0\x91\x90PV[P`\0\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15aE6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80aEPWP0;\x15\x80\x15aEPWP`\0T`\xFF\x16`\x01\x14[aE\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xFFV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15aE\xE5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[aE\xEDaY\x18V[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UaF\x11\x82a?\x9EV[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`j` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x12`W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\r\x87V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90aF\xF5WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90aG.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aGcWP\x82QaGjV[P` \x83\x01Q[`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81\x90aG\x85\x90`\x0F\x0B\x84aF\xB3V[`\x0F\x0B\x81R` \x01\x82`\x0F\x0B\x81RP\x91PP\x92\x91PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x83\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x03\x81\x81\x0B\x80\x84Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x81\x04\x90\x93\x0B``\x82\x01R`\x01`\x80\x1B\x90\x92\x04`\x0F\x0B`\x80\x83\x01RaH\x91\x90c;\x9A\xCA\0aj$V[`\x0F\x0B\x82R` \x81\x01QaH\xAC\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x0B` \x83\x01R`@\x81\x01QaH\xCA\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x0B`@\x83\x01R``\x81\x01QaH\xE8\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x92\x91PPV[`\0\x80aI\x11\x84\x84a\x15sV[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15aI3WaI3a_\xD3V[\x03aIGWPg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xBEV[`\0\x80\x84`\x0F\x0B\x12aI\x80W`\0\x83`\x02\x81\x11\x15aIgWaIga_\xD3V[\x14aIvW\x84`@\x01QaIyV[\x84Q[\x90PaI\xACV[`\0\x83`\x02\x81\x11\x15aI\x94WaI\x94a_\xD3V[\x14aI\xA3W\x84``\x01QaI\xA9V[\x84` \x01Q[\x90P[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91R`\x01\x01T`\x0F\x0B\x80\x82R\x82\x91\x82\x91\x82\x03aJ\x01W`\0\x80`\0\x93P\x93P\x93PPaJ\xC8V[c\xFF\xFF\xFF\xFF\x80\x87\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x0F\x90\x81\x0B\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x88\x01R\x83\x87\x01\x91\x90\x91R\x84Q\x80\x86\x01\x86R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x81\x0B\x94\x83\x01\x94\x90\x94R\x91\x82\x01R\x80Q\x85Q\x91\x94aJ\x87\x93\x92\x90\x92\x0B\x91aK\x0B\x16V[`@\x83\x01QQ\x90\x91P`\0\x90aJ\xA0\x90`\x0F\x0B\x83aF\xB3V[` \x84\x01QQ\x90\x91P`\0\x90aJ\xB9\x90`\x0F\x0B\x84aF\xB3V[\x91\x97P\x90\x95P`\0\x94PPPPP[\x92P\x92P\x92V[`\0aK\0\x83`\x0F\x0BaJ\xEE\x84\x87`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0BaJ\xFB\x91\x90al\xEFV[aY\x8BV[aI\xAC\x90`\x02aj$V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90aKOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81aF\xCAWaF\xCAalvV[\x80Q`\x0F\x0B`\0\x03aK\x8DW\x90Q`\x0F\x0B` \x90\x91\x01RV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15aK\xA6WP\x81QaK\xADV[P` \x82\x01Q[\x80`\x0F\x0B\x82` \x01Q`\x0F\x0B\x03aK\xC3WPPPV[` \x82\x01Q\x82QaK\xDC\x91\x90a\x19\x12\x90`\x0F\x0B\x84aF\xB3V[`\x0F\x90\x81\x0B\x83R\x0B` \x90\x91\x01RPV[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x14\xCA\x82\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92faG\xEFV[`\0aL\xD5`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM6\x91\x90am\xCDV[\x90P\x90V[\x81Q`\x0F\x0B\x15\x80\x15aMRWP` \x82\x01Q`\x0F\x0B\x15[\x15aMfWg\r\xE0\xB6\xB3\xA7d\0\0` \x83\x01R[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aM\xB0WaM\x94\x82` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83`@\x01\x81\x81QaM\xA5\x91\x90ak\xCBV[`\x0F\x0B\x90RPaM\xE6V[aM\xCE\x82` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81QaM\xDF\x91\x90ag\x9BV[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aM\xFFWP\x82QaN\x06V[P` \x83\x01Q[\x81aN/aN$\x85` \x01Q\x84`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q`\x0F\x0B\x90aF\xB3V[aN9\x91\x90ag\x9BV[`\x0F\x0B\x80\x84R`\0\x12\x15aNOWP\x82QaNVV[P` \x83\x01Q[`\x0F\x81\x81\x0B` \x85\x01R\x83Q`\0\x91\x0B\x13\x15aN\xA6WaN\x8A\x83` \x01Q\x84`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84`@\x01\x81\x81QaN\x9B\x91\x90ag\x9BV[`\x0F\x0B\x90RPa\n\xB9V[aN\xC4\x83` \x01Q\x84`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84``\x01\x81\x81QaN\xD5\x91\x90ak\xCBV[`\x0F\x0B\x90RPPPPPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aO\x0FW\x81Q`@\x84\x01\x80QaO\x04\x90\x83\x90ak\xCBV[`\x0F\x0B\x90RPaO)V[\x81Q``\x84\x01\x80QaO\"\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RP[aO4\x83\x83\x83aZKV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aObW\x81Q`@\x84\x01\x80QaOW\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RPPPPV[\x81Q``\x84\x01\x80QaOW\x90\x83\x90ak\xCBV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13aO\x8AW\x81a\x1C\xBEV[P\x90\x91\x90PV[`\0\x80aO\xB2\x84`\0\x01Q\x85`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0aO\xD4\x85` \x01Q\x86``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0aO\xE6`\x0F\x83\x90\x0B\x84aK\x0BV[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x90\x92\x01T\x80\x85\x0B``\x83\x01R\x91\x90\x91\x04\x83\x0B`\x80\x82\x01R\x93\x94P\x91\x90\x84\x90\x0B\x90\x03aPdWP`\0aQ\0V[\x81` \x01Q`\x0F\x0B\x83`\x0F\x0B\x12\x15aP\xA9WaP\x98\x82` \x01Qa\x19\x12\x85\x85``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[aP\xA2\x90\x82ag\x9BV[\x90PaQ\0V[aP\xE4aP\xD6\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0aP\xC7\x91\x90ak\xCBV[` \x85\x01Qa\x19\x12\x90\x87ak\xCBV[`\x80\x84\x01Q`\x0F\x0B\x90aF\xB3V[\x82``\x01QaP\xF3\x91\x90ag\x9BV[aP\xFD\x90\x82ag\x9BV[\x90P[aQ\x1BaQ\x10c\x01\xE13\x80aZ\xC2V[`\x0F\x83\x90\x0B\x90aK\x0BV[\x90PaQ<\x87aQ3\x83g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[`\x0F\x0B\x90a[;V[\x95PPP`\0aQYg\r\xE0\xB6\xB3\xA7d\0\0\x86a\x17\x85\x91\x90ak\xCBV[\x90P`\0aQza\x17\x85g\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0ak\xCBV[\x90P`\0aQ\x96aQ\x8B\x83\x85ak\xCBV[`\x0F\x88\x90\x0B\x90aF\xB3V[` \x8A\x01Q\x90\x91PaQ\xAB\x90`\x0F\x0B\x88aF\xB3V[`\x0F\x0B` \x8A\x01R`\0aQ\xC7\x83g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[\x8AQ\x90\x91PaQ\xD9\x90`\x0F\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B\x8BR\x82\x90\x0B\x15aRlWc\xFF\xFF\xFF\xFF\x8B\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81RaR#\x8B\x82\x85aN\xE1V[c\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x83\x80R\x90\x91R\x81 \x82Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x90UaRj\x90\x8D\x90aL)V[P[c\xFF\xFF\xFF\xFF\x8B\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x15aS-W`\0aR\xBEaR\x9Cc\x01\xE13\x80aZ\xC2V[c\xFF\xFF\xFF\xFF\x80\x8F\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x91\x90aK\x0B\x16V[\x90P`\0aR\xD8\x8BaQ3\x84g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[` \x8D\x01Q\x90\x91PaR\xED\x90`\x0F\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B` \x8E\x01R\x8CQaS\x04\x91\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B\x8DRaS\x18\x90\x84\x90\x0B\x82aF\xB3V[\x92PaS(`\x0F\x8B\x90\x0B\x82aF\xB3V[\x99PPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`\x80\x1B\x03\x8B\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8A\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPV[`\0\x82`\x0F\x0B`\0\x14\x80aS\xB2WP\x81`\x0F\x0B`\0\x14[\x80aS\xCAWP`\0aS\xC4\x86\x85ag\x9BV[`\x0F\x0B\x13\x15[\x80aS\xE2WP`\0aS\xDC\x85\x84ag\x9BV[`\x0F\x0B\x13\x15[\x15aS\xEFWP`\0aI\xACV[`\0\x82`\x0F\x0B\x84`\x0F\x0BaT\x03\x91\x90al\xEFV[\x90P`\0aT\x11\x86\x85ag\x9BV[`\x0F\x0BaT\x1E\x88\x87ag\x9BV[`\x0F\x0BaT+\x91\x90al\xEFV[\x91\x90\x91\x13\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16aTLW`\0\x80\xFD[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15aTuWPc;\x9A\xCA\0\x81`@\x01Q`\x03\x0B\x13\x15[\x80\x15aT\x8FWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15aT\xA6WPc;\x9A\xCA\0\x81``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90aT\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92fc\xFF\xFF\xFF\xFF\x89\x81\x16`\0\x81\x81R` \x93\x84R`@\x90\x81\x90 \x85Q\x81T\x95\x87\x01Q\x87\x84\x01Q``\x89\x01Q`\x80\x90\x99\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x99\x88\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x89\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x89\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x9A\x16\x94\x90\x98\x16\x93\x90\x93\x17\x97\x90\x97\x17\x16\x94\x90\x94\x17\x94\x90\x94\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91U`fT\x90QcC\xB1j\x11`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x87b\xD4\"\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aU\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15aV\x03W=`\0\x80>=`\0\xFD[PPPP`h\x87\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x01`h\x80T\x90PaVi\x91\x90am\xEAV[\x90P[\x80\x15aX,W`haV\x7F`\x01\x83am\xEAV[\x81T\x81\x10aV\x8FWaV\x8FagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16`h\x82\x81T\x81\x10aV\xCEWaV\xCEagoV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15aX\x15W`\0`h\x82\x81T\x81\x10aW\rWaW\ragoV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`haW@`\x01\x84am\xEAV[\x81T\x81\x10aWPWaWPagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16`h\x83\x81T\x81\x10aW\x89WaW\x89agoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`h`\x01\x84aW\xCC\x91\x90am\xEAV[\x81T\x81\x10aW\xDCWaW\xDCagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPaX\x1AV[aX,V[\x80aX$\x81an\x01V[\x91PPaVlV[PaX5aL\xC1V[`@Qc-\xA1\xC5\x9B`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04\x83\x01R\x88\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`D\x83\x01R`\x0F\x87\x81\x0B`d\x84\x01R\x86\x81\x0B`\x84\x84\x01R\x85\x90\x0B`\xA4\x83\x01R\x91\x90\x91\x16\x90c-\xA1\xC5\x9B\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aX\xA8W`\0\x80\xFD[PZ\xF1\x15\x80\x15aX\xBCW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12aO\x8AW\x81a\x1C\xBEV[`\0Ta\x01\0\x90\x04`\xFF\x16aY\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\r\xFFV[a\x15qa[\xC3V[`\0\x80\x82\x12\x15aY\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xFFV[`\x03\x82\x13\x15aZ<WP\x80`\0aY\xF5`\x02\x83amvV[aZ\0\x90`\x01an\x18V[\x90P[\x81\x81\x12\x15aZ6W\x90P\x80`\x02\x81aZ\x1B\x81\x86amvV[aZ%\x91\x90an\x18V[aZ/\x91\x90amvV[\x90PaZ\x03V[P\x91\x90PV[\x81\x15aZFWP`\x01[\x91\x90PV[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aZdWP\x82QaZkV[P` \x83\x01Q[\x82Q`\0\x90\x83\x90aZ\x7F\x90`\x0F\x0B\x84aF\xB3V[aZ\x89\x91\x90ag\x9BV[\x90P`\0\x81`\x0F\x0B\x13\x15aZ\xA0W\x84Q\x91PaZ\xA8V[\x84` \x01Q\x91P[aZ\xB6`\x0F\x82\x90\x0B\x83aK\x0BV[`\x0F\x0B\x90\x93RPPPPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90aZ\xFBWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a[4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a[}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13aG.W\x80\x84\x16`\x0F\x0B\x15a[\xAFWa[\xAC\x82\x86aF\xB3V[\x91P[a[\xB9\x85\x86aF\xB3V[\x94P`\x02\x02a[\x8AV[`\0Ta\x01\0\x90\x04`\xFF\x16a\\.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\r\xFFV[a\x15q3aG\x9DV[`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a\\o`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R` \x01a\\\x97`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x90R\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a@#W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15aZ6W`\0\x80\xFD[`\0\x80`\xC0\x83\x85\x03\x12\x15a\\\xD3W`\0\x80\xFD[\x825a\\\xDE\x81a\\\x9CV[\x91Pa\\\xED\x84` \x85\x01a\\\xAEV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a]\x08W`\0\x80\xFD[\x815a\x1C\xBE\x81a\\\x9CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a@#W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a]@W`\0\x80\xFD[\x855a]K\x81a]\x13V[\x94P` \x86\x015a][\x81a]\x13V[\x93P`@\x86\x015a]k\x81a]\x13V[\x92P``\x86\x015a]{\x81a]\x13V[\x91P`\x80\x86\x015a]\x8B\x81a]\x13V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x80`\x0F\x0B\x81\x14a@#W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a]\xBBW`\0\x80\xFD[\x825a]\xC6\x81a\\\x9CV[\x91P` \x83\x015a]\xD6\x81a]\x99V[\x80\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a^\x14W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a]\xF5V[P\x94\x95\x94PPPPPV[`@\x81R`\0a^2`@\x83\x01\x85a]\xE1V[\x82\x81\x03` \x84\x01Ra\x15\xFF\x81\x85a]\xE1V[`\0\x80\x83`\x1F\x84\x01\x12a^VW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^nW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a>eW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a^\x9FW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a^\xB7W`\0\x80\xFD[a^\xC3\x88\x83\x89\x01a^DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a^\xDCW`\0\x80\xFD[Pa^\xE9\x87\x82\x88\x01a^DV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a_\x08W`\0\x80\xFD[\x825a_\x13\x81a\\\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa_M` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[a\x01\x80\x81\x01a_}\x82\x87a_!V[\x84Q`\x0F\x90\x81\x0B`\xA0\x84\x01R\x84Q\x81\x0B`\xC0\x84\x01R` \x80\x86\x01Q\x82\x0B`\xE0\x85\x01R`@\x86\x01Q\x82\x0Ba\x01\0\x85\x01R``\x86\x01Q\x82\x0Ba\x01 \x85\x01R\x84Q\x82\x0Ba\x01@\x85\x01R\x84\x01Q\x90\x0Ba\x01`\x83\x01Ra\x15\xFFV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a`\x0BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a`OW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a`-V[P\x90\x96\x95PPPPPPV[`\0a\x01\xC0\x82\x84\x03\x12\x15aZ6W`\0\x80\xFD[`@\x81\x01a\x14\xCA\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a`\xA3W`\0\x80\xFD[\x835a`\xAE\x81a\\\x9CV[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a`\xC2W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10aZFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a`\xF2W`\0\x80\xFD[\x825\x91Pa\\\xED` \x84\x01a`\xD0V[`\0` \x82\x84\x03\x12\x15aa\x14W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aa0W`\0\x80\xFD[\x835\x92P` \x84\x015aaB\x81a\\\x9CV[\x91PaaP`@\x85\x01a`\xD0V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aaqW`\0\x80\xFD[\x855aa|\x81a\\\x9CV[\x94P` \x86\x015\x93P`@\x86\x015aa\x93\x81a]\x99V[\x92P``\x86\x015aa\xA3\x81a]\x99V[\x91P`\x80\x86\x015a]\x8B\x81a]\x99V[`\0` \x82\x84\x03\x12\x15aa\xC5W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\xBEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aa\xEFW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15abEWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15abEWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14aZFW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15ab\xA0W`\0\x80\xFD[ab\xA8ab\x14V[\x90Pab\xB3\x82ab|V[\x81Rab\xC1` \x83\x01ab|V[` \x82\x01Rab\xD2`@\x83\x01ab|V[`@\x82\x01Rab\xE3``\x83\x01ab|V[``\x82\x01R`\x80\x82\x015ab\xF6\x81a]\x99V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15ac\x14W`\0\x80\xFD[\x825ac\x1F\x81a\\\x9CV[\x91Pa\\\xED\x84` \x85\x01ab\x8EV[`\0\x80`\0``\x84\x86\x03\x12\x15acCW`\0\x80\xFD[\x835acN\x81a\\\x9CV[\x92P` \x84\x015ac^\x81a]\x99V[\x91P`@\x84\x015acn\x81a]\x99V[\x80\x91PP\x92P\x92P\x92V[`\xA0\x81\x01a\x14\xCA\x82\x84a_!V[`\0\x80` \x83\x85\x03\x12\x15ac\x9AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15ac\xB2W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12ac\xC6W`\0\x80\xFD[\x815\x81\x81\x11\x15ac\xD5W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15ac\xE7W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x02\0\x89\x8B\x03\x12\x15ad\x16W`\0\x80\xFD[\x885ad!\x81a\\\x9CV[\x97P` \x89\x015ad1\x81a\\\x9CV[\x96P`@\x89\x015adA\x81a]\x13V[\x95P``\x89\x015adQ\x81a]\x99V[\x94P`\x80\x89\x015ada\x81a]\x99V[\x93P`\xA0\x89\x015adq\x81a]\x99V[\x92Pad\x80\x8A`\xC0\x8B\x01a\\\xAEV[\x91Pad\x90\x8Aa\x01`\x8B\x01a\\\xAEV[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15ad\xB4W`\0\x80\xFD[\x835ad\xBF\x81a\\\x9CV[\x92P` \x84\x015\x91P`@\x84\x015acn\x81a]\x99V[`\xC0\x81\x01ae\x11\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\x80\x84\x01R` \x84\x01Q\x90\x0B`\xA0\x83\x01Ra\x1C\xBEV[`\xA0\x81\x01a\x14\xCA\x82\x84`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\x80\x81\x01a\x14\xCA\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15ae\xC9W`\0\x80\xFD[\x815a\x1C\xBE\x81a]\x13V[`\0\x80`@\x83\x85\x03\x12\x15ae\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015a]\xD6\x81a]\x99V[`\0\x80`\0\x83\x85\x03``\x81\x12\x15af\x0FW`\0\x80\xFD[\x845af\x1A\x81a\\\x9CV[\x93P` \x85\x81\x015\x93P`?\x19\x82\x01\x12\x15af4W`\0\x80\xFD[P`@\x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15afXW`\0\x80\xFD[\x845afc\x81a\\\x9CV[\x93P` \x85\x015\x92P`@\x85\x015afz\x81a]\x99V[\x91P``\x85\x015af\x8A\x81a]\x99V[\x93\x96\x92\x95P\x90\x93PPV[\x815af\xA0\x81a]\x13V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015af\xCC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015af\xF4\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015ag%\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015agM\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\n\xB9V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15ag\xC5Wag\xC5ag\x85V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15ag\xE1Wag\xE1ag\x85V[P\x01\x93\x92PPPV[`\0`\x01\x82\x01ag\xFCWag\xFCag\x85V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15ah\x15W`\0\x80\xFD[\x815a\x1C\xBE\x81a]\x99V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15ahMW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01ah1V[\x81\x81\x11\x15ah_W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03ah\x91Wah\x91ag\x85V[`\x01\x01\x93\x92PPPV[`\0\x81\x83\x03a\x01\xC0\x81\x12\x15ah\xAFW`\0\x80\xFD[ah\xB7abKV[\x835ah\xC2\x81a\\\x9CV[\x81R` \x84\x015ah\xD2\x81a]\x99V[` \x82\x01R`@\x84\x015ah\xE5\x81a]\x99V[`@\x82\x01R``\x84\x015ah\xF8\x81a]\x99V[``\x82\x01R`\xA0`\x7F\x19\x83\x01\x12\x15ai\x0FW`\0\x80\xFD[ai\x17ab\x14V[\x91P`\x80\x84\x015ai'\x81a]\x13V[\x82R`\xA0\x84\x015ai7\x81a]\x99V[` \x83\x01R`\xC0\x84\x015aiJ\x81a]\x99V[`@\x83\x01R`\xE0\x84\x015ai]\x81a]\x99V[``\x83\x01Ra\x01\0\x84\x015aiq\x81a]\x99V[\x80`\x80\x84\x01RP\x81`\x80\x82\x01Rai\x8C\x85a\x01 \x86\x01ab\x8EV[`\xA0\x82\x01R\x94\x93PPPPV[\x815ai\xA4\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015ai\xCC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015ai\xFC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015agM\x81a]\x99V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15ajTWajTag\x85V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aj\x80Waj\x80ag\x85V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aj\x9CWaj\x9Cag\x85V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aj\xB2Waj\xB2ag\x85V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03ah\x91Wah\x91ag\x85V[`\0`\x80\x82\x84\x03\x12\x15aj\xEDW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15ak\x1EWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Qak,\x81a\\\x9CV[\x81R` \x83\x01Qak<\x81a]\x99V[` \x82\x01R`@\x83\x01QakO\x81a]\x99V[`@\x82\x01R``\x83\x01Qakb\x81a]\x99V[``\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03ak\x8BWak\x8Bag\x85V[`\0\x03\x92\x91PPV[\x815ak\x9F\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x01\x81\x01` \x83\x015af\xCC\x81a]\x99V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15ak\xF6Wak\xF6ag\x85V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15al\x11Wal\x11ag\x85V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15al=Wal=ag\x85V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15almWalmag\x85V[\x02\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80al\xA3Wal\xA3alvV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15al\xCAWal\xCAag\x85V[\x90\x05\x93\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15al\xE5W`\0\x80\xFD[a\x1C\xBE\x83\x83ab\x8EV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15am\x17Wam\x17ag\x85V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15am6Wam6ag\x85V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15amRWamRag\x85V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15amhWamhag\x85V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82am\x85Wam\x85alvV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15am\x9FWam\x9Fag\x85V[P\x05\x90V[\x815am\xAF\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x17\x90UPV[`\0` \x82\x84\x03\x12\x15am\xDFW`\0\x80\xFD[\x81Qa\x1C\xBE\x81a]\x13V[`\0\x82\x82\x10\x15am\xFCWam\xFCag\x85V[P\x03\x90V[`\0\x81an\x10Wan\x10ag\x85V[P`\0\x19\x01\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15an9Wan9ag\x85V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15anRWanRag\x85V[PP\x01\x90V\xFE\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFD\xA2dipfsX\"\x12 i{\xB7\xE7\xA5\x8BSu\xD4xZ\xD1\xFB\xEFw(\xC2LU\xF9\x18\xE9\xEB\x17\xE4_\x8AF\x10\xE6\xFA\xB0dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static SPOTENGINE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x15W`\x005`\xE0\x1C\x80c\xADs;\x8E\x11a\x01\xA7W\x80c\xE34\xBE3\x11a\0\xEEW\x80c\xF2\xFD\xE3\x8B\x11a\0\x97W\x80c\xF8\xA4.Q\x11a\0qW\x80c\xF8\xA4.Q\x14a\nRW\x80c\xFA\xB2\xC4i\x14a\neW\x80c\xFD\xF4\xA0\xC0\x14a\n\x85W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\n\x19W\x80c\xF3\x9E\xEB\x10\x14a\n,W\x80c\xF8f\x18\x84\x14a\n?W`\0\x80\xFD[\x80c\xECzy\xC9\x11a\0\xC8W\x80c\xECzy\xC9\x14a\x08yW\x80c\xEC\xD9\xCB\xA8\x14a\t\x10W\x80c\xED\xF0&S\x14a\ttW`\0\x80\xFD[\x80c\xE34\xBE3\x14a\x07\x92W\x80c\xE3Cs\x8C\x14a\x07\xB3W\x80c\xECbq\xD2\x14a\x08fW`\0\x80\xFD[\x80c\xC7\x16|\xF5\x11a\x01PW\x80c\xD3\x86\xC1\xE8\x11a\x01*W\x80c\xD3\x86\xC1\xE8\x14a\x07YW\x80c\xD9\x87R\xEC\x14a\x07lW\x80c\xE0\xB0b\x1F\x14a\x07\x7FW`\0\x80\xFD[\x80c\xC7\x16|\xF5\x14a\x07\x13W\x80c\xC7!\xBDe\x14a\x07&W\x80c\xC9\xFE\x9A\xC3\x14a\x07FW`\0\x80\xFD[\x80c\xB8\xD8\r\x8B\x11a\x01\x81W\x80c\xB8\xD8\r\x8B\x14a\x06\xC0W\x80c\xC3b\xD1\x9E\x14a\x06\xD3W\x80c\xC5V\x07\xB5\x14a\x07\0W`\0\x80\xFD[\x80c\xADs;\x8E\x14a\x06\x8BW\x80c\xAE\xD8\xE9g\x14a\x06\x9EW\x80c\xB1\xCB\x0FB\x14a\x06\xAFW`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x11a\x02kW\x80c\x8A\x1DC\xC9\x11a\x02\x14W\x80c\x98\xDEr\xFE\x11a\x01\xEEW\x80c\x98\xDEr\xFE\x14a\x06:W\x80c\x9B\xB9\x1Fj\x14a\x06MW\x80c\xA6z\xC3\"\x14a\x06`W`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x05\x98W\x80c\x8A\xF6Bj\x14a\x05\xD7W\x80c\x8D\xA5\xCB[\x14a\x06)W`\0\x80\xFD[\x80c\x7F\xA2\x9DI\x11a\x02EW\x80c\x7F\xA2\x9DI\x14a\x05_W\x80c\x87\x1D\t\x12\x14a\x05rW\x80c\x896\xF7\xCD\x14a\x05\x85W`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x14a\x05$W\x80cqP\x18\xA6\x14a\x057W\x80c|\x1E\x14\x87\x14a\x05?W`\0\x80\xFD[\x80c0\x97+P\x11a\x02\xCDW\x80cF\x04\xD1\x9B\x11a\x02\xA7W\x80cF\x04\xD1\x9B\x14a\x04)W\x80cGB\x8E{\x14a\x048W\x80cI\xF75h\x14a\x04MW`\0\x80\xFD[\x80c0\x97+P\x14a\x03\xACW\x80c=\\\xC9\xDC\x14a\x03\xBFW\x80cE\xBE~\xD6\x14a\x03\xE2W`\0\x80\xFD[\x80c\x14YEz\x11a\x02\xFEW\x80c\x14YEz\x14a\x03pW\x80c\x15<\xA6\xC0\x14a\x03\x83W\x80c+\xAFW\xD3\x14a\x03\x96W`\0\x80\xFD[\x80c\x10\xCA\x93\x88\x14a\x03\x1AW\x80c\x13\x0E\xA3s\x14a\x03/W[`\0\x80\xFD[a\x03-a\x03(6`\x04a\\\xC0V[a\n\x98V[\0[a\x03Xa\x03=6`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x90V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03-a\x03~6`\x04a](V[a\n\xBFV[a\x03-a\x03\x916`\x04a]\xA8V[a\r\x96V[a\x03\x9Ea\x0E[V[`@Qa\x03g\x92\x91\x90a^\x1FV[a\x03-a\x03\xBA6`\x04a^\x89V[a\x10\x86V[a\x03\xD2a\x03\xCD6`\x04a^\xF5V[a\x12gV[`@Qa\x03g\x94\x93\x92\x91\x90a_nV[a\x04\x11a\x03\xF06`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03gV[`\0`@Qa\x03g\x91\x90a_\xE9V[a\x04@a\x13\xB2V[`@Qa\x03g\x91\x90a`\x11V[a\x04`a\x04[6`\x04a`[V[a\x146V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x0F\x90\x81\x0B\x82\x84\x01R\x84\x84\x01Q\x81\x0B\x83\x85\x01R``\x80\x86\x01Q\x82\x0B\x81\x85\x01R`\x80\x80\x87\x01Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x01R\x80\x85\x01Q\x84\x0B`\xA0\x80\x88\x01\x91\x90\x91R\x81\x88\x01Q\x85\x0B`\xC0\x88\x01R\x81\x84\x01Q\x85\x0B`\xE0\x88\x01R\x90\x82\x01Q\x84\x0Ba\x01\0\x87\x01R\x90\x96\x01Q\x80Q`\x03\x90\x81\x0Ba\x01 \x87\x01R\x93\x81\x01Q\x84\x0Ba\x01@\x86\x01R\x94\x85\x01Q\x83\x0Ba\x01`\x85\x01R\x84\x01Q\x90\x91\x0Ba\x01\x80\x83\x01R\x91\x90\x92\x01Q\x90\x0Ba\x01\xA0\x82\x01Ra\x01\xC0\x01a\x03gV[a\x03-a\x0526`\x04a\\\xF6V[a\x14\xD0V[a\x03-a\x15gV[a\x05Ra\x05M6`\x04a^\xF5V[a\x15sV[`@Qa\x03g\x91\x90a`nV[a\x03-a\x05m6`\x04a`\x8FV[a\x16\x08V[a\x03Xa\x05\x806`\x04a`\xDFV[a\x16)V[a\x03-a\x05\x936`\x04aa\x02V[a\x17\xC9V[a\x05\xABa\x05\xA66`\x04aa\x1BV[a\x1CPV[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x03gV[a\x06\x0Fa\x05\xE56`\x04a\\\xF6V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`n` R`@\x90 `\x02\x81\x01T`\x01\x90\x91\x01T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x03gV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[a\x03-a\x06H6`\x04aaYV[a\x1C\xC5V[a\x03-a\x06[6`\x04a\\\xC0V[a$'V[a\x06sa\x06n6`\x04a\\\xF6V[a$HV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x03gV[a\x03-a\x06\x996`\x04aa\xB3V[a$\xECV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x11V[a\x03Xa\x06\xCE6`\x04aa\xDCV[a'\xF5V[`pTa\x06\xE7\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03gV[a\x03-a\x07\x0E6`\x04ac\x01V[a+\x81V[a\x06\x0Fa\x07!6`\x04ac.V[a,\xB2V[a\x079a\x0746`\x04a\\\xF6V[a/0V[`@Qa\x03g\x91\x90acyV[a\x03-a\x07T6`\x04ac\x87V[a/\xAFV[a\x03-a\x07g6`\x04ac\xF9V[a3}V[a\x06\x0Fa\x07z6`\x04ad\x9FV[a6\x86V[a\x03-a\x07\x8D6`\x04ad\x9FV[a<\xB2V[a\x07\xA5a\x07\xA06`\x04a^\xF5V[a=\xADV[`@Qa\x03g\x92\x91\x90ad\xD6V[a\x08Ya\x07\xC16`\x04a\\\xF6V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x90\x91\x01T\x80\x83\x0B``\x83\x01R\x92\x90\x92\x04\x90\x0B`\x80\x82\x01R\x90V[`@Qa\x03g\x91\x90ae.V[a\x03-a\x08t6`\x04a]\xA8V[a>lV[a\t\x03a\x08\x876`\x04a\\\xF6V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`l` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x03g\x91\x90ae|V[a\t#a\t\x1E6`\x04a\\\xF6V[a?jV[`@Qa\x03g\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\t\xF5a\t\x826`\x04a^\xF5V[`@\x80Q``\x80\x82\x01\x83R`\0\x82\x84\x01\x81\x81R\x83R\x83Q` \x80\x82\x01\x86R\x82\x82R\x93\x84\x01Rc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85R`m\x82R\x82\x85 \x93\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82R\x80T`\x0F\x90\x81\x0B\x84\x84\x01\x90\x81R\x84R\x82Q\x80\x86\x01\x90\x93R`\x01\x90\x91\x01T\x90\x0B\x81R\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82QQ`\x0F\x90\x81\x0B\x82R` \x93\x84\x01QQ\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x03gV[a\x03-a\n'6`\x04ae\xB7V[a?\x9EV[a\x03Xa\n:6`\x04ae\xD4V[a@&V[a\x03-a\nM6`\x04ae\xF9V[aA\xB2V[a\x03-a\n`6`\x04afBV[aA\xE1V[`@\x80Q`m\x81R`l` \x82\x01R`n\x91\x81\x01\x91\x90\x91R``\x01a\x03gV[a\x03Xa\n\x936`\x04a\\\xF6V[aDAV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\n\xB9\x82\x82af\x95V[PPPPV[a\n\xCB\x85\x85\x84\x84aE\x16V[`@\x80Q`\xA0\x80\x82\x01\x83R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x85\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x85\x87\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x88\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x8A\x01\x82\x81R`\0\x80\x80R`k\x88R\x9AQ\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9A\x16\x17\x90\x98U\x95Q\x93Q`\x01`\x01`\x80\x1B\x03\x94\x85\x16`\x01`\x80\x1B\x91\x86\x16\x82\x02\x17\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CU\x92Q\x96Q\x96\x84\x16\x96\x84\x16\x83\x02\x96\x90\x96\x17\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8DU\x88Q\x96\x87\x01\x89Rc;\x9A\xCA\0\x80\x88R\x87\x85\x01\x81\x81R\x88\x8B\x01\x82\x81R\x89\x84\x01\x92\x83R\x89\x88\x01\x89\x81R\x8B\x80R\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x88R\x99Q\x7F\xC1\x1D\xC3\x86\x04\x03\xBA\x02&?J\x19\ni\xCA\xB0[\x0E\xD2\xB0\xAD\xAE\x85U6}\x90\x9F\xC6]57\x80T\x93Q\x92Q\x94Q\x9BQ\x88\x16\x87\x02c\xFF\xFF\xFF\xFF\x9C\x8D\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x96\x8E\x16`\x01`@\x1B\x02\x96\x90\x96\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x94\x8E\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x93\x8E\x16\x93\x90\x93\x17\x94\x90\x94\x17\x92\x90\x92\x16\x17\x92\x90\x92\x17\x85\x16\x17\x90U\x88Q\x94\x85\x01\x89R\x85\x85R\x84\x84\x01\x95\x86R\x84\x89\x01\x88\x81R\x90\x85\x01\x88\x81R\x88\x80R`l\x85R\x94Q\x95Q\x83\x16\x82\x02\x95\x83\x16\x95\x90\x95\x17`\0\x80Q` anY\x839\x81Q\x91RU\x93Q\x92Q\x81\x16\x90\x93\x02\x91\x90\x92\x16\x17`\0\x80Q` any\x839\x81Q\x91RU`h\x80T`\x01\x81\x01\x82U\x90\x84R`\x08\x81\x04\x7F\xA2\x154 \xD8D\x92\x8BD!e\x02\x03\xC7{\xAB\xC8\xB3=\x7F.{E\x0E)f\xDB\x0C\"\twS\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x93\x02\x19\x16\x90\x91U\x91Q\x90\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x91\x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[``\x80`\0`h\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xD8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\x9BW\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xF9Wa\x0E\xF9aa\xFEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F?Wa\x0F?aa\xFEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0FhW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x10\x80W`\0\x82\x82\x81Q\x81\x10a\x0F\x8BWa\x0F\x8BagoV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`l\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0F\xFA\x92\x91aF\xB3\x16V[\x86\x84\x81Q\x81\x10a\x10\x0CWa\x10\x0CagoV[` \x02` \x01\x01\x81\x81Qa\x10 \x91\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x10@\x93P\x90\x91\x0B\x90aF\xB3V[\x85\x84\x81Q\x81\x10a\x10RWa\x10RagoV[` \x02` \x01\x01\x81\x81Qa\x10f\x91\x90ag\x9BV[`\x0F\x0B\x90RPa\x10y\x91P\x82\x90Pag\xEAV[\x90Pa\x0FnV[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x12`W`\0`h\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x10\xB8Wa\x10\xB8agoV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`l\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x11DWa\x11DagoV[\x90P` \x02\x01` \x81\x01\x90a\x11Y\x91\x90ah\x03V[`\x0F\x0Ba\x11z\x82`\0\x01Q\x83`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x11\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x11\xD6Wa\x11\xD6agoV[\x90P` \x02\x01` \x81\x01\x90a\x11\xEB\x91\x90ah\x03V[`\x0F\x0Ba\x12\x0C\x82` \x01Q\x83``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[PPP\x80a\x12Y\x90ahuV[\x90Pa\x10\x89V[PPPPPV[a\x12oa\\7V[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x83Q`\x80\x80\x82\x01\x86R\x82\x82R\x81\x84\x01\x83\x90R\x81\x86\x01\x83\x90R``\x80\x83\x01\x84\x90R\x86Q\x80\x88\x01\x88R\x84\x81R\x80\x86\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x8B\x16\x80\x86R`n\x87R\x88\x86 \x89Q\x80\x85\x01\x8BR\x81T`\x0F\x90\x81\x0B\x82R\x8BQ\x80\x8D\x01\x8DR`\x01\x80\x85\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8E\x01R\x84\x8D\x01\x92\x90\x92R\x8DQ\x80\x8F\x01\x8FR`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x82\x90\x04\x83\x0B\x85\x8D\x01R\x83\x8E\x01\x94\x90\x94R\x84\x8AR`l\x8BR\x8C\x8A \x8DQ\x98\x89\x01\x8ER\x80T\x80\x84\x0B\x8AR\x82\x90\x04\x83\x0B\x89\x8D\x01R\x84\x01T\x80\x83\x0B\x89\x8F\x01R\x04\x81\x0B\x87\x87\x01R\x92\x88R`m\x89R\x8A\x88 \x8D\x89R\x89R\x96\x8A\x90 \x8AQ\x94\x85\x01\x8BR\x80T\x83\x0B\x85\x8C\x01\x90\x81R\x85R\x8AQ\x80\x8A\x01\x90\x9BR\x01T\x90\x0B\x88R\x94\x81\x01\x87\x90R\x80Q\x95\x96\x92\x95\x91\x92\x90\x91\x84\x82\x85a\x13\x9D\x81\x85aG6V[\x92\x9E\x91\x9DP\x9BP\x90\x99P\x97PPPPPPPPV[```h\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14,W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x13\xEFW\x90P[PPPPP\x90P\x90V[a\x14\xBB`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R\x84Q`\xA0\x81\x01\x86R\x83\x81R\x91\x82\x01\x83\x90R\x93\x81\x01\x82\x90R\x92\x83\x01\x81\x90R`\x80\x83\x81\x01\x91\x90\x91R\x90\x91\x90\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[a\x14\xCA6\x83\x90\x03\x83\x01\x83ah\x9BV[\x92\x91PPV[`\0a\x14\xDD\x82`\x01a=\xADV[P\x90P`\0a\x15\0\x82`\0\x01Q\x83`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x15\"\x83` \x01Q\x84``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x12`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[a\x15q`\0aG\x9DV[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x15\xFF\x82\x82aG6V[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`l` R`@\x90 \x81\x90a\n\xB9\x82\x82ai\x99V[`\0\x80a\x164a\x13\xB2V[\x90P\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x17\xC0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\x83Wa\x16\x83agoV[` \x02` \x01\x01Q\x90P`\0a\x16\x99\x82\x85aG\xEFV[\x90P`\0\x80a\x16\xA8\x84\x8BaI\x04V[\x91P\x91P`\0a\x16\xB9\x84\x84\x8CaI\x1DV[\x90Pa\x16\xC5\x82\x8Aag\x9BV[\x98P\x82`\x0F\x0B`\0\x14a\x17?Wa\x16\xE5g\r\xE0\xB6\xB3\xA7d\0\0`\x02aj$V[`\x0F\x0B\x81`\x0F\x0B\x03a\x17\x12Wo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x98PPPPPPPPPa\x14\xCAV[`\x80\x84\x01Qa\x172\x90a\x17)`\x0F\x86\x90\x0B\x84aF\xB3V[`\x0F\x0B\x90aF\xB3V[a\x17<\x90\x8Aag\x9BV[\x98P[PPP`\0\x80`\0a\x17Q\x85\x8CaI\xB4V[\x92P\x92P\x92P\x82`\x0F\x0B`\0\x14a\x17\xA8W`\0a\x17s\x84\x84\x87`\x80\x01QaJ\xCFV[\x90P\x81a\x17\x90a\x17\x85\x87`\x01\x8FaI\x1DV[`\x0F\x84\x90\x0B\x90aF\xB3V[a\x17\x9A\x91\x90ag\x9BV[a\x17\xA4\x90\x8Bag\x9BV[\x99PP[PPPPP\x80\x80a\x17\xB8\x90aj\xC2V[\x91PPa\x16ZV[PPP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0a\x18\x18a\x13\xB2V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1CKW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18LWa\x18LagoV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`l\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`m\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x18\xD1\x90\x83\x90aG6V[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x1C7W\x81Q`@\x83\x01Q`\0\x91a\x18\xFA\x91`\x0F\x0B\x90aF\xB3V[`@\x84\x01Q\x83Q\x91\x92Pa\x19\x1B\x91a\x19\x12\x90\x84ag\x9BV[`\x0F\x0B\x90aK\x0BV[`\x0F\x0B\x80\x84R`\0\x12a\x19-W`\0\x80\xFD[a\x19K\x83` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x19\\\x91\x90ag\x9BV[`\x0F\x0B\x90RPc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8B\x84R\x90\x91R\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90Ua\x1A\xEDW`\0[\x86Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1A\xE7W`\0\x87\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x19\xC0Wa\x19\xC0agoV[` \x02` \x01\x01Q\x90P`\0c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x03a\x19\xE4WPa\x1A\xD7V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x82R\x86Q\x80\x88\x01\x88R`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x04\x90\x91\x0B\x93\x83\x01\x93\x90\x93R\x92\x83\x01RQa\x1Aa\x90\x87\x90aKtV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x83Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x84\x83\x01Q\x80Q\x90\x84\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x84\x01U\x93\x85\x01Q\x80Q\x93\x01Q\x92\x81\x16\x92\x16\x90\x92\x02\x17`\x02\x90\x91\x01Ua\x1A\xD4\x82aK\xEDV[PP[a\x1A\xE0\x81aj\xC2V[\x90Pa\x19\x97V[Pa\x1B\xDCV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x82\x90R\x90a\x1Bl\x90\x85\x90aKtV[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x83Q\x81T`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x82U\x84\x83\x01Q\x80Q\x90\x84\x01Q\x82\x16`\x01`\x80\x1B\x90\x81\x02\x91\x83\x16\x91\x90\x91\x17`\x01\x84\x01U\x94\x90\x93\x01Q\x80Q\x92\x01Q\x83\x16\x90\x93\x02\x91\x16\x17`\x02\x90\x91\x01U[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x91\x82\x90 \x85Q\x91\x86\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x86\x01Q``\x87\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua\x1C5\x84\x88aL)V[P[PPP\x80a\x1CD\x90ahuV[\x90Pa\x18\x1DV[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x1Cv\x84aLlV[\x90P`\0a\x1C\x84\x85\x87aI\x04V[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x1C\xB4\x84`\x01\x88aI\x1DV[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[`\0\x83`\x0F\x0B\x13\x80\x15a\x1C\xDBWP`\0\x82`\x0F\x0B\x13[\x80\x15a\x1C\xEAWP`\0\x81`\x0F\x0B\x13[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bILA`\xE8\x1B\x81RP\x90a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0a\x1D/aL\xC1V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x88\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x9F\x91\x90aj\xDBV[`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x04\x95`\xF4\x1B\x81RP\x90a\x1D\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x80\x82\x01\x84R\x82T`\x0F\x90\x81\x0B\x83R\x84Q\x80\x86\x01\x86R`\x01\x80\x86\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8A\x01R\x85\x89\x01\x92\x90\x92R\x86Q\x80\x88\x01\x88R`\x02\x90\x96\x01T\x80\x84\x0B\x87R\x82\x90\x04\x83\x0B\x86\x89\x01R\x84\x87\x01\x95\x86R\x98\x88R`l\x80\x88R\x86\x89 \x87Q`\x80\x80\x82\x01\x8AR\x82T\x80\x87\x0B\x83R\x85\x90\x04\x86\x0B\x82\x8C\x01R\x91\x90\x9B\x01T\x80\x85\x0B\x8C\x8A\x01R\x83\x90\x04\x84\x0B\x8B\x86\x01R\x89\x80R\x90\x88R\x86Q\x90\x81\x01\x87R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x84\x0B\x82R\x82\x90\x04\x83\x0B\x97\x81\x01\x97\x90\x97R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x96\x88\x01\x96\x90\x96R\x90\x94\x04\x84\x0B\x90\x85\x01R\x90QQ\x90\x94\x93\x91\x0B\x15a\x1F\x1BW`@\x84\x01QQ` \x85\x01QQa\x1F\x16\x91a\x1F\x0B\x91`\x0F\x0B\x90aK\x0BV[`\x0F\x89\x90\x0B\x90aF\xB3V[a\x1F6V[a\x1F6a\x1F'\x8AaLlV[`\x80\x01Q`\x0F\x89\x90\x0B\x90aF\xB3V[\x90P\x85`\x0F\x0B\x81`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\nj\x89`\xEB\x1B\x81RP\x90a\x1F|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x84`\x0F\x0B\x81`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\nj\x89`\xEB\x1B\x81RP\x90a\x1F\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x83Q`\0\x90`\x0F\x0B\x81\x03a\x1F\xE1Wa\x1F\xDA\x82\x89ag\x9BV[\x90Pa \0V[\x84Q`@\x86\x01QQa\x1F\xFD\x91\x90a\x17)\x90`\x0F\x8C\x90\x0B\x90aK\x0BV[\x90P[a \x0F\x84\x86`@\x01Q\x8AaM;V[a \x1E\x83\x86` \x01Q\x84aM;V[\x80\x85`\0\x01\x81\x81Qa 0\x91\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91Rc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x8E\x84R\x90\x91R\x81 `\x01\x01\x80T\x85\x94P\x90\x92a n\x91\x85\x91\x0Bag\x9BV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`\x80\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91Uc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x8AQ\x81T`\x01`\x01`\x80\x1B\x03\x19\x16\x90\x86\x16\x17\x81U\x8A\x83\x01Q\x80Q\x90\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17`\x01\x83\x01U\x8B\x83\x01Q\x80Q\x90\x85\x01Q\x90\x87\x16\x96\x16\x02\x94\x90\x94\x17`\x02\x90\x94\x01\x93\x90\x93U`m\x81R\x82\x82 \x8D\x83R\x81R\x82\x82 \x83Q\x80\x83\x01\x85R\x90T`\x0F\x90\x81\x0B\x82R\x8E\x84R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x83R\x92\x84\x90 \x84Q\x92\x83\x01\x90\x94R\x92T\x90\x91\x0B\x81R\x90\x91Pa!b\x86\x83a!]\x8DaknV[aN\xE1V[a!p\x85\x82a!]\x87aknV[\x81`m`\0\x8Ec\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x80`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x85`l`\0\x8Ec\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x84`l`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PPa$\x0E\x8C\x8CaL)V[a$\x19`\0\x8CaL)V[PPPPPPPPPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` R`@\x90 \x81\x90a\n\xB9\x82\x82ak\x94V[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a$\xE2\x92\x85\x92a$\xB5\x92\x91\x90aF\xB3\x16V[a$\xD3\x84`\0\x01Q\x85`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a$\xDD\x91\x90ak\xCBV[aOuV[a\x1C\xBE\x90\x83al\x1BV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a%YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra%\x8Bb\x01Q\x80`\x07alFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a%\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0[`hTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x1CKW`\0`h\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a&\tWa&\tagoV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`l\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x81a&\x83W\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a&\x99WPPa'\xE3V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x91\x90\x91Ra'\x18\x83\x83\x88aO\x91V[a'&\x82\x82`@\x01QaKtV[a'4\x85\x82` \x01QaKtV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x84Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x85\x84\x01Q\x80Q\x90\x85\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x80\x85\x01\x91\x90\x91U\x87\x85\x01Q\x80Q\x90\x87\x01Q\x90\x84\x16\x90\x84\x16\x83\x02\x17`\x02\x90\x94\x01\x93\x90\x93U`l\x85R\x94\x83\x90 \x87Q\x94\x88\x01Q\x94\x82\x16\x94\x82\x16\x86\x02\x94\x90\x94\x17\x84U\x91\x86\x01Q``\x87\x01Q\x90\x83\x16\x92\x16\x90\x93\x02\x17\x91\x01Ua'\xDF\x83aK\xEDV[PPP[\x80a'\xED\x81aj\xC2V[\x91PPa%\xDEV[`\0\x80a(\0a\x13\xB2V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a+yW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a(4Wa(4agoV[` \x02` \x01\x01Q\x90P`\0a(R\x82\x88`\x01`\x01`\x7F\x1B\x03a6\x86V[\x91PP\x80`\x0F\x0B`\0\x14a+fW`\0a(\x97`2a({a(s\x86aLlV[\x85`\x01aI\x1DV[a(\x8D\x90g\r\xE0\xB6\xB3\xA7d\0\0ak\xCBV[a\x17\x85\x91\x90al\x8CV[\x90P`\0a(\xB1`\x0F\x83\x90\x0Bg\x06\xF0[Y\xD3\xB2\0\0aF\xB3V[\x90Pa(\xBD\x81\x83ak\xCBV[\x91Pa(\xC9\x81\x88ag\x9BV[`@\x80Q`\x80\x81\x01\x82R`\0\x80Q` anY\x839\x81Q\x91RT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x84\x81\x01\x91\x90\x91R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x8E\x81R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x80\x84R\x85\x82 \x86Q\x80\x86\x01\x88R\x90T\x84\x0B\x81R\x8F\x83R\x90\x84R\x90\x85\x90 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x92\x99P\x91a)\x8E\x83\x83\x86a)\x84\x89aknV[a!]\x91\x90ak\xCBV[a)\x99\x83\x82\x87aN\xE1V[\x81`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x80`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x82`l`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PPa+U`\0\x8CaL)V[a+``\0\x8DaL)V[PPPPP[PP\x80a+r\x90ahuV[\x90Pa(\x05V[PP\x92\x91PPV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a+\xADWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a+\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92fc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x0F\x90\x81\x0B\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x88\x01R\x83\x87\x01\x91\x82R\x85Q\x80\x87\x01\x87R`\x02\x90\x95\x01T\x80\x84\x0B\x86R\x04\x90\x91\x0B\x94\x83\x01\x94\x90\x94R\x91\x82\x01\x81\x90RQ\x91QQ\x83\x92a-7\x91\x87\x91\x87\x91aS\x9BV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a-pW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`l` R`@\x80\x82 T\x91\x80R`\0\x80Q` anY\x839\x81Q\x91RT\x90\x83\x01Q\x80Q`\x0F\x93\x84\x0B\x93\x92\x90\x92\x0B\x91\x88\x91\x90a-\xBA\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RP` \x83\x01Q\x80Q\x87\x91\x90a-\xD5\x90\x83\x90ag\x9BV[`\x0F\x90\x81\x0B\x90\x91Rc\xFF\xFF\xFF\xFF\x8A\x81\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x88Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x82U\x89\x83\x01Q\x80Q\x90\x84\x01Q\x90\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\x01\x84\x01U\x93\x8A\x01Q\x80Q\x93\x01Q\x92\x81\x16\x92\x16\x90\x92\x02\x17`\x02\x90\x91\x01Ua.Z\x92P\x90\x89\x90\x0B\x90\x84\x90aK\x0B\x16V[c\xFF\xFF\xFF\xFF\x89\x16`\0\x90\x81R`l` R`@\x81 `\x01\x01\x80T\x90\x91\x90a.\x85\x90\x84\x90`\x0F\x0Bag\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa.\xC2\x81\x87`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x80\x80R`l` R`\0\x80Q` any\x839\x81Q\x91R\x80T\x90\x91\x90a.\xEE\x90\x84\x90`\x0F\x0Bag\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa/\x1E\x88aK\xEDV[\x86\x86\x94P\x94PPPP[\x93P\x93\x91PPV[a/8a\\7V[Pc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`n` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x90\x81\x0B\x82R\x84Q\x80\x86\x01\x86R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x87\x01R\x83\x86\x01\x91\x90\x91R\x85Q\x80\x87\x01\x87R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x90\x0B\x92\x82\x01\x92\x90\x92R\x91\x81\x01\x91\x90\x91R\x90V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a0\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`\0a0*\x82\x84\x01\x84ah\x9BV[`\xA0\x81\x01Q\x81Q\x91\x92P\x90c\xFF\xFF\xFF\xFF\x16\x15a2\xFBW\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a0lWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a0\x9FWP`\x80\x82\x01QQ\x82Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a0\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x83Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T`\x03\x81\x81\x0B\x83Rd\x01\0\0\0\0\x82\x04\x81\x0B\x83\x86\x01\x90\x81R`\x01`@\x1B\x83\x04\x82\x0B\x84\x86\x01\x90\x81R`\x01``\x1B\x84\x04\x83\x0B``\x80\x87\x01\x91\x82R`\x01`\x80\x1B\x90\x95\x04`\x0F\x0B`\x80\x87\x01R\x89Q\x84\x0B\x86R\x96\x89\x01Q\x83\x0B\x90\x91R\x93\x87\x01Q\x81\x0B\x90\x93R\x85\x01Q\x90\x91\x0B\x90\x91R\x90P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92f\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x92\x83R`@\x90\x81\x90 \x84Q\x81T\x94\x86\x01Q\x92\x86\x01Q``\x87\x01Q`\x80\x90\x97\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x97\x86\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x87\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x87\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x98\x16\x93\x90\x96\x16\x92\x90\x92\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x92\x90\x92\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90Ua2_aL\xC1V[\x83Q` \x85\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qc-\xA1\xC5\x9B`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x94\x85\x16`\x04\x82\x01R`$\x81\x01\x94\x90\x94R`\0`D\x85\x01R`\x0F\x92\x83\x0B`d\x85\x01R\x82\x0B`\x84\x84\x01R\x90\x0B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c-\xA1\xC5\x9B\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a2\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a2\xF5W=`\0\x80>=`\0\xFD[PPPPP[P`\x80\x81\x81\x01Q\x91Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17`\x01\x83\x01U``\x85\x01Q\x94\x90\x93\x01Q\x93\x82\x16\x93\x90\x91\x16\x90\x91\x02\x91\x90\x91\x17`\x02\x90\x91\x01UPPV[c\xFF\xFF\xFF\xFF\x88\x16a3\x8DW`\0\x80\xFD[a3\xAA\x88\x88\x88\x88\x88\x88a3\xA56\x89\x90\x03\x89\x01\x89al\xD3V[aT9V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`k` R`@\x90 \x82\x90a3\xCB\x82\x82af\x95V[\x90PP`@Q\x80`\x80\x01`@R\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP`l`\0\x8Ac\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x81RP\x81RP`n`\0\x8Ac\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`@\x82\x01Q\x81`\x02\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP\x90PPPPPPPPPPV[`\0\x80`\0\x83`\x0F\x0B\x13`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bILA`\xE8\x1B\x81RP\x90a6\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x80\x82\x01\x84R\x82T`\x0F\x90\x81\x0B\x83R\x84Q\x80\x86\x01\x86R`\x01\x80\x86\x01T\x80\x84\x0B\x83R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x83\x8A\x01R\x85\x89\x01\x92\x90\x92R\x86Q\x80\x88\x01\x88R`\x02\x90\x96\x01T\x80\x84\x0B\x87R\x82\x90\x04\x83\x0B\x86\x89\x01R\x84\x87\x01\x95\x90\x95R\x88\x88R`m\x87R\x85\x88 \x8D\x89R\x87R\x85\x88 \x86Q\x80\x89\x01\x88R\x90\x86\x01T\x83\x0B\x81R\x98\x88R`l\x80\x88R\x86\x89 \x87Q`\x80\x80\x82\x01\x8AR\x82T\x80\x87\x0B\x83R\x85\x90\x04\x86\x0B\x82\x8C\x01R\x91\x90\x97\x01T\x80\x85\x0B\x88\x8A\x01R\x83\x90\x04\x84\x0B\x87\x86\x01R\x98\x80R\x87R\x85Q\x97\x88\x01\x86R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x83\x0B\x89R\x81\x90\x04\x82\x0B\x96\x88\x01\x96\x90\x96R`\0\x80Q` any\x839\x81Q\x91RT\x80\x82\x0B\x95\x88\x01\x95\x90\x95R\x94\x90\x93\x04\x84\x0B\x92\x85\x01\x92\x90\x92R\x90\x93\x92\x90\x91\x87\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19\x01a8\x11W\x82Q\x96P[\x86`\x0F\x0B`\0\x03a8,W`\0\x80\x95P\x95PPPPPa/(V[\x86`\x0F\x0B\x83`\0\x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x94\xC5`\xEC\x1B\x81RP\x90a8tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x86\x83`\0\x01\x81\x81Qa8\x87\x91\x90ak\xCBV[`\x0F\x90\x81\x0B\x90\x91R\x85Q`@\x87\x01QQ\x90\x82\x0B\x92Pa8\xAC\x91\x90\x81\x0B\x90\x8A\x90\x0Bal\xEFV[a8\xB6\x91\x90amvV[\x95P\x83`\0\x01Q`\x0F\x0B\x84` \x01Q`\0\x01Q`\x0F\x0B\x88`\x0F\x0Ba8\xDA\x91\x90al\xEFV[a8\xE4\x91\x90amvV[\x94Pa8\xFE\x82\x85`@\x01Q\x88a8\xF9\x90aknV[aM;V[a9\x11\x81\x85` \x01Q\x87a8\xF9\x90aknV[\x86\x84`\0\x01\x81\x81Qa9#\x91\x90ak\xCBV[\x91P\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x83`n`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP`@\x82\x01Q\x81`\x02\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPP\x90PP\x82`m`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x81R` \x01\x90\x81R` \x01`\0 `\x01\x01`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP`\0`m`\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8A\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80` \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B`\x0F\x0B`\x0F\x0B\x81RPP\x90P`\0`m`\0\x80c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80` \x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B`\x0F\x0B`\x0F\x0B\x81RPP\x90Pa;\x8B\x84\x83\x8AaN\xE1V[a;\x96\x83\x82\x89aN\xE1V[c\xFF\xFF\xFF\xFF\x8B\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8E\x84R\x82R\x80\x83 \x86Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x17\x90\x92U\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x84R\x82\x85 \x87Q\x81T\x90\x93\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U\x93\x83R`l\x82R\x80\x83 \x88Q\x89\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17\x82U\x89\x83\x01Q``\x80\x8C\x01Q\x91\x88\x16\x91\x88\x16\x83\x02\x91\x90\x91\x17`\x01\x93\x90\x93\x01\x92\x90\x92U\x93\x80R\x87Q\x92\x88\x01Q\x92\x85\x16\x92\x85\x16\x84\x02\x92\x90\x92\x17`\0\x80Q` anY\x839\x81Q\x91RU\x86\x01Q\x90\x86\x01Q\x90\x83\x16\x92\x16\x02\x17`\0\x80Q` any\x839\x81Q\x91RUa<\x99\x8B\x8BaL)V[a<\xA4`\0\x8BaL)V[PPPPPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra=+\x82\x82\x85aN\xE1V[c\xFF\xFF\xFF\xFF\x85\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x85Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x17\x90\x91U\x93\x83R`l\x82R\x91\x82\x90 \x85Q\x91\x86\x01Q\x91\x84\x16`\x01`\x80\x1B\x92\x85\x16\x83\x02\x17\x81U\x91\x85\x01Q``\x86\x01Q\x90\x84\x16\x93\x16\x02\x91\x90\x91\x17`\x01\x91\x90\x91\x01Ua\x12`\x85\x85aL)V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`m\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a>^\x81\x83aG6V[\x93P\x93PPP[\x92P\x92\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a>\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\r\xFFV[`\0\x81`\x0F\x0B\x12\x15\x80\x15a>\xF9WPg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x82\x90\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a?3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`q` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x14\xCA\x82aLlV[`\x01`\x01`\xA0\x1B\x03\x81\x16a@\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xFFV[a@#\x81aG\x9DV[PV[`@\x80Q`\x80\x81\x01\x82R`\0\x80Q` anY\x839\x81Q\x91RT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R`\0\x80Q` any\x839\x81Q\x91RT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a@\xC1\x83\x83aG6V[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15aA\nW`\0a@\xEFa@\xE8\x87a@\xE3\x85aknV[aY\x03V[`\0aOuV[\x90Pa@\xFB\x81\x87ak\xCBV[\x95PaA\x08\x84\x84\x83aN\xE1V[P[P`\0\x80\x80R`l` \x90\x81R\x83Q\x84\x82\x01Q`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x91\x83\x16\x82\x02\x17`\0\x80Q` anY\x839\x81Q\x91RU`@\x80\x87\x01Q``\x90\x97\x01Q\x96\x83\x16\x96\x83\x16\x90\x91\x02\x95\x90\x95\x17`\0\x80Q` any\x839\x81Q\x91RU\x96\x82R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x90R\x91\x90\x91 \x90Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16\x94\x16\x93\x90\x93\x17\x90\x92U\x91\x90PV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81\x90`\x01\x01a\x12`\x82\x82am\xA4V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16aB\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`l` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R`\0\x80Q` anY\x839\x81Q\x91RT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R`\0\x80Q` any\x839\x81Q\x91RT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x95\x85R`m\x84R\x82\x85 \x89\x86R\x84R\x82\x85 \x83Q\x80\x86\x01\x85R\x90T\x83\x0B\x81R\x89\x86R\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x85R\x94\x83\x90 \x83Q\x94\x85\x01\x90\x93R\x91T\x90\x0B\x82R\x92\x91\x90aC\x1E\x84\x83\x88aN\xE1V[aC)\x83\x82\x87aN\xE1V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x81\x81R`m` \x90\x81R`@\x80\x83 \x8B\x84R\x82R\x80\x83 \x86Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x17\x90\x92U\x7F\xDA\x90\x04;\xA5\xB4\tk\xA1G\x04\xBC\"z\xB0\xD3\x16}\xA1[\x88~b\xAB.v\xE3}\xAAq\x13V\x84R\x82\x85 \x87Q\x81T\x90\x93\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91U\x93\x83R`l\x82R\x80\x83 \x88Q\x89\x84\x01Q\x90\x86\x16`\x01`\x80\x1B\x91\x87\x16\x82\x02\x17\x82U\x89\x83\x01Q``\x80\x8C\x01Q\x91\x88\x16\x91\x88\x16\x83\x02\x91\x90\x91\x17`\x01\x93\x90\x93\x01\x92\x90\x92U\x93\x80R\x87Q\x92\x88\x01Q\x92\x85\x16\x92\x85\x16\x84\x02\x92\x90\x92\x17`\0\x80Q` anY\x839\x81Q\x91RU\x86\x01Q\x90\x86\x01Q\x90\x83\x16\x92\x16\x02\x17`\0\x80Q` any\x839\x81Q\x91RUaD,\x88\x88aL)V[aD7`\0\x88aL)V[PPPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x15\x80aD\\WP\x81c\xFF\xFF\xFF\xFF\x16`\x05\x14[\x80aDmWP\x81c\xFF\xFF\xFF\xFF\x16`\x1F\x14[\x80aD~WP\x81c\xFF\xFF\xFF\xFF\x16`)\x14[\x80aD\x8FWP\x81c\xFF\xFF\xFF\xFF\x16`m\x14[\x15aD\xA3WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[\x81c\xFF\xFF\xFF\xFF\x16`\x01\x03aD\xBEWPe$a9\xCA\x80\0\x91\x90PV[\x81c\xFF\xFF\xFF\xFF\x16`\x03\x14\x80aD\xD9WP\x81c\xFF\xFF\xFF\xFF\x16`[\x14[\x80aD\xEAWP\x81c\xFF\xFF\xFF\xFF\x16`]\x14[\x80aD\xFBWP\x81c\xFF\xFF\xFF\xFF\x16`o\x14[\x15aE\x0EWPf\x02!\xB2b\xDD\x80\0\x91\x90PV[P`\0\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15aE6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80aEPWP0;\x15\x80\x15aEPWP`\0T`\xFF\x16`\x01\x14[aE\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\r\xFFV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15aE\xE5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[aE\xEDaY\x18V[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UaF\x11\x82a?\x9EV[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`j` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x12`W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\r\x87V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90aF\xF5WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90aG.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aGcWP\x82QaGjV[P` \x83\x01Q[`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81\x90aG\x85\x90`\x0F\x0B\x84aF\xB3V[`\x0F\x0B\x81R` \x01\x82`\x0F\x0B\x81RP\x91PP\x92\x91PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x83\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x03\x81\x81\x0B\x80\x84Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x81\x04\x90\x93\x0B``\x82\x01R`\x01`\x80\x1B\x90\x92\x04`\x0F\x0B`\x80\x83\x01RaH\x91\x90c;\x9A\xCA\0aj$V[`\x0F\x0B\x82R` \x81\x01QaH\xAC\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x0B` \x83\x01R`@\x81\x01QaH\xCA\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x0B`@\x83\x01R``\x81\x01QaH\xE8\x90`\x03\x0Bc;\x9A\xCA\0aj$V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x92\x91PPV[`\0\x80aI\x11\x84\x84a\x15sV[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15aI3WaI3a_\xD3V[\x03aIGWPg\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xBEV[`\0\x80\x84`\x0F\x0B\x12aI\x80W`\0\x83`\x02\x81\x11\x15aIgWaIga_\xD3V[\x14aIvW\x84`@\x01QaIyV[\x84Q[\x90PaI\xACV[`\0\x83`\x02\x81\x11\x15aI\x94WaI\x94a_\xD3V[\x14aI\xA3W\x84``\x01QaI\xA9V[\x84` \x01Q[\x90P[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x81Q\x92\x83\x01\x90\x91R`\x01\x01T`\x0F\x0B\x80\x82R\x82\x91\x82\x91\x82\x03aJ\x01W`\0\x80`\0\x93P\x93P\x93PPaJ\xC8V[c\xFF\xFF\xFF\xFF\x80\x87\x16`\0\x90\x81R`n` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x0F\x90\x81\x0B\x82R\x83Q\x80\x85\x01\x85R`\x01\x84\x01T\x80\x83\x0B\x82R`\x01`\x80\x1B\x90\x81\x90\x04\x83\x0B\x82\x88\x01R\x83\x87\x01\x91\x90\x91R\x84Q\x80\x86\x01\x86R`\x02\x90\x94\x01T\x80\x83\x0B\x85R\x04\x81\x0B\x94\x83\x01\x94\x90\x94R\x91\x82\x01R\x80Q\x85Q\x91\x94aJ\x87\x93\x92\x90\x92\x0B\x91aK\x0B\x16V[`@\x83\x01QQ\x90\x91P`\0\x90aJ\xA0\x90`\x0F\x0B\x83aF\xB3V[` \x84\x01QQ\x90\x91P`\0\x90aJ\xB9\x90`\x0F\x0B\x84aF\xB3V[\x91\x97P\x90\x95P`\0\x94PPPPP[\x92P\x92P\x92V[`\0aK\0\x83`\x0F\x0BaJ\xEE\x84\x87`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0BaJ\xFB\x91\x90al\xEFV[aY\x8BV[aI\xAC\x90`\x02aj$V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90aKOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81aF\xCAWaF\xCAalvV[\x80Q`\x0F\x0B`\0\x03aK\x8DW\x90Q`\x0F\x0B` \x90\x91\x01RV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15aK\xA6WP\x81QaK\xADV[P` \x82\x01Q[\x80`\x0F\x0B\x82` \x01Q`\x0F\x0B\x03aK\xC3WPPPV[` \x82\x01Q\x82QaK\xDC\x91\x90a\x19\x12\x90`\x0F\x0B\x84aF\xB3V[`\x0F\x90\x81\x0B\x83R\x0B` \x90\x91\x01RPV[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x14\xCA\x82\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92faG\xEFV[`\0aL\xD5`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM6\x91\x90am\xCDV[\x90P\x90V[\x81Q`\x0F\x0B\x15\x80\x15aMRWP` \x82\x01Q`\x0F\x0B\x15[\x15aMfWg\r\xE0\xB6\xB3\xA7d\0\0` \x83\x01R[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aM\xB0WaM\x94\x82` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83`@\x01\x81\x81QaM\xA5\x91\x90ak\xCBV[`\x0F\x0B\x90RPaM\xE6V[aM\xCE\x82` \x01Q\x83`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81QaM\xDF\x91\x90ag\x9BV[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aM\xFFWP\x82QaN\x06V[P` \x83\x01Q[\x81aN/aN$\x85` \x01Q\x84`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q`\x0F\x0B\x90aF\xB3V[aN9\x91\x90ag\x9BV[`\x0F\x0B\x80\x84R`\0\x12\x15aNOWP\x82QaNVV[P` \x83\x01Q[`\x0F\x81\x81\x0B` \x85\x01R\x83Q`\0\x91\x0B\x13\x15aN\xA6WaN\x8A\x83` \x01Q\x84`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84`@\x01\x81\x81QaN\x9B\x91\x90ag\x9BV[`\x0F\x0B\x90RPa\n\xB9V[aN\xC4\x83` \x01Q\x84`\0\x01Q`\x0F\x0BaK\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84``\x01\x81\x81QaN\xD5\x91\x90ak\xCBV[`\x0F\x0B\x90RPPPPPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aO\x0FW\x81Q`@\x84\x01\x80QaO\x04\x90\x83\x90ak\xCBV[`\x0F\x0B\x90RPaO)V[\x81Q``\x84\x01\x80QaO\"\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RP[aO4\x83\x83\x83aZKV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15aObW\x81Q`@\x84\x01\x80QaOW\x90\x83\x90ag\x9BV[`\x0F\x0B\x90RPPPPV[\x81Q``\x84\x01\x80QaOW\x90\x83\x90ak\xCBV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13aO\x8AW\x81a\x1C\xBEV[P\x90\x91\x90PV[`\0\x80aO\xB2\x84`\0\x01Q\x85`@\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0aO\xD4\x85` \x01Q\x86``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0aO\xE6`\x0F\x83\x90\x0B\x84aK\x0BV[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x90\x92\x01T\x80\x85\x0B``\x83\x01R\x91\x90\x91\x04\x83\x0B`\x80\x82\x01R\x93\x94P\x91\x90\x84\x90\x0B\x90\x03aPdWP`\0aQ\0V[\x81` \x01Q`\x0F\x0B\x83`\x0F\x0B\x12\x15aP\xA9WaP\x98\x82` \x01Qa\x19\x12\x85\x85``\x01Q`\x0F\x0BaF\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[aP\xA2\x90\x82ag\x9BV[\x90PaQ\0V[aP\xE4aP\xD6\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0aP\xC7\x91\x90ak\xCBV[` \x85\x01Qa\x19\x12\x90\x87ak\xCBV[`\x80\x84\x01Q`\x0F\x0B\x90aF\xB3V[\x82``\x01QaP\xF3\x91\x90ag\x9BV[aP\xFD\x90\x82ag\x9BV[\x90P[aQ\x1BaQ\x10c\x01\xE13\x80aZ\xC2V[`\x0F\x83\x90\x0B\x90aK\x0BV[\x90PaQ<\x87aQ3\x83g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[`\x0F\x0B\x90a[;V[\x95PPP`\0aQYg\r\xE0\xB6\xB3\xA7d\0\0\x86a\x17\x85\x91\x90ak\xCBV[\x90P`\0aQza\x17\x85g\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0ak\xCBV[\x90P`\0aQ\x96aQ\x8B\x83\x85ak\xCBV[`\x0F\x88\x90\x0B\x90aF\xB3V[` \x8A\x01Q\x90\x91PaQ\xAB\x90`\x0F\x0B\x88aF\xB3V[`\x0F\x0B` \x8A\x01R`\0aQ\xC7\x83g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[\x8AQ\x90\x91PaQ\xD9\x90`\x0F\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B\x8BR\x82\x90\x0B\x15aRlWc\xFF\xFF\xFF\xFF\x8B\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81RaR#\x8B\x82\x85aN\xE1V[c\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`m` \x90\x81R`@\x80\x83 \x83\x80R\x90\x91R\x81 \x82Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x90UaRj\x90\x8D\x90aL)V[P[c\xFF\xFF\xFF\xFF\x8B\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x15aS-W`\0aR\xBEaR\x9Cc\x01\xE13\x80aZ\xC2V[c\xFF\xFF\xFF\xFF\x80\x8F\x16`\0\x90\x81R`q` R`@\x90 T`\x0F\x0B\x91\x90aK\x0B\x16V[\x90P`\0aR\xD8\x8BaQ3\x84g\r\xE0\xB6\xB3\xA7d\0\0ag\x9BV[` \x8D\x01Q\x90\x91PaR\xED\x90`\x0F\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B` \x8E\x01R\x8CQaS\x04\x91\x0B\x82aF\xB3V[`\x0F\x90\x81\x0B\x8DRaS\x18\x90\x84\x90\x0B\x82aF\xB3V[\x92PaS(`\x0F\x8B\x90\x0B\x82aF\xB3V[\x99PPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`\x80\x1B\x03\x8B\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8A\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPV[`\0\x82`\x0F\x0B`\0\x14\x80aS\xB2WP\x81`\x0F\x0B`\0\x14[\x80aS\xCAWP`\0aS\xC4\x86\x85ag\x9BV[`\x0F\x0B\x13\x15[\x80aS\xE2WP`\0aS\xDC\x85\x84ag\x9BV[`\x0F\x0B\x13\x15[\x15aS\xEFWP`\0aI\xACV[`\0\x82`\x0F\x0B\x84`\x0F\x0BaT\x03\x91\x90al\xEFV[\x90P`\0aT\x11\x86\x85ag\x9BV[`\x0F\x0BaT\x1E\x88\x87ag\x9BV[`\x0F\x0BaT+\x91\x90al\xEFV[\x91\x90\x91\x13\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16aTLW`\0\x80\xFD[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15aTuWPc;\x9A\xCA\0\x81`@\x01Q`\x03\x0B\x13\x15[\x80\x15aT\x8FWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15aT\xA6WPc;\x9A\xCA\0\x81``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90aT\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x80\x7F\xB6\xADGC\xE8\x8E,\x9F@\xD4\x8EV(|\xC4\xF2\xC3\x011\x81\xBB\x12\x8D\x12\xCA!\x14\xC4\x8F\xE2\x92fc\xFF\xFF\xFF\xFF\x89\x81\x16`\0\x81\x81R` \x93\x84R`@\x90\x81\x90 \x85Q\x81T\x95\x87\x01Q\x87\x84\x01Q``\x89\x01Q`\x80\x90\x99\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x99\x88\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x89\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x89\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x9A\x16\x94\x90\x98\x16\x93\x90\x93\x17\x97\x90\x97\x17\x16\x94\x90\x94\x17\x94\x90\x94\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91U`fT\x90QcC\xB1j\x11`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x87b\xD4\"\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aU\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15aV\x03W=`\0\x80>=`\0\xFD[PPPP`h\x87\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\0`\x01`h\x80T\x90PaVi\x91\x90am\xEAV[\x90P[\x80\x15aX,W`haV\x7F`\x01\x83am\xEAV[\x81T\x81\x10aV\x8FWaV\x8FagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16`h\x82\x81T\x81\x10aV\xCEWaV\xCEagoV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15aX\x15W`\0`h\x82\x81T\x81\x10aW\rWaW\ragoV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`haW@`\x01\x84am\xEAV[\x81T\x81\x10aWPWaWPagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16`h\x83\x81T\x81\x10aW\x89WaW\x89agoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`h`\x01\x84aW\xCC\x91\x90am\xEAV[\x81T\x81\x10aW\xDCWaW\xDCagoV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPaX\x1AV[aX,V[\x80aX$\x81an\x01V[\x91PPaVlV[PaX5aL\xC1V[`@Qc-\xA1\xC5\x9B`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04\x83\x01R\x88\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`D\x83\x01R`\x0F\x87\x81\x0B`d\x84\x01R\x86\x81\x0B`\x84\x84\x01R\x85\x90\x0B`\xA4\x83\x01R\x91\x90\x91\x16\x90c-\xA1\xC5\x9B\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aX\xA8W`\0\x80\xFD[PZ\xF1\x15\x80\x15aX\xBCW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12aO\x8AW\x81a\x1C\xBEV[`\0Ta\x01\0\x90\x04`\xFF\x16aY\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\r\xFFV[a\x15qa[\xC3V[`\0\x80\x82\x12\x15aY\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Fds-math-sqrt-non-positive\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xFFV[`\x03\x82\x13\x15aZ<WP\x80`\0aY\xF5`\x02\x83amvV[aZ\0\x90`\x01an\x18V[\x90P[\x81\x81\x12\x15aZ6W\x90P\x80`\x02\x81aZ\x1B\x81\x86amvV[aZ%\x91\x90an\x18V[aZ/\x91\x90amvV[\x90PaZ\x03V[P\x91\x90PV[\x81\x15aZFWP`\x01[\x91\x90PV[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15aZdWP\x82QaZkV[P` \x83\x01Q[\x82Q`\0\x90\x83\x90aZ\x7F\x90`\x0F\x0B\x84aF\xB3V[aZ\x89\x91\x90ag\x9BV[\x90P`\0\x81`\x0F\x0B\x13\x15aZ\xA0W\x84Q\x91PaZ\xA8V[\x84` \x01Q\x91P[aZ\xB6`\x0F\x82\x90\x0B\x83aK\x0BV[`\x0F\x0B\x90\x93RPPPPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90aZ\xFBWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a[4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a[}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xFF\x91\x90ah V[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13aG.W\x80\x84\x16`\x0F\x0B\x15a[\xAFWa[\xAC\x82\x86aF\xB3V[\x91P[a[\xB9\x85\x86aF\xB3V[\x94P`\x02\x02a[\x8AV[`\0Ta\x01\0\x90\x04`\xFF\x16a\\.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\r\xFFV[a\x15q3aG\x9DV[`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01a\\o`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x81R` \x01a\\\x97`@Q\x80`@\x01`@R\x80`\0`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81RP\x90V[\x90R\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a@#W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15aZ6W`\0\x80\xFD[`\0\x80`\xC0\x83\x85\x03\x12\x15a\\\xD3W`\0\x80\xFD[\x825a\\\xDE\x81a\\\x9CV[\x91Pa\\\xED\x84` \x85\x01a\\\xAEV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a]\x08W`\0\x80\xFD[\x815a\x1C\xBE\x81a\\\x9CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a@#W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a]@W`\0\x80\xFD[\x855a]K\x81a]\x13V[\x94P` \x86\x015a][\x81a]\x13V[\x93P`@\x86\x015a]k\x81a]\x13V[\x92P``\x86\x015a]{\x81a]\x13V[\x91P`\x80\x86\x015a]\x8B\x81a]\x13V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x80`\x0F\x0B\x81\x14a@#W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a]\xBBW`\0\x80\xFD[\x825a]\xC6\x81a\\\x9CV[\x91P` \x83\x015a]\xD6\x81a]\x99V[\x80\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a^\x14W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a]\xF5V[P\x94\x95\x94PPPPPV[`@\x81R`\0a^2`@\x83\x01\x85a]\xE1V[\x82\x81\x03` \x84\x01Ra\x15\xFF\x81\x85a]\xE1V[`\0\x80\x83`\x1F\x84\x01\x12a^VW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^nW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a>eW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a^\x9FW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a^\xB7W`\0\x80\xFD[a^\xC3\x88\x83\x89\x01a^DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a^\xDCW`\0\x80\xFD[Pa^\xE9\x87\x82\x88\x01a^DV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a_\x08W`\0\x80\xFD[\x825a_\x13\x81a\\\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[\x80Q`\x0F\x0B\x82R` \x81\x01Qa_M` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[P`@\x01Q\x80Q`\x0F\x90\x81\x0B``\x84\x01R` \x90\x91\x01Q\x90\x0B`\x80\x90\x91\x01RV[a\x01\x80\x81\x01a_}\x82\x87a_!V[\x84Q`\x0F\x90\x81\x0B`\xA0\x84\x01R\x84Q\x81\x0B`\xC0\x84\x01R` \x80\x86\x01Q\x82\x0B`\xE0\x85\x01R`@\x86\x01Q\x82\x0Ba\x01\0\x85\x01R``\x86\x01Q\x82\x0Ba\x01 \x85\x01R\x84Q\x82\x0Ba\x01@\x85\x01R\x84\x01Q\x90\x0Ba\x01`\x83\x01Ra\x15\xFFV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a`\x0BWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a`OW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a`-V[P\x90\x96\x95PPPPPPV[`\0a\x01\xC0\x82\x84\x03\x12\x15aZ6W`\0\x80\xFD[`@\x81\x01a\x14\xCA\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01RPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a`\xA3W`\0\x80\xFD[\x835a`\xAE\x81a\\\x9CV[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a`\xC2W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10aZFW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a`\xF2W`\0\x80\xFD[\x825\x91Pa\\\xED` \x84\x01a`\xD0V[`\0` \x82\x84\x03\x12\x15aa\x14W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aa0W`\0\x80\xFD[\x835\x92P` \x84\x015aaB\x81a\\\x9CV[\x91PaaP`@\x85\x01a`\xD0V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aaqW`\0\x80\xFD[\x855aa|\x81a\\\x9CV[\x94P` \x86\x015\x93P`@\x86\x015aa\x93\x81a]\x99V[\x92P``\x86\x015aa\xA3\x81a]\x99V[\x91P`\x80\x86\x015a]\x8B\x81a]\x99V[`\0` \x82\x84\x03\x12\x15aa\xC5W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\xBEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aa\xEFW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15abEWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15abEWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14aZFW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15ab\xA0W`\0\x80\xFD[ab\xA8ab\x14V[\x90Pab\xB3\x82ab|V[\x81Rab\xC1` \x83\x01ab|V[` \x82\x01Rab\xD2`@\x83\x01ab|V[`@\x82\x01Rab\xE3``\x83\x01ab|V[``\x82\x01R`\x80\x82\x015ab\xF6\x81a]\x99V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15ac\x14W`\0\x80\xFD[\x825ac\x1F\x81a\\\x9CV[\x91Pa\\\xED\x84` \x85\x01ab\x8EV[`\0\x80`\0``\x84\x86\x03\x12\x15acCW`\0\x80\xFD[\x835acN\x81a\\\x9CV[\x92P` \x84\x015ac^\x81a]\x99V[\x91P`@\x84\x015acn\x81a]\x99V[\x80\x91PP\x92P\x92P\x92V[`\xA0\x81\x01a\x14\xCA\x82\x84a_!V[`\0\x80` \x83\x85\x03\x12\x15ac\x9AW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15ac\xB2W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12ac\xC6W`\0\x80\xFD[\x815\x81\x81\x11\x15ac\xD5W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15ac\xE7W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x02\0\x89\x8B\x03\x12\x15ad\x16W`\0\x80\xFD[\x885ad!\x81a\\\x9CV[\x97P` \x89\x015ad1\x81a\\\x9CV[\x96P`@\x89\x015adA\x81a]\x13V[\x95P``\x89\x015adQ\x81a]\x99V[\x94P`\x80\x89\x015ada\x81a]\x99V[\x93P`\xA0\x89\x015adq\x81a]\x99V[\x92Pad\x80\x8A`\xC0\x8B\x01a\\\xAEV[\x91Pad\x90\x8Aa\x01`\x8B\x01a\\\xAEV[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15ad\xB4W`\0\x80\xFD[\x835ad\xBF\x81a\\\x9CV[\x92P` \x84\x015\x91P`@\x84\x015acn\x81a]\x99V[`\xC0\x81\x01ae\x11\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\x80\x84\x01R` \x84\x01Q\x90\x0B`\xA0\x83\x01Ra\x1C\xBEV[`\xA0\x81\x01a\x14\xCA\x82\x84`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\x80\x81\x01a\x14\xCA\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15ae\xC9W`\0\x80\xFD[\x815a\x1C\xBE\x81a]\x13V[`\0\x80`@\x83\x85\x03\x12\x15ae\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015a]\xD6\x81a]\x99V[`\0\x80`\0\x83\x85\x03``\x81\x12\x15af\x0FW`\0\x80\xFD[\x845af\x1A\x81a\\\x9CV[\x93P` \x85\x81\x015\x93P`?\x19\x82\x01\x12\x15af4W`\0\x80\xFD[P`@\x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15afXW`\0\x80\xFD[\x845afc\x81a\\\x9CV[\x93P` \x85\x015\x92P`@\x85\x015afz\x81a]\x99V[\x91P``\x85\x015af\x8A\x81a]\x99V[\x93\x96\x92\x95P\x90\x93PPV[\x815af\xA0\x81a]\x13V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015af\xCC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015af\xF4\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015ag%\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015agM\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\n\xB9V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15ag\xC5Wag\xC5ag\x85V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15ag\xE1Wag\xE1ag\x85V[P\x01\x93\x92PPPV[`\0`\x01\x82\x01ag\xFCWag\xFCag\x85V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15ah\x15W`\0\x80\xFD[\x815a\x1C\xBE\x81a]\x99V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15ahMW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01ah1V[\x81\x81\x11\x15ah_W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03ah\x91Wah\x91ag\x85V[`\x01\x01\x93\x92PPPV[`\0\x81\x83\x03a\x01\xC0\x81\x12\x15ah\xAFW`\0\x80\xFD[ah\xB7abKV[\x835ah\xC2\x81a\\\x9CV[\x81R` \x84\x015ah\xD2\x81a]\x99V[` \x82\x01R`@\x84\x015ah\xE5\x81a]\x99V[`@\x82\x01R``\x84\x015ah\xF8\x81a]\x99V[``\x82\x01R`\xA0`\x7F\x19\x83\x01\x12\x15ai\x0FW`\0\x80\xFD[ai\x17ab\x14V[\x91P`\x80\x84\x015ai'\x81a]\x13V[\x82R`\xA0\x84\x015ai7\x81a]\x99V[` \x83\x01R`\xC0\x84\x015aiJ\x81a]\x99V[`@\x83\x01R`\xE0\x84\x015ai]\x81a]\x99V[``\x83\x01Ra\x01\0\x84\x015aiq\x81a]\x99V[\x80`\x80\x84\x01RP\x81`\x80\x82\x01Rai\x8C\x85a\x01 \x86\x01ab\x8EV[`\xA0\x82\x01R\x94\x93PPPPV[\x815ai\xA4\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015ai\xCC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015ai\xFC\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015agM\x81a]\x99V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15ajTWajTag\x85V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aj\x80Waj\x80ag\x85V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aj\x9CWaj\x9Cag\x85V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aj\xB2Waj\xB2ag\x85V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03ah\x91Wah\x91ag\x85V[`\0`\x80\x82\x84\x03\x12\x15aj\xEDW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15ak\x1EWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x82Qak,\x81a\\\x9CV[\x81R` \x83\x01Qak<\x81a]\x99V[` \x82\x01R`@\x83\x01QakO\x81a]\x99V[`@\x82\x01R``\x83\x01Qakb\x81a]\x99V[``\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03ak\x8BWak\x8Bag\x85V[`\0\x03\x92\x91PPV[\x815ak\x9F\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x01\x81\x01` \x83\x015af\xCC\x81a]\x99V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15ak\xF6Wak\xF6ag\x85V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15al\x11Wal\x11ag\x85V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15al=Wal=ag\x85V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15almWalmag\x85V[\x02\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80al\xA3Wal\xA3alvV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15al\xCAWal\xCAag\x85V[\x90\x05\x93\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15al\xE5W`\0\x80\xFD[a\x1C\xBE\x83\x83ab\x8EV[`\0`\x01`\x01`\xFF\x1B\x03`\0\x84\x13`\0\x84\x13\x85\x83\x04\x85\x11\x82\x82\x16\x16\x15am\x17Wam\x17ag\x85V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15am6Wam6ag\x85V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15amRWamRag\x85V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15amhWamhag\x85V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0\x82am\x85Wam\x85alvV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15am\x9FWam\x9Fag\x85V[P\x05\x90V[\x815am\xAF\x81a]\x99V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x91\x90\x91\x16\x17\x90UPV[`\0` \x82\x84\x03\x12\x15am\xDFW`\0\x80\xFD[\x81Qa\x1C\xBE\x81a]\x13V[`\0\x82\x82\x10\x15am\xFCWam\xFCag\x85V[P\x03\x90V[`\0\x81an\x10Wan\x10ag\x85V[P`\0\x19\x01\x90V[`\0\x80\x82\x12\x82`\x01`\x01`\xFF\x1B\x03\x03\x84\x13\x81\x15\x16\x15an9Wan9ag\x85V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15anRWanRag\x85V[PP\x01\x90V\xFE\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFD\xA2dipfsX\"\x12 i{\xB7\xE7\xA5\x8BSu\xD4xZ\xD1\xFB\xEFw(\xC2LU\xF9\x18\xE9\xEB\x17\xE4_\x8AF\x10\xE6\xFA\xB0dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static SPOTENGINE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SpotEngine<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SpotEngine<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SpotEngine<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SpotEngine<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SpotEngine<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SpotEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SpotEngine<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SPOTENGINE_ABI.clone(),
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
                SPOTENGINE_ABI.clone(),
                SPOTENGINE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addProduct` (0xd386c1e8) function
        pub fn add_product(
            &self,
            product_id: u32,
            quote_id: u32,
            book: ::ethers::core::types::Address,
            size_increment: i128,
            min_size: i128,
            lp_spread_x18: i128,
            config: Config,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [211, 134, 193, 232],
                    (
                        product_id,
                        quote_id,
                        book,
                        size_increment,
                        min_size,
                        lp_spread_x18,
                        config,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertUtilization` (0x4ac8d8c1) function
        pub fn assert_utilization(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 200, 216, 193], product_id)
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
        ///Calls the contract's `decomposeLps` (0xb8d80d8b) function
        pub fn decompose_lps(
            &self,
            liquidatee: [u8; 32],
            liquidator: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([184, 216, 13, 139], (liquidatee, liquidator))
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
        ///Calls the contract's `getClearinghouse` (0xb1cb0f42) function
        pub fn get_clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 203, 15, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xe343738c) function
        pub fn get_config(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Config> {
            self.0
                .method_hash([227, 67, 115, 140], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCoreRisk` (0x8a1d43c9) function
        pub fn get_core_risk(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, CoreRisk> {
            self.0
                .method_hash([138, 29, 67, 201], (subaccount, product_id, health_type))
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
        ///Calls the contract's `getHealthContribution` (0x871d0912) function
        pub fn get_health_contribution(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([135, 29, 9, 18], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinDepositRate` (0x130ea373) function
        pub fn get_min_deposit_rate(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([19, 14, 163, 115], product_id)
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
        ) -> ::ethers::contract::builders::ContractCall<M, Balances> {
            self.0
                .method_hash([237, 240, 38, 83], (product_id, subaccount))
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
        ///Calls the contract's `getRisk` (0xecd9cba8) function
        pub fn get_risk(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Risk> {
            self.0
                .method_hash([236, 217, 203, 168], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlots` (0xfab2c469) function
        pub fn get_slots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([250, 178, 196, 105], ())
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
        ///Calls the contract's `getToken` (0x45be7ed6) function
        pub fn get_token(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([69, 190, 126, 214], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenBalance` (0xa67ac322) function
        pub fn get_token_balance(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([166, 122, 195, 34], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalBalances` (0x2baf57d3) function
        pub fn get_total_balances(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i128>, ::std::vec::Vec<i128>),
        > {
            self.0
                .method_hash([43, 175, 87, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWithdrawFee` (0xfdf4a0c0) function
        pub fn get_withdraw_fee(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([253, 244, 160, 192], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, offchain_exchange, quote, endpoint, admin),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0x30972b50) function
        pub fn manual_assert(
            &self,
            total_deposits: ::std::vec::Vec<i128>,
            total_borrows: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 151, 43, 80], (total_deposits, total_borrows))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrationFlag` (0xc362d19e) function
        pub fn migration_flag(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([195, 98, 209, 158], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x10ca9388) function
        pub fn set_config(
            &self,
            product_id: u32,
            config: Config,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 202, 147, 136], (product_id, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLpBalance` (0xf8661884) function
        pub fn set_lp_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            lp_balance: LpBalance,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 102, 24, 132], (product_id, subaccount, lp_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLpState` (0x9bb91f6a) function
        pub fn set_lp_state(
            &self,
            product_id: u32,
            lp_state: LpState,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 185, 31, 106], (product_id, lp_state))
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
        ///Calls the contract's `socializeSubaccount` (0x8936f7cd) function
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 54, 247, 205], subaccount)
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedUpdateProductTx` (0x49f73568) function
        pub fn unsigned_update_product_tx(
            &self,
            p: UpdateProductTx,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateProductTx> {
            self.0
                .method_hash([73, 247, 53, 104], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBalance` (0xe0b0621f) function
        pub fn update_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 176, 98, 31], (product_id, subaccount, amount_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBalance` (0xf8a42e51) function
        pub fn update_balance_with_product_id_and_subaccount_and_quote_delta(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
            quote_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 164, 46, 81],
                    (product_id, subaccount, amount_delta, quote_delta),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMinDepositRate` (0xec6271d2) function
        pub fn update_min_deposit_rate(
            &self,
            product_id: u32,
            min_deposit_rate_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 98, 113, 210], (product_id, min_deposit_rate_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePrice` (0x153ca6c0) function
        pub fn update_price(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 60, 166, 192], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateProduct` (0xc9fe9ac3) function
        pub fn update_product(
            &self,
            raw_txn: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 254, 154, 195], raw_txn)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateQuoteFromInsurance` (0xf39eeb10) function
        pub fn update_quote_from_insurance(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([243, 158, 235, 16], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRisk` (0xc55607b5) function
        pub fn update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 86, 7, 181], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateStates` (0xad733b8e) function
        pub fn update_states(&self, dt: u128) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 115, 59, 142], dt)
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
        ///Gets the contract's `InterestPayment` event
        pub fn interest_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InterestPaymentFilter>
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SpotEngineEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SpotEngine<M> {
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
    #[ethevent(name = "AddProduct", abi = "AddProduct(uint32)")]
    pub struct AddProductFilter {
        pub product_id: u32,
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
    #[ethevent(name = "BalanceUpdate", abi = "BalanceUpdate(uint32,bytes32)")]
    pub struct BalanceUpdateFilter {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
        name = "InterestPayment",
        abi = "InterestPayment(uint32,uint128,int128,int128,int128)"
    )]
    pub struct InterestPaymentFilter {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub dt: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub deposit_rate_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub borrow_rate_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub fee_amount: i128,
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
    #[ethevent(name = "ProductUpdate", abi = "ProductUpdate(uint32)")]
    pub struct ProductUpdateFilter {
        pub product_id: u32,
    }
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
    pub enum SpotEngineEvents {
        AddProductFilter(AddProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        InterestPaymentFilter(InterestPaymentFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProductUpdateFilter(ProductUpdateFilter),
    }
    impl ::ethers::contract::EthLogDecode for SpotEngineEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddProductFilter::decode_log(log) {
                return Ok(SpotEngineEvents::AddProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SpotEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = InterestPaymentFilter::decode_log(log) {
                return Ok(SpotEngineEvents::InterestPaymentFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SpotEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::ProductUpdateFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SpotEngineEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProductFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterestPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductFilter> for SpotEngineEvents {
        fn from(value: AddProductFilter) -> Self {
            Self::AddProductFilter(value)
        }
    }
    impl ::core::convert::From<BalanceUpdateFilter> for SpotEngineEvents {
        fn from(value: BalanceUpdateFilter) -> Self {
            Self::BalanceUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SpotEngineEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<InterestPaymentFilter> for SpotEngineEvents {
        fn from(value: InterestPaymentFilter) -> Self {
            Self::InterestPaymentFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SpotEngineEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProductUpdateFilter> for SpotEngineEvents {
        fn from(value: ProductUpdateFilter) -> Self {
            Self::ProductUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `addProduct` function with signature `addProduct(uint32,uint32,address,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0xd386c1e8`
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
        name = "addProduct",
        abi = "addProduct(uint32,uint32,address,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct AddProductCall {
        pub product_id: u32,
        pub quote_id: u32,
        pub book: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub lp_spread_x18: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `assertUtilization` function with signature `assertUtilization(uint32)` and selector `0x4ac8d8c1`
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
    #[ethcall(name = "assertUtilization", abi = "assertUtilization(uint32)")]
    pub struct AssertUtilizationCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `0xd98752ec`
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
    #[ethcall(name = "burnLp", abi = "burnLp(uint32,bytes32,int128)")]
    pub struct BurnLpCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_lp: i128,
    }
    ///Container type for all input parameters for the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32)` and selector `0xb8d80d8b`
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
    #[ethcall(name = "decomposeLps", abi = "decomposeLps(bytes32,bytes32)")]
    pub struct DecomposeLpsCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub liquidator: [u8; 32],
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
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
    #[ethcall(name = "getBalance", abi = "getBalance(uint32,bytes32)")]
    pub struct GetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint32)` and selector `0xe343738c`
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
    #[ethcall(name = "getConfig", abi = "getConfig(uint32)")]
    pub struct GetConfigCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `0x8a1d43c9`
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
    #[ethcall(name = "getCoreRisk", abi = "getCoreRisk(bytes32,uint32,uint8)")]
    pub struct GetCoreRiskCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
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
    #[ethcall(name = "getEndpoint", abi = "getEndpoint()")]
    pub struct GetEndpointCall;
    ///Container type for all input parameters for the `getEngineType` function with signature `getEngineType()` and selector `0x4604d19b`
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
    #[ethcall(name = "getEngineType", abi = "getEngineType()")]
    pub struct GetEngineTypeCall;
    ///Container type for all input parameters for the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `0x871d0912`
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
        name = "getHealthContribution",
        abi = "getHealthContribution(bytes32,uint8)"
    )]
    pub struct GetHealthContributionCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getMinDepositRate` function with signature `getMinDepositRate(uint32)` and selector `0x130ea373`
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
    #[ethcall(name = "getMinDepositRate", abi = "getMinDepositRate(uint32)")]
    pub struct GetMinDepositRateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPoolState` function with signature `getPoolState(uint32)` and selector `0x8af6426a`
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
    #[ethcall(name = "getPoolState", abi = "getPoolState(uint32)")]
    pub struct GetPoolStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
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
    #[ethcall(name = "getProductIds", abi = "getProductIds()")]
    pub struct GetProductIdsCall;
    ///Container type for all input parameters for the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `0xedf02653`
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
    #[ethcall(name = "getRawBalance", abi = "getRawBalance(uint32,bytes32)")]
    pub struct GetRawBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `0xc721bd65`
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
    #[ethcall(name = "getRawLpState", abi = "getRawLpState(uint32)")]
    pub struct GetRawLpStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getRawState` function with signature `getRawState(uint32)` and selector `0xec7a79c9`
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
    #[ethcall(name = "getRawState", abi = "getRawState(uint32)")]
    pub struct GetRawStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
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
    #[ethcall(name = "getRisk", abi = "getRisk(uint32)")]
    pub struct GetRiskCall {
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
    ///Container type for all input parameters for the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
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
        name = "getStateAndBalance",
        abi = "getStateAndBalance(uint32,bytes32)"
    )]
    pub struct GetStateAndBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getStatesAndBalances` function with signature `getStatesAndBalances(uint32,bytes32)` and selector `0x3d5cc9dc`
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
        name = "getStatesAndBalances",
        abi = "getStatesAndBalances(uint32,bytes32)"
    )]
    pub struct GetStatesAndBalancesCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getToken` function with signature `getToken(uint32)` and selector `0x45be7ed6`
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
    #[ethcall(name = "getToken", abi = "getToken(uint32)")]
    pub struct GetTokenCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `0xa67ac322`
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
    #[ethcall(name = "getTokenBalance", abi = "getTokenBalance(uint32)")]
    pub struct GetTokenBalanceCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getTotalBalances` function with signature `getTotalBalances()` and selector `0x2baf57d3`
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
    #[ethcall(name = "getTotalBalances", abi = "getTotalBalances()")]
    pub struct GetTotalBalancesCall;
    ///Container type for all input parameters for the `getWithdrawFee` function with signature `getWithdrawFee(uint32)` and selector `0xfdf4a0c0`
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
    #[ethcall(name = "getWithdrawFee", abi = "getWithdrawFee(uint32)")]
    pub struct GetWithdrawFeeCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address)` and selector `0x1459457a`
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
        abi = "initialize(address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert(int128[],int128[])` and selector `0x30972b50`
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
    #[ethcall(name = "manualAssert", abi = "manualAssert(int128[],int128[])")]
    pub struct ManualAssertCall {
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
    ///Container type for all input parameters for the `migrationFlag` function with signature `migrationFlag()` and selector `0xc362d19e`
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
    #[ethcall(name = "migrationFlag", abi = "migrationFlag()")]
    pub struct MigrationFlagCall;
    ///Container type for all input parameters for the `mintLp` function with signature `mintLp(uint32,bytes32,int128,int128,int128)` and selector `0x98de72fe`
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
    #[ethcall(name = "mintLp", abi = "mintLp(uint32,bytes32,int128,int128,int128)")]
    pub struct MintLpCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_base: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_amount_low: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_amount_high: i128,
    }
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
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint32,(address,int128,int128,int128,int128))` and selector `0x10ca9388`
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
        name = "setConfig",
        abi = "setConfig(uint32,(address,int128,int128,int128,int128))"
    )]
    pub struct SetConfigCall {
        pub product_id: u32,
        pub config: Config,
    }
    ///Container type for all input parameters for the `setLpBalance` function with signature `setLpBalance(uint32,bytes32,(int128))` and selector `0xf8661884`
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
    #[ethcall(name = "setLpBalance", abi = "setLpBalance(uint32,bytes32,(int128))")]
    pub struct SetLpBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub lp_balance: LpBalance,
    }
    ///Container type for all input parameters for the `setLpState` function with signature `setLpState(uint32,(int128,(int128,int128),(int128,int128)))` and selector `0x9bb91f6a`
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
        name = "setLpState",
        abi = "setLpState(uint32,(int128,(int128,int128),(int128,int128)))"
    )]
    pub struct SetLpStateCall {
        pub product_id: u32,
        pub lp_state: LpState,
    }
    ///Container type for all input parameters for the `setState` function with signature `setState(uint32,(int128,int128,int128,int128))` and selector `0x7fa29d49`
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
        name = "setState",
        abi = "setState(uint32,(int128,int128,int128,int128))"
    )]
    pub struct SetStateCall {
        pub product_id: u32,
        pub state: State,
    }
    ///Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32)` and selector `0x8936f7cd`
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
    #[ethcall(name = "socializeSubaccount", abi = "socializeSubaccount(bytes32)")]
    pub struct SocializeSubaccountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `0xc7167cf5`
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
    #[ethcall(name = "swapLp", abi = "swapLp(uint32,int128,int128)")]
    pub struct SwapLpCall {
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
    ///Container type for all input parameters for the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `0x49f73568`
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
        name = "unsignedUpdateProductTx",
        abi = "unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))"
    )]
    pub struct UnsignedUpdateProductTxCall {
        pub p: UpdateProductTx,
    }
    ///Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128)` and selector `0xe0b0621f`
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
    #[ethcall(name = "updateBalance", abi = "updateBalance(uint32,bytes32,int128)")]
    pub struct UpdateBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_delta: i128,
    }
    ///Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128,int128)` and selector `0xf8a42e51`
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
        name = "updateBalance",
        abi = "updateBalance(uint32,bytes32,int128,int128)"
    )]
    pub struct UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_delta: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_delta: i128,
    }
    ///Container type for all input parameters for the `updateMinDepositRate` function with signature `updateMinDepositRate(uint32,int128)` and selector `0xec6271d2`
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
        abi = "updateMinDepositRate(uint32,int128)"
    )]
    pub struct UpdateMinDepositRateCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_deposit_rate_x18: i128,
    }
    ///Container type for all input parameters for the `updatePrice` function with signature `updatePrice(uint32,int128)` and selector `0x153ca6c0`
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
    #[ethcall(name = "updatePrice", abi = "updatePrice(uint32,int128)")]
    pub struct UpdatePriceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all input parameters for the `updateProduct` function with signature `updateProduct(bytes)` and selector `0xc9fe9ac3`
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
    #[ethcall(name = "updateProduct", abi = "updateProduct(bytes)")]
    pub struct UpdateProductCall {
        pub raw_txn: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `0xf39eeb10`
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
        name = "updateQuoteFromInsurance",
        abi = "updateQuoteFromInsurance(bytes32,int128)"
    )]
    pub struct UpdateQuoteFromInsuranceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub insurance: i128,
    }
    ///Container type for all input parameters for the `updateRisk` function with signature `updateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0xc55607b5`
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
        name = "updateRisk",
        abi = "updateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct UpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `updateStates` function with signature `updateStates(uint128)` and selector `0xad733b8e`
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
    #[ethcall(name = "updateStates", abi = "updateStates(uint128)")]
    pub struct UpdateStatesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub dt: u128,
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
    pub enum SpotEngineCalls {
        AddProduct(AddProductCall),
        AssertUtilization(AssertUtilizationCall),
        BurnLp(BurnLpCall),
        DecomposeLps(DecomposeLpsCall),
        GetBalance(GetBalanceCall),
        GetClearinghouse(GetClearinghouseCall),
        GetConfig(GetConfigCall),
        GetCoreRisk(GetCoreRiskCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetHealthContribution(GetHealthContributionCall),
        GetMinDepositRate(GetMinDepositRateCall),
        GetPoolState(GetPoolStateCall),
        GetProductIds(GetProductIdsCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawLpState(GetRawLpStateCall),
        GetRawState(GetRawStateCall),
        GetRisk(GetRiskCall),
        GetSlots(GetSlotsCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        GetStatesAndBalances(GetStatesAndBalancesCall),
        GetToken(GetTokenCall),
        GetTokenBalance(GetTokenBalanceCall),
        GetTotalBalances(GetTotalBalancesCall),
        GetWithdrawFee(GetWithdrawFeeCall),
        Initialize(InitializeCall),
        ManualAssert(ManualAssertCall),
        MigrationFlag(MigrationFlagCall),
        MintLp(MintLpCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetConfig(SetConfigCall),
        SetLpBalance(SetLpBalanceCall),
        SetLpState(SetLpStateCall),
        SetState(SetStateCall),
        SocializeSubaccount(SocializeSubaccountCall),
        SwapLp(SwapLpCall),
        TransferOwnership(TransferOwnershipCall),
        UnsignedUpdateProductTx(UnsignedUpdateProductTxCall),
        UpdateBalance(UpdateBalanceCall),
        UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(
            UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall,
        ),
        UpdateMinDepositRate(UpdateMinDepositRateCall),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
        UpdateQuoteFromInsurance(UpdateQuoteFromInsuranceCall),
        UpdateRisk(UpdateRiskCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpotEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddProductCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddProduct(decoded));
            }
            if let Ok(decoded) =
                <AssertUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssertUtilization(decoded));
            }
            if let Ok(decoded) = <BurnLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BurnLp(decoded));
            }
            if let Ok(decoded) = <DecomposeLpsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DecomposeLps(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
            }
            if let Ok(decoded) = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded) = <GetCoreRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCoreRisk(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) = <GetEngineTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEngineType(decoded));
            }
            if let Ok(decoded) =
                <GetHealthContributionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHealthContribution(decoded));
            }
            if let Ok(decoded) =
                <GetMinDepositRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMinDepositRate(decoded));
            }
            if let Ok(decoded) = <GetPoolStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolState(decoded));
            }
            if let Ok(decoded) = <GetProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetProductIds(decoded));
            }
            if let Ok(decoded) = <GetRawBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawBalance(decoded));
            }
            if let Ok(decoded) = <GetRawLpStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawLpState(decoded));
            }
            if let Ok(decoded) = <GetRawStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRawState(decoded));
            }
            if let Ok(decoded) = <GetRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRisk(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
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
            if let Ok(decoded) = <GetTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetToken(decoded));
            }
            if let Ok(decoded) =
                <GetTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTokenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetTotalBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalBalances(decoded));
            }
            if let Ok(decoded) =
                <GetWithdrawFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetWithdrawFee(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MigrationFlagCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MigrationFlag(decoded));
            }
            if let Ok(decoded) = <MintLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintLp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
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
            if let Ok(decoded) =
                <SocializeSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) = <SwapLpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapLp(decoded));
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
            if let Ok(decoded) = <UpdateBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateBalance(decoded));
            }
            if let Ok(decoded) = <UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(decoded),
                );
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
            if let Ok(decoded) =
                <UpdateQuoteFromInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateQuoteFromInsurance(decoded));
            }
            if let Ok(decoded) = <UpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateRisk(decoded));
            }
            if let Ok(decoded) = <UpdateStatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateStates(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpotEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssertUtilization(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DecomposeLps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCoreRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEngineType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHealthContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMinDepositRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetProductIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawLpState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStateAndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStatesAndBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTokenBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTotalBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetWithdrawFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrationFlag(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLpState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SocializeSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedUpdateProductTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMinDepositRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateQuoteFromInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateStates(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SpotEngineCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecomposeLps(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCoreRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthContribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinDepositRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawLpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStateAndBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStatesAndBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWithdrawFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrationFlag(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SocializeSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLp(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedUpdateProductTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateMinDepositRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateQuoteFromInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductCall> for SpotEngineCalls {
        fn from(value: AddProductCall) -> Self {
            Self::AddProduct(value)
        }
    }
    impl ::core::convert::From<AssertUtilizationCall> for SpotEngineCalls {
        fn from(value: AssertUtilizationCall) -> Self {
            Self::AssertUtilization(value)
        }
    }
    impl ::core::convert::From<BurnLpCall> for SpotEngineCalls {
        fn from(value: BurnLpCall) -> Self {
            Self::BurnLp(value)
        }
    }
    impl ::core::convert::From<DecomposeLpsCall> for SpotEngineCalls {
        fn from(value: DecomposeLpsCall) -> Self {
            Self::DecomposeLps(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for SpotEngineCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for SpotEngineCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for SpotEngineCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetCoreRiskCall> for SpotEngineCalls {
        fn from(value: GetCoreRiskCall) -> Self {
            Self::GetCoreRisk(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for SpotEngineCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetEngineTypeCall> for SpotEngineCalls {
        fn from(value: GetEngineTypeCall) -> Self {
            Self::GetEngineType(value)
        }
    }
    impl ::core::convert::From<GetHealthContributionCall> for SpotEngineCalls {
        fn from(value: GetHealthContributionCall) -> Self {
            Self::GetHealthContribution(value)
        }
    }
    impl ::core::convert::From<GetMinDepositRateCall> for SpotEngineCalls {
        fn from(value: GetMinDepositRateCall) -> Self {
            Self::GetMinDepositRate(value)
        }
    }
    impl ::core::convert::From<GetPoolStateCall> for SpotEngineCalls {
        fn from(value: GetPoolStateCall) -> Self {
            Self::GetPoolState(value)
        }
    }
    impl ::core::convert::From<GetProductIdsCall> for SpotEngineCalls {
        fn from(value: GetProductIdsCall) -> Self {
            Self::GetProductIds(value)
        }
    }
    impl ::core::convert::From<GetRawBalanceCall> for SpotEngineCalls {
        fn from(value: GetRawBalanceCall) -> Self {
            Self::GetRawBalance(value)
        }
    }
    impl ::core::convert::From<GetRawLpStateCall> for SpotEngineCalls {
        fn from(value: GetRawLpStateCall) -> Self {
            Self::GetRawLpState(value)
        }
    }
    impl ::core::convert::From<GetRawStateCall> for SpotEngineCalls {
        fn from(value: GetRawStateCall) -> Self {
            Self::GetRawState(value)
        }
    }
    impl ::core::convert::From<GetRiskCall> for SpotEngineCalls {
        fn from(value: GetRiskCall) -> Self {
            Self::GetRisk(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for SpotEngineCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
        }
    }
    impl ::core::convert::From<GetStateAndBalanceCall> for SpotEngineCalls {
        fn from(value: GetStateAndBalanceCall) -> Self {
            Self::GetStateAndBalance(value)
        }
    }
    impl ::core::convert::From<GetStatesAndBalancesCall> for SpotEngineCalls {
        fn from(value: GetStatesAndBalancesCall) -> Self {
            Self::GetStatesAndBalances(value)
        }
    }
    impl ::core::convert::From<GetTokenCall> for SpotEngineCalls {
        fn from(value: GetTokenCall) -> Self {
            Self::GetToken(value)
        }
    }
    impl ::core::convert::From<GetTokenBalanceCall> for SpotEngineCalls {
        fn from(value: GetTokenBalanceCall) -> Self {
            Self::GetTokenBalance(value)
        }
    }
    impl ::core::convert::From<GetTotalBalancesCall> for SpotEngineCalls {
        fn from(value: GetTotalBalancesCall) -> Self {
            Self::GetTotalBalances(value)
        }
    }
    impl ::core::convert::From<GetWithdrawFeeCall> for SpotEngineCalls {
        fn from(value: GetWithdrawFeeCall) -> Self {
            Self::GetWithdrawFee(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SpotEngineCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for SpotEngineCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<MigrationFlagCall> for SpotEngineCalls {
        fn from(value: MigrationFlagCall) -> Self {
            Self::MigrationFlag(value)
        }
    }
    impl ::core::convert::From<MintLpCall> for SpotEngineCalls {
        fn from(value: MintLpCall) -> Self {
            Self::MintLp(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SpotEngineCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for SpotEngineCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for SpotEngineCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetLpBalanceCall> for SpotEngineCalls {
        fn from(value: SetLpBalanceCall) -> Self {
            Self::SetLpBalance(value)
        }
    }
    impl ::core::convert::From<SetLpStateCall> for SpotEngineCalls {
        fn from(value: SetLpStateCall) -> Self {
            Self::SetLpState(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for SpotEngineCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SocializeSubaccountCall> for SpotEngineCalls {
        fn from(value: SocializeSubaccountCall) -> Self {
            Self::SocializeSubaccount(value)
        }
    }
    impl ::core::convert::From<SwapLpCall> for SpotEngineCalls {
        fn from(value: SwapLpCall) -> Self {
            Self::SwapLp(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SpotEngineCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnsignedUpdateProductTxCall> for SpotEngineCalls {
        fn from(value: UnsignedUpdateProductTxCall) -> Self {
            Self::UnsignedUpdateProductTx(value)
        }
    }
    impl ::core::convert::From<UpdateBalanceCall> for SpotEngineCalls {
        fn from(value: UpdateBalanceCall) -> Self {
            Self::UpdateBalance(value)
        }
    }
    impl ::core::convert::From<UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall>
        for SpotEngineCalls
    {
        fn from(value: UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall) -> Self {
            Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(value)
        }
    }
    impl ::core::convert::From<UpdateMinDepositRateCall> for SpotEngineCalls {
        fn from(value: UpdateMinDepositRateCall) -> Self {
            Self::UpdateMinDepositRate(value)
        }
    }
    impl ::core::convert::From<UpdatePriceCall> for SpotEngineCalls {
        fn from(value: UpdatePriceCall) -> Self {
            Self::UpdatePrice(value)
        }
    }
    impl ::core::convert::From<UpdateProductCall> for SpotEngineCalls {
        fn from(value: UpdateProductCall) -> Self {
            Self::UpdateProduct(value)
        }
    }
    impl ::core::convert::From<UpdateQuoteFromInsuranceCall> for SpotEngineCalls {
        fn from(value: UpdateQuoteFromInsuranceCall) -> Self {
            Self::UpdateQuoteFromInsurance(value)
        }
    }
    impl ::core::convert::From<UpdateRiskCall> for SpotEngineCalls {
        fn from(value: UpdateRiskCall) -> Self {
            Self::UpdateRisk(value)
        }
    }
    impl ::core::convert::From<UpdateStatesCall> for SpotEngineCalls {
        fn from(value: UpdateStatesCall) -> Self {
            Self::UpdateStates(value)
        }
    }
    ///Container type for all return fields from the `burnLp` function with signature `burnLp(uint32,bytes32,int128)` and selector `0xd98752ec`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_base: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_quote: i128,
    }
    ///Container type for all return fields from the `decomposeLps` function with signature `decomposeLps(bytes32,bytes32)` and selector `0xb8d80d8b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub liquidation_fees: i128,
    }
    ///Container type for all return fields from the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint32)` and selector `0xe343738c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetConfigReturn(pub Config);
    ///Container type for all return fields from the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `0x8a1d43c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCoreRiskReturn(pub CoreRisk);
    ///Container type for all return fields from the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `0x871d0912`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHealthContributionReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub health: i128,
    }
    ///Container type for all return fields from the `getMinDepositRate` function with signature `getMinDepositRate(uint32)` and selector `0x130ea373`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMinDepositRateReturn(pub i128);
    ///Container type for all return fields from the `getPoolState` function with signature `getPoolState(uint32)` and selector `0x8af6426a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRawBalanceReturn(pub Balances);
    ///Container type for all return fields from the `getRawLpState` function with signature `getRawLpState(uint32)` and selector `0xc721bd65`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRiskReturn(pub Risk);
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
        pub balances_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub states_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub lp_states_slot: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `getToken` function with signature `getToken(uint32)` and selector `0x45be7ed6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `0xa67ac322`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokenBalanceReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub balance: u128,
    }
    ///Container type for all return fields from the `getTotalBalances` function with signature `getTotalBalances()` and selector `0x2baf57d3`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalBalancesReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub borrows: ::std::vec::Vec<i128>,
    }
    ///Container type for all return fields from the `getWithdrawFee` function with signature `getWithdrawFee(uint32)` and selector `0xfdf4a0c0`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetWithdrawFeeReturn(pub i128);
    ///Container type for all return fields from the `migrationFlag` function with signature `migrationFlag()` and selector `0xc362d19e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MigrationFlagReturn(pub u64);
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
    ///Container type for all return fields from the `swapLp` function with signature `swapLp(uint32,int128,int128)` and selector `0xc7167cf5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `0x49f73568`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all return fields from the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `0xf39eeb10`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateQuoteFromInsuranceReturn(pub i128);
    ///`CoreRisk(int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CoreRisk {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight: i128,
    }
    ///`Balance(int128,int128)`
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
    pub struct Balance {
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
    ///`BalanceNormalized(int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceNormalized {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_normalized: i128,
    }
    ///`Balances((int128),(int128))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Balances {
        pub balance: BalanceNormalized,
        pub lp_balance: LpBalance,
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
    ///`LpBalance(int128)`
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
    pub struct LpBalance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
    }
    ///`LpState(int128,(int128,int128),(int128,int128))`
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
    pub struct LpState {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub supply: i128,
        pub quote: Balance,
        pub base: Balance,
    }
    ///`State(int128,int128,int128,int128)`
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
    pub struct State {
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
    ///`UpdateProductTx(uint32,int128,int128,int128,(address,int128,int128,int128,int128),(int32,int32,int32,int32,int128))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub lp_spread_x18: i128,
        pub config: Config,
        pub risk_store: RiskStore,
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
    ///`RiskStore(int32,int32,int32,int32,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
}
