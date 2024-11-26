use std::ops::DerefMut;

use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use super::base::{DecryptionKey, EncryptionKey};
use crate::data::Data;
use crate::errors::Result;
use crate::traits::PlainBytes;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RSAPrivateKey {
    key: Data,
}
impl RSAPrivateKey {
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Result<RSAPrivateKey> {
        let bytes = bytes.into();
        RsaPrivateKey::from_pkcs8_der(&bytes)?;
        let key = Data::from(bytes);
        Ok(RSAPrivateKey { key })
    }

    fn rsa(&self) -> RsaPrivateKey {
        RsaPrivateKey::from_pkcs8_der(&self.bytes()).expect("valid private RSA key bytes")
    }

    pub fn generate() -> Result<RSAPrivateKey> {
        let bits = 2048;
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let mut data = private_key.to_pkcs8_der()?.to_bytes();
        let data = data.deref_mut();
        let key = Data::new(data.to_vec());
        Ok(RSAPrivateKey { key })
    }

    pub fn public_key(&self) -> RSAPublicKey {
        RSAPublicKey::from_inner(&self.rsa().to_public_key()).expect("valid RSA key bytes")
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<RSAPrivateKey> {
        Ok(crate::from_deflate_bytes::<RSAPrivateKey>(bytes)?)
    }
}
impl DecryptionKey for RSAPrivateKey {
    fn decrypt_bytes(&self, data: &[u8]) -> Result<Data> {
        Ok(Data::from_iter(self.rsa().decrypt(Pkcs1v15Encrypt, data)?))
    }
}

impl From<Data> for RSAPrivateKey {
    fn from(data: Data) -> RSAPrivateKey {
        Self::from_bytes(data.bytes()).expect("expected valid RSA Private Key")
    }
}
impl From<&Data> for RSAPrivateKey {
    fn from(data: &Data) -> RSAPrivateKey {
        Self::from_bytes(data.bytes()).expect("expected valid RSA Private Key")
    }
}
impl From<Vec<u8>> for RSAPrivateKey {
    fn from(data: Vec<u8>) -> RSAPrivateKey {
        Self::from_bytes(data).expect("expected valid RSA Private Key")
    }
}
impl From<&Vec<u8>> for RSAPrivateKey {
    fn from(data: &Vec<u8>) -> RSAPrivateKey {
        Self::from_bytes(data.clone()).expect("expected valid RSA Private Key")
    }
}
impl From<&[u8]> for RSAPrivateKey {
    fn from(data: &[u8]) -> RSAPrivateKey {
        Self::from_bytes(data.to_vec()).expect("expected valid RSA Private Key")
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
        let mut data =
            private_key.to_pkcs8_der().expect("expected valid RSA private key").to_bytes();
        let data = data.deref_mut();

        let key = Data::new(data.to_vec());
        RSAPrivateKey { key }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RSAPublicKey {
    key: Data,
}
impl RSAPublicKey {
    pub fn from_bytes(data: impl Into<Vec<u8>>) -> Result<RSAPublicKey> {
        let data = data.into();
        RsaPublicKey::from_pkcs1_der(&data)?;
        let key = Data::new(data);
        Ok(RSAPublicKey { key })
    }

    pub fn from_inner(public_key: &RsaPublicKey) -> Result<RSAPublicKey> {
        let key = Data::new(public_key.to_pkcs1_der()?.as_bytes().to_vec());
        Ok(RSAPublicKey { key })
    }

    pub fn rsa(&self) -> RsaPublicKey {
        RsaPublicKey::from_pkcs1_der(&self.bytes()).expect("valid public RSA key bytes")
    }
}

impl From<&RsaPublicKey> for RSAPublicKey {
    fn from(public_key: &RsaPublicKey) -> RSAPublicKey {
        let key = Data::new(
            public_key
                .to_pkcs1_der()
                .expect("expected valid RSA public key")
                .as_bytes()
                .to_vec(),
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
impl EncryptionKey for RSAPublicKey {
    fn encrypt_bytes(&self, data: &[u8]) -> Result<Data> {
        let mut rng = rand::thread_rng();
        Ok(Data::from_iter(self.rsa().encrypt(&mut rng, Pkcs1v15Encrypt, data)?))
    }
}
impl From<Data> for RSAPublicKey {
    fn from(data: Data) -> RSAPublicKey {
        Self::from_bytes(data).expect("expected valid RSA Public Key")
    }
}
impl From<&Data> for RSAPublicKey {
    fn from(data: &Data) -> RSAPublicKey {
        Self::from_bytes(data.clone()).expect("expected valid RSA Public Key")
    }
}
impl From<Vec<u8>> for RSAPublicKey {
    fn from(data: Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(data).expect("expected valid RSA Public Key")
    }
}
impl From<&Vec<u8>> for RSAPublicKey {
    fn from(data: &Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(data.clone()).expect("expected valid RSA Public Key")
    }
}
impl From<&[u8]> for RSAPublicKey {
    fn from(data: &[u8]) -> RSAPublicKey {
        Self::from_bytes(data.to_vec()).expect("expected valid RSA Public Key")
    }
}
