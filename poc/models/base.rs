use crate::traits::{PlainBytes, PlainKey};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct ID {
    bytes: Vec<u8>,
}
impl ID {
    pub fn new(bytes: Vec<u8>) -> ID {
        ID { bytes }
    }
    pub fn generate(length: usize) -> Result<ID, String> {
        let mut rng = rand::thread_rng();
        let mut bytes = Vec::with_capacity(length);
        for n in 0..length {
            let mut byte: u8 = rng.gen();
            while byte > 0x29 && byte > 0x40 && byte > 0x60 && byte <= 0x7a && byte <= 0x5a {
                byte = rng.gen();
            }
            bytes[n] = byte;
        }
        Ok(ID { bytes })
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct PrivateKey {
    data: PlainKey,
}
impl From<Vec<u8>> for PrivateKey {
    fn from(data: Vec<u8>) -> PrivateKey {
        let data = PlainKey::from(data);
        PrivateKey { data }
    }
}
impl From<&Vec<u8>> for PrivateKey {
    fn from(data: &Vec<u8>) -> PrivateKey {
        let data = PlainKey::from(data);
        PrivateKey { data }
    }
}

impl PrivateKey {
    pub fn new(data: &[u8]) -> PrivateKey {
        let data = PlainKey::new(data);
        PrivateKey { data }
    }
}

impl PlainBytes for PrivateKey {
    fn bytes(&self) -> Vec<u8> {
        self.data.bytes()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct PublicKey {
    data: PlainKey,
}
impl PublicKey {
    pub fn new(data: &[u8]) -> PublicKey {
        let data = PlainKey::new(data);
        PublicKey { data }
    }
}
impl From<Vec<u8>> for PublicKey {
    fn from(data: Vec<u8>) -> PublicKey {
        let data = PlainKey::from(data);
        PublicKey { data }
    }
}

impl PlainBytes for PublicKey {
    fn bytes(&self) -> Vec<u8> {
        self.data.bytes()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Keypair {
    id: ID,
    private: PrivateKey,
    public: PublicKey,
}
