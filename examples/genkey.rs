use pqpfs::{RSAPrivateKey, Result};

fn main() -> Result<()> {
    let private_key = RSAPrivateKey::generate()?;
    let bytes = private_key.to_flate_bytes()?;
    println!("{}", hex::encode(&bytes));
    eprintln!("len {}", bytes.len());
    Ok(())
}
