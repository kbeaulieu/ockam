use crate::{
    Contact, ContactsDb, KeyAttributes, ProfileAuth, ProfileChangeEvent, ProfileChanges,
    ProfileContacts, ProfileEventAttributes, ProfileIdentifier, ProfileIdentity, ProfileImpl,
    ProfileSecrets, ProfileTrait, ProfileVault,
};

use crate::EntityError::ProfileNotFound;
use ockam_core::Result;
use ockam_vault::ockam_vault_core::{PublicKey, Secret};

#[derive(Clone)]
pub struct Entity<V: ProfileVault> {
    default_profile_identifier: ProfileIdentifier,
    profiles: Vec<ProfileImpl<V>>,
}

impl<V: ProfileVault> Entity<V> {
    #[allow(dead_code)]
    fn new(default_profile: ProfileImpl<V>) -> Self {
        let idref = default_profile.identifier();
        let default_profile_identifier = ProfileIdentifier::from_key_id(idref.key_id().clone());
        let profiles = vec![default_profile];
        Entity {
            default_profile_identifier,
            profiles,
        }
    }

    fn default_profile(&self) -> Option<&ProfileImpl<V>> {
        self.profiles
            .iter()
            .find(|profile| &self.default_profile_identifier == profile.identifier())
    }
}

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

impl<V: ProfileVault> ProfileIdentity for Entity<V> {
    fn identifier(&self) -> &ProfileIdentifier {
        &self.default_profile_identifier
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

impl<V: ProfileVault> ProfileTrait for Entity<V> {}
#[cfg(test)]
#[allow(unreachable_code, unused_variables)]
mod test {
    use crate::{
        Entity, KeyAttributes, Profile, ProfileAuth, ProfileContacts, ProfileIdentity, ProfileImpl,
        ProfileSecrets, ProfileVault,
    };
    use ockam_node::Context;
    use ockam_vault_sync_core::{Vault, VaultSync};

    async fn new_entity(ctx: &Context) -> ockam_core::Result<Entity<VaultSync>> {
        let vault = Vault::create(ctx)?;
        let vault = VaultSync::create_with_worker(ctx, &vault)?;

        // todo: test vault that allows this?: let vault = TestVault::default();
        let profile = ProfileImpl::create_internal(None, vault);
        assert!(profile.is_ok());

        let profile = profile.unwrap();
        Ok(Entity::new(profile))
    }

    #[test]
    fn test_new_entity() {
        let (mut ctx, mut executor) = ockam_node::start_node();
        executor
            .execute(async move {
                let e = new_entity(&ctx).await.unwrap();
                assert!(!e
                    .default_profile_identifier
                    .to_string_representation()
                    .is_empty());
                assert!(!e.profiles.is_empty());

                let default = e.default_profile();

                assert!(default.is_some());
                ctx.stop().await.unwrap();
            })
            .unwrap();
    }

    fn entity_auth_tests<V: ProfileVault>(mut e: Entity<V>) -> ockam_core::Result<()> {
        let channel_state = "test".as_bytes();
        let proof = e.generate_authentication_proof(channel_state);
        assert!(proof.is_ok());

        let proof = proof.unwrap();

        // TODO WIP: Need Contacts for this test to be successful. This tests the delegation but not correct operation currently.
        let default_id = e.default_profile_identifier.clone();
        let valid = e.verify_authentication_proof(channel_state, &default_id, proof.as_slice());
        // assert!(valid.is_ok());
        Ok(())
    }

    fn entity_change_tests<V: ProfileVault>(e: Entity<V>) -> ockam_core::Result<()> {
        // TODO WIP: Need key ops and other event generating APIs to easily test this
        // change_events update_no_verification verify
        Ok(())
    }

    async fn entity_contacts_tests<V: ProfileVault>(
        ctx: &Context,
        mut e: Entity<V>,
    ) -> ockam_core::Result<()> {
        //    verify_and_update_contact

        let alice = new_entity(&ctx).await.unwrap();
        let alice_id = alice.identifier().clone();

        let alice_contact = alice.serialize_to_contact()?;
        let alice_contact = Profile::deserialize_contact(alice_contact.as_slice())?;

        let to_alice_contact = alice.to_contact();
        assert_eq!(alice_contact.identifier(), to_alice_contact.identifier());

        e.verify_contact(&alice_contact)?;

        e.verify_and_add_contact(alice_contact)?;

        assert_eq!(1, e.contacts().len());

        let get_alice_contact = e.get_contact(&alice_id);
        assert!(get_alice_contact.is_some());

        let get_alice_contact = get_alice_contact.unwrap();
        assert_eq!(&alice_id, get_alice_contact.identifier());

        // TODO WIP: after change event emitting APIs are done, make this a non-trivial test
        let change_events = vec![];
        e.verify_and_update_contact(&alice_id, change_events)?;
        Ok(())
    }

    fn entity_secrets_test<V: ProfileVault>(mut e: Entity<V>) -> ockam_core::Result<()> {
        //   get_secret_key  get_root_secret rotate_key

        let key_attributes = KeyAttributes::new("label".to_string());
        e.create_key(key_attributes.clone(), None)?;

        let pubkey = e.get_public_key(&key_attributes)?;
        let secret = e.get_secret_key(&key_attributes)?;
        let root = e.get_root_secret()?;

        let root_key_attributes = KeyAttributes::new(Profile::PROFILE_UPDATE.to_string());

        e.rotate_key(root_key_attributes, None)?;

        /* Uncomment once rotate_key is implemented
        let new_pubkey = e.get_public_key(&key_attributes)?;

        let new_secret = e.get_secret_key(&key_attributes)?;

        assert_ne!(new_pubkey, pubkey);
        assert_ne!(new_secret, secret);
         */
        Ok(())
    }

    #[test]
    fn test_entity_default_profile_delegation() {
        let (mut ctx, mut executor) = ockam_node::start_node();
        executor
            .execute(async move {
                let e = new_entity(&ctx).await.unwrap();
                entity_contacts_tests(&ctx, e.clone()).await.unwrap();
                entity_auth_tests(e.clone()).unwrap();
                entity_change_tests(e.clone()).unwrap();
                entity_secrets_test(e).unwrap();

                ctx.stop().await.unwrap();
            })
            .unwrap();
    }
}
