use one_inch::{
    client::{self, SupportedCurrencies, SupportedNetworks},
    tokens::tokens_price::TokensPricesRequestBuilder,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Setting the network ID to Binance Smart Chain (BSC)
    let network_id = SupportedNetworks::BSC;

    // Retrieving the API token from the environment variables
    let token = env!("ONE_INCH_API_TOKEN");

    // Creating a new One Inch client with the provided API token and network ID
    let client = client::new_with_default_http(token.into(), network_id);

    // Getting all the custom currencies
    let currencies =
        client.get_custom_currencies().await.map_err(|e| eprintln!("Error while making get currencies request: {}", e)).unwrap();

    println!("Total currencies supported : {}", &currencies.codes.len());

    println!("All the currencies list : ");

    for cr in currencies.codes.iter() {
        print!("{}, ", cr);
    }

    println!();

    // timeout of 5 seconds to avoid server restrictions
    std::thread::sleep(Duration::from_secs(5));

    // Getting prices for couple of specified tokens
    let my_tokens_list: Vec<String> = vec![
        "0xce7de646e7208a4ef112cb6ed5038fa6cc6b12e3".into(), // TRX
        "0x1d2f0da169ceb9fc7b3144628db156f3f6c60dbe".into(), // XRP
        "0xba2ae424d960c26247dd6c32edc70b295c744c43".into(), // DOGE
        "0x7083609fce4d1d8dc0c979aab8c869ea2c873402".into(), // DOT
    ];

    let get_prices_details =
        TokensPricesRequestBuilder::new().addresses(my_tokens_list.clone()).currency(SupportedCurrencies::USD).build().unwrap();

    let prices_usd = client
        .get_tokens_price(get_prices_details)
        .await
        .map_err(|e| eprintln!("Error while getting prices for tokens to usd : {e}"))
        .unwrap();

    // Printing info
    for token in &my_tokens_list {
        match prices_usd.prices.get(token) {
            Some(price) => println!("Price for token {}: {}", token, price),
            None => eprintln!("Price for token {} not found", token),
        }
    }
}
