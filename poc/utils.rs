use std::io::Write;

use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::errors::Result;

pub fn xor_ip(a: &mut Vec<u8>, o: &Vec<u8>) {
    let alen = a.len();
    let olen = o.len();

    let max = if alen > olen {
        alen
    } else if olen > alen {
        olen
    } else {
        olen
    };

    for k in 0..max {
        if k < max {
            a[k] = a[k] ^ o[k];
        } else {
            break;
        }
    }
}
pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.into_iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn scrub_with_byte(data: &mut Vec<u8>, byte: u8) {
    let data_range = 0..data.len();
    for n in data_range {
        data[n] = byte;
    }
}
pub fn zerofill(data: &mut Vec<u8>) {
    scrub_with_byte(data, 0);
}

pub fn discharge(data: &mut Vec<u8>) {
    for k in 0..u8::MAX {
        scrub_with_byte(data, u8::MAX ^ k);
    }
}
pub fn rev(data: &mut Vec<u8>) {
    let length = data.len();
    for k in 0..length {
        data[k] = data[k] ^ 0xFF;
    }
}
pub fn drop(data: &mut Vec<u8>) {
    rev(data);
    scrub_with_byte(data, 0x7);
    scrub_with_byte(data, 0x0);
    scrub_with_byte(data, 0x1);
    zerofill(data);
    discharge(data);
    zerofill(data);
}

pub fn to_flate_bytes<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
    e.write(&bincode::serialize(data)?)?;
    Ok(e.finish()?)
}

pub fn from_deflate_bytes<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T> {
    let mut d = DeflateDecoder::new(Vec::new());
    d.write(bytes)?;
    let bytes = d.finish()?;
    Ok(bincode::deserialize::<T>(&bytes)?)
}
