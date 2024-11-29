use std::iter::Iterator;

use crate::{Data, DataSeq, Result};
pub trait PlainBytes {
    fn bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
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
        let dec_sec = self.decrypt_bytes(&data.collect::<Vec<u8>>())?;
        let mut data = Data::new(Vec::new());
        for chunk in dec_sec.iter() {
            data.extend(chunk.iter());
        }
        Ok(data)
    }
    fn decrypt_bytes(&self, data: &[u8]) -> Result<DataSeq>;
}
