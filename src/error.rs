#[derive(Debug)]
pub enum BitfinexError {
    ExceedMaxOfferCount,
    BitfinexGenericError(String),
    InvalidCurrency,
    InvalidKeyDigest,
    RateLimited,
    BitfinexTempUnavailable,
    NonceSmall,
}
