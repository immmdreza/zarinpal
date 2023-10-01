//! Zarinpal payment gateway api client that support all api methods as follows
//!
//! - Request (To start a payment process)
//! - Verify (To verify payments)
//! - Unverified (To fetch unverified payments)
//!
//! Supports `Wages`, `Currency`, `Card pan` and other ...

use error::{ApiError, ZarinResult};
use methods::ApiMethod;

pub mod error;
pub mod extensions;
pub mod methods;
pub mod prelude;
pub mod results;

/// [`ZarinpalClient`] is an interface to all zarinpal payment gateway api clients.
/// This will be useful to implement extension methods on everything that implements this.
///
/// You may want to use [`Zarinpal`] to send requests!
#[async_trait::async_trait]
pub trait ZarinpalClient {
    /// Inner http client that is responsible for sending requests.
    fn client(&self) -> &reqwest::Client;

    /// The merchant id passed to the client.
    fn merchant_id(&self) -> &str;

    /// The base url for all requests.
    fn base_url(&self) -> &reqwest::Url;

    async fn send<M: ApiMethod + Send + Sync>(&self, mut method: M) -> ZarinResult<M::Result> {
        let mut url = self.base_url().clone();
        url.set_path(M::PATH);

        method.set_merchant_id_if_needed(self.merchant_id().clone());

        let result = self
            .client()
            .post(url)
            .json(&method)
            .send()
            .await?
            .json::<crate::results::__private::ApiResult<M::Result>>()
            .await;

        result
            .map(|f| Into::<Result<M::Result, ApiError>>::into(f))?
            .map_err(|e| e.into())
    }
}

/// High-level Zarinpal payment gateway api client to simply send requests to the api.
///
/// # Examples
///
/// Using `.send()` method to send api requests:
/// ```
/// // Merchant id from zarinpal dashboard.
/// let merchant_id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx";
/// // The new method fails if the `merchant_id` is invalid or can't create reqwest::Client.
/// let zarinpal = Zarinpal::new(merchant_id)?;
///
/// let unverified = zarinpal.send(UnverifiedRequests::builder().build()).await?;
/// ```
///
/// ### Example 2
/// Let's verify 10 recent unverified payment requests.
/// ```
/// let zarinpal = Zarinpal::new("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")?();

/// let unverified = zarinpal.unverified_requests().build().await?();
/// for (i, request) in unverified.authorities().iter().enumerate().take(10) {
///     let verify = request.verify(&zarinpal).await?();
///
///     println!("{}- {verify:#?}", i + 1)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Zarinpal {
    client: reqwest::Client,
    // merchant_id_uuid: uuid::Uuid,
    merchant_id: String,
    base_url: reqwest::Url,
}

#[async_trait::async_trait]
impl ZarinpalClient for Zarinpal {
    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn merchant_id(&self) -> &str {
        &self.merchant_id
    }

    fn base_url(&self) -> &reqwest::Url {
        &self.base_url
    }
}

impl Zarinpal {
    /// Creates a new instance of [`Zarinpal`] client.
    ///
    /// This method will fail if `merchant_id` is not a vail uuid.
    ///
    /// ## Note
    /// Almost all of zarinpal requests can carry `merchant_id` with themselves (as a field).
    /// merchant id here will be replaced with `merchant_id` field in requests if it's not present
    pub fn new(merchant_id: &str) -> Result<Self, uuid::Error> {
        let merchant_id_uuid = uuid::Uuid::parse_str(merchant_id)?;
        Ok(Self {
            client: reqwest::Client::new(),
            merchant_id: merchant_id_uuid.to_string(),
            // merchant_id_uuid,
            base_url: "https://api.zarinpal.com/".parse().unwrap(),
        })
    }

    /// Creates a new instance of [`Zarinpal`] client with custom [`reqwest::Client`]
    /// as inner http client.
    ///
    /// This method will fail if `merchant_id` is not a vail uuid.
    ///
    /// ## Note
    /// Almost all of zarinpal requests can carry `merchant_id` with themselves (as a field).
    /// merchant id here will be replaced with `merchant_id` field in requests if it's not present
    pub fn new_with_client(
        merchant_id: &str,
        client: reqwest::Client,
    ) -> Result<Self, uuid::Error> {
        let merchant_id_uuid = uuid::Uuid::parse_str(merchant_id)?;
        Ok(Self {
            client,
            merchant_id: merchant_id_uuid.to_string(),
            // merchant_id_uuid,
            base_url: "https://api.zarinpal.com/".parse().unwrap(),
        })
    }
}
