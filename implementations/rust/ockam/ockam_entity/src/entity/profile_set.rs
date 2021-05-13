use crate::{ProfileIdentifier, ProfileTrait, ProfileVault};
use std::borrow::Borrow;

pub struct ProfileSet<V: ProfileVault> {
    profiles: Vec<Box<dyn ProfileTrait<V>>>,
}

pub trait ProfileSetManagement<V: ProfileVault> {
    fn add_profile(&mut self, profile: Box<dyn ProfileTrait<V>>);
    fn remove_profile(&mut self, profile_id: &ProfileIdentifier);
    fn get_profile(&self, profile_id: &ProfileIdentifier) -> Option<&dyn ProfileTrait<V>>;
}

impl<V: ProfileVault> ProfileSetManagement<V> for ProfileSet<V> {
    fn add_profile(&mut self, profile: Box<dyn ProfileTrait<V>>) {
        self.profiles.push(profile);
    }

    fn remove_profile(&mut self, profile_id: &ProfileIdentifier) {
        let mut to_remove: Option<usize> = None;
        for (index, profile) in self.profiles.iter().enumerate() {
            if profile_id == profile.identifier() {
                to_remove = Some(index);
                break;
            }
        }
        if let Some(remove) = to_remove {
            self.profiles.remove(remove);
        }
    }

    fn get_profile(&self, profile_id: &ProfileIdentifier) -> Option<&dyn ProfileTrait<V>> {
        for profile in self.profiles.iter() {
            if profile_id == profile.identifier() {
                return Some(profile.borrow());
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{
        MockProfile, MockVault, ProfileIdentity, ProfileSet, ProfileSetManagement, ProfileTrait,
    };

    #[test]
    fn test_profile_set() {
        let _vault: MockVault = Default::default();
        let default_profile: MockProfile = Default::default();
        let profiles: Vec<Box<dyn ProfileTrait<MockVault>>> = vec![Box::new(default_profile)];
        let mut ps = ProfileSet { profiles };
        assert!(!ps.profiles.is_empty());

        let alice: MockProfile = Default::default();
        let alice_id = alice.identifier().clone();

        ps.add_profile(Box::new(alice));

        let get_alice = ps.get_profile(&alice_id);
        assert!(get_alice.is_some());

        let new_alice = get_alice.unwrap();
        assert_eq!(new_alice.identifier(), &alice_id);

        ps.remove_profile(&alice_id);

        let no_alice = ps.get_profile(&alice_id);
        assert!(no_alice.is_none());
    }
}
