use std::io;
use std::error::Error;

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().to_string()
}

pub fn get_user_input_for_tokens() -> Result<(String, String), Box<dyn Error>> {
    // Prompt the user for the sell token.
    let sell_token = get_user_input("Enter the sell token");

    // Prompt the user for the buy token.
    let buy_token = get_user_input("Enter the buy token");

    Ok((sell_token, buy_token))
}

pub fn get_user_input_for_interval() -> Result<u64, Box<dyn Error>> {
    // Prompt the user for the interval.
    let interval_str = get_user_input("Enter the interval in seconds");

    // Validate and convert the interval to a u64.
    let interval = validate_u64_input(&interval_str)?;

    Ok(interval)
}

fn validate_u64_input(input: &str) -> Result<u64, Box<dyn Error>> {
    let number = input.parse::<u64>()?;
    Ok(number)
}