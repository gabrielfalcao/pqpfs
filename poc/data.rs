use std::ops::{BitXor, BitXorAssign, Drop, Index, IndexMut};

use security_framework::os::macos::passwords::SecKeychainItemPassword;
use serde::{Deserialize, Serialize};

use crate::utils::{drop, xor, xor_ip};
use crate::Result;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Data {
    pub inner: Vec<u8>,
}
impl Data {
    pub fn new(inner: Vec<u8>) -> Data {
        Data { inner }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn to_hex(&self, sep: &str) -> String {
        self.inner
            .iter()
            .map(|o| format!("{:02x}", o))
            .collect::<Vec<String>>()
            .join(sep)
    }

    pub fn from_hex(data: &str) -> Result<Data> {
        let data = hex::decode(data)?;
        Ok(Data::new(data))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex(""))
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Data {
        Data::new(data)
    }
}
impl From<&Vec<u8>> for Data {
    fn from(data: &Vec<u8>) -> Data {
        let data = data.clone();
        Data::new(data)
    }
}
impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Data {
        let data = data.to_vec();
        Data::new(data)
    }
}
impl Into<Vec<u8>> for Data {
    fn into(self) -> Vec<u8> {
        self.bytes()
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        drop(&mut self.inner);
    }
}

impl BitXor for Data {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Data::from(xor(&self.inner, &rhs.bytes()))
    }
}

impl BitXorAssign for Data {
    fn bitxor_assign(&mut self, rhs: Self) {
        xor_ip(&mut self.inner, &rhs.bytes())
    }
}

impl std::ops::Add for Data {
    type Output = Data;

    fn add(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s + o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl std::ops::Sub for Data {
    type Output = Data;

    fn sub(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s - o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl std::ops::Div for Data {
    type Output = Data;

    fn div(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s / o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl std::ops::Mul for Data {
    type Output = Data;

    fn mul(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s * o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl std::ops::Rem for Data {
    type Output = Data;

    fn rem(self, other: Data) -> Data {
        Data::from(
            self.to_vec()
                .iter()
                .zip(other.to_vec())
                .map(|(s, o)| s % o)
                .collect::<Vec<u8>>(),
        )
    }
}
impl Index<usize> for Data {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}
impl From<SecKeychainItemPassword> for Data {
    fn from(data: SecKeychainItemPassword) -> Data {
        let data = data.to_vec();
        Data::new(data)
    }
}
