use reqwest::Client;
use std::error::Error;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use std::time::Duration;

use lettre::message::{Message, Mailbox};
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::Transport;

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

async fn monitor_price_change() -> Result<(), Box<dyn Error>> {
    let mut previous_price = String::new();

    loop {
        println!("Monitoring price change... previous_price {}" , previous_price);
        let response_data = make_get_request().await?;
        let buy_price = parse_response_data(&response_data)?;

        if !previous_price.is_empty() && buy_price != previous_price {
            println!("Buy price changed: {}", buy_price);
            // send_email(&buy_price)?;
            break;
        }

        previous_price = buy_price;
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

// fn send_email(new_price: &str) -> Result<(), Box<dyn Error>> {
//     let email = Message::builder()
//         .from("you@example.com".parse().unwrap())
//         .to("stesfatsionmulugeta@gmail.com".parse().unwrap())
//         .subject("DAI Buy Price Changed")
//         .body(format!("The new guaranteed buying price of DAI is: {}", new_price))?;

//     // Replace smtp.example.com with the address of your SMTP server
//     let smtp_address = "smtp.example.com";
//     let mailer = SmtpTransport::relay(&smtp_address)?
//         .credentials("your_username", "your_password")
//         .build();

//     mailer.send(&email)?;

//     println!("Email sent");

//     Ok(())
// }

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(monitor_price_change()).unwrap();
}
