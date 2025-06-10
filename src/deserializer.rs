
use serde::de::{self, Deserializer};
use chrono::{Local, DateTime};
use serde::Deserialize;


pub fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(IntOrBoolVisitor)
}


pub fn from_mts<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    if let Some(result) = DateTime::from_timestamp_millis(timestamp) {
        Ok(result.with_timezone(&Local))
    } else {
        Err(de::Error::custom("Failed to parse"))
    }
}

pub fn to_mts<S>(
    datetime: &DateTime<Local>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    serializer.serialize_i64(datetime.timestamp_millis())
}


struct IntOrBoolVisitor;

impl<'de> de::Visitor<'de> for IntOrBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean or an integer (0 or 1)")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value) // Directly return the boolean value
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(de::Error::custom("Expected 0 or 1 for boolean field")),
        }
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(de::Error::custom("Expected 0 or 1 for boolean field")),
        }
    }

    // Optionally handle other numeric types if needed
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(de::Error::custom("Expected 0 or 1 for boolean field")),
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(de::Error::custom("Expected 0 or 1 for boolean field")),
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(de::Error::custom("Expected 'true' or 'false' for boolean field")),
        }
    }
}