use core::fmt;
use std::{
    convert::{From, Into},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Local};
use hex::encode;
use reqwest::{
    self,
    header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};
use ring::hmac;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, json};

use crate::{
    deserializer::{from_mts, int_to_bool},
    error::BitfinexError,
};

static BITFINEX_PUB_HOST: &str = "https://api-pub.bitfinex.com/v2";
static BITFINEX_AUTH_HOST: &str = "https://api.bitfinex.com/v2";

fn parse_error(body: &str) -> Option<(String, String)> {
    // Looks for: "error",<code>,"<message>"
    let prefix = r#""error","#;
    if let Some(start) = body.find(prefix) {
        let rest = &body[start + prefix.len()..];
        let mut parts = rest
            .splitn(3, ',')
            .map(|s| s.trim_matches(|c| c == '"' || c == ' '));
        let code = parts.next()?.to_string();
        let message = parts
            .next()?
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();
        Some((code, message))
    } else {
        None
    }
}

// --- Data Models --- //
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub typ: String,
    pub ccy: String,
    pub balance: f64,
    pub unsettled_amount: f64,
    pub free: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ledger {
    pub id: u64,
    pub ccy: String,
    pub wallet: String,
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub amount: f64,
    pub balance: f64,

    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub name: String,

    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    #[serde(deserialize_with = "int_to_bool")]
    pub verified: bool,
    pub verification_level: u8,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub timezone: String,
    pub locale: String,
    pub company: String,

    #[serde(deserialize_with = "int_to_bool")]
    pub email_verified: bool,

    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub subaccount_type: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,

    pub master_account_created: Option<u64>,
    pub group_id: Option<u16>,
    pub master_account_id: Option<u32>,

    pub inherit_master_account_verification: Option<u8>,
    #[serde(deserialize_with = "int_to_bool")]
    pub is_group_master: bool,
    pub group_withdraw_enabled: Option<u8>,

    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub ppt_enabled: Option<String>,
    #[serde(deserialize_with = "int_to_bool")]
    pub merchant_enabled: bool,
    pub competition_enabled: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,

    pub two_factor_modes: Vec<String>,

    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,

    #[serde(deserialize_with = "int_to_bool")]
    pub is_sercurities_master: bool,
    pub securities_enabled: Option<u8>,
    pub is_securities_investor_accredited: Option<u8>,
    pub is_securities_el_salvador: Option<u8>,

    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<u8>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_12: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_13: Option<String>,

    pub allow_disable_ctxswitch: Option<u8>,
    #[serde(deserialize_with = "int_to_bool")]
    pub ctxswitch_disabled: bool,

    #[serde(skip_serializing)]
    _placeholder_14: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_15: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_16: Option<u8>,
    #[serde(skip_serializing)]
    _placeholder_17: Option<String>,

    pub last_login: DateTime<Local>,

    #[serde(skip_serializing)]
    _placeholder_18: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_19: Option<String>,

    pub verification_level_submitted: u8,

    #[serde(skip_serializing)]
    _placeholder_20: Option<String>,

    pub comp_countries: Vec<String>,
    pub comp_countries_resid: Vec<String>,
    pub compl_account_type: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_21: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_22: Option<String>,

    #[serde(deserialize_with = "int_to_bool")]
    pub is_merchant_enterprise: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    #[serde(deserialize_with = "int_to_bool")]
    pub read: bool,
    #[serde(deserialize_with = "int_to_bool")]
    pub write: bool,
}

