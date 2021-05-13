#[cfg(test)]
pub(crate) mod test {
    use crate::{
        Contact, ContactsDb, KeyAttributes, ProfileAuth, ProfileChangeEvent, ProfileChanges,
        ProfileContacts, ProfileEventAttributes, ProfileIdentifier, ProfileIdentity,
        ProfileSecrets, ProfileTrait, ProfileVault, ProfileVaultAccess,
    };
    use ockam_core::Result;
    use ockam_vault_core::{PublicKey, Secret};

    #[derive(Debug)]
    pub struct MockProfile {
        identifier: ProfileIdentifier,
    }

    impl Default for MockProfile {
        fn default() -> Self {
            MockProfile {
                identifier: ProfileIdentifier::random(),
            }
        }
    }

    impl ProfileAuth for MockProfile {
        fn generate_authentication_proof(&mut self, _channel_state: &[u8]) -> Result<Vec<u8>> {
            todo!()
        }

        fn verify_authentication_proof(
            &mut self,
            _channel_state: &[u8],
            _responder_contact_id: &ProfileIdentifier,
            _proof: &[u8],
        ) -> Result<bool> {
            todo!()
        }
    }

    impl ProfileChanges for MockProfile {
        fn change_events(&self) -> &[ProfileChangeEvent] {
            todo!()
        }

        fn update_no_verification(&mut self, _change_event: ProfileChangeEvent) -> Result<()> {
            todo!()
        }

        fn verify(&mut self) -> Result<()> {
            todo!()
        }
    }

    impl ProfileContacts for MockProfile {
        fn contacts(&self) -> &ContactsDb {
            todo!()
        }

        fn to_contact(&self) -> Contact {
            todo!()
        }

        fn serialize_to_contact(&self) -> Result<Vec<u8>> {
            todo!()
        }

        fn get_contact(&self, _id: &ProfileIdentifier) -> Option<&Contact> {
            todo!()
        }

        fn verify_contact(&mut self, _contact: &Contact) -> Result<()> {
            todo!()
        }

        fn verify_and_add_contact(&mut self, _contact: Contact) -> Result<()> {
            todo!()
        }

        fn verify_and_update_contact(
            &mut self,
            _profile_id: &ProfileIdentifier,
            _change_events: Vec<ProfileChangeEvent>,
        ) -> Result<()> {
            todo!()
        }
    }

    impl ProfileIdentity for MockProfile {
        fn identifier(&self) -> &ProfileIdentifier {
            &self.identifier
        }
    }

    impl ProfileSecrets for MockProfile {
        fn create_key(
            &mut self,
            _key_attributes: KeyAttributes,
            _attributes: Option<ProfileEventAttributes>,
        ) -> Result<()> {
            todo!()
        }

        fn rotate_key(
            &mut self,
            _key_attributes: KeyAttributes,
            _attributes: Option<ProfileEventAttributes>,
        ) -> Result<()> {
            todo!()
        }

        fn get_secret_key(&mut self, _key_attributes: &KeyAttributes) -> Result<Secret> {
            todo!()
        }

        fn get_public_key(&self, _key_attributes: &KeyAttributes) -> Result<PublicKey> {
            todo!()
        }

        fn get_root_secret(&mut self) -> Result<Secret> {
            todo!()
        }
    }

    impl<V: ProfileVault> ProfileVaultAccess<V> for MockProfile {
        fn vault(&mut self) -> V {
            todo!()
        }
    }

    impl<V: ProfileVault> ProfileTrait<V> for MockProfile {}
}
