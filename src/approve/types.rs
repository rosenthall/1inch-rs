use serde::{Deserialize, Serialize};
use crate::{builder_setter};
use crate::utils::builder::BasicBuilderError;


/// Builder struct to create instance of `AllowanceDetails`
pub struct AllowanceDetailsBuilder {
    token_address : Option<String>,
    wallet_address : Option<String>
}

/// Represents the details required for performing a /version/id/approve/allowance request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowanceDetails {
    token_address : String,
    wallet_address : String
}

impl AllowanceDetailsBuilder {

    fn new() -> AllowanceDetailsBuilder {
        AllowanceDetailsBuilder {
            token_address : None,
            wallet_address : None
        }
    }

    builder_setter!(token_address, String);
    builder_setter!(wallet_address, String);


    /// Attempts to construct a `AllowanceDetails` from the builder, returning errors if required fields are missing.
    pub fn build(&self) -> Result<AllowanceDetails, BasicBuilderError> {
        Ok(AllowanceDetails {
            wallet_address : self.wallet_address.clone().ok_or(BasicBuilderError::MissingField("wallet_address"))?,
            token_address : self.token_address.clone().ok_or(BasicBuilderError::MissingField("token_address"))?
        })
    }

}

#[derive(Deserialize)]
pub struct AllowanceResponse {
    allowance : String
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowance_details_builder() {
        let allowance_details = AllowanceDetailsBuilder::new()
            .wallet_address("0x30A557351eab496FD69F537BE1F8c744A18F94Fd".into())
            .token_address("0x55d398326f99059ff775485246999027b3197955".into())
            .build().unwrap();


        assert_eq!(&allowance_details.token_address, "0x55d398326f99059ff775485246999027b3197955");
        assert_eq!(&allowance_details.wallet_address, "0x30A557351eab496FD69F537BE1F8c744A18F94Fd");

        assert_ne!(&allowance_details.token_address, "Everything another than address.");
        assert_ne!(&allowance_details.wallet_address, "Everything another than address.");


    }
}