#[derive(Serialize, Deserialize)]
pub struct KeyPermission {
    pub account: Permission,
    pub orders: Permission,
    pub funding: Permission,
    pub settings: Permission,
    pub wallets: Permission,
    pub withdraw: Permission,
    pub history: Permission,
    pub positions: Permission,
    pub ui_withdraw: Permission,
    pub bfxpay: Permission,
    pub eaas_agreement: Permission,
    pub eaas_withdraw: Permission,
    pub eaas_deposit: Permission,
    pub eaas_brokerage: Permission,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAddress {
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub method: String,
    pub ccy: String,

    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub address: String,
    pub pool_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAddressResult {
    #[serde(deserialize_with = "from_mts")]
    pub created: DateTime<Local>,
    pub noti_type: String,
    pub message_id: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub addresses: Vec<DepositAddress>,

    pub code: Option<u8>,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PlatformStatus {
    #[serde(deserialize_with = "int_to_bool")]
    pub status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FundingStats {
    #[serde[deserialize_with = "from_mts"]]
    pub time: DateTime<Local>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub frr: f64,
    pub avg_period: f64,
    
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub funding_amount: f64,
    pub funding_amount_used: f64,
    
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,

    pub funding_below_threshold: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DerivativesStatus {
    pub key: String,
    #[serde(deserialize_with = "from_mts")]
    pub time: DateTime<Local>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,

    pub deriv_price: f64,
    pub spot_price: f64,
    
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub insurance_fund_balance: f64,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,

    #[serde(deserialize_with = "from_mts")]
    pub next_funding_evt_time: DateTime<Local>,
    pub next_funding_accrued: f64,
    pub next_funding_step: u64,
    
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub current_funding: f64,
    
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,

    pub mark_price: f64,

    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,

    pub open_interest: f64,
    
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,

    pub clamp_min: f64,
    pub clamp_max: f64,
}

// --- Enums --- //
pub enum LedgerType {
    Exchange = 5,
    Interest = 28,
    Transfer = 51,
    TradingFee = 201,
}

impl From<&str> for LedgerType {
    fn from(value: &str) -> Self {
        match value {
            "Exchange" => LedgerType::Exchange,
            "Interest" => LedgerType::Interest,
            "Transfer" => LedgerType::Transfer,
            "TradingFee" => LedgerType::TradingFee,
            _ => LedgerType::Interest,
        }
    }
}

impl From<LedgerType> for u8 {
    fn from(value: LedgerType) -> Self {
        match value {
            LedgerType::Exchange => 5,
            LedgerType::Interest => 28,
            LedgerType::Transfer => 51,
            LedgerType::TradingFee => 201,
        }
    }
}

#[derive(PartialEq)]
pub enum StatKey {
    PosSize,        // Total longs/shorts in base currency (i.e. BTC for tBTCUSD)
    FundingSize,    // Total active funding in specified CCY
    CreditsSize,    // Total funding used in positions in specified CCY
    CreditsSizeSym, // Total funding used in positions on a specific pair in specified CCY
    Vol1d,          // Total trading volume for specified time period (1d/7d/30d)
    Vol7d,
    Vol30d,
    Vwap, // Volume weighted average price for the day
}

impl From<&str> for StatKey {
    fn from(value: &str) -> Self {
        match value {
            "pos.size" => StatKey::PosSize,
            "funding.size" => StatKey::FundingSize,
            "credits.size" => StatKey::CreditsSize,
            "credits.size.sym" => StatKey::CreditsSizeSym,
            "vol.1d" => StatKey::Vol1d,
            "vol.7d" => StatKey::Vol7d,
            "vol.30d" => StatKey::Vol30d,
            "vwap" => StatKey::Vwap,
            _ => StatKey::PosSize, // Default case
        }
    }
}

impl StatKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatKey::PosSize => "pos.size",
            StatKey::FundingSize => "funding.size",
            StatKey::CreditsSize => "credits.size",
            StatKey::CreditsSizeSym => "credits.size.sym",
            StatKey::Vol1d => "vol.1d",
            StatKey::Vol7d => "vol.7d",
            StatKey::Vol30d => "vol.30d",
            StatKey::Vwap => "vwap",
        }
    }
}

pub enum WalletType {
    Exchange,
    Margin,
    Funding,
}

impl From<&str> for WalletType {
    fn from(value: &str) -> Self {
        match value {
            "exchange" => WalletType::Exchange,
            "margin" => WalletType::Margin,
            "funding" => WalletType::Funding,
            _ => WalletType::Funding,
        }
    }
}

impl WalletType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletType::Exchange => "exchange",
            WalletType::Margin => "margin",
            WalletType::Funding => "funding",
        }
    }
}

