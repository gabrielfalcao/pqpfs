use des::cipher::block_padding::Pkcs7;
use des::cipher::{BlockDecryptMut, BlockEncryptMut, Iv, Key, KeyIvInit};
use des::Des;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::data::{Data, DataSeq};
use crate::errors::{Error, Result};
use crate::traits::{DecryptionKey, EncryptionKey, Keygen, PlainBytes};
use crate::utils::chunk_padded;

type Des64CbcEnc = cbc::Encryptor<Des>;
type Des64CbcDec = cbc::Decryptor<Des>;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct DesKey {
    key: [u8; 8],
    iv: [u8; 8],
}

impl DesKey {
    pub fn new(key: [u8; 8], iv: [u8; 8]) -> DesKey {
        DesKey { key, iv }
    }

    pub fn key(&self) -> Key<Des64CbcEnc> {
        self.key.into()
    }

    pub fn iv(&self) -> Iv<Des64CbcEnc> {
        self.iv.into()
    }
}

impl EncryptionKey for DesKey {
    type DecryptionKey = DesKey;

    fn encrypt_bytes(&self, data: &[u8]) -> Result<DataSeq> {
        let mut ds = DataSeq::new();
        let mut data = data.to_vec();
        data.reverse();
        for chunk in chunk_padded(&data, 8, 0) {
            let cipher = Des64CbcEnc::new((&self.key).into(), (&self.iv).into());
            ds.push(Data::from(cipher.encrypt_padded_vec_mut::<Pkcs7>(&chunk)))
        }
        Ok(ds)
    }
}
impl DecryptionKey for DesKey {
    type EncryptionKey = DesKey;

    fn decrypt_bytes(&self, data: DataSeq) -> Result<DataSeq> {
        let mut bytes = Vec::new();
        for chunk in data {
            let cipher = Des64CbcDec::new((&self.key).into(), (&self.iv).into());
            bytes.extend(
                cipher
                    .decrypt_padded_vec_mut::<Pkcs7>(&chunk.to_bytes())
                    .map_err(|e| Error::DecodingError(format!("DES {}", e)))?,
            );
        }
        bytes.reverse();
        let mut ds = DataSeq::new();
        for chunk in bytes.chunks(8) {
            ds.push(Data::from(chunk));
        }
        Ok(ds)
    }
}

impl Keygen for DesKey {
    fn generate() -> Result<DesKey> {
        let mut rng = rand::thread_rng();
        let mut key: [u8; 8] = [0u8; 8];
        rng.fill(&mut key[..]);
        let mut iv: [u8; 8] = [0u8; 8];
        rng.fill(&mut iv[..]);
        Ok(DesKey { key, iv })
    }
}
impl PlainBytes for DesKey {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.key[0],
            self.key[1],
            self.key[2],
            self.key[3],
            self.key[4],
            self.key[5],
            self.key[6],
            self.key[7],
            self.iv[0],
            self.iv[1],
            self.iv[2],
            self.iv[3],
            self.iv[4],
            self.iv[5],
            self.iv[6],
            self.iv[7],
        ]
        .to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> DesKey {
        let mut key: [u8; 8] = [0xF1; 8];
        let mut iv: [u8; 8] = [0xF2; 8];
        key.copy_from_slice(&bytes[..8]);
        iv.copy_from_slice(&bytes[9..16]);
        DesKey { key, iv }
    }
}
