use crate::EntityError::ProfileNotFound;
use crate::{Entity, ProfileChangeEvent, ProfileChanges, ProfileVault, ProfileIdentity};
use ockam_core::Result;

impl<V: ProfileVault> ProfileChanges for Entity<V> {
    fn change_events(&self) -> &[ProfileChangeEvent] {
        if let Some(profile) = self.default_profile() {
            profile.change_events()
        } else {
            &[]
        }
    }

    fn update_no_verification(&mut self, change_event: ProfileChangeEvent) -> Result<()> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.update_no_verification(change_event);
            }
        }
        Err(ProfileNotFound.into())
    }

    fn verify(&mut self) -> Result<()> {
        if let Some(profile) = self.default_profile() {
            profile.clone().verify()
        } else {
            Err(ProfileNotFound.into())
        }
    }
}
