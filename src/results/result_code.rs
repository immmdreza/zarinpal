use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The result code of a request made to the api.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultCode {
    #[error("Validation error")]
    Validation,
    #[error("Terminal is not valid, please check merchant_id or ip address.")]
    InvalidTerminalInfo,
    #[error("Terminal is not active, please contact our support team.")]
    InactiveTerminal,
    #[error("To many attempts, please try again later.")]
    ToManyAttempts,
    #[error("Terminal user is suspend : (please contact our support team).")]
    SuspendTerminal,
    #[error("Terminal user level is not valid : ( please contact our support team).")]
    TerminalLevelToLow,
    #[error("Terminal user level is not valid : ( please contact our support team).")]
    TerminalBlueLevelRestriction,
    #[error("Success.")]
    Success,
    #[error("Terminal do not allow to accept floating wages.")]
    FloatingWagesNotAllowed,
    #[error("Terminal do not allow to accept wages, please add default bank account in panel.")]
    TerminalCantAcceptWages,
    #[error("Wages is not valid, Total wages(floating) has been overload max amount.")]
    TotalFloatingWagesHigherThanMaxAmount,
    #[error("Wages floating is not valid.")]
    InvalidWagesFloating,
    #[error("Wages is not valid, Total wages(fixed) has been overload max amount.")]
    TotalFixedWagesHigherThanMaxAmount,
    #[error("Wages is not valid, Total wages(floating) has been reached the limit in max parts.")]
    TooManyFloutingWagesPartition,
    #[error("The minimum amount for wages(floating) should be 10,000 Rials.")]
    FloatingWagesAmountTooLow,
    #[error("One or more iban entered for wages(floating) from the bank side are inactive.")]
    OneOrMoreIBansAreInactive,
    #[error("Wages need to set Iban in shaparak.")]
    IBanNotSetInShaparak,
    #[error("Wages have a error!")]
    ErrorInWages,
    #[error("Invalid extra params, expire_in is not valid.")]
    InvalidExpireInValue,
    #[error("Session is not valid, amounts values is not the same.")]
    InvalidSeasonUnmatchedAmounts,
    #[error("Session is not valid, session is not active paid try.")]
    InvalidSeasonNoActivePayment,
    #[error("Oops!!, please contact our support team.")]
    InvalidSeason,
    #[error("Session is not this merchant_id session.")]
    InvalidSeasonInvalidMerchantId,
    #[error("Invalid authority.")]
    InvalidAuthority,
    #[error("Already verified.")]
    Verified,
    #[error("Unknown error code: {0}")]
    Unknown(i64),
}

impl Serialize for ResultCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64((*self).into())
    }
}

impl<'de> Deserialize<'de> for ResultCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(i64::deserialize(deserializer)?.into())
    }
}

impl From<i64> for ResultCode {
    fn from(value: i64) -> Self {
        use ResultCode::*;
        match value {
            -9 => Validation,
            -10 => InvalidTerminalInfo,
            -11 => InactiveTerminal,
            -12 => ToManyAttempts,
            -15 => SuspendTerminal,
            -16 => TerminalLevelToLow,
            -17 => TerminalBlueLevelRestriction,
            100 => Success,
            -30 => FloatingWagesNotAllowed,
            -31 => TerminalCantAcceptWages,
            -32 => TotalFloatingWagesHigherThanMaxAmount,
            -33 => InvalidWagesFloating,
            -34 => TotalFixedWagesHigherThanMaxAmount,
            -35 => TooManyFloutingWagesPartition,
            -36 => FloatingWagesAmountTooLow,
            -37 => OneOrMoreIBansAreInactive,
            -38 => IBanNotSetInShaparak,
            -39 => ErrorInWages,
            -40 => InvalidExpireInValue,
            -50 => InvalidSeasonUnmatchedAmounts,
            -51 => InvalidSeasonNoActivePayment,
            -52 => InvalidSeason,
            -53 => InvalidSeasonInvalidMerchantId,
            -54 => InvalidAuthority,
            101 => Verified,
            e => Unknown(e),
        }
    }
}

impl From<ResultCode> for i64 {
    fn from(value: ResultCode) -> Self {
        match value {
            ResultCode::Validation => -9,
            ResultCode::InvalidTerminalInfo => -10,
            ResultCode::InactiveTerminal => -11,
            ResultCode::ToManyAttempts => -12,
            ResultCode::SuspendTerminal => -15,
            ResultCode::TerminalLevelToLow => -16,
            ResultCode::TerminalBlueLevelRestriction => -17,
            ResultCode::Success => 100,
            ResultCode::FloatingWagesNotAllowed => 30,
            ResultCode::TerminalCantAcceptWages => 31,
            ResultCode::TotalFloatingWagesHigherThanMaxAmount => 32,
            ResultCode::InvalidWagesFloating => 33,
            ResultCode::TotalFixedWagesHigherThanMaxAmount => 34,
            ResultCode::TooManyFloutingWagesPartition => 35,
            ResultCode::FloatingWagesAmountTooLow => 36,
            ResultCode::OneOrMoreIBansAreInactive => 37,
            ResultCode::IBanNotSetInShaparak => 38,
            ResultCode::ErrorInWages => 39,
            ResultCode::InvalidExpireInValue => 40,
            ResultCode::InvalidSeasonUnmatchedAmounts => 50,
            ResultCode::InvalidSeasonNoActivePayment => 51,
            ResultCode::InvalidSeason => 52,
            ResultCode::InvalidSeasonInvalidMerchantId => 53,
            ResultCode::InvalidAuthority => 54,
            ResultCode::Verified => 101,
            ResultCode::Unknown(e) => e,
        }
    }
}
