use crate::models::KeyStuff;
use crate::utils::{scrub_with_byte, zerofill};

#[derive(Debug, Clone)]
pub struct ID {
    bytes: Vec<u8>,
}
impl Copy for ID {
    fn copy(&self) -> ID {
        self.clone()
    }
}

impl ID {
    pub fn generate(length: usize) -> Result<ID, String> {
        let mut bytes = [0; length];
        rand::thread_rng()
            .try_fill(&mut bytes[..])
            .map_err(|err| err.to_string())?;
        ID { bytes }
    }
}

impl Into<Vec<u8>> for ID {
    fn into(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

pub trait StorageAccess {
    fn create(&self, id: ID, data: &[u8]) -> Vec<u8>;
    fn update(&self, id: ID, data: &[u8]) -> Result<(), String>;
    fn get(&self, id: ID) -> Vec<u8>;
    fn get_mut(&self, id: ID) -> &mut [u8];
    fn delete(&self, id: ID) -> Result<(), String>;
}

pub trait Storage<O: StorageAccess> {
    fn access(&self) -> O;
    fn create(&self, id: ID, data: &[u8]) -> Vec<u8> {
        self.access().create(id, data)
    }
    fn update(&self, id: ID, data: &[u8]) -> Result<(), String> {
        self.access().update(id, data)
    }
    fn get(&self, id: ID) -> Vec<u8> {
        self.access().get(id)
    }
    fn get_mut(&self, id: ID) -> &mut [u8] {
        self.access().get_mut(id)
    }
    fn delete(&self, id: ID) -> Result<(), String> {
        self.pulverize(id);
        self.zerofill(id);
        self.access().delete(id)
    }
    /// scrubs data at low-level reducing the capacity of the memory area to 0
    fn zerofill(&self, id: ID) -> Result<(), String> {
        let mut data: &mut [u8] = self.get_mut(id);
        zerofill(data);
        self.update(&id, data)?;
        Ok(())
    }
    /// scrubs data at low-level by charging, that is, maxing the memory area with choice of bytes to that effect then deleting all references
    /// MUST execute after zerofill()
    fn pulverize(&self, id: ID) -> Result<(), String> {
        let mut data: &mut [u8] = self.get_mut(id);

        let mut type_range = (0..u8::MAX).to_vec();
        type_range.reverse();

        zerofill(data);
        self.update(&id, data)?;
        for k in type_range {
            scrub_with_byte(data, k);
            self.update(&id, data)?;
        }
        zerofill(data);
        self.update(&id, data)?;
    }
}

pub trait KeyContainer<T>: Storage<T> {
    fn key_stuff(&self) -> KeyStuff;
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.key_stuff().encrypt(data)
    }
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.key_stuff().decrypt(data)
    }
    fn rekey(&mut self, id: ID, key_stuff: KeyStuff) -> Result<(), String> {
        let rekeyed = key_stuff.encrypt(self.decrypt(&self.get(id)?));
        self.delete(id)?;
        self.delete(&self.key_stuff().id)?;
        self.create(&key_stuff.id, &key_stuff.data)?;
        self.create(id, &rekeyed)?;
    }
}
