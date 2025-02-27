use std::ops::DerefMut;

use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::data::Data;
use crate::errors::{Error, Result};
use crate::traits::PlainBytes;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct RenderState<'o> {
    previous: &'o RenderState,
}

impl RenderState {}
impl PlainBytes for RenderStateKey {}
impl From<Data> for RenderStateKey {
    fn from(data: Data) -> RenderStateKey {
        Self::from_bytes(&data.to_vec())
    }
}
impl From<&Data> for RenderStateKey {
    fn from(data: &Data) -> RenderStateKey {
        Self::from_bytes(&data.to_vec())
    }
}
impl From<Vec<u8>> for RenderStateKey {
    fn from(data: Vec<u8>) -> RenderStateKey {
        Self::from_bytes(&data)
    }
}
impl From<&Vec<u8>> for RenderStateKey {
    fn from(data: &Vec<u8>) -> RenderStateKey {
        Self::from_bytes(data)
    }
}
impl From<&[u8]> for RenderStateKey {
    fn from(data: &[u8]) -> RenderStateKey {
        Self::from_bytes(data)
    }
}
