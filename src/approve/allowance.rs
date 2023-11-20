use std::error::Error;
use reqwest::Url;
use crate::client::{OneInchClient};
use crate::approve::{AllowanceDetails, AllowanceResponse};
use crate::consts::{BASIC_URL, SWAP_API_VERSION};


impl OneInchClient {
    pub async fn get_allowance(&self, details: AllowanceDetails) -> Result<AllowanceResponse, Box<dyn Error>> {

        let url = format!("{}/swap/{}/{}/approve/allowance", BASIC_URL, SWAP_API_VERSION, self.network_id.to_string());

        let url_with_params = Url::parse_with_params(&url, &[
            ("tokenAddress", details.token_address),
            ("walletAddress", details.wallet_address),
        ]).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let request_result = self.http_client
            .get(url_with_params)
            .header("Authorization", &self.token)
            .send()
            .await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let allowance_response: AllowanceResponse = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(allowance_response)
    }
}