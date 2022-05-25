// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    constants::SessionType,
    context::handle_manager::HandleDropAction,
    handles::{KeyHandle, ObjectHandle, SessionHandle},
    interface_types::{
        algorithm::HashingAlgorithm,
        session_handles::{AuthSession, PolicySession},
    },
    structures::{Nonce, SymmetricDefinition},
    tss2_esys::*,
    Context, Error, Result,
};
use log::error;
use std::{convert::TryInto, ptr::null};
impl Context {
    /// Start new authentication session and return the Session object
    /// associated with the session.
    ///
    /// If the returned session handle from ESYS api is ESYS_TR_NONE then
    /// the value of the option in the result will be None.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tss_esapi::{Context, Tcti,
    /// #     constants::SessionType,
    /// #     interface_types::algorithm::HashingAlgorithm,
    /// #     structures::SymmetricDefinition,
    /// # };
    /// # // Create context
    /// # let mut context =
    /// #     Context::new(
    /// #         Tcti::from_environment_variable().expect("Failed to get TCTI"),
    /// #     ).expect("Failed to create Context");
    /// // Create auth session without key_handle, bind_handle
    /// // and Nonce
    /// let session = context
    ///     .start_auth_session(
    ///         None,
    ///         None,
    ///         None,
    ///         SessionType::Hmac,
    ///         SymmetricDefinition::AES_256_CFB,
    ///         HashingAlgorithm::Sha256,
    ///     )
    ///     .expect("Failed to create session")
    ///     .expect("Recived invalid handle");
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn start_auth_session(
        &mut self,
        tpm_key: Option<KeyHandle>,
        bind: Option<ObjectHandle>,
        nonce: Option<&Nonce>,
        session_type: SessionType,
        symmetric: SymmetricDefinition,
        auth_hash: HashingAlgorithm,
    ) -> Result<Option<AuthSession>> {
        let potential_tpm2b_nonce = nonce.map(|v| v.clone().into());

        let mut esys_session_handle = ESYS_TR_NONE;

        let ret = unsafe {
            Esys_StartAuthSession(
                self.mut_context(),
                tpm_key.map(|v| v.into()).unwrap_or(ESYS_TR_NONE),
                bind.map(|v| v.into()).unwrap_or(ESYS_TR_NONE),
                self.optional_session_1(),
                self.optional_session_2(),
                self.optional_session_3(),
                potential_tpm2b_nonce.as_ref().map_or_else(null, |v| v),
                session_type.into(),
                &symmetric.try_into()?,
                auth_hash.into(),
                &mut esys_session_handle,
            )
        };

        let ret = Error::from_tss_rc(ret);
        if ret.is_success() {
            self.handle_manager.add_handle(
                ObjectHandle::from(esys_session_handle),
                HandleDropAction::Flush,
            )?;
            Ok(AuthSession::create(
                session_type,
                SessionHandle::from(esys_session_handle),
                auth_hash,
            ))
        } else {
            error!("Error when creating a session: {}", ret);
            Err(ret)
        }
    }

    /// Restart the TPM Policy
    pub fn policy_restart(&mut self, policy_session: PolicySession) -> Result<()> {
        let ret = unsafe {
            Esys_PolicyRestart(
                self.mut_context(),
                SessionHandle::from(policy_session).into(),
                self.optional_session_1(),
                self.optional_session_2(),
                self.optional_session_3(),
            )
        };
        let ret = Error::from_tss_rc(ret);

        if ret.is_success() {
            Ok(())
        } else {
            error!("Error restarting policy: {}", ret);
            Err(ret)
        }
    }
}
