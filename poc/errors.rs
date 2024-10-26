use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::Display;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidUtf8(String),
    ParseIntError(String),
    RSAError(String),
    PKCS8Error(String),
    PKCS1Error(String),
    HexDecodeError(String),
    DeserializationError(String),
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("variant", &self.variant())?;
        s.serialize_field("message", &format!("{}", self))?;
        s.end()
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.variant(),
            match self {
                Self::InvalidUtf8(s) => format!("{}", s),
                Self::ParseIntError(s) => format!("{}", s),
                Self::RSAError(s) => format!("{}", s),
                Self::PKCS8Error(s) => format!("{}", s),
                Self::PKCS1Error(s) => format!("{}", s),
                Self::HexDecodeError(s) => format!("{}", s),
                Self::DeserializationError(s) => format!("{}", s),
            }
        )
    }
}
impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::InvalidUtf8(_) => "InvalidUtf8",
            Error::ParseIntError(_) => "ParseIntError",
            Error::RSAError(_) => "RSAError",
            Error::PKCS8Error(_) => "PKCS8Error",
            Error::PKCS1Error(_) => "PKCS1Error",
            Error::HexDecodeError(_) => "HexDecodeError",
            Error::DeserializationError(_) => "DeserializationError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(format!("{}", e))
    }
}
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8(format!("{}", e))
    }
}
impl From<rsa::Error> for Error {
    fn from(e: rsa::Error) -> Self {
        Error::RSAError(format!("{}", e))
    }
}
impl From<rsa::pkcs8::Error> for Error {
    fn from(e: rsa::pkcs8::Error) -> Self {
        Error::PKCS8Error(format!("{}", e))
    }
}
impl From<rsa::pkcs1::Error> for Error {
    fn from(e: rsa::pkcs1::Error) -> Self {
        Error::PKCS1Error(format!("{}", e))
    }
}
impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::HexDecodeError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
