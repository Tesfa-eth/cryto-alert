# Crypto Price Monitor

Crypto Price Monitor is a command line application built in Rust that fetches and monitors the price of a cryptocurrency pair from the 0x API. The user is prompted to enter two tokens and an interval in seconds. If the guaranteed buying price of the token pair changes, the application sends an email notification and terminates.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

Ensure that you have the following installed on your local machine:

- [Rust](https://www.rust-lang.org/tools/install)

### Installing

Clone the repo to your local machine:

```bash
git clone https://github.com/YOUR_USERNAME/crypto-price-monitor.git
```

Navigate into the directory:
```
cd crypto-alert
```

Build the project:
```
cargo build
```
Run the project:
```
cargo run
```
## Running the tests
```
cargo test
```

## How to use

After running cargo run, follow the on-screen prompts to enter the sell token, the buy token, and the monitoring interval in seconds.

The program will start monitoring the price of the specified token pair, checking the 0x API at the specified interval. If the buying price of the token pair changes, the application will notify and then terminate.

### Built with

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used
- [Reqwest](https://docs.rs/reqwest/) - The HTTP client library
- [Tokio](https://tokio.rs/) - The asynchronous runtime used
- [Serde JSON](https://docs.serde.rs/serde_json/) - The library used for JSON serialization and deserialization



