pub use verifier::*;
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
pub mod verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Q"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("Q"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_P"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_P"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assignPubKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assignPubKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkIndividualSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkIndividualSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signerIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("deletePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deletePubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Verifier.Point"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPubkeyAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPubkeyAddress"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("initialSet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                                8usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Verifier.Point[8]"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("requireValidSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requireValidSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("message"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("e"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signerBitmask"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requireValidTxSignatures"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requireValidTxSignatures",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("revertGasInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revertGasInfo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasUsed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("validateSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSignature"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("linkedSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static VERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x19a\0\x1EV[a\0\xDEV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15a\0\xDCW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[a\x1DG\x80a\0\xED`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xC9E\xD3Z\x11a\0fW\x80c\xC9E\xD3Z\x14a\x02\0W\x80c\xE4\x93\xEF\x8C\x14a\x02\x13W\x80c\xEBvJ&\x14a\x02,W\x80c\xF2\xFD\xE3\x8B\x14a\x02OW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xC1W\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\xBB\xEF\x84\xB4\x14a\x01\xDAW\x80c\xBE\x13\xBA\xC4\x14a\x01\xEDW`\0\x80\xFD[\x80cUu}\xBF\x11a\0\xD3W\x80cUu}\xBF\x14a\x01BW\x80cU\xE7g;\x14a\x01UW\x80c]\x18\x16\xD9\x14a\x01\x83W\x80c_\xCB}X\x14a\x01\xAEW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\0\xFAW\x80c5Ob*\x14a\x01\x0FW\x80c<d\xC2\x15\x14a\x01/W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x16\x0EV[a\x02bV[\0[a\x01\x1Cd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01=6`\x04a\x16MV[a\x03\x06V[a\x01\ra\x01P6`\x04a\x16\xB4V[a\x03OV[a\x01ha\x01c6`\x04a\x17iV[a\x05GV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01&V[a\x01\x96a\x01\x916`\x04a\x17iV[a\x05\x9BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01&V[a\x01\ra\x01\xBC6`\x04a\x17\xEEV[a\x05\xE9V[a\x01\ra\x07\x86V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\x96V[a\x01\ra\x01\xE86`\x04a\x18vV[a\x07\x9AV[a\x01\ra\x01\xFB6`\x04a\x18\x8FV[a\x08(V[a\x01\ra\x02\x0E6`\x04a\x19_V[a\x08@V[a\x01\x1Cp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02?a\x02:6`\x04a\x19\xC0V[a\x08\xCEV[`@Q\x90\x15\x15\x81R` \x01a\x01&V[a\x01\ra\x02]6`\x04a\x1A\x17V[a\t\x02V[a\x02k\x81a\t\x8FV[a\x02tW`\0\x80\xFD[`\0a\x02\x7F\x82a\n\x0BV[\x90Pa\x02\xAE`\x02\x82` \x01Qa\x02\x95\x91\x90a\x1AHV[\x15a\x02\xA1W`\x1Ca\x02\xA4V[`\x1B[\x82Q\x87\x87\x87a\x0B\xA8V[a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03\x0F\x82a\rYV[a\x03\x18\x82a\rYV[`@Q` \x01a\x03)\x92\x91\x90a\x1A\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x02\xF6\x91`\x04\x01a\x1A\xD2V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03r\x94\x93\x92\x91\x90a\x1B\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x03\xC9\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x04\xEAW`\0\x86\x86\x83\x81\x81\x10a\x04\x01Wa\x04\x01a\x1B&V[\x90P` \x02\x81\x01\x90a\x04\x13\x91\x90a\x1B<V[\x90P\x11\x15a\x04\xD8Wa\x04&`\x01\x83a\x1B\x99V[\x91Pa\x04\x8C\x83\x87\x87\x84\x81\x81\x10a\x04>Wa\x04>a\x1B&V[\x90P` \x02\x81\x01\x90a\x04P\x91\x90a\x1B<V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x08\xCE\x91PPV[a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[\x80a\x04\xE2\x81a\x1B\xB1V[\x91PPa\x03\xE5V[Pa\x02\xB1T\x81\x14a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05qWa\x05qa\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80a\x05\xA7\x83a\x05GV[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x05\xCA\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x06\tWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06#WP0;\x15\x80\x15a\x06#WP`\0T`\xFF\x16`\x01\x14[a\x06\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\xB8W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xC0a\x0E\xA7V[`\0[`\x08\x81\x10\x15a\x07;Wa\x06\xEB\x83\x82`\x08\x81\x10a\x06\xE1Wa\x06\xE1a\x1B&V[` \x02\x01Qa\x0F\x1AV[a\x07+Wa\x07+\x81\x84\x83`\x08\x81\x10a\x07\x05Wa\x07\x05a\x1B&V[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x07\x1DWa\x07\x1Da\x1B&V[` \x02\x01Q` \x01Qa\x0F4V[a\x074\x81a\x1B\xB1V[\x90Pa\x06\xC3V[P\x80\x15a\x07\x82W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x07\x8Ea\x10\x1CV[a\x07\x98`\0a\x10vV[V[a\x07\xA2a\x10\x1CV[a\x07\xE0`\x99\x82`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x0F\x1AV[a\x08%W`\x01a\x02\xB1`\0\x82\x82Ta\x07\xF8\x91\x90a\x1B\xCAV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x08\x11Wa\x08\x11a\x1B&V[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x080a\x10\x1CV[a\x08;\x83\x83\x83a\x0F4V[PPPV[`\0a\x08L\x83\x83a\x10\xE0V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08\x8DWP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x08\x8DWP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x08\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF6\x91\x90a\x1A\xD2V[PPPPPPV[`\0\x80a\x08\xDA\x83a\x05\x9BV[\x90P`\0a\x08\xE8\x86\x86a\x10\xE0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\t\na\x10\x1CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF6V[a\x08%\x81a\x10vV[`\0\x80\x80[`\x08\x81\x10\x15a\t\xF3W`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\t\xE2Wa\t\xC4`\x99\x83`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\t\xD4WP`\0\x94\x93PPPPV[a\t\xDF`\x01\x84a\x1B\x99V[\x92P[Pa\t\xEC\x81a\x1B\xB1V[\x90Pa\t\x94V[Pa\x02\xB1Ta\n\x03\x82`\x02a\x1B\xE1V[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\nYWPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\nBWa\nBa\x1B&V[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\nuW`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05qWa\x05qa\x1B&V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x0B2Wa\n\xA5`\x02`\xFF\x86\x16\x83\x1Ca\x1C\0V[`\xFF\x16`\x01\x03a\x0B\"Wa\n\xC5`\x99\x82`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\n\xCFW`\0\x80\xFD[a\x0B\x1Ba\n\xE0`\x01\x83\x1B\x86\x18a\n\x0BV[`\x99\x83`\x08\x81\x10a\n\xF3Wa\n\xF3a\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x11\x04V[\x91Pa\x0B2V[a\x0B+\x81a\x1B\xB1V[\x90Pa\n\x8CV[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0BKWa\x0BKa\x1B&V[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0B\x80Wa\x0B\x80a\x1B&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0B\xDD\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1B\xCAV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x0C\x13\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1B\xCAV[\x90P`\0\x82\x90\x03a\x0C#W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0CwW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\r\x89WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\r\xBCW\x80a\r\xA6\x81a\x1C\"V[\x91Pa\r\xB5\x90P`\n\x83a\x1CHV[\x91Pa\r\x8DV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE0Wa\r\xE0a\x17\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\nW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0E\x9FWa\x0E(`\x01\x83a\x1CnV[\x91Pa\x0E5`\n\x86a\x1C\x96V[a\x0E@\x90`0a\x1C\xBCV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x0E^Wa\x0E^a\x1B&V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0E\x98`\n\x86a\x1CHV[\x94Pa\x0E\x0EV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[a\x07\x98a\x12\xDBV[\x80Q`\0\x90\x15\x80\x15a\x0F.WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x0FAW`\0\x80\xFD[a\x0FW`\x99\x84`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\x0FvW`\x01a\x02\xB1`\0\x82\x82Ta\x0Fp\x91\x90a\x1B\x99V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x0F\x9EWa\x0F\x9Ea\x1B&V[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x10\x16W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x0F\xDDWa\x0F\xDDa\x1B&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x10\x0E\x91\x90a\x1B\x99V[\x17\x90Pa\x0F\xBCV[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xF6V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x10\xEF\x85\x85a\x13OV[\x91P\x91Pa\x10\xFC\x81a\x13\x94V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11!\x83a\x0F\x1AV[\x15a\x11-WP\x80a\x0F.V[a\x116\x82a\x0F\x1AV[\x15a\x11BWP\x81a\x0F.V[\x81Q\x83Q`\0\x91\x90\x03a\x11\xD3W\x82` \x01Q\x84` \x01Q\x14a\x11yWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F.V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x11\xCAd\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x11\xBE`\x02d\x01\0\0\x03\xD0\x19a\x1B\xCAV[d\x01\0\0\x03\xD0\x19a\x14\xDEV[\x82\t\x90Pa\x120V[d\x01\0\0\x03\xD0\x19a\x12\nd\x01\0\0\x03\xD0\x19\x86Qa\x11\xF6\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x86Q\x08a\x11\xBE`\x02d\x01\0\0\x03\xD0\x19a\x1B\xCAV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12&\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x12W\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x12t\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x12\x91\x83d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12\xBE\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[a\x07\x983a\x10vV[`\0\x80\x82Q`A\x03a\x13\x85W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x13y\x87\x82\x85\x85a\x154V[\x94P\x94PPPPa\x13\x8DV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x13\xA8Wa\x13\xA8a\x1C\xE7V[\x03a\x13\xB0WPV[`\x01\x81`\x04\x81\x11\x15a\x13\xC4Wa\x13\xC4a\x1C\xE7V[\x03a\x14\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[`\x02\x81`\x04\x81\x11\x15a\x14%Wa\x14%a\x1C\xE7V[\x03a\x14rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\xF6V[`\x03\x81`\x04\x81\x11\x15a\x14\x86Wa\x14\x86a\x1C\xE7V[\x03a\x08%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[`\0`\x01[\x83\x15a\x0E\x9FWa\x14\xF4`\x02\x85a\x1AHV[`\x01\x03a\x15\x0FW\x82\x80a\x15\tWa\x15\ta\x1A2V[\x85\x82\t\x90P[\x82\x80a\x15\x1DWa\x15\x1Da\x1A2V[\x85\x86\t\x94Pa\x15-`\x02\x85a\x1C\xFDV[\x93Pa\x14\xE3V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15kWP`\0\x90P`\x03a\x15\xEFV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xBFW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\xE8W`\0`\x01\x92P\x92PPa\x15\xEFV[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x16\tW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x16$W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x16B``\x86\x01a\x15\xF8V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16`W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x16\x81W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x99W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\x8DW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x16\xCCW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xE4W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16\xF8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17\x07W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x17\x19W`\0\x80\xFD[` \x92\x83\x01\x97P\x95P\x90\x87\x015\x90\x80\x82\x16\x82\x14a\x175W`\0\x80\xFD[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x17KW`\0\x80\xFD[Pa\x17X\x88\x82\x89\x01a\x16oV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17{W`\0\x80\xFD[a\x17\x84\x82a\x15\xF8V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xC5Wa\x17\xC5a\x17\x8BV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xC5Wa\x17\xC5a\x17\x8BV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x18\x02W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x18\x11W`\0\x80\xFD[a\x18\x19a\x17\xA1V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x18+W`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x18lW`@\x81\x88\x03\x12\x15a\x18FW`\0\x80\x81\xFD[a\x18Na\x17\xCBV[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x18-V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x88W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18\xA4W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\tW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x18\xE3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xFEWa\x18\xFEa\x17\x8BV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x19&Wa\x19&a\x17\x8BV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x19?W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19uW`\0\x80\xFD[\x845\x93Pa\x19\x85` \x86\x01a\x18\xBBV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA8W`\0\x80\xFD[a\x19\xB4\x87\x82\x88\x01a\x18\xD2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19\xD5W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xF3W`\0\x80\xFD[a\x19\xFF\x86\x82\x87\x01a\x18\xD2V[\x92PPa\x1A\x0E`@\x85\x01a\x15\xF8V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1A)W`\0\x80\xFD[a\x17\x84\x82a\x18\xBBV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1AWWa\x1AWa\x1A2V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1AwW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A_V[\x83\x81\x11\x15a\x10\x16WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x1A\xA5\x81`\x02\x85\x01` \x88\x01a\x1A\\V[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x1A\xC6\x81`\x03\x84\x01` \x88\x01a\x1A\\V[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1A\xF1\x81`@\x85\x01` \x87\x01a\x1A\\V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1BSW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BnW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\x8DW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1B\xACWa\x1B\xACa\x1B\x83V[P\x01\x90V[`\0`\x01\x82\x01a\x1B\xC3Wa\x1B\xC3a\x1B\x83V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x1B\xDCWa\x1B\xDCa\x1B\x83V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\xFBWa\x1B\xFBa\x1B\x83V[P\x02\x90V[`\0`\xFF\x83\x16\x80a\x1C\x13Wa\x1C\x13a\x1A2V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a\x1C>Wa\x1C>a\x1B\x83V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1CbWa\x1Cba\x1A2V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x1C\x8EWa\x1C\x8Ea\x1B\x83V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1C\xB0Wa\x1C\xB0a\x1A2V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1C\xDEWa\x1C\xDEa\x1B\x83V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a\x1D\x0CWa\x1D\x0Ca\x1A2V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x89\xD1\x03Xv?\xF8\xD1\x01v\xA2\nx\xFFH\xEC\xAA\x16qiX\xF6\x89{d\xE2\x1F\xD90KW\x87dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static VERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xC9E\xD3Z\x11a\0fW\x80c\xC9E\xD3Z\x14a\x02\0W\x80c\xE4\x93\xEF\x8C\x14a\x02\x13W\x80c\xEBvJ&\x14a\x02,W\x80c\xF2\xFD\xE3\x8B\x14a\x02OW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xC1W\x80c\x8D\xA5\xCB[\x14a\x01\xC9W\x80c\xBB\xEF\x84\xB4\x14a\x01\xDAW\x80c\xBE\x13\xBA\xC4\x14a\x01\xEDW`\0\x80\xFD[\x80cUu}\xBF\x11a\0\xD3W\x80cUu}\xBF\x14a\x01BW\x80cU\xE7g;\x14a\x01UW\x80c]\x18\x16\xD9\x14a\x01\x83W\x80c_\xCB}X\x14a\x01\xAEW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\0\xFAW\x80c5Ob*\x14a\x01\x0FW\x80c<d\xC2\x15\x14a\x01/W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x16\x0EV[a\x02bV[\0[a\x01\x1Cd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01=6`\x04a\x16MV[a\x03\x06V[a\x01\ra\x01P6`\x04a\x16\xB4V[a\x03OV[a\x01ha\x01c6`\x04a\x17iV[a\x05GV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01&V[a\x01\x96a\x01\x916`\x04a\x17iV[a\x05\x9BV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01&V[a\x01\ra\x01\xBC6`\x04a\x17\xEEV[a\x05\xE9V[a\x01\ra\x07\x86V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\x96V[a\x01\ra\x01\xE86`\x04a\x18vV[a\x07\x9AV[a\x01\ra\x01\xFB6`\x04a\x18\x8FV[a\x08(V[a\x01\ra\x02\x0E6`\x04a\x19_V[a\x08@V[a\x01\x1Cp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02?a\x02:6`\x04a\x19\xC0V[a\x08\xCEV[`@Q\x90\x15\x15\x81R` \x01a\x01&V[a\x01\ra\x02]6`\x04a\x1A\x17V[a\t\x02V[a\x02k\x81a\t\x8FV[a\x02tW`\0\x80\xFD[`\0a\x02\x7F\x82a\n\x0BV[\x90Pa\x02\xAE`\x02\x82` \x01Qa\x02\x95\x91\x90a\x1AHV[\x15a\x02\xA1W`\x1Ca\x02\xA4V[`\x1B[\x82Q\x87\x87\x87a\x0B\xA8V[a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03\x0F\x82a\rYV[a\x03\x18\x82a\rYV[`@Q` \x01a\x03)\x92\x91\x90a\x1A\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x02\xF6\x91`\x04\x01a\x1A\xD2V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03r\x94\x93\x92\x91\x90a\x1B\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x03\xC9\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x04\xEAW`\0\x86\x86\x83\x81\x81\x10a\x04\x01Wa\x04\x01a\x1B&V[\x90P` \x02\x81\x01\x90a\x04\x13\x91\x90a\x1B<V[\x90P\x11\x15a\x04\xD8Wa\x04&`\x01\x83a\x1B\x99V[\x91Pa\x04\x8C\x83\x87\x87\x84\x81\x81\x10a\x04>Wa\x04>a\x1B&V[\x90P` \x02\x81\x01\x90a\x04P\x91\x90a\x1B<V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x08\xCE\x91PPV[a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[\x80a\x04\xE2\x81a\x1B\xB1V[\x91PPa\x03\xE5V[Pa\x02\xB1T\x81\x14a\x05=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05qWa\x05qa\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80a\x05\xA7\x83a\x05GV[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x05\xCA\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x06\tWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06#WP0;\x15\x80\x15a\x06#WP`\0T`\xFF\x16`\x01\x14[a\x06\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\xB8W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xC0a\x0E\xA7V[`\0[`\x08\x81\x10\x15a\x07;Wa\x06\xEB\x83\x82`\x08\x81\x10a\x06\xE1Wa\x06\xE1a\x1B&V[` \x02\x01Qa\x0F\x1AV[a\x07+Wa\x07+\x81\x84\x83`\x08\x81\x10a\x07\x05Wa\x07\x05a\x1B&V[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x07\x1DWa\x07\x1Da\x1B&V[` \x02\x01Q` \x01Qa\x0F4V[a\x074\x81a\x1B\xB1V[\x90Pa\x06\xC3V[P\x80\x15a\x07\x82W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x07\x8Ea\x10\x1CV[a\x07\x98`\0a\x10vV[V[a\x07\xA2a\x10\x1CV[a\x07\xE0`\x99\x82`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x0F\x1AV[a\x08%W`\x01a\x02\xB1`\0\x82\x82Ta\x07\xF8\x91\x90a\x1B\xCAV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x08\x11Wa\x08\x11a\x1B&V[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x080a\x10\x1CV[a\x08;\x83\x83\x83a\x0F4V[PPPV[`\0a\x08L\x83\x83a\x10\xE0V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08\x8DWP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x08\x8DWP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x08\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xF6\x91\x90a\x1A\xD2V[PPPPPPV[`\0\x80a\x08\xDA\x83a\x05\x9BV[\x90P`\0a\x08\xE8\x86\x86a\x10\xE0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\t\na\x10\x1CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF6V[a\x08%\x81a\x10vV[`\0\x80\x80[`\x08\x81\x10\x15a\t\xF3W`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\t\xE2Wa\t\xC4`\x99\x83`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\t\xD4WP`\0\x94\x93PPPPV[a\t\xDF`\x01\x84a\x1B\x99V[\x92P[Pa\t\xEC\x81a\x1B\xB1V[\x90Pa\t\x94V[Pa\x02\xB1Ta\n\x03\x82`\x02a\x1B\xE1V[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\nYWPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\nBWa\nBa\x1B&V[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\nuW`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05qWa\x05qa\x1B&V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x0B2Wa\n\xA5`\x02`\xFF\x86\x16\x83\x1Ca\x1C\0V[`\xFF\x16`\x01\x03a\x0B\"Wa\n\xC5`\x99\x82`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\n\xCFW`\0\x80\xFD[a\x0B\x1Ba\n\xE0`\x01\x83\x1B\x86\x18a\n\x0BV[`\x99\x83`\x08\x81\x10a\n\xF3Wa\n\xF3a\x1B&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x11\x04V[\x91Pa\x0B2V[a\x0B+\x81a\x1B\xB1V[\x90Pa\n\x8CV[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0BKWa\x0BKa\x1B&V[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0B\x80Wa\x0B\x80a\x1B&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0B\xDD\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1B\xCAV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x0C\x13\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1B\xCAV[\x90P`\0\x82\x90\x03a\x0C#W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0CwW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\r\x89WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\r\xBCW\x80a\r\xA6\x81a\x1C\"V[\x91Pa\r\xB5\x90P`\n\x83a\x1CHV[\x91Pa\r\x8DV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE0Wa\r\xE0a\x17\x8BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\nW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0E\x9FWa\x0E(`\x01\x83a\x1CnV[\x91Pa\x0E5`\n\x86a\x1C\x96V[a\x0E@\x90`0a\x1C\xBCV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x0E^Wa\x0E^a\x1B&V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0E\x98`\n\x86a\x1CHV[\x94Pa\x0E\x0EV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[a\x07\x98a\x12\xDBV[\x80Q`\0\x90\x15\x80\x15a\x0F.WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x0FAW`\0\x80\xFD[a\x0FW`\x99\x84`\x08\x81\x10a\x07\xB8Wa\x07\xB8a\x1B&V[\x15a\x0FvW`\x01a\x02\xB1`\0\x82\x82Ta\x0Fp\x91\x90a\x1B\x99V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x0F\x9EWa\x0F\x9Ea\x1B&V[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x10\x16W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x0F\xDDWa\x0F\xDDa\x1B&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x10\x0E\x91\x90a\x1B\x99V[\x17\x90Pa\x0F\xBCV[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xF6V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x10\xEF\x85\x85a\x13OV[\x91P\x91Pa\x10\xFC\x81a\x13\x94V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11!\x83a\x0F\x1AV[\x15a\x11-WP\x80a\x0F.V[a\x116\x82a\x0F\x1AV[\x15a\x11BWP\x81a\x0F.V[\x81Q\x83Q`\0\x91\x90\x03a\x11\xD3W\x82` \x01Q\x84` \x01Q\x14a\x11yWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F.V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x11\xCAd\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x11\xBE`\x02d\x01\0\0\x03\xD0\x19a\x1B\xCAV[d\x01\0\0\x03\xD0\x19a\x14\xDEV[\x82\t\x90Pa\x120V[d\x01\0\0\x03\xD0\x19a\x12\nd\x01\0\0\x03\xD0\x19\x86Qa\x11\xF6\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x86Q\x08a\x11\xBE`\x02d\x01\0\0\x03\xD0\x19a\x1B\xCAV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12&\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x12W\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x12t\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x12\x91\x83d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12\xBE\x90d\x01\0\0\x03\xD0\x19a\x1B\xCAV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[a\x07\x983a\x10vV[`\0\x80\x82Q`A\x03a\x13\x85W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x13y\x87\x82\x85\x85a\x154V[\x94P\x94PPPPa\x13\x8DV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x13\xA8Wa\x13\xA8a\x1C\xE7V[\x03a\x13\xB0WPV[`\x01\x81`\x04\x81\x11\x15a\x13\xC4Wa\x13\xC4a\x1C\xE7V[\x03a\x14\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF6V[`\x02\x81`\x04\x81\x11\x15a\x14%Wa\x14%a\x1C\xE7V[\x03a\x14rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\xF6V[`\x03\x81`\x04\x81\x11\x15a\x14\x86Wa\x14\x86a\x1C\xE7V[\x03a\x08%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\xF6V[`\0`\x01[\x83\x15a\x0E\x9FWa\x14\xF4`\x02\x85a\x1AHV[`\x01\x03a\x15\x0FW\x82\x80a\x15\tWa\x15\ta\x1A2V[\x85\x82\t\x90P[\x82\x80a\x15\x1DWa\x15\x1Da\x1A2V[\x85\x86\t\x94Pa\x15-`\x02\x85a\x1C\xFDV[\x93Pa\x14\xE3V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15kWP`\0\x90P`\x03a\x15\xEFV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xBFW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\xE8W`\0`\x01\x92P\x92PPa\x15\xEFV[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x16\tW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x16$W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x16B``\x86\x01a\x15\xF8V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x16`W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x16\x81W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x99W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13\x8DW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x16\xCCW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xE4W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16\xF8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x17\x07W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x17\x19W`\0\x80\xFD[` \x92\x83\x01\x97P\x95P\x90\x87\x015\x90\x80\x82\x16\x82\x14a\x175W`\0\x80\xFD[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x17KW`\0\x80\xFD[Pa\x17X\x88\x82\x89\x01a\x16oV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17{W`\0\x80\xFD[a\x17\x84\x82a\x15\xF8V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xC5Wa\x17\xC5a\x17\x8BV[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\xC5Wa\x17\xC5a\x17\x8BV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x18\x02W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x18\x11W`\0\x80\xFD[a\x18\x19a\x17\xA1V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x18+W`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x18lW`@\x81\x88\x03\x12\x15a\x18FW`\0\x80\x81\xFD[a\x18Na\x17\xCBV[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x18-V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\x88W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18\xA4W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\tW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x18\xE3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\xFEWa\x18\xFEa\x17\x8BV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x19&Wa\x19&a\x17\x8BV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x19?W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19uW`\0\x80\xFD[\x845\x93Pa\x19\x85` \x86\x01a\x18\xBBV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xA8W`\0\x80\xFD[a\x19\xB4\x87\x82\x88\x01a\x18\xD2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x19\xD5W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xF3W`\0\x80\xFD[a\x19\xFF\x86\x82\x87\x01a\x18\xD2V[\x92PPa\x1A\x0E`@\x85\x01a\x15\xF8V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1A)W`\0\x80\xFD[a\x17\x84\x82a\x18\xBBV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1AWWa\x1AWa\x1A2V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1AwW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A_V[\x83\x81\x11\x15a\x10\x16WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x1A\xA5\x81`\x02\x85\x01` \x88\x01a\x1A\\V[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x1A\xC6\x81`\x03\x84\x01` \x88\x01a\x1A\\V[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1A\xF1\x81`@\x85\x01` \x87\x01a\x1A\\V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1BSW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1BnW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x13\x8DW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1B\xACWa\x1B\xACa\x1B\x83V[P\x01\x90V[`\0`\x01\x82\x01a\x1B\xC3Wa\x1B\xC3a\x1B\x83V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x1B\xDCWa\x1B\xDCa\x1B\x83V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1B\xFBWa\x1B\xFBa\x1B\x83V[P\x02\x90V[`\0`\xFF\x83\x16\x80a\x1C\x13Wa\x1C\x13a\x1A2V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a\x1C>Wa\x1C>a\x1B\x83V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1CbWa\x1Cba\x1A2V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x1C\x8EWa\x1C\x8Ea\x1B\x83V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1C\xB0Wa\x1C\xB0a\x1A2V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1C\xDEWa\x1C\xDEa\x1B\x83V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a\x1D\x0CWa\x1D\x0Ca\x1A2V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x89\xD1\x03Xv?\xF8\xD1\x01v\xA2\nx\xFFH\xEC\xAA\x16qiX\xF6\x89{d\xE2\x1F\xD90KW\x87dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static VERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VERIFIER_ABI.clone(),
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
                VERIFIER_ABI.clone(),
                VERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `Q` (0xe493ef8c) function
        pub fn q(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 147, 239, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_P` (0x354f622a) function
        pub fn p(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 79, 98, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assignPubKey` (0xbe13bac4) function
        pub fn assign_pub_key(
            &self,
            i: ::ethers::core::types::U256,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 19, 186, 196], (i, x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkIndividualSignature` (0xeb764a26) function
        pub fn check_individual_signature(
            &self,
            digest: [u8; 32],
            signature: ::ethers::core::types::Bytes,
            signer_index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 118, 74, 38], (digest, signature, signer_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deletePubkey` (0xbbef84b4) function
        pub fn delete_pubkey(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 239, 132, 180], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPubkey` (0x55e7673b) function
        pub fn get_pubkey(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, Point> {
            self.0
                .method_hash([85, 231, 103, 59], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPubkeyAddress` (0x5d1816d9) function
        pub fn get_pubkey_address(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 24, 22, 217], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x5fcb7d58) function
        pub fn initialize(
            &self,
            initial_set: [Point; 8],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 203, 125, 88], initial_set)
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
        ///Calls the contract's `requireValidSignature` (0x2ac478b6) function
        pub fn require_valid_signature(
            &self,
            message: [u8; 32],
            e: [u8; 32],
            s: [u8; 32],
            signer_bitmask: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 196, 120, 182], (message, e, s, signer_bitmask))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requireValidTxSignatures` (0x55757dbf) function
        pub fn require_valid_tx_signatures(
            &self,
            txn: ::ethers::core::types::Bytes,
            idx: u64,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 117, 125, 191], (txn, idx, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertGasInfo` (0x3c64c215) function
        pub fn revert_gas_info(
            &self,
            i: ::ethers::core::types::U256,
            gas_used: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 100, 194, 21], (i, gas_used))
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
        ///Calls the contract's `validateSignature` (0xc945d35a) function
        pub fn validate_signature(
            &self,
            sender: [u8; 32],
            linked_signer: ::ethers::core::types::Address,
            digest: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 69, 211, 90],
                    (sender, linked_signer, digest, signature),
                )
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VerifierEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Verifier<M> {
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
    pub enum VerifierEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for VerifierEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VerifierEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VerifierEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VerifierEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for VerifierEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for VerifierEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `Q` function with signature `Q()` and selector `0xe493ef8c`
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
    #[ethcall(name = "Q", abi = "Q()")]
    pub struct QCall;
    ///Container type for all input parameters for the `_P` function with signature `_P()` and selector `0x354f622a`
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
    #[ethcall(name = "_P", abi = "_P()")]
    pub struct PCall;
    ///Container type for all input parameters for the `assignPubKey` function with signature `assignPubKey(uint256,uint256,uint256)` and selector `0xbe13bac4`
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
    #[ethcall(name = "assignPubKey", abi = "assignPubKey(uint256,uint256,uint256)")]
    pub struct AssignPubKeyCall {
        pub i: ::ethers::core::types::U256,
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkIndividualSignature` function with signature `checkIndividualSignature(bytes32,bytes,uint8)` and selector `0xeb764a26`
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
        name = "checkIndividualSignature",
        abi = "checkIndividualSignature(bytes32,bytes,uint8)"
    )]
    pub struct CheckIndividualSignatureCall {
        pub digest: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
        pub signer_index: u8,
    }
    ///Container type for all input parameters for the `deletePubkey` function with signature `deletePubkey(uint256)` and selector `0xbbef84b4`
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
    #[ethcall(name = "deletePubkey", abi = "deletePubkey(uint256)")]
    pub struct DeletePubkeyCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPubkey` function with signature `getPubkey(uint8)` and selector `0x55e7673b`
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
    #[ethcall(name = "getPubkey", abi = "getPubkey(uint8)")]
    pub struct GetPubkeyCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `getPubkeyAddress` function with signature `getPubkeyAddress(uint8)` and selector `0x5d1816d9`
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
    #[ethcall(name = "getPubkeyAddress", abi = "getPubkeyAddress(uint8)")]
    pub struct GetPubkeyAddressCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize((uint256,uint256)[8])` and selector `0x5fcb7d58`
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
    #[ethcall(name = "initialize", abi = "initialize((uint256,uint256)[8])")]
    pub struct InitializeCall {
        pub initial_set: [Point; 8],
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
    ///Container type for all input parameters for the `requireValidSignature` function with signature `requireValidSignature(bytes32,bytes32,bytes32,uint8)` and selector `0x2ac478b6`
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
        name = "requireValidSignature",
        abi = "requireValidSignature(bytes32,bytes32,bytes32,uint8)"
    )]
    pub struct RequireValidSignatureCall {
        pub message: [u8; 32],
        pub e: [u8; 32],
        pub s: [u8; 32],
        pub signer_bitmask: u8,
    }
    ///Container type for all input parameters for the `requireValidTxSignatures` function with signature `requireValidTxSignatures(bytes,uint64,bytes[])` and selector `0x55757dbf`
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
        name = "requireValidTxSignatures",
        abi = "requireValidTxSignatures(bytes,uint64,bytes[])"
    )]
    pub struct RequireValidTxSignaturesCall {
        pub txn: ::ethers::core::types::Bytes,
        pub idx: u64,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `revertGasInfo` function with signature `revertGasInfo(uint256,uint256)` and selector `0x3c64c215`
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
    #[ethcall(name = "revertGasInfo", abi = "revertGasInfo(uint256,uint256)")]
    pub struct RevertGasInfoCall {
        pub i: ::ethers::core::types::U256,
        pub gas_used: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `validateSignature` function with signature `validateSignature(bytes32,address,bytes32,bytes)` and selector `0xc945d35a`
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
        name = "validateSignature",
        abi = "validateSignature(bytes32,address,bytes32,bytes)"
    )]
    pub struct ValidateSignatureCall {
        pub sender: [u8; 32],
        pub linked_signer: ::ethers::core::types::Address,
        pub digest: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifierCalls {
        Q(QCall),
        P(PCall),
        AssignPubKey(AssignPubKeyCall),
        CheckIndividualSignature(CheckIndividualSignatureCall),
        DeletePubkey(DeletePubkeyCall),
        GetPubkey(GetPubkeyCall),
        GetPubkeyAddress(GetPubkeyAddressCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequireValidSignature(RequireValidSignatureCall),
        RequireValidTxSignatures(RequireValidTxSignaturesCall),
        RevertGasInfo(RevertGasInfoCall),
        TransferOwnership(TransferOwnershipCall),
        ValidateSignature(ValidateSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for VerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <QCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Q(decoded));
            }
            if let Ok(decoded) = <PCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::P(decoded));
            }
            if let Ok(decoded) = <AssignPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <CheckIndividualSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckIndividualSignature(decoded));
            }
            if let Ok(decoded) = <DeletePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeletePubkey(decoded));
            }
            if let Ok(decoded) = <GetPubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPubkey(decoded));
            }
            if let Ok(decoded) =
                <GetPubkeyAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPubkeyAddress(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequireValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequireValidSignature(decoded));
            }
            if let Ok(decoded) =
                <RequireValidTxSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequireValidTxSignatures(decoded));
            }
            if let Ok(decoded) = <RevertGasInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertGasInfo(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ValidateSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Q(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::P(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssignPubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckIndividualSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeletePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPubkeyAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequireValidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequireValidTxSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertGasInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for VerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Q(element) => ::core::fmt::Display::fmt(element, f),
                Self::P(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckIndividualSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPubkeyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidTxSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertGasInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<QCall> for VerifierCalls {
        fn from(value: QCall) -> Self {
            Self::Q(value)
        }
    }
    impl ::core::convert::From<PCall> for VerifierCalls {
        fn from(value: PCall) -> Self {
            Self::P(value)
        }
    }
    impl ::core::convert::From<AssignPubKeyCall> for VerifierCalls {
        fn from(value: AssignPubKeyCall) -> Self {
            Self::AssignPubKey(value)
        }
    }
    impl ::core::convert::From<CheckIndividualSignatureCall> for VerifierCalls {
        fn from(value: CheckIndividualSignatureCall) -> Self {
            Self::CheckIndividualSignature(value)
        }
    }
    impl ::core::convert::From<DeletePubkeyCall> for VerifierCalls {
        fn from(value: DeletePubkeyCall) -> Self {
            Self::DeletePubkey(value)
        }
    }
    impl ::core::convert::From<GetPubkeyCall> for VerifierCalls {
        fn from(value: GetPubkeyCall) -> Self {
            Self::GetPubkey(value)
        }
    }
    impl ::core::convert::From<GetPubkeyAddressCall> for VerifierCalls {
        fn from(value: GetPubkeyAddressCall) -> Self {
            Self::GetPubkeyAddress(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for VerifierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VerifierCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VerifierCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequireValidSignatureCall> for VerifierCalls {
        fn from(value: RequireValidSignatureCall) -> Self {
            Self::RequireValidSignature(value)
        }
    }
    impl ::core::convert::From<RequireValidTxSignaturesCall> for VerifierCalls {
        fn from(value: RequireValidTxSignaturesCall) -> Self {
            Self::RequireValidTxSignatures(value)
        }
    }
    impl ::core::convert::From<RevertGasInfoCall> for VerifierCalls {
        fn from(value: RevertGasInfoCall) -> Self {
            Self::RevertGasInfo(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VerifierCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ValidateSignatureCall> for VerifierCalls {
        fn from(value: ValidateSignatureCall) -> Self {
            Self::ValidateSignature(value)
        }
    }
    ///Container type for all return fields from the `Q` function with signature `Q()` and selector `0xe493ef8c`
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
    pub struct QReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_P` function with signature `_P()` and selector `0x354f622a`
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
    pub struct PReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkIndividualSignature` function with signature `checkIndividualSignature(bytes32,bytes,uint8)` and selector `0xeb764a26`
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
    pub struct CheckIndividualSignatureReturn(pub bool);
    ///Container type for all return fields from the `getPubkey` function with signature `getPubkey(uint8)` and selector `0x55e7673b`
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
    pub struct GetPubkeyReturn(pub Point);
    ///Container type for all return fields from the `getPubkeyAddress` function with signature `getPubkeyAddress(uint8)` and selector `0x5d1816d9`
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
    pub struct GetPubkeyAddressReturn(pub ::ethers::core::types::Address);
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
    ///`Point(uint256,uint256)`
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
    pub struct Point {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
}
