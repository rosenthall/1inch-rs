use one_inch::{
    client::{self, SupportedNetworks},
    swap::approve::*,
};

#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);

    // Creating allowance details using the AllowanceDetailsBuilder
    let allowance_details = AllowanceDetailsBuilder::new()
        .wallet_address("0x30A557351eab496FD69F537BE1F8c744A18F94Fd".into())
        .token_address("0x55d398326f99059ff775485246999027b3197955".into())
        .build()
        .unwrap();

    // Calling the One Inch API to get the allowance for the specified details
    let allowance = client
        .get_allowance(allowance_details.clone())
        .await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while getting allowance : {}", e)
        })
        .unwrap();

    // Printing the obtained allowance information
    println!(
        "Allowance of token {} on account {} in network {:?} is : {} ",
        allowance_details.token_address, allowance_details.wallet_address, network_id, allowance.allowance
    )
}
