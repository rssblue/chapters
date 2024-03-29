use crate::Image;
use chrono::Duration;
use serde::{Deserialize, Serialize};

// Serialize impl
impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Image::Url(url) => {
                serializer.serialize_newtype_variant("image", 0, "Url", url.as_str())
            }
        }
    }
}

pub fn float_to_duration_option<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = match Option::<f64>::deserialize(deserializer) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };
    Ok(f.map(|f| Duration::milliseconds((f * 1000.0) as i64)))
}

pub fn float_to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = f64::deserialize(deserializer)?;
    Ok(Duration::milliseconds((f * 1000.0) as i64))
}

pub fn duration_option_to_float_option<S>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match duration {
        Some(duration) => duration_to_float(duration, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn duration_to_float<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // If float ends in .0 or doesn't even have a decimal point, serialize as an integer.
    let float = duration.num_milliseconds() as f64 / 1000.0;
    if float.fract() == 0.0 {
        return serializer.serialize_i64(float as i64);
    }
    serializer.serialize_f64(float)
}

pub fn string_to_url<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(url::Url::parse(&s).ok())
}

pub fn url_to_string<S>(url: &url::Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(url.as_str())
}

pub fn url_option_to_string<S>(url: &Option<url::Url>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match url {
        Some(url) => url_to_string(url, serializer),
        None => serializer.serialize_none(),
    }
}
