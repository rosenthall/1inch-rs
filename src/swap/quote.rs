use std::error::Error;

use reqwest::Url;
use crate::client::OneInchClient;
use crate::consts::{BASIC_URL, SWAP_API_VERSION};
use crate::swap::{QuoteDetails, QuoteResponse, SwapError, SwapRequestError};
use crate::utils::params::insert_optional_param;

impl OneInchClient {
    /// Gets a `QuoteResponse` via given data..
    pub async fn quote(
        &self,
        details: QuoteDetails,
    ) -> Result<QuoteResponse, Box<dyn Error>> {
        let url = format!(
            "{}/swap/{}/{}/quote/",
            BASIC_URL,
            SWAP_API_VERSION,
            self.network_id
        );

        // Adding required parameters
        let mut params: Vec<(&str, String)> = vec![
            ("src", details.src),
            ("dst", details.dst),
            ("amount", details.amount),
        ];


        // Adding optional bool parameters
        insert_optional_param(&mut params, "includeGas", details.include_gas.map(|a| a.to_string()));
        insert_optional_param(&mut params, "includeProtocols", details.include_protocols.map(|a| a.to_string()));
        insert_optional_param(&mut params, "includeTokensInfo", details.include_tokens_info.map(|a| a.to_string()));


        // Adding optional num parameters
        insert_optional_param(&mut params, "fee", details.fee.map(|a| a.to_string()));
        insert_optional_param(&mut params, "complexityLevel", details.complexity_level.map(|a| a.to_string()));
        insert_optional_param(&mut params, "parts", details.parts.map(|a| a.to_string()));
        insert_optional_param(&mut params, "mainRouteParts", details.main_route_parts.map(|a| a.to_string()));
        insert_optional_param(&mut params, "gasLimit", details.gas_limit.map(|a| a.to_string()));

        // Adding optional string parameters
        insert_optional_param(&mut params, "protocols", details.protocols);
        insert_optional_param(&mut params, "gasPrice", details.gas_price);
        insert_optional_param(&mut params, "connectorTokens", details.connector_tokens);



        let url_with_params =
            Url::parse_with_params(&url, params).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let response = match self
            .http_client
            .get(url_with_params)
            .header("Authorization", &self.token)
            .send()
            .await {
            Ok(response) => response,
            Err(e) => return Err(SwapError::Network(e).into()),
        };

        if response.status().as_u16() == 400 {
            let error_body = response.text().await.unwrap_or_default();
            return match serde_json::from_str::<SwapRequestError>(&error_body) {
                Ok(err) => {
                    Err(SwapError::SwapRequest {
                        description: err.description,
                        error: err.error,
                        status_code: err.status_code,
                        request_id: err.request_id,
                    }.into())
                },
                Err(e) => {
                    Err(SwapError::Other(format!("Error parsing error response: {}", e)).into())
                },
            }
        }

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(SwapError::Other(format!("Server responded with error: {}", response.status())).into());
        }


        let quote_data: QuoteResponse = match response.json().await {
            Ok(data) => data,
            Err(e) => return Err(SwapError::Network(e).into()),
        };

        Ok(quote_data)
    }
}
