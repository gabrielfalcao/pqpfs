use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};


pub trait PlainBytes {
    fn bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
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
    Zeroize,
    ZeroizeOnDrop,
)]
pub struct PlainKey {
    #[zeroize]
    data: Vec<u8>,
    #[zeroize]
    length: usize,
}
impl PlainKey {
    pub fn new(data: &[u8]) -> PlainKey {
        let length = data.len();
        let data = data.to_vec();
        PlainKey { data, length }
    }
}

impl From<Vec<u8>> for PlainKey {
    fn from(data: Vec<u8>) -> PlainKey {
        let length = data.len();
        PlainKey { data, length }
    }
}
impl From<&Vec<u8>> for PlainKey {
    fn from(data: &Vec<u8>) -> PlainKey {
        let data = data.clone();
        let length = data.len();
        PlainKey { data, length }
    }
}
impl Into<Vec<u8>> for PlainKey {
    fn into(self) -> Vec<u8> {
        self.data.clone()
    }
}

impl PlainBytes for PlainKey {
    fn bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
    fn len(&self) -> usize {
        self.length
    }
}

pub trait SignedBytes {
    fn created_at(&self) -> Vec<u8>;
    fn signatures(&self) -> Vec<PlainKey>;
}
