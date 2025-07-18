#![cfg(feature = "cli")]

use chrono::{DateTime, Local};
use clap::builder::PossibleValuesParser;
use clap::{Parser, Subcommand, value_parser};

use crate::client::Client;
use crate::utils::resolve_env_path_or_create;

/// A convenient CLI tool for Bitfinex
#[derive(Parser)]
#[command(
    author,
    version,
    help_template = "\
{before-help}
{about-with-newline}
Version: {version}
Author: {author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Trading {
        #[command(subcommand)]
        action: TradingAction,
    },
    Funding {
        #[command(subcommand)]
        action: FundingAction,
    },
    Public {
        #[command(subcommand)]
        action: PublicAction,
    },
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
}

/// Funding-related utilities
#[derive(Subcommand)]
enum FundingAction {
    // --- Public actions --- ///
    /// Get current funding offers in the book.
    Book {
        /// Symbol to get the order book for.
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "2",
            value_parser = value_parser!(u8).range(1..=4),
            help = "Decimal precision level of rates.",
        )]
        precision: u8,
    },
    /// Get raw book content
    RawBook { symbol: String },
    /// Get current funding ticker.
    Ticker { symbol: String },
    /// Get public funding candle data.
    Candles {
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "30",
            value_parser = value_parser!(u8).range(2..=120),
        )]
        period: Option<u8>,

        #[arg(
            short,
            long,
            default_value = "30",
            value_parser = PossibleValuesParser::new(["0", "10", "30", "120"]),
            help = "Aggregation period. 0 means no aggregation.",
        )]
        agg_period: Option<String>,

        #[arg(
            short,
            long,
            default_value = "30m",
            value_parser = PossibleValuesParser::new(["1m", "5m", "15m", "30m", "1h", "3h", "4h", "6h", "12h", "1d", "1w", "2w", "1M"]),
            help = "Time frame for the candles. Default is 30 minutes.",
        )]
        time_frame: Option<String>,

        #[arg(
            long,
            default_value = "20",
            value_parser = value_parser!(u16).range(1..=10000),
            help = "Number of candles to return (max 10000).",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the candles in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the candles in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },

    /// Get public funding trade data.
    Trades {
        /// Symbol to get trades for.
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "100",
            value_parser = value_parser!(u16).range(1..=10000),
            help = "Number of trades to return (max 10000).",
        )]
        limit: u16,

        #[arg(
            long,
            help = "Start time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    // --- Authenticated actions --- ///
    /// Submit a new funding offer
    Submit {
        /// Symbol to offer funding for (e.g., "fUSD", "fBTC").
        symbol: String,

        #[arg(
            short,
            long,
            required = true,
            help = "Amount of funding to offer (e.g., 1000.0 for 1000 USD)."
        )]
        amount: f64,

        #[arg(short, long, required = true, help = "Daily rate of funding to offer.")]
        rate: f64,

        #[arg(
            short,
            long,
            required = true,
            value_parser = value_parser!(u8).range(2..=120),
            help = "Period of the funding offer in days (2-120).",
        )]
        period: u8,

        #[arg(
            long,
            default_value = "LIMIT",
            value_parser = PossibleValuesParser::new(["LIMIT", "FRRDELTAVAR", "FRRDELTAFIX"]),
        )]
        order_type: Option<String>,
    },
    /// Cancels an existing Funding Offer based on the offer ID entered.
    Cancel {
        /// ID of the funding offer to cancel.
        id: u64,
    },
    /// Cancel all of your current funding offers of a specific symbol.
    CancelAll {
        /// Symbol to get the funding credit for (e.g., "fUSD", "fBTC").
        symbol: String,
    },
    /// Get active funding offers.
    Offers {
        /// Symbol to get the funding credit for (e.g., "fUSD", "fBTC").
        symbol: String,
    },
    /// Funds used in active positions
    Credits {
        /// Symbol to get the funding credit for (e.g., "fUSD", "fBTC").
        symbol: String,
    },
    /// Get past inactive funding offers.
    HistOffers {
        /// Symbol to get the funding credit for (e.g., "fUSD", "fBTC").
        symbol: String,

        #[arg(
            long,
            default_value = "20",
            value_parser = value_parser!(u16).range(1..=500),
            help = "Number of candles to return (max 500).",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    /// Inactive funds used in positions.
    HistCredits {
        /// Symbol to get the funding credit for (e.g., "fUSD", "fBTC").
        symbol: String,

        #[arg(
            long,
            default_value = "20",
            value_parser = value_parser!(u16).range(1..=500),
            help = "Number of records to return (max 500).",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the credits in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the credits in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
}

/// User-related utilities
#[derive(Subcommand)]
enum AuthAction {
    /// Get current user information.
    UserInfo,
    /// Get all wallets of current user.
    Wallets,
    /// Get permissions of current API key.
    KeyPermission,
    /// Get ledger records of current user.
    Ledger {
        /// Currency to filter the ledger records by.
        ccy: String,

        #[arg(
            short,
            long,
            default_value="25",
            value_parser = value_parser!(u16).range(1..=2500),
            help = "Number of records to return (max: 2500).",
        )]
        limit: Option<u16>,

        #[arg(
            short,
            long,
            default_value = "Interest",
            value_parser = PossibleValuesParser::new(["Interest", "Exchange", "Transfer", "TradingFee"]),
            help = "Type of ledger records to return.",
        )]
        category: Option<String>,
    },
    /// Get wallet addresses for deposit
    DepositAddress {
        #[arg(
            short,
            long,
            default_value = "exchange",
            value_parser = PossibleValuesParser::new(["exchange", "margin", "funding"]),
            help = "Type of wallet."
        )]
        wallet_type: String,

        #[arg(
            short,
            long,
            default_value = "tetherusl",
            value_parser = PossibleValuesParser::new([
                "bitcoin", "litecoin", "ethereum", "tetheruso", "tetherusl", "tetherusx", "tetheruss",
                "ethereumc", "zcash", "monero", "iota"]),
            help = "Deposit method"

        )]
        method: String,
    },
}

