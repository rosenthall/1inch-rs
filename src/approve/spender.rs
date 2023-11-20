use std::error::Error;
use crate::approve::{RouterAddress, SpenderDetails};
use crate::client::OneInchClient;
use crate::consts::{BASIC_URL, SWAP_API_VERSION};

impl OneInchClient {
    pub async fn get_router_address(&self, details: SpenderDetails) -> Result<RouterAddress, Box<dyn Error>> {

        let url = format!("{}/swap/{}/{}/approve/spender", BASIC_URL, SWAP_API_VERSION, details.chain);

        let request_result = self.http_client
            .get(url)
            .header("Authorization", &self.token)
            .send()
            .await;

        let response = request_result
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .error_for_status()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let address: RouterAddress = response.json().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;


        Ok(address)

    }
}