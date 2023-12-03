use crate::swap::approve::{ApproveCallData, ApproveTranactionDetails};
use crate::client::OneInchClient;
use crate::consts::{BASIC_URL, SWAP_API_VERSION};
use reqwest::Url;
use std::error::Error;
use crate::utils::params::insert_optional_param;

impl OneInchClient {
    /// Performs request to get `ApproveCallData` for specific token, account and amount.
    /// Actually we will get a raw transaction which changes the required token`s allowance value to a specified amount on executing.
    pub async fn approve(
        &self,
        details: ApproveTranactionDetails,
    ) -> Result<ApproveCallData, Box<dyn Error>> {
        let url = format!(
            "{}/swap/{}/{}/approve/transaction",
            BASIC_URL,
            SWAP_API_VERSION,
            self.network_id
        );

        let mut params: Vec<(&str, String)> = vec![
            ("chain", self.network_id.to_string()),
            ("tokenAddress", details.token_address),
        ];


        insert_optional_param(&mut params, "amount", details.amount);

        let url_with_params =
            Url::parse_with_params(&url, params).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let request_result = self
            .http_client
            .get(url_with_params)
            .header("Authorization", &self.token)
            .send()
            .await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let approve_response: ApproveCallData = response
            .json()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(approve_response)
    }
}
