use reqwest::Client;
use std::io::{self, Write};
use std::error::Error;
use std::process;
use std::str::FromStr;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use std::time::Duration;
use serde_json::{Value, from_str};

// prompt the user for input and return it as a string.
fn get_input(prompt: &str) -> io::Result<String> {
    // Print the prompt to stdout.
    print!("{}", prompt);
    io::stdout().flush()?;

    // Read a line from stdin.
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Remove the trailing newline.
    input.truncate(input.trim_end().len());

    Ok(input)
}

// validate the user's input as a u64.
fn validate_u64_input(input: &str) -> Result<u64, Box<dyn Error>> {
    let interval = u64::from_str(input)?;

    Ok(interval)
}

// Monitors the price of DAI on the 0x API.
// If the price changes, it prints a message and returns.
async fn monitor_price_change(url: &str, interval: u64) -> Result<(), Box<dyn Error>> {
    let mut previous_price = None;

    loop {
        // Get the current price from the 0x API.
        let current_price = get_price(url).await?;

        // If there is a previous price and it's different from the current price, print a message and return.
        if let Some(previous_price) = previous_price {
            if previous_price != current_price {
                println!("Price changed from {} to {}", previous_price, current_price);
                return Ok(());
            }
        }

        // Set the previous price to the current price for the next iteration.
        previous_price = Some(current_price);

               // Wait for the specified interval.
        sleep(Duration::from_secs(interval)).await;
    }
}

// get the current price from the 0x API.
async fn get_price(url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching current price...");
    
    // Send a GET request to the 0x API and get the response data.
    let response_data = send_get_request(url).await?;

    // Parse the price from the response data.
    let price = parse_price(&response_data)?;

    println!("Current price: {}", price);
    Ok(price)
}

// send a GET request to the given URL and returns the response data.
async fn send_get_request(url: &str) -> Result<String, Box<dyn Error>> {
    // Create a new Client.
    let client = Client::new();
    // println!("Sending GET request to 0x API...");

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

// Function to parse the price from the 0x API response data.
fn parse_price(response_data: &str) -> Result<String, Box<dyn Error>> {
    // Parse the response data into a Value.
    let parsed_data: Value = from_str(response_data)?;

    // Extract the price from the Value.
    let price = parsed_data["price"]
        .as_str()
        .ok_or("Failed to extract price")?;

    Ok(price.to_string())
}

fn main() {
    // Print a welcome message.
    println!("Welcome to the 0x API Price Monitor!");

    // Prompt the user for the buy token, sell token, and interval.
    let buy_token = get_input("Enter the token to buy: ").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    let sell_token = get_input("Enter the token to sell: ").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    let interval = get_input("Enter the interval in seconds to check the price: ")
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });
    let interval = validate_u64_input(&interval).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    // Build the URL for the 0x API.
    let url = format!(
        "https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken={}&sellToken={}",
        buy_token, sell_token
    );

    // Create a new Tokio Runtime.
    let rt = Runtime::new().unwrap();

    // Print a message indicating that price monitoring is starting.
    println!("Starting price monitoring...");

    // Block on the price monitoring task.
    rt.block_on(monitor_price_change(&url, interval)).unwrap();
}