//! # Bitfinex API Library & CLI Tool
//!
//! A fully asynchronous Rust library and CLI for accessing the Bitfinex V2 API.
//!
//! ## Highlights
//! - Async-first: Non-blocking, efficient API calls.
//! - CLI included: Query Bitfinex from your terminal.
//! - Lightweight: Minimal dependencies.
//! - Auto-retry: Handles "Nonce: small" errors automatically.
//!
//! ## Example
//! ```rust
//! use bfx::client::Client;
//! async fn run() {
//!     let client = Client::new("".into(), "".into());
//!     let ticker = client.request_trading_ticker("tBTCUSD").await.unwrap();
//!     println!("{:?}", ticker);
//! }
//! ```
//!
//! ## Feature flags
//! - `cli` - Only used when you want to build and run as CLI.
// #[cfg(feature = "cli")]
pub mod cli;
pub mod client;
mod deserializer;
mod error;
mod funding;
mod trading;
pub mod utils;
