use std::ops::DerefMut;

use num_traits::FromPrimitive;
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{BigUint, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::data::{Data, DataSeq};
use crate::errors::{Error, Result};
use crate::traits::{DecryptionKey, EncryptionKey, Keygen, PlainBytes};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RSAPrivateKey {
    key: Data,
}

impl RSAPrivateKey {
    fn rsa(&self) -> RsaPrivateKey {
        RsaPrivateKey::from_pkcs8_der(&self.to_bytes()).expect("valid private RSA key bytes")
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

    pub fn to_vec(&self) -> Vec<u8> {
        self.key.to_vec()
    }
}
impl EncryptionKey for RSAPublicKey {
    type DecryptionKey = RSAPrivateKey;

    fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq> {
        let mut rng = rand::thread_rng();
        let mut ds = DataSeq::new();
        // https://datatracker.ietf.org/doc/html/rfc8017#section-7.2.1
        for chunk in data.chunks((u8::MAX - 10).into()) {
            ds.push(Data::from(
                self.rsa()
                    .encrypt(&mut rng, Pkcs1v15Encrypt, chunk)
                    .map_err(|e| Error::RSAError(format!("encrypt {} bytes {}", data.len(), e)))?,
            ));
        }
        Ok(ds)
    }
}
impl DecryptionKey for RSAPrivateKey {
    type EncryptionKey = RSAPublicKey;

    fn decrypt_bytes(&self, data: DataSeq) -> Result<DataSeq> {
        let mut dec_seq = DataSeq::new();
        for chunk in data {
            dec_seq.push(Data::from(
                self.rsa()
                    .decrypt(Pkcs1v15Encrypt, &chunk.to_bytes())
                    .map_err(|e| Error::RSAError(format!("decrypt {} bytes {}", chunk.len(), e)))?,
            ));
        }
        Ok(dec_seq)
    }
}
impl Keygen for RSAPrivateKey {
    fn generate() -> Result<RSAPrivateKey> {
        let bits = 2048;
        let mut rng = rand::thread_rng();
        let exp = BigUint::from_u64(65537u64).expect("BigUint");
        let private_key = RsaPrivateKey::new_with_exp(&mut rng, bits, &exp)?;
        let mut data = private_key.to_pkcs8_der()?.to_bytes();
        let data = data.deref_mut();
        let key = Data::new(data.to_vec());
        Ok(RSAPrivateKey { key })
    }
}
impl EncryptionKey for RSAPrivateKey {
    type DecryptionKey = RSAPrivateKey;

    fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq> {
        self.public_key().encrypt_bytes(data)
    }
}

impl From<Data> for RSAPrivateKey {
    fn from(data: Data) -> RSAPrivateKey {
        RSAPrivateKey::from_bytes(&data.to_bytes())
    }
}
impl From<&Data> for RSAPrivateKey {
    fn from(data: &Data) -> RSAPrivateKey {
        RSAPrivateKey::from_bytes(&data.to_bytes())
    }
}
impl From<Vec<u8>> for RSAPrivateKey {
    fn from(data: Vec<u8>) -> RSAPrivateKey {
        RSAPrivateKey::from_bytes(&data)
    }
}
impl From<&Vec<u8>> for RSAPrivateKey {
    fn from(data: &Vec<u8>) -> RSAPrivateKey {
        RSAPrivateKey::from_bytes(data)
    }
}
impl From<&[u8]> for RSAPrivateKey {
    fn from(data: &[u8]) -> RSAPrivateKey {
        RSAPrivateKey::from_bytes(data)
    }
}

impl PlainBytes for RSAPrivateKey {
    fn to_bytes(&self) -> Vec<u8> {
        self.key.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> RSAPrivateKey {
        let bytes = bytes.to_vec();
        RsaPrivateKey::from_pkcs8_der(&bytes).expect("RSAPrivateKey");
        let key = Data::from(bytes);
        RSAPrivateKey { key }
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
    fn from_inner(public_key: &RsaPublicKey) -> Result<RSAPublicKey> {
        let key = Data::new(public_key.to_pkcs1_der()?.as_bytes().to_vec());
        Ok(RSAPublicKey { key })
    }

    fn rsa(&self) -> RsaPublicKey {
        RsaPublicKey::from_pkcs1_der(&self.to_bytes()).expect("valid public RSA key bytes")
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
    fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> RSAPublicKey {
        let bytes = bytes.to_vec();
        RsaPublicKey::from_pkcs1_der(&bytes).expect("RSAPublicKey");
        let key = Data::from(bytes);
        RSAPublicKey { key }
    }
}
impl From<Data> for RSAPublicKey {
    fn from(data: Data) -> RSAPublicKey {
        Self::from_bytes(&data.to_vec())
    }
}
impl From<&Data> for RSAPublicKey {
    fn from(data: &Data) -> RSAPublicKey {
        Self::from_bytes(&data.to_vec())
    }
}
impl From<Vec<u8>> for RSAPublicKey {
    fn from(data: Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(&data)
    }
}
impl From<&Vec<u8>> for RSAPublicKey {
    fn from(data: &Vec<u8>) -> RSAPublicKey {
        Self::from_bytes(data)
    }
}
impl From<&[u8]> for RSAPublicKey {
    fn from(data: &[u8]) -> RSAPublicKey {
        Self::from_bytes(data)
    }
}
