use crate::{ProfileRequestMessage, ProfileResponseMessage, ProfileTrait, ResultMessage};
use async_trait::async_trait;
use ockam_core::{Address, Result, Routed, Worker};
use ockam_node::Context;
use rand::random;

pub struct ProfileWorker<P: ProfileTrait> {
    inner: P,
}

impl<P: ProfileTrait> ProfileWorker<P> {
    fn new(inner: P) -> Self {
        Self { inner }
    }

    pub async fn create_with_inner(ctx: &Context, inner: P) -> Result<Address> {
        let address: Address = random();

        ctx.start_worker(address.clone(), Self::new(inner)).await?;

        Ok(address)
    }

    fn handle_request(&mut self, msg: <Self as Worker>::Message) -> Result<ProfileResponseMessage> {
        Ok(match msg {
            ProfileRequestMessage::GenerateAuthenticationProof { channel_state } => {
                let res = self.inner.generate_authentication_proof(&channel_state)?;
                ProfileResponseMessage::GenerateAuthenticationProof(res)
            }
            ProfileRequestMessage::VerifyAuthenticationProof {
                channel_state,
                responder_contact_id,
                proof,
            } => {
                let res = self.inner.verify_authentication_proof(
                    &channel_state,
                    &responder_contact_id,
                    &proof,
                )?;
                ProfileResponseMessage::VerifyAuthenticationProof(res)
            }
            ProfileRequestMessage::VerifyAndAddContact { contact } => {
                self.inner.verify_and_add_contact(contact)?;
                ProfileResponseMessage::VerifyAndAddContact
            }
            ProfileRequestMessage::ToContact => {
                let res = self.inner.to_contact()?;
                ProfileResponseMessage::ToContact(res)
            }
            ProfileRequestMessage::Contacts => {
                let res = self.inner.contacts()?.clone();
                ProfileResponseMessage::Contacts(res)
            }
        })
    }
}

#[async_trait]
impl<P: ProfileTrait> Worker for ProfileWorker<P> {
    type Message = ProfileRequestMessage;
    type Context = Context;

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let return_route = msg.return_route();
        let response = self.handle_request(msg.body());

        let response = ResultMessage::new(response);

        ctx.send(return_route, response).await?;

        Ok(())
    }
}
