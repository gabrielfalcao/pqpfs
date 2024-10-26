use crate::traits::{PlainBytes, PlainKey};
use serde::{Deserialize, Serialize};

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

#[derive(
    Debug,
    Clone,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct Keypair {
    id: Vec<u8>,
    private: PrivateKey,
    public: PublicKey,
}
