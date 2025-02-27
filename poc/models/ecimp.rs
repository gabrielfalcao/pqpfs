use std::ops::DerefMut;

use elliptic_curve::{FieldBytes, SecretKey};
use num_traits::FromPrimitive;
use p384::NistP384;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::data::{Data, DataSeq};
use crate::errors::{Error, Result};
use crate::traits::{DecryptionKey, EncryptionKey, PlainBytes};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ECP384PrivateKey {
    key: Data,
}

impl ECP384PrivateKey {
    fn ecp384(&self) -> ECP384PrivateKey {
        ECP384PrivateKey::from_bytes(&self.to_bytes().into()).expect("valid private P384 key bytes")
    }

    pub fn generate() -> Result<ECP384PrivateKey> {
        let secret_key = SecretKey::<NistP384>::random(&mut OsRng);
        let key = Data::new(secret_key.to_bytes().as_slice().to_vec());
        Ok(ECP384PrivateKey { key })
    }

    pub fn public_key(&self) -> ECP384PublicKey {
        ECP384PublicKey::from_inner(&self.ecp384().public_key()).expect("valid P384 key bytes")
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<ECP384PrivateKey> {
        Ok(crate::from_deflate_bytes::<ECP384PrivateKey>(bytes)?)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.key.to_vec()
    }
}
// impl EncryptionKey for ECP384PublicKey {
//     fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq> {
//         let mut rng = rand::thread_rng();
//         let mut ds = DataSeq::new();
//         // https://datatracker.ietf.org/doc/html/rfc8017#section-7.2.1
//         for chunk in data.chunks((u8::MAX - 10).into()) {
//             ds.push(Data::from(
//                 self.ecp384()
//                     .encrypt(&mut rng, Pkcs1v15Encrypt, chunk)
//                     .map_err(|e| Error::ECP384Error(format!("encrypt {} bytes {}", data.len(), e)))?,
//             ));
//         }
//         Ok(ds)
//     }
// }
// impl DecryptionKey for ECP384PrivateKey {
//     fn decrypt_bytes(&self, data: DataSeq) -> Result<DataSeq> {
//         let mut dec_seq = DataSeq::new();
//         for chunk in data {
//             dec_seq.push(Data::from(
//                 self.ecp384()
//                     .decrypt(Pkcs1v15Encrypt, &chunk.to_bytes())
//                     .map_err(|e| Error::ECP384Error(format!("decrypt {} bytes {}", chunk.len(), e)))?,
//             ));
//         }
//         Ok(dec_seq)
//     }
// }
// impl EncryptionKey for ECP384PrivateKey {
//     fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq> {
//         self.public_key().encrypt_bytes(data)
//     }
// }

// impl From<Data> for ECP384PrivateKey {
//     fn from(data: Data) -> ECP384PrivateKey {
//         ECP384PrivateKey::from_bytes(&data.to_bytes())
//     }
// }
// impl From<&Data> for ECP384PrivateKey {
//     fn from(data: &Data) -> ECP384PrivateKey {
//         ECP384PrivateKey::from_bytes(&data.to_bytes())
//     }
// }
// impl From<Vec<u8>> for ECP384PrivateKey {
//     fn from(data: Vec<u8>) -> ECP384PrivateKey {
//         ECP384PrivateKey::from_bytes(&data)
//     }
// }
// impl From<&Vec<u8>> for ECP384PrivateKey {
//     fn from(data: &Vec<u8>) -> ECP384PrivateKey {
//         ECP384PrivateKey::from_bytes(data)
//     }
// }
// impl From<&[u8]> for ECP384PrivateKey {
//     fn from(data: &[u8]) -> ECP384PrivateKey {
//         ECP384PrivateKey::from_bytes(data)
//     }
// }

// impl PlainBytes for ECP384PrivateKey {
//     fn to_bytes(&self) -> Vec<u8> {
//         self.key.to_vec()
//     }

//     fn from_bytes(bytes: &[u8]) -> ECP384PrivateKey {
//         let bytes = bytes.to_vec();
//         ECP384PrivateKey::from_pkcs8_der(&bytes).expect("ECP384PrivateKey");
//         let key = Data::from(bytes);
//         ECP384PrivateKey { key }
//     }
// }
// impl From<&ECP384PrivateKey> for ECP384PrivateKey {
//     fn from(private_key: &ECP384PrivateKey) -> ECP384PrivateKey {
//         let mut data =
//             private_key.to_pkcs8_der().expect("expected valid ECP384 private key").to_bytes();
//         let data = data.deref_mut();

//         let key = Data::new(data.to_vec());
//         ECP384PrivateKey { key }
//     }
// }

// #[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
// pub struct ECP384PublicKey {
//     key: Data,
// }
// impl ECP384PublicKey {
//     fn from_inner(public_key: &ECP384PublicKey) -> Result<ECP384PublicKey> {
//         let key = Data::new(public_key.to_pkcs1_der()?.as_bytes().to_vec());
//         Ok(ECP384PublicKey { key })
//     }

//     fn ecp384(&self) -> ECP384PublicKey {
//         ECP384PublicKey::from_pkcs1_der(&self.to_bytes()).expect("valid public ECP384 key bytes")
//     }
// }

// impl From<&ECP384PublicKey> for ECP384PublicKey {
//     fn from(public_key: &ECP384PublicKey) -> ECP384PublicKey {
//         let key = Data::new(
//             public_key
//                 .to_bytes()
//                 .as_slice()
//                 .to_vec(),
//         );
//         ECP384PublicKey { key }
//     }
// }
// impl PlainBytes for ECP384PublicKey {
//     fn to_bytes(&self) -> Vec<u8> {
//         self.key.to_bytes()
//     }

//     fn from_bytes(bytes: &[u8]) -> ECP384PublicKey {
//         let bytes = bytes.to_vec();
//         ECP384PublicKey::from_pkcs1_der(&bytes).expect("ECP384PublicKey");
//         let key = Data::from(bytes);
//         ECP384PublicKey { key }
//     }
// }
// impl From<Data> for ECP384PublicKey {
//     fn from(data: Data) -> ECP384PublicKey {
//         Self::from_bytes(&data.to_vec())
//     }
// }
// impl From<&Data> for ECP384PublicKey {
//     fn from(data: &Data) -> ECP384PublicKey {
//         Self::from_bytes(&data.to_vec())
//     }
// }
// impl From<Vec<u8>> for ECP384PublicKey {
//     fn from(data: Vec<u8>) -> ECP384PublicKey {
//         Self::from_bytes(&data)
//     }
// }
// impl From<&Vec<u8>> for ECP384PublicKey {
//     fn from(data: &Vec<u8>) -> ECP384PublicKey {
//         Self::from_bytes(data)
//     }
// }
// impl From<&[u8]> for ECP384PublicKey {
//     fn from(data: &[u8]) -> ECP384PublicKey {
//         Self::from_bytes(data)
//     }
// }
