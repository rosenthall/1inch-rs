use std::time::Duration;
use num_bigint::BigUint;
use one_inch::approve::*;
use one_inch::client::{self, SupportedNetworks};
use one_inch::swap::SwapDetailsBuilder;

#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;


    // Contract addresses of tokens we want to swap
    let src = "0x55d398326f99059ff775485246999027b3197955".to_string(); // USDT address in bsc
    let dst = "0x1D2F0da169ceB9fC7B3144628dB156f3F6c60dBE".to_string(); // XRP address in bsc
    let my_address= "0x13961a09bCD42DCC078765286Be746d87f20E82e".to_string();
    let bnb_in_wei = "1000000000000000000".to_string(); //(10 ^ -18)

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);


    // Making basic swap
    let swap_details = SwapDetailsBuilder::new()
        .amount(bnb_in_wei.clone()) // 5 * (10 ^ -18)
        .from_addr(my_address.clone())
        .src(src.clone())
        .dst(dst.clone())
        .slippage(2).unwrap()
        .build().unwrap();


    let basic_swap = client.swap(swap_details).await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while getting raw swap tx: {}", e)
        }).unwrap();


    println!("Response for perfoming basic swap : {:#?}", basic_swap);


    // timeout of 5 seconds to avoid server restrictions
    tokio::time::sleep(Duration::from_secs(5)).await;


    // Making swap with some additional parameters
    let extended_swap_details = SwapDetailsBuilder::new()
        .amount(bnb_in_wei) // 5 * (10 ^ -18)
        .from_addr(my_address)
        .src(src)
        .dst(dst)
        .slippage(2).unwrap()
        .include_tokens_info(true)
        .include_gas(true)
        .include_protocols(true)
        .build().unwrap();


    let extended_swap = client.swap(extended_swap_details).await
        .map_err(|e| {
            // Handling and printing an error if it occurs
            eprintln!("Error while getting raw swap tx: {}", e)
        }).unwrap();

    println!("Response for perfoming another swap : {:#?}", extended_swap);

}
