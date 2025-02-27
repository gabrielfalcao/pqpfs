#[macro_export]
macro_rules! impl_plain_bytes {
    ($n:ty, o:$block) => {
        impl pqpfs::PlainBytes for $o
        impl From<pqpfs::Data> for <$n> {
            fn from(data: pqpfs::Data) -> <$n> {
                Self::from_bytes(&data.to_vec())
            }
        }
        impl From<&pqpfs::Data> for <$n> {
            fn from(data: &pqpfs::Data) -> <$n> {
                Self::from_bytes(&data.to_vec())
            }
        }
        impl From<Vec<u8>> for <$n> {
            fn from(data: Vec<u8>) -> <$n> {
                Self::from_bytes(&data)
            }
        }
        impl From<&Vec<u8>> for <$n> {
            fn from(data: &Vec<u8>) -> <$n> {
                Self::from_bytes(data)
            }
        }
        impl From<&[u8]> for <$n> {
            fn from(data: &[u8]) -> <$n> {
                Self::from_bytes(data)
            }
        }
    };
    ($t:ty) => {
        impl_plain_bytes!($t, {
            fn to_bytes(&self) -> Vec<u8>; {
                self.to_plain_bytes()
            }
            fn from_bytes(bytes: &[u8]) -> Self {
                Self::from_plain_bytes(bytes).expect(&format!("{} ", std::any::type_name($t)))
            }
        })
    }
}
