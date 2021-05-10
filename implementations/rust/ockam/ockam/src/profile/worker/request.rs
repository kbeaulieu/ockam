use crate::{Contact, ProfileIdentifier};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProfileRequestMessage {
    GenerateAuthenticationProof {
        channel_state: Vec<u8>,
    },
    VerifyAuthenticationProof {
        channel_state: Vec<u8>,
        responder_contact_id: ProfileIdentifier,
        proof: Vec<u8>,
    },
    VerifyAndAddContact {
        contact: Contact,
    },
    ToContact,
    Contacts,
}
