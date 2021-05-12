use crate::{ProfileTrait, ProfileVault, Responder};
use ockam_channel::{CreateResponderChannelMessage, SecureChannel};
use ockam_core::{Address, Result, Routed, Worker};
use ockam_key_exchange_xx::XXNewKeyExchanger;
use ockam_node::Context;
use rand::random;

pub(crate) struct ProfileChannelListener<V: ProfileVault, P: ProfileTrait<V>> {
    pub(crate) profile: P, // TODO: Avoid copying profile
    pub(crate) vault: V,
    pub(crate) listener_address: Option<Address>,
}

#[ockam_core::worker]
impl<V: ProfileVault, P: ProfileTrait<V>> Worker for ProfileChannelListener<V, P> {
    type Message = CreateResponderChannelMessage;
    type Context = Context;

    async fn initialize(&mut self, ctx: &mut Self::Context) -> Result<()> {
        let listener_address: Address = random();
        let new_key_exchanger = XXNewKeyExchanger::new(self.vault.clone());
        let vault = self.vault.clone();
        SecureChannel::create_listener_extended(
            ctx,
            listener_address.clone(),
            new_key_exchanger,
            vault,
        )
        .await?;

        self.listener_address = Some(listener_address);

        Ok(())
    }

    async fn shutdown(&mut self, ctx: &mut Self::Context) -> Result<()> {
        ctx.stop_worker(self.listener_address.take().unwrap()).await
    }

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        Responder::create(
            ctx,
            &mut self.profile,
            self.listener_address.clone().unwrap(),
            msg,
        )
        .await
    }
}
