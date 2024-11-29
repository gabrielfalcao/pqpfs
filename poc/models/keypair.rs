use serde::{Deserialize, Serialize};
use crate::{EncryptionKey, DecryptionKey, ID};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Keypair<E: EncryptionKey, D: DecryptionKey> {
    id: ID,
    public: E,
    private: D,
}
