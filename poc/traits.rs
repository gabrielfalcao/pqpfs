use std::iter::Iterator;

use sanitation::SString;
use serde::{Deserialize, Serialize};

use crate::{Data, DataSeq, Result};

pub trait PlainBytes: for<'a> Deserialize<'a> + Serialize + Sized{
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;

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

pub trait EncryptionKey {
    fn encrypt(&self, data: impl Iterator<Item = u8>) -> Result<Data> {
        let data_seq = self.encrypt_bytes(&data.collect::<Vec<u8>>())?;
        data_seq.to_data()
    }
    fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq>;
}

pub trait DecryptionKey {
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
