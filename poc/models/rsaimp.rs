use super::base::{PrivateKey, PublicKey};
use crate::errors::Result;
use crate::traits::PlainBytes;
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::Pkcs1v15Encrypt;
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
impl From<&[u8]> for RSAPrivateKey {
    fn from(data: &[u8]) -> RSAPrivateKey {
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
    fn rsa(&self) -> RsaPrivateKey {
        RsaPrivateKey::from_pkcs8_der(&self.bytes()).expect("valid private RSA key bytes")
    }
    pub fn generate() -> Result<RSAPrivateKey> {
        let bits = 2048;
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let key = PrivateKey::new(private_key.to_pkcs8_der()?.to_bytes().deref_mut());
        Ok(RSAPrivateKey { key })
    }
    pub fn public_key(&self) -> RSAPublicKey {
        RSAPublicKey::from_inner(&self.rsa().to_public_key()).expect("valid RSA key bytes")
    }
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(self.rsa().decrypt(Pkcs1v15Encrypt, &data)?.to_vec())
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
impl RSAPublicKey {
    pub fn from_bytes(data: &[u8]) -> Result<RSAPublicKey> {
        RsaPublicKey::from_pkcs1_der(data)?;
        let key = PublicKey::new(data);
        Ok(RSAPublicKey { key })
    }
    fn from_inner(public_key: &RsaPublicKey) -> Result<RSAPublicKey> {
        let key = PublicKey::new(public_key.to_pkcs1_der()?.as_bytes());
        Ok(RSAPublicKey { key })
    }
    fn rsa(&self) -> RsaPublicKey {
        RsaPublicKey::from_pkcs1_der(&self.bytes()).expect("valid public RSA key bytes")
    }
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        Ok(self.rsa().encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?)
    }
}
impl From<Vec<u8>> for RSAPublicKey {
    fn from(data: Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(&data).expect("expected valid RSA Public Key")
    }
}
impl From<&Vec<u8>> for RSAPublicKey {
    fn from(data: &Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(data).expect("expected valid RSA Public Key")
    }
}
impl From<&[u8]> for RSAPublicKey {
    fn from(data: &[u8]) -> RSAPublicKey {
        Self::from_bytes(data).expect("expected valid RSA Public Key")
    }
}
