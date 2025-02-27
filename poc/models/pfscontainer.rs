use serde::{Deserialize, Serialize};
use crate::{Data, EncryptionKey, Keygen, DecryptionKey};

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct PFSCipherText<K: EncryptionKey> {
    pub data: Data,
    pub private_key: DecryptionKey,
}
impl<K: EncryptionKey> PFSCipherText<K> {
    pub fn new(data: &Data, private_key: &K) -> PFSCipherText<K> {
        PFSCipherText {
            data: data.clone(),
            private_key: private_key.clone(),
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct PFSContainer<K: EncryptionKey> {
    pub ciphertexts: Vec<PFSCipherText<K>>,
    pub private_key: K,
}
impl<K: EncryptionKey> PFSContainer<K> {
    pub fn new(private_key: &K) -> PFSContainer<K> {
        PFSContainer {
            ciphertexts: Vec::new(),
            private_key: private_key.clone(),
        }
    }

    pub fn add_ciphertext(&mut self, ciphertext: &PFSCipherText<K>) {
        self.ciphertexts.push(ciphertext.clone());
    }
}
