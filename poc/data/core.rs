use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::Extend;

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Data {
    pub inner: Vec<u8>,
}
impl Data {
    pub fn new(inner: Vec<u8>) -> Data {
        Data { inner }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.to_vec()
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

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        crate::to_flate_bytes(self)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<Data> {
        Ok(crate::from_deflate_bytes::<Data>(bytes)?)
    }

    pub fn iter(&self) -> DataIterator {
        DataIterator::new(self)
    }

    pub fn filter(self, predicate: impl FnMut(&u8) -> bool) -> Data {
        self.iter().filter(predicate).collect::<Data>()
    }

    pub fn map(self, predicate: impl FnMut(u8) -> u8) -> Data {
        self.iter().map(predicate).collect::<Data>()
    }

    pub fn difference(&self, other: &Data) -> Data {
        self.set().difference(&other.set()).cloned().collect()
    }

    pub fn intersection(&self, other: &Data) -> Data {
        self.set().intersection(&other.set()).cloned().collect()
    }

    pub fn contains(&mut self, byte: u8) -> bool {
        !self.inner.iter().filter(|c| **c == byte).collect::<Vec<_>>().is_empty()
    }

    pub fn sort_by(&mut self, f: impl FnMut(&u8, &u8) -> Ordering) {
        self.inner.sort_by(f)
    }

    pub fn get(&mut self, index: usize) -> Option<u8> {
        self.inner.get(index).map(|byte| *byte)
    }

    pub fn push(&mut self, byte: u8) {
        self.inner.push(byte)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.inner.pop()
    }

    pub fn set(&self) -> BTreeSet<u8> {
        let mut set = BTreeSet::new();
        for v in self.iter() {
            set.insert(v);
        }
        set
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn extend<T: Iterator<Item = u8>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }

    pub fn extended<T: Iterator<Item = u8>>(&self, iter: T) -> Data {
        let mut data = self.clone();
        data.extend(iter);
        data
    }

    pub fn then<T>(&self, mut no_more: impl FnMut(Self) -> T) -> Option<T> {
        if self.len() > 0 {
            Some(no_more(self.clone()))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex(""))
    }
}

pub struct DataIterator<'a> {
    data: &'a Data,
    pos: usize,
}

impl<'a> DataIterator<'a> {
    pub fn new(data: &'a Data) -> DataIterator<'a> {
        DataIterator { data: data, pos: 0 }
    }
}

impl std::iter::Iterator for DataIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            Some(self.data[self.pos])
        } else {
            None
        }
    }
}
