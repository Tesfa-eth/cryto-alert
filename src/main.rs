use reqwest::Client;
use std::error::Error;
use tokio::runtime::Runtime;

use serde_json::{Value, from_str};

fn parse_response_data(response_data: &str) -> Result<String, Box<dyn Error>> {
    let parsed_data: Value = from_str(response_data)?;

    let buy_price = parsed_data["guaranteedPrice"]
        .as_str()
        .ok_or("Failed to extract buying price of DAI")?;

    Ok(buy_price.to_string())
}

async fn make_get_request() -> Result<String, Box<dyn Error>> {
    let url = "https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken=DAI&sellToken=USDC";

    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    Ok(body)
}

fn main() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(make_get_request());

    let response_data = match result {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    println!("Response data: {}", response_data);

    let buy_price = parse_response_data(&response_data).unwrap();
    println!("Buy price: {}", buy_price);
}
