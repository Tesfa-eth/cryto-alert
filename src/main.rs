use reqwest::Client;
use std::error::Error;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use std::time::Duration;

use serde_json::{Value, from_str};

// The URL for the 0x API.
const URL: &str = "https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken=DAI&sellToken=USDC";

// The interval at which to check the price, in seconds.
const INTERVAL: u64 = 5;

/// Starts the application.
fn main() {
    let rt = Runtime::new().unwrap();
    println!("Starting application...");
    rt.block_on(monitor_price_change()).unwrap();
}

/// Monitors the price of DAI on the 0x API.
/// If the price changes, it prints a message and returns.
async fn monitor_price_change() -> Result<(), Box<dyn Error>> {
    println!("Monitoring price...");
    let mut previous_price = None;

    loop {
        // Get the current price from the 0x API.
        let current_price = get_price().await?;

        println!("Current price: {}", current_price);

        // If there is a previous price and it's different from the current price, print a message and return.
        if let Some(previous_price) = previous_price {
            if previous_price != current_price {
                println!("Price changed from {} to {}", previous_price, current_price);
                return Ok(());
            }
        }

        // Set the previous price to the current price for the next iteration.
        previous_price = Some(current_price);

        // Wait for the interval duration before the next check.
        sleep(Duration::from_secs(INTERVAL)).await;
    }
}

/// Gets the current price of DAI from the 0x API.
async fn get_price() -> Result<String, Box<dyn Error>> {
    // Send the GET request to the 0x API.
    let response_data = send_get_request(URL).await?;

    // Parse the price from the response data.
    let price = parse_price(&response_data)?;

    Ok(price)
}

/// Sends a GET request to the given URL and returns the response data as a string.
async fn send_get_request(url: &str) -> Result<String, Box<dyn Error>> {
    // Create a new Client.
    let client = Client::new();

    // Send the GET request.
    let response = client.get(url).send().await?;

    // Check if the status is success.
    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Non-success status code")));
    }

    // Get the response body as a string.
    let body = response.text().await?;

    Ok(body)
}

/// Parses the price of DAI from the given 0x API response data.
fn parse_price(response_data: &str) -> Result<String, Box<dyn Error>> {
    // Parse the response data into a Value.
    let parsed_data: Value = from_str(response_data)?;

    // Extract the price from the Value.
    let price = parsed_data["guaranteedPrice"]
        .as_str()
        .ok_or("Failed to extract price")?;

    Ok(price.to_string())
}