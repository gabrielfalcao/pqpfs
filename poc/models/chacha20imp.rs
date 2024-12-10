use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use serde::{Deserialize, Serialize};

use super::base::{DecryptionKey, EncryptionKey};
use crate::data::Data;
use crate::errors::Result;
use crate::traits::PlainBytes;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Chacha20Key {
    key: Data,
    key_start: Option<usize>,
    key_end: Option<usize>,
    nonce_start: Option<usize>,
    nonce_end: Option<usize>,
}
impl Chacha20Key {
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Result<Chacha20Key> {
        let bytes = bytes.into();
        if bytes.len() < (256 / 8) {
            return Err(Error::InvalidKeyError(format!(
                "chacha20 key MUST be 256 bits or greater in length"
            )));
        }
        if bytes.len() < (352 / 8) {
            return Err(Error::InvalidKeyError(format!(
                "chacha20 nonce MUST be 96 bits or greater in length"
            )));
        }

        Ok(Chacha20Key { key })
    }

    fn key_slice(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        let start = self.key_start.unwrap_or(0);
        let end = self.key_start.unwrap_or(31);
        for index in start..end {
            key[index] = self.data[index];
        }
        key
    }

    fn nonce_slice(&self) -> [u8; 32] {
        let mut nonce = [0u8; 12];
        let start = self.nonce_start.unwrap_or(0);
        let end = self.nonce_start.unwrap_or(11);
        for index in start..end {
            nonce[index] = self.data[index];
        }
        nonce
    }

    fn chacha20(&self) -> ChaCha20 {
        ChaCha20::new_from_slice(&self.key_slice())
    }
}

impl From<Data> for Chacha20Key {
    fn from(data: Data) -> Chacha20Key {
        Self::from_bytes(data.to_bytes()).expect("expected valid Chacha20 Key")
    }
}
impl From<&Data> for Chacha20Key {
    fn from(data: &Data) -> Chacha20Key {
        Self::from_bytes(data.to_bytes()).expect("expected valid Chacha20 Key")
    }
}
impl From<Vec<u8>> for Chacha20Key {
    fn from(data: Vec<u8>) -> Chacha20Key {
        Self::from_bytes(data).expect("expected valid Chacha20 Key")
    }
}
impl From<&Vec<u8>> for Chacha20Key {
    fn from(data: &Vec<u8>) -> Chacha20Key {
        Self::from_bytes(data.clone()).expect("expected valid Chacha20 Key")
    }
}
impl From<&[u8]> for Chacha20Key {
    fn from(data: &[u8]) -> Chacha20Key {
        Self::from_bytes(data.to_vec()).expect("expected valid Chacha20 Key")
    }
}

impl PlainBytes for Chacha20Key {
    fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes()
    }

    fn len(&self) -> usize {
        self.key.len()
    }
}
