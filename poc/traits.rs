use std::iter::Iterator;

use sanitation::SString;
use serde::{Deserialize, Serialize};
use sha3::Digest;
pub use sha3::Sha3_384;

use crate::{Data, DataSeq, Result};

pub trait PlainBytes: for<'a> Deserialize<'a> + Serialize + Sized {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes)
            .expect(&format!("{}::from_plain_bytes", std::any::type_name::<Self>()))
    }
    fn to_plain_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("bytes")
    }
    fn from_plain_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize::<Self>(&bytes)?)
    }
    fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }
    fn from_deflate_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(crate::from_deflate_bytes::<Self>(bytes)?)
    }
    fn to_hex(&self, sep: &str, hint: bool) -> String {
        self.to_bytes()
            .iter()
            .map(|o| format!("{}{:02x}", if hint && sep.len() > 0 { "0x" } else { "" }, o))
            .collect::<SString>()
            .unchecked_safe()
    }
    fn sha3384(&self) -> Vec<u8> {
        let mut sha3_384 = Sha3_384::new();
        sha3_384.update(&self.to_plain_bytes());
        sha3_384.finalize().to_vec()
    }
    fn id3384(&self) -> String {
        let bytes = self.sha3384();
        hex::encode(&bytes[..8])
    }
}

pub trait SignedBytes<T: PlainBytes> {
    fn created_at(&self) -> Vec<u8>;
    fn signatures(&self) -> Vec<T>;
}

pub trait EphemeralDevice {
    fn created_at(&self) -> t16::Data;
    fn received_at(&self) -> t16::Data;
    fn expires_at(&self) -> t16::Data;
}

pub trait EphemeralEncryptionDevice: EphemeralDevice {
    fn accept_bytes(&self, data: &[u8]) -> Result<usize>;
    fn encrypt(&self) -> Result<Data>;
    // fn created_at(&self) -> t16::Data;
    // fn received_at(&self) -> t16::Data;
    // fn expires_at(&self) -> t16::Data;
}
pub trait EphemeralDecryptionDevice {
    fn accept_bytes(&self, data: &[u8]) -> Result<usize>;
    fn decrypt(&self) -> crate::Result<Data>;
    fn created_at(&self) -> t16::Data;
    fn received_at(&self) -> t16::Data;
    fn expires_at(&self) -> t16::Data;
}

pub trait EncryptionKey: Clone {
    // type DecryptionKey;
    fn encrypt(&self, data: impl Iterator<Item = u8>) -> Result<Data> {
        let data_seq = self.encrypt_bytes(&data.collect::<Vec<u8>>())?;
        data_seq.to_data()
    }
    fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq>;
}

pub trait DecryptionKey: Clone {
    // type EncryptionKey;
    fn decrypt(&self, data: impl Iterator<Item = u8>) -> Result<Data> {
        let enc_seq = DataSeq::from_data(&Data::new(data.collect::<Vec<u8>>()))?;
        let dec_sec = self.decrypt_bytes(enc_seq)?;
        let mut data = Data::new(Vec::new());
        for chunk in dec_sec.iter() {
            data.extend(chunk.iter());
        }
        Ok(data)
    }
    fn decrypt_bytes(&self, data: DataSeq) -> Result<DataSeq>;
}

pub trait Keygen: Clone {
    fn generate() -> Result<Self>;
}
