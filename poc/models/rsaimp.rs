use super::base::{PrivateKey, PublicKey};
use crate::errors::{Error, Result};
use crate::traits::{PlainBytes, PlainKey};
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RSAPrivateKey {
    key: PrivateKey,
}

impl From<Vec<u8>> for RSAPrivateKey {
    fn from(data: Vec<u8>) -> RSAPrivateKey {
        Self::from_bytes(&data).expect("expected valid RSA Private Key")
    }
}
impl From<&Vec<u8>> for RSAPrivateKey {
    fn from(data: &Vec<u8>) -> RSAPrivateKey {
        Self::from_bytes(data).expect("expected valid RSA Private Key")
    }
}

impl PlainBytes for RSAPrivateKey {
    fn bytes(&self) -> Vec<u8> {
        self.key.bytes()
    }
    fn len(&self) -> usize {
        self.key.len()
    }
}
impl From<&RsaPrivateKey> for RSAPrivateKey {
    fn from(private_key: &RsaPrivateKey) -> RSAPrivateKey {
        let key = PrivateKey::new(
            private_key
                .to_pkcs8_der()
                .expect("expected valid RSA private key")
                .to_bytes()
                .deref_mut(),
        );
        RSAPrivateKey { key }
    }
}

impl RSAPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<RSAPrivateKey> {
        RsaPrivateKey::from_pkcs8_der(bytes)?;
        let data = bytes.to_vec();
        let key = PrivateKey::from(data);
        Ok(RSAPrivateKey { key })
    }
    pub fn generate() -> Result<RSAPrivateKey> {
        let bits = 2048;
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let key = PrivateKey::new(private_key.to_pkcs8_der()?.to_bytes().deref_mut());
        Ok(RSAPrivateKey { key })
    }
}
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RSAPublicKey {
    key: PublicKey,
}
impl From<&RsaPublicKey> for RSAPublicKey {
    fn from(public_key: &RsaPublicKey) -> RSAPublicKey {
        let key = PublicKey::new(
            public_key
                .to_pkcs1_der()
                .expect("expected valid RSA public key")
                .as_bytes(),
        );
        RSAPublicKey { key }
    }
}
impl PlainBytes for RSAPublicKey {
    fn bytes(&self) -> Vec<u8> {
        self.key.bytes()
    }
    fn len(&self) -> usize {
        self.key.len()
    }
}
