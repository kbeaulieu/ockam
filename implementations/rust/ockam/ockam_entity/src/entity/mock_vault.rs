#[cfg(test)]
pub(crate) mod test {
    use ockam_core::Result;
    use ockam_vault::ockam_vault_core::{Secret, SecretAttributes, SmallBuffer};
    use ockam_vault_core::{
        AsymmetricVault, Buffer, Hasher, KeyId, KeyIdVault, PublicKey, SecretKey, SecretVault,
        Signer, SymmetricVault, Verifier,
    };
    use zeroize::Zeroize;

    #[derive(Copy, Clone, Default)]
    pub struct MockVault {}

    impl Zeroize for MockVault {
        fn zeroize(&mut self) {}
    }

    impl SecretVault for MockVault {
        fn secret_generate(&mut self, _attributes: SecretAttributes) -> Result<Secret> {
            todo!()
        }

        fn secret_import(
            &mut self,
            _secret: &[u8],
            _attributes: SecretAttributes,
        ) -> Result<Secret> {
            todo!()
        }

        fn secret_export(&mut self, _context: &Secret) -> Result<SecretKey> {
            todo!()
        }

        fn secret_attributes_get(&mut self, _context: &Secret) -> Result<SecretAttributes> {
            todo!()
        }

        fn secret_public_key_get(&mut self, _context: &Secret) -> Result<PublicKey> {
            todo!()
        }

        fn secret_destroy(&mut self, _context: Secret) -> Result<()> {
            todo!()
        }
    }

    impl SymmetricVault for MockVault {
        fn aead_aes_gcm_encrypt(
            &mut self,
            _context: &Secret,
            _plaintext: &[u8],
            _nonce: &[u8],
            _aad: &[u8],
        ) -> Result<Buffer<u8>> {
            todo!()
        }

        fn aead_aes_gcm_decrypt(
            &mut self,
            _context: &Secret,
            _cipher_text: &[u8],
            _nonce: &[u8],
            _aad: &[u8],
        ) -> Result<Buffer<u8>> {
            todo!()
        }
    }

    impl AsymmetricVault for MockVault {
        fn ec_diffie_hellman(
            &mut self,
            _context: &Secret,
            _peer_public_key: &PublicKey,
        ) -> Result<Secret> {
            todo!()
        }
    }

    impl KeyIdVault for MockVault {
        fn get_secret_by_key_id(&mut self, _key_id: &str) -> Result<Secret> {
            todo!()
        }

        fn compute_key_id_for_public_key(&mut self, _public_key: &PublicKey) -> Result<KeyId> {
            todo!()
        }
    }

    impl Hasher for MockVault {
        fn sha256(&mut self, _data: &[u8]) -> Result<[u8; 32]> {
            todo!()
        }

        fn hkdf_sha256(
            &mut self,
            _salt: &Secret,
            _info: &[u8],
            _ikm: Option<&Secret>,
            _output_attributes: SmallBuffer<SecretAttributes>,
        ) -> Result<SmallBuffer<Secret>> {
            todo!()
        }
    }

    impl Signer for MockVault {
        fn sign(&mut self, _secret_key: &Secret, _data: &[u8]) -> Result<[u8; 64]> {
            todo!()
        }
    }

    impl Verifier for MockVault {
        fn verify(
            &mut self,
            _signature: &[u8; 64],
            _public_key: &PublicKey,
            _data: &[u8],
        ) -> Result<bool> {
            todo!()
        }
    }
}
