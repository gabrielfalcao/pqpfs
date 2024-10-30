use security_framework::os::macos::keychain::SecKeychain;

use crate::models::ID;
use crate::data::Data;
use crate::storage::traits::StorageAccess;
use crate::{Error, Result};

pub struct MacOSKeychainStorage {
    pub namespace: String,
}

impl MacOSKeychainStorage {
    pub fn new(namespace: impl std::fmt::Display) -> MacOSKeychainStorage {
        let namespace = namespace.to_string();
        MacOSKeychainStorage { namespace }
    }

    fn set(&self, key: impl std::fmt::Display, value: &Data) -> Result<()> {
        let key = key.to_string();
        let keychain = SecKeychain::default()?;
        keychain.set_generic_password(&self.namespace, &key, &value.to_vec())?;
        Ok(())
    }

    fn get_bytes(&self, key: &ID) -> Result<Data> {
        let key = hex::encode(&key.bytes());
        let keychain = SecKeychain::default()?;
        let result = keychain.find_generic_password(&self.namespace, &key);
        match result {
            Ok((data, _)) => Ok(Data::from(data)),
            Err(_) => Err(Error::StorageError(format!("{:#?} not found in MacOS Keychain", key))),
        }
    }

    fn del(&self, key: impl std::fmt::Display) -> Result<()> {
        let key = key.to_string();
        let keychain = SecKeychain::default()?;
        let (_, item) = keychain.find_generic_password(&self.namespace, &key)?;
        item.delete();
        Ok(())
    }
}

impl StorageAccess for MacOSKeychainStorage {
    fn create(&self, id: &ID, data: &Data) -> Result<Data> {
        self.set(id, data)?;
        Ok(data.clone())
    }

    fn update(&self, id: &ID, data: &Data) -> Result<()> {
        self.set(id, data)
    }

    fn get(&self, id: &ID) -> Result<Data> {
        self.get_bytes(id)
    }

    fn delete(&self, id: &ID) -> Result<()> {
        self.del(id)
    }
}
