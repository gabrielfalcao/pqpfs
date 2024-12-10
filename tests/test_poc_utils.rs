use pqpfs::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Blob {
    pub name: String,
    pub data: Data,
}

#[test]
fn test_utils_chunk_padded() {
    let data = vec![0x01, 0x10, 0xF1, 0x61];
    assert_eq!(chunk_padded(&data, 6, 0), vec![vec![0x01, 0x10, 0xF1, 0x61, 0, 0]]);
}


#[test]
fn test_flate_bytes() -> Result<()> {
    let blob = Blob  {
        name: String::from(file!()),
        data: Data::random(rand::thread_rng(), u16::MAX.into())
    };
    let bytes = to_flate_bytes(&blob)?;
    assert_eq!(blob, from_deflate_bytes::<Blob>(&bytes)?);
    Ok(())
}
