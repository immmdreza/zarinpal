//! Everything you need to get started with the crate.
//!
//! Just do like this
//! ```
//! use zarinpal::prelude::*;
//! ```

pub use crate::{
    error::ZarinResult,
    extensions::ZarinpalSendExtension,
    methods::{
        request::{Currency, Metadata, RequestPayment, Wage},
        unverified::UnverifiedRequests,
        verify::VerifyPayment,
        ApiMethod,
    },
    results::{
        request::Request,
        result_code::ResultCode,
        unverified::{Authorities, Unverified},
        verify::Verify,
        ApiResult, RequestResult,
    },
    Zarinpal, ZarinpalClient,
};
