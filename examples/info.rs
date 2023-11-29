use std::time::Duration;
use one_inch::client::{self, SupportedNetworks};


/*
 In this example we will get list of tokens that are available for swap in the 1inch
 and also list of liquidity sources
*/


#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);

    let tokens_list_response = client.get_tokens_list().await.map_err(|e| eprintln!("Got error while getting tokens list on 1inch : {e:?}")).unwrap();
    println!("Amount on tokens in 1inch in BSC is : {}", tokens_list_response.tokens.len());



    println!("Sleeping 5 seconds before making a second request..");
    tokio::time::sleep(Duration::from_secs(5)).await;


    let lp_pools_response = client.get_liquidity_sources().await.map_err(|e| eprintln!("Got error while getting liquidity pools on 1inch : {e:?}")).unwrap();

    println!("{:?}", lp_pools_response.protocols)
}
