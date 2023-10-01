//! Contains result types of the requests.

pub mod __private;
pub mod request;
pub mod result_code;
pub mod unverified;
pub mod verify;

use serde::de::DeserializeOwned;

use result_code::ResultCode;

use crate::error::ApiError;

pub trait RequestResult: DeserializeOwned {
    /// **Error code returned from api.**
    ///
    /// _In this case (Successful request) it must either be `100` (Success) or `100` (Already verified,
    /// if you did a [`crate::methods::verify::VerifyPayment`] request)._
    fn code(&self) -> ResultCode;

    /// **Result message.**
    ///
    /// _In this case (Successful request) It acts as a description of requests success
    /// and not an actual data._
    fn message(&self) -> &str;
}

pub type ApiResult<T> = Result<T, ApiError>;

impl<R: RequestResult> From<__private::ApiResult<R>> for ApiResult<R> {
    fn from(value: __private::ApiResult<R>) -> Self {
        match value.data {
            __private::WiredOption::Some(data) => Ok(data),
            __private::WiredOption::None => match value.errors {
                __private::WiredOption::Some(errors) => Err(errors),
                __private::WiredOption::None => unreachable!(),
            },
        }
    }
}
