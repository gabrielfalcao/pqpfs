use pqpfs::{RSAPrivateKey, Result};

fn main() -> Result<()> {
    let private_key = RSAPrivateKey::generate()?;
    println!("{}", hex::encode(private_key.to_flate_bytes()?));
    Ok(())
}
