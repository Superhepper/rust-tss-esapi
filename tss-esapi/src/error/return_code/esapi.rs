// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    constants::return_code::BaseError, error::return_code::BaseReturnCode, Error, Result,
    WrapperErrorKind,
};
use log::error;
use std::convert::TryFrom;

/// struct representing the error codes generated by the ESAPI layer
/// in TSS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EsapiReturnCode {
    base_error: BaseError,
}

impl EsapiReturnCode {
    /// Returns the [BaseError] associated with the ESAPI return code.
    pub const fn base_error(&self) -> BaseError {
        self.base_error
    }
}

impl From<EsapiReturnCode> for BaseReturnCode {
    fn from(esapi_return_code: EsapiReturnCode) -> Self {
        esapi_return_code.base_error.into()
    }
}

impl TryFrom<BaseReturnCode> for EsapiReturnCode {
    type Error = Error;

    fn try_from(base_return_code: BaseReturnCode) -> Result<Self> {
        EsapiReturnCode::try_from(BaseError::from(base_return_code))
    }
}

impl TryFrom<u16> for EsapiReturnCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        EsapiReturnCode::try_from(BaseError::try_from(value)?)
    }
}

impl From<EsapiReturnCode> for u16 {
    fn from(esapi_error_code: EsapiReturnCode) -> Self {
        BaseReturnCode::from(esapi_error_code).into()
    }
}

impl TryFrom<BaseError> for EsapiReturnCode {
    type Error = Error;

    fn try_from(base_error: BaseError) -> Result<Self> {
        match base_error {
            BaseError::GeneralFailure
            | BaseError::NotImplemented
            | BaseError::BadContext
            | BaseError::AbiMismatch
            | BaseError::BadReference
            | BaseError::BadSequence
            | BaseError::TryAgain
            | BaseError::BadValue
            | BaseError::NoDecryptParam
            | BaseError::NoEncryptParam
            | BaseError::MalformedResponse
            | BaseError::InsufficientResponse
            | BaseError::IncompatibleTcti
            | BaseError::BadTctiStructure
            | BaseError::Memory
            | BaseError::BadTr
            | BaseError::MultipleDecryptSessions
            | BaseError::MultipleEncryptSessions
            | BaseError::NotSupported => Ok(EsapiReturnCode { base_error }),
            _ => {
                error!(
                    "{} is not a valid EsapiReturnCode base error",
                    u16::from(base_error)
                );
                Err(Error::local_error(WrapperErrorKind::InvalidParam))
            }
        }
    }
}

impl From<EsapiReturnCode> for BaseError {
    fn from(esapi_return_code: EsapiReturnCode) -> Self {
        esapi_return_code.base_error
    }
}

impl std::error::Error for EsapiReturnCode {}

impl std::fmt::Display for EsapiReturnCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BaseReturnCode::from(*self))
    }
}