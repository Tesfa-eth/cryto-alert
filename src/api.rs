use reqwest::Client;
use std::error::Error;
use serde_json::{Value, from_str};

pub async fn get_price(url: &str) -> Result<String, Box<dyn Error>> {
    // Send a GET request to the 0x API and get the response data.
    let response_data = send_get_request(url).await?;

    // Parse the price from the response data.
    let price = parse_price(&response_data)?;

    Ok(price)
}

pub fn parse_price(response_data: &str) -> Result<String, Box<dyn Error>> {
    // Parse the JSON response data.
    let parsed_data: Value = from_str(response_data)?;

    // Extract the price from the parsed data.
    let price = parsed_data["price"]
        .as_str()
        .ok_or("Failed to extract price")?;

    Ok(price.to_string())
}

pub async fn send_get_request(url: &str) -> Result<String, Box<dyn Error>> {
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