use std::iter::Iterator;
use crate::Result;

pub trait EncryptionKey {
    fn encrypt(&self, data: impl Iterator<Item = u8>) -> Result<crate::data::DataSeq> {
        self.encrypt_bytes(&data.collect::<Vec<u8>>())
    }
    fn encrypt_bytes(&self, data: &[u8]) -> Result<crate::data::DataSeq>;
}

pub trait DecryptionKey {
    fn decrypt(&self, data: impl Iterator<Item = u8>) -> Result<crate::data::Data> {
        self.decrypt_bytes(&data.collect::<Vec<u8>>())
    }
    fn decrypt_bytes(&self, data: &[u8]) -> Result<crate::data::Data>;
}
