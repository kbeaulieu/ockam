use crate::{ProfileIdentifier, Entity, ProfileIdentity, ProfileVault};

impl<V: ProfileVault> ProfileIdentity for Entity<V> {
    fn identifier(&self) -> &ProfileIdentifier {
        &self.default_profile_identifier
    }
}
