use serde::Deserialize;

use crate::methods::request::Wage;

use super::{RequestResult, ResultCode};

/// Indicates who's responsible for paying the payment fee.
#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeeType {
    /// Payer of the payment.
    Payer,

    /// Merchant of the payment.
    Merchant,

    /// An unknown fee type.
    Unknown,
}

impl FeeType {
    /// Returns `true` if the fee type is [`Payer`].
    ///
    /// [`Payer`]: FeeType::Payer
    #[must_use]
    pub fn is_payer(&self) -> bool {
        matches!(self, Self::Payer)
    }

    /// Returns `true` if the fee type is [`Merchant`].
    ///
    /// [`Merchant`]: FeeType::Merchant
    #[must_use]
    pub fn is_merchant(&self) -> bool {
        matches!(self, Self::Merchant)
    }
}

/// The result type of a successful [`crate::methods::verify::VerifyPayment`] request.
///
/// Error code `101` ([`ResultCode::Verified`]) means this payment was verified before.
#[derive(Debug, Clone, Deserialize)]
pub struct Verify {
    code: ResultCode,
    message: String,

    /// SHA256 hash of card number.
    card_hash: String,

    /// Masked card number in a format like `60379986****5434`.
    card_pan: String,

    /// Reference id of the payment.
    ref_id: u64,

    /// Fee type. Indicates if the [`FeeType::Merchant`] is responsible for payment fee or [`FeeType::Payer`].
    fee_type: FeeType,

    /// Fee amount.
    fee: u64,

    /// Wages you've entered while sending payment request, just in case.
    #[serde(default)]
    wages: Option<Vec<Wage>>,
}

impl Verify {
    /// SHA256 hash of card number.
    pub fn card_hash(&self) -> &str {
        self.card_hash.as_ref()
    }

    /// Masked card number in a format like `60379986****5434`.
    pub fn card_pan(&self) -> &str {
        self.card_pan.as_ref()
    }

    /// Reference id of a successful payment.
    pub fn ref_id(&self) -> u64 {
        self.ref_id
    }

    /// Fee amount.
    pub fn fee(&self) -> u64 {
        self.fee
    }

    /// Wages you've entered while sending payment request, just in case.
    pub fn wages(&self) -> Option<&Vec<Wage>> {
        self.wages.as_ref()
    }

    /// Fee type. Indicates if the [`FeeType::Merchant`] is responsible for payment fee or [`FeeType::Payer`].
    pub fn fee_type(&self) -> FeeType {
        self.fee_type
    }
}

impl Verify {
    /// Indicates if this payment was verified before.
    #[must_use]
    pub fn already_verified(&self) -> bool {
        matches!(self.code, ResultCode::Verified)
    }
}

impl RequestResult for Verify {
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
        let inner_model = Verify {
            code: ResultCode::Success,
            message: "Verified".to_string(),
            card_hash: "1EBE3EBEBE35C7EC0F8D6EE4F2F859107A87822CA179BC9528767EA7B5489B69"
                .to_string(),
            card_pan: "502229******5995".to_string(),
            ref_id: 201,
            fee_type: FeeType::Merchant,
            fee: 0,
            wages: None,
        };

        let from_json = serde_json::from_value::<crate::results::__private::ApiResult<Verify>>(
            serde_json::json!({
                "data": {
                    "code": 100,
                    "message": "Verified",
                    "card_hash": "1EBE3EBEBE35C7EC0F8D6EE4F2F859107A87822CA179BC9528767EA7B5489B69",
                    "card_pan": "502229******5995",
                    "ref_id": 201,
                    "fee_type": "Merchant",
                    "fee": 0
                },
                "errors": []
            }),
        )
        .unwrap();

        let data: Option<Verify> = from_json.data.into();
        let data = data.unwrap();

        assert_eq!(data.card_hash, inner_model.card_hash);
        assert_eq!(data.code, inner_model.code);
        assert_eq!(data.message, inner_model.message);
        assert_eq!(data.fee, inner_model.fee);
        assert_eq!(data.fee_type, inner_model.fee_type);
        assert_eq!(data.card_pan, inner_model.card_pan);
        assert_eq!(data.ref_id, inner_model.ref_id);
        assert!(data.wages.is_none());
    }

    #[test]
    fn test_deserialization_with_wages() {
        // cSpell:disable

        let inner_model = Verify {
            code: ResultCode::Success,
            message: "Paid".to_string(),
            card_hash: "16A8E235A8C6047574D413008DB1FC9D51A44E3D37C83BAFC6491A72B696D4541FE77F7B057E884A9F5BD101F477C4B22990C1FC833FEB79DDAE9C6F56BE889B"
                .to_string(),
            card_pan: "502229******8920".to_string(),
            ref_id: 21790905,
            fee_type: FeeType::Merchant,
            fee: 0,
            wages: Some(vec![
                Wage::builder()
                    .iban("IR130570028780010957775103")
                    .amount(1000)
                    .description("تسهیم سود فروش از محصول به مسعود امینی")
                    .build(),
                Wage::builder()
                    .iban("IR670170000000352965862009")
                    .amount(5000)
                    .description("تسهیم سود فروش از محصول به یوسفی")
                    .build()
            ]),
        };

        let from_json = serde_json::from_value::<crate::results::__private::ApiResult<Verify>>(
            serde_json::json!({
                "data": {
                    "wages": [
                        {
                            "iban": "IR130570028780010957775103",
                            "amount": 1000,
                            "description": "تسهیم سود فروش از محصول به مسعود امینی"
                        },
                        {
                            "iban": "IR670170000000352965862009",
                            "amount": 5000,
                            "description": "تسهیم سود فروش از محصول به یوسفی"
                        }
                    ],
                    "code": 100,
                    "message": "Paid",
                    "card_hash": "16A8E235A8C6047574D413008DB1FC9D51A44E3D37C83BAFC6491A72B696D4541FE77F7B057E884A9F5BD101F477C4B22990C1FC833FEB79DDAE9C6F56BE889B",
                    "card_pan": "502229******8920",
                    "ref_id": 21790905,
                    "fee_type": "Merchant",
                    "fee": 0
                },
                "errors": []
            }),
        )
        .unwrap();

        let data: Option<Verify> = from_json.data.into();
        let data = data.unwrap();

        assert_eq!(data.card_hash, inner_model.card_hash);
        assert_eq!(data.code, inner_model.code);
        assert_eq!(data.message, inner_model.message);
        assert_eq!(data.fee, inner_model.fee);
        assert_eq!(data.fee_type, inner_model.fee_type);
        assert_eq!(data.card_pan, inner_model.card_pan);
        assert_eq!(data.ref_id, inner_model.ref_id);
        assert!(data.wages.is_some());

        let wages = data.wages.unwrap();
        assert_eq!(wages.len(), 2)
    }
}
