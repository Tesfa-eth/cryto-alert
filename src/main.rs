use reqwest::Client;
use std::error::Error;
use std::io;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use serde_json::{Value, from_str};

async fn prompt_and_monitor() -> Result<(), Box<dyn Error>> {
    // Get user input for tokens and interval.
    let (sell_token, buy_token) = get_user_input_for_tokens()?;
    let interval = get_user_input_for_interval()?;

    // Start monitoring the price.
    monitor_price_change(&sell_token, &buy_token, interval).await?;

    Ok(())
}

fn get_user_input_for_tokens() -> Result<(String, String), Box<dyn Error>> {
    // Prompt the user for the sell token.
    let sell_token = get_user_input("Enter the sell token");

    // Prompt the user for the buy token.
    let buy_token = get_user_input("Enter the buy token");

    Ok((sell_token, buy_token))
}

fn get_user_input_for_interval() -> Result<u64, Box<dyn Error>> {
    // Prompt the user for the interval.
    let interval_str = get_user_input("Enter the interval in seconds");

    // Validate and convert the interval to a u64.
    let interval = validate_u64_input(&interval_str)?;

    Ok(interval)
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().to_string()
}

fn validate_u64_input(input: &str) -> Result<u64, Box<dyn Error>> {
    let number = input.parse::<u64>()?;
    Ok(number)
}

async fn monitor_price_change(sell_token: &str, buy_token: &str, interval: u64) -> Result<(), Box<dyn Error>> {
    println!("Monitoring price change for {} -> {}", sell_token, buy_token);
    // Create the URL for the 0x API.
    let url = format!("https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken={}&sellToken={}", sell_token, buy_token);
    
    println!("Checking initial price...");
    // Get the current price.
    let mut current_price = match get_price(&url).await {
        Ok(price) => price,
        Err(e) => {
            println!("Failed to fetch initial price: {:?}", e);
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


async fn get_price(url: &str) -> Result<String, Box<dyn Error>> {
    // Send a GET request to the 0x API and get the response data.
    let response_data = send_get_request(url).await?;

    // Parse the price from the response data.
    let price = parse_price(&response_data)?;

    Ok(price)
}

async fn send_get_request(url: &str) -> Result<String, Box<dyn Error>> {
    // Create a new Client.
    let client = Client::new();

    // Send the GET request and get the Response.
    let response = client.get(url).send().await?;

    // Check if the status is success.
    if !response.status().is_success() {
        return Err("GET request failed".into());
    }

    // Get the response body as a string.
    let body = response.text().await?;

    Ok(body)
}

fn parse_price(response_data: &str) -> Result<String, Box<dyn Error>> {
    // Parse the JSON response data.
    let parsed_data: Value = from_str(response_data)?;

    // Extract the price from the parsed data.
    let price = parsed_data["price"]
        .as_str()
        .ok_or("Failed to extract price")?;

    Ok(price.to_string())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(prompt_and_monitor()).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_validate_u64_input() {
        assert!(validate_u64_input("5").is_ok());
        assert!(validate_u64_input("five").is_err());
    }

    #[test]
    fn test_parse_price() {
        let response_data = r#"{
            "price": "123.45"
        }"#;
        assert_eq!(parse_price(response_data).unwrap(), "123.45");
    }

    #[test]
    fn test_send_get_request() {
        let fake_url = "https://httpbin.org/get";
        let response = block_on(send_get_request(fake_url));
        assert!(response.is_ok());
    }
}
