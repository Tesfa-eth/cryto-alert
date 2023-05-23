use crate::api::get_price;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub async fn monitor_price_change(sell_token: &str, buy_token: &str, interval: u64) -> Result<(), Box<dyn Error>> {
    println!("Monitoring price change for {} -> {}", sell_token, buy_token);
    // Create the URL for the 0x API.
    let url = format!("https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken={}&sellToken={}", sell_token, buy_token);
    
    println!("Checking initial price...");
    // Get the current price.
    let mut current_price = match get_price(&url).await {
        Ok(price) => price,
        Err(e) => {
            println!("Failed to fetch initial price: {:?}. Please make sure you are using a valid token name", e);
            return Ok(());  // Return early.
        }
    };

    println!("Current price: {}", current_price);

    loop {
        println!("Checking price...");
        // Wait for the specified interval.
        sleep(Duration::from_secs(interval)).await;

        // Get the new price.
        match get_price(&url).await {
            Ok(new_price) => {
                // Compare the new price with the current price.
                if new_price != current_price {
                    println!("Price changed: {} -> {}", current_price, new_price);
                    current_price = new_price;
                }
            }
            Err(e) => {
                println!("Failed to fetch price: {:?}", e);
                continue;  // Skip this iteration.
            }
        };

        println!("Price unchanged. Checking again in {} seconds...", interval);
    }

    
}