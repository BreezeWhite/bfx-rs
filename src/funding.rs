use std::{
    cmp::max,
    convert::{From, Into},
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

use crate::{
    client::Client,
    deserializer::{from_mts, int_to_bool, to_mts},
    error::BitfinexError,
    utils::parse_ccy_from_symbol,
};

// --- Enums --- //
pub enum BookPrecision {
    One,
    Two,
    Three,
    Four,
}

impl From<u8> for BookPrecision {
    fn from(value: u8) -> Self {
        match value {
            1 => BookPrecision::One,
            2 => BookPrecision::Two,
            3 => BookPrecision::Three,
            4 => BookPrecision::Four,
            _ => BookPrecision::Two,
        }
    }
}

impl From<BookPrecision> for u8 {
    fn from(value: BookPrecision) -> Self {
        match value {
            BookPrecision::One => 1,
            BookPrecision::Two => 2,
            BookPrecision::Three => 3,
            BookPrecision::Four => 4,
        }
    }
}

pub enum FundingOrderType {
    Limit,
    FrrDeltaVar,
    FrrDeltaFix,
}

impl From<&str> for FundingOrderType {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "LIMIT" => FundingOrderType::Limit,
            "FRRDELTAVAR" => FundingOrderType::FrrDeltaVar,
            "FRRDELTAFIX" => FundingOrderType::FrrDeltaFix,
            _ => FundingOrderType::Limit,
        }
    }
}

impl From<String> for FundingOrderType {
    fn from(value: String) -> Self {
        FundingOrderType::from(value.as_str())
    }
}

impl std::fmt::Display for FundingOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FundingOrderType::Limit => write!(f, "LIMIT"),
            FundingOrderType::FrrDeltaFix => write!(f, "FRRDELTAFIX"),
            FundingOrderType::FrrDeltaVar => write!(f, "FRRDELTAVAR"),
        }
    }
}

#[derive(PartialEq)]
pub enum CandleAggPeriod {
    A10,
    A30,
    A120,
    Nil,
}

impl From<u8> for CandleAggPeriod {
    fn from(value: u8) -> Self {
        match value {
            10 => Self::A10,
            30 => Self::A30,
            120 => Self::A120,
            0 | _ => Self::Nil,
        }
    }
}

impl From<CandleAggPeriod> for u8 {
    fn from(value: CandleAggPeriod) -> Self {
        match value {
            CandleAggPeriod::A10 => 10,
            CandleAggPeriod::A30 => 30,
            CandleAggPeriod::A120 => 120,
            CandleAggPeriod::Nil => 0, // Special case for Nil
        }
    }
}

pub enum CandleTimeFrame {
    Min1,
    Min5,
    Min15,
    Min30,
    Hour1,
    Hour3,
    Hour4,
    Hour6,
    Hour12,
    Day1,
    Week1,
    Week2,
    Month1,
}

impl From<&str> for CandleTimeFrame {
    fn from(value: &str) -> Self {
        match value {
            "1m" => CandleTimeFrame::Min1,
            "5m" => CandleTimeFrame::Min5,
            "15m" => CandleTimeFrame::Min15,
            "30m" => CandleTimeFrame::Min30,
            "1h" => CandleTimeFrame::Hour1,
            "3h" => CandleTimeFrame::Hour3,
            "4h" => CandleTimeFrame::Hour4,
            "6h" => CandleTimeFrame::Hour6,
            "12h" => CandleTimeFrame::Hour12,
            "1d" => CandleTimeFrame::Day1,
            "1w" => CandleTimeFrame::Week1,
            "2w" => CandleTimeFrame::Week2,
            "1M" => CandleTimeFrame::Month1,
            _ => CandleTimeFrame::Hour4,
        }
    }
}