/// Public endpoints that does not related to trading nor funding
#[derive(Subcommand)]
enum PublicAction {
    /// Get various statistics on a specified trading pair or funding currency.
    Stat {
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "pos.size",
            value_parser = PossibleValuesParser::new(["pos.size", "funding.size", "credits.size", "credits.size.sym", "vol.1d", "vol.7d", "vol.30d", "vwap"]),
            help = "Stat type to return.",
        )]
        key: String,

        #[arg(
            long,
            default_value = "tBTCUSD",
            help = "Trading pair that is only applied to credits.size.sym key."
        )]
        side_pair: Option<String>,

        #[arg(
            long,
            default_value = "false",
            help = "Side for pos.size key. If not specified, default to Long."
        )]
        use_short: Option<bool>,

        #[arg(
            long,
            default_value = "10",
            value_parser = value_parser!(u16).range(1..=10000),
            help = "Limit for the number of records to return (max: 10000).",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the stats in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the stats in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    /// Get exchange rate for a specified currency pair.
    ExRate {
        /// Target currency to get the exchange rate for.
        from_ccy: String,
        /// Base currency to convert to
        to_ccy: String,
    }, // Exchange Rate

    /// All available pairs on Bitfinex.
    AvailPairs,

    /// All available currencies on Bitfinex.
    AvailCurrencies,

    /// Get the current status of the platform, “Operative” or “Maintenance”.
    PlatformStatus,

    /// Get derivatives pair status
    DerivStatus { keys: String },

    /// Get a list of the most recent funding data for the given currency.
    FundingStats {
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "10",
            value_parser = value_parser!(u16).range(1..=250),
            help = "Total number of records to return",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the stats in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the stats in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
}

