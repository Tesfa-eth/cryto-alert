use std::error::Error;
use tokio::runtime::Runtime;
mod api;
mod input;
mod monitor;

use input::{get_user_input_for_interval, get_user_input_for_tokens};
use monitor::monitor_price_change;


async fn prompt_and_monitor() -> Result<(), Box<dyn Error>> {
    // Get user input for tokens and interval.
    let (sell_token, buy_token) = get_user_input_for_tokens()?;
    let interval = get_user_input_for_interval()?;
    // Start monitoring the price.
    monitor_price_change(&sell_token, &buy_token, interval).await?;

    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(prompt_and_monitor()).unwrap();
}
