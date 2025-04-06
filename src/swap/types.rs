use crate::builder_setter;

use crate::common::token::TokenInfo;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Enumerates potential errors when constructing `SwapDetails`.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum SwapDetailsBuilderError {
    /// Indicates a required field is missing its value.
    #[error("Missing {0}")]
    MissingField(&'static str),

    /// Indicates the provided slippage value is outside the allowable range.
    #[error("Invalid slippage value. It should be between 0 and 50.")]
    InvalidSlippage,

    #[error("Invalid fee value. It should be between 0 and 3.")]
    InvalidFee,
}

/// Enumerates potential errors when constructing `QuoteDetails`.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum QuoteDetailsBuilderError {
    /// Indicates a required field is missing its value.
    #[error("Missing {0}")]
    MissingField(&'static str),

    #[error("Invalid fee value. It should be between 0 and 3.")]
    InvalidFee,
}

/// Represents the details required for performing a token swap.
#[derive(Debug, Clone)]
pub struct SwapDetails {
    pub src: String,     // Source token address.
    pub dst: String,     // Destination token address.
    pub amount: String,  // Amount to be swapped.
    pub from: String,    // Address of the user initiating the swap.
    pub slippage: usize, // Permitted slippage percentage.

    // Optional fields
    pub fee: Option<u8>,
    pub protocols: Option<String>,
    pub gas_price: Option<String>,
    pub complexity_level: Option<u128>,
    pub parts: Option<u128>,
    pub main_route_parts: Option<u128>,
    pub gas_limit: Option<u128>,

    pub include_tokens_info: Option<bool>,
    pub include_protocols: Option<bool>,
    pub include_gas: Option<bool>,
    pub connector_tokens: Option<String>,
    pub permit: Option<String>,
    pub receiver: Option<String>,
    pub referrer: Option<String>,

    pub disable_estimate: Option<bool>,   // If true, disables estimation.
    pub allow_partial_fill: Option<bool>, // If true, allows the swap to be partially filled.
}

/// A builder pattern implementation for creating a `SwapDetails`.
#[derive(Default)]
pub struct SwapDetailsBuilder {
    src: Option<String>,
    dst: Option<String>,
    amount: Option<String>,
    from_addr: Option<String>,
    slippage: Option<usize>,

    // Optional fields
    fee: Option<u8>,
    protocols: Option<String>,
    gas_price: Option<String>,
    complexity_level: Option<u128>,
    parts: Option<u128>,
    main_route_parts: Option<u128>,
    gas_limit: Option<u128>,

    include_tokens_info: Option<bool>,
    include_protocols: Option<bool>,
    include_gas: Option<bool>,
    connector_tokens: Option<String>,
    permit: Option<String>,
    receiver: Option<String>,
    referrer: Option<String>,

    disable_estimate: Option<bool>,   // If true, disables estimation.
    allow_partial_fill: Option<bool>, // If true, allows the swap to be partially filled.
}

/// SwapResponse is a struct to deserialize data we can get on swap request.
#[derive(Deserialize, Debug)]
pub struct SwapResponse {
    #[serde(rename = "fromToken")]
    pub from_token: Option<TokenInfo>,

    #[serde(rename = "toToken")]
    pub to_token: Option<TokenInfo>,

    #[serde(rename = "toAmount")]
    pub to_amount: String,

    pub protocols: Option<Vec<Vec<Vec<SelectedProtocol>>>>,

    #[serde(rename = "tx")]
    pub transaction: SwapTranactionData,
}

/// SwapTranactionData is a struct contains some information and a binary
/// representation of raw_tranaction to perform swap on blockchain.
#[derive(Deserialize, Debug)]
pub struct SwapTranactionData {
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,

    #[serde(rename = "gasPrice")]
    pub gas_price: String,

    pub gas: u128,
}

/// Represents errors that can occur during both swap or quote request.
/// We use the same struct to handle errors that may occur with `swap` and
/// `quote` requests because the possible errors are almost identical. This enum
/// aggregates various types of errors related to these operations,
/// including HTTP requests, JSON parsing, and swap API specific errors.
#[derive(Error, Debug)]
pub enum SwapError {
    /// Error related to network requests.
    ///
    /// Used for handling issues with network requests, such as server
    /// unavailability, network connectivity problems, etc.
    #[error("Network error: {0}")]
    Network(reqwest::Error),

    /// Error while parsing JSON.
    ///
    /// Occurs when the server's response cannot be correctly deserialized from
    /// JSON. This could happen if the response format is different than
    /// expected.
    #[error("JSON parsing error: {0}")]
    JsonParse(serde_json::Error),

    /// Specific error related to swap/quote API.
    ///
    /// Represents errors specific to the swap API, like insufficient funds or
    /// invalid request parameters.
    #[error("Swap request error: {description}")]
    SwapRequest { description: String, error: String, status_code: u16, request_id: String },

    /// A general error.
    ///
    /// Used for other types of errors that do not fit into the above
    /// categories.
    #[error("Other error: {0}")]
    Other(String),
}

/// Represents an error response from the swap/quote API.
///
/// This structure is used to deserialize the JSON error response from the both
/// swap/quote API. It contains details about the error that occurred during a
/// request.
#[derive(serde::Deserialize)]
pub struct SwapRequestError {
    /// A brief description of the error.
    pub error: String,

    /// A more detailed description of the error.
    pub description: String,

    /// The HTTP status code associated with the error.
    #[serde(rename = "statusCode")]
    pub status_code: u16,

    /// A unique identifier for the request, useful for debugging.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Additional metadata related to the error, if any.
    pub meta: Vec<HttpExceptionMeta>,
}

/// Represents additional metadata in the swap API error response.
///
/// Each item in the `meta` field of `SwapRequestError` will be deserialized
/// into this structure. It provides more context about the error, such as the
/// affected parameters or values.
#[derive(serde::Deserialize)]
pub struct HttpExceptionMeta {
    /// The type of metadata.
    #[serde(rename = "type")]
    pub type_field: String,

    /// The value associated with this metadata.
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectedProtocol {
    pub name: String,
    pub part: f64,

    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,

    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
}

impl SwapDetailsBuilder {
    /// Constructs a new `SwapDetailsBuilder` with all fields uninitialized.
    pub fn new() -> Self {
        SwapDetailsBuilder::default()
    }

    builder_setter!(src, String);
    builder_setter!(dst, String);
    builder_setter!(amount, String);
    builder_setter!(from_addr, String);

    builder_setter!(protocols, String);
    builder_setter!(gas_price, String);
    builder_setter!(complexity_level, u128);
    builder_setter!(parts, u128);
    builder_setter!(main_route_parts, u128);
    builder_setter!(gas_limit, u128);

    builder_setter!(include_tokens_info, bool);
    builder_setter!(include_protocols, bool);
    builder_setter!(include_gas, bool);

    builder_setter!(connector_tokens, String);
    builder_setter!(permit, String);
    builder_setter!(receiver, String);
    builder_setter!(referrer, String);

    builder_setter!(disable_estimate, bool);
    builder_setter!(allow_partial_fill, bool);

    /// Special setter for fee that ensures value is within allowable range.
    pub fn fee(mut self, fee: u8) -> Result<Self, SwapDetailsBuilderError> {
        if fee > 3 {
            return Err(SwapDetailsBuilderError::InvalidFee);
        }
        self.fee = Some(fee);
        Ok(self)
    }

    /// Special setter for slippage that ensures value is within allowable
    /// range.
    pub fn slippage(mut self, slippage: usize) -> Result<Self, SwapDetailsBuilderError> {
        if slippage > 50 {
            return Err(SwapDetailsBuilderError::InvalidSlippage);
        }
        self.slippage = Some(slippage);
        Ok(self)
    }

    /// Attempts to construct a ['SwapDetails'](crate::swap::types::SwapDetails)
    /// from the builder, returning errors if required fields are missing or if
    /// some of values are incorrect.
    pub fn build(self) -> Result<SwapDetails, SwapDetailsBuilderError> {
        Ok(SwapDetails {
            src: self.src.ok_or(SwapDetailsBuilderError::MissingField("src"))?,
            dst: self.dst.ok_or(SwapDetailsBuilderError::MissingField("dst"))?,
            amount: self.amount.ok_or(SwapDetailsBuilderError::MissingField("amount"))?.to_string(),
            from: self.from_addr.ok_or(SwapDetailsBuilderError::MissingField("from_addr"))?,
            slippage: self.slippage.ok_or(SwapDetailsBuilderError::MissingField("slippage"))?,

            fee: self.fee,
            protocols: self.protocols,
            gas_price: self.gas_price,
            complexity_level: self.complexity_level,
            parts: self.parts,
            main_route_parts: self.main_route_parts,
            gas_limit: self.gas_limit,
            include_tokens_info: self.include_tokens_info,
            include_protocols: self.include_protocols,
            include_gas: self.include_gas,
            connector_tokens: self.connector_tokens,
            permit: self.permit,
            receiver: self.receiver,
            referrer: self.referrer,
            disable_estimate: self.disable_estimate,
            allow_partial_fill: self.allow_partial_fill,
        })
    }
}

/// QuoteDetails is struct that contains data we need to perform /quote request.
#[derive(Debug, Clone)]
pub struct QuoteDetails {
    pub src: String,    // Source token address.
    pub dst: String,    // Destination token address.
    pub amount: String, // Amount to be swapped.

    // Optional fields
    pub fee: Option<u8>,
    pub protocols: Option<String>,
    pub gas_price: Option<String>,
    pub complexity_level: Option<u128>,
    pub parts: Option<u128>,
    pub main_route_parts: Option<u128>,
    pub gas_limit: Option<u128>,

    pub include_tokens_info: Option<bool>,
    pub include_protocols: Option<bool>,
    pub include_gas: Option<bool>,
    pub connector_tokens: Option<String>,
}

/// QuoteDetailsBuilder is struct to create instance of `QuoteDetails`
#[derive(Default)]
pub struct QuoteDetailsBuilder {
    pub src: Option<String>,
    pub dst: Option<String>,
    pub amount: Option<String>,

    // Optional fields
    pub fee: Option<u8>,
    pub protocols: Option<String>,
    pub gas_price: Option<String>,
    pub complexity_level: Option<u128>,
    pub parts: Option<u128>,
    pub main_route_parts: Option<u128>,
    pub gas_limit: Option<u128>,

    pub include_tokens_info: Option<bool>,
    pub include_protocols: Option<bool>,
    pub include_gas: Option<bool>,
    pub connector_tokens: Option<String>,
}

impl QuoteDetailsBuilder {
    pub fn new() -> Self {
        QuoteDetailsBuilder::default()
    }

    builder_setter!(src, String);
    builder_setter!(dst, String);
    builder_setter!(amount, String);

    builder_setter!(protocols, String);
    builder_setter!(gas_price, String);
    builder_setter!(complexity_level, u128);
    builder_setter!(parts, u128);
    builder_setter!(main_route_parts, u128);
    builder_setter!(gas_limit, u128);

    builder_setter!(include_tokens_info, bool);
    builder_setter!(include_protocols, bool);
    builder_setter!(include_gas, bool);
    builder_setter!(connector_tokens, String);

    /// Special setter for fee that ensures value is within allowable range.
    pub fn fee(mut self, fee: u8) -> Result<Self, QuoteDetailsBuilderError> {
        if fee > 3 {
            return Err(QuoteDetailsBuilderError::InvalidFee);
        }
        self.fee = Some(fee);
        Ok(self)
    }

    /// Attempts to construct a [`QuoteDetails`](crate::swap::QuoteDetails) from
    /// the builder, returning errors if required fields are missing or if some
    /// of values are incorrect.
    pub fn build(self) -> Result<QuoteDetails, QuoteDetailsBuilderError> {
        Ok(QuoteDetails {
            src: self.src.ok_or(QuoteDetailsBuilderError::MissingField("src"))?,
            dst: self.dst.ok_or(QuoteDetailsBuilderError::MissingField("dst"))?,
            amount: self.amount.ok_or(QuoteDetailsBuilderError::MissingField("amount"))?.to_string(),

            fee: self.fee,
            protocols: self.protocols,
            gas_price: self.gas_price,
            complexity_level: self.complexity_level,
            parts: self.parts,
            main_route_parts: self.main_route_parts,
            gas_limit: self.gas_limit,
            include_tokens_info: self.include_tokens_info,
            include_protocols: self.include_protocols,
            include_gas: self.include_gas,
            connector_tokens: self.connector_tokens,
        })
    }
}

/// SwapResponse is a struct to deserialize data we can get on quote request.
#[derive(Deserialize, Debug)]
pub struct QuoteResponse {
    #[serde(rename = "fromToken")]
    pub from_token: Option<TokenInfo>,

    #[serde(rename = "toToken")]
    pub to_token: Option<TokenInfo>,

    #[serde(rename = "toAmount")]
    pub to_amount: String,
    pub protocols: Option<Vec<Vec<Vec<SelectedProtocol>>>>,
}

/// Represents the details required for performing a token swap.
#[derive(Debug, Clone)]
pub struct SwapDetailsV6 {
    pub src: String,     // Source token address.
    pub dst: String,     // Destination token address.
    pub amount: String,  // Amount to be swapped.
    pub from: String,    // Address of the user initiating the swap.
    pub origin: String,  // An EOA address that initiate the transaction
    pub slippage: usize, // Permitted slippage percentage.

    // Optional fields
    pub fee: Option<u8>,
    pub protocols: Option<String>,
    pub gas_price: Option<String>,
    pub complexity_level: Option<u128>,
    pub parts: Option<u128>,
    pub main_route_parts: Option<u128>,
    pub gas_limit: Option<u128>,

    pub include_tokens_info: Option<bool>,
    pub include_protocols: Option<bool>,
    pub include_gas: Option<bool>,
    pub connector_tokens: Option<String>,
    pub permit: Option<String>,
    pub receiver: Option<String>,
    pub referrer: Option<String>,

    pub disable_estimate: Option<bool>,   // If true, disables estimation.
    pub allow_partial_fill: Option<bool>, // If true, allows the swap to be partially filled.

    pub use_permit2: Option<bool>,
}

/// Represents the details required for performing a token swap.
#[derive(Default)]
pub struct SwapDetailsV6Builder {
    pub src: Option<String>,     // Source token address.
    pub dst: Option<String>,     // Destination token address.
    pub amount: Option<String>,  // Amount to be swapped.
    pub from: Option<String>,    // Address of the user initiating the swap.
    pub origin: Option<String>,  // An EOA address that initiate the transaction
    pub slippage: Option<usize>, // Permitted slippage percentage.

    // Optional fields
    pub fee: Option<u8>,
    pub protocols: Option<String>,
    pub gas_price: Option<String>,
    pub complexity_level: Option<u128>,
    pub parts: Option<u128>,
    pub main_route_parts: Option<u128>,
    pub gas_limit: Option<u128>,

    pub include_tokens_info: Option<bool>,
    pub include_protocols: Option<bool>,
    pub include_gas: Option<bool>,
    pub connector_tokens: Option<String>,
    pub permit: Option<String>,
    pub receiver: Option<String>,
    pub referrer: Option<String>,

    pub disable_estimate: Option<bool>,   // If true, disables estimation.
    pub allow_partial_fill: Option<bool>, // If true, allows the swap to be partially filled.

    pub use_permit2: Option<bool>,
}

impl SwapDetailsV6Builder {
    pub fn new() -> Self {
        SwapDetailsV6Builder::default()
    }

    builder_setter!(src, String);
    builder_setter!(dst, String);
    builder_setter!(amount, String);
    builder_setter!(origin, String);
    builder_setter!(from, String);

    builder_setter!(protocols, String);
    builder_setter!(gas_price, String);
    builder_setter!(complexity_level, u128);
    builder_setter!(parts, u128);
    builder_setter!(main_route_parts, u128);
    builder_setter!(gas_limit, u128);

    builder_setter!(include_tokens_info, bool);
    builder_setter!(include_protocols, bool);
    builder_setter!(include_gas, bool);

    builder_setter!(connector_tokens, String);
    builder_setter!(permit, String);
    builder_setter!(receiver, String);
    builder_setter!(referrer, String);

    builder_setter!(disable_estimate, bool);
    builder_setter!(allow_partial_fill, bool);
    builder_setter!(use_permit2, bool);


    /// Special setter for fee that ensures value is within allowable range.
    pub fn fee(mut self, fee: u8) -> Result<Self, QuoteDetailsBuilderError> {
        if fee > 3 {
            return Err(QuoteDetailsBuilderError::InvalidFee);
        }
        self.fee = Some(fee);
        Ok(self)
    }

        /// Special setter for slippage that ensures value is within allowable
    /// range.
    pub fn slippage(mut self, slippage: usize) -> Result<Self, SwapDetailsBuilderError> {
        if slippage > 50 {
            return Err(SwapDetailsBuilderError::InvalidSlippage);
        }
        self.slippage = Some(slippage);
        Ok(self)
    }

        /// Attempts to construct a ['SwapDetails'](crate::swap::types::SwapDetailsV6Builder)
    /// from the builder, returning errors if required fields are missing or if
    /// some of values are incorrect.
    pub fn build(self) -> Result<SwapDetailsV6, SwapDetailsBuilderError> {
        Ok(SwapDetailsV6 {
            src: self.src.ok_or(SwapDetailsBuilderError::MissingField("src"))?,
            dst: self.dst.ok_or(SwapDetailsBuilderError::MissingField("dst"))?,
            amount: self.amount.ok_or(SwapDetailsBuilderError::MissingField("amount"))?.to_string(),
            from: self.from.ok_or(SwapDetailsBuilderError::MissingField("from"))?,
            origin: self.origin.ok_or(SwapDetailsBuilderError::MissingField("origin"))?,
            slippage: self.slippage.ok_or(SwapDetailsBuilderError::MissingField("slippage"))?,

            fee: self.fee,
            protocols: self.protocols,
            gas_price: self.gas_price,
            complexity_level: self.complexity_level,
            parts: self.parts,
            main_route_parts: self.main_route_parts,
            gas_limit: self.gas_limit,
            include_tokens_info: self.include_tokens_info,
            include_protocols: self.include_protocols,
            include_gas: self.include_gas,
            connector_tokens: self.connector_tokens,
            permit: self.permit,
            receiver: self.receiver,
            referrer: self.referrer,
            disable_estimate: self.disable_estimate,
            allow_partial_fill: self.allow_partial_fill,
            use_permit2: self.use_permit2,
        })
    }
}

/// SwapResponse is a struct to deserialize data we can get on swap request.
#[derive(Deserialize, Debug)]
pub struct SwapV6Response {
    #[serde(rename = "fromToken")]
    pub from_token: Option<TokenInfo>,

    #[serde(rename = "toToken")]
    pub to_token: Option<TokenInfo>,

    #[serde(rename = "dstAmount")]
    pub dst_amount: String,

    pub protocols: Option<Vec<Vec<Vec<SelectedProtocol>>>>,

    #[serde(rename = "tx")]
    pub transaction: SwapTranactionData,
}

/// Tests for the `SwapDetailsBuilder` and related components.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a successful construction of `SwapDetails` using the builder.
    #[test]
    fn test_valid_swap_details_builder() {
        let swap_details = SwapDetailsBuilder::new()
            .src("from_token".to_string())
            .dst("to_token".to_string())
            .amount("1000".to_string())
            .from_addr("from_addr".to_string())
            .slippage(5)
            .expect("Invalid slippage")
            .disable_estimate(false)
            .allow_partial_fill(false)
            .build()
            .expect("Failed to build SwapDetails");

        assert_eq!(swap_details.src, "from_token");
        assert_eq!(swap_details.dst, "to_token");
        assert_eq!(swap_details.amount, "1000");
        assert_eq!(swap_details.from, "from_addr");
        assert_eq!(swap_details.slippage, 5);
        assert!(!swap_details.disable_estimate.unwrap());
        assert!(!swap_details.allow_partial_fill.unwrap());
    }

    /// Tests the builder's response to an invalid slippage value.
    #[test]
    fn test_invalid_slippage_in_builder() {
        let result = SwapDetailsBuilder::new()
            .src("from_token".to_string())
            .dst("to_token".to_string())
            .amount("1000".to_string())
            .from_addr("from_addr".to_string())
            .slippage(102);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, SwapDetailsBuilderError::InvalidSlippage);
        }
    }
}