/// Trading/exchange related utilities
#[derive(Subcommand)]
enum TradingAction {
    // --- Public Actions --- //
    /// Get aggregated book content
    Book {
        /// Symbol to get the order book for.
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "2",
            value_parser = value_parser!(u8).range(1..=4),
            help = "Decimal precision level of rates.",
        )]
        precision: u8,
    },
    /// Get raw book content
    RawBook { symbol: String },
    /// Get current tick of symbol
    Ticker { symbol: String },
    /// Get candles of symbol
    Candles {
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "30m",
            value_parser = PossibleValuesParser::new(["1m", "5m", "15m", "30m", "1h", "3h", "4h", "6h", "12h", "1d", "1w", "2w", "1M"]),
            help = "Time frame for the candles. Default is 30 minutes.",
        )]
        time_frame: Option<String>,

        #[arg(
            long,
            default_value = "20",
            value_parser = value_parser!(u16).range(1..=10000),
            help = "Number of candles to return (max 10000).",
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the candles in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the candles in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    /// Get public trades records
    Trades {
        /// Symbol to get trades for.
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "100",
            value_parser = value_parser!(u16).range(1..=10000),
            help = "Number of trades to return (max 10000).",
        )]
        limit: u16,

        #[arg(
            long,
            help = "Start time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the trades in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    // --- Authenticated Actions --- //
    /// Gets all the current user's active orders.
    Orders {
        #[arg(short, long, help = "Specify symbol for fetching orders")]
        symbol: Option<String>,

        #[arg(short, long, help = "Group ID of target orders")]
        group_id: Option<u64>,

        #[arg(
            short,
            long,
            help = "Client ID of target orders. If specified, --client-id-date is also required."
        )]
        client_id: Option<String>,

        #[arg(
            short = 'd',
            long,
            value_name = "YYYY-MM-DD",
            help = "Filter based on --client-id."
        )]
        client_id_date: Option<String>,
    },
    /// Retrieves all user's closed/cancelled orders up to 2 weeks in the past.
    HistOrders {
        #[arg(short, long, help = "Symbol to filter orders")]
        symbol: Option<String>,

        #[arg(
            short,
            long,
            default_value="10",
            value_parser = value_parser!(u16).range(1..=2500),
            help = "Number limit of orders"
        )]
        limit: Option<u16>,

        #[arg(
            long,
            help = "Start time for the orders in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        start: Option<DateTime<Local>>,

        #[arg(
            long,
            help = "End time for the orders in ISO 8601 format (e.g., 2025-01-01T00:00:00Z)."
        )]
        end: Option<DateTime<Local>>,
    },
    /// Submits an order on a trading pair (e.g. tBTCUSD, tLTCBTC, ...).
    Submit {
        symbol: String,

        #[arg(
            short,
            long,
            default_value = "exchange-limit",
            value_parser = PossibleValuesParser::new([
                "limit", "exchange-limit", "market", "exchange-market", "stop",
                "exchange-stop", "stop-limit", "exchange-stop-limit", "trailing-stop",
                "exchange-trailing-stop", "fok", "exchange-fok", "ioc", "exchange-ioc"
            ]),
            help = "Type order to submit",
        )]
        order_type: String,

        #[arg(
            short,
            long,
            required = true,
            help = "Amount of unit to submit (positive for buy, negative for sell)."
        )]
        amount: String,

        #[arg(short, long, required = true, help = "Price for each unit")]
        price: String,

        #[arg(
            long,
            default_value = "10",
            value_parser = value_parser!(u32).range(1..=100),
            help = "The leverage for a derivative order, supported by derivative symbol orders only."
        )]
        lev: Option<u32>,

        #[arg(long, help = "The trailing price for a trailing stop order.")]
        price_trailing: Option<String>,

        #[arg(long, help = "Auxiliary Limit price (only for STOP LIMIT).")]
        price_aux_limit: Option<String>,

        #[arg(long, help = "One-Cancels-Other stop price.")]
        price_oco_stop: Option<String>,

        #[arg(short, long, help = "Group ID for the order.")]
        gid: Option<u32>,

        #[arg(
            short,
            long,
            help = "Client Order ID; should be unique in the day (UTC+0)."
        )]
        cid: Option<u32>,

        #[arg(
            long,
            help = "The sum of all order flags. See: https://docs.bitfinex.com/docs/flag-values"
        )]
        flags: Option<u32>,

        #[arg(
            long,
            value_name = "YYYY-MM-DD hh:mm:ss",
            help = "Datetime for automatic order cancellation"
        )]
        time_in_force: Option<String>,
    },
    /// Updates an existing order, can be used to update margin, exchange, and derivative orders.
    Update {
        /// ID of the order.
        id: u64,

        #[arg(
            short,
            long,
            required = true,
            help = "Amount of unit to submit (positive for buy, negative for sell)."
        )]
        amount: Option<String>,

        #[arg(short, long, required = true, help = "Price for each unit")]
        price: Option<String>,

        #[arg(long, help = "The delta to apply to the amount value.")]
        delta: Option<String>,

        #[arg(
            long,
            default_value = "10",
            value_parser = value_parser!(u32).range(1..=100),
            help = "The leverage for a derivative order, supported by derivative symbol orders only."
        )]
        lev: Option<u32>,

        #[arg(long, help = "The trailing price for a trailing stop order.")]
        price_trailing: Option<String>,

        #[arg(long, help = "Auxiliary Limit price (only for STOP LIMIT).")]
        price_aux_limit: Option<String>,

        #[arg(short, long, help = "Group ID for the order.")]
        gid: Option<u32>,

        #[arg(
            short,
            long,
            help = "Client Order ID; should be unique in the day (UTC+0)."
        )]
        cid: Option<u64>,

        #[arg(long, value_name = "YYYY-MM-DD", help = "Date of Client Order ID.")]
        cid_date: Option<String>,

        #[arg(
            long,
            help = "The sum of all order flags. See: https://docs.bitfinex.com/docs/flag-values"
        )]
        flags: Option<u32>,

        #[arg(
            long,
            value_name = "YYYY-MM-DD hh:mm:ss",
            help = "Datetime for automatic order cancellation"
        )]
        time_in_force: Option<String>,
    },
    /// Cancels one of the current user's orders.
    Cancel {
        #[arg(short, long, help = "ID of the order.")]
        id: Option<u64>,

        #[arg(
            short,
            long,
            help = "Client Order ID; should be unique in the day (UTC+0)."
        )]
        cid: Option<u64>,

        #[arg(long, value_name = "YYYY-MM-DD", help = "Date of Client Order ID.")]
        cid_date: Option<String>,
    },
    /// Cancels all of the current user's orders, including derivative.
    CancelAll,
}

