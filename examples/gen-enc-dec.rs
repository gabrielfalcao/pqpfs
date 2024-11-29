use clap::*;
use iocore::Path;
use pqpfs::{Data, DecryptionKey, EncryptionKey, RSAPrivateKey, Result};
use sanitation::SString;

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[arg()]
    pub plaintext: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let private_key = RSAPrivateKey::generate()?;
    let public_key = private_key.public_key();

    let plaintext = args.plaintext.as_bytes().to_vec();
    let data = Data::new(plaintext);
    let ciphertext = public_key.encrypt(data.iter())?;
    let plaintext = private_key.decrypt(ciphertext.iter())?;
    println!("{}", hex::encode(ciphertext.bytes()));
    println!("{}", SString::from(plaintext.bytes()).unchecked_safe());
    Ok(())
}
