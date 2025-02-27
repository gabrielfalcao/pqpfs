use serde::{Deserialize, Serialize};
use crate::{EncryptionKey, ID};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Keypair<K: EncryptionKey> {
    id: ID,
    public: K,
    private: K::DecryptionKey,
}