fn load_key() -> (String, String) {
    // Try to load from env var
    let api_key = std::env::var("API_KEY").ok();
    let api_secret = std::env::var("API_SECRET").ok();
    if api_key.is_some() && api_secret.is_some() {
        return (api_key.unwrap(), api_secret.unwrap());
    }

    // Load from .env file
    let env_path = resolve_env_path_or_create();
    dotenv::from_path(env_path).expect("Failed to load .env file");

    let api_key = std::env::var("API_KEY").unwrap();
    let api_secret = std::env::var("API_SECRET").unwrap();
    (api_key, api_secret)
}

fn get_client_with_key() -> Client {
    let (api_key, api_secret) = load_key();
    Client::new(api_key, api_secret)
}

fn get_client() -> Client {
    Client::new(String::new(), String::new())
}

pub async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Public { action } => {
            process_public_action(action).await;
        }
        Commands::Auth { action } => {
            process_auth_action(action).await;
        }
        Commands::Funding { action } => {
            process_funding_action(action).await;
        }
        Commands::Trading { action } => {
            process_trading_action(action).await;
        }
    }
}

async fn process_public_action(action: &PublicAction) {
    let client = get_client();
    match action {
        PublicAction::Stat {
            symbol,
            key,
            side_pair,
            use_short,
            limit,
            start,
            end,
        } => {
            let k = key.as_str();
            let stat = client
                .request_stat(
                    symbol,
                    k.into(),
                    side_pair.clone(),
                    *use_short,
                    *limit,
                    start.clone(),
                    end.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_public_stat(&stat);
        }
        PublicAction::ExRate { from_ccy, to_ccy } => {
            let rate = client
                .request_exchange_rate(from_ccy, to_ccy)
                .await
                .unwrap();
            pretty_print_json(&rate);
        }
        PublicAction::AvailPairs => {
            let pairs = client.request_avail_exchange_pairs().await.unwrap();
            pretty_print_json(&pairs);
        }
        PublicAction::AvailCurrencies => {
            let currencies = client.request_avail_ccy_list().await.unwrap();
            pretty_print_json(&currencies);
        }
        PublicAction::PlatformStatus => {
            let status = client.request_platform_status().await.unwrap();
            pretty_print_json(&status);
        }
        PublicAction::DerivStatus { keys } => {
            let status = client.request_deriv_status(keys).await.unwrap();
            pretty_print_json(&status);
        }
        PublicAction::FundingStats {
            symbol,
            limit,
            start,
            end,
        } => {
            let stats = client
                .request_funding_stats(symbol, limit.clone(), start.clone(), end.clone())
                .await
                .unwrap();
            pretty_print::print_funding_stats(&stats);
        }
    }
}

async fn process_auth_action(action: &AuthAction) {
    let client = get_client_with_key();
    match action {
        AuthAction::UserInfo => {
            let result = client.request_user_info().await.unwrap();
            pretty_print::print_user_info(&result);
        }
        AuthAction::Wallets => {
            let wallets = client.request_wallets().await.unwrap();
            pretty_print::print_wallet(&wallets);
        }
        AuthAction::KeyPermission => {
            let perm = client.request_key_permission().await.unwrap();
            pretty_print::print_key_permission(&perm);
        }
        AuthAction::Ledger {
            ccy,
            limit,
            category,
        } => {
            let cat = category.clone().unwrap();
            let result = client
                .request_ledger(ccy, *limit, Some(cat.as_str().into()))
                .await
                .unwrap();
            pretty_print::print_ledger(&result);
        }
        AuthAction::DepositAddress {
            wallet_type,
            method,
        } => {
            let addresses = get_client_with_key()
                .request_deposit_address(wallet_type.as_str().into(), method.as_str().into())
                .await
                .unwrap();
            pretty_print_json(&addresses);
        }
    }
}

async fn process_funding_action(action: &FundingAction) {
    match action {
        // --- Public actions --- //
        FundingAction::Book { symbol, precision } => {
            let book = get_client()
                .request_funding_book(symbol, (*precision).into())
                .await
                .unwrap();
            pretty_print::print_funding_book(&book);
        }
        FundingAction::RawBook { symbol } => {
            let book = get_client().request_funding_book_raw(symbol).await.unwrap();
            pretty_print::print_funding_book_raw(&book);
        }
        FundingAction::Ticker { symbol } => {
            let ticker = get_client().request_funding_ticker(symbol).await.unwrap();
            pretty_print_json(&ticker);
        }
        FundingAction::Candles {
            symbol,
            period,
            agg_period,
            time_frame,
            limit,
            start,
            end,
        } => {
            let agg_period = agg_period.as_ref().unwrap().parse::<u8>().unwrap();
            println!("Agg period: {}", agg_period);
            let time_frame = time_frame.as_ref().unwrap();
            let candles = get_client()
                .request_funding_candles(
                    symbol,
                    (*period).unwrap(),
                    agg_period.into(),
                    time_frame.as_str().into(),
                    *limit,
                    start.clone(),
                    end.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_candle(&candles);
        }
        FundingAction::Trades {
            symbol,
            limit,
            start,
            end,
        } => {
            let trades = get_client()
                .request_funding_trades(symbol, Some(*limit), start.clone(), end.clone())
                .await
                .unwrap();
            pretty_print::print_funding_trade(&trades);
        }
        // --- Authenticated actions --- //
        FundingAction::Submit {
            symbol,
            amount,
            rate,
            period,
            order_type,
        } => {
            let order_type = order_type.as_ref().unwrap().as_str();
            let result = get_client_with_key()
                .submit_funding_offer(symbol, *amount, *rate, *period, order_type.into())
                .await
                .unwrap();
            pretty_print_json(&result);
        }
        FundingAction::Cancel { id } => {
            let result = get_client_with_key()
                .cancel_funding_offer(*id)
                .await
                .unwrap();
            pretty_print_json(&result);
        }
        FundingAction::CancelAll { symbol } => {
            get_client_with_key().cancel_funding_offer_all(symbol).await;
            println!("Canceled all funding offers");
        }
        FundingAction::Offers { symbol } => {
            let offers = get_client_with_key()
                .request_funding_offers(symbol)
                .await
                .unwrap();
            pretty_print::print_funding_offer(&offers);
        }
        FundingAction::Credits { symbol } => {
            let credits = get_client_with_key()
                .request_funding_credits(symbol)
                .await
                .unwrap();
            pretty_print::print_funding_credits(&credits);
        }
        FundingAction::HistOffers {
            symbol,
            limit,
            start,
            end,
        } => {
            let offers = get_client_with_key()
                .request_funding_offers_hist(symbol, *limit, start.clone(), end.clone())
                .await
                .unwrap();
            pretty_print::print_funding_offer(&offers);
        }
        FundingAction::HistCredits {
            symbol,
            limit,
            start,
            end,
        } => {
            let credits = get_client_with_key()
                .request_funding_credits_hist(symbol, *limit, start.clone(), end.clone())
                .await
                .unwrap();
            pretty_print::print_funding_credits(&credits);
        }
    }
}

async fn process_trading_action(action: &TradingAction) {
    match action {
        // --- Public actions --- //
        TradingAction::Book { symbol, precision } => {
            let book = get_client()
                .request_trading_book(symbol, (*precision).into())
                .await
                .unwrap();
            pretty_print::print_trading_book(&book);
        }
        TradingAction::RawBook { symbol } => {
            let book = get_client().request_trading_book_raw(symbol).await.unwrap();
            pretty_print::print_trading_book_raw(&book);
        }
        TradingAction::Ticker { symbol } => {
            let ticker = get_client().request_trading_ticker(symbol).await.unwrap();
            pretty_print_json(&ticker);
        }
        TradingAction::Candles {
            symbol,
            time_frame,
            limit,
            start,
            end,
        } => {
            let time_frame = time_frame.as_ref().unwrap();
            let candles = get_client()
                .request_trading_candles(
                    symbol,
                    time_frame.as_str().into(),
                    *limit,
                    start.clone(),
                    end.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_candle(&candles);
        }
        TradingAction::Trades {
            symbol,
            limit,
            start,
            end,
        } => {
            let trades = get_client()
                .request_trading_trades(symbol, Some(*limit), start.clone(), end.clone())
                .await
                .unwrap();
            pretty_print::print_trading_trade(&trades);
        }
        TradingAction::Orders {
            symbol,
            group_id,
            client_id,
            client_id_date,
        } => {
            let orders = get_client_with_key()
                .request_trading_orders(
                    symbol.clone(),
                    *group_id,
                    client_id.clone(),
                    client_id_date.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_trading_order(&orders);
        }
        TradingAction::HistOrders {
            symbol,
            limit,
            start,
            end,
        } => {
            let orders = get_client_with_key()
                .request_trading_orders_hist(
                    symbol.clone(),
                    limit.clone(),
                    start.clone(),
                    end.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_trading_order(&orders);
        }
        TradingAction::Submit {
            symbol,
            order_type,
            amount,
            price,
            lev,
            price_trailing,
            price_aux_limit,
            price_oco_stop,
            gid,
            cid,
            flags,
            time_in_force,
        } => {
            let orders = get_client_with_key()
                .submit_trading_order(
                    symbol,
                    order_type.as_str().into(),
                    amount,
                    price,
                    lev.clone(),
                    price_trailing.clone(),
                    price_aux_limit.clone(),
                    price_oco_stop.clone(),
                    gid.clone(),
                    cid.clone(),
                    flags.clone(),
                    time_in_force.clone(),
                )
                .await
                .unwrap();
            pretty_print::print_trading_order(&orders);
        }
        TradingAction::Update {
            id,
            amount,
            price,
            delta,
            lev,
            price_trailing,
            price_aux_limit,
            gid,
            cid,
            cid_date,
            flags,
            time_in_force,
        } => {
            let order = get_client_with_key()
                .update_trading_order(
                    *id,
                    amount.clone(),
                    price.clone(),
                    delta.clone(),
                    lev.clone(),
                    price_trailing.clone(),
                    price_aux_limit.clone(),
                    gid.clone(),
                    cid.clone(),
                    cid_date.clone(),
                    flags.clone(),
                    time_in_force.clone(),
                )
                .await
                .unwrap();

            pretty_print_json(&order);
        }
        TradingAction::Cancel { id, cid, cid_date } => {
            let order = get_client_with_key()
                .cancel_trading_order(id.clone(), cid.clone(), cid_date.clone())
                .await
                .unwrap();

            pretty_print_json(&order);
        }
        TradingAction::CancelAll => {
            let orders = get_client_with_key()
                .cancel_trading_order_all()
                .await
                .unwrap();
            pretty_print::print_trading_order(&orders);
        }
    }
}

fn pretty_print_json<T: serde::Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

mod pretty_print {
    use crate::client::{
        FundingStats, KeyPermission, Permission, Ledger, Stat, Wallet, User,
    };
    use crate::funding::{
        Candle, FundingBook, FundingBookRaw, FundingCredit, FundingOffer, FundingTrade,
    };
    use crate::trading::{TradingBook, TradingBookRaw, TradingOrder, TradingTrade};
    use tabled::{builder::Builder, settings::Style};

    fn build_and_print(builder: Builder) {
        let mut table = builder.build();
        table.with(Style::rounded());
        println!("{}", table);
    }

    pub fn print_user_info(user: &User) {
        let mut builder = Builder::default();
        builder.push_record(["id".to_string(), user.id.to_string()]);
        builder.push_record(["email".to_string(), user.email.clone()]);
        builder.push_record(["email-verified".to_string(), user.email_verified.to_string()]);
        builder.push_record(["name".to_string(), user.name.clone()]);
        builder.push_record(["created".to_string(), user.created.to_rfc3339()]);
        builder.push_record(["verified".to_string(), user.verified.to_string()]);
        builder.push_record(["verification-level".to_string(), user.verification_level.to_string()]);
        builder.push_record(["timezone".to_string(), user.timezone.clone()]);
        builder.push_record(["locale".to_string(), user.locale.clone()]);
        builder.push_record(["company".to_string(), user.company.clone()]);
        builder.push_record(["subaccount-type", &user.subaccount_type.clone().map_or(String::new(), |v| v)]);
        builder.push_record(["master-account-created", &user.master_account_created.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["group-id", &user.group_id.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["master-account-id", &user.master_account_id.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["inherit-master-account-verification", &user.inherit_master_account_verification.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["is-group-master", &user.is_group_master.to_string()]);
        builder.push_record(["group-withdraw-enabled", &user.group_withdraw_enabled.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["ppt-enabled", &user.ppt_enabled.clone().map_or(String::new(), |v| v)]);
        builder.push_record(["merchant-enabled", &user.merchant_enabled.to_string()]);
        builder.push_record(["competition-enabled", &user.competition_enabled.clone().map_or(String::new(), |v| v)]);
        builder.push_record(["two-factor-modes", &serde_json::to_string_pretty(&user.two_factor_modes).unwrap()]);
        builder.push_record(["is-sercurities-master", &user.is_sercurities_master.to_string()]);
        builder.push_record(["securities-enabled", &user.securities_enabled.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["is-securities-investor-accredited", &user.is_securities_investor_accredited.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["is-securities-el-salvador", &user.is_securities_el_salvador.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["allow-disabled-ctxswitch", &user.allow_disable_ctxswitch.map_or(String::new(), |v| v.to_string())]);
        builder.push_record(["ctxswitch-disabled", &user.ctxswitch_disabled.to_string()]);
        builder.push_record(["last-login", &user.last_login.to_rfc3339()]);
        builder.push_record(["verification-level-submitted", &user.verification_level_submitted.to_string()]);
        builder.push_record(["comp-countries", &serde_json::to_string_pretty(&user.comp_countries).unwrap()]);
        builder.push_record(["comp-countries-resid", &serde_json::to_string_pretty(&user.comp_countries_resid).unwrap()]);
        builder.push_record(["compl-account-type", &user.compl_account_type.clone().map_or(String::new(), |v| v)]);
        builder.push_record(["is-merchant-enterprise", &user.is_merchant_enterprise.to_string()]);

        let mut table = builder.build();
        table.with(Style::modern());
        println!("{table}");
    }

    pub fn print_key_permission(perm: &KeyPermission) {
        let mut builder = Builder::default();
        let format_p = |p: &Permission| {
            format!("Read: {} / Write: {}", p.read, p.write)
        };
        builder.push_record(["account".to_string(), format_p(&perm.account)]);
        builder.push_record(["orders".to_string(), format_p(&perm.orders)]);
        builder.push_record(["funding".to_string(), format_p(&perm.funding)]);
        builder.push_record(["settings".to_string(), format_p(&perm.settings)]);
        builder.push_record(["wallets".to_string(), format_p(&perm.wallets)]);
        builder.push_record(["withdraw".to_string(), format_p(&perm.withdraw)]);
        builder.push_record(["history".to_string(), format_p(&perm.history)]);
        builder.push_record(["positions".to_string(), format_p(&perm.positions)]);
        builder.push_record(["ui_withdraw".to_string(), format_p(&perm.ui_withdraw)]);
        builder.push_record(["bfxpay".to_string(), format_p(&perm.bfxpay)]);

        let mut table = builder.build();
        table.with(Style::modern());
        println!("{table}");
    }

    pub fn print_wallet(wallets: &Vec<Wallet>) {
        let mut builder = Builder::default();
        builder.push_record(["ccy", "type", "free", "balance", "unsettled"]);
        for w in wallets {
            builder.push_record([
                w.ccy.clone(),
                w.typ.clone(),
                w.free.to_string(),
                w.balance.to_string(),
                w.unsettled_amount.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_ledger(ledgers: &Vec<Ledger>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "amount", "balance", "ccy", "time"]);
        for l in ledgers {
            builder.push_record([
                l.id.to_string(),
                l.amount.to_string(),
                l.balance.to_string(),
                l.ccy.clone(),
                l.time.to_rfc3339(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_trading_order(orders: &Vec<TradingOrder>) {
        let mut builder = Builder::default();
        builder.push_record([
            "id",
            "symbol",
            "price",
            "amount",
            "order-type",
            "status",
            "created",
            "updated",
        ]);
        for o in orders {
            builder.push_record([
                o.id.to_string(),
                o.symbol.clone(),
                o.price.to_string(),
                o.amount_orig.to_string(),
                o.order_type.to_string(),
                o.status.clone(),
                o.created.to_rfc3339(),
                o.updated.to_rfc3339(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_trading_trade(trades: &Vec<TradingTrade>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "time", "amount", "price"]);
        for t in trades {
            builder.push_record([
                t.id.to_string(),
                t.time.to_rfc3339(),
                t.amount.to_string(),
                t.price.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_trading_book(books: &Vec<TradingBook>) {
        let mut builder = Builder::default();
        builder.push_record(["price", "count", "amount"]);
        for b in books {
            builder.push_record([
                b.price.to_string(),
                b.count.to_string(),
                b.amount.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_trading_book_raw(books: &Vec<TradingBookRaw>) {
        let mut builder = Builder::default();
        builder.push_record(["order-id", "price", "amount"]);
        for b in books {
            builder.push_record([
                b.order_id.to_string(),
                b.price.to_string(),
                b.amount.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_credits(orders: &Vec<FundingCredit>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "amount", "rate", "period", "pair", "created"]);
        for o in orders {
            builder.push_record([
                o.id.to_string(),
                o.amount.to_string(),
                o.rate.to_string(),
                o.period.to_string(),
                o.pair.clone(),
                o.created.to_rfc3339(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_offer(orders: &Vec<FundingOffer>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "amount", "rate", "period", "status", "created"]);
        for o in orders {
            builder.push_record([
                o.id.to_string(),
                o.amount_ori.to_string(),
                o.rate.to_string(),
                o.period.to_string(),
                o.status.to_string(),
                o.created.to_rfc3339(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_candle(candles: &Vec<Candle>) {
        let mut builder = Builder::default();
        builder.push_record(["time", "open", "close", "high", "low", "volume"]);
        for c in candles {
            builder.push_record([
                c.time.to_rfc3339(),
                c.open.to_string(),
                c.close.to_string(),
                c.high.to_string(),
                c.low.to_string(),
                c.volume.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_trade(trades: &Vec<FundingTrade>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "amount", "rate", "period", "created"]);
        for t in trades {
            builder.push_record([
                t.id.to_string(),
                t.amount.to_string(),
                t.rate.to_string(),
                t.period.to_string(),
                t.created.to_rfc3339(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_book(books: &Vec<FundingBook>) {
        let mut builder = Builder::default();
        builder.push_record(["rate", "amount", "period", "count"]);
        for b in books {
            builder.push_record([
                b.rate.to_string(),
                b.amount.to_string(),
                b.period.to_string(),
                b.count.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_book_raw(books: &Vec<FundingBookRaw>) {
        let mut builder = Builder::default();
        builder.push_record(["id", "rate", "amount", "period"]);
        for b in books {
            builder.push_record([
                b.id.to_string(),
                b.rate.to_string(),
                b.amount.to_string(),
                b.period.to_string(),
            ]);
        }
        build_and_print(builder);
    }

    pub fn print_public_stat(stat: &Vec<Stat>) {
        let mut builder = Builder::default();
        builder.push_record(["time", "value"]);
        for s in stat {
            builder.push_record([s.time.to_rfc3339(), s.value.to_string()]);
        }
        build_and_print(builder);
    }

    pub fn print_funding_stats(stats: &Vec<FundingStats>) {
        let mut builder = Builder::default();
        builder.push_record([
            "time",
            "frr",
            "avg_preiod",
            "amount",
            "amount_used",
            "below_threshold",
        ]);
        for s in stats {
            builder.push_record([
                s.time.to_rfc3339(),
                s.frr.to_string(),
                s.avg_period.to_string(),
                s.funding_amount.to_string(),
                s.funding_amount_used.to_string(),
                s.funding_below_threshold.to_string(),
            ]);
        }
        build_and_print(builder);
    }
}
