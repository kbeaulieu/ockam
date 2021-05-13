use crate::EntityError::ProfileNotFound;
use crate::{
    Contact, ContactsDb, Entity, ProfileChangeEvent, ProfileContacts, ProfileIdentifier,
    ProfileIdentity, ProfileVault,
};
use ockam_core::Result;

impl<V: ProfileVault> ProfileContacts for Entity<V> {
    fn contacts(&self) -> &ContactsDb {
        if let Some(profile) = self.default_profile() {
            profile.contacts()
        } else {
            /* Since the return value is a reference, and we have no default profile, panic.
            We could potentially store an empty contacts db somewhere and return it here, but that
            seems like a waste. */
            panic!("Entity has no default profile")
        }
    }

    fn to_contact(&self) -> Contact {
        if let Some(profile) = self.default_profile() {
            profile.to_contact()
        } else {
            panic!("Entity has no default profile")
        }
    }

    fn serialize_to_contact(&self) -> Result<Vec<u8>> {
        if let Some(profile) = self.default_profile() {
            profile.serialize_to_contact()
        } else {
            Err(ProfileNotFound.into())
        }
    }

    fn get_contact(&self, id: &ProfileIdentifier) -> Option<&Contact> {
        if let Some(profile) = self.default_profile() {
            profile.get_contact(id)
        } else {
            None
        }
    }

    fn verify_contact(&mut self, contact: &Contact) -> Result<()> {
        if let Some(profile) = self.default_profile() {
            profile.clone().verify_contact(contact)
        } else {
            Err(ProfileNotFound.into())
        }
    }

    fn verify_and_add_contact(&mut self, contact: Contact) -> Result<()> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.verify_and_add_contact(contact);
            }
        }
        Err(ProfileNotFound.into())
    }

    fn verify_and_update_contact(
        &mut self,
        profile_id: &ProfileIdentifier,
        change_events: Vec<ProfileChangeEvent>,
    ) -> Result<()> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.verify_and_update_contact(profile_id, change_events);
            }
        }
        Err(ProfileNotFound.into())
    }
}
