use std::error::Error;

use crate::{
    client::OneInchClient,
    consts::{BASIC_URL, SWAP_API_VERSION, SWAP_V6_API_VERSION},
    swap::{SwapDetails, SwapError, SwapRequestError, SwapResponse},
    utils::params::insert_optional_param,
};
use reqwest::Url;

use super::{SwapDetailsV6, SwapV6Response};

impl OneInchClient {
    /// Performs swap request with predefined parameters.
    pub async fn swap(&self, details: SwapDetails) -> Result<SwapResponse, Box<dyn Error>> {
        let url = format!("{}/swap/{}/{}/swap/", BASIC_URL, SWAP_API_VERSION, self.network_id);

        // Adding required parameters
        let mut params: Vec<(&str, String)> = vec![
            ("from", details.from),
            ("slippage", details.slippage.to_string()),
            ("src", details.src),
            ("dst", details.dst),
            ("amount", details.amount),
        ];

        // Adding optional bool parameters
        insert_optional_param(&mut params, "disableEstimate", details.disable_estimate.map(|a| a.to_string()));
        insert_optional_param(&mut params, "allowPartialFill", details.allow_partial_fill.map(|a| a.to_string()));
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
        insert_optional_param(&mut params, "permit", details.permit);
        insert_optional_param(&mut params, "receiver", details.receiver);
        insert_optional_param(&mut params, "referrer", details.referrer);

        let url_with_params = Url::parse_with_params(&url, params).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let response = match self.http_client.get(url_with_params).header("Authorization", &self.token).send().await {
            Ok(response) => response,
            Err(e) => return Err(SwapError::Network(e).into()),
        };

        if response.status().as_u16() == 400 {
            let error_body = response.text().await.unwrap_or_default();
            return match serde_json::from_str::<SwapRequestError>(&error_body) {
                Ok(err) => Err(SwapError::SwapRequest {
                    description: err.description,
                    error: err.error,
                    status_code: err.status_code,
                    request_id: err.request_id,
                }
                .into()),
                Err(e) => Err(SwapError::Other(format!("Error parsing error response: {}", e)).into()),
            };
        }

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(SwapError::Other(format!("Server responded with error: {}", response.status())).into());
        }

        let swap_data: SwapResponse = match response.json().await {
            Ok(data) => data,
            Err(e) => return Err(SwapError::Network(e).into()),
        };

        Ok(swap_data)
    }

        /// Performs swap request with predefined parameters.
        pub async fn swap_v6(&self, details: SwapDetailsV6) -> Result<SwapV6Response, Box<dyn Error>> {
            let url = format!("{}/swap/{}/{}/swap/", BASIC_URL, SWAP_V6_API_VERSION, self.network_id);
    
            // Adding required parameters
            let mut params: Vec<(&str, String)> = vec![
                ("from", details.from),
                ("slippage", details.slippage.to_string()),
                ("src", details.src),
                ("dst", details.dst),
                ("amount", details.amount),
                ("origin", details.origin)
            ];
    
            // Adding optional bool parameters
            insert_optional_param(&mut params, "disableEstimate", details.disable_estimate.map(|a| a.to_string()));
            insert_optional_param(&mut params, "allowPartialFill", details.allow_partial_fill.map(|a| a.to_string()));
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
            insert_optional_param(&mut params, "permit", details.permit);
            insert_optional_param(&mut params, "receiver", details.receiver);
            insert_optional_param(&mut params, "referrer", details.referrer);

            insert_optional_param(&mut params, "usePermit2", details.use_permit2.map(|a| a.to_string()));
    
            let url_with_params = Url::parse_with_params(&url, params).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
            let response = match self.http_client.get(url_with_params).header("Authorization", &self.token).send().await {
                Ok(response) => response,
                Err(e) => return Err(SwapError::Network(e).into()),
            };
    
            if response.status().as_u16() == 400 {
                let error_body = response.text().await.unwrap_or_default();
                return match serde_json::from_str::<SwapRequestError>(&error_body) {
                    Ok(err) => Err(SwapError::SwapRequest {
                        description: err.description,
                        error: err.error,
                        status_code: err.status_code,
                        request_id: err.request_id,
                    }
                    .into()),
                    Err(e) => Err(SwapError::Other(format!("Error parsing error response: {}", e)).into()),
                };
            }
    
            if response.status().is_client_error() || response.status().is_server_error() {
                return Err(SwapError::Other(format!("Server responded with error: {}", response.status())).into());
            }
    
            let swap_data: SwapV6Response = match response.json().await {
                Ok(data) => data,
                Err(e) => return Err(SwapError::Network(e).into()),
            };
    
            Ok(swap_data)
        }
}

#[cfg(test)]
mod tests {
    use crate::{client::{new_with_default_http, SupportedNetworks}, swap::SwapDetailsV6Builder};
    
    #[tokio::test]
    async fn test_swap_v6() {
        let client = new_with_default_http("Your OneInch API KEY".to_string(), SupportedNetworks::Base);
        // let 
        let builder = SwapDetailsV6Builder::new()
        .src("0x4200000000000000000000000000000000000006".to_string())
        .dst("0x833589fcd6edb6e08f4c7c32d4f71b54bda02913".to_string())
        .amount("1000000000000000000".to_string())
        .from("0xDCc3100ba3768D277cABffe2f117887A661ee5A4".to_string())
        .origin("0xDCc3100ba3768D277cABffe2f117887A661ee5A4".to_string())
        .slippage(10).unwrap()
        .fee(0).unwrap()
        .gas_price(1000000000.to_string())
        .gas_limit(10000000)
        .disable_estimate(true)
        .use_permit2(true);

        let params = builder.build().unwrap();
        let res = client.swap_v6(params).await.unwrap();
        println!("swap_v6 response: {:?}", res);
    }
}