use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

use crate::error::ApiError;

struct MapOrSeq<T>(PhantomData<fn() -> T>);

impl<'de, T> Visitor<'de> for MapOrSeq<T>
where
    T: Deserialize<'de>,
{
    type Value = WiredOption<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("seq or map")
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        Ok(WiredOption::None)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        Ok(WiredOption::Some(Deserialize::deserialize(
            de::value::MapAccessDeserializer::new(map),
        )?))
    }
}

/// [`T`] Here must be an mapping (struct).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum WiredOption<T> {
    Some(T),
    None,
}

impl<T> Default for WiredOption<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<WiredOption<T>> for Option<T> {
    fn from(value: WiredOption<T>) -> Self {
        match value {
            WiredOption::Some(s) => s.into(),
            WiredOption::None => Option::None,
        }
    }
}

#[allow(dead_code)]
impl<T> WiredOption<T> {
    fn is_none(&self) -> bool {
        matches!(self, WiredOption::None)
    }

    fn is_some(&self) -> bool {
        matches!(self, WiredOption::Some(_))
    }
}

impl<'de, T> Deserialize<'de> for WiredOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MapOrSeq(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct Data<T> {
        data: WiredOption<T>,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct Hi {
        text: String,
    }

    #[test]
    fn test_1() {
        let json = serde_json::json!({
            "data": []
        })
        .to_string();

        let result = serde_json::from_str::<Data<Hi>>(&json).unwrap();
        assert!(result.data.is_none())
    }

    #[test]
    fn test_2() {
        let json = serde_json::json!({
            "data": {
                "text": "Hi"
            }
        })
        .to_string();

        let result = serde_json::from_str::<Data<Hi>>(&json).unwrap();
        println!("{result:#?}");
        assert!(result.data.is_some())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResult<R> {
    #[serde(default = "WiredOption::default")]
    pub(crate) data: WiredOption<R>,
    #[serde(default = "WiredOption::default")]
    pub(crate) errors: WiredOption<ApiError>,
}
