use serde::Deserialize;

use super::{result_code::ResultCode, verify::FeeType, RequestResult};

/// The result type of a successful [`crate::methods::request::RequestPayment`] request.
#[derive(Debug, Clone, Deserialize)]
pub struct Request {
    code: ResultCode,
    message: String,

    /// Unique authority of the payment request.
    authority: String,

    /// Fee type. Indicates if the [`FeeType::Merchant`] is responsible for payment fee or [`FeeType::Payer`].
    fee_type: FeeType,

    /// Fee amount.
    fee: u64,
}

impl Request {
    /// Returns a url to the zarinpal payment gateway for this payment request (`authority` attached.)
    ///
    /// _This is the url that user should be redirected to, after a successful payment request._
    pub fn gateway_url(&self) -> reqwest::Url {
        format!("https://www.zarinpal.com/pg/StartPay/{}", self.authority())
            .parse()
            .unwrap()
    }
}

impl Request {
    /// Unique authority of the payment request.
    pub fn authority(&self) -> &str {
        self.authority.as_ref()
    }

    /// Fee type. Indicates if the [`FeeType::Merchant`] is responsible for payment fee or [`FeeType::Payer`].
    pub fn fee_type(&self) -> FeeType {
        self.fee_type
    }

    /// Fee amount.
    pub fn fee(&self) -> u64 {
        self.fee
    }
}

impl RequestResult for Request {
    fn code(&self) -> ResultCode {
        self.code
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
        let inner_model = Request {
            code: ResultCode::Success,
            message: "Success".to_string(),
            authority: "A00000000000000000000000000217885159".to_string(),
            fee_type: FeeType::Merchant,
            fee: 100,
        };

        let from_json = serde_json::from_value::<crate::results::__private::ApiResult<Request>>(
            serde_json::json!({
                "data": {
                    "code": 100,
                    "message": "Success",
                    "authority": "A00000000000000000000000000217885159",
                    "fee_type": "Merchant",
                    "fee": 100,
                },
                "errors": [],
            }),
        )
        .unwrap();

        let data: Option<Request> = from_json.data.into();
        let data = data.unwrap();

        assert_eq!(data.authority, inner_model.authority);
        assert_eq!(data.code, inner_model.code);
        assert_eq!(data.message, inner_model.message);
        assert_eq!(data.fee, inner_model.fee);
        assert_eq!(data.fee_type, inner_model.fee_type);
    }
}
