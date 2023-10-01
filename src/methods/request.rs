use std::future::{Future, IntoFuture};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{error::ZarinResult, results::request::Request, ZarinpalClient};

use super::ApiMethod;

#[derive(Debug, Clone, Serialize, Default)]
pub enum Currency {
    #[default]
    IRR,
    IRT,
}

/// Metadata of a payment request.
#[derive(Debug, Clone, Serialize, TypedBuilder, Default)]
pub struct Metadata {
    /// Mobile number of payer. (Can be useful for zarinpal to save card info)
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<String>,

    /// Email address of the payer.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    /// Order id.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,

    /// Card pan to accept payment only from this card.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    card_pan: Option<String>,
}

/// Info about a wage in payment request.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Wage {
    /// Shaparak iban number of the participant.
    #[builder(setter(into))]
    iban: String,

    /// The amount for this participant.
    amount: u64,

    /// Description.
    #[builder(setter(into))]
    description: String,
}

/// Request a new payment.
///
/// This type implements [`IntoFuture`], which means you can call `.await` directly
/// on it when built.
///
/// ```
/// let zarinpal = Zarinpal::new(...).unwrap();
///
/// let built = RequestPayment::builder()
///     .amount(10000)
///     .callback_url("example.com")
///     .description("...")
///     // Takes a reference to your client.
///     .zarinpal(&zarinpal)
///     .build();
///
/// let result = built.await.unwrap();
/// ```
///
/// But you may want to use an extension method to make your life brighter.
///
/// _The example below is as the same of above._
/// ```
/// let built = zarinpal
///     .request_payment(10000, "example.com", "...")
///     .build();
///
/// let result = built.await.unwrap();
/// ```
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct RequestPayment<'z, Z: ZarinpalClient> {
    /// (Optional) Merchant id of whoever makes the payment request.
    ///
    /// If you leave this field as `None`, [`ZarinpalClient`] will set it.
    #[builder(default, setter(strip_option, into))]
    merchant_id: Option<String>,

    /// (Optional) Currency for the payment.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<Currency>,

    /// Payment amount.
    amount: u64,

    /// Callback url of the payment.
    #[builder(setter(into))]
    callback_url: String,

    /// Description.
    #[builder(setter(into))]
    description: String,

    /// (Optional) Metadata of the payment. (contains: `mobile`, `email`, `order_id` and `card_pan`).
    #[builder(default)]
    metadata: Metadata,

    /// (Optional) Wages information.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    wages: Option<Vec<Wage>>,

    /// The zarinpal client to send this request with.
    #[serde(skip_serializing)]
    #[builder(setter(strip_option))]
    zarinpal: Option<&'z Z>,
}

impl<'z, Z: ZarinpalClient + Sync + Send> IntoFuture for RequestPayment<'z, Z> {
    type Output = ZarinResult<Request>;
    type IntoFuture = ::core::pin::Pin<Box<dyn Future<Output = Self::Output> + Send + 'z>>;

    fn into_future(mut self) -> Self::IntoFuture {
        let zarinpal = std::mem::take(&mut self.zarinpal).unwrap(); // Can't be none if object is built!
        Box::pin(zarinpal.send(self))
    }
}

impl<'z, Z: ZarinpalClient> ApiMethod for RequestPayment<'z, Z> {
    const PATH: &'static str = "pg/v4/payment/request.json";

    type Result = Request;

