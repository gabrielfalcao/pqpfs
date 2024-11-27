use pqpfs::*;

#[test]
fn test_utils_chunk_padded() {
    let data = vec![0x01, 0x10, 0xF1, 0x61];
    assert_eq!(chunk_padded(&data, 6, 0), vec![vec![0x01, 0x10, 0xF1, 0x61, 0, 0]]);
}