impl From<CandleTimeFrame> for String {
    fn from(value: CandleTimeFrame) -> Self {
        match value {
            CandleTimeFrame::Min1 => String::from("1m"),
            CandleTimeFrame::Min5 => String::from("5m"),
            CandleTimeFrame::Min15 => String::from("15m"),
            CandleTimeFrame::Min30 => String::from("30m"),
            CandleTimeFrame::Hour1 => String::from("1h"),
            CandleTimeFrame::Hour3 => String::from("3h"),
            CandleTimeFrame::Hour4 => String::from("4h"),
            CandleTimeFrame::Hour6 => String::from("6h"),
            CandleTimeFrame::Hour12 => String::from("12h"),
            CandleTimeFrame::Day1 => String::from("1d"),
            CandleTimeFrame::Week1 => String::from("1w"),
            CandleTimeFrame::Week2 => String::from("2w"),
            CandleTimeFrame::Month1 => String::from("1M"),
        }
    }
}

// --- Data Models --- //
#[derive(Serialize, Deserialize)]
pub struct Candle {
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FundingBook {
    pub rate: f64,
    pub period: u8,
    pub count: u16,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingTrade {
    pub id: u64,

    #[serde(deserialize_with = "from_mts", serialize_with = "to_mts")]
    pub created: DateTime<Local>,

    pub amount: f64,
    pub rate: f64,
    pub period: u8,
}

#[derive(Serialize, Deserialize)]
pub struct FundingBookRaw {
    pub id: u64,
    pub period: u8,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FundingTicker {
    pub frr: f64,
    pub bid: f64,
    pub bid_period: u8,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: u8,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub frr_amount_available: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FundingCredit {
    pub id: u64,
    pub symbol: String,
    pub side: i8, // 1 lender, 0 lender and borrower, -1 borrower

    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    #[serde(deserialize_with = "from_mts")]
    pub updated: DateTime<Local>,
    pub amount: f64,

    #[serde(skip_serializing)]
    _flags: Option<i8>,

    pub status: String,    // Active, Closed
    pub rate_type: String, // Fixed, Var

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub rate: f64,
    pub period: u8,

    #[serde(deserialize_with = "from_mts")]
    pub opened: DateTime<Local>,
    #[serde(deserialize_with = "from_mts")]
    pub last_payout: DateTime<Local>,
    pub notify: Option<bool>,
    #[serde(deserialize_with = "int_to_bool")]
    pub hidden: bool,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,

    #[serde(deserialize_with = "int_to_bool")]
    pub renew: bool,

    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    #[serde(deserialize_with = "int_to_bool")]
    pub no_close: bool,
    pub pair: String,
}

#[derive(Serialize, Deserialize)]
pub struct FundingOffer {
    pub id: u64,
    pub symbol: String,

    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    #[serde(deserialize_with = "from_mts")]
    pub updated: DateTime<Local>,

    pub amount: f64,
    pub amount_ori: f64,
    pub rate_type: String, // Fixed, Var

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _flags: Option<i8>,

    pub status: String, // ACTIVE

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,

    pub rate: f64,
    pub period: u8,

    pub notify: Option<u8>,
    pub hidden: Option<u8>,

    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,

    pub renew: Option<u8>,

    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FundingOfferResult {
    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    pub event_type: String,
    pub message_id: Option<u64>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub offer: FundingOffer,
    pub code: Option<u16>,
    pub status: String,
    pub message: Option<String>,
}

// --- Funding Functions --- //
impl Client {
    // --- Public Endpoints --- //
    /// 1. The returned amount > 0 is for ask, amount < 0 is for bid.
    /// 2. For `prec` level, from precise to less precise: 1 -> 4
    /// 
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-book#for-funding-currency-symbols-ex-fusd>
    pub async fn request_funding_book(
        &self,
        symbol: &str,
        prec: BookPrecision,
    ) -> Result<Vec<FundingBook>, BitfinexError> {
        if !symbol.starts_with("f") {
            panic!("You must specify funding symbol for funding book");
        }
        let prec = u8::from(prec);
        let url = format!("book/{symbol}/P{prec}?len=250");
        let body = self.get(&url).await?;
        let books: Vec<FundingBook> = from_str(&body).unwrap();
        Ok(books)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-book#for-funding-currency-symbols-ex-fusd-1>
    pub async fn request_funding_book_raw(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundingBookRaw>, BitfinexError> {
        if !symbol.starts_with("f") {
            panic!("You must specify funding symbol for funding book raw");
        }
        let url = format!("book/{symbol}/R0?len=250");
        let body = self.get(&url).await?;
        let books: Vec<FundingBookRaw> = from_str(&body).unwrap();
        Ok(books)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-trades#for-funding-currency-symbols-ex-fusd>
    pub async fn request_funding_trades(
        &self,
        symbol: &str,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<FundingTrade>, BitfinexError> {
        if !symbol.starts_with("f") {
            panic!("You must specify funding symbol for funding trades");
        }
        let mut url = format!("trades/{symbol}/hist?sort=-1");
        if let Some(limit) = limit {
            // max: 10000
            url = format!("{url}&limit={limit}");
        }
        if let Some(start) = start {
            url = format!("{url}&start={}", start.timestamp_millis());
        }
        if let Some(end) = end {
            url = format!("{url}&end={}", end.timestamp_millis());
        }
        let body = self.get(&url).await?;
        let trades: Vec<FundingTrade> = from_str(&body).unwrap();
        Ok(trades)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-tickers#for-funding-currency-symbols-ex-fusd>
    pub async fn request_funding_ticker(
        &self,
        symbol: &str,
    ) -> Result<FundingTicker, BitfinexError> {
        if !symbol.starts_with("f") {
            panic!("You must specify funding symbol for funding ticker");
        }
        let url = format!("ticker/{symbol}");
        let body = self.get(&url).await?;
        let ticker: FundingTicker = from_str(&body).unwrap();
        Ok(ticker)
    }

    /// ## Aggregation Rules:
    /// 1. `period` can only be multiply of `agg_period`
    /// For example, if `agg_period` is A10, then `period` could only be 10, 20, 30, ..., etc.
    /// 
    /// 2. Set `agg_period` to `Nil` to not aggregate.
    /// 3. Other than the above combinations, Bitfinex returns empty result.
    /// 
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-candles#funding-currency-candles>
    pub async fn request_funding_candles(
        &self,
        symbol: &str,
        period: u8,
        agg_period: CandleAggPeriod,
        time_frame: CandleTimeFrame,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<Candle>, BitfinexError> {
        let mut sub_query: Vec<String> = Vec::new();
        sub_query.push("trade".into());
        sub_query.push(time_frame.into());
        sub_query.push(symbol.into());

        if agg_period != CandleAggPeriod::Nil {
            // format: a10:p2:p30
            let agg_p = u8::from(agg_period);
            sub_query.push(format!("a{agg_p}"));
            let start_period = max(1, max(period, agg_p) - agg_p) + 1;
            sub_query.push(format!("p{start_period}"));
        }
        sub_query.push(format!("p{period}"));
        let sub_q = sub_query.join(":");

        let mut url = format!("candles/{sub_q}/hist?sort=-1");
        if let Some(limit) = limit {
            // max 10000
            url = format!("{url}&limit={limit}");
        }
        if let Some(start) = start {
            url = format!("{url}&start={}", start.timestamp_millis());
        }
        if let Some(end) = end {
            url = format!("{url}&end={}", end.timestamp_millis());
        }

        let body = self.get(&url).await?;
        let candles: Vec<Candle> = from_str(&body).unwrap();
        Ok(candles)
    }

    /// The default setup of candles in UI
    pub async fn request_funding_candles_default(
        &self,
        symbol: &str,
    ) -> Result<Vec<Candle>, BitfinexError> {
        // Wrapper of candles.
        self.request_funding_candles(symbol, 30, 30.into(), "30m".into(), None, None, None)
            .await
    }

    // --- Authenticated Endpoints --- //
    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-funding-credits>
    pub async fn request_funding_credits(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundingCredit>, BitfinexError> {
        let url = format!("auth/r/funding/credits/{symbol}");
        let body = self.post_url(&url).await?;
        let orders: Vec<FundingCredit> = from_str(&body).unwrap();
        Ok(orders)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-funding-credits-hist>
    pub async fn request_funding_credits_hist(
        &self,
        symbol: &str,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<FundingCredit>, BitfinexError> {
        let url = format!("auth/r/funding/credits/{symbol}/hist");
        let mut params = Vec::<(&str, String)>::new();
        if let Some(limit) = limit {
            // Max 500
            params.push(("limit", limit.to_string()));
        }
        if let Some(start) = start {
            params.push(("start", (start.timestamp_millis()).to_string()));
        }
        if let Some(end) = end {
            params.push(("end", (end.timestamp_millis()).to_string()));
        }
        let body = self.post_with_params(&url, params).await?;
        let credits: Vec<FundingCredit> = from_str(&body).unwrap();
        Ok(credits)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-funding-offers>
    pub async fn request_funding_offers(
        &self,
        symbol: &str,
    ) -> Result<Vec<FundingOffer>, BitfinexError> {
        let url = format!("auth/r/funding/offers/{symbol}");
        let body = self.post_url(&url).await?;
        let orders: Vec<FundingOffer> = from_str(&body).unwrap();
        Ok(orders)
    }

    // Ref: <https://docs.bitfinex.com/reference/rest-auth-funding-offers-hist>
    pub async fn request_funding_offers_hist(
        &self,
        symbol: &str,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<FundingOffer>, BitfinexError> {
        let url = format!("auth/r/funding/offers/{symbol}/hist");
        let mut params = Vec::<(&str, String)>::new();
        if let Some(limit) = limit {
            // Max 500
            params.push(("limit", limit.to_string()));
        }
        if let Some(start) = start {
            params.push(("start", (start.timestamp_millis()).to_string()));
        }
        if let Some(end) = end {
            params.push(("end", (end.timestamp_millis()).to_string()));
        }
        let body = self.post_with_params(&url, params).await?;
        let offers: Vec<FundingOffer> = from_str(&body).unwrap();
        Ok(offers)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-submit-funding-offer>
    pub async fn submit_funding_offer(
        &self,
        symbol: &str,
        amount: f64,
        rate: f64,
        period: u8,
        order_type: FundingOrderType,
    ) -> Result<FundingOffer, BitfinexError> {
        assert!(
            (2..=120).contains(&period),
            "Out of available period range: {period}"
        );
        let url = String::from("auth/w/funding/offer/submit");
        let payload = json!({
            "symbol": symbol,
            "amount": amount.to_string(),
            "rate": rate.to_string(),
            "period": period,
            "type": order_type.to_string(),
        });

        let body = self.post_with_payload(&url, payload.to_string()).await?;
        let resp: FundingOfferResult = from_str(&body).unwrap();
        Ok(resp.offer)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-cancel-funding-offer>
    pub async fn cancel_funding_offer(&self, offer_id: u64) -> Result<FundingOffer, BitfinexError> {
        let url = String::from("auth/w/funding/offer/cancel");
        let payload = json!({"id": offer_id}).to_string();
        let body = self.post_with_payload(&url, payload).await?;
        let resp: FundingOfferResult = from_str(&body).unwrap();
        Ok(resp.offer)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-cancel-all-funding-offers>
    pub async fn cancel_funding_offer_all(&self, symbol: &str) {
        let url = String::from("auth/w/funding/offer/cancel/all");
        let ccy = parse_ccy_from_symbol(symbol);
        let payload = json!({"currency": ccy}).to_string();
        let _ = self.post_with_payload(&url, payload).await;
    }
}
