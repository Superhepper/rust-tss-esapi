// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    constants::{tss::*, CapabilityType, PropertyTag},
    handles::{NvIndexHandle, NvIndexTpmHandle, ObjectHandle, TpmHandle},
    interface_types::resource_handles::NvAuth,
    nv::storage::NvPublic,
    structures::{CapabilityData, Name},
    Context, Error, Result, WrapperErrorKind,
};

/// Allows reading an NV Index completely, regardless of the max TPM NV buffer size
pub fn read_full(
    context: &mut Context,
    auth_handle: NvAuth,
    nv_index_handle: NvIndexTpmHandle,
) -> Result<Vec<u8>> {
    let maxsize = context
        .get_tpm_property(PropertyTag::NvBufferMax)?
        .unwrap_or(512) as usize;

    let nv_idx = TpmHandle::NvIndex(nv_index_handle);
    let nv_idx = context.execute_without_session(|ctx| ctx.tr_from_tpm_public(nv_idx))?;
    let nv_idx: NvIndexHandle = nv_idx.into();

    let (nvpub, _) = context.execute_without_session(|ctx| ctx.nv_read_public(nv_idx))?;
    let nvsize = nvpub.data_size();

    let mut result = Vec::new();
    result.reserve_exact(nvsize);

    for offset in (0..nvsize).step_by(maxsize) {
        let size: u16 = std::cmp::min(maxsize, nvsize - offset) as u16;

        let res = context.nv_read(auth_handle, nv_idx, size, offset as u16)?;
        result.extend_from_slice(&res);
    }

    Ok(result)
}

/// Lists all the currently defined NV Indexes' names and public components
pub fn list(context: &mut Context) -> Result<Vec<(NvPublic, Name)>> {
    context.execute_without_session(|ctx| {
        let (handles, _) = ctx.get_capability(
            CapabilityType::Handles,
            TPM2_NV_INDEX_FIRST,
            TPM2_PT_NV_INDEX_MAX,
        )?;
        let handles = match handles {
            CapabilityData::Handles(handles) => handles,
            _ => return Err(Error::local_error(WrapperErrorKind::WrongValueFromTpm)),
        };
        handles
            .iter()
            .map(|h| ctx.tr_from_tpm_public(*h))
            .collect::<Result<Vec<ObjectHandle>>>()?
            .iter()
            .map(|h| NvIndexHandle::from(*h))
            .map(|h| ctx.nv_read_public(h))
            .collect()
    })
}
