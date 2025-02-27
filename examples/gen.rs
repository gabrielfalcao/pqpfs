use pqpfs::{RSAPrivateKey, Result};


fn main() -> Result<()> {
    let private_key = RSAPrivateKey::generate()?;
    println!("[{}]", private_key.to_vec().iter().map(|byte|byte.to_string()).collect::<Vec<String>>().join(", "));
    Ok(())
}
