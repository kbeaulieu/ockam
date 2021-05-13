use crate::EntityError::ProfileNotFound;
use crate::{Entity, ProfileAuth, ProfileIdentifier, ProfileVault};
use ockam_core::Result;

impl<V: ProfileVault> ProfileAuth for Entity<V> {
    fn generate_authentication_proof(&mut self, channel_state: &[u8]) -> Result<Vec<u8>> {
        if let Some(profile) = self.default_profile() {
            profile.clone().generate_authentication_proof(channel_state)
        } else {
            Err(ProfileNotFound.into())
        }
    }

    fn verify_authentication_proof(
        &mut self,
        channel_state: &[u8],
        responder_contact_id: &ProfileIdentifier,
        proof: &[u8],
    ) -> Result<bool> {
        if let Some(profile) = self.default_profile() {
            profile
                .clone()
                .verify_authentication_proof(channel_state, responder_contact_id, proof)
        } else {
            Err(ProfileNotFound.into())
        }
    }
}
