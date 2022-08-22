// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    structures::{Auth, SensitiveData},
    traits::{InPlaceFfiDataZeroizer, Marshall, UnMarshall},
    tss2_esys::{TPM2B_SENSITIVE_CREATE, TPMS_SENSITIVE_CREATE},
    Error, Result, ReturnCode, WrapperErrorKind,
};
use log::error;
use std::convert::{TryFrom, TryInto};
use zeroize::Zeroize;

/// Structure that defines the values to be placed in the sensitive
/// area of a created object.
///
/// # Details
/// This corresponds to the TPMS_SENSITIVE_CREATE
/// structure.
///
/// There is a corresponding buffer type [SensitiveCreateBuffer](crate::structures::SensitiveCreateBuffer)
/// that holds the data in a marshalled form.
#[derive(Debug, Clone, Eq, PartialEq, Zeroize)]
pub struct SensitiveCreate {
    user_auth: Auth,
    data: SensitiveData,
}

impl SensitiveCreate {
    /// Creates new SensitiveCreate.
    pub const fn new(user_auth: Auth, data: SensitiveData) -> Self {
        SensitiveCreate { user_auth, data }
    }

    /// Returns the user auth
    pub const fn user_auth(&self) -> &Auth {
        &self.user_auth
    }

    /// Returns the sensitive data
    pub const fn data(&self) -> &SensitiveData {
        &self.data
    }
}

impl InPlaceFfiDataZeroizer<TPMS_SENSITIVE_CREATE> for SensitiveCreate {
    fn zeroize_ffi_data_in_place(ffi_data: &mut TPMS_SENSITIVE_CREATE) {
        Auth::zeroize_ffi_data_in_place(&mut ffi_data.userAuth);
        SensitiveData::zeroize_ffi_data_in_place(&mut ffi_data.data);
    }
}

impl From<SensitiveCreate> for TPMS_SENSITIVE_CREATE {
    fn from(sensitive_create: SensitiveCreate) -> Self {
        TPMS_SENSITIVE_CREATE {
            userAuth: sensitive_create.user_auth.into(),
            data: sensitive_create.data.into(),
        }
    }
}

impl TryFrom<TPMS_SENSITIVE_CREATE> for SensitiveCreate {
    type Error = Error;

    fn try_from(tpms_sensitive_create: TPMS_SENSITIVE_CREATE) -> Result<Self> {
        Ok(SensitiveCreate {
            user_auth: tpms_sensitive_create.userAuth.try_into()?,
            data: tpms_sensitive_create.data.try_into()?,
        })
    }
}

impl Marshall for SensitiveCreate {
    const BUFFER_SIZE: usize = std::mem::size_of::<TPMS_SENSITIVE_CREATE>();

    /// Produce a marshalled [TPMS_SENSITIVE_CREATE]
    ///
    /// Note: for [TPM2B_SENSITIVE_CREATE] marshalling use [SensitiveCreateBuffer][`crate::structures::SensitiveCreateBuffer]
    fn marshall(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0; Self::BUFFER_SIZE];
        let mut offset = 0;

        ReturnCode::ensure_success(
            unsafe {
                crate::tss2_esys::Tss2_MU_TPMS_SENSITIVE_CREATE_Marshal(
                    &self.clone().into(),
                    buffer.as_mut_ptr(),
                    Self::BUFFER_SIZE.try_into().map_err(|e| {
                        error!("Failed to convert size of buffer to TSS size_t type: {}", e);
                        Error::local_error(WrapperErrorKind::InvalidParam)
                    })?,
                    &mut offset,
                )
            },
            |ret| {
                error!("Failed to marshal SensitiveCreate: {}", ret);
            },
        )?;

        let checked_offset = usize::try_from(offset).map_err(|e| {
            error!("Failed to parse offset as usize: {}", e);
            Error::local_error(WrapperErrorKind::InvalidParam)
        })?;

        buffer.truncate(checked_offset);

        Ok(buffer)
    }
}

impl UnMarshall for SensitiveCreate {
    /// Unmarshall the structure from [`TPMS_SENSITIVE_CREATE`]
    ///
    /// Note: for [TPM2B_SENSITIVE_CREATE] unmarshalling use [SensitiveCreateBuffer][`crate::structures::SensitiveCreateBuffer]
    fn unmarshall(marshalled_data: &[u8]) -> Result<Self> {
        let mut dest = TPMS_SENSITIVE_CREATE::default();
        let mut offset = 0;

        ReturnCode::ensure_success(
            unsafe {
                crate::tss2_esys::Tss2_MU_TPMS_SENSITIVE_CREATE_Unmarshal(
                    marshalled_data.as_ptr(),
                    marshalled_data.len().try_into().map_err(|e| {
                        error!("Failed to convert length of marshalled data: {}", e);
                        Error::local_error(WrapperErrorKind::InvalidParam)
                    })?,
                    &mut offset,
                    &mut dest,
                )
            },
            |ret| error!("Failed to unmarshal SensitiveCreate: {}", ret),
        )?;

        SensitiveCreate::try_from(dest)
    }
}

impl TryFrom<TPM2B_SENSITIVE_CREATE> for SensitiveCreate {
    type Error = Error;

    fn try_from(tpm2b_sensitive_create: TPM2B_SENSITIVE_CREATE) -> Result<Self> {
        SensitiveCreate::try_from(tpm2b_sensitive_create.sensitive)
    }
}

impl TryFrom<SensitiveCreate> for TPM2B_SENSITIVE_CREATE {
    type Error = Error;

    fn try_from(sensitive_create: SensitiveCreate) -> Result<Self> {
        let mut buffer = vec![0; SensitiveCreate::BUFFER_SIZE];
        let mut size = 0;
        let sensitive = TPMS_SENSITIVE_CREATE::from(sensitive_create);

        ReturnCode::ensure_success(
            unsafe {
                crate::tss2_esys::Tss2_MU_TPMS_SENSITIVE_CREATE_Marshal(
                    &sensitive,
                    buffer.as_mut_ptr(),
                    SensitiveCreate::BUFFER_SIZE.try_into().map_err(|e| {
                        error!("Failed to convert size of buffer to TSS size_t type: {}", e);
                        Error::local_error(WrapperErrorKind::InvalidParam)
                    })?,
                    &mut size,
                )
            },
            |ret| error!("Failed to marshal SensitiveCreate: {}", ret),
        )?;

        Ok(TPM2B_SENSITIVE_CREATE {
            size: size.try_into().map_err(|e| {
                error!(
                    "Failed to convert size of buffer from TSS size_t type: {}",
                    e
                );
                Error::local_error(WrapperErrorKind::InvalidParam)
            })?,
            sensitive,
        })
    }
}
