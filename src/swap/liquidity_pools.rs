use crate::{
    client::OneInchClient,
    consts::{BASIC_URL, SWAP_API_VERSION},
};
use serde::Deserialize;
use std::error::Error;

/// LiquidityProtocolImage is struct that defines information about LP source.
#[derive(Deserialize, Debug)]
pub struct LiquidityProtocolImage {
    pub id: String,
    pub title: String,
    pub img: String,
    #[serde(rename = "img_color")]
    pub img_color: String,
}

/// LiquidityProtocolsResponse is struct that defines object that server returns
/// on /liquidity-sources request
#[derive(Deserialize, Debug)]
pub struct LiquidityProtocolsResponse {
    pub protocols: Vec<LiquidityProtocolImage>,
}

impl OneInchClient {
    /// Get current list of liquidity sources that are available for routing in
    /// 1inch.
    pub async fn get_liquidity_sources(&self) -> Result<LiquidityProtocolsResponse, Box<dyn Error>> {
        let url = format!("{}/swap/{}/{}/liquidity-sources", BASIC_URL, SWAP_API_VERSION, self.network_id);

        let request_result = self.http_client.get(url).header("Authorization", &self.token).send().await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let liquidity_sources_response: LiquidityProtocolsResponse =
            response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(liquidity_sources_response)
    }
}
