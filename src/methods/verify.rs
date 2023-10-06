use std::future::{Future, IntoFuture};

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::{error::ZarinResult, results::verify::Verify, ZarinpalClient};

use super::ApiMethod;

/// Verify a payment request.
///
/// This type implements [`IntoFuture`], which means you can call `.await` directly
/// on it when built.
///
/// ```
/// use zarinpal::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result::<(), Box<dyn std::error::Error>> {
///     let zarinpal = Zarinpal::new("...")?;
///
///     let built = VerifyPayment::builder()
///         .amount(10000)
///         .authority("A00000000000000000000000000217885159")
///         // Takes a reference to your client.
///         .zarinpal(&zarinpal)
///         .build();
///
///     let result = built.await?;
///
///     Ok(())
/// }
/// ```
///
/// But you may want to use an extension method to make your life brighter.
///
/// _The example below is as the same of above._
/// ```
/// use zarinpal::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result::<(), Box<dyn std::error::Error>> {
///     let zarinpal = Zarinpal::new("...")?;
///
///     let built = zarinpal
///         .verify_payment("A00000000000000000000000000217885159", 10000)
///         .build();
///
///     let result = built.await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct VerifyPayment<'z, Z: ZarinpalClient> {
    /// (Optional) Merchant id of whoever makes the payment request.
    ///
    /// If you leave this field as `None`, [`ZarinpalClient`] will set it.
    #[builder(default, setter(strip_option, into))]
    merchant_id: Option<String>,

    /// Payment amount.
    amount: u64,

    /// The unique authority of the payment.
    #[builder(setter(into))]
    authority: String,

    /// The zarinpal client to send this request with.
    #[serde(skip_serializing)]
    #[builder(setter(strip_option))]
    zarinpal: Option<&'z Z>,
}

impl<'z, Z: ZarinpalClient + Sync + Send> IntoFuture for VerifyPayment<'z, Z> {
    type Output = ZarinResult<Verify>;
    type IntoFuture = ::core::pin::Pin<Box<dyn Future<Output = Self::Output> + Send + 'z>>;

    fn into_future(mut self) -> Self::IntoFuture {
        let zarinpal = std::mem::take(&mut self.zarinpal).unwrap(); // Can't be none if object is built!
        Box::pin(zarinpal.send(self))
    }
}

impl<'z, Z: ZarinpalClient> ApiMethod for VerifyPayment<'z, Z> {
    const PATH: &'static str = "pg/v4/payment/verify.json";

    type Result = Verify;

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
        let zarinpal = Zarinpal::new_test().unwrap();

        let raw_json = serde_json::json!({
            "merchant_id": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
            "amount": 1000,
            "authority": "A00000000000000000000000000217885159"
        });

        let from_model = serde_json::to_value(
            &VerifyPayment::builder()
                .merchant_id("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
                .amount(1000)
                .authority("A00000000000000000000000000217885159")
                .zarinpal(&zarinpal)
                .build(),
        )
        .unwrap();

        // DO NOT test using string representing, since field ordering are different.
        assert_eq!(raw_json, from_model)
    }

    use std::future::Future;
    use std::pin::Pin;

    struct MyFuture<'a, T>(&'a T);

    impl<'a, T: Sync> IntoFuture for MyFuture<'a, T> {
        type Output = ();
        type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

        fn into_future(self) -> Self::IntoFuture {
            Box::pin(async move {
                let _t = self.0;
                ()
            })
        }
    }
}
