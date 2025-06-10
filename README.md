# Bitfinex API Library & CLI Tool (Rust)

A fully asynchronous Rust library and CLI for accessing the [Bitfinex V2 API](https://docs.bitfinex.com/reference).

- Async-first: Non-blocking, efficient API calls.
- CLI included: Query Bitfinex from your terminal.
- Lightweight: Minimal dependencies.
- Auto-retry: Handles "Nonce: small" errors automatically.

## Quick Start

### Use in Rust Project
Add to Your Rust Project:
```bash
cargo add bfx
```

### Example usage
```rust
use bfx::client::Client;

async fn async_main() {
    let client = Client::new("".to_string(), "".to_string());
    let ticker = client.request_trading_ticker("tBTCUSD").await.unwrap();

    // Required `serde_json` crate.
    // Run `cargo add serde_json`
    println!("{}", serde_json::to_string_pretty(&ticker));
    // Example output:
    // {
    //     "bid": 109440.0,
    //     "bid_size": 73.57899877,
    //     "ask": 109450.0,
    //     "ask_size": 17.71589491,
    //     "daily_change": 3680.0,
    //     "daily_change_relative": 0.03479247,
    //     "last_price": 109450.0,
    //     "volume": 392.35200592,
    //     "high": 110680.0,
    //     "low": 105480.0
    // }
}

fn main() {
    // Required `tokio` crate.
    // Run `cargo add tokio --no-default-features`
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}
```

### CLI Installation

**With script (Mac/Linux):**
```bash
curl -sSL https://raw.githubusercontent.com/BreezeWhite/bfx-rs/main/install.sh | bash
```

**With script (Windows):**
```ps1
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
iwr -useb https://raw.githubusercontent.com/BreezeWhite/bfx-rs/main/install_win.ps1 | iex
```

**Wth Cargo:**

```bash
cargo install bfx --features cli
```

## CLI Usage

```bash
A convenient CLI tool for Bitfinex

Version: 0.1.0
Author: BreezeWhite, <miyashita2010@tuta.io>

Usage: bfx <COMMAND>

Commands:
  trading  Trading/exchange related utilities
  funding  Funding-related utilities
  public   Public endpoints that does not related to trading nor funding
  auth     User-related utilities
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## More Examples

### Initialize .env file

Initialize the `.bfx_cli.env` file to store API key and API secret for later use.

```rust
use bfx::client::Client;
use bfx::utils::resolve_env_path_or_create;

fn main() {
    // This will first try to find the `.bfx_cli.env` file from current directory.
    // If not found then try finding under $HOME directory of currcent user.
    // If all fails, this will ask the user to input the API key and secret,
    // Then store them as `.bfx_cli.env` file under the $HOME directory.
    let env_path = resolve_env_path_or_create();

    // Then you can use the path to the .env file to load API key and secret
    // as environment variables with `dotenv` crate.
    dotenv::from_path(env_path).expect("Failed to load .env file");

    // After loading the .env file, you can now access the API key and
    // secret using std::env::var.
    let api_key = std::env::var("API_KEY").unwrap();
    let api_secret = std::env::var("API_SECRET").unwrap();

    // Use the key and secret to initialize the client.
    let client = Client::new(api_key, api_secret);
}
```

For CLI, `bfx` will automatically detect if there is a need to ask the
user to input API key and secret when calling to authenticated endpoints.
When call to public endpoints, there is no need to have a .env file.
