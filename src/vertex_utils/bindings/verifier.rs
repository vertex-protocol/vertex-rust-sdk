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
            constructor: ::core::option::Option::None,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1CG\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c_\xCB}X\x11a\0\x97W\x80c\xBE\x13\xBA\xC4\x11a\0fW\x80c\xBE\x13\xBA\xC4\x14a\x01\xFCW\x80c\xE4\x93\xEF\x8C\x14a\x02\x0FW\x80c\xEBvJ&\x14a\x02(W\x80c\xF2\xFD\xE3\x8B\x14a\x02KW`\0\x80\xFD[\x80c_\xCB}X\x14a\x01\xBDW\x80cqP\x18\xA6\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xD8W\x80c\xBB\xEF\x84\xB4\x14a\x01\xE9W`\0\x80\xFD[\x80c<d\xC2\x15\x11a\0\xD3W\x80c<d\xC2\x15\x14a\x01>W\x80cUu}\xBF\x14a\x01QW\x80cU\xE7g;\x14a\x01dW\x80c]\x18\x16\xD9\x14a\x01\x92W`\0\x80\xFD[\x80c\r\x8En,\x14a\0\xFAW\x80c*\xC4x\xB6\x14a\x01\x0EW\x80c5Ob*\x14a\x01#W[`\0\x80\xFD[`@Q`\x1B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01!a\x01\x1C6`\x04a\x15|V[a\x02^V[\0[a\x010d\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01a\x01\x05V[a\x01!a\x01L6`\x04a\x15\xBBV[a\x03\x02V[a\x01!a\x01_6`\x04a\x16\"V[a\x03KV[a\x01wa\x01r6`\x04a\x16\xD7V[a\x05CV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\x05V[a\x01\xA5a\x01\xA06`\x04a\x16\xD7V[a\x05\x97V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x05V[a\x01!a\x01\xCB6`\x04a\x17\x8DV[a\x05\xE5V[a\x01!a\x07\x82V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xA5V[a\x01!a\x01\xF76`\x04a\x18\x15V[a\x07\x96V[a\x01!a\x02\n6`\x04a\x18.V[a\x08$V[a\x010p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02;a\x0266`\x04a\x18ZV[a\x08<V[`@Q\x90\x15\x15\x81R` \x01a\x01\x05V[a\x01!a\x02Y6`\x04a\x19\tV[a\x08pV[a\x02g\x81a\x08\xFDV[a\x02pW`\0\x80\xFD[`\0a\x02{\x82a\tyV[\x90Pa\x02\xAA`\x02\x82` \x01Qa\x02\x91\x91\x90a\x19HV[\x15a\x02\x9DW`\x1Ca\x02\xA0V[`\x1B[\x82Q\x87\x87\x87a\x0B\x16V[a\x02\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03\x0B\x82a\x0C\xC7V[a\x03\x14\x82a\x0C\xC7V[`@Q` \x01a\x03%\x92\x91\x90a\x19\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x02\xF2\x91`\x04\x01a\x19\xD2V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03n\x94\x93\x92\x91\x90a\x1A\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x03\xC5\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x04\xE6W`\0\x86\x86\x83\x81\x81\x10a\x03\xFDWa\x03\xFDa\x1A&V[\x90P` \x02\x81\x01\x90a\x04\x0F\x91\x90a\x1A<V[\x90P\x11\x15a\x04\xD4Wa\x04\"`\x01\x83a\x1A\x99V[\x91Pa\x04\x88\x83\x87\x87\x84\x81\x81\x10a\x04:Wa\x04:a\x1A&V[\x90P` \x02\x81\x01\x90a\x04L\x91\x90a\x1A<V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x08<\x91PPV[a\x04\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[\x80a\x04\xDE\x81a\x1A\xB1V[\x91PPa\x03\xE1V[Pa\x02\xB1T\x81\x14a\x059W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05mWa\x05ma\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80a\x05\xA3\x83a\x05CV[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x05\xC6\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x06\x05WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x1FWP0;\x15\x80\x15a\x06\x1FWP`\0T`\xFF\x16`\x01\x14[a\x06\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF2V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\xB4W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xBCa\x0E\x15V[`\0[`\x08\x81\x10\x15a\x077Wa\x06\xE7\x83\x82`\x08\x81\x10a\x06\xDDWa\x06\xDDa\x1A&V[` \x02\x01Qa\x0E\x88V[a\x07'Wa\x07'\x81\x84\x83`\x08\x81\x10a\x07\x01Wa\x07\x01a\x1A&V[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x07\x19Wa\x07\x19a\x1A&V[` \x02\x01Q` \x01Qa\x0E\xA2V[a\x070\x81a\x1A\xB1V[\x90Pa\x06\xBFV[P\x80\x15a\x07~W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x07\x8Aa\x0F\x8AV[a\x07\x94`\0a\x0F\xE4V[V[a\x07\x9Ea\x0F\x8AV[a\x07\xDC`\x99\x82`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x0E\x88V[a\x08!W`\x01a\x02\xB1`\0\x82\x82Ta\x07\xF4\x91\x90a\x1A\xCAV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x08\rWa\x08\ra\x1A&V[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x08,a\x0F\x8AV[a\x087\x83\x83\x83a\x0E\xA2V[PPPV[`\0\x80a\x08H\x83a\x05\x97V[\x90P`\0a\x08V\x86\x86a\x10NV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\x08xa\x0F\x8AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF2V[a\x08!\x81a\x0F\xE4V[`\0\x80\x80[`\x08\x81\x10\x15a\taW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\tPWa\t2`\x99\x83`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\tBWP`\0\x94\x93PPPPV[a\tM`\x01\x84a\x1A\x99V[\x92P[Pa\tZ\x81a\x1A\xB1V[\x90Pa\t\x02V[Pa\x02\xB1Ta\tq\x82`\x02a\x1A\xE1V[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\t\xC7WPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\t\xB0Wa\t\xB0a\x1A&V[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\t\xE3W`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05mWa\x05ma\x1A&V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\n\xA0Wa\n\x13`\x02`\xFF\x86\x16\x83\x1Ca\x1B\0V[`\xFF\x16`\x01\x03a\n\x90Wa\n3`\x99\x82`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\n=W`\0\x80\xFD[a\n\x89a\nN`\x01\x83\x1B\x86\x18a\tyV[`\x99\x83`\x08\x81\x10a\naWa\naa\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x10rV[\x91Pa\n\xA0V[a\n\x99\x81a\x1A\xB1V[\x90Pa\t\xFAV[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\n\xB9Wa\n\xB9a\x1A&V[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\n\xEEWa\n\xEEa\x1A&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0BK\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1A\xCAV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x0B\x81\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1A\xCAV[\x90P`\0\x82\x90\x03a\x0B\x91W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xE5W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0CHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x0C\xF7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\r*W\x80a\r\x14\x81a\x1B\"V[\x91Pa\r#\x90P`\n\x83a\x1BHV[\x91Pa\x0C\xFBV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rNWa\rNa\x16\xF9V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\rxW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0E\rWa\r\x96`\x01\x83a\x1BnV[\x91Pa\r\xA3`\n\x86a\x1B\x96V[a\r\xAE\x90`0a\x1B\xBCV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\r\xCCWa\r\xCCa\x1A&V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0E\x06`\n\x86a\x1BHV[\x94Pa\r|V[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[a\x07\x94a\x12IV[\x80Q`\0\x90\x15\x80\x15a\x0E\x9CWP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x0E\xAFW`\0\x80\xFD[a\x0E\xC5`\x99\x84`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\x0E\xE4W`\x01a\x02\xB1`\0\x82\x82Ta\x0E\xDE\x91\x90a\x1A\x99V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x0F\x0CWa\x0F\x0Ca\x1A&V[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x0F\x84W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x0FKWa\x0FKa\x1A&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x0F|\x91\x90a\x1A\x99V[\x17\x90Pa\x0F*V[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xF2V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x10]\x85\x85a\x12\xBDV[\x91P\x91Pa\x10j\x81a\x13\x02V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\x8F\x83a\x0E\x88V[\x15a\x10\x9BWP\x80a\x0E\x9CV[a\x10\xA4\x82a\x0E\x88V[\x15a\x10\xB0WP\x81a\x0E\x9CV[\x81Q\x83Q`\0\x91\x90\x03a\x11AW\x82` \x01Q\x84` \x01Q\x14a\x10\xE7WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0E\x9CV[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x118d\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x11,`\x02d\x01\0\0\x03\xD0\x19a\x1A\xCAV[d\x01\0\0\x03\xD0\x19a\x14LV[\x82\t\x90Pa\x11\x9EV[d\x01\0\0\x03\xD0\x19a\x11xd\x01\0\0\x03\xD0\x19\x86Qa\x11d\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x86Q\x08a\x11,`\x02d\x01\0\0\x03\xD0\x19a\x1A\xCAV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x11\x94\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x11\xC5\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x11\xE2\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x11\xFF\x83d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12,\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x12\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[a\x07\x943a\x0F\xE4V[`\0\x80\x82Q`A\x03a\x12\xF3W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x12\xE7\x87\x82\x85\x85a\x14\xA2V[\x94P\x94PPPPa\x12\xFBV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x13\x16Wa\x13\x16a\x1B\xE7V[\x03a\x13\x1EWPV[`\x01\x81`\x04\x81\x11\x15a\x132Wa\x132a\x1B\xE7V[\x03a\x13\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[`\x02\x81`\x04\x81\x11\x15a\x13\x93Wa\x13\x93a\x1B\xE7V[\x03a\x13\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\xF2V[`\x03\x81`\x04\x81\x11\x15a\x13\xF4Wa\x13\xF4a\x1B\xE7V[\x03a\x08!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[`\0`\x01[\x83\x15a\x0E\rWa\x14b`\x02\x85a\x19HV[`\x01\x03a\x14}W\x82\x80a\x14wWa\x14wa\x192V[\x85\x82\t\x90P[\x82\x80a\x14\x8BWa\x14\x8Ba\x192V[\x85\x86\t\x94Pa\x14\x9B`\x02\x85a\x1B\xFDV[\x93Pa\x14QV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x14\xD9WP`\0\x90P`\x03a\x15]V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15-W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15VW`\0`\x01\x92P\x92PPa\x15]V[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x15wW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x92W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x15\xB0``\x86\x01a\x15fV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xCEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x15\xEFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x07W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x12\xFBW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x16:W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16RW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16fW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16uW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x16\x87W`\0\x80\xFD[` \x92\x83\x01\x97P\x95P\x90\x87\x015\x90\x80\x82\x16\x82\x14a\x16\xA3W`\0\x80\xFD[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x16\xB9W`\0\x80\xFD[Pa\x16\xC6\x88\x82\x89\x01a\x15\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16\xE9W`\0\x80\xFD[a\x16\xF2\x82a\x15fV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x173Wa\x173a\x16\xF9V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x173Wa\x173a\x16\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x85Wa\x17\x85a\x16\xF9V[`@R\x91\x90PV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x17\xA1W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x17\xB0W`\0\x80\xFD[a\x17\xB8a\x17\x0FV[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x17\xCAW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x18\x0BW`@\x81\x88\x03\x12\x15a\x17\xE5W`\0\x80\x81\xFD[a\x17\xEDa\x179V[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x17\xCCV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18'W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18CW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18oW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x8FW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x18\xA3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18\xB5Wa\x18\xB5a\x16\xF9V[a\x18\xC7`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x17\\V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x18\xDDW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x19\0`@\x85\x01a\x15fV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x19\x1BW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xF2W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x19WWa\x19Wa\x192V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x19wW\x81\x81\x01Q\x83\x82\x01R` \x01a\x19_V[\x83\x81\x11\x15a\x0F\x84WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x19\xA5\x81`\x02\x85\x01` \x88\x01a\x19\\V[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x19\xC6\x81`\x03\x84\x01` \x88\x01a\x19\\V[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x19\xF1\x81`@\x85\x01` \x87\x01a\x19\\V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1ASW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1AnW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x12\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xACWa\x1A\xACa\x1A\x83V[P\x01\x90V[`\0`\x01\x82\x01a\x1A\xC3Wa\x1A\xC3a\x1A\x83V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x1A\xDCWa\x1A\xDCa\x1A\x83V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1A\xFBWa\x1A\xFBa\x1A\x83V[P\x02\x90V[`\0`\xFF\x83\x16\x80a\x1B\x13Wa\x1B\x13a\x192V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a\x1B>Wa\x1B>a\x1A\x83V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1BbWa\x1Bba\x192V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x1B\x8EWa\x1B\x8Ea\x1A\x83V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1B\xB0Wa\x1B\xB0a\x192V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1B\xDEWa\x1B\xDEa\x1A\x83V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a\x1C\x0CWa\x1C\x0Ca\x192V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAC\xB9W\xEA^\xACz\\\n/\xC9F(\"\x9C\xF7\xAC\xC5\xF0#[\x17|a\xDC\xDFb\x90\x7FL:\xCFdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static VERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c_\xCB}X\x11a\0\x97W\x80c\xBE\x13\xBA\xC4\x11a\0fW\x80c\xBE\x13\xBA\xC4\x14a\x01\xFCW\x80c\xE4\x93\xEF\x8C\x14a\x02\x0FW\x80c\xEBvJ&\x14a\x02(W\x80c\xF2\xFD\xE3\x8B\x14a\x02KW`\0\x80\xFD[\x80c_\xCB}X\x14a\x01\xBDW\x80cqP\x18\xA6\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xD8W\x80c\xBB\xEF\x84\xB4\x14a\x01\xE9W`\0\x80\xFD[\x80c<d\xC2\x15\x11a\0\xD3W\x80c<d\xC2\x15\x14a\x01>W\x80cUu}\xBF\x14a\x01QW\x80cU\xE7g;\x14a\x01dW\x80c]\x18\x16\xD9\x14a\x01\x92W`\0\x80\xFD[\x80c\r\x8En,\x14a\0\xFAW\x80c*\xC4x\xB6\x14a\x01\x0EW\x80c5Ob*\x14a\x01#W[`\0\x80\xFD[`@Q`\x1B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01!a\x01\x1C6`\x04a\x15|V[a\x02^V[\0[a\x010d\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01a\x01\x05V[a\x01!a\x01L6`\x04a\x15\xBBV[a\x03\x02V[a\x01!a\x01_6`\x04a\x16\"V[a\x03KV[a\x01wa\x01r6`\x04a\x16\xD7V[a\x05CV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01\x05V[a\x01\xA5a\x01\xA06`\x04a\x16\xD7V[a\x05\x97V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x05V[a\x01!a\x01\xCB6`\x04a\x17\x8DV[a\x05\xE5V[a\x01!a\x07\x82V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xA5V[a\x01!a\x01\xF76`\x04a\x18\x15V[a\x07\x96V[a\x01!a\x02\n6`\x04a\x18.V[a\x08$V[a\x010p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02;a\x0266`\x04a\x18ZV[a\x08<V[`@Q\x90\x15\x15\x81R` \x01a\x01\x05V[a\x01!a\x02Y6`\x04a\x19\tV[a\x08pV[a\x02g\x81a\x08\xFDV[a\x02pW`\0\x80\xFD[`\0a\x02{\x82a\tyV[\x90Pa\x02\xAA`\x02\x82` \x01Qa\x02\x91\x91\x90a\x19HV[\x15a\x02\x9DW`\x1Ca\x02\xA0V[`\x1B[\x82Q\x87\x87\x87a\x0B\x16V[a\x02\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03\x0B\x82a\x0C\xC7V[a\x03\x14\x82a\x0C\xC7V[`@Q` \x01a\x03%\x92\x91\x90a\x19\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x02\xF2\x91`\x04\x01a\x19\xD2V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03n\x94\x93\x92\x91\x90a\x1A\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x03\xC5\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x04\xE6W`\0\x86\x86\x83\x81\x81\x10a\x03\xFDWa\x03\xFDa\x1A&V[\x90P` \x02\x81\x01\x90a\x04\x0F\x91\x90a\x1A<V[\x90P\x11\x15a\x04\xD4Wa\x04\"`\x01\x83a\x1A\x99V[\x91Pa\x04\x88\x83\x87\x87\x84\x81\x81\x10a\x04:Wa\x04:a\x1A&V[\x90P` \x02\x81\x01\x90a\x04L\x91\x90a\x1A<V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x08<\x91PPV[a\x04\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[\x80a\x04\xDE\x81a\x1A\xB1V[\x91PPa\x03\xE1V[Pa\x02\xB1T\x81\x14a\x059W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05mWa\x05ma\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80a\x05\xA3\x83a\x05CV[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x05\xC6\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x06\x05WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x1FWP0;\x15\x80\x15a\x06\x1FWP`\0T`\xFF\x16`\x01\x14[a\x06\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF2V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\xB4W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xBCa\x0E\x15V[`\0[`\x08\x81\x10\x15a\x077Wa\x06\xE7\x83\x82`\x08\x81\x10a\x06\xDDWa\x06\xDDa\x1A&V[` \x02\x01Qa\x0E\x88V[a\x07'Wa\x07'\x81\x84\x83`\x08\x81\x10a\x07\x01Wa\x07\x01a\x1A&V[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x07\x19Wa\x07\x19a\x1A&V[` \x02\x01Q` \x01Qa\x0E\xA2V[a\x070\x81a\x1A\xB1V[\x90Pa\x06\xBFV[P\x80\x15a\x07~W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x07\x8Aa\x0F\x8AV[a\x07\x94`\0a\x0F\xE4V[V[a\x07\x9Ea\x0F\x8AV[a\x07\xDC`\x99\x82`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x0E\x88V[a\x08!W`\x01a\x02\xB1`\0\x82\x82Ta\x07\xF4\x91\x90a\x1A\xCAV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x08\rWa\x08\ra\x1A&V[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x08,a\x0F\x8AV[a\x087\x83\x83\x83a\x0E\xA2V[PPPV[`\0\x80a\x08H\x83a\x05\x97V[\x90P`\0a\x08V\x86\x86a\x10NV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\x08xa\x0F\x8AV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x02\xF2V[a\x08!\x81a\x0F\xE4V[`\0\x80\x80[`\x08\x81\x10\x15a\taW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\tPWa\t2`\x99\x83`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\tBWP`\0\x94\x93PPPPV[a\tM`\x01\x84a\x1A\x99V[\x92P[Pa\tZ\x81a\x1A\xB1V[\x90Pa\t\x02V[Pa\x02\xB1Ta\tq\x82`\x02a\x1A\xE1V[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\t\xC7WPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\t\xB0Wa\t\xB0a\x1A&V[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\t\xE3W`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05mWa\x05ma\x1A&V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\n\xA0Wa\n\x13`\x02`\xFF\x86\x16\x83\x1Ca\x1B\0V[`\xFF\x16`\x01\x03a\n\x90Wa\n3`\x99\x82`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\n=W`\0\x80\xFD[a\n\x89a\nN`\x01\x83\x1B\x86\x18a\tyV[`\x99\x83`\x08\x81\x10a\naWa\naa\x1A&V[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x10rV[\x91Pa\n\xA0V[a\n\x99\x81a\x1A\xB1V[\x90Pa\t\xFAV[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\n\xB9Wa\n\xB9a\x1A&V[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\n\xEEWa\n\xEEa\x1A&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0BK\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1A\xCAV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x0B\x81\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a\x1A\xCAV[\x90P`\0\x82\x90\x03a\x0B\x91W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\xE5W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0CHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x0C\xF7WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\r*W\x80a\r\x14\x81a\x1B\"V[\x91Pa\r#\x90P`\n\x83a\x1BHV[\x91Pa\x0C\xFBV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rNWa\rNa\x16\xF9V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\rxW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0E\rWa\r\x96`\x01\x83a\x1BnV[\x91Pa\r\xA3`\n\x86a\x1B\x96V[a\r\xAE\x90`0a\x1B\xBCV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\r\xCCWa\r\xCCa\x1A&V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x0E\x06`\n\x86a\x1BHV[\x94Pa\r|V[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[a\x07\x94a\x12IV[\x80Q`\0\x90\x15\x80\x15a\x0E\x9CWP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x0E\xAFW`\0\x80\xFD[a\x0E\xC5`\x99\x84`\x08\x81\x10a\x07\xB4Wa\x07\xB4a\x1A&V[\x15a\x0E\xE4W`\x01a\x02\xB1`\0\x82\x82Ta\x0E\xDE\x91\x90a\x1A\x99V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x0F\x0CWa\x0F\x0Ca\x1A&V[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x0F\x84W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x0FKWa\x0FKa\x1A&V[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x0F|\x91\x90a\x1A\x99V[\x17\x90Pa\x0F*V[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xF2V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x10]\x85\x85a\x12\xBDV[\x91P\x91Pa\x10j\x81a\x13\x02V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\x8F\x83a\x0E\x88V[\x15a\x10\x9BWP\x80a\x0E\x9CV[a\x10\xA4\x82a\x0E\x88V[\x15a\x10\xB0WP\x81a\x0E\x9CV[\x81Q\x83Q`\0\x91\x90\x03a\x11AW\x82` \x01Q\x84` \x01Q\x14a\x10\xE7WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0E\x9CV[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x118d\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x11,`\x02d\x01\0\0\x03\xD0\x19a\x1A\xCAV[d\x01\0\0\x03\xD0\x19a\x14LV[\x82\t\x90Pa\x11\x9EV[d\x01\0\0\x03\xD0\x19a\x11xd\x01\0\0\x03\xD0\x19\x86Qa\x11d\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x86Q\x08a\x11,`\x02d\x01\0\0\x03\xD0\x19a\x1A\xCAV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x11\x94\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x11\xC5\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x11\xE2\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x11\xFF\x83d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x12,\x90d\x01\0\0\x03\xD0\x19a\x1A\xCAV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x12\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[a\x07\x943a\x0F\xE4V[`\0\x80\x82Q`A\x03a\x12\xF3W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x12\xE7\x87\x82\x85\x85a\x14\xA2V[\x94P\x94PPPPa\x12\xFBV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x13\x16Wa\x13\x16a\x1B\xE7V[\x03a\x13\x1EWPV[`\x01\x81`\x04\x81\x11\x15a\x132Wa\x132a\x1B\xE7V[\x03a\x13\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\xF2V[`\x02\x81`\x04\x81\x11\x15a\x13\x93Wa\x13\x93a\x1B\xE7V[\x03a\x13\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x02\xF2V[`\x03\x81`\x04\x81\x11\x15a\x13\xF4Wa\x13\xF4a\x1B\xE7V[\x03a\x08!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x02\xF2V[`\0`\x01[\x83\x15a\x0E\rWa\x14b`\x02\x85a\x19HV[`\x01\x03a\x14}W\x82\x80a\x14wWa\x14wa\x192V[\x85\x82\t\x90P[\x82\x80a\x14\x8BWa\x14\x8Ba\x192V[\x85\x86\t\x94Pa\x14\x9B`\x02\x85a\x1B\xFDV[\x93Pa\x14QV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x14\xD9WP`\0\x90P`\x03a\x15]V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15-W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15VW`\0`\x01\x92P\x92PPa\x15]V[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x15wW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x92W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x15\xB0``\x86\x01a\x15fV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xCEW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x15\xEFW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x07W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x12\xFBW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x16:W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16RW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x16fW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x16uW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x16\x87W`\0\x80\xFD[` \x92\x83\x01\x97P\x95P\x90\x87\x015\x90\x80\x82\x16\x82\x14a\x16\xA3W`\0\x80\xFD[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x16\xB9W`\0\x80\xFD[Pa\x16\xC6\x88\x82\x89\x01a\x15\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x16\xE9W`\0\x80\xFD[a\x16\xF2\x82a\x15fV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x173Wa\x173a\x16\xF9V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x173Wa\x173a\x16\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x17\x85Wa\x17\x85a\x16\xF9V[`@R\x91\x90PV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x17\xA1W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x17\xB0W`\0\x80\xFD[a\x17\xB8a\x17\x0FV[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x17\xCAW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x18\x0BW`@\x81\x88\x03\x12\x15a\x17\xE5W`\0\x80\x81\xFD[a\x17\xEDa\x179V[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x17\xCCV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18'W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18CW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18oW`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x8FW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x18\xA3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18\xB5Wa\x18\xB5a\x16\xF9V[a\x18\xC7`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x17\\V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x18\xDDW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x19\0`@\x85\x01a\x15fV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x19\x1BW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\xF2W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x19WWa\x19Wa\x192V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x19wW\x81\x81\x01Q\x83\x82\x01R` \x01a\x19_V[\x83\x81\x11\x15a\x0F\x84WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x19\xA5\x81`\x02\x85\x01` \x88\x01a\x19\\V[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x19\xC6\x81`\x03\x84\x01` \x88\x01a\x19\\V[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x19\xF1\x81`@\x85\x01` \x87\x01a\x19\\V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1ASW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1AnW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x12\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1A\xACWa\x1A\xACa\x1A\x83V[P\x01\x90V[`\0`\x01\x82\x01a\x1A\xC3Wa\x1A\xC3a\x1A\x83V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x1A\xDCWa\x1A\xDCa\x1A\x83V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1A\xFBWa\x1A\xFBa\x1A\x83V[P\x02\x90V[`\0`\xFF\x83\x16\x80a\x1B\x13Wa\x1B\x13a\x192V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a\x1B>Wa\x1B>a\x1A\x83V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1BbWa\x1Bba\x192V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x1B\x8EWa\x1B\x8Ea\x1A\x83V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a\x1B\xB0Wa\x1B\xB0a\x192V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1B\xDEWa\x1B\xDEa\x1A\x83V[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82a\x1C\x0CWa\x1C\x0Ca\x192V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAC\xB9W\xEA^\xACz\\\n/\xC9F(\"\x9C\xF7\xAC\xC5\xF0#[\x17|a\xDC\xDFb\x90\x7FL:\xCFdsolcC\0\x08\r\x003";
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
        ///Calls the contract's `getVersion` (0x0d8e6e2c) function
        pub fn get_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([13, 142, 110, 44], ())
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
        GetVersion(GetVersionCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequireValidSignature(RequireValidSignatureCall),
        RequireValidTxSignatures(RequireValidTxSignaturesCall),
        RevertGasInfo(RevertGasInfoCall),
        TransferOwnership(TransferOwnershipCall),
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
            if let Ok(decoded) = <GetVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVersion(decoded));
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
                Self::GetVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidTxSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertGasInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<GetVersionCall> for VerifierCalls {
        fn from(value: GetVersionCall) -> Self {
            Self::GetVersion(value)
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
