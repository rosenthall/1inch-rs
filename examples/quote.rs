use one_inch::{
    client::{self, SupportedNetworks},
    swap::QuoteDetailsBuilder,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;

    // Contract addresses of tokens we want to swap
    let src = "0x55d398326f99059ff775485246999027b3197955".to_string(); // USDT address in bsc
    let dst = "0x1D2F0da169ceB9fC7B3144628dB156f3F6c60dBE".to_string(); // XRP address in bsc
    let bnb_in_wei = "1000000000000000000".to_string(); //(10 ^ -18)

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);

    // Making basic swap request
    let simple_quote_details =
        QuoteDetailsBuilder::new().amount(bnb_in_wei.clone()).src(src.clone()).dst(dst.clone()).fee(2).unwrap().build().unwrap();

    let basic_quote = client
        .quote(simple_quote_details)
        .await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while making quote request: {}", e)
        })
        .unwrap();

    println!("Response for perfoming basic quote request: {:#?}", basic_quote);

    println!("Sleeping 5 seconds before making a second request..");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Making request when includeTokensInfo + includeProtocols + includeGas is true

    let extended_quote_details = QuoteDetailsBuilder::new()
        .amount(bnb_in_wei.clone())
        .src(src.clone())
        .dst(dst.clone())
        .fee(2)
        .unwrap()
        .include_gas(true)
        .include_protocols(true)
        .include_tokens_info(true)
        .build()
        .unwrap();

    let extented_quote = client
        .quote(extended_quote_details)
        .await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while making quote request: {}", e)
        })
        .unwrap();

    println!("Response for perfoming extented quote request: {:#?}", extented_quote);

    // Making quote request where server should return error.
    // Error will approach because we making request withot any timeouts
    // So we will get 429 error code
    let error_swap_details = QuoteDetailsBuilder::new().amount(bnb_in_wei).src(dst).dst(src).build().unwrap();

    let error_quote = client.quote(error_swap_details).await;

    println!("Got error(which is good!) for third quote request : {:#?}", error_quote.unwrap_err());
}
