//! Contains a universal [`Error`] type and associated [`ZarinResult`] for the create.s

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Deserializer};
use thiserror::Error;

use crate::results::result_code::ResultCode;

/// An error that ocurred while sending a request to the api.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// Error code returned from api.
    code: ResultCode,

    /// Error message.
    message: String,

    /// A set of validations and their description
    /// that're failed and must be fixed before sending request.
    ///
    /// Eg:
    ///
    /// ```json
    /// {
    ///     "merchant_id": [
    ///         "Merchant id is not a valid uuid."
    ///     ]
    /// }
    /// ```
    #[serde(deserialize_with = "deserialize_validations")]
    validations: HashMap<String, Vec<String>>,
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Error code ({}) ocurred while communicating with zarinpal api: {}",
            self.code, self.message
        )?;
        writeln!(f, "Here're detailed information:")?;
        writeln!(f, "{:#?}", self.validations)
    }
}

impl ApiError {
    /// Error code returned from api.
    pub fn code(&self) -> ResultCode {
        self.code
    }

    /// Error message.
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    /// A set of validations that're failed and must be fixed before sending request.
    pub fn validations(&self) -> &HashMap<String, Vec<String>> {
        &self.validations
    }
}

fn deserialize_validations<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opening = Vec::<HashMap<String, String>>::deserialize(deserializer)?;
    let mut result = HashMap::<String, Vec<String>>::new();

    for item in opening {
        for (key, val) in item {
            let entry = result.entry(key);
            let vec = entry.or_insert_with(|| vec![]);
            vec.push(val)
        }
    }

    Ok(result)
}

/// Represents an error that ocurred inside this ([`zarinpal`]) crate.
///
/// Includes errors related to zarinpal api and http client.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Zarinpal api error: {0}")]
    ZarinpalApiError(ApiError),
    #[error("Http client error: {0}")]
    HttpClientError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::HttpClientError(value)
    }
}

impl From<ApiError> for Error {
    fn from(value: ApiError) -> Self {
        Error::ZarinpalApiError(value)
    }
}

/// Result type for this crate's [`Error`] type.
pub type ZarinResult<T> = Result<T, Error>;
