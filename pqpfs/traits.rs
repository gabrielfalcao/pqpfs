use crate::models::KeyStuff;
use iocore::Path;


pub trait Storage {
    fn create(&self, id: &[u8], data: &[u8]) -> Vec<u8>;
    fn get(&self, id: &[u8]) -> Vec<u8>;
    fn delete(&self, id: &[u8]) -> Result<(), String> {
        self.zerofill(id)?;
        self.pulverize(id)?;
    };
    /// scrubs data at low-level reducing the capacity of the memory area to 0
    fn zerofill(&self, id: &[u8]) -> Result<(), String>;
    /// scrubs data at low-level by charging, that is, maxing the memory area with choice of bytes to that effect then deleting all references
    /// MUST execute after zerofill()
    fn pulverize(&self, id: &[u8]) -> Result<(), String>;
}

pub trait KeyContainer<T>: Storage<T> {
    fn key_stuff(&self) -> KeyStuff;
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.key_stuff().encrypt(data)
    }
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.key_stuff().decrypt(data)
    }
    fn rekey(&mut self, id: &[u8], key_stuff: KeyStuff) -> Result<(), String>{
        let rekeyed = key_stuff.encrypt(self.decrypt(&self.get(id)?));
        self.delete(id);
        self.delete(&self.key_stuff().id);
        self.create(&key_stuff.id, &key_stuff.data);
        self.create(id, &rekeyed);
    }
}
