pub trait PlainBytes {
    fn bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
}

pub trait SignedBytes<T: PlainBytes> {
    fn created_at(&self) -> Vec<u8>;
    fn signatures(&self) -> Vec<T>;
}
