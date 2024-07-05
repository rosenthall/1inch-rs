use crate::{
    builder_setter, client,
    client::OneInchClient,
    consts::{BASIC_URL, SPOT_PRICE_API_VERSION},
    utils::builder::BasicBuilderError,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

/// Builder struct to create instance of
/// [`TokensPricesRequestDetails`](crate::tokens::tokens_price::TokensPricesRequestDetails)
#[derive(Default)]
pub struct TokensPricesRequestBuilder {
    addresses: Option<Vec<String>>,
    currency: Option<client::SupportedCurrencies>,
}

impl TokensPricesRequestBuilder {
    pub fn new() -> TokensPricesRequestBuilder {
        TokensPricesRequestBuilder::default()
    }

    builder_setter!(addresses, Vec<String>);
    builder_setter!(currency, client::SupportedCurrencies);

    /// Attempts to construct a
    /// [`TokensPricesRequestDetails`](crate::tokens::tokens_price::TokensPricesRequestDetails)
    /// from the builder, returning errors if required fields are missing.
    pub fn build(&self) -> Result<TokensPricesRequestDetails, BasicBuilderError> {
        Ok(TokensPricesRequestDetails {
            addresses: self.addresses.clone().ok_or(BasicBuilderError::MissingField("addresses"))?,
            currency: self.currency.clone(),
        })
    }
}

/// Request type to get prices for specified tokens in specific currencry.
/// Currency `None` means that price will returned in native Wei of blockchain.
#[derive(Debug, Clone)]
pub struct TokensPricesRequestDetails {
    pub addresses: Vec<String>,
    pub currency: Option<client::SupportedCurrencies>,
}

/// Represents the struct we receive after making request to get current prices
/// for specified tokens. In fact response is just a hashmap where key is
/// token`s address and value its price in selected currency. Note : 1inch
/// always returns tokens addresses in lowercase.
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPricesResponse {
    #[serde(flatten)]
    pub prices: HashMap<String, String>,
}

impl OneInchClient {
    /// Performs request to get price of specified tokens in specified currency.
    pub async fn get_tokens_price(&self, details: TokensPricesRequestDetails) -> Result<TokenPricesResponse, Box<dyn Error>> {
        let base_url = format!("{}/price/{}/{}/", BASIC_URL, SPOT_PRICE_API_VERSION, self.network_id);

        let comma_separated_addresses = details
            .addresses
            .iter()
            .map(|addr| addr.to_string())
            .map(|addr| addr.to_lowercase())
            .collect::<Vec<String>>()
            .join(",");

        let mut url_with_params = format!("{}{}", base_url, comma_separated_addresses);

        // Adding `currency` param if not None
        if let Some(currency) = details.currency {
            url_with_params = format!("{}?currency={}", url_with_params, currency);
        }

        let url = Url::parse(&url_with_params).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        dbg!(&url.clone().to_string());

        let request_result = self.http_client.get(url).header("Authorization", &self.token).send().await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let tokens_prices_response: TokenPricesResponse = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(tokens_prices_response)
    }
}
