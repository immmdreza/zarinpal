//! Extension traits for [`Zarinpal`].

use crate::{
    methods::{request::RequestPayment, unverified::UnverifiedRequests, verify::VerifyPayment},
    ZarinpalClient,
};

pub trait ZarinpalSendExtension: ZarinpalClient + Sized {
    /// Request a payment through Zarinpal payments gateway.
    fn request_payment<'z>(
        &'z self,
        amount: u64,
        callback_url: reqwest::Url,
        description: impl Into<String>,
    ) -> crate::methods::request::RequestPaymentBuilder<
        '_,
        Self,
        (
            (),
            (),
            (u64,),
            (String,),
            (String,),
            (),
            (),
            (Option<&Self>,),
        ),
    > {
        RequestPayment::builder()
            .zarinpal(self)
            .amount(amount)
            .callback_url(callback_url)
            .description(description)
    }

    /// Verify a previously made payment requests through Zarinpal payments gateway.
    fn verify_payment<'z>(
        &'z self,
        authority: impl Into<String>,
        amount: u64,
    ) -> crate::methods::verify::VerifyPaymentBuilder<
        '_,
        Self,
        ((), (u64,), (String,), (Option<&Self>,)),
    > {
        VerifyPayment::builder()
            .zarinpal(self)
            .amount(amount)
            .authority(authority)
    }

    /// Returns a list of at most 100 recent unverified payment requests.
    fn unverified_requests<'z>(
        &'z self,
    ) -> crate::methods::unverified::UnverifiedRequestsBuilder<'_, Self, ((), (Option<&Self>,))>
    {
        UnverifiedRequests::builder().zarinpal(self)
    }
}

impl<T> ZarinpalSendExtension for T where T: ZarinpalClient {}

#[cfg(test)]
mod tests {
    use crate::{
        methods::request::{Currency, Metadata},
        prelude::ZarinpalSendExtension,
        Zarinpal,
    };

    #[tokio::test]
    async fn test_1() {
        let zarinpal = Zarinpal::new_test().unwrap();

        let unverified = zarinpal.unverified_requests().build().await;
        println!("{unverified:#?}")
    }

    #[tokio::test]
    async fn test_2() {
        let zarinpal = Zarinpal::new_test().unwrap();

        let unverified = zarinpal
            .request_payment(
                10000,
                "https://google.com/".parse().unwrap(),
                "Test Payment 1",
            )
            .build()
            .await;

        println!("{unverified:#?}")
    }

    #[tokio::test]
    async fn test_3() {
        let zarinpal = Zarinpal::new_test().unwrap();

        let unverified = zarinpal
            .request_payment(
                10000,
                "https://google.com/".parse().unwrap(),
                "Test Payment 1",
            )
            // Setting some optional field
            .currency(Currency::IRT)
            .metadata(Metadata::builder().mobile("mobile").email("email").build())
            .build()
            .await;

        println!("{unverified:#?}")
    }
}
