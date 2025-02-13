pub use withdraw_pool::*;
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
pub mod withdraw_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkMarkedIdxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkMarkedIdxs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("idxs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkProductBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkProductBalances",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fastWithdrawalFeeAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fastWithdrawalFeeAmount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20Base"),
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
                    ::std::borrow::ToOwned::to_owned("fees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                                name: ::std::borrow::ToOwned::to_owned("_verifier"),
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
                    ::std::borrow::ToOwned::to_owned("markedIdxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("markedIdxs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("minIdx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minIdx"),
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
                    ::std::borrow::ToOwned::to_owned("removeLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeLiquidity"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
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
                    ::std::borrow::ToOwned::to_owned("submitFastWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitFastWithdrawal",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signatures"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitWithdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20Base"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WITHDRAWPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x19a\0\x1EV[a\0\xDEV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15a\0\xDCW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[a\x1A\xFE\x80a\0\xED`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x8CW\x80c\xC3\xB5\x818\x11a\0fW\x80c\xC3\xB5\x818\x14a\x01\xEBW\x80c\xEF{v\x90\x14a\x02\x11W\x80c\xF2\xFD\xE3\x8B\x14a\x024W\x80c\xFD\x8CR\xCD\x14a\x02GW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\x9DW\x80c\xB4o<\xB2\x14a\x01\xB8W\x80c\xB7,7\xF0\x14a\x01\xCBW`\0\x80\xFD[\x80cf\xB3-\xAD\x11a\0\xBDW\x80cf\xB3-\xAD\x14a\x015W\x80cqP\x18\xA6\x14a\x01hW\x80c}\xDE\x18\xE2\x14a\x01pW`\0\x80\xFD[\x80c4\xFB\x05A\x14a\0\xE4W\x80cH\\\xC9U\x14a\0\xF9W\x80cZY7\xF0\x14a\x01\x0CW[`\0\x80\xFD[a\0\xF7a\0\xF26`\x04a\x10\tV[a\x02ZV[\0[a\0\xF7a\x01\x076`\x04a\x10aV[a\x02\xDBV[a\x01\x1Fa\x01\x1A6`\x04a\x10\xE6V[a\x04?V[`@Qa\x01,\x91\x90a\x11(V[`@Q\x80\x91\x03\x90\xF3[a\x01Xa\x01C6`\x04a\x11lV[`\x9B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01,V[a\0\xF7a\x05iV[`\x9DTa\x01\x84\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01,V[`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[a\0\xF7a\x01\xC66`\x04a\x11\x87V[a\x05}V[a\x01\xDEa\x01\xD96`\x04a\x10\xE6V[a\x08dV[`@Qa\x01,\x91\x90a\x126V[a\x01\xFEa\x01\xF96`\x04a\x12\x84V[a\t:V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x01,V[a\x01\xFEa\x02\x1F6`\x04a\x12\xC9V[`\x9C` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[a\0\xF7a\x02B6`\x04a\x12\xE4V[a\n\x98V[a\0\xF7a\x02U6`\x04a\x13\x01V[a\x0B(V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02qW`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x90 T`\xFF\x16a\x02\xD5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x9B` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9D\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90Ua\x02\xD5\x84\x84\x84a\x0B?V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x02\xFBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\x15WP0;\x15\x80\x15a\x03\x15WP`\0T`\xFF\x16`\x01\x14[a\x03\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x03\xAFW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x03\xB7a\x0B\\V[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a\x04:W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\\Wa\x04\\a\x13HV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x85W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05_W`\0a\x04\xC4\x86\x86\x84\x81\x81\x10a\x04\xAAWa\x04\xAAa\x13^V[\x90P` \x02\x01` \x81\x01\x90a\x04\xBF\x91\x90a\x12\xC9V[a\x0B\xCFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05/\x91\x90a\x13tV[\x83\x83\x81Q\x81\x10a\x05AWa\x05Aa\x13^V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x05W\x81a\x13\xA3V[\x91PPa\x04\x8BV[P\x90P[\x92\x91PPV[a\x05qa\x0C`V[a\x05{`\0a\x0C\xBAV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90 T`\xFF\x16\x15a\x05\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FWithdrawal already submitted\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[`\x9DTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x86\x16\x11a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Fidx too small\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9AT\x90QcUu}\xBF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x90cUu}\xBF\x90a\x06\xA5\x90\x88\x90\x88\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x13\xE5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xD3W=`\0\x80>=`\0\xFD[P`\0\x92Pa\x06\xE9\x91P\x86\x90P`\x01\x81\x89a\x14\xA1V[\x81\x01\x90a\x06\xF6\x91\x90a\x15\xA4V[\x90P`\0a\x07\x0B\x82`\0\x01Q` \x01Qa\x0B\xCFV[\x82Q\x80Q`@\x91\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaCO`\xF0\x1B` \x84\x01R\x92\x93P``\x1C\x91\x90o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x01`\x80\x1B\x03\x83\x16\x11\x15a\x07nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[P`\0a\x07\x84\x84\x86`\0\x01Q` \x01Q\x84a\t:V[\x90P\x80`\x01`\x01`\x80\x1B\x03\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x11a\x07\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FFee larger than balance\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[a\x07\xF1\x81\x83a\x16\xCAV[\x85Q` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9C\x90\x91R`@\x81 \x80T\x92\x94P\x83\x92\x90\x91\x90a\x08%\x90\x84\x90`\x0F\x0Ba\x16\xF2V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa\x08W\x84\x84\x84a\x0B?V[PPPPPPPPPPPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x81Wa\x08\x81a\x13HV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xAAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05_W`\x9B`\0\x86\x86\x84\x81\x81\x10a\x08\xCEWa\x08\xCEa\x13^V[\x90P` \x02\x01` \x81\x01\x90a\x08\xE3\x91\x90a\x11lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Q`\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\t\x18Wa\t\x18a\x13^V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\t2\x81a\x13\xA3V[\x91PPa\x08\xB0V[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9F\x91\x90a\x17SV[\x90P`\x12`\xFF\x82\x16\x11\x15a\t\xB2W`\0\x80\xFD[`\0a\t\xBF\x82`\x12a\x17vV[a\t\xCA\x90`\na\x18}V[\x90P`\0a\t\xD8\x82\x86a\x18\x8CV[\x90P`\0a\t\xEDf\x03\x8D~\xA4\xC6\x80\0\x83a\r\x19V[`\x99T`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x8A\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ne\x91\x90a\x19EV[a\np\x90`\x05a\x18\x8CV[\x90P`\0a\n~\x83\x83a\r\x99V[\x90Pa\n\x8A\x85\x82a\x19`V[\x9A\x99PPPPPPPPPPV[a\n\xA0a\x0C`V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x83V[a\x0B%\x81a\x0C\xBAV[PV[a\x0B0a\x0C`V[a\x04:a\x0B<\x84a\x0B\xCFV[\x82\x84[a\x04:`\x01`\x01`\xA0\x1B\x03\x84\x16\x83`\x01`\x01`\x80\x1B\x03\x84\x16a\r\xB7V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0B\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x83V[a\x05{a\x0E\xD5V[`\0\x80a\x0B\xDAa\x0FIV[`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CJ\x91\x90a\x19\xB5V[Q\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05cW`\0\x80\xFD[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x83V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\r`WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x05_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a\r\xAEW\x81a\r\xB0V[\x82[\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x0E(\x91\x90a\x1AEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0EeW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0EjV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0E\x94WP\x80Q\x15\x80a\x0E\x94WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\x94\x91\x90a\x1AaV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x0E\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x83V[a\x05{3a\x0C\xBAV[`\x99T`@Qc].\x9A\xD1`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a\x0Fz\x90\x84\x90`\x04\x01a\x1A\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBB\x91\x90a\x1A\xABV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B%W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\x1FW`\0\x80\xFD[\x845a\x10*\x81a\x0F\xC0V[\x93P` \x85\x015a\x10:\x81a\x0F\xC0V[\x92Pa\x10H`@\x86\x01a\x0F\xD5V[\x91Pa\x10V``\x86\x01a\x0F\xF1V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10tW`\0\x80\xFD[\x825a\x10\x7F\x81a\x0F\xC0V[\x91P` \x83\x015a\x10\x8F\x81a\x0F\xC0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x10\xACW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\xDFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x10\xF9W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x10W`\0\x80\xFD[a\x11\x1C\x85\x82\x86\x01a\x10\x9AV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11`W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x11DV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x11~W`\0\x80\xFD[a\r\xB0\x82a\x0F\xF1V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x11\x9FW`\0\x80\xFD[a\x11\xA8\x86a\x0F\xF1V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xC5W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x11\xD9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xE8W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x11\xFAW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x88\x015\x91P\x80\x82\x11\x15a\x12\x18W`\0\x80\xFD[Pa\x12%\x88\x82\x89\x01a\x10\x9AV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11`W\x83Q\x15\x15\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x12RV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x99W`\0\x80\xFD[\x835a\x12\xA4\x81a\x0F\xC0V[\x92Pa\x12\xB2` \x85\x01a\x12pV[\x91Pa\x12\xC0`@\x85\x01a\x0F\xD5V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x12\xDBW`\0\x80\xFD[a\r\xB0\x82a\x12pV[`\0` \x82\x84\x03\x12\x15a\x12\xF6W`\0\x80\xFD[\x815a\r\xB0\x81a\x0F\xC0V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x16W`\0\x80\xFD[a\x13\x1F\x84a\x12pV[\x92Pa\x13-` \x85\x01a\x0F\xD5V[\x91P`@\x84\x015a\x13=\x81a\x0F\xC0V[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x86W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x13\xB5Wa\x13\xB5a\x13\x8DV[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R`\0a\x13\xF9``\x83\x01\x87\x89a\x13\xBCV[` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82\x86\x01R\x84\x83\x03`@\x86\x01R\x82\x86\x84R\x82\x84\x01\x90P\x82\x87`\x05\x1B\x85\x01\x01\x88`\0[\x89\x81\x10\x15a\x14\x8FW\x86\x83\x03`\x1F\x19\x01\x84R\x8156\x8C\x90\x03`\x1E\x19\x01\x81\x12a\x14NW`\0\x80\xFD[\x8B\x01\x805\x86\x81\x11\x15a\x14_W`\0\x80\xFD[\x806\x03\x8D\x13\x15a\x14nW`\0\x80\xFD[a\x14{\x85\x82\x8A\x85\x01a\x13\xBCV[\x95\x88\x01\x95\x94PPP\x90\x85\x01\x90`\x01\x01a\x14(V[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80\x85\x85\x11\x15a\x14\xB1W`\0\x80\xFD[\x83\x86\x11\x15a\x14\xBEW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xEEWa\x14\xEEa\x13HV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xEEWa\x14\xEEa\x13HV[`\0\x82`\x1F\x83\x01\x12a\x15(W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15CWa\x15Ca\x13HV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x15kWa\x15ka\x13HV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x15\x84W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xB6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xCEW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a\x15\xE3W`\0\x80\xFD[a\x15\xEBa\x14\xCBV[`\x80\x82\x12\x15a\x15\xF9W`\0\x80\xFD[a\x16\x01a\x14\xF4V[\x91P\x835\x82Ra\x16\x13` \x85\x01a\x12pV[` \x83\x01Ra\x16$`@\x85\x01a\x0F\xD5V[`@\x83\x01Ra\x165``\x85\x01a\x0F\xF1V[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a\x16PW`\0\x80\xFD[a\x16\\\x87\x83\x86\x01a\x15\x17V[` \x82\x01R\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x16\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16nV[\x83\x81\x11\x15a\x02\xD5WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x16\xB6\x81`@\x85\x01` \x87\x01a\x16kV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x16\xEAWa\x16\xEAa\x13\x8DV[\x03\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x17%Wa\x17%a\x13\x8DV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x17JWa\x17Ja\x13\x8DV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17eW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\r\xB0W`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a\x17\x90Wa\x17\x90a\x13\x8DV[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15a\x17\xD4W\x81`\0\x19\x04\x82\x11\x15a\x17\xBAWa\x17\xBAa\x13\x8DV[\x80\x85\x16\x15a\x17\xC7W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x17\x9EV[P\x92P\x92\x90PV[`\0\x82a\x17\xEBWP`\x01a\x05cV[\x81a\x17\xF8WP`\0a\x05cV[\x81`\x01\x81\x14a\x18\x0EW`\x02\x81\x14a\x18\x18Wa\x184V[`\x01\x91PPa\x05cV[`\xFF\x84\x11\x15a\x18)Wa\x18)a\x13\x8DV[PP`\x01\x82\x1Ba\x05cV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x18WWP\x81\x81\na\x05cV[a\x18a\x83\x83a\x17\x99V[\x80`\0\x19\x04\x82\x11\x15a\x18uWa\x18ua\x13\x8DV[\x02\x93\x92PPPV[`\0a\r\xB0`\xFF\x84\x16\x83a\x17\xDCV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x18\xC5Wa\x18\xC5a\x13\x8DV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a\x18\xF1Wa\x18\xF1a\x13\x8DV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\rWa\x19\ra\x13\x8DV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19#Wa\x19#a\x13\x8DV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[\x80Q`\x0F\x81\x90\x0B\x81\x14a\x0F\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19WW`\0\x80\xFD[a\r\xB0\x82a\x193V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a\x19\x85WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a\x19\xACWa\x19\xACa\x13\x8DV[\x90\x05\x93\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x19\xC7W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x19\xEAWa\x19\xEAa\x13HV[`@R\x82Qa\x19\xF8\x81a\x0F\xC0V[\x81Ra\x1A\x06` \x84\x01a\x193V[` \x82\x01Ra\x1A\x17`@\x84\x01a\x193V[`@\x82\x01Ra\x1A(``\x84\x01a\x193V[``\x82\x01Ra\x1A9`\x80\x84\x01a\x193V[`\x80\x82\x01R\x93\x92PPPV[`\0\x82Qa\x1AW\x81\x84` \x87\x01a\x16kV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1AsW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xB0W`\0\x80\xFD[` \x81\x01`\x02\x83\x10a\x1A\xA5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\xBDW`\0\x80\xFD[\x81Qa\r\xB0\x81a\x0F\xC0V\xFE\xA2dipfsX\"\x12 \xC0\xC6\xC0\xC8\xFCW\x02\x9F\x83J\xE9{%0\xB0./\xEF\x89K1d\x949\xE0\x898\xB0\xD8p\0\xCDdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static WITHDRAWPOOL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x8CW\x80c\xC3\xB5\x818\x11a\0fW\x80c\xC3\xB5\x818\x14a\x01\xEBW\x80c\xEF{v\x90\x14a\x02\x11W\x80c\xF2\xFD\xE3\x8B\x14a\x024W\x80c\xFD\x8CR\xCD\x14a\x02GW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\x9DW\x80c\xB4o<\xB2\x14a\x01\xB8W\x80c\xB7,7\xF0\x14a\x01\xCBW`\0\x80\xFD[\x80cf\xB3-\xAD\x11a\0\xBDW\x80cf\xB3-\xAD\x14a\x015W\x80cqP\x18\xA6\x14a\x01hW\x80c}\xDE\x18\xE2\x14a\x01pW`\0\x80\xFD[\x80c4\xFB\x05A\x14a\0\xE4W\x80cH\\\xC9U\x14a\0\xF9W\x80cZY7\xF0\x14a\x01\x0CW[`\0\x80\xFD[a\0\xF7a\0\xF26`\x04a\x10\tV[a\x02ZV[\0[a\0\xF7a\x01\x076`\x04a\x10aV[a\x02\xDBV[a\x01\x1Fa\x01\x1A6`\x04a\x10\xE6V[a\x04?V[`@Qa\x01,\x91\x90a\x11(V[`@Q\x80\x91\x03\x90\xF3[a\x01Xa\x01C6`\x04a\x11lV[`\x9B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01,V[a\0\xF7a\x05iV[`\x9DTa\x01\x84\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01,V[`gT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[a\0\xF7a\x01\xC66`\x04a\x11\x87V[a\x05}V[a\x01\xDEa\x01\xD96`\x04a\x10\xE6V[a\x08dV[`@Qa\x01,\x91\x90a\x126V[a\x01\xFEa\x01\xF96`\x04a\x12\x84V[a\t:V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x01,V[a\x01\xFEa\x02\x1F6`\x04a\x12\xC9V[`\x9C` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[a\0\xF7a\x02B6`\x04a\x12\xE4V[a\n\x98V[a\0\xF7a\x02U6`\x04a\x13\x01V[a\x0B(V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02qW`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x90 T`\xFF\x16a\x02\xD5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\x9B` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9D\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90Ua\x02\xD5\x84\x84\x84a\x0B?V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x02\xFBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x03\x15WP0;\x15\x80\x15a\x03\x15WP`\0T`\xFF\x16`\x01\x14[a\x03\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x03\xAFW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x03\xB7a\x0B\\V[`\x99\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90\x92U`\x9A\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a\x04:W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\\Wa\x04\\a\x13HV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x85W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05_W`\0a\x04\xC4\x86\x86\x84\x81\x81\x10a\x04\xAAWa\x04\xAAa\x13^V[\x90P` \x02\x01` \x81\x01\x90a\x04\xBF\x91\x90a\x12\xC9V[a\x0B\xCFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05/\x91\x90a\x13tV[\x83\x83\x81Q\x81\x10a\x05AWa\x05Aa\x13^V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x05W\x81a\x13\xA3V[\x91PPa\x04\x8BV[P\x90P[\x92\x91PPV[a\x05qa\x0C`V[a\x05{`\0a\x0C\xBAV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90 T`\xFF\x16\x15a\x05\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FWithdrawal already submitted\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[`\x9DTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x86\x16\x11a\x06FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Fidx too small\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9AT\x90QcUu}\xBF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x81\x90cUu}\xBF\x90a\x06\xA5\x90\x88\x90\x88\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x13\xE5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xD3W=`\0\x80>=`\0\xFD[P`\0\x92Pa\x06\xE9\x91P\x86\x90P`\x01\x81\x89a\x14\xA1V[\x81\x01\x90a\x06\xF6\x91\x90a\x15\xA4V[\x90P`\0a\x07\x0B\x82`\0\x01Q` \x01Qa\x0B\xCFV[\x82Q\x80Q`@\x91\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaCO`\xF0\x1B` \x84\x01R\x92\x93P``\x1C\x91\x90o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`\x01`\x80\x1B\x03\x83\x16\x11\x15a\x07nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[P`\0a\x07\x84\x84\x86`\0\x01Q` \x01Q\x84a\t:V[\x90P\x80`\x01`\x01`\x80\x1B\x03\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x11a\x07\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FFee larger than balance\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x83V[a\x07\xF1\x81\x83a\x16\xCAV[\x85Q` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9C\x90\x91R`@\x81 \x80T\x92\x94P\x83\x92\x90\x91\x90a\x08%\x90\x84\x90`\x0F\x0Ba\x16\xF2V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa\x08W\x84\x84\x84a\x0B?V[PPPPPPPPPPPV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x81Wa\x08\x81a\x13HV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xAAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05_W`\x9B`\0\x86\x86\x84\x81\x81\x10a\x08\xCEWa\x08\xCEa\x13^V[\x90P` \x02\x01` \x81\x01\x90a\x08\xE3\x91\x90a\x11lV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Q`\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\t\x18Wa\t\x18a\x13^V[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\t2\x81a\x13\xA3V[\x91PPa\x08\xB0V[`\0\x80\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9F\x91\x90a\x17SV[\x90P`\x12`\xFF\x82\x16\x11\x15a\t\xB2W`\0\x80\xFD[`\0a\t\xBF\x82`\x12a\x17vV[a\t\xCA\x90`\na\x18}V[\x90P`\0a\t\xD8\x82\x86a\x18\x8CV[\x90P`\0a\t\xEDf\x03\x8D~\xA4\xC6\x80\0\x83a\r\x19V[`\x99T`@Qc\x03\xF7\xD2\x83`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x8A\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xF4\xA0\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ne\x91\x90a\x19EV[a\np\x90`\x05a\x18\x8CV[\x90P`\0a\n~\x83\x83a\r\x99V[\x90Pa\n\x8A\x85\x82a\x19`V[\x9A\x99PPPPPPPPPPV[a\n\xA0a\x0C`V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x83V[a\x0B%\x81a\x0C\xBAV[PV[a\x0B0a\x0C`V[a\x04:a\x0B<\x84a\x0B\xCFV[\x82\x84[a\x04:`\x01`\x01`\xA0\x1B\x03\x84\x16\x83`\x01`\x01`\x80\x1B\x03\x84\x16a\r\xB7V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0B\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x83V[a\x05{a\x0E\xD5V[`\0\x80a\x0B\xDAa\x0FIV[`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE3Cs\x8C\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CJ\x91\x90a\x19\xB5V[Q\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05cW`\0\x80\xFD[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x83V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\r`WPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x05_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a\r\xAEW\x81a\r\xB0V[\x82[\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x0E(\x91\x90a\x1AEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0EeW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0EjV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0E\x94WP\x80Q\x15\x80a\x0E\x94WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\x94\x91\x90a\x1AaV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a*#`\xF1\x1B\x81RP\x90a\x0E\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x83\x91\x90a\x16\x97V[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x83V[a\x05{3a\x0C\xBAV[`\x99T`@Qc].\x9A\xD1`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c].\x9A\xD1\x90a\x0Fz\x90\x84\x90`\x04\x01a\x1A\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xBB\x91\x90a\x1A\xABV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B%W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\x1FW`\0\x80\xFD[\x845a\x10*\x81a\x0F\xC0V[\x93P` \x85\x015a\x10:\x81a\x0F\xC0V[\x92Pa\x10H`@\x86\x01a\x0F\xD5V[\x91Pa\x10V``\x86\x01a\x0F\xF1V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10tW`\0\x80\xFD[\x825a\x10\x7F\x81a\x0F\xC0V[\x91P` \x83\x015a\x10\x8F\x81a\x0F\xC0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x10\xACW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xC4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\xDFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x10\xF9W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x10W`\0\x80\xFD[a\x11\x1C\x85\x82\x86\x01a\x10\x9AV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11`W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x11DV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x11~W`\0\x80\xFD[a\r\xB0\x82a\x0F\xF1V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x11\x9FW`\0\x80\xFD[a\x11\xA8\x86a\x0F\xF1V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xC5W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x11\xD9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xE8W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x11\xFAW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x88\x015\x91P\x80\x82\x11\x15a\x12\x18W`\0\x80\xFD[Pa\x12%\x88\x82\x89\x01a\x10\x9AV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11`W\x83Q\x15\x15\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x12RV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xECW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x99W`\0\x80\xFD[\x835a\x12\xA4\x81a\x0F\xC0V[\x92Pa\x12\xB2` \x85\x01a\x12pV[\x91Pa\x12\xC0`@\x85\x01a\x0F\xD5V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x12\xDBW`\0\x80\xFD[a\r\xB0\x82a\x12pV[`\0` \x82\x84\x03\x12\x15a\x12\xF6W`\0\x80\xFD[\x815a\r\xB0\x81a\x0F\xC0V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\x16W`\0\x80\xFD[a\x13\x1F\x84a\x12pV[\x92Pa\x13-` \x85\x01a\x0F\xD5V[\x91P`@\x84\x015a\x13=\x81a\x0F\xC0V[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x86W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x13\xB5Wa\x13\xB5a\x13\x8DV[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R`\0a\x13\xF9``\x83\x01\x87\x89a\x13\xBCV[` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82\x86\x01R\x84\x83\x03`@\x86\x01R\x82\x86\x84R\x82\x84\x01\x90P\x82\x87`\x05\x1B\x85\x01\x01\x88`\0[\x89\x81\x10\x15a\x14\x8FW\x86\x83\x03`\x1F\x19\x01\x84R\x8156\x8C\x90\x03`\x1E\x19\x01\x81\x12a\x14NW`\0\x80\xFD[\x8B\x01\x805\x86\x81\x11\x15a\x14_W`\0\x80\xFD[\x806\x03\x8D\x13\x15a\x14nW`\0\x80\xFD[a\x14{\x85\x82\x8A\x85\x01a\x13\xBCV[\x95\x88\x01\x95\x94PPP\x90\x85\x01\x90`\x01\x01a\x14(V[P\x90\x9C\x9BPPPPPPPPPPPPV[`\0\x80\x85\x85\x11\x15a\x14\xB1W`\0\x80\xFD[\x83\x86\x11\x15a\x14\xBEW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xEEWa\x14\xEEa\x13HV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x14\xEEWa\x14\xEEa\x13HV[`\0\x82`\x1F\x83\x01\x12a\x15(W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15CWa\x15Ca\x13HV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x15kWa\x15ka\x13HV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x15\x84W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xB6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x15\xCEW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a\x15\xE3W`\0\x80\xFD[a\x15\xEBa\x14\xCBV[`\x80\x82\x12\x15a\x15\xF9W`\0\x80\xFD[a\x16\x01a\x14\xF4V[\x91P\x835\x82Ra\x16\x13` \x85\x01a\x12pV[` \x83\x01Ra\x16$`@\x85\x01a\x0F\xD5V[`@\x83\x01Ra\x165``\x85\x01a\x0F\xF1V[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a\x16PW`\0\x80\xFD[a\x16\\\x87\x83\x86\x01a\x15\x17V[` \x82\x01R\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x16\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16nV[\x83\x81\x11\x15a\x02\xD5WPP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x16\xB6\x81`@\x85\x01` \x87\x01a\x16kV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x16\xEAWa\x16\xEAa\x13\x8DV[\x03\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x17%Wa\x17%a\x13\x8DV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x17JWa\x17Ja\x13\x8DV[P\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17eW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\r\xB0W`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15a\x17\x90Wa\x17\x90a\x13\x8DV[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15a\x17\xD4W\x81`\0\x19\x04\x82\x11\x15a\x17\xBAWa\x17\xBAa\x13\x8DV[\x80\x85\x16\x15a\x17\xC7W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x17\x9EV[P\x92P\x92\x90PV[`\0\x82a\x17\xEBWP`\x01a\x05cV[\x81a\x17\xF8WP`\0a\x05cV[\x81`\x01\x81\x14a\x18\x0EW`\x02\x81\x14a\x18\x18Wa\x184V[`\x01\x91PPa\x05cV[`\xFF\x84\x11\x15a\x18)Wa\x18)a\x13\x8DV[PP`\x01\x82\x1Ba\x05cV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x18WWP\x81\x81\na\x05cV[a\x18a\x83\x83a\x17\x99V[\x80`\0\x19\x04\x82\x11\x15a\x18uWa\x18ua\x13\x8DV[\x02\x93\x92PPPV[`\0a\r\xB0`\xFF\x84\x16\x83a\x17\xDCV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a\x18\xC5Wa\x18\xC5a\x13\x8DV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a\x18\xF1Wa\x18\xF1a\x13\x8DV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a\x19\rWa\x19\ra\x13\x8DV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a\x19#Wa\x19#a\x13\x8DV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[\x80Q`\x0F\x81\x90\x0B\x81\x14a\x0F\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19WW`\0\x80\xFD[a\r\xB0\x82a\x193V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a\x19\x85WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a\x19\xACWa\x19\xACa\x13\x8DV[\x90\x05\x93\x92PPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x19\xC7W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x19\xEAWa\x19\xEAa\x13HV[`@R\x82Qa\x19\xF8\x81a\x0F\xC0V[\x81Ra\x1A\x06` \x84\x01a\x193V[` \x82\x01Ra\x1A\x17`@\x84\x01a\x193V[`@\x82\x01Ra\x1A(``\x84\x01a\x193V[``\x82\x01Ra\x1A9`\x80\x84\x01a\x193V[`\x80\x82\x01R\x93\x92PPPV[`\0\x82Qa\x1AW\x81\x84` \x87\x01a\x16kV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1AsW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xB0W`\0\x80\xFD[` \x81\x01`\x02\x83\x10a\x1A\xA5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\xBDW`\0\x80\xFD[\x81Qa\r\xB0\x81a\x0F\xC0V\xFE\xA2dipfsX\"\x12 \xC0\xC6\xC0\xC8\xFCW\x02\x9F\x83J\xE9{%0\xB0./\xEF\x89K1d\x949\xE0\x898\xB0\xD8p\0\xCDdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static WITHDRAWPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct WithdrawPool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WithdrawPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WithdrawPool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WithdrawPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WithdrawPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WithdrawPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WithdrawPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                WITHDRAWPOOL_ABI.clone(),
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
                WITHDRAWPOOL_ABI.clone(),
                WITHDRAWPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkMarkedIdxs` (0xb72c37f0) function
        pub fn check_marked_idxs(
            &self,
            idxs: ::std::vec::Vec<u64>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([183, 44, 55, 240], idxs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkProductBalances` (0x5a5937f0) function
        pub fn check_product_balances(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([90, 89, 55, 240], product_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fastWithdrawalFeeAmount` (0xc3b58138) function
        pub fn fast_withdrawal_fee_amount(
            &self,
            token: ::ethers::core::types::Address,
            product_id: u32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([195, 181, 129, 56], (token, product_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees` (0xef7b7690) function
        pub fn fees(&self, p0: u32) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([239, 123, 118, 144], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (clearinghouse, verifier))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markedIdxs` (0x66b32dad) function
        pub fn marked_idxs(&self, p0: u64) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 179, 45, 173], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minIdx` (0x7dde18e2) function
        pub fn min_idx(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([125, 222, 24, 226], ())
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
        ///Calls the contract's `removeLiquidity` (0xfd8c52cd) function
        pub fn remove_liquidity(
            &self,
            product_id: u32,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 140, 82, 205], (product_id, amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitFastWithdrawal` (0xb46f3cb2) function
        pub fn submit_fast_withdrawal(
            &self,
            idx: u64,
            transaction: ::ethers::core::types::Bytes,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 111, 60, 178], (idx, transaction, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitWithdrawal` (0x34fb0541) function
        pub fn submit_withdrawal(
            &self,
            token: ::ethers::core::types::Address,
            send_to: ::ethers::core::types::Address,
            amount: u128,
            idx: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 251, 5, 65], (token, send_to, amount, idx))
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawPoolEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for WithdrawPool<M> {
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
    pub enum WithdrawPoolEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for WithdrawPoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(WithdrawPoolEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(WithdrawPoolEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WithdrawPoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for WithdrawPoolEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for WithdrawPoolEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `checkMarkedIdxs` function with signature `checkMarkedIdxs(uint64[])` and selector `0xb72c37f0`
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
    #[ethcall(name = "checkMarkedIdxs", abi = "checkMarkedIdxs(uint64[])")]
    pub struct CheckMarkedIdxsCall {
        pub idxs: ::std::vec::Vec<u64>,
    }
    ///Container type for all input parameters for the `checkProductBalances` function with signature `checkProductBalances(uint32[])` and selector `0x5a5937f0`
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
    #[ethcall(name = "checkProductBalances", abi = "checkProductBalances(uint32[])")]
    pub struct CheckProductBalancesCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `fastWithdrawalFeeAmount` function with signature `fastWithdrawalFeeAmount(address,uint32,uint128)` and selector `0xc3b58138`
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
        name = "fastWithdrawalFeeAmount",
        abi = "fastWithdrawalFeeAmount(address,uint32,uint128)"
    )]
    pub struct FastWithdrawalFeeAmountCall {
        pub token: ::ethers::core::types::Address,
        pub product_id: u32,
        pub amount: u128,
    }
    ///Container type for all input parameters for the `fees` function with signature `fees(uint32)` and selector `0xef7b7690`
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
    #[ethcall(name = "fees", abi = "fees(uint32)")]
    pub struct FeesCall(pub u32);
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
        pub verifier: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `markedIdxs` function with signature `markedIdxs(uint64)` and selector `0x66b32dad`
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
    #[ethcall(name = "markedIdxs", abi = "markedIdxs(uint64)")]
    pub struct MarkedIdxsCall(pub u64);
    ///Container type for all input parameters for the `minIdx` function with signature `minIdx()` and selector `0x7dde18e2`
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
    #[ethcall(name = "minIdx", abi = "minIdx()")]
    pub struct MinIdxCall;
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
    ///Container type for all input parameters for the `removeLiquidity` function with signature `removeLiquidity(uint32,uint128,address)` and selector `0xfd8c52cd`
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
        name = "removeLiquidity",
        abi = "removeLiquidity(uint32,uint128,address)"
    )]
    pub struct RemoveLiquidityCall {
        pub product_id: u32,
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `submitFastWithdrawal` function with signature `submitFastWithdrawal(uint64,bytes,bytes[])` and selector `0xb46f3cb2`
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
        name = "submitFastWithdrawal",
        abi = "submitFastWithdrawal(uint64,bytes,bytes[])"
    )]
    pub struct SubmitFastWithdrawalCall {
        pub idx: u64,
        pub transaction: ::ethers::core::types::Bytes,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `submitWithdrawal` function with signature `submitWithdrawal(address,address,uint128,uint64)` and selector `0x34fb0541`
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
        name = "submitWithdrawal",
        abi = "submitWithdrawal(address,address,uint128,uint64)"
    )]
    pub struct SubmitWithdrawalCall {
        pub token: ::ethers::core::types::Address,
        pub send_to: ::ethers::core::types::Address,
        pub amount: u128,
        pub idx: u64,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WithdrawPoolCalls {
        CheckMarkedIdxs(CheckMarkedIdxsCall),
        CheckProductBalances(CheckProductBalancesCall),
        FastWithdrawalFeeAmount(FastWithdrawalFeeAmountCall),
        Fees(FeesCall),
        Initialize(InitializeCall),
        MarkedIdxs(MarkedIdxsCall),
        MinIdx(MinIdxCall),
        Owner(OwnerCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RenounceOwnership(RenounceOwnershipCall),
        SubmitFastWithdrawal(SubmitFastWithdrawalCall),
        SubmitWithdrawal(SubmitWithdrawalCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for WithdrawPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckMarkedIdxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckMarkedIdxs(decoded));
            }
            if let Ok(decoded) =
                <CheckProductBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckProductBalances(decoded));
            }
            if let Ok(decoded) =
                <FastWithdrawalFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FastWithdrawalFeeAmount(decoded));
            }
            if let Ok(decoded) = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MarkedIdxsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MarkedIdxs(decoded));
            }
            if let Ok(decoded) = <MinIdxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinIdx(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SubmitFastWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitFastWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <SubmitWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WithdrawPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckMarkedIdxs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckProductBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FastWithdrawalFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MarkedIdxs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinIdx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitFastWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitWithdrawal(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for WithdrawPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckMarkedIdxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckProductBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::FastWithdrawalFeeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarkedIdxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinIdx(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitFastWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckMarkedIdxsCall> for WithdrawPoolCalls {
        fn from(value: CheckMarkedIdxsCall) -> Self {
            Self::CheckMarkedIdxs(value)
        }
    }
    impl ::core::convert::From<CheckProductBalancesCall> for WithdrawPoolCalls {
        fn from(value: CheckProductBalancesCall) -> Self {
            Self::CheckProductBalances(value)
        }
    }
    impl ::core::convert::From<FastWithdrawalFeeAmountCall> for WithdrawPoolCalls {
        fn from(value: FastWithdrawalFeeAmountCall) -> Self {
            Self::FastWithdrawalFeeAmount(value)
        }
    }
    impl ::core::convert::From<FeesCall> for WithdrawPoolCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for WithdrawPoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MarkedIdxsCall> for WithdrawPoolCalls {
        fn from(value: MarkedIdxsCall) -> Self {
            Self::MarkedIdxs(value)
        }
    }
    impl ::core::convert::From<MinIdxCall> for WithdrawPoolCalls {
        fn from(value: MinIdxCall) -> Self {
            Self::MinIdx(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for WithdrawPoolCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidityCall> for WithdrawPoolCalls {
        fn from(value: RemoveLiquidityCall) -> Self {
            Self::RemoveLiquidity(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for WithdrawPoolCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SubmitFastWithdrawalCall> for WithdrawPoolCalls {
        fn from(value: SubmitFastWithdrawalCall) -> Self {
            Self::SubmitFastWithdrawal(value)
        }
    }
    impl ::core::convert::From<SubmitWithdrawalCall> for WithdrawPoolCalls {
        fn from(value: SubmitWithdrawalCall) -> Self {
            Self::SubmitWithdrawal(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for WithdrawPoolCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `checkMarkedIdxs` function with signature `checkMarkedIdxs(uint64[])` and selector `0xb72c37f0`
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
    pub struct CheckMarkedIdxsReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `checkProductBalances` function with signature `checkProductBalances(uint32[])` and selector `0x5a5937f0`
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
    pub struct CheckProductBalancesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `fastWithdrawalFeeAmount` function with signature `fastWithdrawalFeeAmount(address,uint32,uint128)` and selector `0xc3b58138`
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
    pub struct FastWithdrawalFeeAmountReturn(pub i128);
    ///Container type for all return fields from the `fees` function with signature `fees(uint32)` and selector `0xef7b7690`
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
    pub struct FeesReturn(pub i128);
    ///Container type for all return fields from the `markedIdxs` function with signature `markedIdxs(uint64)` and selector `0x66b32dad`
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
    pub struct MarkedIdxsReturn(pub bool);
    ///Container type for all return fields from the `minIdx` function with signature `minIdx()` and selector `0x7dde18e2`
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
    pub struct MinIdxReturn(pub u64);
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
}
