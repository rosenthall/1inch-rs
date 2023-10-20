use serde::{Serialize, Deserialize};
use thiserror::Error;
use num_bigint::BigUint;

/// Enumerates potential errors when constructing `SwapDetails`.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum SwapDetailsBuilderError {
    /// Indicates a required field is missing its value.
    #[error("Missing {0}")]
    MissingField(&'static str),

    /// Indicates the provided slippage value is outside the allowable range.
    #[error("Invalid slippage value. It should be between 0 and 100.")]
    InvalidSlippage,
}

/// Represents the details required for performing a token swap.
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapDetails {
    from_token_addr: String,      // Source token address.
    to_token_addr: String,        // Destination token address.
    amount: String,               // Amount to be swapped.
    from_addr: String,            // Address of the user initiating the swap.
    slippage: usize,              // Permitted slippage percentage.
    disable_estimate: bool,       // If true, disables estimation.
    allow_partial_fill: bool,     // If true, allows the swap to be partially filled.
}

/// A builder pattern implementation for creating a `SwapDetails`.
pub struct SwapDetailsBuilder {
    from_token_addr: Option<String>,
    to_token_addr: Option<String>,
    amount: Option<BigUint>,
    from_addr: Option<String>,
    slippage: Option<usize>,
    disable_estimate: Option<bool>,
    allow_partial_fill: Option<bool>,
}

// Macro to generate setter methods for `SwapDetailsBuilder`.
macro_rules! builder_setter {
    ($field_name:ident, $field_type:ty) => {
        /// Sets the value for the specified field.
        pub fn $field_name(mut self, value: $field_type) -> Self {
            self.$field_name = Some(value);
            self
        }
    };
}

impl SwapDetailsBuilder {
    /// Constructs a new `SwapDetailsBuilder` with all fields uninitialized.
    pub fn new() -> Self {
        SwapDetailsBuilder {
            from_token_addr: None,
            to_token_addr: None,
            amount: None,
            from_addr: None,
            slippage: None,
            disable_estimate: None,
            allow_partial_fill: None,
        }
    }

    builder_setter!(from_token_addr, String);
    builder_setter!(to_token_addr, String);
    builder_setter!(amount, BigUint);
    builder_setter!(from_addr, String);
    builder_setter!(disable_estimate, bool);
    builder_setter!(allow_partial_fill, bool);

    /// Special setter for slippage that ensures value is within allowable range.
    pub fn slippage(mut self, slippage: usize) -> Result<Self, SwapDetailsBuilderError> {
        if slippage > 100 {
            return Err(SwapDetailsBuilderError::InvalidSlippage);
        }
        self.slippage = Some(slippage);
        Ok(self)
    }

    /// Attempts to construct a `SwapDetails` from the builder, returning errors if required fields are missing.
    pub fn build(self) -> Result<SwapDetails, SwapDetailsBuilderError> {
        Ok(SwapDetails {
            from_token_addr: self.from_token_addr.ok_or(SwapDetailsBuilderError::MissingField("from_token_addr"))?,
            to_token_addr: self.to_token_addr.ok_or(SwapDetailsBuilderError::MissingField("to_token_addr"))?,
            amount: self.amount.ok_or(SwapDetailsBuilderError::MissingField("amount"))?.to_string(),
            from_addr: self.from_addr.ok_or(SwapDetailsBuilderError::MissingField("from_addr"))?,
            slippage: self.slippage.ok_or(SwapDetailsBuilderError::MissingField("slippage"))?,
            disable_estimate: self.disable_estimate.unwrap_or(false),
            allow_partial_fill: self.allow_partial_fill.unwrap_or(false),
        })
    }
}

/// Tests for the `SwapDetailsBuilder` and related components.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a successful construction of `SwapDetails` using the builder.
    #[test]
    fn test_valid_swap_details_builder() {
        let swap_details = SwapDetailsBuilder::new()
            .from_token_addr("from_token".to_string())
            .to_token_addr("to_token".to_string())
            .amount(BigUint::from(1000u32))
            .from_addr("from_addr".to_string())
            .slippage(5).expect("Invalid slippage")
            .disable_estimate(false)
            .allow_partial_fill(false)
            .build()
            .expect("Failed to build SwapDetails");

        assert_eq!(swap_details.from_token_addr, "from_token");
        assert_eq!(swap_details.to_token_addr, "to_token");
        assert_eq!(swap_details.amount, "1000");
        assert_eq!(swap_details.from_addr, "from_addr");
        assert_eq!(swap_details.slippage, 5);
        assert_eq!(swap_details.disable_estimate, false);
        assert_eq!(swap_details.allow_partial_fill, false);
    }

    /// Tests the builder's response to an invalid slippage value.
    #[test]
    fn test_invalid_slippage_in_builder() {
        let result = SwapDetailsBuilder::new()
            .from_token_addr("from_token".to_string())
            .to_token_addr("to_token".to_string())
            .amount(BigUint::from(1000u32))
            .from_addr("from_addr".to_string())
            .slippage(102);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, SwapDetailsBuilderError::InvalidSlippage);
        }
    }
}
