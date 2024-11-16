use std::io::Write;

use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Data {
    pub inner: Vec<u8>,
}
impl Data {
    pub fn new(inner: Vec<u8>) -> Data {
        Data { inner }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn to_hex(&self, sep: &str) -> String {
        self.inner
            .iter()
            .map(|o| format!("{:02x}", o))
            .collect::<Vec<String>>()
            .join(sep)
    }

    pub fn from_hex(data: &str) -> Result<Data> {
        let data = hex::decode(data)?;
        Ok(Data::new(data))
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(self)?;
        let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
        e.write(&bytes)?;
        Ok(e.finish()?)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<Data> {
        let mut d = DeflateDecoder::new(Vec::new());
        d.write(bytes)?;
        Ok(Data::new(d.finish()?))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex(""))
    }
}
