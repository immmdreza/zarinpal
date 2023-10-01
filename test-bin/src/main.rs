use zarinpal::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Default merchant_id to be used in every request.
    let zarinpal = Zarinpal::new("merchant_id")?;

    let request = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .build()
        .await?;

    let _verified = zarinpal
        .verify_payment(request.authority(), 10000)
        .build()
        .await?;

    let _request_2 = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .metadata(
            Metadata::builder()
                .mobile("98912345678")
                .email("test@example.com")
                .build(),
        )
        .build()
        .await?;

    let _request_3 = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .currency(Currency::IRT) // Tomans
        .build()
        .await?;

    let _request_4 = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .wages([
            Wage::builder()
                .iban("...")
                .amount(5000)
                .description("To my first friend")
                .build(),
            Wage::builder()
                .iban("...")
                .amount(5000)
                .description("To my second friend")
                .build(),
        ])
        .build()
        .await?;

    let unverified_payments = zarinpal.unverified_requests().build().await?;

    for unverified in unverified_payments.authorities() {
        println!("{}", unverified.authority())
    }

    Ok(())
}
