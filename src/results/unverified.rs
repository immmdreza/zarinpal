use serde::Deserialize;

use crate::{
    prelude::{ZarinResult, ZarinpalSendExtension},
    Zarinpal,
};

use super::{result_code::ResultCode, RequestResult};

/// Authority information of a payment request that can be used to verify the payment later.
#[derive(Debug, Clone, Deserialize)]
pub struct Authorities {
    /// Unique authority of the payment request.
    authority: String,
    /// Payment amount.
    amount: u64,
    /// Callback url of the payment.
    callback_url: String,
    /// Refer url.
    referer: String,
    /// Date and time of the request in a format like: `2020-06-27 10:22:02`.
    date: String,
}

impl Authorities {
    /// Unique authority of the payment request.
    pub fn authority(&self) -> &str {
        self.authority.as_ref()
    }

    /// Payment amount.
    pub fn amount(&self) -> u64 {
        self.amount
    }

    /// Callback url of the payment.
    pub fn callback_url(&self) -> &str {
        self.callback_url.as_ref()
    }

    /// Refer url.
    pub fn referer(&self) -> &str {
        self.referer.as_ref()
    }

    /// Date and time of the request in a format like: `2020-06-27 10:22:02`.
    pub fn date(&self) -> &str {
        self.date.as_ref()
    }

    /// Directly verify this payment requests using `authority` and `amount`.
    pub async fn verify(&self, zarinpal: &Zarinpal) -> ZarinResult<crate::prelude::Verify> {
        zarinpal
            .verify_payment(self.authority(), self.amount())
            .build()
            .await
    }
}

/// The result type of a successful [`crate::methods::unverified::UnverifiedRequests`] request.
#[derive(Debug, Clone, Deserialize)]
pub struct Unverified {
    code: String,
    message: String,

    /// Extra information about the payment request that can be used to verify a payment later.
    authorities: Vec<Authorities>,
}

impl Unverified {
    /// Extra information about the payment request that can be used to verify a payment later.
    pub fn authorities(&self) -> &[Authorities] {
        self.authorities.as_ref()
    }
}

impl RequestResult for Unverified {
    fn code(&self) -> ResultCode {
        self.code.parse::<i64>().unwrap().into()
    }

    fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let inner_model = Unverified {
            code: "100".to_string(),
            message: "Success".to_string(),
            authorities: vec![Authorities {
                authority: "A00000000000000000000000000207288780".to_string(),
                amount: 50500,
                callback_url: "https://golroz.com/vpay".to_string(),
                referer: "https://golroz.com/test-form/".to_string(),
                date: "2020-07-01 17:33:25".to_string(),
            }],
        };

        let from_json = serde_json::from_value::<crate::results::__private::ApiResult<Unverified>>(
            serde_json::json!({
                "data": {
                    "code": "100",
                    "message": "Success",
                    "authorities": [
                        {
                            "authority": "A00000000000000000000000000207288780",
                            "amount": 50500,
                            "callback_url": "https://golroz.com/vpay",
                            "referer": "https://golroz.com/test-form/",
                            "date": "2020-07-01 17:33:25"
                        },
                    ]
                }
            }),
        )
        .unwrap();

        let data: Option<Unverified> = from_json.data.into();
        let data = data.unwrap();

        assert_eq!(data.code, inner_model.code);
        assert_eq!(data.message, inner_model.message);
        assert_eq!(data.authorities.len(), 1)
    }
}