pub enum DepositMethod {
    Bitcoin,
    Litecoin,
    Ethereum,
    Tetheruso,
    Tetherusl,
    Tetherusx,
    Tetheruss,
    Ethereumc,
    Zcash,
    Monero,
    Iota,
}

impl From<&str> for DepositMethod {
    fn from(value: &str) -> Self {
        match value {
            "bitcoin" => DepositMethod::Bitcoin,
            "litecoin" => DepositMethod::Litecoin,
            "ethereum" => DepositMethod::Ethereum,
            "tetheruso" => DepositMethod::Tetheruso,
            "tetherusl" => DepositMethod::Tetherusl,
            "tetherusx" => DepositMethod::Tetherusx,
            "tetheruss" => DepositMethod::Tetheruss,
            "ethereumc" => DepositMethod::Ethereumc,
            "zcash" => DepositMethod::Zcash,
            "monero" => DepositMethod::Monero,
            "iota" => DepositMethod::Iota,
            _ => panic!("Unknown deposit method: {}", value),
        }
    }
}

impl fmt::Display for DepositMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            DepositMethod::Bitcoin => "bitcoin",
            DepositMethod::Litecoin => "litecoin",
            DepositMethod::Ethereum => "ethereum",
            DepositMethod::Tetheruso => "tetheruso",
            DepositMethod::Tetherusl => "tetherusl",
            DepositMethod::Tetherusx => "tetherusx",
            DepositMethod::Tetheruss => "tetheruss",
            DepositMethod::Ethereumc => "ethereumc",
            DepositMethod::Zcash => "zcash",
            DepositMethod::Monero => "monero",
            DepositMethod::Iota => "iota",
        };
        write!(f, "{}", method_str)
    }
}

// --- Bitfinex Client --- //
pub struct Client {
    api_key: String,
    api_secret: String,
}

