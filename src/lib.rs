//! # one-inch
//!
//! This crate provides tools for interacting with the 1inch HTTP API, including
//! swapping, pricing, allowance approval, and other functionalities related to
//! decentralized finance operations.
//!
//! A lot of usage examples can be found in the `examples` directory of the
//! repository, which demonstrate how to effectively utilize the library for
//! various operations. Check out the examples here: [1inch-rs examples](https://github.com/rosenthall/1inch-rs/tree/master/examples).
extern crate core;

/// The client module for interacting with the 1inch API.
/// Offers methods to make requests to the API and handle responses.
pub mod client;

// Constants used across the crate, including API namespace versions, and a
// basic url.
mod consts;

// Utilities that makes development easier, like macroses, etc.
pub(crate) mod utils;

/// Functions for performing swaps through the 1inch API, including finding
/// optimal swap routes and executing swap transactions.
pub mod swap;

/// Common structures definitions shared by other modules.
pub mod common;

/// Modules related to tokens, including retrieving supported currencies, token
/// metadata, getting its price.
pub mod tokens;
