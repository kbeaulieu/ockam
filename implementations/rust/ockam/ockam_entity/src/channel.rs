use crate::{ProfileTrait, ProfileVault};
use ockam_core::{Address, Result, Route};
use ockam_node::Context;

mod responder;
pub(crate) use responder::*;
mod initiator;
pub(crate) use initiator::*;
mod listener;
pub(crate) use listener::*;
mod messages;
pub(crate) use messages::*;

#[allow(dead_code)]
struct SecureChannel {}

impl SecureChannel {
    /// Create mutually authenticated secure channel
    #[allow(dead_code)]
    pub async fn create_secure_channel<R, V, P>(
        ctx: &Context,
        profile: &mut P,
        route: R,
    ) -> Result<Address>
    where
        R: Into<Route>,
        V: ProfileVault,
        P: ProfileTrait<V>,
    {
        Initiator::create(ctx, route, profile).await
    }

    /// Create mutually authenticated secure channel listener
    #[allow(dead_code)]
    pub async fn create_secure_channel_listener<A, V, P>(
        ctx: &Context,
        mut profile: P,
        address: A,
    ) -> Result<()>
    where
        A: Into<Address>,
        V: ProfileVault,
        P: ProfileTrait<V>,
    {
        let vault = profile.vault();
        let listener = ProfileChannelListener {
            profile,
            vault,
            listener_address: None,
        };
        ctx.start_worker(address.into(), listener).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Entity, Profile};
    use ockam_vault_sync_core::Vault;

    #[test]
    fn test_channel() {
        let (mut ctx, mut executor) = ockam_node::start_node();
        executor
            .execute(async move {
                let vault = Vault::create(&ctx).unwrap();

                let alice = Profile::create(&ctx, &vault).unwrap();
                let bob = Profile::create(&ctx, &vault).unwrap();

                let bob = Entity::new(bob);
                let mut alice = Entity::new(alice);

                SecureChannel::create_secure_channel_listener(&ctx, bob, "bob_listener")
                    .await
                    .unwrap();

                let alice_channel = SecureChannel::create_secure_channel(
                    &ctx,
                    &mut alice,
                    Route::new().append("bob_listener"),
                )
                .await
                .unwrap();

                ctx.send(
                    Route::new().append(alice_channel).append(ctx.address()),
                    "Hello, Bob!".to_string(),
                )
                .await
                .unwrap();
                let msg = ctx.receive::<String>().await.unwrap().take();
                let return_route = msg.return_route();
                assert_eq!("Hello, Bob!", msg.body());

                ctx.send(return_route, "Hello, Alice!".to_string())
                    .await
                    .unwrap();
                assert_eq!(
                    "Hello, Alice!",
                    ctx.receive::<String>().await.unwrap().take().body()
                );

                ctx.stop().await.unwrap();
            })
            .unwrap();
    }
}