impl Client {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Client {
            api_key,
            api_secret,
        }
    }

    // Inner utility functions
    fn sign_payload(&self, secret: &[u8], payload: &[u8]) -> String {
        let signed_key = hmac::Key::new(hmac::HMAC_SHA384, secret);
        let signature = encode(hmac::sign(&signed_key, payload).as_ref());

        signature
    }

    fn generate_nonce(&self) -> String {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let timestamp = since_epoch.as_secs() * 1_000_000;
        timestamp.to_string()
    }

    fn build_headers(&self, url: &String, payload: Option<String>) -> HeaderMap {
        let nonce = self.generate_nonce();
        let payload = match payload {
            Some(p) => p,
            None => "".to_string(),
        };
        let signature_path = format!("/api/v2/{}{}{}", url, nonce, payload);

        let signature = self.sign_payload(self.api_secret.as_bytes(), signature_path.as_bytes());

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-api-rs"));
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(nonce.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("bfx-apikey"),
            HeaderValue::from_str(self.api_key.as_str()).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("bfx-signature"),
            HeaderValue::from_str(signature.as_str()).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers
    }

    fn handle_error(&self, body: &String) -> Result<(), BitfinexError> {
        if let Some((err_code, err_msg)) = parse_error(body) {
            match err_code.as_str() {
                "10001" => {
                    // Generic error code
                    if err_msg.contains("Limit: too many active offers") {
                        return Err(BitfinexError::ExceedMaxOfferCount);
                    }
                    // "error",10001,"Invalid offer: incorrect amount, minimum is 150.0 dollar or equivalent in UST"
                    // "error",10001,"FRR offset larger than 30% of FRR, aborting."
                    return Err(BitfinexError::BitfinexGenericError(err_msg));
                }
                "10020" => {
                    // "error",10020,"currency: invalid"
                    // "error",10020,"time_interval: invalid"
                    return Err(BitfinexError::InvalidCurrency);
                }
                "10100" => {
                    // "error",10100,"apikey: digest invalid"
                    return Err(BitfinexError::InvalidKeyDigest);
                }
                "10114" => {
                    // "error",10114,"nonce: small"
                    return Err(BitfinexError::NonceSmall);
                }
                "11000" => {
                    // "error",11000,"ready: invalid"
                    return Err(BitfinexError::BitfinexTempUnavailable);
                }
                "11010" => {
                    // "error", 11010, "ratelimit: error"
                    return Err(BitfinexError::RateLimited);
                }
                _ => {
                    // Bitfinex Generic Error
                    return Err(BitfinexError::BitfinexGenericError(err_msg));
                }
            }
        }
        Ok(())
    }

    // General public functions
    pub async fn get(&self, url: &String) -> Result<String, BitfinexError> {
        let endpoint = format!("{BITFINEX_PUB_HOST}/{url}");

        let retry_cnt: u8 = 5;
        let retry_interval = 1;
        for _ in 0..=retry_cnt {
            let response = reqwest::get(&endpoint).await;
            if let Ok(resp) = response {
                let body = resp.text().await.unwrap();
                match self.handle_error(&body) {
                    Err(BitfinexError::NonceSmall) => {
                        println!("Catched NonceSmall error. Retrying..");
                        tokio::time::sleep(Duration::from_secs(retry_interval)).await;
                        continue;
                    }
                    Err(err) => {
                        eprintln!("Error occured: {err:#?}");
                        return Err(err);
                    }
                    Ok(_) => return Ok(body),
                }
            } else {
                println!("Bad response: {response:#?}");
                tokio::time::sleep(Duration::from_secs(retry_interval)).await;
            }
        }
        Err(BitfinexError::BitfinexGenericError(
            "Exceed max retry count".into(),
        ))
    }

    pub async fn post(
        &self,
        url: &String,
        payload: Option<String>,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<String, BitfinexError> {
        let endpoint = format!("{BITFINEX_AUTH_HOST}/{url}");

        let client = reqwest::Client::new();
        let retry_cnt: u8 = 5;
        let retry_interval = 1;
        for _ in 0..=retry_cnt {
            let mut builder = client
                .post(&endpoint)
                .headers(self.build_headers(url, payload.clone()));
            if let Some(ref payload) = payload {
                builder = builder.body(payload.clone());
            }
            if let Some(ref params) = params {
                builder = builder.query(params);
            }
            let response = builder.send().await;

            if let Ok(resp) = response {
                let body: String = resp.text().await.unwrap();
                match self.handle_error(&body) {
                    Err(BitfinexError::NonceSmall) => {
                        println!("Catched NonceSmall error. Retrying..");
                        tokio::time::sleep(Duration::from_secs(retry_interval)).await;
                        continue;
                    }
                    Err(err) => {
                        eprintln!("Error occured: {err:#?}");
                        return Err(err);
                    }
                    Ok(_) => return Ok(body),
                }
            } else {
                eprintln!("Bad response: {response:#?}");
                tokio::time::sleep(Duration::from_secs(retry_interval)).await;
            }
        }
        Err(BitfinexError::BitfinexGenericError(
            "Exceed max retry count".into(),
        ))
    }

    pub async fn post_url(&self, url: &String) -> Result<String, BitfinexError> {
        self.post(url, None, None).await
    }

    pub async fn post_with_payload(
        &self,
        url: &String,
        payload: String,
    ) -> Result<String, BitfinexError> {
        self.post(url, Some(payload), None).await
    }

    pub async fn post_with_params(
        &self,
        url: &String,
        params: Vec<(&str, String)>,
    ) -> Result<String, BitfinexError> {
        self.post(url, None, Some(params)).await
    }

    // --- Public APIs --- //
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-foreign-exchange-rate>
    pub async fn request_exchange_rate(
        &self,
        ccy: &str,
        to_ccy: &str,
    ) -> Result<f64, BitfinexError> {
        let url = String::from("calc/fx");
        let payload = json!({"ccy1": ccy, "ccy2": to_ccy}).to_string();
        let res = self.post_with_payload(&url, payload).await?;
        let res: Vec<f64> = from_str(&res).unwrap();
        Ok(res[0])
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-conf>
    pub async fn request_avail_exchange_pairs(&self) -> Result<Vec<String>, BitfinexError> {
        let body = self
            .get(&String::from("conf/pub:list:pair:exchange"))
            .await?;
        let res: Vec<Vec<String>> = from_str(&body).unwrap();
        Ok(res[0].to_owned())
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-conf>
    pub async fn request_avail_ccy_list(&self) -> Result<Vec<String>, BitfinexError> {
        let body = self.get(&String::from("conf/pub:list:currency")).await?;
        let res: Vec<Vec<String>> = from_str(&body).unwrap();
        Ok(res[0].to_owned())
    }

    /// 1. `side_pair` is only available for key `credits.size.sym`.
    /// 2. `use_short` is only available for key `pos.size`.
    /// 3. For key `pos.size`, defaults to use long.
    /// 4. `limit` is up to 10000.
    /// 
    /// **Funding-only keys**
    /// funding.size / credits.size / credits.size.sym
    /// **Trading-only keys**
    /// pos.size
    /// 
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-stats>
    pub async fn request_stat(
        &self,
        symbol: &str,
        key: StatKey,
        side_pair: Option<String>, // Only for credits.size.sym. Default to tBTCUSD
        use_short: Option<bool>,   // Only for pos.size
        limit: Option<u16>, // Max 10000
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<Stat>, BitfinexError> {
        let k = key.as_str();
        let mut url = format!("stats1/{k}");

        if [StatKey::FundingSize, StatKey::CreditsSize].contains(&key) {
            if !symbol.starts_with("f") {
                panic!("You must specify funding symbol for {k} stat");
            }
            url = format!("{url}:1m:{symbol}");
        } else if key == StatKey::CreditsSizeSym {
            if !symbol.starts_with("f") {
                panic!("You must specify funding symbol for {k} stat");
            }

            let side_pair = match side_pair {
                Some(v) => v,
                None => {
                    println!(
                        "Querying credits.size.sym without specifying side_pair. Defaulting to tBTCUSD"
                    );
                    String::from("tBTCUSD")
                }
            };
            url = format!("{url}:1m:{symbol}:{side_pair}");
        } else if key == StatKey::PosSize {
            if !symbol.starts_with("t") {
                panic!("You must specify trading pair for pos.size stat");
            }
            let side_pair = match use_short {
                Some(true) => "short",
                Some(false) => "long",
                None => "long",
            };
            url = format!("{url}:1m:{symbol}:{side_pair}");
        } else if key == StatKey::Vwap {
            url = format!("{url}:1d:{symbol}");
        } else {
            url = format!("{url}:30m:BFX");
        }

        url = format!("{url}/hist?sort=-1");

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
        let stats: Vec<Stat> = from_str(&body).unwrap();
        Ok(stats)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-public-platform-status>
    pub async fn request_platform_status(&self) -> Result<PlatformStatus, BitfinexError> {
        let body = self.get(&String::from("platform/status")).await?;
        let res: PlatformStatus = from_str(&body).unwrap();
        Ok(res)
    }

    /// ## Parameters:
    /// - `limit` is up to 250
    /// 
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-funding-stats>
    pub async fn request_funding_stats(
        &self,
        symbol: &str,
        limit: Option<u16>, // Max 250
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
    ) -> Result<Vec<FundingStats>, BitfinexError> {
        let mut url = format!("funding/stats/{symbol}/hist?");

        if let Some(limit) = limit {
            // max 250
            url = format!("{url}&limit={limit}");
        }
        if let Some(start) = start {
            url = format!("{url}&start={}", start.timestamp_millis());
        }
        if let Some(end) = end {
            url = format!("{url}&end={}", end.timestamp_millis());
        }

        let body = self.get(&url).await?;
        let stats: Vec<FundingStats> = from_str(&body).unwrap();
        Ok(stats)
    }

    /// ## Parameters:
    /// - `keys`: comma seprated pairs (e.g. tBTCF0:USTF0,tETHF0:USTF0). 'ALL' for all pairs.
    /// 
    /// Ref: <https://docs.bitfinex.com/reference/rest-public-derivatives-status>
    pub async fn request_deriv_status(&self, keys: &str) -> Result<Vec<DerivativesStatus>, BitfinexError> {
        let url = format!("status/deriv?keys={keys}");
        let body = self.get(&url).await?;
        let sts: Vec<DerivativesStatus> = from_str(&body).unwrap();
        Ok(sts)
    }

    // --- Authenticated APIs --- //
    // User-related API
    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-info-user>
    pub async fn request_user_info(&self) -> Result<User, BitfinexError> {
        let body = self.post_url(&String::from("auth/r/info/user")).await?;
        let user: User = from_str(&body).unwrap();
        Ok(user)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-wallets>
    pub async fn request_wallets(&self) -> Result<Vec<Wallet>, BitfinexError> {
        let body = self.post_url(&String::from("auth/r/wallets")).await?;
        let wallets: Vec<Wallet> = from_str(&body).unwrap();
        Ok(wallets)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-ledgers>
    pub async fn request_ledger(
        &self,
        ccy: &str,
        limit: Option<u16>,
        category: Option<LedgerType>,
    ) -> Result<Vec<Ledger>, BitfinexError> {
        let url = format!("auth/r/ledgers/{ccy}/hist");
        let cat: u8 = match category {
            Some(category) => category.into(),
            None => LedgerType::Interest.into(),
        };
        let payload = json!({"category": cat}).to_string();

        let mut params = Vec::<(&str, String)>::new();
        if let Some(limit) = limit {
            // Max 2500
            params.push(("limit", limit.to_string()));
        }

        let body = self.post(&url, Some(payload), Some(params)).await?;
        // let ledgers: Vec<Ledger> = from_str(&body).unwrap();
        let ledgers: Vec<Ledger> = from_str(&body).unwrap();
        Ok(ledgers)
    }

    /// Ref: <https://docs.bitfinex.com/reference/key-permissions>
    pub async fn request_key_permission(&self) -> Result<KeyPermission, BitfinexError> {
        let body = self.post_url(&String::from("auth/r/permissions")).await?;

        let perm: Vec<Permission> = from_str(&body).unwrap();
        let mut temp_data = serde_json::Map::<String, Value>::new();
        for p in perm {
            let v = json!({
                "name": p.name,
                "read": match p.read {true => 1, false => 0},
                "write": match p.write {true => 1, false => 0},
            });
            temp_data.insert(p.name.clone(), v);
        }
        let value: Value = Value::Object(temp_data);
        let permission: KeyPermission = serde_json::from_value(value).unwrap();
        Ok(permission)
    }

    /// Ref: <https://docs.bitfinex.com/reference/rest-auth-deposit-address>
    pub async fn request_deposit_address(
        &self,
        wallet: WalletType,
        method: DepositMethod,
    ) -> Result<Vec<DepositAddress>, BitfinexError> {
        let url = String::from("auth/w/deposit/address");
        let payload = json!({
            "wallet": wallet.as_str(),
            "method": method.to_string(),
            "op_renew": 0
        });

        let body = self.post_with_payload(&url, payload.to_string()).await?;

        let result: DepositAddressResult = from_str(&body).unwrap();
        Ok(result.addresses)
    }
}
