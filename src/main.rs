use reqwest::Client;
use std::error::Error;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use std::time::Duration;
use std::fs::File;
use std::path::Path;

use plotters::prelude::*;

use serde_json::{Value, from_str};

fn parse_response_data(response_data: &str) -> Result<f64, Box<dyn Error>> {
    let parsed_data: Value = from_str(response_data)?;

    let buy_price = parsed_data["guaranteedPrice"]
        .as_str()
        .ok_or("Failed to extract buying price of DAI")?;

    Ok(buy_price.parse()?)
}

async fn make_get_request() -> Result<String, Box<dyn Error>> {
    let url = "https://api.0x.org/swap/v1/quote?sellAmount=100000000&buyToken=DAI&sellToken=USDC";

    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    Ok(body)
}

async fn monitor_price_change() -> Result<(), Box<dyn Error>> {
    let mut price_data = vec![];

    loop {
        let response_data = make_get_request().await?;
        let buy_price = parse_response_data(&response_data)?;

        price_data.push(buy_price);

        draw_chart(&price_data)?;

        sleep(Duration::from_secs(300)).await;
    }
}

fn draw_chart(data: &[f64]) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("price_chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_price = *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
    let min_price = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("DAI Price Chart", ("sans-serif", 40).into_font())
        .build_ranged(0f32..(data.len() as f32), min_price as f32..max_price as f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(i, price)| (i as f32, *price as f32)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(monitor_price_change()).unwrap();
}
