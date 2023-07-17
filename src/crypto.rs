use std::sync::Arc;

use openssl::{rsa, sha, symm};
use openssl::rsa::Padding;

const SALT: [u8; 16] = [
    102, 0, 21, 56, 172, 34, 123, 32, 45, 250, 6, 42, 80, 66, 190, 200,
];

#[derive(Clone)]
pub struct KeyManager {
    aes_key: Arc<String>,
    rsa_pub: Arc<String>,
    rsa_priv: Arc<String>,
}

impl KeyManager {
    pub fn new(aes_key: String, rsa_pub: String, rsa_priv: String) -> Self {
        KeyManager {
            aes_key: Arc::new(aes_key),
            rsa_pub: Arc::new(rsa_pub),
            rsa_priv: Arc::new(rsa_priv),
        }
    }

    pub fn encrypt_aes(&self, data: &[u8]) -> Vec<u8> {
        let cipher = symm::Cipher::aes_256_ctr();
        let key = sha::sha256(self.aes_key.as_bytes());

        symm::encrypt(cipher, &key, Some(&SALT), data).unwrap()
    }

    pub fn decrypt_aes(&self, data: &[u8]) -> Vec<u8> {
        let cipher = symm::Cipher::aes_256_ctr();
        let key = sha::sha256(self.aes_key.as_bytes());

        symm::decrypt(cipher, &key, Some(&SALT), data).unwrap()
    }

    pub fn encrypt_rsa(&self, data: &[u8]) -> Vec<u8> {
        let key = rsa::Rsa::public_key_from_pem_pkcs1(self.rsa_pub.as_bytes()).unwrap();
        let mut encrypted = Vec::new();
        for chunk in data.chunks(128) {
            let mut buf = [0; 256];
            key.public_encrypt(chunk, &mut buf, Padding::PKCS1_OAEP)
                .unwrap();
            encrypted.extend_from_slice(&buf);
        }

        encrypted
    }

    pub fn decrypt_rsa(&self, data: &[u8]) -> Vec<u8> {
        let key = rsa::Rsa::private_key_from_pem(self.rsa_priv.as_bytes()).unwrap();
        let mut decrypted = Vec::new();
        for chunk in data.chunks(256) {
            let mut buf = [0; 256];
            let read = key
                .private_decrypt(chunk, &mut buf, Padding::PKCS1_OAEP)
                .unwrap();
            decrypted.extend_from_slice(&buf[..read]);
        }

        decrypted
    }
}
