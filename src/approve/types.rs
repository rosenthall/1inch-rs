use serde::{Deserialize, Serialize};
use crate::{builder_setter};
use crate::utils::builder::BasicBuilderError;


/// Builder struct to create instance of `AllowanceDetails`
pub struct AllowanceDetailsBuilder {
    token_address : Option<String>,
    wallet_address : Option<String>
}

/// Represents the details required for an approve/allowance request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowanceDetails {
    /// Address of the token contract for which to retrieve allowance.
    pub token_address: String,
    /// Address of the wallet for which to get allowance.
    pub wallet_address: String,
}

impl AllowanceDetailsBuilder {

    pub fn new() -> AllowanceDetailsBuilder {
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

/// Represents the struct we receive after making request to get current Allowance.
#[derive(Deserialize, Debug)]
pub struct AllowanceResponse {
    pub allowance : String
}


/// Builder struct to create instance of `SpenderDetails`
pub struct SpenderDetailsBuilder {
    chain : Option<u32>
}

impl SpenderDetailsBuilder {
    builder_setter!(chain, u32);

    pub fn build(&self) -> Result<SpenderDetails, BasicBuilderError> {
        Ok(SpenderDetails {
            chain : self.chain.ok_or(BasicBuilderError::MissingField("token_address"))?
        })

    }

}

/// Struct contains the value we need to perform approve/spender request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpenderDetails {
    pub chain : u32,
}


/// Struct represents router address as 1inch returns it.
#[derive(Debug, Clone, Deserialize)]
pub struct RouterAddress {
    pub address : String
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