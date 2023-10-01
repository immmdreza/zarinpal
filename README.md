# Zarinpal payment gateway api

This's a rust implementation of zarinpal payment gateway api client.

## Installation

```cmd
cargo add zarinpal
```

## Usage example

Here's how you can get started with the crate

### Initialize client

```rust
use zarinpal::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Default merchant_id to be used in every request.
    let zarinpal = Zarinpal::new("merchant_id")?;

    Ok(())
}
```

### Request a basic payment

```rust
    // ~~~ sniff ~~~

    let request = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .build()
        .await?;
```

### Verify a payment

```rust
    // ~~~ sniff ~~~

    let verified = zarinpal
        .verify_payment(request.authority(), 10000)
        .build()
        .await?;
```

### Request payment with metadata

```rust
    // ~~~ sniff ~~~

    let request_2 = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .metadata(
            Metadata::builder()
                .mobile("98912345678")
                .email("test@example.com")
                .build(),
        )
        .build()
        .await?;

```

### Change currency in a payment

```rust
    // ~~~ sniff ~~~

    let request_3 = zarinpal
        .request_payment(10000, "example.com".parse()?, "Test payment")
        .currency(Currency::IRT) // Tomans
        .build()
        .await?;
```

### Payment request with wages

```rust
    // ~~~ sniff ~~~

    let request_4 = zarinpal
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
```

### Unverified payments

Revert a list of 100 recent unverified payments.

```rust
    // ~~~ sniff ~~~

    let unverified_payments = zarinpal.unverified_requests().build().await?;

    for unverified in unverified_payments.authorities() {
        println!("{}", unverified.authority())
    }
```

Happy making money 🔥
