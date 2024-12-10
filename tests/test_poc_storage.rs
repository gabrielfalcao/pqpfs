use pqpfs::{Data, MacOSKeychainStorage, Result, StorageAccess, ID};

#[test]
fn test_mac_os_keychain_storage() -> Result<()> {
    let storage = MacOSKeychainStorage::new("pqpfs::poc::test");
    let id = ID::generate()?;
    let data = Data::from(ID::generate()?.to_bytes());
    let stored = storage.create(&id, &data)?;
    assert_eq!(data, stored);
    let retrieved = storage.get(&id)?;
    assert_eq!(stored, retrieved);
    storage.delete(&id)?;
    Ok(())
}
