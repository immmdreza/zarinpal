use std::future::{Future, IntoFuture};

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::{error::ZarinResult, results::unverified::Unverified, ZarinpalClient};

use super::ApiMethod;

/// Get 100 recent unverified payment requests..
///
/// This type implements [`IntoFuture`], which means you can call `.await` directly
/// on it when built.
///
/// ```
/// let zarinpal = Zarinpal::new(...).unwrap();
///
/// let built = UnverifiedRequests::builder()
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
/// let built = zarinpal.unverified_requests().build();
///
/// let result = built.await.unwrap();
/// ```
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct UnverifiedRequests<'z, Z: ZarinpalClient> {
    /// (Optional) Merchant id of whoever makes the payment request.
    ///
    /// If you leave this field as `None`, [`ZarinpalClient`] will set it.
    #[builder(default, setter(strip_option, into))]
    merchant_id: Option<String>,

    /// The zarinpal client to send this request with.
    #[serde(skip_serializing)]
    #[builder(setter(strip_option))]
    zarinpal: Option<&'z Z>,
}

impl<'z, Z: ZarinpalClient + Sync + Send> IntoFuture for UnverifiedRequests<'z, Z> {
    type Output = ZarinResult<Unverified>;
    type IntoFuture = ::core::pin::Pin<Box<dyn Future<Output = Self::Output> + Send + 'z>>;

    fn into_future(mut self) -> Self::IntoFuture {
        let zarinpal = std::mem::take(&mut self.zarinpal).unwrap(); // Can't be none if object is built!
        Box::pin(zarinpal.send(self))
    }
}

impl<'z, Z: ZarinpalClient> ApiMethod for UnverifiedRequests<'z, Z> {
    const PATH: &'static str = "pg/v4/payment/unVerified.json";

    type Result = Unverified;

    fn set_merchant_id_if_needed(&mut self, merchant_id: impl Into<String>) {
        match self.merchant_id {
            None => self.merchant_id = Some(merchant_id.into()),
            _ => (),
        }
    }
}
