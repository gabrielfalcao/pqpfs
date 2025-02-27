use clap::*;
use pqpfs::{Data, DecryptionKey, EncryptionKey, RSAPrivateKey, Result};
use iocore::Path;

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[arg()]
    pub path: Path,
}

fn main() -> Result<()> {
    let op = Cli::parse();
    Ok(())
}
