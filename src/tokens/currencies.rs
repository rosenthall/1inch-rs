use serde::Deserialize;

use std::error::Error;
use crate::client::OneInchClient;
use crate::consts::{BASIC_URL, SPOT_PRICE_API_VERSION};


/// `CurrenciesResponse` is a struct that defines a list of supported currencies (like USD, UAH, SEK, etc) as 1inch returns it.
/// In fact, it's a `Vec<String>`.
/// Note that you can just use [`SupportedCurrencies`](crate::client::SupportedCurrencies) in most of cases.
#[derive(Deserialize)]
pub struct CurrenciesResponse {
    pub codes : Vec<String>
}

impl OneInchClient {
    /// Get current list of currencies
    pub async fn get_custom_currencies(
        &self,
    ) -> Result<CurrenciesResponse, Box<dyn Error>> {
        let url = format!(
            "{}/price/{}/{}/currencies",
            BASIC_URL,
            SPOT_PRICE_API_VERSION,
            self.network_id
        );
        let request_result = self
            .http_client
            .get(url)
            .header("Authorization", &self.token)
            .send()
            .await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let currencies_list_response: CurrenciesResponse = response
            .json()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(currencies_list_response)
    }
}
