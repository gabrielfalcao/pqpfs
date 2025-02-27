use pqpfs::*;

#[test]
fn test_poc_pfscontainer() -> Result<()> {
    let private_key = RSAPrivateKey::generate()?;
    let data0 = Data::from_iter((0..u8::MAX).map(|s| 0u8));
    // let data1 = Data::from_iter((0..u8::MAX).map(|s| 1u8));
    // let data2 = Data::from_iter((0..u8::MAX).map(|s| 2u8));

    let mut container = PFSContainer::<RSAPrivateKey>::new(&private_key);
    let private_key0 = RSAPrivateKey::generate()?;
    let ciphertext0 = PFSCipherText::<RSAPrivateKey>::new(&data0, &private_key0);
    container.add_ciphertext(&ciphertext0);
    Ok(())
}
