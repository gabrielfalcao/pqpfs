pub trait PlainBytes {
    fn bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
}

pub trait SignedBytes<T: PlainBytes> {
    fn created_at(&self) -> Vec<u8>;
    fn signatures(&self) -> Vec<T>;
}

pub trait EphemeralDevice {
    fn created_at(&self) -> t16::Data;
    fn received_at(&self) -> t16::Data;
    fn expires_at(&self) -> t16::Data;
}

pub trait EphemeralEncryptionDevice: EphemeralDevice {
    fn accept_bytes(&self, data: &[u8]) -> crate::Result<usize>;
    fn encrypt(&self) -> crate::Result<crate::Data>;
    // fn created_at(&self) -> t16::Data;
    // fn received_at(&self) -> t16::Data;
    // fn expires_at(&self) -> t16::Data;
}
pub trait EphemeralDecryptionDevice{
    fn accept_bytes(&self, data: &[u8]) -> crate::Result<usize>;
    fn decrypt(&self) -> crate::Result<crate::Data>;
    fn created_at(&self) -> t16::Data;
    fn received_at(&self) -> t16::Data;
    fn expires_at(&self) -> t16::Data;
}