    fn set_merchant_id_if_needed(&mut self, merchant_id: impl Into<String>) {
        match self.merchant_id {
            None => self.merchant_id = Some(merchant_id.into()),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Zarinpal;

    use super::*;

    #[test]
    fn test_serialization() {
        let zarinpal = Zarinpal::new(uuid::Uuid::new_v4().to_string().as_str()).unwrap();

        let raw_json = serde_json::json!({
            "merchant_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
            "amount": 1000,
            "callback_url": "http://alireza.work/verify",
            "description": "Transaction description.",
            "metadata": {
                "mobile": "09106869409",
                "email": "info.test@gmail.com"
            }
        });

        let from_model = serde_json::to_value(
            &RequestPayment::builder()
                .merchant_id("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
                .amount(1000)
                .callback_url("http://alireza.work/verify")
                .description("Transaction description.")
                .metadata(
                    Metadata::builder()
                        .mobile("09106869409")
                        .email("info.test@gmail.com")
                        .build(),
                )
                .zarinpal(&zarinpal)
                .build(),
        )
        .unwrap();

        // DO NOT test using string representing, since field ordering are different.
        assert_eq!(raw_json, from_model)
    }

    // cSpell:disable

    #[test]
    fn test_serialization_with_currency() {
        let zarinpal = Zarinpal::new(uuid::Uuid::new_v4().to_string().as_str()).unwrap();

        let raw_json = serde_json::json!({
          "merchant_id": "1344b5d4-0048-11e8-94db-005056a205be",
          "currency": "IRT",
          "amount": 10000,
          "callback_url": "http://yoursite.com/verify",
          "description": "افزایش اعتبار کاربر شماره ۱۱۳۴۶۲۹",
          "metadata": {"mobile": "09121234567","email": "info.test@gmail.com"}
        });

        let from_model = serde_json::to_value(
            &RequestPayment::builder()
                .merchant_id("1344b5d4-0048-11e8-94db-005056a205be")
                .amount(10000)
                .callback_url("http://yoursite.com/verify")
                .currency(Currency::IRT)
                .description("افزایش اعتبار کاربر شماره ۱۱۳۴۶۲۹")
                .metadata(
                    Metadata::builder()
                        .mobile("09121234567")
                        .email("info.test@gmail.com")
                        .build(),
                )
                .zarinpal(&zarinpal)
                .build(),
        )
        .unwrap();

        // DO NOT test using string representing, since field ordering are different.
        assert_eq!(raw_json, from_model)
    }

    #[test]
    fn test_serialization_with_wages() {
        let zarinpal = Zarinpal::new(uuid::Uuid::new_v4().to_string().as_str()).unwrap();

        let raw_json = serde_json::json!({
          "merchant_id": "1344b5d4-0048-11e8-94db-005056a205be",
          "amount": 20000,
          "callback_url": "http://yoursite.com/verify",
          "description": "Transaction description.",
          "metadata": {
            "mobile": "091212334567",
            "email": "info.test@gmail.com"
          },
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
          ]
        });

        let from_model = serde_json::to_value(
            &RequestPayment::builder()
                .merchant_id("1344b5d4-0048-11e8-94db-005056a205be")
                .amount(20000)
                .callback_url("http://yoursite.com/verify")
                .description("Transaction description.")
                .metadata(
                    Metadata::builder()
                        .mobile("091212334567")
                        .email("info.test@gmail.com")
                        .build(),
                )
                .wages([
                    Wage::builder()
                        .iban("IR130570028780010957775103")
                        .amount(1000)
                        .description("تسهیم سود فروش از محصول به مسعود امینی")
                        .build(),
                    Wage::builder()
                        .iban("IR670170000000352965862009")
                        .amount(5000)
                        .description("تسهیم سود فروش از محصول به یوسفی")
                        .build(),
                ])
                .zarinpal(&zarinpal)
                .build(),
        )
        .unwrap();

        // DO NOT test using string representing, since field ordering are different.
        assert_eq!(raw_json, from_model)
    }

    #[test]
    fn test_serialization_with_card_pan() {
        let zarinpal = Zarinpal::new(uuid::Uuid::new_v4().to_string().as_str()).unwrap();

        let raw_json = serde_json::json!({
            "merchant_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
            "amount": 10000,
            "callback_url": "http://yoursite.com/verify",
            "description": "پرداخت تست ۱۱۰",
            "metadata": {
                "mobile": "09121234567",
                "email": "info.test@gmail.com",
                "card_pan":"5022291083818920",
            }
        });

        let from_model = serde_json::to_value(
            &RequestPayment::builder()
                .merchant_id("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
                .amount(10000)
                .callback_url("http://yoursite.com/verify")
                .description("پرداخت تست ۱۱۰")
                .metadata(
                    Metadata::builder()
                        .mobile("09121234567")
                        .email("info.test@gmail.com")
                        .card_pan("5022291083818920")
                        .build(),
                )
                .zarinpal(&zarinpal)
                .build(),
        )
        .unwrap();

        // DO NOT test using string representing, since field ordering are different.
        assert_eq!(raw_json, from_model)
    }
}
