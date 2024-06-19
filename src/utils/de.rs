use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};

pub fn deserialize_option_f64<'de, D>(deserializer: D) -> std::result::Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) if s == "-" => Ok(None),
        Some(serde_json::Value::String(s)) => s.parse::<f64>().map(Some).map_err(de::Error::custom),
        Some(serde_json::Value::Number(num)) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number")).map(Some),
        Some(_) => Err(de::Error::custom("Invalid type")),
        None => Ok(None),
    }
}

pub fn deserialize_option_i64<'de, D>(deserializer: D) -> std::result::Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) if s == "-" => Ok(None),
        Some(serde_json::Value::String(s)) => s.parse::<i64>().map(Some).map_err(de::Error::custom),
        Some(serde_json::Value::Number(num)) => num.as_i64().ok_or_else(|| de::Error::custom("Invalid number")).map(Some),
        Some(_) => Err(de::Error::custom("Invalid type")),
        None => Ok(None),
    }
}