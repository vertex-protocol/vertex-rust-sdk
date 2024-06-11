//! # vertex_sdk
//!
//! ## Usage
//! See the [examples](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/examples) and [sanity](https://github.com/vertex-protocol/vertex-rust-sdk/tree/main/src/sanity) directories.
//!
//! ## Quickstart: `prelude`
//! A prelude is provided which imports all the important data types and traits for you. Use this
//! when you want to quickly bootstrap a new project.
//!
//! ```rust
//! use vertex_sdk::prelude::*;
//! ```
//!
//! ## Modules
//! ### `core`
//! Core traits that define API interaction. These traits must be imported when using the client. The
//! simplest way to import the traits is by using the prelude.
//!
//! ### `vertex-utils`
//! Contains request and response models.
//!
//! ### `builders`
//! Use builders for improved UX when writing complex queries or executes. You can build each
//! query or execute struct for later use or send it directly from the builder.
//!
//! ### `vertex_client`
//! REST implementation of core traits.
//!
//! ### `sanity`
//! Sanity checks for core SDK functionality.

pub mod builders;
pub mod core;
pub mod sanity;
pub mod utils;
pub mod vertex_client;
pub mod vertex_utils;

#[doc(hidden)]
pub use vertex_utils::*;

pub mod prelude {
    pub use crate::core::*;

    pub use crate::utils::client_mode::ClientMode;

    pub use crate::math::{to_i128_x18, to_i128_x6};
    pub use crate::utils::private_key::private_key;
    pub use crate::vertex_client::VertexClient;
}
