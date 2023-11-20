use one_inch::approve::*;
use one_inch::client::{self, SupportedNetworks};

#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);

    let usdt_address_bsc = "0x55d398326f99059ff775485246999027b3197955".to_string();

    // Getting raw tx that can be used to create transaction in blockchain
    let approve_details = ApproveTranactionDetailsBuilder::new()
        .amount(Some("5000000000000000".to_string()))
        .chain(53)
        .token_address(usdt_address_bsc)
        .build()
        .unwrap();

    let response = client
        .approve(approve_details)
        .await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while getting raw tx: {}", e)
        })
        .unwrap();

    dbg!(response);
}
