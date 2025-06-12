use std::convert::{From, Into};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};

use crate::{
    client::Client,
    deserializer::from_mts,
    error::BitfinexError,
    funding::{BookPrecision, Candle, CandleTimeFrame},
};

// --- Trading Enums --- /
#[derive(Deserialize, Serialize)]
pub enum TradingOrderType {
    Limit,
    ExchangeLimit,
    Market,
    ExchangeMarket,
    Stop,
    ExchangeStop,
    StopLimit,
    ExchangeStopLimit,
    TrailingStop,
    ExchangeTrailingStop,
    Fok,
    ExchangeFok,
    Ioc,
    ExchangeIoc,
}

impl From<&str> for TradingOrderType {
    fn from(value: &str) -> Self {
        match value.to_uppercase().replace("-", "").as_str() {
            "LIMIT" => TradingOrderType::Limit,
            "EXCHANGE LIMIT" => TradingOrderType::ExchangeLimit,
            "MARKET" => TradingOrderType::Market,
            "EXCHANGE MARKET" => TradingOrderType::ExchangeMarket,
            "STOP" => TradingOrderType::Stop,
            "EXCHANGE STOP" => TradingOrderType::ExchangeStop,
            "STOP LIMIT" => TradingOrderType::StopLimit,
            "EXCHANGE STOP LIMIT" => TradingOrderType::ExchangeStopLimit,
            "TRAILING STOP" => TradingOrderType::TrailingStop,
            "EXCHANGE TRAILING STOP" => TradingOrderType::ExchangeTrailingStop,
            "FOK" => TradingOrderType::Fok,
            "EXCHANGE FOK" => TradingOrderType::ExchangeFok,
            "IOC" => TradingOrderType::Ioc,
            "EXCHANGE IOC" => TradingOrderType::ExchangeIoc,
            _ => TradingOrderType::Limit, // Default case
        }
    }
}

impl From<String> for TradingOrderType {
    fn from(value: String) -> Self {
        TradingOrderType::from(value.as_str())
    }
}

impl std::fmt::Display for TradingOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingOrderType::Limit => write!(f, "LIMIT"),
            TradingOrderType::ExchangeLimit => write!(f, "EXCHANGE LIMIT"),
            TradingOrderType::Market => write!(f, "MARKET"),
            TradingOrderType::ExchangeMarket => write!(f, "EXCHANGE MARKET"),
            TradingOrderType::Stop => write!(f, "STOP"),
            TradingOrderType::ExchangeStop => write!(f, "EXCHANGE STOP"),
            TradingOrderType::StopLimit => write!(f, "STOP LIMIT"),
            TradingOrderType::ExchangeStopLimit => write!(f, "EXCHANGE STOP LIMIT"),
            TradingOrderType::TrailingStop => write!(f, "TRAILING STOP"),
            TradingOrderType::ExchangeTrailingStop => write!(f, "EXCHANGE TRAILING STOP"),
            TradingOrderType::Fok => write!(f, "FOK"),
            TradingOrderType::ExchangeFok => write!(f, "EXCHANGE FOK"),
            TradingOrderType::Ioc => write!(f, "IOC"),
            TradingOrderType::ExchangeIoc => write!(f, "EXCHANGE IOC"),
        }
    }
}

