use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::NaiveDateTime;

pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match date {
        Some(ref dt) => serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)?)),
        None => Ok(None),
    }
}
