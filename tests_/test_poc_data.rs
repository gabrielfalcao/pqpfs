use pqpfs::*;

#[test]
fn test_data_add() {
    assert_eq!(data![1, 2, 3] + data![5, 8, 13], data![6, 10, 16]);
}
#[test]
fn test_data_sub() {
    assert_eq!(data![6, 10, 16] - data![5, 8, 13], data![1, 2, 3]);
}

#[test]
fn test_data_div() {
    assert_eq!(data![20, 30, 60] / data![2, 10, 20], data![10, 3, 3]);
}

#[test]
fn test_data_mul() {
    assert_eq!(data![2, 3, 6] * data![10, 8, 6], data![20, 24, 36]);
}

#[test]
fn test_data_rem() {
    assert_eq!(data![12, 13, 16] % data![10, 8, 6], data![2, 5, 4]);
}

#[test]
fn test_data_xor() {
    assert_eq!(data![0x0F, 0xF0, 0xAF] % data![0xFF, 0xFF, 0xFF], data![0x0F, 0xF0, 0xAF]);
}

#[test]
fn test_data_shl() {
    assert_eq!(data![1, 2, 3] << data![1, 2, 3], data![2, 8, 24]);
}

#[test]
fn test_data_shr() {
    assert_eq!(data![2, 8, 24] >> data![1, 2, 3], data![1, 2, 3]);
}

#[test]
fn test_data_not() {
    assert_eq!(!data![0b01010101], data![0b10101010]);
}

#[test]
fn test_data_iter() {
    let sut = data![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let mut pos = 0;
    let mut items = Vec::new();
    while pos < sut.len() {
        items.push(sut[pos]);
        pos += 1;
    }
    assert_eq!(items, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
}

#[test]
fn test_data_random() {
    let length: usize = u8::MAX.into();
    let data = Data::random(rand::thread_rng(), length);
    assert_eq!(data.len(), length);
}

#[test]
fn test_data_eq() {
    assert_eq!(data![1], data![1]);
    assert_ne!(data![0, 0], data![0, 1]);
}
