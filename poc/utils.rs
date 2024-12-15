pub use std::f64::consts::{PI, TAU};
use std::io::{Read, Write};

use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
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
    let bytes = bincode::serialize(data)?;
    let mut e = DeflateEncoder::new(Vec::with_capacity(bytes.len()), Compression::best());
    e.write_all(&bytes)?;
    Ok(e.finish()?)
}

pub fn from_deflate_bytes<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T> {
    let mut d = DeflateDecoder::new(bytes);
    let mut bytes = Vec::<u8>::with_capacity(bytes.len());
    d.read_to_end(&mut bytes)?;
    Ok(bincode::deserialize::<T>(&bytes)?)
}

pub fn chunk_padded(items: &[u8], chunk_size: usize, padding: u8) -> Vec<Vec<u8>> {
    let rem = rem(items, chunk_size);
    let mut items = items.iter().map(|byte| *byte).collect::<Vec<u8>>();

    for _ in 0..rem {
        items.push(padding)
    }

    let mut chunked = Vec::<Vec<u8>>::new();
    for chunk in items.chunks(chunk_size) {
        chunked.push(chunk.to_vec());
    }
    chunked
}
pub(crate) fn rem(items: &[u8], chunk_size: usize) -> usize {
    if items.len() > chunk_size {
        items.len() % chunk_size
    } else if items.len() > 0 && chunk_size > items.len() {
        chunk_size % items.len()
    } else {
        0
    }
}

pub enum Angle {
    Radian(f64),
    Displacement(f64),
}
pub fn radian_to_degree(radian: f64) -> f64 {
    radian * 57.2958
}
pub fn degree_to_radian(degree: f64) -> f64 {
    degree / 57.2958
}
// pub fn angular_frequency(angle: Angle, secs: f64) -> f64 {
//     use Angle::*;
//     match angle {
//         Radian(deg) => degree_to_radian(deg) / secs,
//         Displacement(rad) => rad / secs
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rem() {
        let data = vec![0x01, 0x10, 0xF1, 0x61];
        assert_eq!(rem(&data, 6), 2);
        let data = vec![0x01, 0x10, 0xF1, 0x61, 0x01, 0x10, 0xF1, 0x61];
        assert_eq!(rem(&data, 6), 2);
        let data = vec![];
        assert_eq!(rem(&data, 6), 0);
    }
    #[test]
    fn test_radian_to_degree() {
        assert_eq!(radian_to_degree(PI), 180.00006436155007);
        assert_eq!(radian_to_degree(TAU), 360.00012872310015);
    }
    #[test]
    fn test_degree_to_radian() {
        assert_eq!(degree_to_radian(180.00006436155007), PI);
        assert_eq!(degree_to_radian(360.00012872310015), TAU);
    }
    // #[test]
    // fn test_angular_frequency() {
    //     assert_eq!(angular_frequency(Angle::Radian(9.42), 5.40), 1.75);
    // }
}
