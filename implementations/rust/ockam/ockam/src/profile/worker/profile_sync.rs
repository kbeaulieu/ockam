use crate::{
    Contact, ContactsDb, OckamError, ProfileAuth, ProfileChangeEvent, ProfileContacts,
    ProfileIdentifier, ProfileRequestMessage, ProfileResponseMessage, ProfileTrait, ProfileWorker,
    ResultMessage,
};
use ockam_core::{Address, Result, Route};
use ockam_node::{block_future, Context};
use rand::random;
use tracing::debug;

pub struct ProfileSync {
    ctx: Context,
    profile_worker_address: Address,
}

impl ProfileSync {
    pub(crate) async fn send_message(&self, m: ProfileRequestMessage) -> Result<()> {
        self.ctx
            .send(Route::new().append(self.profile_worker_address.clone()), m)
            .await
    }

    pub(crate) async fn receive_message(&mut self) -> Result<ProfileResponseMessage> {
        self.ctx
            .receive::<ResultMessage<ProfileResponseMessage>>()
            .await?
            .take()
            .body()
            .inner("FIXME")
    }
}

impl Clone for ProfileSync {
    fn clone(&self) -> Self {
        self.start_another().unwrap()
    }
}

impl ProfileSync {
    /// Start another Vault at the same address.
    pub fn start_another(&self) -> Result<Self> {
        let profile_worker_address = self.profile_worker_address.clone();

        let clone = Self::create_with_worker(&self.ctx, &profile_worker_address)?;

        Ok(clone)
    }
}

impl ProfileSync {
    /// Create and start a new Vault using Worker.
    pub fn create_with_worker(ctx: &Context, profile: &Address) -> Result<Self> {
        let address: Address = random();

        debug!("Starting ProfileSync at {}", &address);

        let ctx = block_future(
            &ctx.runtime(),
            async move { ctx.new_context(address).await },
        )?;

        Ok(Self {
            ctx,
            profile_worker_address: profile.clone(),
        })
    }

    pub async fn create<P: ProfileTrait>(ctx: &Context, profile: P) -> Result<Self> {
        let profile_address = ProfileWorker::create_with_inner(ctx, profile).await?;

        Self::create_with_worker(ctx, &profile_address)
    }
}

impl ProfileAuth for ProfileSync {
    fn generate_authentication_proof(&mut self, channel_state: &[u8]) -> Result<Vec<u8>> {
        block_future(&self.ctx.runtime(), async move {
            self.send_message(ProfileRequestMessage::GenerateAuthenticationProof {
                channel_state: channel_state.to_vec(),
            })
            .await?;

            let resp = self.receive_message().await?;

            if let ProfileResponseMessage::GenerateAuthenticationProof(s) = resp {
                Ok(s)
            } else {
                Err(OckamError::ProfileInvalidResponseType.into())
            }
        })
    }

    fn verify_authentication_proof(
        &mut self,
        channel_state: &[u8],
        responder_contact_id: &ProfileIdentifier,
        proof: &[u8],
    ) -> Result<bool> {
        block_future(&self.ctx.runtime(), async move {
            self.send_message(ProfileRequestMessage::VerifyAuthenticationProof {
                channel_state: channel_state.to_vec(),
                responder_contact_id: responder_contact_id.clone(),
                proof: proof.to_vec(),
            })
            .await?;

            let resp = self.receive_message().await?;

            if let ProfileResponseMessage::VerifyAuthenticationProof(s) = resp {
                Ok(s)
            } else {
                Err(OckamError::ProfileInvalidResponseType.into())
            }
        })
    }
}

impl ProfileContacts for ProfileSync {
    fn contacts(&mut self) -> Result<ContactsDb> {
        block_future(&self.ctx.runtime(), async move {
            self.send_message(ProfileRequestMessage::Contacts).await?;

            let resp = self.receive_message().await?;

            if let ProfileResponseMessage::Contacts(s) = resp {
                Ok(s)
            } else {
                Err(OckamError::ProfileInvalidResponseType.into())
            }
        })
    }

    fn to_contact(&mut self) -> Result<Contact> {
        block_future(&self.ctx.runtime(), async move {
            self.send_message(ProfileRequestMessage::ToContact).await?;

            let resp = self.receive_message().await?;

            if let ProfileResponseMessage::ToContact(s) = resp {
                Ok(s)
            } else {
                Err(OckamError::ProfileInvalidResponseType.into())
            }
        })
    }

    fn serialize_to_contact(&mut self) -> Result<Vec<u8>> {
        todo!()
    }

    fn get_contact(&mut self, id: &ProfileIdentifier) -> Result<Option<Contact>> {
        todo!()
    }

    fn verify_contact(&mut self, contact: &Contact) -> Result<()> {
        todo!()
    }

    fn verify_and_add_contact(&mut self, contact: Contact) -> Result<()> {
        todo!()
    }

    fn verify_and_update_contact(
        &mut self,
        profile_id: &ProfileIdentifier,
        change_events: Vec<ProfileChangeEvent>,
    ) -> Result<()> {
        todo!()
    }
}
