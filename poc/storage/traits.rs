use crate::data::Data;
use crate::models::{DecryptionKey, EncryptionKey, ID};
use crate::utils::{scrub_with_byte, zerofill};
use crate::Result;

pub trait StorageAccess {
    fn create(&self, id: &ID, data: &Data) -> Result<Data>;
    fn update(&self, id: &ID, data: &Data) -> Result<()>;
    fn get(&self, id: &ID) -> Result<Data>;
    fn delete(&self, id: &ID) -> Result<()>;
}

pub trait Storage<O: StorageAccess> {
    fn access(&self) -> O;
    fn create(&self, id: &ID, data: &Data) -> Result<Data> {
        Ok(self.access().create(id, data)?)
    }
    fn update(&self, id: &ID, data: &Data) -> Result<()> {
        self.access().update(id, data)
    }
    fn get(&self, id: &ID) -> Result<Data> {
        self.access().get(id)
    }
    fn delete(&self, id: &ID) -> Result<()> {
        self.pulverize(id)?;
        self.zerofill(id)?;
        self.access().delete(id)
    }
    /// scrubs data at low-level reducing the capacity of the memory area to 0
    fn zerofill(&self, id: &ID) -> Result<()> {
        let mut data = self.get(id)?;
        zerofill(&mut data.inner);
        self.update(&id, &data)?;
        Ok(())
    }
    /// scrubs data at low-level by charging, that is, maxing the memory area with choice of bytes to that effect then deleting all references
    /// MUST execute after zerofill()
    fn pulverize(&self, id: &ID) -> Result<()> {
        let mut data = self.get(id)?;

        let mut type_range = (0..u8::MAX).collect::<Vec<u8>>();
        type_range.reverse();

        zerofill(&mut data.inner);
        self.update(&id, &data)?;
        for k in type_range {
            scrub_with_byte(&mut data.inner, k);
            self.update(&id, &data)?;
        }
        zerofill(&mut data.inner);
        self.update(&id, &data)?;
        Ok(())
    }
}

pub trait KeyContainer<L: StorageAccess, E: EncryptionKey, D: DecryptionKey>: Storage<L> {
    fn encryptor(&self) -> E;
    fn decryptor(&self) -> D;

    fn encrypt(&self, data: &Data) -> Result<Data> {
        Ok(self.encryptor().encrypt(data)?)
    }
    fn decrypt(&self, data: &Data) -> Result<Data> {
        Ok(self.decryptor().decrypt(data)?)
    }
    fn rekey(&mut self, id: &ID) -> Result<()> {
        let keyed = &mut self.get(id)?;
        self.delete(id)?;
        let rekeyed = self.encrypt(&self.decrypt(&keyed)?)?;
        self.create(id, &rekeyed)?;
        Ok(())
    }
}
