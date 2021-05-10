use crate::{Contact, ContactsDb};
use ockam_core::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResultMessage<M> {
    inner: Result<M, u32>,
}

impl<M> ResultMessage<M>
where
    M: Message,
{
    pub fn inner(self, error_domain: &'static str) -> ockam_core::Result<M> {
        self.inner
            .map_err(|e| ockam_core::Error::new(e, error_domain))
    }
}

impl<M> ResultMessage<M>
where
    M: Message,
{
    pub fn new(inner: ockam_core::Result<M>) -> Self {
        Self {
            inner: inner.map_err(|e| e.code()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProfileResponseMessage {
    GenerateAuthenticationProof(Vec<u8>),
    VerifyAuthenticationProof(bool),
    VerifyAndAddContact,
    ToContact(Contact),
    Contacts(ContactsDb),
}