// --- Trading Models --- //
#[derive(Serialize, Deserialize)]
pub struct TradingTicker {
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_relative: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TradingTickerHist {
    pub symbol: String,
    pub bid: f64,

    #[serde(skip_serializing)]
    _placeholder_0: Option<String>,

    pub ask: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,

    #[serde(deserialize_with = "from_mts")]
    time: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct TradingTrade {
    pub id: u64,
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,
    pub amount: f64,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TradingBook {
    pub price: f64,
    pub count: u32,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TradingBookRaw {
    pub order_id: u64,
    pub price: f64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TradingOrder {
    pub id: u64,
    pub group_id: Option<u64>,
    pub client_order_id: u64,
    pub symbol: String,
    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    #[serde(deserialize_with = "from_mts")]
    pub updated: DateTime<Local>,
    pub amount: f64,
    pub amount_orig: f64,
    pub order_type: TradingOrderType,
    pub type_prev: Option<TradingOrderType>,
    pub mts_time_in_force: Option<u64>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub flags: Option<u64>,
    pub status: String,

    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: f64,
    pub price_aux_limit: f64,

    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,

    pub notify: Option<u8>,
    pub hidden: Option<u8>,
    pub placed_id: Option<u64>,

    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,

    pub routing: String,

    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,

    pub meta: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TradingOrderMultiResult {
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,
    pub noti_type: String,
    pub message_id: Option<u64>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub orders: Vec<TradingOrder>,
    pub code: Option<u16>,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TradingOrderResult {
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,
    pub noti_type: String,
    pub message_id: Option<u64>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub order: TradingOrder,
    pub code: Option<u16>,
    pub status: String,
    pub message: Option<String>,
}

// --- Trading Functions --- //
impl Client {
    // --- Public Endpoints --- //
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-book#for-trading-pair-symbols-ex-tbtcusd>
    pub async fn request_trading_book(
        &self,
        symbol: &str,
        prec: BookPrecision,
    ) -> Result<Vec<TradingBook>, BitfinexError> {
        if !symbol.starts_with("t") {
            panic!("You must specify trading symbol for trading book");
        }
        let prec = u8::from(prec);
        let url = format!("book/{symbol}/P{prec}?len=250");
        let body = self.get(&url).await?;
        let books: Vec<TradingBook> = from_str(&body).unwrap();
        Ok(books)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-book#response-fields-raw-books>
    pub async fn request_trading_book_raw(
        &self,
        symbol: &str,
    ) -> Result<Vec<TradingBookRaw>, BitfinexError> {
        if !symbol.starts_with("t") {
            panic!("You must specify trading symbol for trading book raw");
        }
        let url = format!("book/{symbol}/R0?len=250");
        let body = self.get(&url).await?;
        let books: Vec<TradingBookRaw> = from_str(&body).unwrap();
        Ok(books)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-trades#for-trading-pair-symbols-ex-tbtcusd>
    pub async fn request_trading_trades(
        &self,
        symbol: &str,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<TradingTrade>, BitfinexError> {
        if !symbol.starts_with("t") {
            panic!("You must specify trading symbol for trading trades");
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
        let trades: Vec<TradingTrade> = from_str(&body).unwrap();
        Ok(trades)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-ticker#response-fields-trading-pairs-ex-tbtcusd>
    pub async fn request_trading_ticker(
        &self,
        symbol: &str,
    ) -> Result<TradingTicker, BitfinexError> {
        if !symbol.starts_with("t") {
            panic!("You must specify trading symbol for trading ticker");
        }
        let url = format!("ticker/{symbol}");
        let body = self.get(&url).await?;
        let ticker: TradingTicker = from_str(&body).unwrap();
        Ok(ticker)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-candles#trading-pair-candles>
    pub async fn request_trading_candles(
        &self,
        symbol: &str,
        time_frame: CandleTimeFrame,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<Candle>, BitfinexError> {
        if !symbol.starts_with("t") {
            panic!("You must specify trading pair for trading candles");
        }

        let time_frame: String = time_frame.into();
        let mut url = format!("candles/trade:{time_frame}:{symbol}/hist?sort=-1");
        if let Some(limit) = limit {
            // Max 10000
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

    // --- Authenticated Endpoints --- //
    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-retrieve-orders>
    pub async fn request_trading_orders(
        &self,
        symbol: Option<String>,
        group_id: Option<u64>,
        client_id: Option<String>,
        client_id_date: Option<String>, // YYYY-MM-DD format. Should be specified if client_id is provided
    ) -> Result<Vec<TradingOrder>, BitfinexError> {
        let mut url = format!("auth/r/orders");
        if let Some(sym) = symbol {
            url = format!("{url}/{sym}");
        }

        let mut data = json!({});
        if let Some(gid) = group_id {
            data["gid"] = Value::from(gid);
        }
        if let Some(cid) = client_id {
            data["cid"] = Value::from(cid);
            if client_id_date.is_none() {
                panic!("You must specify cid_date if cid is provided");
            }
            let cid_date = client_id_date.unwrap();
            data["cid_date"] = Value::from(cid_date);
        }
        let payload = data.to_string();

        let body = self.post_with_payload(&url, payload).await?;
        let orders: Vec<TradingOrder> = from_str(&body).unwrap();
        Ok(orders)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-submit-order>
    pub async fn submit_trading_order(
        &self,
        symbol: &str,
        order_type: TradingOrderType,
        amount: &str,
        price: &str,
        lev: Option<u32>,
        price_trailing: Option<String>,  // Only for trailing stop
        price_aux_limit: Option<String>, // Only for stop limit
        price_oco_stop: Option<String>,  // Only for stop
        gid: Option<u32>,                // Group ID
        cid: Option<u32>,                // Client Order ID
        flags: Option<u32>,              // The sum of all order flags
        time_in_force: Option<String>,   // 2020-01-15 10:45:23
    ) -> Result<Vec<TradingOrder>, BitfinexError> {
        let url = String::from("auth/w/order/submit");

        let mut data = json!({
            "symbol": symbol,
            "type": order_type.to_string(),
            "amount": amount,
            "price": price,
        });

        if let Some(lev) = lev {
            data["lev"] = Value::from(lev);
        }
        if let Some(price_trailing) = price_trailing {
            data["price_trailing"] = Value::from(price_trailing);
        }
        if let Some(price_aux_limit) = price_aux_limit {
            data["price_aux_limit"] = Value::from(price_aux_limit);
        }
        if let Some(price_oco_stop) = price_oco_stop {
            data["price_oco_stop"] = Value::from(price_oco_stop);
        }
        if let Some(gid) = gid {
            data["gid"] = Value::from(gid);
        }
        if let Some(cid) = cid {
            data["cid"] = Value::from(cid);
        }
        if let Some(flags) = flags {
            data["flags"] = Value::from(flags);
        }
        if let Some(tif) = time_in_force {
            data["tif"] = Value::from(tif);
        }
        let payload = data.to_string();

        let body = self.post_with_payload(&url, payload).await;
        let result: TradingOrderMultiResult = match body {
            Ok(b) => from_str(&b).unwrap(),
            Err(e) => return Err(e),
        };
        Ok(result.orders)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-update-order>
    pub async fn update_trading_order(
        &self,
        id: u64,
        amount: Option<String>,
        price: Option<String>,
        delta: Option<String>, // The delta to apply to the amount value.
        lev: Option<u32>, // Set the leverage for a derivative order, supported by derivative symbol orders only.
        price_trailing: Option<String>, // Only for trailing stop
        price_aux_limit: Option<String>, // Only for stop limit
        gid: Option<u32>, // Group ID
        cid: Option<u64>, // Client ID
        cid_date: Option<String>, // YYYY-MM-DD format
        flags: Option<u32>, // The sum of all order flags
        time_in_force: Option<String>, // 2020-01-15 10:45:23
    ) -> Result<TradingOrder, BitfinexError> {
        let url = String::from("auth/w/order/submit");

        let mut data = json!({
            "id": id,
        });

        if let Some(amount) = amount {
            data["amount"] = Value::from(amount);
        }
        if let Some(price) = price {
            data["price"] = Value::from(price);
        }
        if let Some(delta) = delta {
            data["delta"] = Value::from(delta);
        }
        if let Some(lev) = lev {
            data["lev"] = Value::from(lev);
        }
        if let Some(price_trailing) = price_trailing {
            data["price_trailing"] = Value::from(price_trailing);
        }
        if let Some(price_aux_limit) = price_aux_limit {
            data["price_aux_limit"] = Value::from(price_aux_limit);
        }
        if let Some(gid) = gid {
            data["gid"] = Value::from(gid);
        }
        if let Some(cid) = cid {
            data["cid"] = Value::from(cid);
        }
        if let Some(cid_date) = cid_date {
            data["cid_date"] = Value::from(cid_date);
        }
        if let Some(flags) = flags {
            data["flags"] = Value::from(flags);
        }
        if let Some(tif) = time_in_force {
            data["tif"] = Value::from(tif);
        }
        let payload = data.to_string();

        let body = self.post_with_payload(&url, payload).await?;
        let result: TradingOrderResult = from_str(&body).unwrap();
        Ok(result.order)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-cancel-order>
    pub async fn cancel_trading_order(
        &self,
        id: Option<u64>,
        cid: Option<u64>,
        cid_date: Option<String>, // YYYY-MM-DD format, should be specified if cid is provided
    ) -> Result<TradingOrder, BitfinexError> {
        if id.is_none() && cid.is_none() {
            panic!("You must specify either id or cid to cancel trading order");
        }
        let url = String::from("auth/w/order/cancel");

        let mut data = json!({});
        if let Some(id) = id {
            data["id"] = Value::from(id);
        }
        if let Some(cid) = cid {
            data["cid"] = Value::from(cid);
            if cid_date.is_none() {
                panic!("You must specify cid_date if cid is provided");
            }
            let cid_date = cid_date.unwrap();
            data["cid_date"] = Value::from(cid_date);
        }
        let payload = data.to_string();

        let body = self.post_with_payload(&url, payload).await?;
        let result: TradingOrderResult = from_str(&body).unwrap();
        Ok(result.order)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-cancel-orders-multiple>
    pub async fn cancel_trading_order_all(&self) -> Result<Vec<TradingOrder>, BitfinexError> {
        let url = String::from("auth/w/order/cancel/multi");
        let payload = json!({"all": 1}).to_string();
        let body = self.post_with_payload(&url, payload).await?;
        let result: TradingOrderMultiResult = from_str(&body).unwrap();
        Ok(result.orders)
    }

    /// Ref:
    /// - <https://docs.bitfinex.com/reference/rest-auth-orders-history>
    /// - <https://docs.bitfinex.com/reference/rest-auth-orders-history-by-symbol>
    pub async fn request_trading_orders_hist(
        &self,
        symbol: Option<String>,
        limit: Option<u16>,
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<TradingOrder>, BitfinexError> {
        let mut url = String::from("auth/r/orders");

        if let Some(sym) = symbol {
            url = format!("{url}/{sym}");
        }
        url = format!("{url}/hist");

        let mut data = json!({});
        if let Some(limit) = limit {
            // Max 2500
            data["limit"] = Value::from(limit);
        }
        if let Some(start) = start {
            data["start"] = Value::from(start.timestamp_millis());
        }
        if let Some(end) = end {
            data["end"] = Value::from(end.timestamp_millis());
        }

        let payload = data.to_string();
        let body = self.post_with_payload(&url, payload).await?;
        let orders: Vec<TradingOrder> = from_str(&body).unwrap();
        Ok(orders)
    }
}
