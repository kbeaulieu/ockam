use crate::EntityError::ProfileNotFound;
use crate::{
    Entity, KeyAttributes, ProfileEventAttributes, ProfileIdentity, ProfileSecrets, ProfileVault,
    ProfileVaultAccess,
};
use ockam_core::Result;
use ockam_vault_core::{PublicKey, Secret};

impl<V: ProfileVault> ProfileVaultAccess<V> for Entity<V> {
    fn vault(&mut self) -> V {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.vault();
            }
        }
        panic!("Entity has no default profile")
    }
}

impl<V: ProfileVault> ProfileSecrets for Entity<V> {
    fn create_key(
        &mut self,
        key_attributes: KeyAttributes,
        attributes: Option<ProfileEventAttributes>,
    ) -> Result<()> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.create_key(key_attributes, attributes);
            }
        }
        Err(ProfileNotFound.into())
    }

    fn rotate_key(
        &mut self,
        key_attributes: KeyAttributes,
        attributes: Option<ProfileEventAttributes>,
    ) -> Result<()> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.rotate_key(key_attributes, attributes);
            }
        }
        Err(ProfileNotFound.into())
    }

    fn get_secret_key(&mut self, key_attributes: &KeyAttributes) -> Result<Secret> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.get_secret_key(key_attributes);
            }
        }
        Err(ProfileNotFound.into())
    }

    fn get_public_key(&self, key_attributes: &KeyAttributes) -> Result<PublicKey> {
        if let Some(profile) = self.default_profile() {
            profile.get_public_key(key_attributes)
        } else {
            Err(ProfileNotFound.into())
        }
    }

    fn get_root_secret(&mut self) -> Result<Secret> {
        for profile in &mut self.profiles {
            if &self.default_profile_identifier == profile.identifier() {
                return profile.get_root_secret();
            }
        }
        Err(ProfileNotFound.into())
    }
}
