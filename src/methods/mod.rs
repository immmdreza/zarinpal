//! This module contains requests type.

pub mod request;
pub mod unverified;
pub mod verify;

use serde::Serialize;

use crate::results::RequestResult;

pub trait ApiMethod: Serialize {
    const PATH: &'static str;

    type Result: RequestResult;

    fn set_merchant_id_if_needed(&mut self, merchant_id: impl Into<String>);
}
