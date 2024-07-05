use crate::{
    client::OneInchClient,
    common::token::TokenInfo,
    consts::{BASIC_URL, SWAP_API_VERSION},
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

/// Struct represents list of tokens that are available for swap. We use it to
/// performing /tokens request In fact the struct is just hashmap where string
/// key is token`s address and its value is TokenInfo object.
#[derive(Debug, Deserialize)]
pub struct TokensListResponse {
    pub tokens: HashMap<String, TokenInfo>,
}

impl OneInchClient {
    /// Get current list of tokens that are available for swaping in 1inch.
    pub async fn get_tokens_list(&self) -> Result<TokensListResponse, Box<dyn Error>> {
        let url = format!("{}/swap/{}/{}/tokens", BASIC_URL, SWAP_API_VERSION, self.network_id);

        let request_result = self.http_client.get(url).header("Authorization", &self.token).send().await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let token_list_response: TokensListResponse = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(token_list_response)
    }
}
